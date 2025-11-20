use abyss::bepinex::{BepInExInstallation, REQUIRED_SUBFOLDERS};
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
