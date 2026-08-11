use super::GameplayState;
use crate::data::GameData;

#[test]
fn disassembly_returns_recipe_inputs() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);
    let recipe = data
        .recipe_for_output("healing_draught")
        .expect("healing draught recipe should exist");

    state.progression.known_recipes.insert(recipe.id.clone());
    state.inventory.insert("healing_draught".to_owned(), 1);

    state.disassemble_recipe(&data, recipe);

    assert_eq!(
        state
            .inventory
            .get("healing_draught")
            .copied()
            .unwrap_or_default(),
        0
    );
    assert_eq!(
        state.inventory.get("sunleaf").copied().unwrap_or_default(),
        1
    );
    assert_eq!(
        state
            .inventory
            .get("whisper_moss")
            .copied()
            .unwrap_or_default(),
        1
    );
}

/// The rule the console shipped without: a whole brew's worth of bottles,
/// taken apart, cannot yield more reagents than the brew cost. It did —
/// nine recipes make more than one bottle, and `coldiron_tincture` and
/// `shiftlong_tonic` turn **three reagents into three bottles**, each
/// handing back all three. Six free reagents a brew, no travel, no season,
/// repeatable forever, against a game whose whole outer loop is deciding
/// where to walk.
#[test]
fn taking_a_batch_apart_never_makes_reagents() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut printers = Vec::new();

    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            let returned = recipe.output_amount * super::salvage_share(recipe, ingredient.amount);
            if returned > ingredient.amount {
                printers.push(format!(
                    "{}: {} x{} in, {returned} back",
                    recipe.id, ingredient.item_id, ingredient.amount
                ));
            }
        }
    }

    assert!(
        printers.is_empty(),
        "recipes the archive console prints reagents from:
{printers:#?}"
    );
}

/// Rounding down means a batch recipe can divide away to nothing, and a
/// console entry that eats a bottle and hands back an empty pair of hands
/// is worse than no entry. Those are filtered out of the list instead.
#[test]
fn nothing_is_offered_that_gives_nothing_back() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for recipe in &data.recipes {
        state.progression.known_recipes.insert(recipe.id.clone());
        state.inventory.insert(recipe.output_item_id.clone(), 1);
    }

    let empty = state
        .available_disassembly_recipes(&data)
        .into_iter()
        .filter(|recipe| {
            recipe
                .ingredients
                .iter()
                .all(|ingredient| super::salvage_share(recipe, ingredient.amount) == 0)
        })
        .map(|recipe| recipe.id.clone())
        .collect::<Vec<_>>();

    assert!(
        empty.is_empty(),
        "recipes offered for disassembly that return nothing: {empty:?}"
    );
}
