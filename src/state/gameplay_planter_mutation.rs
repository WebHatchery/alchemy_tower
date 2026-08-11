use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry};

#[path = "gameplay_planter_mutation_text.rs"]
mod mutation_text;

impl GameplayState {
    /// Which formula a bed would take and which bottle it would eat.
    ///
    /// A mutation asks for an effect kind rather than a named brew — the one
    /// place in the game that already worked that way — so any glow bottle in
    /// the bag will do. It used to take the *first* one it found while walking
    /// the inventory, which is a `BTreeMap` and therefore alphabetical by item
    /// id: planting a bed could spend a 284-coin Heldstar Vigil because `h`
    /// sorts before `k`, while a Kindling Tonic worth 22 sat beside it.
    ///
    /// Every other spend in the game already knows better — a delivery hands
    /// over the worst bottle that qualifies, a sale parts with the worst held,
    /// the bench pours the best deliberately — so this takes the cheapest thing
    /// that fits, which is also exactly what the sinkless tail is for.
    pub(super) fn planter_mutation_candidate(
        &self,
        data: &GameData,
        planted_item_id: &str,
    ) -> Option<(String, String)> {
        for formula in data.mutation_formulas_for_seed(planted_item_id) {
            let cheapest = self
                .inventory
                .iter()
                .filter(|(item_id, amount)| **amount > 0 && *item_id != planted_item_id)
                .filter_map(|(item_id, _)| Some((item_id, data.item(item_id)?)))
                .filter(|(_, item)| item.category == ItemCategory::Potion)
                .filter(|(_, item)| {
                    formula.required_effect_kind.is_empty()
                        || item
                            .effects
                            .iter()
                            .any(|effect| effect.kind.as_str() == formula.required_effect_kind)
                })
                .min_by_key(|(item_id, item)| (item.base_value, (*item_id).clone()));
            if let Some((item_id, _)) = cheapest {
                return Some((formula.id.clone(), item_id.clone()));
            }
        }
        None
    }

    pub(super) fn apply_planter_mutation(
        &mut self,
        data: &GameData,
        state: &mut PlanterStateEntry,
        candidate: Option<&(String, String)>,
    ) -> Option<String> {
        if !state.mutation_formula_id.is_empty() {
            return None;
        }
        let (formula_id, catalyst_item_id) = candidate?;
        let formula = data
            .mutation_formulas
            .iter()
            .find(|formula| formula.id == *formula_id)?;

        self.inventory.get(catalyst_item_id)?;
        self.take_from_inventory(catalyst_item_id, 1);

        state.mutation_formula_id = formula.id.clone();
        state.mutation_yield_bonus = formula.yield_bonus;
        state.mutation_growth_bonus_days = formula.growth_bonus_days;
        state.mutation_note = formula.mutation_note.clone();

        // Name the bottle as well as the bed. A mutation costs a brew, and
        // until the banners were fixed the only thing the player was told was
        // that something had changed colour.
        self.trigger_planter_mutation_feedback(mutation_text::toast(
            data,
            &state.planted_item_id,
            catalyst_item_id,
        ));

        Some(mutation_text::status(
            data,
            catalyst_item_id,
            &formula.mutation_note,
        ))
    }
}

#[cfg(test)]
#[path = "gameplay_planter_mutation/tests.rs"]
mod tests;
