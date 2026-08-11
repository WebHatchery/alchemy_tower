//! What a day of work costs, and what a restorative is for.
//!
//! Vitality shipped as a number that only ever went up. `apply_effect` raised
//! it and the HUD drew it, and nothing anywhere took any away — so `Restore`,
//! which 51 of the game's 110 authored effect blocks use, healed a wound the
//! game could not inflict. Half the potions in the tower were flavour text.
//!
//! It is the working day now. Standing over a cauldron costs; walking a route
//! and bending down costs less. Run out and you wake up at home having lost the
//! morning, the same collapse the small hours already cause. Sleeping in a bed
//! by choice gives the day back in full; being carried home does not quite.
//!
//! That makes a restorative the thing that buys another hour at the bench,
//! which is what a potion called Healing Draught should be for.

use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    /// Spend some of the day. Stops at zero rather than going negative — the
    /// collapse is handled by `handle_sleep_pressure`, so that running out
    /// mid-brew finishes the brew first and drags the player home afterwards
    /// rather than interrupting itself.
    pub(super) fn spend_vitality(&mut self, amount: f32) {
        self.vitality = (self.vitality - amount).clamp(0.0, 100.0);
    }

    /// Nothing left in the day.
    pub(super) fn is_exhausted(&self) -> bool {
        self.vitality <= 0.0
    }

    pub(super) fn restore_vitality_to(&mut self, value: f32) {
        self.vitality = value.clamp(0.0, 100.0);
    }

    pub(super) fn spend_brewing_vitality(&mut self, data: &GameData) {
        self.spend_vitality(data.config.balance.vitality.brew_cost);
    }

    pub(super) fn spend_gathering_vitality(&mut self, data: &GameData) {
        self.spend_vitality(data.config.balance.vitality.gather_cost);
    }
}

#[cfg(test)]
#[path = "gameplay_vitality/tests.rs"]
mod tests;
