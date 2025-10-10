use reqwest::blocking::{Client, get};
use reqwest::header::USER_AGENT;
use serde_json::Value;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

enum OS {
    Windows,
    Linux,
    MacOs,
}

/// Abstract provider interface to allow different implementations for different operating systems
pub trait BepinexProvider {
    ///Check if all necessary directories, subdirectories and files at the game's root directory
    ///exist
    fn check_installation(&self) -> Result<bool, String>;

    ///Download BepInEx and extract it in the game's directory
    fn install(&self) -> Result<(), Box<dyn Error>>;
}

pub struct LinuxBepinexProvider {
    pub game_path: PathBuf,
}

impl BepinexProvider for LinuxBepinexProvider {
    fn check_installation(&self) -> Result<bool, String> {
        let bepinex_dir = self.game_path.join("BepInEx");
        let bepinex_core_dir = bepinex_dir.join("core");
        let root_files = [
            self.game_path.join("libdoorstop.so"),
            self.game_path.join("run_bepinex.sh"),
            self.game_path.join(".doorstop_version"),
            self.game_path.join("changelog.txt"),
        ];

        let res = bepinex_dir.exists()
            && bepinex_dir.is_dir()
            && bepinex_core_dir.exists()
            && bepinex_core_dir.is_dir()
            && root_files.iter().all(|path| path.exists());

        Ok(res)
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        install_bepinex_latest(OS::Linux, &self.game_path)?;

        //Update the launch script according to the BepInEx installation docs
        let game_executable_name = "Hollow Knight Silksong";
        update_launch_script(&self.game_path, game_executable_name)
    }
}

pub struct WindowsBepinexProvider {
    pub game_path: PathBuf,
}

impl BepinexProvider for WindowsBepinexProvider {
    fn check_installation(&self) -> Result<bool, String> {
        let bepinex_dir = self.game_path.join("BepInEx");
        let bepinex_core_dir = bepinex_dir.join("core");
        let root_files = [
            self.game_path.join("doorstop_config.ini"),
            self.game_path.join("winhttp.dll"),
            self.game_path.join(".doorstop_version"),
            self.game_path.join("changelog.txt"),
        ];

        let res = bepinex_dir.exists()
            && bepinex_dir.is_dir()
            && bepinex_core_dir.exists()
            && bepinex_core_dir.is_dir()
            && root_files.iter().all(|path| path.exists());

        Ok(res)
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        install_bepinex_latest(OS::Windows, &self.game_path)
    }
}

pub struct MacOSBepinexProvider {
    pub game_path: PathBuf,
}

impl BepinexProvider for MacOSBepinexProvider {
    fn check_installation(&self) -> Result<bool, String> {
        let bepinex_dir = self.game_path.join("BepInEx");
        let bepinex_core_dir = bepinex_dir.join("core");
        let root_files = [
            self.game_path.join("libdoorstop.dylib"),
            self.game_path.join("run_bepinex.sh"),
            self.game_path.join(".doorstop_version"),
            self.game_path.join("changelog.txt"),
        ];

        let res = bepinex_dir.exists()
            && bepinex_dir.is_dir()
            && bepinex_core_dir.exists()
            && bepinex_core_dir.is_dir()
            && root_files.iter().all(|path| path.exists());

        Ok(res)
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        install_bepinex_latest(OS::MacOs, &self.game_path)?;

        //Update the launch script according to the BepInEx installation docs
        let game_executable_name = "Hollow Knight Silksong.app";
        update_launch_script(&self.game_path, game_executable_name)
    }
}

pub struct UnsupportedBepinexProvider;

impl BepinexProvider for UnsupportedBepinexProvider {
    fn check_installation(&self) -> Result<bool, String> {
        Err("The current operating system is not supported!".into())
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        Err("The current operating system is not supported!".into())
    }
}

