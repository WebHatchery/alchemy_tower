use super::GameplayState;
use crate::alchemy::resolve_brew;
use crate::data::{GameData, StationDefinition};

/// A mixture the game has no recipe for, at a bench that will take it.
/// Found rather than named, so a new recipe covering the pair cannot quietly
/// turn these tests into tests of the written-recipe path instead.
fn an_off_book_mixture(data: &GameData) -> (&StationDefinition, Vec<String>) {
    let station = data
        .stations
        .iter()
        .find(|station| station.id == "entry_cauldron")
        .expect("the entry cauldron should exist");
    let reagents = data
        .items
        .iter()
        .filter(|item| item.category == crate::data::ItemCategory::Ingredient)
        .filter(|item| item.quality > 0)
        .map(|item| item.id.clone())
        .collect::<Vec<_>>();

    for left in &reagents {
        for right in &reagents {
            let selected = vec![left.clone(), right.clone()];
            if crate::alchemy::match_recipe(data, station, &selected).is_none() {
                return (station, selected);
            }
        }
    }
    panic!("every pair of reagents is already a recipe");
}

fn brew(state: &mut GameplayState, data: &GameData) -> bool {
    let (station, selected) = an_off_book_mixture(data);
    let ingredients = state.brew_ingredients(data, &selected);
    let resolution = resolve_brew(
        data,
        station,
        &selected,
        &ingredients,
        None,
        2,
        2,
        "steady",
        state.salvage_familiarity(station, &selected),
    );
    state.record_salvage_attempt(data, station, &selected, &resolution)
}

/// Brewing something nobody wrote down used to be a dead end that answered
/// "not that" however many times you tried it. The third clean attempt is
/// now a find, and the journal says so.
#[test]
fn a_mixture_made_three_times_becomes_a_formula_you_found() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (station, selected) = an_off_book_mixture(&data);

    for attempt in 1..data.config.balance.salvage.discovery_attempts {
        assert!(!brew(&mut state, &data), "attempt {attempt} is not a find");
        assert!(!state.salvage_is_discovered(&data, station, &selected));
    }

    assert!(brew(&mut state, &data), "the third attempt should land");
    assert!(state.salvage_is_discovered(&data, station, &selected));
    let signature = GameplayState::salvage_signature(station, &selected);
    assert!(state.has_journal_milestone(&format!("found_formula_{signature}")));

    // And it is a find once, not every time afterwards.
    assert!(!brew(&mut state, &data));
}

/// Reagent order is how the player loads the pot, not what they made.
#[test]
fn the_same_reagents_in_another_order_are_the_same_mixture() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let station = data
        .stations
        .iter()
        .find(|station| station.id == "entry_cauldron")
        .expect("bench");
    assert_eq!(
        GameplayState::salvage_signature(
            station,
            &["whisper_moss".to_owned(), "sunleaf".to_owned()]
        ),
        GameplayState::salvage_signature(
            station,
            &["sunleaf".to_owned(), "whisper_moss".to_owned()]
        )
    );
}

/// Familiarity has to reach the brewer, or the discovery is a journal line
/// about nothing. A mixture the player has worked out comes out better than
/// the first blind attempt at it.
#[test]
fn a_formula_you_worked_out_brews_better_than_a_blind_attempt() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (station, selected) = an_off_book_mixture(&data);
    let ingredients = state.brew_ingredients(&data, &selected);

    let blind = resolve_brew(
        &data,
        station,
        &selected,
        &ingredients,
        None,
        2,
        2,
        "steady",
        0,
    );
    for _ in 0..data.config.balance.salvage.discovery_attempts {
        brew(&mut state, &data);
    }
    let practised = resolve_brew(
        &data,
        station,
        &selected,
        &ingredients,
        None,
        2,
        2,
        "steady",
        state.salvage_familiarity(station, &selected),
    );

    assert!(
        practised.quality_score > blind.quality_score,
        "working a mixture out changed nothing: {} vs {}",
        practised.quality_score,
        blind.quality_score
    );
}

/// Tuning that lives in a file but is not actually read is worse than a
/// constant, because the file says it is configurable and turning the knob
/// does nothing. Move the numbers and check the game moves with them.
#[test]
fn the_salvage_curve_is_read_from_the_data_rather_than_baked_in() {
    let mut data = crate::data::load_embedded().expect("embedded game data should load");
    let (station, selected) = an_off_book_mixture(&data);
    let station = station.clone();
    let mut state = GameplayState::new(&data);
    let ingredients = state.brew_ingredients(&data, &selected);

    let stock = resolve_brew(
        &data,
        &station,
        &selected,
        &ingredients,
        None,
        2,
        2,
        "steady",
        99,
    )
    .quality_score;

    // The per-attempt bonus rather than the cap: this mixture scores well
    // under the cap, so raising the ceiling alone would prove nothing.
    data.config.balance.salvage.bonus_per_attempt += 5;
    let richer = resolve_brew(
        &data,
        &station,
        &selected,
        &ingredients,
        None,
        2,
        2,
        "steady",
        99,
    )
    .quality_score;
    assert!(
        richer > stock,
        "raising the practice bonus changed nothing: {richer} vs {stock}"
    );

    // And the discovery threshold, which is read on a different path.
    data.config.balance.salvage.discovery_attempts = 1;
    assert!(
        brew(&mut state, &data),
        "one attempt should be a find when the data says one attempt"
    );
}
