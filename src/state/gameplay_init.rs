use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_journal_support::initial_journal_milestones;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_progression_types::ProgressionState;
use super::gameplay_runtime_types::RuntimeState;
use super::gameplay_support::starting_day_time;
use super::gameplay_world_types::WorldState;
use super::GameplayState;
use crate::data::{GameData, PlayerGender};
use std::collections::BTreeMap;

impl GameplayState {
    pub(crate) fn new(data: &GameData) -> Self {
        Self::new_with_gender(data, PlayerGender::Male)
    }

    pub(crate) fn new_with_gender(data: &GameData, player_gender: PlayerGender) -> Self {
        let mut state = Self {
            player_gender,
            world: WorldState::new(data, starting_day_time(data)),
            progression: ProgressionState::new(initial_journal_milestones()),
            coins: 24,
            vitality: 100.0,
            inventory: BTreeMap::new(),
            runtime: RuntimeState::new(data),
            ui: OverlayState::new_gameplay(),
            alchemy: AlchemySession::with_tuning(&data.config.balance.alchemy),
            mastery_threshold: crate::alchemy::mastery_threshold(&data.config.balance.alchemy),
        };
        state.seed_starter_recipes(data);
        state.initialize_npc_motion_states(data);
        state.refresh_available_nodes(data);
        state
    }

    fn seed_starter_recipes(&mut self, data: &GameData) {
        for recipe in &data.recipes {
            if recipe.starter_known {
                self.progression.known_recipes.insert(recipe.id.clone());
            }
        }
    }
}
