use std::path::Path;

use super::*;

#[test]
fn required_texture_manifest_files_exist() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let manifest = build_texture_manifest(&data);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let missing = manifest
        .texture_configs
        .iter()
        .filter(|config| !root.join(&config.path).exists())
        .map(|config| format!("{} -> {}", config.key, config.path))
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "missing required texture files:\n{}",
        missing.join("\n")
    );
}
