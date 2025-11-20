use anyhow::Result;
use std::path::{Path, PathBuf};

pub const BEPINEX_DIR_NAME: &str = "BepInEx";
pub const REQUIRED_SUBFOLDERS: &[&str] = &["core", "plugins", "config"];

pub struct BepInExInstallation {
    pub root_dir: PathBuf,
    pub is_valid: bool,
    pub missing_subfolders: Vec<String>,
}

impl BepInExInstallation {
    pub fn check(game_dir: &Path) -> Result<Self> {
        let bepinex_dir = game_dir.join(BEPINEX_DIR_NAME);
        
        if !bepinex_dir.exists() || !bepinex_dir.is_dir() {
            return Ok(Self {
                root_dir: bepinex_dir,
                is_valid: false,
                missing_subfolders: vec![], // If root is missing, subfolders are irrelevant/implied missing
            });
        }

        let mut missing = Vec::new();
        for sub in REQUIRED_SUBFOLDERS {
            let sub_path = bepinex_dir.join(sub);
            if !sub_path.exists() || !sub_path.is_dir() {
                missing.push(sub.to_string());
            }
        }

        Ok(Self {
            root_dir: bepinex_dir,
            is_valid: missing.is_empty(),
            missing_subfolders: missing,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_missing_bepinex_folder() {
        let dir = tempdir().unwrap();
        let install = BepInExInstallation::check(dir.path()).unwrap();
        assert!(!install.is_valid);
        // We don't list subfolders if the main folder is missing, as per implementation choice
        // but we could change this if needed. For now, let's stick to the impl.
    }

    #[test]
    fn test_valid_installation() {
        let dir = tempdir().unwrap();
        let bepinex = dir.path().join("BepInEx");
        fs::create_dir(&bepinex).unwrap();
        for sub in REQUIRED_SUBFOLDERS {
            fs::create_dir(bepinex.join(sub)).unwrap();
        }

        let install = BepInExInstallation::check(dir.path()).unwrap();
        assert!(install.is_valid);
        assert!(install.missing_subfolders.is_empty());
    }

    #[test]
    fn test_missing_subfolders() {
        let dir = tempdir().unwrap();
        let bepinex = dir.path().join("BepInEx");
        fs::create_dir(&bepinex).unwrap();
        // Create only 'core'
        fs::create_dir(bepinex.join("core")).unwrap();

        let install = BepInExInstallation::check(dir.path()).unwrap();
        assert!(!install.is_valid);
        assert!(install.missing_subfolders.contains(&"plugins".to_string()));
        assert!(install.missing_subfolders.contains(&"config".to_string()));
        assert!(!install.missing_subfolders.contains(&"core".to_string()));
    }
}
