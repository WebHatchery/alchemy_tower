//! Pouring a brew on something rather than drinking it or handing it over.
//!
//! The game is about applied alchemy, and for a long time the only thing a
//! finished bottle could do to the world was change a townsperson's opinion of
//! the player. Wilted ground, frightened creatures and blocked paths were
//! collision boxes and art — scenery a brew could not touch.
//!
//! A target is a thing that can be treated. It asks for a kind of effect and,
//! optionally, a grade; treating it spends a qualifying bottle and records
//! journal milestones. Those milestones are deliberately the same currency
//! every other gate already reads, so a warp, a station or a patch of ground
//! can wait on something having been *treated* without a line of new gating
//! code.

use super::gameplay_support::quality_band_rank;
use super::GameplayState;
use crate::content::ui_format;
use crate::data::{ApplyTargetDefinition, AreaDefinition, GameData, ItemCategory};

impl GameplayState {
    pub(super) fn target_is_treated(&self, target: &ApplyTargetDefinition) -> bool {
        self.progression.treated_targets.contains(&target.id)
    }

    /// The bottle on the shelf that would do this job, worst acceptable first —
    /// the same courtesy delivery pays, so treating a wilted bed does not cost
    /// the player their best work.
    pub(super) fn bottle_for_target(
        &self,
        data: &GameData,
        target: &ApplyTargetDefinition,
    ) -> Option<String> {
        let mut best: Option<(u8, String)> = None;
        for (item_id, held) in &self.inventory {
            if *held == 0 {
                continue;
            }
            let Some(item) = data.item(item_id) else {
                continue;
            };
            if item.category != ItemCategory::Potion {
                continue;
            }
            if !item
                .effects
                .iter()
                .any(|effect| effect.kind.to_string() == target.required_effect_kind)
            {
                continue;
            }
            let Some(bottle) =
                self.worst_held_bottle_at_or_above(data, item_id, &target.minimum_quality_band)
            else {
                continue;
            };
            let rank = quality_band_rank(&bottle.quality_band);
            if best.as_ref().is_none_or(|(current, _)| rank < *current) {
                best = Some((rank, item_id.clone()));
            }
        }
        best.map(|(_, item_id)| item_id)
    }

    /// Treat the target with a qualifying bottle. Returns false when there is
    /// nothing on the shelf that would do, so the caller can say so.
    pub(super) fn treat_target(&mut self, data: &GameData, target: &ApplyTargetDefinition) -> bool {
        let Some(item_id) = self.bottle_for_target(data, target) else {
            return false;
        };
        if !self.spend_bottle_at_or_above(data, &item_id, &target.minimum_quality_band) {
            return false;
        }
        self.progression.treated_targets.insert(target.id.clone());
        for milestone in &target.completion_milestones {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
        self.refresh_available_nodes(data);
        self.trigger_quest_complete_feedback(ui_format(
            "target_treated_toast",
            &[("name", &target.name)],
        ));
        self.runtime.status_text = target.treated_note.clone();
        true
    }

    /// The untreated target the player is standing next to, if any. A treated
    /// one is scenery again — the work is done.
    pub(super) fn interaction_apply_target<'a>(
        &self,
        area: &'a AreaDefinition,
    ) -> Option<&'a ApplyTargetDefinition> {
        let player = self.world.player.position;
        area.apply_targets
            .iter()
            .filter(|target| !self.target_is_treated(target))
            .find(|target| {
                let dx = target.position[0] - player.x;
                let dy = target.position[1] - player.y;
                (dx * dx + dy * dy).sqrt() <= target.radius
            })
    }

    pub(super) fn handle_apply_target_interaction(
        &mut self,
        data: &GameData,
        target: &ApplyTargetDefinition,
    ) {
        let target = target.clone();
        if !self.treat_target(data, &target) {
            self.runtime.status_text = self.target_requirement_text(&target);
        }
    }

    /// What the player is told when they have nothing that would work.
    pub(super) fn target_requirement_text(&self, target: &ApplyTargetDefinition) -> String {
        if target.minimum_quality_band.is_empty() {
            ui_format(
                "target_needs_effect",
                &[("effect", &target.required_effect_kind)],
            )
        } else {
            ui_format(
                "target_needs_graded_effect",
                &[
                    ("effect", &target.required_effect_kind),
                    ("band", &target.minimum_quality_band),
                ],
            )
        }
    }
}

#[cfg(test)]
#[path = "gameplay_apply_targets/tests.rs"]
mod tests;
