use abyss::installer::install_bepinex;
use std::fs;
use tempfile::tempdir;
use zip::write::FileOptions;

#[test]
fn test_install_bepinex_extracts_files() {
    // 1. Create a mock zip file
    let temp_dir = tempdir().unwrap();
    let zip_path = temp_dir.path().join("bepinex.zip");
    let game_dir = temp_dir.path().join("Game");
    fs::create_dir(&game_dir).unwrap();

    let file = fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Stored);

    // Add BepInEx/core/BepInEx.dll
    zip.start_file("BepInEx/core/BepInEx.dll", options).unwrap();
    use std::io::Write;
    zip.write_all(b"fake dll content").unwrap();

    // Add doorstop_config.ini
    zip.start_file("doorstop_config.ini", options).unwrap();
    zip.write_all(b"fake config").unwrap();

    zip.finish().unwrap();

    // 2. Run install
    install_bepinex(&zip_path, &game_dir).unwrap();

    // 3. Verify files exist in game_dir
    assert!(game_dir.join("BepInEx/core/BepInEx.dll").exists());
    assert!(game_dir.join("doorstop_config.ini").exists());
    
    let dll_content = fs::read(game_dir.join("BepInEx/core/BepInEx.dll")).unwrap();
    assert_eq!(dll_content, b"fake dll content");
}
