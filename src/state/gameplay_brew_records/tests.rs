use super::GameplayState;
use crate::alchemy::resolve_brew;

/// An overcharge collapse is the one failure the log used to misfile: it
/// spelled the stability rule out itself and left `destabilized` off, so a
/// brew that visibly collapsed into its unstable output was archived as
/// clean. Potion memory is rebuilt from this log whenever a save loads with
/// no memories, so the lie survived the session it was told in.
#[test]
fn an_overcharge_collapse_is_logged_as_the_failure_it_was() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    // Any formula that brews clean on spec will do — what matters is that
    // the *only* thing wrong with the overcharged version is the collapse.
    // Some recipes want a catalyst or a reagent order this test does not
    // set up, so ask the brewer which one it can make cleanly.
    let (station, recipe, selected) = data
        .recipes
        .iter()
        .filter_map(|recipe| {
            let station = data
                .stations
                .iter()
                .find(|station| station.id == recipe.station_id)?;
            let selected = recipe
                .ingredients
                .iter()
                .map(|ingredient| ingredient.item_id.clone())
                .collect::<Vec<_>>();
            let on_spec = resolve_brew(
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
            on_spec.is_stable().then_some((station, recipe, selected))
        })
        .next()
        .expect("some recipe should brew clean when run to its own spec");

    // Stirred far past the requirement: the numbers are better than on spec
    // and the process still matches, which is exactly the shape the old
    // rule read as stable.
    let resolution = resolve_brew(
        &data,
        station,
        &selected,
        &state.brew_ingredients(&data, &selected),
        None,
        recipe.required_heat,
        recipe.required_stirs + 20,
        &recipe.required_timing,
        0,
    );
    assert!(resolution.destabilized, "the setup should collapse");
    assert!(resolution.process_match, "the process itself was not wrong");
    assert!(resolution.minimum_quality_met && resolution.minimum_elements_met);
    assert!(!resolution.is_stable());

    state.record_brew_inventory_result(&data, &resolution, resolution.is_stable());
    let logged = state
        .progression
        .experiment_log
        .last()
        .expect("the brew should be logged");
    assert!(
        !logged.stable,
        "a collapse filed as a stable brew in the archive"
    );

    // What a save/load does: memories are dropped and rebuilt from the log.
    let logged_output = logged.output_item_id.clone();
    state.progression.potion_memories.clear();
    state.rebuild_memory_state(&data);
    assert_eq!(
        state
            .progression
            .potion_memories
            .get(&logged_output)
            .map(|memory| memory.successful_brews),
        Some(0),
        "reloading turned a collapse into a successful brew"
    );
}
