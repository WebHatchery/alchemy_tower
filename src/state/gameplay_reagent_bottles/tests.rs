use super::GameplayState;
use crate::data::{BottleBatchEntry, GameData, ItemCategory};

/// A potion that some recipe asks for as a reagent, so these tests follow
/// the content rather than pinning one bottle by name.
fn a_bottle_used_as_a_reagent(data: &GameData) -> String {
    data.recipes
        .iter()
        .flat_map(|recipe| recipe.ingredients.iter())
        .map(|ingredient| ingredient.item_id.clone())
        .find(|item_id| {
            data.item(item_id)
                .is_some_and(|item| item.category == ItemCategory::Potion)
        })
        .expect("some recipe should call for a finished bottle")
}

fn batch(item_id: &str, band: &str, score: u32, count: u32) -> BottleBatchEntry {
    BottleBatchEntry {
        item_id: item_id.to_owned(),
        quality_score: score,
        quality_band: band.to_owned(),
        traits: vec!["luminous".to_owned()],
        count,
    }
}

/// The gap the late tier shipped with: every potion leaves `quality` unset,
/// so a poured bottle was worth the schema default of 20 whether it was
/// Crude or Masterwork, and brewing the input well bought nothing.
#[test]
fn the_bottle_you_poured_is_the_bottle_the_brew_gets() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let item_id = a_bottle_used_as_a_reagent(&data);
    let selected = vec![item_id.clone()];

    state.inventory.insert(item_id.clone(), 1);
    let plain = state.brew_ingredients(&data, &selected);

    state
        .progression
        .bottle_stock
        .insert(item_id.clone(), vec![batch(&item_id, "Masterwork", 92, 1)]);
    let graded = state.brew_ingredients(&data, &selected);

    assert!(
        graded[0].quality > plain[0].quality,
        "a masterwork bottle poured in at quality {} against the plain {}",
        graded[0].quality,
        plain[0].quality
    );
    assert!(
        graded[0].traits.iter().any(|held| held == "luminous"),
        "the traits the bottle was brewed with did not reach the pot"
    );
}

/// The pot takes the best bottle, and only once. A recipe asking for two of
/// the same solution when one of them is good gets the good one and then an
/// ordinary one.
#[test]
fn one_good_bottle_improves_one_slot() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let item_id = a_bottle_used_as_a_reagent(&data);

    state.inventory.insert(item_id.clone(), 2);
    state.progression.bottle_stock.insert(
        item_id.clone(),
        vec![
            batch(&item_id, "Crude", 15, 1),
            batch(&item_id, "Masterwork", 92, 1),
        ],
    );

    let both = state.brew_ingredients(&data, &[item_id.clone(), item_id.clone()]);
    assert_eq!(
        both[0].quality, 92,
        "the first slot should get the good one"
    );
    assert_eq!(both[1].quality, 15, "the second slot should get the other");
}

/// Brewing has to spend the bottle it poured. `take_from_inventory` trims
/// the *worst* batch, which is right for a sale and wrong here: pour a
/// Masterwork and the shelf would quietly keep it and drop a Crude one, so
/// the same bottle would improve every future brew of that recipe.
#[test]
fn brewing_spends_the_bottle_it_poured() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let item_id = a_bottle_used_as_a_reagent(&data);

    state.inventory.insert(item_id.clone(), 2);
    state.progression.bottle_stock.insert(
        item_id.clone(),
        vec![
            batch(&item_id, "Crude", 15, 1),
            batch(&item_id, "Masterwork", 92, 1),
        ],
    );

    // The real consume path, so the ordering it depends on is under test
    // rather than the helper on its own.
    state.consume_brew_inputs(&data, std::slice::from_ref(&item_id));

    let left = &state.progression.bottle_stock[&item_id];
    assert_eq!(left.len(), 1);
    assert_eq!(
        left[0].quality_band, "Crude",
        "the masterwork survived the brew that poured it"
    );
}

/// The whole reason the late tier exists: a compound brew is worth what its
/// inputs were worth. Run every second-order recipe to its own spec twice —
/// once with ordinary bottles and once with bottles brewed at Masterwork —
/// and the second run has to score higher, or "brew the input well" is
/// advice with nothing behind it.
#[test]
fn a_compound_brew_is_worth_what_its_inputs_were_worth() {
    use crate::alchemy::resolve_brew;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut checked = 0usize;

    for recipe in &data.recipes {
        let bottles = recipe
            .ingredients
            .iter()
            .filter(|ingredient| {
                data.item(&ingredient.item_id)
                    .is_some_and(|item| item.category == ItemCategory::Potion)
            })
            .map(|ingredient| ingredient.item_id.clone())
            .collect::<Vec<_>>();
        if bottles.is_empty() {
            continue;
        }
        let station = data
            .stations
            .iter()
            .find(|station| station.id == recipe.station_id)
            .expect("the recipe's bench");
        let selected = recipe
            .ingredients
            .iter()
            .flat_map(|ingredient| {
                std::iter::repeat_n(ingredient.item_id.clone(), ingredient.amount as usize)
            })
            .collect::<Vec<_>>();

        let mut state = GameplayState::new(&data);
        for item_id in &selected {
            *state.inventory.entry(item_id.clone()).or_insert(0) += 1;
        }
        let brew = |state: &GameplayState| {
            resolve_brew(
                &data,
                station,
                &selected,
                &state.brew_ingredients(&data, &selected),
                None,
                recipe.required_heat,
                recipe.required_stirs,
                &recipe.required_timing,
                0,
            )
            .quality_score
        };

        let ordinary = brew(&state);
        for item_id in &bottles {
            state
                .progression
                .bottle_stock
                .insert(item_id.clone(), vec![batch(item_id, "Masterwork", 92, 1)]);
        }
        let well_made = brew(&state);

        assert!(
            well_made > ordinary,
            "{} scored {well_made} on masterwork reagents against {ordinary} on plain ones",
            recipe.id
        );
        checked += 1;
    }

    assert!(checked > 0, "no second-order recipe to check");
}

/// The materials list reads the quality of the bottle it would pour, not the
/// item file's default. Without it the one decision the late tier asks for —
/// brew the input well before folding it — is invisible at the bench.
#[test]
fn the_materials_list_grades_the_bottle_on_the_shelf() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let item_id = a_bottle_used_as_a_reagent(&data);
    let authored = data.item(&item_id).expect("the reagent").quality;

    state.inventory.insert(item_id.clone(), 1);
    assert_eq!(state.reagent_quality(&data, &item_id), authored);

    state
        .progression
        .bottle_stock
        .insert(item_id.clone(), vec![batch(&item_id, "Masterwork", 92, 1)]);
    assert_eq!(state.reagent_quality(&data, &item_id), 92);
}
