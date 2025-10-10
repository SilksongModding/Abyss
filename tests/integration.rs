use abyss::bepinex_provider::{
    BepinexProvider, LinuxBepinexProvider, MacOSBepinexProvider, WindowsBepinexProvider,
};
use abyss::detector::Detector;
use abyss::provider::RealSteamProvider;
use tempfile::tempdir;

// Runs only when ENABLE_INTEGRATION_TESTS=1 is set in the environment
#[test]
fn real_provider_integration() {
    if std::env::var("ENABLE_INTEGRATION_TESTS").unwrap_or_default() != "1" {
        eprintln!("Skipping integration test; set ENABLE_INTEGRATION_TESTS=1 to run");
        return;
    }

    let detector = Detector::new(RealSteamProvider);
    // This may fail if Steam isn't installed on the CI machine; that's OK for an optional test
    let res = detector.detect_game_dir(None, None, &[]);
    println!("Integration result: {:?}", res);
    // We don't assert success, just ensure the call doesn't panic
}

#[test]
fn bepinex_installation_integration_linux() {
    let temp_dir = tempdir().unwrap();
    let game_path = temp_dir.path().to_path_buf();

    let provider = LinuxBepinexProvider { game_path };
    provider.install().unwrap();

    assert!(provider.check_installation().unwrap());
}

#[test]
fn bepinex_installation_integration_windows() {
    let temp_dir = tempdir().unwrap();
    let game_path = temp_dir.path().to_path_buf();

    let provider = WindowsBepinexProvider { game_path };
    provider.install().unwrap();

    assert!(provider.check_installation().unwrap());
}

#[test]
fn bepinex_installation_integration_macos() {
    let temp_dir = tempdir().unwrap();
    let game_path = temp_dir.path().to_path_buf();

    let provider = MacOSBepinexProvider { game_path };
    provider.install().unwrap();

    assert!(provider.check_installation().unwrap());
}
