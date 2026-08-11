use super::gameplay_quest_requirements::{
    effect_requirement_target, quest_required_effect_kinds, quest_required_traits,
    trait_requirement_target,
};
use crate::content::ui_format;
use crate::data::QuestDefinition;

pub(super) fn carry_requirement_summary(carried: u32, required: u32) -> String {
    ui_format(
        "quests_requirement_carry",
        &[
            ("carried", &carried.to_string()),
            ("required", &required.to_string()),
        ],
    )
}

pub(super) fn quality_requirement_summary(minimum_quality_band: &str) -> String {
    ui_format(
        "quests_requirement_quality",
        &[("band", minimum_quality_band)],
    )
}

pub(super) fn ready_requirement_summary() -> String {
    ui_format("quests_requirement_ready", &[])
}

pub(super) fn trait_requirement_summary(quest: &QuestDefinition) -> String {
    let required_traits = quest_required_traits(quest);
    let target = trait_requirement_target(quest);
    if target == 0 {
        return ui_format("quests_trait_ready", &[]);
    }
    if target >= required_traits.len() {
        return ui_format(
            "quests_trait_all",
            &[("traits", &required_traits.join(" + "))],
        );
    }
    ui_format(
        "quests_trait_partial",
        &[
            ("target", &target.to_string()),
            ("count", &required_traits.len().to_string()),
            ("traits", &required_traits.join(", ")),
        ],
    )
}

pub(super) fn effect_requirement_summary(quest: &QuestDefinition) -> String {
    let required_effects = quest_required_effect_kinds(quest);
    let target = effect_requirement_target(quest);
    if target == 0 {
        return ui_format("quests_effect_ready", &[]);
    }
    if target >= required_effects.len() {
        return ui_format(
            "quests_effect_all",
            &[("effects", &required_effects.join(" + "))],
        );
    }
    ui_format(
        "quests_effect_partial",
        &[
            ("target", &target.to_string()),
            ("count", &required_effects.len().to_string()),
            ("effects", &required_effects.join(", ")),
        ],
    )
}

#[cfg(test)]
#[path = "gameplay_quest_requirement_summary/tests.rs"]
mod tests;
