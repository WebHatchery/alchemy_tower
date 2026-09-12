use super::gameplay_overlay_types::OverlayScreen;
use super::gameplay_overlay_window;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::input::{
    cancel_pressed, confirm_pressed, journal_pressed, left_mouse_pressed, mouse_position_point,
    rect_contains_point, select_next_pressed, select_previous_pressed, switch_next_pressed,
    switch_previous_pressed,
};
use crate::journal_layout::{journal_next_rect, journal_previous_rect, journal_tab_rect};

#[path = "gameplay_overlay_input_text.rs"]
mod overlay_input_text;

impl GameplayState {
    pub(super) fn handle_active_overlay_inputs(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
    ) -> bool {
        let Some(overlay) = self.overlay().cloned() else {
            return false;
        };

        // Overlay hit-testing reads the mouse in UI design space, so enable the
        // scale transform for the duration of this dispatch.
        crate::ui_scale::set_overlay_mouse(true);
        if left_mouse_pressed()
            && rect_contains_point(crate::ui::overlay_close_rect(), mouse_position_point())
        {
            self.clear_overlay();
            self.runtime.status_text = self.closed_overlay_status(&overlay);
            crate::ui_scale::set_overlay_mouse(false);
            return true;
        }
        match overlay {
            OverlayScreen::Dialogue(_) => self.handle_dialogue_inputs(data),
            OverlayScreen::Shop => self.handle_shop_inputs(data),
            OverlayScreen::Rune => self.handle_rune_inputs(data),
            OverlayScreen::Archive => self.handle_archive_inputs(data),
            OverlayScreen::Ending => {
                if cancel_pressed() {
                    self.clear_overlay();
                } else if confirm_pressed()
                    || (left_mouse_pressed()
                        && rect_contains_point(
                            crate::ui::overlay_primary_action_rect(),
                            mouse_position_point(),
                        ))
                {
                    // Read on through the epilogue, then close out.
                    if self.ui.ending_page + 1 < self.epilogue_page_count() {
                        self.ui.ending_page += 1;
                    } else {
                        self.clear_overlay();
                    }
                }
            }
            OverlayScreen::QuestBoard => self.handle_quest_board_inputs(data),
            OverlayScreen::Journal => self.handle_journal_overlay_inputs(data),
            OverlayScreen::Alchemy => self.handle_alchemy_inputs(data, audio),
            OverlayScreen::Inventory => self.handle_inventory_overlay_inputs(data),
        }
        crate::ui_scale::set_overlay_mouse(false);

        true
    }

    fn handle_journal_overlay_inputs(&mut self, data: &GameData) {
        let journal_tab_count = self.journal_tabs().len();
        self.ui.journal_tab = self.ui.journal_tab.min(journal_tab_count.saturating_sub(1));
        if left_mouse_pressed() {
            let mouse = mouse_position_point();
            if rect_contains_point(journal_previous_rect(), mouse) {
                self.ui.journal_index = self.ui.journal_index.saturating_sub(1);
                return;
            }
            if rect_contains_point(journal_next_rect(), mouse) {
                self.ui.journal_index = self.ui.journal_index.saturating_add(1);
                return;
            }
            for index in 0..journal_tab_count {
                if rect_contains_point(journal_tab_rect(index, journal_tab_count), mouse) {
                    self.ui.journal_tab = index;
                    self.ui.journal_index = 0;
                    return;
                }
            }
            if let Some(index) = self.journal_row_at_point(data, mouse) {
                self.ui.journal_index = index;
                return;
            }
        }
        let previous_tab = self.ui.journal_tab;
        if switch_previous_pressed() {
            self.ui.journal_tab = self.ui.journal_tab.saturating_sub(1);
        }
        if switch_next_pressed() {
            self.ui.journal_tab =
                (self.ui.journal_tab + 1).min(journal_tab_count.saturating_sub(1));
        }
        if self.ui.journal_tab != previous_tab {
            self.ui.journal_index = 0;
        }
        // The routes tab holds far more herb memories than its column can show,
        // and the notes tab far more recorded beats, so both need a way to walk
        // them. Switching tabs resets the index, above, so the two lists do not
        // inherit each other's position.
        if select_previous_pressed() {
            self.ui.journal_index = self.ui.journal_index.saturating_sub(1);
        }
        if select_next_pressed() {
            self.ui.journal_index = self.ui.journal_index.saturating_add(1);
        }
        if journal_pressed() {
            self.clear_overlay();
            self.runtime.status_text = overlay_input_text::closed_journal();
        }
    }

