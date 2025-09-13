use anyhow::{Context, Result};
use std::path::PathBuf;

/// Minimal app representation used by providers
#[derive(Clone, Debug)]
pub struct App {
    pub install_dir: String,
}

/// Minimal library representation used by providers
#[derive(Clone, Debug)]
pub struct Library {
    pub path: PathBuf,
}

impl Library {
    pub fn resolve_app_dir(&self, app: &App) -> PathBuf {
        self.path
            .join("steamapps")
            .join("common")
            .join(&app.install_dir)
    }
}

/// Abstract provider interface to follow SOLID and enable testing
pub trait SteamProvider {
    /// Find an app by id, returning its install dir and the library it was found in
    fn find_app(&self, app_id: u32) -> Result<Option<(App, Library)>>;

    /// List libraries present in the Steam installation
    fn libraries(&self) -> Result<Vec<Library>>;
}

/// Real provider backed by `steamlocate` crate
pub struct RealSteamProvider;

impl SteamProvider for RealSteamProvider {
    fn find_app(&self, app_id: u32) -> Result<Option<(App, Library)>> {
        let steam = steamlocate::SteamDir::locate()
            .context("Failed to locate Steam installation. Is Steam installed?")?;

        if let Some((app, lib)) = steam.find_app(app_id)? {
            Ok(Some((
                App {
                    install_dir: app.install_dir,
                },
                Library {
                    path: lib.path().to_path_buf(),
                },
            )))
        } else {
            Ok(None)
        }
    }

    fn libraries(&self) -> Result<Vec<Library>> {
        let steam = steamlocate::SteamDir::locate()
            .context("Failed to locate Steam installation. Is Steam installed?")?;

        let mut out = Vec::new();
        for lib_res in steam.libraries()? {
            if let Ok(lib) = lib_res {
                out.push(Library {
                    path: lib.path().to_path_buf(),
                });
            }
        }
        Ok(out)
    }
}
