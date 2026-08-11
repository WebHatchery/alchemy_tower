//! Overcharge and volatility.
//!
//! Brewing past a recipe's required heat and stir count is *allowed* — it pushes
//! the output quality higher (see the potency bonus in `quality.rs`), which is
//! how a brewer reaches the Excellent/Masterwork bands and clears steep morph
//! quality gates. The cost is instability: every step of overcharge, and every
//! volatile ingredient, adds to a deterministic instability score. Calm and
//! pure ingredients, a matched catalyst, and recipe mastery bleed it back off.
//!
//! When instability reaches [`INSTABILITY_LIMIT`] the brew destabilizes and
//! collapses to the recipe's unstable output no matter how good the numbers
//! otherwise look. The score is fully derived from the setup (no randomness),
//! so the preview can show it and the player can judge how hard to push.

use crate::data::{ItemDefinition, RecipeDefinition};

/// Quality added per step of heat above the recipe requirement.
const OVERCHARGE_HEAT_POTENCY: u32 = 3;
/// Quality added per stir above the recipe requirement.
const OVERCHARGE_STIR_POTENCY: u32 = 2;

/// Instability added per step of heat above the recipe requirement.
const HEAT_INSTABILITY: u32 = 14;
/// Instability added per stir above the recipe requirement.
const STIR_INSTABILITY: u32 = 11;
/// Instability added per `volatile` trait among the ingredients/catalyst.
const VOLATILE_INSTABILITY: u32 = 12;
/// Instability removed per `pure`/`calm` (stabilizing) trait.
const STABILIZER_REDUCTION: u32 = 9;
/// Instability removed when a matched catalyst steadies the reaction.
const CATALYST_STABILIZE: u32 = 6;
/// Instability removed per clean brew of the same formula, up to mastery
/// (capped alongside the quality bonus).
const MASTERY_STABILIZE: u32 = 3;

/// At or above this score the brew destabilizes into its unstable output.
pub(super) const INSTABILITY_LIMIT: u32 = 100;

const VOLATILE_TRAITS: [&str; 1] = ["volatile"];
const STABILIZER_TRAITS: [&str; 2] = ["pure", "calm"];

/// Extra quality granted for overcharging heat/stirs past the recipe target.
/// Returns 0 when the process is at or below the required values.
pub(super) fn overcharge_potency(recipe: &RecipeDefinition, heat: i32, stirs: u32) -> u32 {
    overcharge_heat_steps(recipe, heat) * OVERCHARGE_HEAT_POTENCY
        + overcharge_stir_steps(recipe, stirs) * OVERCHARGE_STIR_POTENCY
}

/// Deterministic instability score for a brew setup. Higher is riskier;
/// [`INSTABILITY_LIMIT`] is the point of collapse.
pub(super) fn brew_instability(
    recipe: &RecipeDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
    catalyst_matched: bool,
    heat: i32,
    stirs: u32,
    mastery_brews: u32,
) -> u32 {
    let mut instability = overcharge_heat_steps(recipe, heat) * HEAT_INSTABILITY
        + overcharge_stir_steps(recipe, stirs) * STIR_INSTABILITY
        + trait_count(ingredients, catalyst, &VOLATILE_TRAITS) * VOLATILE_INSTABILITY;

    let stabilizers = trait_count(ingredients, catalyst, &STABILIZER_TRAITS);
    instability = instability.saturating_sub(stabilizers * STABILIZER_REDUCTION);

    if catalyst_matched && catalyst.is_some() {
        instability = instability.saturating_sub(CATALYST_STABILIZE);
    }

    instability
        .saturating_sub(mastery_brews.min(super::quality::MASTERED_BREW_COUNT) * MASTERY_STABILIZE)
}

pub(super) fn is_destabilized(instability: u32) -> bool {
    instability >= INSTABILITY_LIMIT
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
    wanted: &[&str],
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

fn matching_traits(item: &ItemDefinition, wanted: &[&str]) -> u32 {
    item.traits
        .iter()
        .filter(|item_trait| wanted.contains(&item_trait.as_str()))
        .count() as u32
}

#[cfg(test)]
mod tests;
