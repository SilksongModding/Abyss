use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use tracing::info;
use zip::ZipArchive;

const GITHUB_API_LATEST_RELEASE: &str =
    "https://api.github.com/repos/BepInEx/BepInEx/releases/latest";

#[derive(Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>,
    tag_name: String,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn download_bepinex() -> Result<(PathBuf, String)> {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        info!("Using GITHUB_TOKEN for authentication");
        let mut auth_val = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
            .context("Invalid header value for GITHUB_TOKEN")?;
        auth_val.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth_val);
    }

    let client = Client::builder()
        .user_agent("abyss-installer")
        .default_headers(headers)
        .build()
        .context("Failed to build HTTP client")?;

    info!("Fetching latest BepInEx release info...");
    let resp = client.get(GITHUB_API_LATEST_RELEASE).send();

    match resp {
        Ok(resp) => {
            if resp.status() == reqwest::StatusCode::FORBIDDEN {
                info!("GitHub API rate limit exceeded. Falling back to web scraping...");
                return download_bepinex_scrape(&client);
            }

            let resp = resp.error_for_status().context("GitHub API returned error")?;
            let release: Release = resp.json().context("Failed to parse release JSON")?;
            info!("Latest version: {}", release.tag_name);

            let target_os = std::env::consts::OS;
            let os_keyword = match target_os {
                "linux" => "linux",
                "macos" => "macos",
                "windows" => "win",
                _ => "x64", // Fallback
            };

            let asset = release
                .assets
                .iter()
                .find(|a| a.name.contains(os_keyword) && a.name.contains("x64") && a.name.ends_with(".zip"))
                .or_else(|| {
                    // Fallback to just x64 if specific OS not found (e.g. maybe just "unix" or generic)
                    release.assets.iter().find(|a| a.name.contains("x64") && a.name.ends_with(".zip"))
                })
                .context("Could not find a suitable x64 zip asset in the latest release")?;

            download_file(&client, &asset.browser_download_url, &release.tag_name)
        }
        Err(_) => {
             info!("Failed to connect to GitHub API. Falling back to web scraping...");
             download_bepinex_scrape(&client)
        }
    }
}

fn download_bepinex_scrape(client: &Client) -> Result<(PathBuf, String)> {
    // 1. Get latest release tag by following redirect
    let resp = client
        .get("https://github.com/BepInEx/BepInEx/releases/latest")
        .send()
        .context("Failed to fetch latest release page")?
        .error_for_status()?;
    
    let final_url = resp.url().as_str();
    let tag_name = final_url
        .split('/')
        .last()
        .context("Failed to extract tag name from URL")?
        .to_string();
    
    info!("Latest version (scraped): {}", tag_name);

    // 2. Fetch expanded assets
    let assets_url = format!("https://github.com/BepInEx/BepInEx/releases/expanded_assets/{}", tag_name);
    let resp = client
        .get(&assets_url)
        .send()
        .context("Failed to fetch assets page")?
        .error_for_status()?;
    
    let html = resp.text().context("Failed to read assets page HTML")?;

    // 3. Find asset link
    let target_os = std::env::consts::OS;
    let os_keyword = match target_os {
        "linux" => "linux",
        "macos" => "macos",
        "windows" => "win",
        _ => "x64",
    };

    // Simple string parsing to find the link
    // Looking for href="/BepInEx/BepInEx/releases/download/TAG/FILENAME"
    // We want FILENAME to contain os_keyword and x64 and end in .zip
    
    let pattern = format!("href=\"/BepInEx/BepInEx/releases/download/{}/", tag_name);
    for line in html.lines() {
        if let Some(idx) = line.find(&pattern) {
            let start = idx + 6; // skip href="
            let end = line[start..].find('"').unwrap_or(line.len());
            let path = &line[start..start+end];
            
            if path.contains(os_keyword) && path.contains("x64") && path.ends_with(".zip") {
                let download_url = format!("https://github.com{}", path);
                return download_file(client, &download_url, &tag_name);
            }
        }
    }

    // Fallback search if OS specific not found
    for line in html.lines() {
        if let Some(idx) = line.find(&pattern) {
            let start = idx + 6;
            let end = line[start..].find('"').unwrap_or(line.len());
            let path = &line[start..start+end];
            
            if path.contains("x64") && path.ends_with(".zip") {
                let download_url = format!("https://github.com{}", path);
                return download_file(client, &download_url, &tag_name);
            }
        }
    }

    Err(anyhow::anyhow!("Could not find a suitable x64 zip asset via scraping"))
}

fn download_file(client: &Client, url: &str, version: &str) -> Result<(PathBuf, String)> {
    info!("Downloading from {}...", url);
    let mut resp = client
        .get(url)
        .send()
        .context("Failed to download asset")?
        .error_for_status()?;

    let mut temp_file = NamedTempFile::new().context("Failed to create temp file")?;
    io::copy(&mut resp, &mut temp_file).context("Failed to write to temp file")?;

    let (_file, path) = temp_file.keep().context("Failed to persist temp file")?;
    
    Ok((path, version.to_string()))
}

pub fn install_bepinex(zip_path: &Path, game_dir: &Path) -> Result<()> {
    info!("Extracting to {}...", game_dir.display());
    let file = fs::File::open(zip_path).context("Failed to open zip file")?;
    let mut archive = ZipArchive::new(file).context("Failed to read zip archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => game_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