    fn journal_row_at_point(&self, data: &GameData, point: [f32; 2]) -> Option<usize> {
        let panel = crate::journal_layout::journal_panel_rect();
        let row_offset = |top: f32, step: f32, count: usize, start: usize| {
            if point[1] < panel.y + top {
                return None;
            }
            let offset = ((point[1] - panel.y - top) / step) as usize;
            (offset < count).then_some(start + offset)
        };
        match self.ui.journal_tab {
            0 => {
                let route_start = gameplay_overlay_window::visible_window_start(
                    self.ui.journal_index,
                    data.gathering_routes.len(),
                    7,
                );
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(panel.x + 20.0, panel.y + 148.0, 380.0, 170.0),
                    point,
                ) {
                    return row_offset(
                        148.0,
                        22.0,
                        7.min(data.gathering_routes.len() - route_start),
                        route_start,
                    );
                }
                let herb_total = self.herb_memories(data).len();
                let herb_start = gameplay_overlay_window::visible_window_start(
                    self.ui.journal_index,
                    herb_total,
                    5,
                );
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(
                        panel.x + 420.0,
                        panel.y + 148.0,
                        panel.w - 440.0,
                        140.0,
                    ),
                    point,
                ) {
                    return row_offset(148.0, 22.0, 5.min(herb_total - herb_start), herb_start);
                }
                None
            }
            1 => {
                let total = self.progression.journal_milestones.len();
                let start =
                    gameplay_overlay_window::visible_window_start(self.ui.journal_index, total, 6);
                let note_x = panel.x + 20.0 + (panel.w - 40.0) * 0.44 + 24.0;
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(
                        note_x,
                        panel.y + 180.0,
                        panel.w - note_x - 20.0,
                        140.0,
                    ),
                    point,
                ) {
                    return row_offset(180.0, 22.0, 6.min(total - start), start);
                }
                None
            }
            2 => {
                let total = self.potion_memories(data).len();
                let (start, _) = gameplay_overlay_window::paged_window(
                    self.ui.journal_index.min(total.saturating_sub(1)),
                    total,
                    2,
                );
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(
                        panel.x + 24.0,
                        panel.y + 160.0,
                        panel.w - 48.0,
                        260.0,
                    ),
                    point,
                ) {
                    return row_offset(160.0, 126.0, 2.min(total - start), start);
                }
                None
            }
            3 if self.greenhouse_journal_unlocked() => {
                let total = self
                    .visible_stations(data)
                    .into_iter()
                    .filter(|station| station.kind == crate::data::StationKind::Planter)
                    .count();
                let start =
                    gameplay_overlay_window::visible_window_start(self.ui.journal_index, total, 6);
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(
                        panel.x + 24.0,
                        panel.y + 148.0,
                        panel.w - 48.0,
                        330.0,
                    ),
                    point,
                ) {
                    return row_offset(148.0, 52.0, 6.min(total - start), start);
                }
                None
            }
            _ => {
                let total = data.npcs.len();
                let start =
                    gameplay_overlay_window::visible_window_start(self.ui.journal_index, total, 3);
                if crate::input::rect_contains_point(
                    macroquad::prelude::Rect::new(
                        panel.x + 24.0,
                        panel.y + 148.0,
                        panel.w - 48.0,
                        360.0,
                    ),
                    point,
                ) {
                    return row_offset(148.0, 90.0, 3.min(total - start), start);
                }
                None
            }
        }
    }
}
