use super::GameplayState;
use crate::content::{narrative_text, NarrativeReaction};
use crate::data::{GameData, NpcDefinition, QuestDefinition};

#[path = "gameplay_npc_dialogue_text.rs"]
mod npc_dialogue_text;

pub(super) struct NpcDialogueSelection<'a> {
    pub(super) start: &'a str,
    pub(super) progress: &'a str,
    pub(super) complete: &'a str,
}

impl GameplayState {
    /// The one step of this townsperson's arc that is currently live: the first
    /// request they have not finished. `None` once the whole chain is done,
    /// which every caller already treats as "nothing left to ask of them".
    pub(super) fn npc_active_quest<'a>(
        &self,
        data: &'a GameData,
        npc: &NpcDefinition,
    ) -> Option<&'a QuestDefinition> {
        npc.quest_chain()
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .find(|quest| !self.progression.completed_quests.contains(&quest.id))
    }

    /// Whether the player has finished anything at all for this townsperson.
    /// Drives the warmer "you already helped me" lines, which must survive the
    /// arc moving on to its next beat.
    pub(super) fn npc_has_been_helped(&self, npc: &NpcDefinition) -> bool {
        npc.quest_chain()
            .iter()
            .any(|quest_id| self.progression.completed_quests.contains(quest_id))
    }

    /// What this townsperson says about being where they currently are, if the
    /// stop they are on has anything to say. A stop at home does not — that is
    /// where the rest of their dialogue is already set.
    pub(super) fn npc_while_here_line<'a>(&self, npc: &'a NpcDefinition) -> Option<&'a str> {
        let window = self.current_time_window();
        npc.schedule
            .iter()
            .find(|entry| entry.time_window == window)
            .map(|entry| entry.while_here_line.as_str())
            .filter(|line| !line.is_empty())
    }

    pub(super) fn phase1_town_recovery_reached(&self) -> bool {
        self.has_journal_milestone("greenhouse_repaired")
            || self
                .progression
                .completed_quests
                .contains("cultivation_for_brin")
    }

    pub(super) fn phase1_first_relief_reached(&self) -> bool {
        self.has_journal_milestone("first_town_relief")
            || self
                .progression
                .completed_quests
                .contains("healing_for_mira")
    }

    pub(super) fn phase1_first_brew_reached(&self) -> bool {
        self.has_journal_milestone("first_true_brew") || self.progression.total_brews > 0
    }

    pub(super) fn npc_dialogue_selection<'a>(
        &'a self,
        data: &'a GameData,
        npc: &'a NpcDefinition,
    ) -> NpcDialogueSelection<'a> {
        let mut selection = NpcDialogueSelection {
            start: npc.dialogue_complete.as_str(),
            progress: npc.dialogue_complete.as_str(),
            complete: npc.dialogue_complete.as_str(),
        };

        if npc.id == "crow_guide" {
            let crow = &npc.crow_phase1_dialogue;
            if !crow.first_meeting.is_empty() {
                let line = if self.phase1_town_recovery_reached()
                    && !crow.first_tower_restoration.is_empty()
                {
                    crow.first_tower_restoration.as_str()
                } else if self.phase1_first_relief_reached()
                    && !crow.first_quest_complete.is_empty()
                {
                    crow.first_quest_complete.as_str()
                } else if self.phase1_first_brew_reached() && !crow.first_brew.is_empty() {
                    crow.first_brew.as_str()
                } else {
                    crow.first_meeting.as_str()
                };
                selection.start = line;
                selection.progress = line;
                selection.complete = line;
            }
            return selection;
        }

        let phase1 = &npc.phase1_dialogue;
        let quest = self.npc_active_quest(data, npc);
        let quest_started = quest
            .map(|quest| self.progression.started_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_completed = self.npc_has_been_helped(npc);
        let quest_available = quest
            .map(|quest| self.quest_is_available(quest))
            .unwrap_or(false);

        // A line authored on the live step of an arc is the most specific thing
        // this NPC has to say, so it takes the slot it belongs in — but it used
        // to take both and return outright, which left the phase-1
        // `active_request` reminder unreachable for seven of the eight, and
        // Mira's `intro` unreachable altogether because her first errand is
        // available from the opening minute.
        //
        // Accepted: the beat is the reminder, and `active_request` opens.
        // Merely offered: the beat is the pitch, and the NPC's own voice opens
        // — which is where the *reason* for the errand lives, rather than
        // jumping straight to "make me X".
        let beat = quest
            .filter(|_| quest_started || quest_available)
            .map(|quest| {
                if quest_started {
                    quest.giver_active_line.as_str()
                } else {
                    quest.giver_intro_line.as_str()
                }
            })
            .filter(|beat| !beat.is_empty());

        // What they say with nothing of yours pending, most specific first.
        //
        // The town-recovery observation used to be checked ahead of all of this
        // and returned outright, so from the moment the greenhouse reopened —
        // which is early — it was the only thing eight townsfolk could say.
        // `post_help_relief` was reachable for three of them, and only because
        // their arcs happened to finish first.
        let arc_finished = quest.is_none();
        if arc_finished && !npc.dialogue_complete.is_empty() {
            // Everything they asked for is done. This is their settled word on
            // it, and the only place the schema's own dialogue line is heard.
            selection.complete = npc.dialogue_complete.as_str();
        } else if quest_completed && !phase1.post_help_relief.is_empty() {
            selection.complete = phase1.post_help_relief.as_str();
        } else if self.phase1_town_recovery_reached()
            && !phase1.town_recovery_observation.is_empty()
        {
            selection.complete = phase1.town_recovery_observation.as_str();
        }

        // How they open, most specific first.
        let opener = if quest_started && !phase1.active_request.is_empty() {
            // Working on it: the terse reminder of what was asked for.
            Some(phase1.active_request.as_str())
        } else if let Some(here) = self
            .npc_while_here_line(npc)
            .filter(|_| !quest_started && !quest_available)
        {
            // Caught somewhere they do not work, with nothing of yours pending.
            // Where a townsperson goes at which hour has been authored since
            // before the schedule had a reader for anything but their sprite,
            // and until now they said the same words wherever they stood.
            Some(here)
        } else if !quest_started
            && !quest_available
            && self.phase1_town_recovery_reached()
            && !phase1.town_recovery_observation.is_empty()
        {
            // Nothing of yours pending and the town on its feet.
            Some(phase1.town_recovery_observation.as_str())
        } else if self.phase1_first_brew_reached() && !phase1.pre_help_concern.is_empty() {
            Some(phase1.pre_help_concern.as_str())
        } else if !phase1.intro.is_empty() {
            Some(phase1.intro.as_str())
        } else {
            None
        };
        if let Some(opener) = opener {
            selection.start = opener;
            selection.progress = opener;
        }

        // The arc beat carries the conversation on from that opener rather than
        // replacing it, so both the errand and the voice asking get heard.
        if let Some(beat) = beat {
            selection.progress = beat;
            if opener.is_none() {
                selection.start = beat;
            }
        }

        selection
    }

    /// Key for remembering that a line has been said.
    ///
    /// Reactions carry no id, and speaker-plus-order will not serve: seven of
    /// them already share an order with a sibling, which would mark both said
    /// the moment either was. Hashing the words themselves gives a short stable
    /// key that survives reordering and re-authoring elsewhere in the file, and
    /// two byte-identical lines from one speaker are interchangeable anyway.
    fn reaction_key(reaction: &NarrativeReaction) -> String {
        // FNV-1a. Small, dependency-free, and stable across builds — which a
        // key written into save files has to be.
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in reaction.npc_id.bytes().chain(reaction.line.bytes()) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x1000_0000_01b3);
        }
        format!("{hash:016x}")
    }

    /// The reaction this townsperson would offer next.
    ///
    /// This used to be `max_by_key(order)` over everything earned, and earning
    /// is monotonic — so a line that came due at the same moment as a later one,
    /// or behind an ancestor of it, lost every time and could never be spoken.
    /// Thirty-six of the hundred and sixty authored lines were unreachable that
    /// way, including three of Ione's in a row.
    ///
    /// Earned-but-unsaid lines now go first, earliest first, so a run of beats
    /// that came due together is worked through one conversation at a time. Once
    /// everything is said the latest line stands as their current word, which is
    /// what the old behaviour was reaching for.
    ///
    /// The conversation still only moves forward: a line that becomes earned
    /// after later ones have already been spoken is skipped rather than dragging
    /// the townsperson back to an older beat. Lines that come due *together* are
    /// all above the last thing said, so they still get their turn.
    fn npc_phase1_followup(&self, npc_id: &str) -> Option<&'static NarrativeReaction> {
        let earned = || {
            narrative_text()
                .reactions
                .iter()
                .filter(move |reaction| reaction.npc_id == npc_id)
                .filter(|reaction| self.reaction_is_earned(reaction))
        };
        let already_said = |reaction: &NarrativeReaction| {
            self.progression
                .spoken_reactions
                .contains(&Self::reaction_key(reaction))
        };
        let furthest_said = earned()
            .filter(|reaction| already_said(reaction))
            .map(|reaction| reaction.order)
            .max();

        earned()
            .filter(|reaction| !already_said(reaction))
            .filter(|reaction| furthest_said.is_none_or(|said| reaction.order >= said))
            .min_by_key(|reaction| reaction.order)
            .or_else(|| earned().max_by_key(|reaction| reaction.order))
    }

    pub(super) fn npc_phase1_followup_line(&self, npc_id: &str) -> Option<&'static str> {
        self.npc_phase1_followup(npc_id)
            .map(|reaction| reaction.line.as_str())
    }

    /// Note that the line currently on offer has now been said. Called when the
    /// player advances the conversation — that is the moment they have read it —
    /// so the next one is waiting the next time they come by.
    pub(super) fn mark_followup_spoken(&mut self, npc_id: &str) {
        let Some(key) = self.npc_phase1_followup(npc_id).map(Self::reaction_key) else {
            return;
        };
        self.progression.spoken_reactions.insert(key);
    }

    fn reaction_is_earned(&self, reaction: &NarrativeReaction) -> bool {
        let quest_done = reaction.after_quest.is_empty()
            || self
                .progression
                .completed_quests
                .contains(&reaction.after_quest);
        let milestone_done = reaction.after_milestone.is_empty()
            || self.has_journal_milestone(&reaction.after_milestone);
        quest_done && milestone_done
    }

    pub(super) fn append_npc_story_line(&self, npc_id: &str, base: String) -> String {
        let extra = match self.npc_phase1_followup_line(npc_id) {
            Some(extra) => extra,
            None => return base,
        };

        if base.contains(extra) {
            return base;
        }

        npc_dialogue_text::with_followup(&base, extra)
    }
}

#[cfg(test)]
#[path = "gameplay_npc_dialogue/tests.rs"]
mod tests;
