use crate::provider::SteamProvider;
use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::warn;

/// Official Steam AppID for Hollow Knight: Silksong
pub const SILKSONG_APP_ID: u32 = 1030300;

/// Default folder name hints we'll search under steamapps/common
pub const DEFAULT_NAME_HINTS: &[&str] = &[
    "Hollow Knight Silksong",
    "Hollow Knight: Silksong",
    "Silksong",
];

// Provider types and real implementation live in `src/provider.rs`.

fn ensure_dir(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        Ok(path.to_path_buf())
    } else {
        anyhow::bail!("Provided path is not a directory: {}", path.display())
    }
}

pub struct Detector<P: SteamProvider> {
    provider: P,
}

impl<P: SteamProvider> Detector<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Detects the game dir following the documented precedence
    pub fn detect_game_dir(
        &self,
        explicit_dir: Option<&Path>,
        app_id: Option<u32>,
        extra_hints: &[String],
    ) -> Result<PathBuf> {
        // 1) Explicit override (CLI or Env)
        if let Some(p) = explicit_dir {
            return ensure_dir(p);
        }

        // 3a/3b) by app id via provider
        if let Some(id) = app_id {
            match self.provider.find_app(id) {
                Ok(Some((app, lib))) => return ensure_dir(&lib.resolve_app_dir(&app)),
                Ok(None) => (),
                Err(e) => warn!("Provider find_app failed for {}: {:#}", id, e),
            }
        } else {
            // user didn't pass custom app id; provider may still find official id
            match self.provider.find_app(SILKSONG_APP_ID) {
                Ok(Some((app, lib))) => return ensure_dir(&lib.resolve_app_dir(&app)),
                Ok(None) => (),
                Err(e) => warn!("Provider find_app failed for SILKSONG_APP_ID: {:#}", e),
            }
        }

        // 3c) folder name hints
        let mut hints: Vec<&str> = DEFAULT_NAME_HINTS.to_vec();
        for h in extra_hints {
            hints.push(h.as_str());
        }

        for lib in self.provider.libraries()? {
            let common = lib.path.join("steamapps").join("common");
            if !common.is_dir() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&common) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    for hint in &hints {
                        if name.eq_ignore_ascii_case(hint)
                            || name.to_lowercase().contains(&hint.to_lowercase())
                        {
                            let candidate = entry.path();
                            if candidate.is_dir() {
                                return Ok(candidate);
                            }
                        }
                    }
                }
            }
        }

        anyhow::bail!(
            "Game directory not found. Provide it with --game-dir or ABYSS_GAME_DIR, or pass a known --app-id / --name-hint."
        )
    }
}


