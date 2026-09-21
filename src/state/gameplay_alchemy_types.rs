pub(super) const SLOT_COUNT: usize = 3;
pub(super) const ALCHEMY_TIMINGS: [&str; 3] = ["steady", "early", "late"];

#[derive(Clone, Debug)]
pub(super) struct AlchemySession {
    pub(super) index: usize,
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing_index: usize,
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
            slots: [None, None, None],
            catalyst: None,
        }
    }
}

impl AlchemySession {
    pub(super) fn with_initial_heat(initial_heat: i32) -> Self {
        Self {
            heat: initial_heat,
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
