use super::GameplayState;
use crate::data::EffectKind;

/// `Glow` was one of four effect kinds and the only thing it did was tint
/// the player sprite. Twenty-one gather nodes appear only in the dark, and
/// they were picked as easily at midnight as at noon.
#[test]
fn the_dark_needs_a_light_and_a_glow_brew_is_one() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    state.set_clock_minutes(13.0 * 60.0);
    assert_eq!(state.current_time_window(), "day");
    assert!(state.can_see_to_gather(&data), "daylight needs no help");

    state.set_clock_minutes(23.0 * 60.0);
    assert_eq!(state.current_time_window(), "night");
    assert!(!state.can_see_to_gather(&data), "midnight is dark");

    let potion = data
        .item("glow_potion")
        .expect("the glow potion should exist");
    let glow = potion
        .effects
        .iter()
        .find(|effect| effect.kind == EffectKind::Glow)
        .expect("a glow potion should glow");
    state.apply_effect(glow);
    assert!(
        state.can_see_to_gather(&data),
        "a lit brew should buy the night shift"
    );
}

/// Which windows are dark is a design knob, not a fact about the clock.
#[test]
fn the_dark_hours_are_read_from_the_data() {
    let mut data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    state.set_clock_minutes(13.0 * 60.0);
    assert!(state.can_see_to_gather(&data));

    data.config
        .balance
        .gathering
        .dark_time_windows
        .push("day".to_owned());
    assert!(
        !state.can_see_to_gather(&data),
        "adding a window to the dark list changed nothing"
    );
}

/// A light is a convenience, not a wall. Every ingredient the game needs
/// should be reachable by somebody who never brews a glow potion — nodes
/// that only appear at night are the ones to watch, since they are the only
/// ones the darkness rule can lock away entirely.
#[test]
fn nothing_is_reachable_only_by_lamplight() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let dark = &data.config.balance.gathering.dark_time_windows;

    let mut night_only = std::collections::BTreeSet::new();
    let mut reachable_in_the_light = std::collections::BTreeSet::new();
    for area in &data.areas {
        for node in &area.gather_nodes {
            let always_dark = !node.time_windows.is_empty()
                && node.time_windows.iter().all(|window| dark.contains(window));
            if always_dark {
                night_only.insert(node.item_id.clone());
            } else {
                reachable_in_the_light.insert(node.item_id.clone());
            }
        }
    }

    // Shops and gifts count as light-hours sources too.
    for station in &data.stations {
        reachable_in_the_light.extend(station.stock.iter().map(|s| s.item_id.clone()));
    }

    let stranded = night_only
        .difference(&reachable_in_the_light)
        .cloned()
        .collect::<Vec<_>>();

    // Anything genuinely night-only has to be brewable *into* by a formula
    // that a glow potion itself does not require, or the rule bootstraps
    // badly. The glow potion's own reagents are the ones that matter.
    let glow = data
        .recipes
        .iter()
        .find(|recipe| recipe.id == "glow_potion_recipe")
        .expect("the glow potion recipe should exist");
    let blocked_glow = glow
        .ingredients
        .iter()
        .filter(|ingredient| stranded.contains(&ingredient.item_id))
        .map(|ingredient| ingredient.item_id.clone())
        .collect::<Vec<_>>();

    assert!(
        blocked_glow.is_empty(),
        "the glow potion needs {blocked_glow:?}, which can only be gathered by glow-light"
    );
}
