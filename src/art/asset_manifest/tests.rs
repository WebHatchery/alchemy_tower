use std::fs;
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

#[test]
fn character_sheets_are_complete_320_by_256_rgba_sheets() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut ids = data
        .npcs
        .iter()
        .map(|npc| npc.id.clone())
        .collect::<Vec<_>>();
    ids.extend([PLAYER_ID, FEMALE_PLAYER_ID].into_iter().map(str::to_owned));

    let mut failures = Vec::new();
    for id in ids {
        let path = root.join(format!("assets/art/characters/{id}.png"));
        match png_dimensions(&path) {
            Ok((width, height, color_type)) if (width, height) == (320, 256) && color_type == 6 => {
            }
            Ok((width, height, color_type)) => failures.push(format!(
                "{id}: expected 320x256 RGBA, got {width}x{height} color type {color_type}"
            )),
            Err(error) => failures.push(format!("{id}: {error}")),
        }
    }

    assert!(
        failures.is_empty(),
        "invalid character sheets:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_placed_herb_has_a_64_by_64_rgba_world_sprite() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sprite_ids = data
        .areas
        .iter()
        .flat_map(|area| area.gather_nodes.iter())
        .map(|node| {
            if node.render.sprite_id.is_empty() {
                node.item_id.clone()
            } else {
                node.render.sprite_id.clone()
            }
        })
        .collect::<Vec<_>>();
    sprite_ids.sort();
    sprite_ids.dedup();

    let mut failures = Vec::new();
    for id in sprite_ids {
        let path = root.join(format!("assets/art/items/world/{id}.png"));
        match png_dimensions(&path) {
            Ok((width, height, color_type)) if (width, height) == (64, 64) && color_type == 6 => {}
            Ok((width, height, color_type)) => failures.push(format!(
                "{id}: expected 64x64 RGBA, got {width}x{height} color type {color_type}"
            )),
            Err(error) => failures.push(format!("{id}: {error}")),
        }
    }

    assert!(failures.is_empty(), "invalid herb sprites:\n{}", failures.join("\n"));
}

fn png_dimensions(path: &Path) -> Result<(u32, u32, u8), String> {
    let bytes =
        fs::read(path).map_err(|error| format!("unable to read {}: {error}", path.display()))?;
    if bytes.len() < 29 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return Err("not a PNG with an IHDR header".to_owned());
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    let color_type = bytes[25];
    Ok((width, height, color_type))
}
