use super::GameplayState;
use crate::data::{GameData, QuestDefinition};

#[path = "gameplay_quest_unlock_text.rs"]
mod quest_unlock_text;

use self::quest_unlock_text::QuestUnlockRequirements;

impl GameplayState {
    pub(super) fn quest_is_available(&self, quest: &QuestDefinition) -> bool {
        quest
            .prerequisite_quests
            .iter()
            .all(|quest_id| self.progression.completed_quests.contains(quest_id))
            && (quest.required_unlocked_warp.is_empty()
                || self
                    .progression
                    .unlocked_warps
                    .contains(&quest.required_unlocked_warp))
            && self.progression.total_brews >= quest.minimum_total_brews
            && self.has_mastered_requirement(quest)
            && self.has_rapport_requirement(quest)
            && self.has_journal_requirement(quest)
    }

    /// Beat gate. The same field warps, stations and gather nodes already read,
    /// which is what lets a request wait on the ending without any new gating
    /// machinery — `observatory_ending` is a journal beat like any other.
    fn has_journal_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_journal_milestone.is_empty()
            || self.has_journal_milestone(&quest.required_journal_milestone)
    }

    /// Standing gate. A townsperson who counts the player a confidant asks for
    /// things they would not mention to a stranger.
    fn has_rapport_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_rapport_npc_id.is_empty()
            || self.rapport_value(&quest.required_rapport_npc_id) >= quest.required_rapport
    }

    /// Mastery is seven clean brews of one formula. `total_brews` measures how
    /// busy somebody has been; this measures whether they can make a particular
    /// thing the same way twice.
    fn has_mastered_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_mastered_recipe.is_empty()
            || self.recipe_mastery_brews(&quest.required_mastered_recipe)
                >= crate::alchemy::MASTERED_BREW_COUNT
    }

    pub(super) fn quest_unlock_summary(&self, data: &GameData, quest: &QuestDefinition) -> String {
        // Name the blocking request the way the player saw it. Raw quest ids
        // used to leak straight into this line, and a chain multiplies them.
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .map(|quest_id| {
                data.quest(quest_id)
                    .map(|blocking| blocking.title.clone())
                    .unwrap_or_else(|| quest_id.clone())
            })
            .collect::<Vec<_>>();
        let missing_warp = !quest.required_unlocked_warp.is_empty()
            && !self
                .progression
                .unlocked_warps
                .contains(&quest.required_unlocked_warp);
        let missing_total_brews = self.progression.total_brews < quest.minimum_total_brews;
        // Name the formula the way the bench does, not by its id.
        let missing_mastery = if self.has_mastered_requirement(quest) {
            String::new()
        } else {
            data.recipe(&quest.required_mastered_recipe)
                .map(|recipe| recipe.name.clone())
                .unwrap_or_else(|| quest.required_mastered_recipe.clone())
        };

        // Name the beat the way the journal did. A raw beat id here would be the
        // same leak the prerequisite titles above were fixed for.
        let missing_beat = if self.has_journal_requirement(quest) {
            String::new()
        } else {
            beat_title(data, &quest.required_journal_milestone)
        };

        quest_unlock_text::summary(QuestUnlockRequirements {
            missing_prereqs,
            missing_warp,
            missing_total_brews,
            minimum_total_brews: quest.minimum_total_brews,
            missing_mastery,
            missing_beat,
        })
    }
}

/// A beat's title, wherever it was authored: the narrative spine, a quest's
/// completion milestones, a recipe's discovery milestones, or an apply target.
/// Falls back to the id, which is better than an empty reason.
fn beat_title(data: &GameData, beat_id: &str) -> String {
    let spine = crate::content::narrative_text()
        .milestones
        .all()
        .into_iter()
        .find(|milestone| milestone.id == beat_id)
        .map(|milestone| milestone.title.clone());
    spine
        .or_else(|| {
            data.quests
                .iter()
                .flat_map(|quest| quest.completion_milestones.iter())
                .chain(
                    data.recipes
                        .iter()
                        .flat_map(|recipe| recipe.discovery_milestones.iter()),
                )
                .chain(
                    data.areas
                        .iter()
                        .flat_map(|area| area.apply_targets.iter())
                        .flat_map(|target| target.completion_milestones.iter()),
                )
                .find(|milestone| milestone.id == beat_id)
                .map(|milestone| milestone.title.clone())
        })
        .unwrap_or_else(|| beat_id.to_owned())
}

#[cfg(test)]
#[path = "gameplay_quest_availability/tests.rs"]
mod tests;
