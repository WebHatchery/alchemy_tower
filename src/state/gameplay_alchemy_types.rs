pub(super) const SLOT_COUNT: usize = 3;
pub(super) const ALCHEMY_TIMINGS: [&str; 3] = ["steady", "early", "late"];

/// What the heat dial can actually be set to. A recipe or morph asking for
/// anything outside this range can never fire, however well authored it is.
pub(crate) const ALCHEMY_MIN_HEAT: i32 = 1;
pub(crate) const ALCHEMY_MAX_HEAT: i32 = 3;

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
