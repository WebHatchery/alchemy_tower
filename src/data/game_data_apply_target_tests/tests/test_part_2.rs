use super::*;

#[test]
fn every_effect_a_bottle_can_carry_has_something_to_pour_it_on() {
    use crate::data::ItemCategory;

    let data = load_embedded().expect("embedded game data should load");
    let poured = data
        .areas
        .iter()
        .flat_map(|area| area.apply_targets.iter())
        .map(|target| target.required_effect_kind.clone())
        .collect::<std::collections::HashSet<_>>();

    let unpourable = data
        .items
        .iter()
        .filter(|item| item.category == ItemCategory::Potion)
        .flat_map(|item| item.effects.iter())
        .map(|effect| effect.kind.as_str())
        .filter(|kind| !poured.contains(*kind))
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        unpourable.is_empty(),
        "effect kinds with nothing in the world to use them on: {unpourable:?}"
    );
}
