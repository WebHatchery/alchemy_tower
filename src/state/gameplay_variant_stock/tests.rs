use super::GameplayState;
use crate::data::GameData;

/// The first item in the data that has a variant worth something, so these
/// tests follow the content rather than pinning one herb by name.
fn an_item_with_a_variant(data: &GameData) -> (String, String) {
    let item = data
        .items
        .iter()
        .find(|item| {
            item.wild_variants
                .iter()
                .any(|variant| variant.quality_bonus > 0)
        })
        .expect("some ingredient should have a variant worth gathering for");
    let variant = item
        .wild_variants
        .iter()
        .max_by_key(|variant| variant.quality_bonus)
        .expect("the variant just found");
    (item.id.clone(), variant.id.clone())
}

/// A variant used to be a string in the journal. Gathering one under the
/// right sky recorded the name against the herb's best-seen line and threw
/// the unit's quality away, because inventory counts units and had no way
/// to say one of them was better. This asserts the pickup reaches the pot.
#[test]
fn a_variant_gathered_under_the_right_sky_reaches_the_bench() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (item_id, variant_id) = an_item_with_a_variant(&data);
    let selected = vec![item_id.clone()];

    let plain = state.brew_ingredients(&data, &selected);
    state.note_variant_gathered(&item_id, &variant_id);
    let improved = state.brew_ingredients(&data, &selected);

    assert!(
        improved[0].quality > plain[0].quality,
        "the variant unit brews no better than the plain herb"
    );
    assert!(improved[0].traits.len() >= plain[0].traits.len());
}

/// One unit is one unit. A recipe calling for two of the same herb when only
/// one variant was gathered should get the good one and then a plain one,
/// not the good one counted twice.
#[test]
fn a_single_variant_unit_is_only_spent_once() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (item_id, variant_id) = an_item_with_a_variant(&data);
    state.note_variant_gathered(&item_id, &variant_id);

    let both = state.brew_ingredients(&data, &[item_id.clone(), item_id.clone()]);
    let base = data.item(&item_id).expect("the item").quality;
    assert!(
        both[0].quality > base,
        "the first slot should get the good one"
    );
    assert_eq!(both[1].quality, base, "the second slot should be plain");
}

/// The gap this system shipped with, named in the TODO for six passes: the
/// belt shows one stack per id, the bench quietly spends the best unit in
/// it, and *nothing on screen said which stacks held one*. The player was
/// making the one decision the system exists for — brew now or walk back
/// out for a better strain — with no information at all.
///
/// Both surfaces are checked here because they answer different questions:
/// the bench says which stack is worth loading, the journal says what is in
/// the bag and how much of it.
#[test]
fn a_held_variant_is_visible_at_the_bench_and_in_the_journal() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (item_id, variant_id) = an_item_with_a_variant(&data);
    state.inventory.insert(item_id.clone(), 3);

    let plain_quality = state.reagent_quality(&data, &item_id);
    let plain_title = state
        .alchemy_materials_panel_view(&data)
        .rows
        .into_iter()
        .find(|row| row.title.starts_with(data.item_name(&item_id)))
        .map(|row| row.title)
        .expect("the herb should be listed at the bench");

    state.note_variant_gathered(&item_id, &variant_id);
    state.note_variant_gathered(&item_id, &variant_id);

    assert!(
        state.reagent_quality(&data, &item_id) > plain_quality,
        "the bench still reads the plain quality for a stack holding a variant"
    );
    let marked_title = state
        .alchemy_materials_panel_view(&data)
        .rows
        .into_iter()
        .find(|row| row.title.starts_with(data.item_name(&item_id)))
        .map(|row| row.title)
        .expect("the herb should still be listed");
    assert_ne!(
        marked_title, plain_title,
        "the row reads the same whether or not the stack holds a variant"
    );

    let (name, count) = state
        .held_variant_summary(&data, &item_id)
        .expect("two gathered units should be held");
    assert_eq!(count, 2, "the journal should count what is in the bag");
    assert!(!name.is_empty(), "the strain should be named");
}

/// Brewing consumes the variant along with the herb. Without this the same
/// lucky pickup would improve every future brew of that recipe forever.
#[test]
fn brewing_spends_the_variant_it_used() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let (item_id, variant_id) = an_item_with_a_variant(&data);
    state.note_variant_gathered(&item_id, &variant_id);
    assert!(state.best_held_variant(&data, &item_id).is_some());

    state.spend_brew_variants(&data, std::slice::from_ref(&item_id));
    assert!(
        state.best_held_variant(&data, &item_id).is_none(),
        "the variant survived the brew that used it"
    );
    assert!(
        !state.progression.variant_stock.contains_key(&item_id),
        "an emptied entry should be dropped rather than left at zero"
    );
}
