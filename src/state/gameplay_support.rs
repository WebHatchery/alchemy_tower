use super::GameplayState;
use crate::data::GameData;

#[path = "gameplay_support_text.rs"]
mod support_text;

impl GameplayState {
    pub(super) fn update_area_banner(&mut self, data: &GameData, frame_time: f32) {
        self.runtime.area_banner_seconds = (self.runtime.area_banner_seconds - frame_time).max(0.0);
        if self.runtime.area_banner_area_id != self.world.current_area_id {
            self.runtime.area_banner_area_id = self.world.current_area_id.clone();
            self.runtime.area_banner_label = data
                .area(&self.world.current_area_id)
                .map(|area| area.name.clone())
                .unwrap_or_default();
            self.runtime.area_banner_seconds = 2.6;
        }
    }
}

pub(super) fn quality_band_rank(band: &str) -> u8 {
    support_text::quality_band_rank(band)
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    support_text::planter_stage_label(growth_days, total_days)
}

pub(super) fn starting_day_time(data: &GameData) -> f32 {
    data.config.day_length_seconds * 0.30
}

#[cfg(test)]
#[path = "gameplay_support/tests.rs"]
mod tests;
