use super::gameplay_overlay_types::{JournalColumn, OverlayScreen};
use super::gameplay_overlay_window;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::input::{
    cancel_pressed, confirm_pressed, journal_pressed, left_mouse_pressed, mouse_position_point,
    rect_contains_point, select_next_pressed, select_previous_pressed, switch_next_pressed,
    switch_previous_pressed,
};
use crate::journal_layout::{
    journal_access_next_rect, journal_access_previous_rect, journal_next_rect,
    journal_previous_rect, journal_tab_rect,
};

#[path = "gameplay_overlay_input_text.rs"]
mod overlay_input_text;

#[derive(Clone, Copy)]
enum JournalRowSelection {
    Route(usize),
    Herb(usize),
    Shared(usize),
}

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
            if self.ui.journal_tab == 0
                && rect_contains_point(journal_access_previous_rect(), mouse)
            {
                self.ui.journal_access_page = self.ui.journal_access_page.saturating_sub(1);
                return;
            }
            if self.ui.journal_tab == 0 && rect_contains_point(journal_access_next_rect(), mouse) {
                let page_count = self.journal_route_access_warps(data).len().div_ceil(2);
                self.ui.journal_access_page = self
                    .ui
                    .journal_access_page
                    .saturating_add(1)
                    .min(page_count.saturating_sub(1));
                return;
            }
            if rect_contains_point(journal_previous_rect(), mouse) {
                self.move_journal_selection(false);
                return;
            }
            if rect_contains_point(journal_next_rect(), mouse) {
                self.move_journal_selection(true);
                return;
            }
            for index in 0..journal_tab_count {
                if rect_contains_point(journal_tab_rect(index, journal_tab_count), mouse) {
                    self.ui.journal_tab = index;
                    self.reset_journal_selection();
                    return;
                }
            }
            if let Some(selection) = self.journal_row_at_point(data, mouse) {
                match selection {
                    JournalRowSelection::Route(index) => {
                        self.ui.journal_route_index = index;
                        self.ui.journal_column = JournalColumn::Route;
                    }
                    JournalRowSelection::Herb(index) => {
                        self.ui.journal_herb_index = index;
                        self.ui.journal_column = JournalColumn::Herb;
                    }
                    JournalRowSelection::Shared(index) => self.ui.journal_index = index,
                }
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
            self.reset_journal_selection();
        }
        // The routes tab holds far more herb memories than its column can show,
        // and the notes tab far more recorded beats, so both need a way to walk
        // them. Switching tabs resets the index, above, so the two lists do not
        // inherit each other's position.
        if select_previous_pressed() {
            self.move_journal_selection(false);
        }
        if select_next_pressed() {
            self.move_journal_selection(true);
        }
        if journal_pressed() {
            self.clear_overlay();
            self.runtime.status_text = overlay_input_text::closed_journal();
        }
    }

    fn reset_journal_selection(&mut self) {
        self.ui.journal_index = 0;
        self.ui.journal_route_index = 0;
        self.ui.journal_herb_index = 0;
        self.ui.journal_column = JournalColumn::Route;
        self.ui.journal_access_page = 0;
    }

    fn move_journal_selection(&mut self, next: bool) {
        let index = match (self.ui.journal_tab, self.ui.journal_column) {
            (0, JournalColumn::Route) => &mut self.ui.journal_route_index,
            (0, JournalColumn::Herb) => &mut self.ui.journal_herb_index,
            _ => &mut self.ui.journal_index,
        };
        if next {
            *index = index.saturating_add(1);
        } else {
            *index = index.saturating_sub(1);
        }
    }

    fn journal_row_at_point(
        &self,
        data: &GameData,
        point: [f32; 2],
    ) -> Option<JournalRowSelection> {
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
                    self.ui.journal_route_index,
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
                        7.min(data.gathering_routes.len().saturating_sub(route_start)),
                        route_start,
                    )
                    .map(JournalRowSelection::Route);
                }
                let herb_total = self.herb_memories(data).len();
                let herb_start = gameplay_overlay_window::visible_window_start(
                    self.ui.journal_herb_index,
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
                    return row_offset(
                        148.0,
                        22.0,
                        5.min(herb_total.saturating_sub(herb_start)),
                        herb_start,
                    )
                    .map(JournalRowSelection::Herb);
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
                    return row_offset(180.0, 22.0, 6.min(total.saturating_sub(start)), start)
                        .map(JournalRowSelection::Shared);
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
                    return row_offset(160.0, 126.0, 2.min(total.saturating_sub(start)), start)
                        .map(JournalRowSelection::Shared);
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
                    return row_offset(148.0, 52.0, 6.min(total.saturating_sub(start)), start)
                        .map(JournalRowSelection::Shared);
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
                    return row_offset(148.0, 90.0, 3.min(total.saturating_sub(start)), start)
                        .map(JournalRowSelection::Shared);
                }
                None
            }
        }
    }
}
