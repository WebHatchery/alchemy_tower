use super::GameplayState;

#[test]
fn mastery_gate_blocks_until_recipe_is_mastered() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    let warp = data
        .areas
        .iter()
        .flat_map(|area| area.warps.iter())
        .find(|warp| warp.id == "containment_to_rune_workshop")
        .expect("rune workshop warp should exist")
        .clone();
    assert_eq!(warp.required_mastered_recipe, "glow_potion_recipe");

    // Pay the coin cost but leave the recipe unmastered: still locked.
    state.coins = warp.required_coins + 10;
    assert!(!state.can_unlock_warp(&warp));
    assert!(!state.warp_is_unlocked(&warp));

    // Reaching the mastered threshold opens the gate.
    state.progression.recipe_mastery.insert(
        "glow_potion_recipe".to_owned(),
        crate::alchemy::MASTERED_BREW_COUNT,
    );
    assert!(state.can_unlock_warp(&warp));
}

/// The Southern Pass authored a `required_completed_quest` that no schema
/// field claimed, so serde dropped it and the switchback was walkable from
/// minute one — which also meant `restore_warp_route` never ran and the
/// `pass_road_open` milestone behind three NPC lines was never recorded.
/// This pins both halves: locked at a new game, and opening the quest both
/// unlocks the route and files the milestone.
#[test]
fn a_story_gated_warp_stays_shut_until_its_quest_is_done() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    let warp = data
        .areas
        .iter()
        .flat_map(|area| area.warps.iter())
        .find(|warp| warp.id == "town_to_pass")
        .expect("southern pass warp should exist")
        .clone();
    assert_eq!(warp.required_completed_quest, "nightwatch_for_elric");

    assert!(!state.warp_is_unlocked(&warp));
    assert!(!state.can_unlock_warp(&warp));

    state
        .progression
        .completed_quests
        .insert(warp.required_completed_quest.clone());
    assert!(state.can_unlock_warp(&warp));

    state.handle_warp_interaction(&data, &warp);
    assert!(state.warp_is_unlocked(&warp));
    assert!(
        state.has_journal_milestone("pass_road_open"),
        "restoring the pass should record the milestone its NPC lines wait on"
    );
}
