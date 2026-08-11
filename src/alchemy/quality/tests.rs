use super::{mastery_stage, MASTERED_BREW_COUNT};
use crate::alchemy::resolve_brew;
use crate::data::GameData;

/// Everything a brew of one recipe's own reagents needs, run to spec.
fn brew_at_mastery(data: &GameData, mastery_brews: u32) -> (u32, u32, String) {
    let recipe = data
        .recipes
        .iter()
        .find(|recipe| recipe.id == "healing_draught_recipe")
        .expect("the healing draught recipe should exist");
    let station = data
        .stations
        .iter()
        .find(|station| station.id == recipe.station_id)
        .expect("its bench should exist");
    let selected = recipe
        .ingredients
        .iter()
        .map(|ingredient| ingredient.item_id.clone())
        .collect::<Vec<_>>();
    let ingredients = selected
        .iter()
        .filter_map(|item_id| data.item(item_id))
        .cloned()
        .collect::<Vec<_>>();
    let resolution = resolve_brew(
        data,
        station,
        &selected,
        &ingredients,
        None,
        recipe.required_heat,
        recipe.required_stirs,
        &recipe.required_timing,
        mastery_brews,
    );
    (
        resolution.quality_score,
        resolution.output_amount,
        resolution.mastery_stage.to_owned(),
    )
}

/// The seventh clean brew is what flips a formula to "Mastered" and opens
/// the mastery gates, and it used to be the one brew in the run that did
/// nothing: the quality ramp, the stability ramp and the extra bottle all
/// capped at six. The payoff now lands on the step that names it.
#[test]
fn the_brew_that_earns_mastery_is_worth_making() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let (before_quality, before_output, before_stage) =
        brew_at_mastery(&data, MASTERED_BREW_COUNT - 2);
    let (mastered_quality, mastered_output, mastered_stage) =
        brew_at_mastery(&data, MASTERED_BREW_COUNT - 1);

    assert!(
        mastered_quality > before_quality,
        "mastery brewed no better than the step before it: {mastered_quality} vs {before_quality}"
    );
    assert!(
        mastered_output > before_output,
        "mastery yielded no more bottles: {mastered_output} vs {before_output}"
    );
    assert_ne!(before_stage, mastery_stage(MASTERED_BREW_COUNT));
    assert_eq!(mastered_stage, mastery_stage(MASTERED_BREW_COUNT));

    // A failed seventh attempt neither earns mastery nor receives its
    // extra output. Only a successful brew advances the count.
    let recipe = data
        .recipes
        .iter()
        .find(|recipe| recipe.id == "healing_draught_recipe")
        .expect("the healing draught recipe should exist");
    let station = data
        .stations
        .iter()
        .find(|station| station.id == recipe.station_id)
        .expect("its bench should exist");
    let selected = recipe
        .ingredients
        .iter()
        .map(|ingredient| ingredient.item_id.clone())
        .collect::<Vec<_>>();
    let ingredients = selected
        .iter()
        .filter_map(|item_id| data.item(item_id))
        .cloned()
        .collect::<Vec<_>>();
    let failed = resolve_brew(
        &data,
        station,
        &selected,
        &ingredients,
        None,
        recipe.required_heat - 1,
        recipe.required_stirs,
        &recipe.required_timing,
        MASTERED_BREW_COUNT - 1,
    );
    assert!(!failed.is_stable());
    assert_eq!(failed.output_amount, recipe.output_amount);
    assert_ne!(failed.mastery_stage, mastery_stage(MASTERED_BREW_COUNT));
}

/// Mastery is defined in this crate as being able to make one particular
/// thing the same way twice, so a mastered formula cannot fail its own
/// quality bar however poor the reagents are. Process and stability still
/// apply — this only removes the one failure a practised hand would not make.
#[test]
fn a_mastered_formula_never_falls_below_its_own_bar() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let recipe = data
        .recipes
        .iter()
        .find(|recipe| recipe.minimum_quality > 0)
        .expect("some recipe should set a quality bar");
    let station = data
        .stations
        .iter()
        .find(|station| station.id == recipe.station_id)
        .expect("its bench should exist");
    let selected = recipe
        .ingredients
        .iter()
        .map(|ingredient| ingredient.item_id.clone())
        .collect::<Vec<_>>();

    // Deliberately awful reagents: every quality field zeroed.
    let ingredients = selected
        .iter()
        .filter_map(|item_id| data.item(item_id))
        .map(|item| {
            let mut poor = item.clone();
            poor.quality = 0;
            poor.synthesis_value = 0;
            poor
        })
        .collect::<Vec<_>>();

    let unmastered = resolve_brew(
        &data,
        station,
        &selected,
        &ingredients,
        None,
        recipe.required_heat,
        recipe.required_stirs,
        &recipe.required_timing,
        0,
    );
    let mastered = resolve_brew(
        &data,
        station,
        &selected,
        &ingredients,
        None,
        recipe.required_heat,
        recipe.required_stirs,
        &recipe.required_timing,
        MASTERED_BREW_COUNT,
    );

    assert!(
        mastered.quality_score >= recipe.minimum_quality,
        "a mastered formula scored {} against its own bar of {}",
        mastered.quality_score,
        recipe.minimum_quality
    );
    assert!(mastered.minimum_quality_met);
    assert!(
        mastered.quality_score > unmastered.quality_score,
        "mastery should be worth something with poor reagents too"
    );
}
