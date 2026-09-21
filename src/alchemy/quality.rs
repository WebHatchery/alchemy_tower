use crate::content::ui_copy;
use crate::data::{AlchemyTuning, ItemDefinition, RecipeDefinition, StationDefinition};

#[path = "quality_factors.rs"]
mod quality_factors;

pub(super) use self::quality_factors::weighted_quality_average;
use self::quality_factors::{
    preferred_trait_matches, shared_trait_bonus, synthesis_efficiency_bonus, total_synthesis_weight,
};

pub(crate) fn quality_band(score: u32, tuning: &AlchemyTuning) -> &'static str {
    tuning
        .quality_bands
        .iter()
        .rev()
        .find(|band| score >= band.minimum_score)
        .map(|band| ui_copy(&band.label_key))
        .unwrap_or_else(|| ui_copy(&tuning.quality_bands[0].label_key))
}

pub(crate) fn mastery_stage(successful_brews: u32, tuning: &AlchemyTuning) -> &'static str {
    tuning
        .mastery_stages
        .iter()
        .rev()
        .find(|stage| successful_brews >= stage.minimum_brews)
        .map(|stage| ui_copy(&stage.label_key))
        .unwrap_or_else(|| ui_copy(&tuning.mastery_stages[0].label_key))
}

pub(crate) fn mastery_threshold(tuning: &AlchemyTuning) -> u32 {
    tuning
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn calculate_quality(
    recipe: &RecipeDefinition,
    station: &StationDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing_match: bool,
    sequence_match: bool,
    catalyst_match: bool,
    room_bonus_applied: bool,
    minimum_elements_met: bool,
    mastery_brews: u32,
    tuning: &AlchemyTuning,
) -> u32 {
    let total_weight = total_synthesis_weight(ingredients);
    let mut score = weighted_quality_average(ingredients);

    score += synthesis_efficiency_bonus(
        ingredients,
        total_weight,
        tuning.scoring.synthesis_efficiency_multiplier,
    );
    score += shared_trait_bonus(ingredients) * tuning.scoring.shared_trait_multiplier;
    score += preferred_trait_matches(recipe, ingredients, catalyst) as u32
        * tuning.scoring.preferred_trait_match_bonus;
    score +=
        mastery_brews.min(mastery_threshold(tuning)) * tuning.scoring.mastery_brew_quality_bonus;

    // Heat and stirs at the target brew cleanly; overcharging past the target
    // adds potency (see `volatility::overcharge_potency`) at the cost of
    // instability; underfiring/understirring still degrades the brew.
    if heat >= recipe.required_heat {
        score += tuning.scoring.heat_match_bonus;
    } else {
        score = score.saturating_sub(
            (recipe.required_heat - heat).unsigned_abs() * tuning.scoring.heat_shortfall_penalty,
        );
    }

    if stirs >= recipe.required_stirs {
        score += tuning.scoring.stirs_match_bonus;
    } else {
        score = score.saturating_sub(
            stirs.abs_diff(recipe.required_stirs) * tuning.scoring.stirs_shortfall_penalty,
        );
    }

    score += super::volatility::overcharge_potency(recipe, heat, stirs, &tuning.volatility);

    if timing_match {
        score += tuning.scoring.timing_match_bonus;
    } else if !recipe.required_timing.is_empty() {
        score = score.saturating_sub(tuning.scoring.timing_mismatch_penalty);
    }

    if sequence_match {
        score += tuning.scoring.sequence_match_bonus;
    } else if !recipe.required_sequence.is_empty() {
        score = score.saturating_sub(tuning.scoring.sequence_mismatch_penalty);
    }

    if catalyst_match {
        if let Some(catalyst) = catalyst {
            score += catalyst.quality / tuning.scoring.catalyst_quality_divisor
                + catalyst.synthesis_value
                + recipe.catalyst_quality_bonus;
        }
    } else if !recipe.catalyst_tag.is_empty() {
        score = score.saturating_sub(tuning.scoring.catalyst_mismatch_penalty);
    }

    if room_bonus_applied {
        score += station.room_bonus.quality_bonus;
    }

    if minimum_elements_met {
        score += tuning.scoring.minimum_elements_met_bonus;
    } else if recipe.minimum_elements.total() > 0 {
        score = score.saturating_sub(tuning.scoring.minimum_elements_missed_penalty);
    }

    // Mastery means being able to make one particular thing the same way twice,
    // so a mastered formula never scores below its own bar. Reagents and process
    // still decide how far *above* it the brew lands — this only removes the
    // possibility of a mastered recipe failing on quality, which is what makes
    // the seventh brew worth reaching rather than a label.
    if mastery_brews >= mastery_threshold(tuning) {
        score = score.max(recipe.minimum_quality);
    }

    score.min(tuning.scoring.score_cap)
}

pub(super) fn room_bonus_applies(
    station: &StationDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> bool {
    if station.room_bonus.quality_bonus == 0 {
        return false;
    }
    let favored_trait_hit = station.room_bonus.favored_traits.iter().any(|trait_name| {
        ingredients.iter().any(|item| {
            item.traits
                .iter()
                .any(|item_trait| item_trait == trait_name)
        }) || catalyst
            .map(|item| {
                item.traits
                    .iter()
                    .any(|item_trait| item_trait == trait_name)
            })
            .unwrap_or(false)
    });
    let favored_category_hit = station
        .room_bonus
        .favored_categories
        .iter()
        .any(|category| {
            ingredients
                .iter()
                .any(|item| item.category.as_str() == category)
                || catalyst
                    .map(|item| item.category.as_str() == category)
                    .unwrap_or(false)
        });

    favored_trait_hit || favored_category_hit
}

#[cfg(test)]
mod tests;
