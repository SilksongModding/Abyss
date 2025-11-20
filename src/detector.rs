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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{App, Library};
    use tempfile::tempdir;
    use tracing_test::traced_test;

    struct FakeProvider {
        app: Option<(App, Library)>,
        libs: Vec<Library>,
    }

    impl SteamProvider for FakeProvider {
        fn find_app(&self, _app_id: u32) -> Result<Option<(App, Library)>> {
            Ok(self.app.clone())
        }

        fn libraries(&self) -> Result<Vec<Library>> {
            Ok(self.libs.clone())
        }
    }

    #[test]
    fn explicit_override_wins() {
        let dir = tempdir().unwrap();
        let provider = FakeProvider {
            app: None,
            libs: vec![],
        };
        let detector = Detector::new(provider);
        let got = detector
            .detect_game_dir(Some(dir.path()), None, &[])
            .unwrap();
        assert_eq!(got, dir.path());
    }



    #[test]
    fn finds_by_app_id_from_provider() {
        // create a fake library with steamapps/common/Game
        let lib_dir = tempdir().unwrap();
        let common = lib_dir.path().join("steamapps").join("common");
        std::fs::create_dir_all(common.join("GameName")).unwrap();

        let app = App {
            install_dir: "GameName".into(),
        };
        let lib = Library {
            path: lib_dir.path().to_path_buf(),
        };
        let provider = FakeProvider {
            app: Some((app.clone(), lib.clone())),
            libs: vec![lib.clone()],
        };

        let detector = Detector::new(provider);
        let got = detector.detect_game_dir(None, Some(123), &[]).unwrap();
        assert!(
            got.ends_with(
                std::path::Path::new("steamapps")
                    .join("common")
                    .join("GameName")
            )
        );
    }

    #[test]
    fn finds_by_name_hint() {
        let lib_dir = tempdir().unwrap();
        let common = lib_dir.path().join("steamapps").join("common");
        std::fs::create_dir_all(common.join("HappyGame")).unwrap();

        let provider = FakeProvider {
            app: None,
            libs: vec![Library {
                path: lib_dir.path().to_path_buf(),
            }],
        };
        let detector = Detector::new(provider);

        let hints = vec!["HappyGame".to_string()];
        let got = detector.detect_game_dir(None, None, &hints).unwrap();
        assert!(
            got.ends_with(
                std::path::Path::new("steamapps")
                    .join("common")
                    .join("HappyGame")
            )
        );
    }

    #[test]
    #[traced_test]
    fn provider_find_app_error_is_logged() {
        struct ErrProvider;
        impl SteamProvider for ErrProvider {
            fn find_app(&self, _app_id: u32) -> Result<Option<(App, Library)>> {
                Err(anyhow::anyhow!("boom"))
            }

            fn libraries(&self) -> Result<Vec<Library>> {
                Ok(vec![])
            }
        }

        let detector = Detector::new(ErrProvider);
        let _ = detector.detect_game_dir(None, Some(1), &[]);
        logs_assert(|lines: &[&str]| {
            if lines.iter().any(|l| l.contains("Provider find_app failed")) {
                Ok(())
            } else {
                Err(format!(
                    "logs did not contain Provider find_app failed: {:?}",
                    lines
                ))
            }
        });
    }

    #[test]
    fn broken_library_entries_are_skipped() {
        // library exists but steamapps/common is not present
        let lib_dir = tempdir().unwrap();
        let provider = FakeProvider {
            app: None,
            libs: vec![Library {
                path: lib_dir.path().to_path_buf(),
            }],
        };
        let detector = Detector::new(provider);

        let res = detector.detect_game_dir(None, None, &[]);
        assert!(res.is_err());
    }
}
