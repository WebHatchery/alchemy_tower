use std::path::Path;

use super::*;

#[test]
fn required_audio_variation_files_exist() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut missing = Vec::new();

    for (base_name, count) in REQUIRED_VARIATION_SETS {
        for index in 1..=*count {
            let path = audio_loading::variation_path(base_name, index);
            if !root.join(&path).exists() {
                missing.push(path);
            }
        }
    }

    assert!(
        missing.is_empty(),
        "missing required audio files:\n{}",
        missing.join("\n")
    );
}

#[test]
fn area_footstep_sound_sets_are_known() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let known_sets = [
        "dirt_path",
        "gravel",
        "greenhouse",
        "leaf",
        "sand",
        "shore",
        "stone",
    ];
    let unknown = data
        .areas
        .iter()
        .filter(|area| !known_sets.contains(&area.footstep_sound_set.as_str()))
        .map(|area| format!("{} -> {}", area.id, area.footstep_sound_set))
        .collect::<Vec<_>>();

    assert!(
        unknown.is_empty(),
        "unknown area footstep sound set(s):\n{}",
        unknown.join("\n")
    );
}

/// A footstep set that is synthesised, shipped and then assigned to no area
/// is dead weight in the asset pack. This nearly happened in reverse: the
/// outdoor set looked unused because seven areas name no set at all, and it
/// took reading the schema default to find they were using it all along.
#[test]
fn every_footstep_set_is_actually_walked_on() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut used = std::collections::HashSet::new();
    for area in &data.areas {
        used.insert(area.footstep_sound_set.clone());
    }

    let unused = REQUIRED_VARIATION_SETS
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| name.starts_with("footstep_"))
        .map(|name| name.trim_start_matches("footstep_"))
        .filter(|set| !used.contains(*set))
        .collect::<Vec<_>>();

    assert!(
        unused.is_empty(),
        "footstep sets nothing walks on: {unused:?} (areas use {used:?})"
    );
}
