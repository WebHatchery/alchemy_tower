use std::collections::BTreeMap;

use crate::data::{ElementProfile, GameData, ItemDefinition, RecipeDefinition, StationDefinition};

pub(crate) fn match_recipe<'a>(
    data: &'a GameData,
    station: &StationDefinition,
    selected_items: &[String],
) -> Option<&'a RecipeDefinition> {
    let selected_counts = item_counts(selected_items);

    data.recipes.iter().find(|recipe| {
        recipe.station_id == station.id
            && recipe.ingredients.len() == selected_counts.len()
            && recipe.ingredients.iter().all(|ingredient| {
                selected_counts.get(&ingredient.item_id) == Some(&ingredient.amount)
            })
    })
}

fn item_counts(selected_items: &[String]) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for item_id in selected_items {
        *counts.entry(item_id.clone()).or_insert(0) += 1;
    }
    counts
}

pub(super) fn total_elements(
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> ElementProfile {
    let mut total = ElementProfile::default();
    for item in ingredients {
        total.add_assign(&item.elements);
    }
    if let Some(catalyst) = catalyst {
        total.add_assign(&catalyst.elements);
    }
    total
}

/// Whether the loaded slots satisfy a required reagent order. Reads the
/// ingredients the brew is actually made from rather than looking them up by id,
/// so a trait a wild variant added can satisfy a sequence token the plain herb
/// could not.
pub(super) fn sequence_matches(
    ingredients: &[ItemDefinition],
    required_sequence: &[String],
) -> bool {
    if required_sequence.is_empty() {
        return true;
    }
    if ingredients.len() < required_sequence.len() {
        return false;
    }

    ingredients
        .iter()
        .zip(required_sequence.iter())
        .all(|(item, token)| sequence_token_matches(item, token))
}

fn sequence_token_matches(item: &ItemDefinition, token: &str) -> bool {
    item.id == token
        || item.category.as_str() == token
        || item.traits.iter().any(|item_trait| item_trait == token)
}

#[cfg(test)]
mod tests;
