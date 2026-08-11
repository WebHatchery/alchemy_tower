//! Taking a bottle apart for what is left of its reagents.
//!
//! This returned every ingredient at full amount for one bottle, which made
//! the archive console a **reagent printer**: nine recipes brew more than one
//! bottle at a time, and two of them turn three reagents into three bottles —
//! disassemble all three and three reagents have become nine, repeatable
//! forever, with no travel and no season to wait for.
//!
//! A bottle now gives back its *share* of the pour — the ingredient amount
//! divided by how many bottles the recipe makes, rounded down — so taking a
//! whole batch apart can never yield more than the batch cost.
//!
//! The mastery bottle is deliberately *not* in the divisor, and the arithmetic
//! is why. A brew costs 5 vitality and a gather costs 1.5, so gathering yields
//! about 3.3 units per 5 vitality. A mastered one-bottle recipe hands back its
//! two bottles' worth of reagents for the same 5 vitality — 2 or 3 units for a
//! two- or three-reagent recipe — which is *worse* than walking out and picking
//! them. Counting mastery in the divisor would round every ordinary recipe's
//! return to nothing and kill the feature to close a hole that is not open.
//!
//! The batch recipes were the hole: three reagents into three bottles, each
//! handing back three, is six free reagents per brew and no travel.

use super::GameplayState;
use crate::data::{GameData, RecipeDefinition};

#[path = "gameplay_disassembly_text.rs"]
mod disassembly_text;

/// What one bottle of this recipe gives back: its share of the pour, rounded
/// down.
pub(super) fn salvage_share(recipe: &RecipeDefinition, amount: u32) -> u32 {
    amount / recipe.output_amount.max(1)
}

impl GameplayState {
    pub(super) fn available_disassembly_recipes<'a>(
        &self,
        data: &'a GameData,
    ) -> Vec<&'a RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .filter(|recipe| {
                self.inventory
                    .get(&recipe.output_item_id)
                    .copied()
                    .unwrap_or_default()
                    > 0
            })
            // A batch recipe whose every reagent divides away to nothing has
            // nothing to give back, and offering it would spend a bottle for
            // an empty hand.
            .filter(|recipe| {
                recipe
                    .ingredients
                    .iter()
                    .any(|ingredient| salvage_share(recipe, ingredient.amount) > 0)
            })
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        recipes
    }

    pub(super) fn disassemble_recipe(&mut self, data: &GameData, recipe: &RecipeDefinition) {
        if self
            .inventory
            .get(&recipe.output_item_id)
            .copied()
            .unwrap_or_default()
            == 0
        {
            self.runtime.status_text = disassembly_text::cannot_disassemble(&recipe.name);
            return;
        }
        self.take_from_inventory(&recipe.output_item_id, 1);

        let mut returned = Vec::new();
        for ingredient in &recipe.ingredients {
            let share = salvage_share(recipe, ingredient.amount);
            if share == 0 {
                continue;
            }
            *self
                .inventory
                .entry(ingredient.item_id.clone())
                .or_insert(0) += share;
            self.note_inventory_observation(data, &ingredient.item_id);
            returned.push(disassembly_text::returned_input(
                data,
                &ingredient.item_id,
                share,
            ));
        }

        self.trigger_disassembly_feedback(disassembly_text::toast(&recipe.name));
        self.runtime.status_text = disassembly_text::disassembled(&recipe.name, &returned);
    }
}

#[cfg(test)]
#[path = "gameplay_disassembly/tests.rs"]
mod tests;
