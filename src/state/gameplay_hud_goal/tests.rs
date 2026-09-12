use super::GameplayState;

#[test]
fn the_first_quest_hud_guidance_names_the_recipe_and_ingredients() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    state
        .progression
        .started_quests
        .insert("healing_for_mira".to_owned());

    let goal = state.hud_goal(&data);
    assert!(goal.action.contains("Healing Draught"));
    assert!(goal.action.contains("Sunleaf"));
    assert!(goal.action.contains("Whisper Moss"));
    assert!(goal.action.contains("cauldron"));
}

#[test]
fn the_first_quest_recipe_guidance_resolves_after_brewing_starts_progress() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    state
        .progression
        .started_quests
        .insert("healing_for_mira".to_owned());
    state.progression.total_brews = 1;

    let goal = state.hud_goal(&data);
    assert!(
        !goal.action.contains("Open the cauldron"),
        "the first-brew recipe prompt should not persist after brewing"
    );
}
