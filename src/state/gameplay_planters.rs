use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry, StationDefinition};

impl GameplayState {
    pub(super) fn interact_with_planter(&mut self, data: &GameData, station: &StationDefinition) {
        let existing_state = self.progression.planter_states.get(&station.id).cloned();
        // A bed nobody has touched yet has no entry at all, and that counts as
        // empty. Reading the candidate off the *existing* entry meant the first
        // approach to a fresh bed always fell through to "you have no seed for
        // this" while the player was holding one, and only the second worked.
        let bed_is_empty = existing_state
            .as_ref()
            .map(|state| state.planted_item_id.is_empty())
            .unwrap_or(true);
        let candidate = bed_is_empty
            .then(|| self.planter_seed_choice(data, station))
            .flatten();
        let mutation_candidate = existing_state.as_ref().and_then(|state| {
            (!state.planted_item_id.is_empty()
                && !state.ready
                && state.mutation_formula_id.is_empty())
            .then(|| self.planter_mutation_candidate(data, &state.planted_item_id))
            .flatten()
        });
        let mut state = self
            .progression
            .planter_states
            .remove(&station.id)
            .unwrap_or(PlanterStateEntry {
                station_id: station.id.clone(),
                planted_item_id: String::new(),
                planted_day: self.world.day_index,
                ready: false,
                tended_day: 0,
                tended_days: 0,
                growth_days: 0,
                mutation_formula_id: String::new(),
                mutation_yield_bonus: 0,
                mutation_growth_bonus_days: 0,
                mutation_note: String::new(),
            });
        if state.ready && !state.planted_item_id.is_empty() {
            self.harvest_planter(data, station, &mut state);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }
        if !state.planted_item_id.is_empty() {
            self.tend_or_report_planter(data, station, &mut state, mutation_candidate.as_ref());
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }

        let Some(item_id) = candidate else {
            self.report_missing_planter_seed(data, station);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        };
        self.plant_seed_in_planter(data, station, &mut state, item_id);
        self.progression
            .planter_states
            .insert(station.id.clone(), state);
    }

    pub(super) fn planter_seed_choice(
        &self,
        data: &GameData,
        station: &StationDefinition,
    ) -> Option<String> {
        self.inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| planter_accepts(station, item, item_id))
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
    }
}

/// Whether this bed will take that seed. A bed that names its seeds means what
/// it says: the list is the rule, not a filter applied on top of a rarity floor.
/// Previously the floor won, so a common herb named in `planter_seed_ids` was
/// listed to the player as accepted and then silently refused.
pub(crate) fn planter_accepts(
    station: &StationDefinition,
    item: &crate::data::ItemDefinition,
    item_id: &str,
) -> bool {
    if item.category != ItemCategory::Ingredient {
        return false;
    }
    if station.planter_seed_ids.is_empty() {
        // An unspecialised bed still holds the old line: rare stock only.
        return item.rarity >= 2;
    }
    station.planter_seed_ids.iter().any(|seed| seed == item_id)
}

#[cfg(test)]
#[path = "gameplay_planters/tests.rs"]
mod tests;
