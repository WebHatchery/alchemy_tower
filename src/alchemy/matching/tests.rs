use super::sequence_matches;

/// A required sequence is checked against the slots as loaded, so a token
/// naming a trait none of the recipe's own reagents carries can never be
/// satisfied in any order — the recipe is then permanently faulted and there
/// is nothing on screen to explain why.
#[test]
fn every_required_sequence_is_satisfiable_by_its_own_reagents() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut impossible = Vec::new();

    for recipe in &data.recipes {
        if recipe.required_sequence.is_empty() {
            continue;
        }
        let mut slots = Vec::new();
        for ingredient in &recipe.ingredients {
            for _ in 0..ingredient.amount {
                slots.push(ingredient.item_id.clone());
            }
        }
        if slots.len() < recipe.required_sequence.len() {
            impossible.push(format!(
                "{}: {} slots for a {}-step sequence",
                recipe.id,
                slots.len(),
                recipe.required_sequence.len()
            ));
            continue;
        }
        if !any_ordering_matches(&data, &slots, &recipe.required_sequence) {
            impossible.push(format!(
                "{}: no ordering of {:?} satisfies {:?}",
                recipe.id, slots, recipe.required_sequence
            ));
        }
    }

    assert!(
        impossible.is_empty(),
        "recipes whose sequence can never be met:\n{impossible:#?}"
    );
}

/// Slot counts here are two or three, so trying every arrangement is both
/// exact and cheap — no need to reason about which reagent fits which step.
fn any_ordering_matches(
    data: &crate::data::GameData,
    slots: &[String],
    sequence: &[String],
) -> bool {
    permutations(slots).into_iter().any(|ordering| {
        let items = ordering
            .iter()
            .filter_map(|item_id| data.item(item_id))
            .cloned()
            .collect::<Vec<_>>();
        sequence_matches(&items, sequence)
    })
}

fn permutations(slots: &[String]) -> Vec<Vec<String>> {
    if slots.len() <= 1 {
        return vec![slots.to_vec()];
    }
    let mut out = Vec::new();
    for (index, item) in slots.iter().enumerate() {
        let mut rest = slots.to_vec();
        rest.remove(index);
        for mut tail in permutations(&rest) {
            let mut ordering = vec![item.clone()];
            ordering.append(&mut tail);
            out.push(ordering);
        }
    }
    out
}
