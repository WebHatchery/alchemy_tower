use std::collections::BTreeMap;

use crate::data::{ItemDefinition, RecipeDefinition};

pub(super) fn total_synthesis_weight(ingredients: &[&ItemDefinition]) -> u32 {
    ingredients
        .iter()
        .map(|item| item.synthesis_weight.max(1))
        .sum::<u32>()
        .max(1)
}

pub(in crate::alchemy) fn weighted_quality_average(ingredients: &[&ItemDefinition]) -> u32 {
    if ingredients.is_empty() {
        return 0;
    }

    let total_weight = total_synthesis_weight(ingredients);
    ingredients
        .iter()
        .map(|item| item.quality * item.synthesis_weight.max(1))
        .sum::<u32>()
        / total_weight
}

pub(super) fn synthesis_efficiency_bonus(
    ingredients: &[&ItemDefinition],
    total_weight: u32,
) -> u32 {
    if ingredients.is_empty() {
        return 0;
    }

    ingredients
        .iter()
        .map(|item| item.synthesis_value)
        .sum::<u32>()
        .saturating_mul(2)
        / total_weight
}

pub(super) fn shared_trait_bonus(ingredients: &[&ItemDefinition]) -> u32 {
    let mut counts = BTreeMap::<String, u32>::new();
    for item in ingredients {
        for item_trait in &item.traits {
            *counts.entry(item_trait.clone()).or_insert(0) += 1;
        }
    }
    counts.values().filter(|count| **count > 1).count() as u32
}

pub(super) fn preferred_trait_matches(
    recipe: &RecipeDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> usize {
    recipe
        .preferred_traits
        .iter()
        .filter(|preferred| {
            ingredients.iter().any(|item| {
                item.traits
                    .iter()
                    .any(|item_trait| item_trait == *preferred)
            }) || catalyst
                .map(|item| {
                    item.traits
                        .iter()
                        .any(|item_trait| item_trait == *preferred)
                })
                .unwrap_or(false)
        })
        .count()
}

#[cfg(test)]
#[path = "quality_factors/tests.rs"]
mod tests;
