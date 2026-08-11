use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, NpcDefinition};

#[path = "gameplay_rapport_text.rs"]
mod rapport_text;

impl GameplayState {
    pub(super) fn rapport_value(&self, npc_id: &str) -> i32 {
        self.progression
            .relationships
            .get(npc_id)
            .copied()
            .unwrap_or_default()
    }

    /// Human-readable standing for the journal rapport tab.
    ///
    /// The tiers themselves are tuning and live in `config.balance.rapport`.
    /// FRIEND is reachable by seeing one errand through (+1 on accept, +2 on
    /// completion); CONFIDANT is reliability rather than story, since board
    /// orders pay rapport too; and accepting and finishing all three beats of an
    /// arc is worth exactly KIN. The top tier wants the arc finished as well as
    /// the number, so a player who supplied a lot of board orders is a confidant
    /// rather than kin.
    pub(super) fn rapport_tier_label(
        &self,
        data: &GameData,
        npc_id: &str,
        rapport: i32,
    ) -> &'static str {
        let tiers = &data.config.balance.rapport;
        if rapport >= tiers.kin && self.has_reached_trust(npc_id) {
            ui_copy("rapport_tier_kin")
        } else if rapport >= tiers.confidant {
            ui_copy("rapport_tier_confidant")
        } else if rapport >= tiers.friend {
            ui_copy("rapport_tier_friend")
        } else if rapport >= 1 {
            ui_copy("rapport_tier_acquaintance")
        } else {
            ui_copy("rapport_tier_stranger")
        }
    }

    fn friendship_milestone_id(npc_id: &str) -> String {
        format!("rapport_friend_{npc_id}")
    }

    fn trusted_milestone_id(npc_id: &str) -> String {
        format!("rapport_trusted_{npc_id}")
    }

    pub(super) fn has_reached_trust(&self, npc_id: &str) -> bool {
        self.has_journal_milestone(&Self::trusted_milestone_id(npc_id))
    }

    /// The parting gift, once every request this townsperson had is finished.
    /// Gated on the arc actually being complete rather than on a rapport number,
    /// because that is what it means — the number is only how the journal says
    /// it.
    pub(super) fn try_grant_trusted_gift(&mut self, data: &GameData, npc: &NpcDefinition) -> bool {
        if npc.id == "crow_guide" || npc.trusted_line.is_empty() {
            return false;
        }
        if self.has_reached_trust(&npc.id) {
            return false;
        }
        // Every beat done, and there was something to do in the first place.
        if !self.npc_has_been_helped(npc) || self.npc_active_quest(data, npc).is_some() {
            return false;
        }

        self.coins += npc.trusted_reward_coins;
        if !npc.trusted_reward_item_id.is_empty() && npc.trusted_reward_amount > 0 {
            *self
                .inventory
                .entry(npc.trusted_reward_item_id.clone())
                .or_insert(0) += npc.trusted_reward_amount;
        }

        self.push_journal_milestone(
            &Self::trusted_milestone_id(&npc.id),
            &ui_format("rapport_trusted_title", &[("name", &npc.name)]),
            &npc.trusted_line,
        );
        self.trigger_quest_complete_feedback(rapport_text::friendship_toast(&npc.name));
        self.runtime.status_text = npc.trusted_line.clone();
        true
    }

    pub(super) fn has_reached_friendship(&self, npc_id: &str) -> bool {
        self.has_journal_milestone(&Self::friendship_milestone_id(npc_id))
    }

    /// If this townsperson has just crossed into the friend tier and their gift
    /// has not been handed over yet, grant it once: coins/item reward, a warm
    /// line, and a permanent journal beat. Returns true when a gift was given,
    /// so the caller can surface it instead of the normal dialogue flow.
    pub(super) fn try_grant_friendship_gift(
        &mut self,
        data: &GameData,
        npc: &NpcDefinition,
    ) -> bool {
        // The Crow is a guide, not a townsperson, and has no friendship arc.
        if npc.id == "crow_guide" || npc.friendship_line.is_empty() {
            return false;
        }
        if self.rapport_value(&npc.id) < data.config.balance.rapport.friend
            || self.has_reached_friendship(&npc.id)
        {
            return false;
        }

        self.coins += npc.friendship_reward_coins;
        if !npc.friendship_reward_item_id.is_empty() && npc.friendship_reward_amount > 0 {
            *self
                .inventory
                .entry(npc.friendship_reward_item_id.clone())
                .or_insert(0) += npc.friendship_reward_amount;
        }

        // Record the beat permanently (also raises a journal-note toast).
        self.push_journal_milestone(
            &Self::friendship_milestone_id(&npc.id),
            &ui_format("rapport_friend_title", &[("name", &npc.name)]),
            &npc.friendship_line,
        );
        self.trigger_quest_complete_feedback(rapport_text::friendship_toast(&npc.name));
        self.runtime.status_text = rapport_text::friendship_status(data, npc);
        true
    }
}

#[cfg(test)]
#[path = "gameplay_rapport/tests.rs"]
mod tests;
