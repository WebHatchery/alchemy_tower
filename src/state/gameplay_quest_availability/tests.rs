use crate::alchemy::MASTERED_BREW_COUNT;
use crate::state::gameplay::GameplayState;

/// The gate is the point of the field, so drive it rather than trusting the
/// expression: a request wanting a mastered formula must stay shut at six
/// clean brews and open at seven, and must say which formula while shut.
#[test]
fn a_mastery_gated_request_waits_for_the_seventh_brew() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let quest = data
        .quests
        .iter()
        .find(|quest| !quest.required_mastered_recipe.is_empty())
        .expect("some request should ask for a mastered formula");
    let recipe_id = quest.required_mastered_recipe.clone();
    let recipe_name = data
        .recipe(&recipe_id)
        .expect("the guard checks this resolves")
        .name
        .clone();

    let mut state = GameplayState::new(&data);
    state.progression.total_brews = 999;
    for prerequisite in &quest.prerequisite_quests {
        state
            .progression
            .completed_quests
            .insert(prerequisite.clone());
    }

    state
        .progression
        .recipe_mastery
        .insert(recipe_id.clone(), MASTERED_BREW_COUNT - 1);
    assert!(
        !state.quest_is_available(quest),
        "six clean brews is not mastery"
    );
    let locked = state.quest_unlock_summary(&data, quest);
    assert!(
        locked.contains(&recipe_name),
        "a shut request should name the formula it is waiting on: {locked}"
    );

    state
        .progression
        .recipe_mastery
        .insert(recipe_id, MASTERED_BREW_COUNT);
    assert!(
        state.quest_is_available(quest),
        "the seventh brew should open it"
    );
}

/// The field is new, and a new gate field is exactly what this project keeps
/// getting wrong — a key with no reader looks configured and opens nothing.
/// So drive it: a request waiting on a beat must stay shut until the beat is
/// recorded, must open the moment it is, and must name the beat by its
/// *title* while shut rather than leaking the id.
#[test]
fn a_beat_gated_request_waits_for_the_beat() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let quest = data
        .quests
        .iter()
        .find(|quest| !quest.required_journal_milestone.is_empty())
        .expect("some request should wait on a journal beat");
    let beat = quest.required_journal_milestone.clone();
    let title = super::beat_title(&data, &beat);
    assert_ne!(title, beat, "the beat should be authored with a title");

    let mut state = GameplayState::new(&data);
    state.progression.total_brews = 999;
    for prerequisite in &quest.prerequisite_quests {
        state
            .progression
            .completed_quests
            .insert(prerequisite.clone());
    }
    if !quest.required_mastered_recipe.is_empty() {
        state.progression.recipe_mastery.insert(
            quest.required_mastered_recipe.clone(),
            crate::alchemy::MASTERED_BREW_COUNT,
        );
    }

    assert!(
        !state.quest_is_available(quest),
        "{} is on offer before its beat is recorded",
        quest.id
    );
    let locked = state.quest_unlock_summary(&data, quest);
    assert!(
        locked.contains(&title),
        "a shut request should name the beat it waits on: {locked}"
    );

    state.push_journal_milestone(&beat, &title, "recorded by the test");
    assert!(
        state.quest_is_available(quest),
        "recording the beat should open it"
    );
}
