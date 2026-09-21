pub(super) const SLOT_COUNT: usize = 3;
use crate::data::AlchemyTuning;

#[derive(Clone, Debug)]
pub(super) struct AlchemySession {
    pub(super) index: usize,
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing_index: usize,
    pub(super) timing_ids: Vec<String>,
    pub(super) slots: [Option<String>; SLOT_COUNT],
    pub(super) catalyst: Option<String>,
}

impl Default for AlchemySession {
    fn default() -> Self {
        Self {
            index: 0,
            heat: 2,
            stirs: 0,
            timing_index: 0,
            timing_ids: Vec::new(),
            slots: [None, None, None],
            catalyst: None,
        }
    }
}

impl AlchemySession {
    pub(super) fn with_tuning(tuning: &AlchemyTuning) -> Self {
        Self {
            heat: tuning.initial_heat,
            timing_ids: tuning.timing_ids.clone(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct SavedAlchemySetup {
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing_index: usize,
    pub(super) slots: [Option<String>; SLOT_COUNT],
    pub(super) catalyst: Option<String>,
}

#[cfg(test)]
#[path = "gameplay_alchemy_types/tests.rs"]
mod tests;
