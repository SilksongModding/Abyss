use abyss::detector::Detector;
use abyss::provider::{App, Library, SteamProvider};
use anyhow::Result;
use tempfile::tempdir;


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

/*
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
*/

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
