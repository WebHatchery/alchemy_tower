use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, RuneRecipeDefinition, StationDefinition};

#[path = "gameplay_rune_recipe_text.rs"]
mod rune_recipe_text;

impl GameplayState {
    pub(super) fn available_rune_recipes<'a>(
        &self,
        data: &'a GameData,
        station: &StationDefinition,
    ) -> Vec<&'a RuneRecipeDefinition> {
        data.rune_recipes
            .iter()
            .filter(|recipe| recipe.station_id == station.id)
            .filter(|recipe| self.has_rune_recipe_inputs(recipe))
            .collect()
    }

    pub(super) fn apply_rune_recipe(&mut self, data: &GameData, recipe: &RuneRecipeDefinition) {
        if !self.transform_worst_held_bottle(data, &recipe.input_item_id, &recipe.output_item_id) {
            return;
        }
        self.take_from_inventory(&recipe.rune_item_id, 1);
        self.note_inventory_observation(data, &recipe.output_item_id);
        self.ensure_potion_memory_learned(&recipe.output_item_id, None);
        let milestone = &narrative_text().milestones.first_rune_imbuing;
        self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        self.runtime.status_text = rune_recipe_text::imbued(data, recipe);
    }

    fn has_rune_recipe_inputs(&self, recipe: &RuneRecipeDefinition) -> bool {
        self.inventory
            .get(&recipe.input_item_id)
            .copied()
            .unwrap_or_default()
            > 0
            && self
                .inventory
                .get(&recipe.rune_item_id)
                .copied()
                .unwrap_or_default()
                > 0
    }
}

#[cfg(test)]
#[path = "gameplay_rune_recipes/tests.rs"]
mod tests;
