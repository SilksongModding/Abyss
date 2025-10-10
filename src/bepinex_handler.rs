use crate::bepinex_provider::{
    BepinexProvider, LinuxBepinexProvider, MacOSBepinexProvider, UnsupportedBepinexProvider,
    WindowsBepinexProvider,
};
use std::{error::Error, io::stdin, path::PathBuf};

/// Enums for OS detection
enum OS {
    Windows,
    Linux,
    MacOs,
    Unsupported,
}

pub struct BepinexHandler {
    provider: Box<dyn BepinexProvider>,
}

impl BepinexHandler {
    pub fn from_os(game_path: PathBuf, os: &str) -> Self {
        let current_os: OS = match os {
            "windows" => OS::Windows,
            "linux" => OS::Linux,
            "macos" => OS::MacOs,
            _ => OS::Unsupported,
        };

        let provider: Box<dyn BepinexProvider> = match current_os {
            OS::Windows => Box::new(WindowsBepinexProvider { game_path }),
            OS::Linux => Box::new(LinuxBepinexProvider { game_path }),
            OS::MacOs => Box::new(MacOSBepinexProvider { game_path }),
            OS::Unsupported => Box::new(UnsupportedBepinexProvider),
        };

        Self { provider }
    }

    pub fn new(game_path: PathBuf) -> Self {
        Self::from_os(game_path, std::env::consts::OS)
    }

    /// Dynamic dispatch of the correct method
    pub fn check_installation(&self) -> Result<bool, String> {
        self.provider.check_installation()
    }

    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        // Ask to skip or overwrite installation if already present
        if self.check_installation()? {
            println!("BepInEx is already installed, do you want to reinstall it? [y/N]");
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");

            if input == "y\n" || input == "Y\n" {
                return self.provider.install();
            } else {
                return Err("Installation aborted by the user".into());
            }
        }
        self.provider.install()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_bepinex_handler_linux() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let handler = BepinexHandler::from_os(game_path.clone(), "linux");
        assert!(handler.check_installation().is_ok());
        assert!(handler.install().is_ok());
    }

    #[test]
    fn test_bepinex_handler_windows() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let handler = BepinexHandler::from_os(game_path.clone(), "windows");
        assert!(handler.check_installation().is_ok());
        assert!(handler.install().is_ok());
    }

    #[test]
    fn test_bepinex_handler_macos() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let handler = BepinexHandler::from_os(game_path.clone(), "macos");
        assert!(handler.check_installation().is_ok());
        assert!(handler.install().is_ok());
    }

    #[test]
    fn test_bepinex_handler_unknown() {
        let temp_dir = tempdir().unwrap();
        let game_path = temp_dir.path().to_path_buf();

        let handler = BepinexHandler::from_os(game_path.clone(), "freebsd");
        assert!(handler.check_installation().is_err());
        assert!(handler.install().is_err());
    }
}
