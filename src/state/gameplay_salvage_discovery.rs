//! Formulas the player finds without being told one exists.
//!
//! A mixture no recipe describes falls to the salvage path: capped quality and
//! one of four consolation bottles. That made off-book brewing a dead end —
//! the game's only answer to curiosity was "not that", forever, however many
//! times you tried it.
//!
//! Doing the same off-book thing repeatedly is now how a formula gets found.
//! The mixture is remembered by signature, the salvage cap lifts as it becomes
//! familiar, and on the third clean attempt the journal records it as something
//! the player worked out rather than read. This is the one place the engine can
//! still surprise somebody who is not following instructions, so it is worth
//! the bookkeeping.

use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::content::ui_format;
use crate::data::{GameData, StationDefinition};

impl GameplayState {
    /// What makes two off-book brews "the same thing": the bench and the
    /// reagents, regardless of the order they went in. Heat, stirs and timing
    /// are deliberately excluded — salvage does not read them, and requiring
    /// them to match would make the discovery depend on something the player
    /// cannot see going wrong.
    pub(super) fn salvage_signature(station: &StationDefinition, selected: &[String]) -> String {
        let mut reagents = selected.to_vec();
        reagents.sort();
        format!("{}|{}", station.id, reagents.join("+"))
    }

    /// How many times this exact off-book mixture has been made before.
    pub(super) fn salvage_familiarity(
        &self,
        station: &StationDefinition,
        selected: &[String],
    ) -> u32 {
        self.progression
            .salvage_familiarity
            .get(&Self::salvage_signature(station, selected))
            .copied()
            .unwrap_or_default()
    }

    /// Whether this mixture has been worked out — the player has made it enough
    /// times for the tower to treat it as a formula rather than an accident.
    pub(super) fn salvage_is_discovered(
        &self,
        data: &GameData,
        station: &StationDefinition,
        selected: &[String],
    ) -> bool {
        self.salvage_familiarity(station, selected)
            >= data.config.balance.salvage.discovery_attempts
    }

    /// Record one off-book brew and, on the attempt that earns it, journal the
    /// find. Returns true when this brew was the one that crossed over.
    pub(super) fn record_salvage_attempt(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        selected: &[String],
        resolution: &BrewResolution<'_>,
    ) -> bool {
        // Only mixtures that actually came to something count. A pot of mud
        // brewed forty times is not a discovery, and letting it become one
        // would make the celebration meaningless.
        if resolution.quality_score == 0 {
            return false;
        }

        let signature = Self::salvage_signature(station, selected);
        let attempts = self
            .progression
            .salvage_familiarity
            .entry(signature.clone())
            .or_insert(0);
        *attempts += 1;
        if *attempts != data.config.balance.salvage.discovery_attempts {
            return false;
        }

        let reagents = selected
            .iter()
            .map(|item_id| data.item_name(item_id))
            .collect::<Vec<_>>()
            .join(", ");
        let output = data.item_name(&resolution.output_item_id);
        self.push_journal_milestone(
            &format!("found_formula_{signature}"),
            &ui_format("journal_found_formula_title", &[("output", output)]),
            &ui_format(
                "journal_found_formula_text",
                &[
                    ("reagents", &reagents),
                    ("station", &station.name),
                    ("output", output),
                ],
            ),
        );
        true
    }
}

#[cfg(test)]
#[path = "gameplay_salvage_discovery/tests.rs"]
mod tests;
