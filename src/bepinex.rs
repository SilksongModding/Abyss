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


