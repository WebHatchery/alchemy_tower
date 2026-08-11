//! Which opening hint to show next, and whether it has been shown before.
//!
//! Every one of these was invisible until the event banners were fixed, so this
//! is the first pass in which anybody could read them in the game. Two things
//! were wrong the moment they could be: the shown-flags lived in runtime state,
//! which is rebuilt on load, so the crow's introduction played again every time
//! a save was opened; and three hints named keys as literals — "Press J", "with
//! E" — while `input_bindings.json` owns those keys and the rest of the HUD
//! reads them from it.

use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;

pub(super) enum TutorialHintTone {
    Note,
    Goal,
    Success,
    Item,
}

/// One hint: the copy key that names it and the tone it is raised in. Keeping
/// them in a list rather than a ladder of `if`s is what lets a guard walk every
/// one of them.
struct TutorialHint {
    key: &'static str,
    tone: TutorialHintTone,
}

impl GameplayState {
    pub(super) fn take_next_tutorial_hint(
        &mut self,
        data: &GameData,
    ) -> Option<(String, TutorialHintTone)> {
        let hint = self.next_tutorial_hint(data)?;
        self.progression
            .shown_tutorial_hints
            .insert(hint.key.to_owned());
        Some((self.tutorial_hint_text(hint.key), hint.tone))
    }

    /// The next hint whose moment has come and which has not been shown.
    fn next_tutorial_hint(&self, data: &GameData) -> Option<TutorialHint> {
        let near_alchemy = self.tutorial_near_alchemy_station(data);
        let near_quest_npc = self.tutorial_near_quest_npc(data);
        let nearby_available_node = self.tutorial_near_available_gather_node(data);
        let unlockable_warp_here = self.tutorial_unlockable_warp_here(data);
        let has_quick_potions = !self.quick_potions(data).is_empty();
        let helped_mira = self
            .progression
            .completed_quests
            .contains("healing_for_mira");

        [
            (TUTORIAL_CROW_INTRO, TutorialHintTone::Note, true),
            (TUTORIAL_SAVE, TutorialHintTone::Note, true),
            (TUTORIAL_JOURNAL, TutorialHintTone::Goal, true),
            (
                TUTORIAL_ALCHEMY_OPEN,
                TutorialHintTone::Success,
                near_alchemy,
            ),
            (
                TUTORIAL_BREW_GOAL,
                TutorialHintTone::Goal,
                self.progression.total_brews == 0 && near_alchemy,
            ),
            (TUTORIAL_POTIONS, TutorialHintTone::Item, has_quick_potions),
            (
                TUTORIAL_GATHER,
                TutorialHintTone::Success,
                self.progression.herb_memories.is_empty() && nearby_available_node,
            ),
            (
                TUTORIAL_MIRA_DELIVERY,
                TutorialHintTone::Success,
                self.progression.total_brews > 0 && !helped_mira,
            ),
            (
                TUTORIAL_ROWAN_GOAL,
                TutorialHintTone::Goal,
                helped_mira && !self.progression.completed_quests.contains("glow_for_rowan"),
            ),
            (
                TUTORIAL_QUEST,
                TutorialHintTone::Goal,
                self.progression.started_quests.is_empty()
                    && self.progression.completed_quests.is_empty()
                    && (near_quest_npc || !self.available_board_quests(data).is_empty()),
            ),
            (
                TUTORIAL_DELIVERY_READY,
                TutorialHintTone::Success,
                self.tutorial_delivery_ready(data),
            ),
            (
                TUTORIAL_ROUTE_READY,
                TutorialHintTone::Note,
                unlockable_warp_here,
            ),
        ]
        .into_iter()
        .find(|(key, _, due)| *due && !self.progression.shown_tutorial_hints.contains(*key))
        .map(|(key, tone, _)| TutorialHint { key, tone })
    }

    /// A hint's words, with the keys it names filled in from the bindings.
    ///
    /// `tutorial_potions` used to be formatted with a `quick_potions`
    /// substitution the copy had no placeholder for — the belt keys were looked
    /// up, joined and dropped — which is the same shape as the banner that
    /// discarded its own text, one layer down.
    pub(super) fn tutorial_hint_text(&self, key: &str) -> String {
        let bindings = input_bindings();
        match key {
            TUTORIAL_SAVE => ui_format(
                key,
                &[
                    ("save", &bindings.global.save),
                    ("load", &bindings.global.load),
                ],
            ),
            TUTORIAL_JOURNAL => ui_format(key, &[("journal", &bindings.global.journal)]),
            TUTORIAL_ALCHEMY_OPEN => ui_format(
                key,
                &[
                    ("interact", &bindings.global.interact),
                    ("alchemy", &bindings.alchemy.open),
                ],
            ),
            TUTORIAL_GATHER | TUTORIAL_DELIVERY_READY | TUTORIAL_ROUTE_READY => {
                ui_format(key, &[("interact", &bindings.global.interact)])
            }
            TUTORIAL_POTIONS => ui_format(
                key,
                &[("quick_potions", &bindings.global.quick_potions.join(", "))],
            ),
            _ => ui_copy(key).to_owned(),
        }
    }
}

impl GameplayState {
    /// Every hint as the player would read it, for the guard that keeps them
    /// inside the banner they are raised in.
    #[cfg(test)]
    pub(crate) fn tutorial_hint_texts(&self) -> Vec<(&'static str, String)> {
        TUTORIAL_HINT_KEYS
            .into_iter()
            .map(|key| (key, self.tutorial_hint_text(key)))
            .collect()
    }
}

pub(super) const TUTORIAL_CROW_INTRO: &str = "tutorial_crow_intro";
pub(super) const TUTORIAL_SAVE: &str = "tutorial_save";
pub(super) const TUTORIAL_JOURNAL: &str = "tutorial_journal";
pub(super) const TUTORIAL_ALCHEMY_OPEN: &str = "tutorial_alchemy_open";
pub(super) const TUTORIAL_BREW_GOAL: &str = "tutorial_brew_goal";
pub(super) const TUTORIAL_POTIONS: &str = "tutorial_potions";
pub(super) const TUTORIAL_GATHER: &str = "tutorial_gather";
pub(super) const TUTORIAL_MIRA_DELIVERY: &str = "tutorial_mira_delivery";
pub(super) const TUTORIAL_ROWAN_GOAL: &str = "tutorial_rowan_goal";
pub(super) const TUTORIAL_QUEST: &str = "tutorial_quest";
pub(super) const TUTORIAL_DELIVERY_READY: &str = "tutorial_delivery_ready";
pub(super) const TUTORIAL_ROUTE_READY: &str = "tutorial_route_ready";

/// Every hint the game can raise, for the guards that walk them.
#[cfg(test)]
pub(super) const TUTORIAL_HINT_KEYS: [&str; 12] = [
    TUTORIAL_CROW_INTRO,
    TUTORIAL_SAVE,
    TUTORIAL_JOURNAL,
    TUTORIAL_ALCHEMY_OPEN,
    TUTORIAL_BREW_GOAL,
    TUTORIAL_POTIONS,
    TUTORIAL_GATHER,
    TUTORIAL_MIRA_DELIVERY,
    TUTORIAL_ROWAN_GOAL,
    TUTORIAL_QUEST,
    TUTORIAL_DELIVERY_READY,
    TUTORIAL_ROUTE_READY,
];

#[cfg(test)]
#[path = "gameplay_tutorial_hint_selection/tests.rs"]
mod tests;
