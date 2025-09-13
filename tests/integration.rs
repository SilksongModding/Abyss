use abyss::detector::Detector;
use abyss::provider::RealSteamProvider;

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
