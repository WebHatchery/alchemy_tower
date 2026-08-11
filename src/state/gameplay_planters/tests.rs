use super::planter_accepts;
use super::GameplayState;

/// Tending used to be spent by the clock. `tend_or_report_planter` added a
/// day of growth, and the midnight rollover then recomputed the same field
/// from elapsed time alone and threw the visit away — so the beds ripened
/// on a pure timer and turning up changed nothing past the current day.
/// The two models are composed now: elapsed time is the floor, and each
/// day tended is worth a day on top of it. This walks a real bed across a
/// rollover to prove the visit survives.
#[test]
fn a_tended_bed_stays_ahead_of_one_left_alone() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let station = data
        .stations
        .iter()
        .find(|station| station.planter_harvest_days >= 3)
        .expect("some bed should take more than a couple of days");

    let mut state = GameplayState::new(&data);
    let seed = station
        .planter_seed_ids
        .first()
        .cloned()
        .expect("a specialised bed names its seeds");
    state.inventory.insert(seed.clone(), 1);
    state.interact_with_planter(&data, station);
    assert_eq!(
        state.progression.planter_states[&station.id].planted_item_id, seed,
        "the seed should have gone in"
    );

    // Tend on the day it was planted, then let midnight pass.
    state.interact_with_planter(&data, station);
    assert_eq!(state.progression.planter_states[&station.id].tended_days, 1);
    state.world.day_index += 1;
    state.advance_planters(&data);

    let tended = state.progression.planter_states[&station.id].growth_days;
    assert_eq!(
        tended, 2,
        "one day elapsed plus one day tended should survive the rollover"
    );

    // The same bed, same elapsed time, never visited.
    let mut untended = GameplayState::new(&data);
    untended.inventory.insert(seed, 1);
    untended.interact_with_planter(&data, station);
    untended.world.day_index += 1;
    untended.advance_planters(&data);
    assert_eq!(
        untended.progression.planter_states[&station.id].growth_days, 1,
        "an untended bed should still grow, just slower"
    );
    assert!(tended > untended.progression.planter_states[&station.id].growth_days);
}

/// A bed lists what it accepts, and that list is shown to the player. Any id
/// on it that the bed would in fact refuse is a promise the game breaks the
/// moment someone tries it.
#[test]
fn every_advertised_seed_is_one_the_bed_will_take() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut refused = Vec::new();

    for station in &data.stations {
        for seed_id in &station.planter_seed_ids {
            match data.item(seed_id) {
                None => refused.push(format!("{} -> no item {seed_id}", station.id)),
                Some(item) if !planter_accepts(station, item, seed_id) => refused.push(format!(
                    "{} advertises {seed_id} and refuses it",
                    station.id
                )),
                Some(_) => {}
            }
        }
    }

    assert!(
        refused.is_empty(),
        "beds that lie about their seeds:\n{refused:#?}"
    );
}

/// Planting is a two-step trade: a seed in the bed, then a potion to steer
/// what it becomes. A seed with no mutation formula can be grown but never
/// steered, which is fine; a formula for a seed no bed accepts is content
/// that can never be reached.
#[test]
fn every_mutation_formula_has_a_bed_that_grows_its_seed() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let unreachable = data
        .mutation_formulas
        .iter()
        .filter(|formula| {
            !data.stations.iter().any(|station| {
                data.item(&formula.seed_item_id)
                    .map(|item| planter_accepts(station, item, &formula.seed_item_id))
                    .unwrap_or(false)
            })
        })
        .map(|formula| format!("{} seeds {}", formula.id, formula.seed_item_id))
        .collect::<Vec<_>>();

    assert!(
        unreachable.is_empty(),
        "mutation formulas whose seed no bed will take:\n{unreachable:#?}"
    );
}