/// Check for the latest release, download the zip for the specified platform and extract it in the
/// specified game directory
fn install_bepinex_latest(platform: OS, game_path: &Path) -> Result<(), Box<dyn Error>> {
    let repo_author = "BepInEx";
    let repo_name = "BepInEx";
    let github_latest_url = "https://api.github.com/repos/BepInEx/BepInEx/releases/latest";

    // Get the latest release tag
    println!("Fetching the latest release...");
    let client = Client::new();
    let response = client
        .get(github_latest_url)
        .header(USER_AGENT, "Abyss/0.1") // Github requires this
        .send()?;

    let data: Value = response.json()?;

    let tag;
    match data.get("tag_name") {
        Some(value) => {
            tag = value.to_string().trim_matches('"').to_string();
            println!("Found the latest version {}", tag);
        }
        None => return Err("Could not find the latest release of BepInEx".into()),
    };

    // Build the download url depending on the platform
    let release_url = build_release_url(repo_author, repo_name, &tag, platform);
    if release_url.is_empty() {
        return Err("Unsupported platform for the github download!".into());
    }

    // Download of the zipped file
    println!("Downloading {}", release_url);
    let download_response = get(release_url)?;
    let body = download_response.bytes()?;

    // Store the file
    let temp_file = game_path.join("downloaded.zip");
    let mut file = File::create(&temp_file)?;
    file.write_all(&body)?;

    // Extraction
    println!("Extracting...");
    let file = File::open(&temp_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = game_path.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut out_file = File::create(outpath)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }

    // Clean up
    println!("Cleaning up...");
    fs::remove_file(temp_file)?;

    Ok(())
}

fn build_release_url(author: &str, repo: &str, tag: &str, platform: OS) -> String {
    match platform {
        OS::Windows => {
            format!(
                "https://github.com/{}/{}/releases/download/{}/BepInEx_{}_x64_{}.zip",
                author,
                repo,
                tag,
                "win",
                &tag[1..],
            )
        }
        OS::Linux => {
            format!(
                "https://github.com/{}/{}/releases/download/{}/BepInEx_{}_x64_{}.zip",
                author,
                repo,
                tag,
                "linux",
                &tag[1..],
            )
        }
        OS::MacOs => {
            format!(
                "https://github.com/{}/{}/releases/download/{}/BepInEx_{}_x64_{}.zip",
                author,
                repo,
                tag,
                "macos",
                &tag[1..],
            )
        }
    }
}

fn update_launch_script(game_path: &Path, exe_name: &str) -> Result<(), Box<dyn Error>> {
    let launch_script_name = "run_bepinex.sh";

    let launch_script_path = game_path.join(launch_script_name);
    match launch_script_path.exists() {
        true => {
            let mut script = String::new();
            let mut file = fs::File::open(&launch_script_path)?;

            file.read_to_string(&mut script)?;

            let updated_script = script.replace(
                r#"executable_name="""#,
                &format!(r#"executable_name="{}""#, &exe_name),
            );

            let mut file = fs::File::create(&launch_script_path)?;
            file.write_all(updated_script.as_bytes())?;
        }
        false => return Err("Could not find the BepInEx launch script!".into()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_check_installation_windows() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let bepinex_dir = game_path.join("BepInEx");
        fs::create_dir_all(bepinex_dir.join("core")).unwrap();
        File::create(game_path.join("doorstop_config.ini")).unwrap();
        File::create(game_path.join("winhttp.dll")).unwrap();
        File::create(game_path.join(".doorstop_version")).unwrap();
        File::create(game_path.join("changelog.txt")).unwrap();

        let provider = WindowsBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_installation_linux() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let bepinex_dir = game_path.join("BepInEx");
        fs::create_dir_all(bepinex_dir.join("core")).unwrap();
        File::create(game_path.join("libdoorstop.so")).unwrap();
        File::create(game_path.join("run_bepinex.sh")).unwrap();
        File::create(game_path.join(".doorstop_version")).unwrap();
        File::create(game_path.join("changelog.txt")).unwrap();

        let provider = LinuxBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_installation_macos() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let bepinex_dir = game_path.join("BepInEx");
        fs::create_dir_all(bepinex_dir.join("core")).unwrap();
        File::create(game_path.join("libdoorstop.dylib")).unwrap();
        File::create(game_path.join("run_bepinex.sh")).unwrap();
        File::create(game_path.join(".doorstop_version")).unwrap();
        File::create(game_path.join("changelog.txt")).unwrap();

        let provider = MacOSBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_installation_failed_linux() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let provider = LinuxBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_check_installation_failed_windows() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let provider = WindowsBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_check_installation_failed_macos() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let provider = MacOSBepinexProvider { game_path };

        let result = provider.check_installation();

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
