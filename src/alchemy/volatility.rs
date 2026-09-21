//! Overcharge and volatility.
//!
//! Brewing past a recipe's required heat and stir count is *allowed* — it pushes
//! the output quality higher (see the potency bonus in `quality.rs`), which is
//! how a brewer reaches the Excellent/Masterwork bands and clears steep morph
//! quality gates. The cost is instability: every step of overcharge, and every
//! volatile ingredient, adds to a deterministic instability score. Calm and
//! pure ingredients, a matched catalyst, and recipe mastery bleed it back off.
//!
//! When instability reaches the authored `instability_limit` the brew destabilizes and
//! collapses to the recipe's unstable output no matter how good the numbers
//! otherwise look. The score is fully derived from the setup (no randomness),
//! so the preview can show it and the player can judge how hard to push.

use crate::data::{AlchemyVolatilityTuning, ItemDefinition, RecipeDefinition};

/// Extra quality granted for overcharging heat/stirs past the recipe target.
/// Returns 0 when the process is at or below the required values.
pub(super) fn overcharge_potency(
    recipe: &RecipeDefinition,
    heat: i32,
    stirs: u32,
    tuning: &AlchemyVolatilityTuning,
) -> u32 {
    overcharge_heat_steps(recipe, heat) * tuning.overcharge_heat_potency
        + overcharge_stir_steps(recipe, stirs) * tuning.overcharge_stir_potency
}

/// Deterministic instability score for a brew setup. Higher is riskier;
/// the authored `instability_limit` is the point of collapse.
#[allow(clippy::too_many_arguments)]
pub(super) fn brew_instability(
    recipe: &RecipeDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
    catalyst_matched: bool,
    heat: i32,
    stirs: u32,
    mastery_brews: u32,
    mastery_threshold: u32,
    tuning: &AlchemyVolatilityTuning,
) -> u32 {
    let mut instability = overcharge_heat_steps(recipe, heat) * tuning.heat_instability
        + overcharge_stir_steps(recipe, stirs) * tuning.stir_instability
        + trait_count(
            ingredients,
            catalyst,
            std::slice::from_ref(&tuning.volatile_trait),
        ) * tuning.volatile_instability;

    let stabilizers = trait_count(ingredients, catalyst, &tuning.stabilizer_traits);
    instability = instability.saturating_sub(stabilizers * tuning.stabilizer_reduction);

    if catalyst_matched && catalyst.is_some() {
        instability = instability.saturating_sub(tuning.catalyst_stabilize);
    }

    instability.saturating_sub(mastery_brews.min(mastery_threshold) * tuning.mastery_stabilize)
}

pub(super) fn is_destabilized(instability: u32, limit: u32) -> bool {
    instability >= limit
}

fn overcharge_heat_steps(recipe: &RecipeDefinition, heat: i32) -> u32 {
    (heat - recipe.required_heat).max(0) as u32
}

fn overcharge_stir_steps(recipe: &RecipeDefinition, stirs: u32) -> u32 {
    stirs.saturating_sub(recipe.required_stirs)
}

fn trait_count(
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
    wanted: &[String],
) -> u32 {
    let mut count = 0;
    for item in ingredients {
        count += matching_traits(item, wanted);
    }
    if let Some(catalyst) = catalyst {
        count += matching_traits(catalyst, wanted);
    }
    count
}

fn matching_traits(item: &ItemDefinition, wanted: &[String]) -> u32 {
    item.traits
        .iter()
        .filter(|item_trait| wanted.iter().any(|wanted| wanted == *item_trait))
        .count() as u32
}

#[cfg(test)]
mod tests;
