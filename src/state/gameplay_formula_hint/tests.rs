use super::{GameplayState, ReagentOrigin};

/// A hint that sends the player to ground still waiting on a quest is worse
/// than no hint, because they cannot tell the difference between "not here"
/// and "not yet". Naming the first area in file order did exactly that:
/// whisper moss grows in seven places and the terraces under the tower wall
/// — which open at the end of Brin's arc — sorted ahead of the plains.
#[test]
fn a_reagent_is_pointed_at_ground_that_is_already_open() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    let mut shut = Vec::new();
    for item in &data.items {
        let open_somewhere = state.area_growing(&data, &item.id, true).is_some();
        if !open_somewhere {
            continue;
        }
        let Some(ReagentOrigin::Ground(named)) = state.reagent_origin(&data, &item.id) else {
            continue;
        };
        let area = data
            .areas
            .iter()
            .find(|area| area.name == named)
            .expect("the hint should name a real area");
        let workable = area.gather_nodes.iter().any(|node| {
            node.item_id == item.id
                && state.story_gate_is_open(&node.required_completed_quest, true)
                && state.story_gate_is_open(&node.required_journal_milestone, false)
        });
        if !workable {
            shut.push(format!("{} sent to {named}", item.id));
        }
    }

    assert!(
        shut.is_empty(),
        "reagents pointed at ground that is still shut: {shut:#?}"
    );
}

/// The rule this pass exists to make true: every reagent in the game, at a
/// new game, points somewhere for at least one of the formulae it feeds.
/// A reagent whose origin cannot be named is one the journal would send the
/// player looking for with no direction, which is the state the whole shelf
/// was in.
#[test]
fn every_undiscovered_formula_a_reagent_feeds_points_somewhere() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    let mut silent = Vec::new();
    for item in &data.items {
        let feeds = data.recipes.iter().any(|recipe| {
            recipe
                .ingredients
                .iter()
                .any(|ingredient| ingredient.item_id == item.id)
        });
        if !feeds {
            continue;
        }
        if state.undiscovered_formula_hint(&data, &item.id).is_none() {
            silent.push(item.id.clone());
        }
    }

    assert!(
        silent.is_empty(),
        "reagents whose unlogged formulae point nowhere: {silent:#?}"
    );
}

/// The hint must not become the answer. It says where the missing half
/// comes from and never what it is or what the formula is called.
#[test]
fn a_hint_names_neither_the_formula_nor_the_reagent_it_wants() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    for item in &data.items {
        let Some(hint) = state.undiscovered_formula_hint(&data, &item.id) else {
            continue;
        };
        for recipe in &data.recipes {
            assert!(
                !hint.contains(&recipe.name),
                "{}'s hint names the formula {}: {hint}",
                item.id,
                recipe.name
            );
        }
        for other in &data.items {
            if other.id == item.id || state.has_met_reagent(&other.id) {
                continue;
            }
            assert!(
                !hint.contains(&other.name),
                "{}'s hint names {}: {hint}",
                item.id,
                other.name
            );
        }
    }
}

/// Once the walking is done the hint has to stop sending the player out
/// and start naming the bench. This is not an edge case: a player who has
/// been round the valley has met most of the shelf, so for most of the game
/// this is the branch they read, and pointing them at ground they have
/// already worked would be the old useless count in a longer sentence.
#[test]
fn a_formula_whose_reagents_are_all_met_names_the_bench_instead() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    // Forget the starter formulae, so the healing draught is unlogged and
    // this measures the hint rather than the seeding.
    state.progression.known_recipes.clear();

    let away = state
        .undiscovered_formula_hint(&data, "whisper_moss")
        .expect("whisper moss feeds unlogged formulae");

    for item in &data.items {
        state.inventory.insert(item.id.clone(), 1);
    }
    let home = state
        .undiscovered_formula_hint(&data, "whisper_moss")
        .expect("whisper moss still feeds unlogged formulae");

    assert_ne!(away, home, "the hint ignored what the player has met");
    // Found in the data rather than spelled out: the bench named has to be
    // one that really brews an unlogged whisper moss formula, so renaming a
    // station or moving a recipe cannot turn this into a test of nothing.
    let benches = data
        .recipes
        .iter()
        .filter(|recipe| {
            recipe
                .ingredients
                .iter()
                .any(|ingredient| ingredient.item_id == "whisper_moss")
        })
        .filter_map(|recipe| {
            data.stations
                .iter()
                .find(|station| station.id == recipe.station_id)
        })
        .map(|station| station.name.as_str())
        .collect::<Vec<_>>();
    assert!(
        benches.iter().any(|bench| home.contains(bench)),
        "everything met should point at a bench that brews it: {home}"
    );
}
