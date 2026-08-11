use super::GameplayState;
use crate::alchemy::resolve_brew;

/// Working a formula out for the first time now writes to the journal, the
/// way finishing a quest always has. This walks the real path — resolve the
/// brew, hand the resolution to the outcome code — rather than trusting the
/// call site by eye, and checks the second brew does not record it twice.
#[test]
fn working_out_a_formula_is_written_into_the_journal() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let recipe = data
        .recipes
        .iter()
        .find(|recipe| !recipe.discovery_milestones.is_empty())
        .expect("some recipe should record a discovery");
    let milestone_id = recipe.discovery_milestones[0].id.clone();
    let station = data
        .stations
        .iter()
        .find(|station| station.id == recipe.station_id)
        .expect("the recipe's bench should exist");

    let mut selected = Vec::new();
    for entry in &recipe.ingredients {
        for _ in 0..entry.amount {
            selected.push(entry.item_id.clone());
        }
    }

    let mut state = GameplayState::new(&data);
    assert!(
        !state.has_journal_milestone(&milestone_id),
        "a new game should not already know this"
    );

    let resolution = resolve_brew(
        &data,
        station,
        &selected,
        &state.brew_ingredients(&data, &selected),
        None,
        recipe.required_heat,
        recipe.required_stirs,
        &recipe.required_timing,
        0,
    );
    assert_eq!(
        resolution.recipe.map(|matched| matched.id.as_str()),
        Some(recipe.id.as_str()),
        "the ingredients should match the recipe they came from"
    );
    state.update_brew_result_status(&data, &resolution, true);

    assert!(
        state.has_journal_milestone(&milestone_id),
        "discovering {} should record {milestone_id}",
        recipe.id
    );

    let before = state.progression.journal_milestones.len();
    state.update_brew_result_status(&data, &resolution, true);
    assert_eq!(
        state.progression.journal_milestones.len(),
        before,
        "brewing it again should not record the discovery a second time"
    );
}
