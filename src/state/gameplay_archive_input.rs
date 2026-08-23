use super::gameplay_overlay_types::ARCHIVE_TABS;
use super::gameplay_overlay_window::paged_window;
use super::GameplayState;
use crate::archive_layout::{archive_panel_rect, archive_tab_rect_at};
use crate::content::narrative_text;
use crate::data::GameData;
use crate::input::{
    archive_filter_pressed, cancel_pressed, confirm_pressed, left_mouse_pressed,
    mouse_position_point, rect_contains_point, select_next_pressed, select_previous_pressed,
    switch_next_pressed, switch_previous_pressed,
};

#[path = "gameplay_archive_input_text.rs"]
mod archive_input_text;

impl GameplayState {
    pub(super) fn handle_archive_inputs(&mut self, data: &GameData) {
        if left_mouse_pressed() {
            let point = mouse_position_point();
            let panel = archive_panel_rect();
            for index in 0..ARCHIVE_TABS.len() {
                if rect_contains_point(archive_tab_rect_at(panel.x, panel.y, index), point) {
                    self.ui.archive_tab = index;
                    self.ui.archive_index = 0;
                    return;
                }
            }
            if self.archive_tab_id() == "timeline"
                && rect_contains_point(crate::ui::overlay_primary_action_rect(), point)
            {
                self.handle_archive_timeline_submit(data);
                return;
            }
            if self.archive_tab_id() == "experiments"
                && rect_contains_point(crate::ui::archive_filter_rect(), point)
            {
                self.cycle_archive_experiment_filter();
                return;
            }
            let selection_len = self.archive_selection_len(data);
            if selection_len > 0 {
                let selected = self.archive_selected_index(selection_len);
                let (start, _) = paged_window(selected, selection_len, 6);
                for offset in 0..selection_len.saturating_sub(start).min(6) {
                    if !rect_contains_point(crate::ui::archive_list_entry_rect(offset), point) {
                        continue;
                    }
                    let index = start + offset;
                    if index == selected {
                        self.confirm_archive_selection(data);
                    } else {
                        self.ui.archive_index = index;
                    }
                    return;
                }
            }
        }
        if switch_previous_pressed() {
            self.ui.archive_tab = self.ui.archive_tab.saturating_sub(1);
            self.ui.archive_index = 0;
        }
        if switch_next_pressed() {
            self.ui.archive_tab =
                (self.ui.archive_tab + 1).min(ARCHIVE_TABS.len().saturating_sub(1));
            self.ui.archive_index = 0;
        }

        let selection_len = self.archive_selection_len(data);
        if selection_len > 0 {
            if select_previous_pressed() {
                self.ui.archive_index = self.ui.archive_index.saturating_sub(1);
            }
            if select_next_pressed() {
                self.ui.archive_index =
                    (self.ui.archive_index + 1).min(selection_len.saturating_sub(1));
            }
        } else {
            self.ui.archive_index = 0;
        }

        if ARCHIVE_TABS[self.ui.archive_tab] == "experiments" && archive_filter_pressed() {
            self.cycle_archive_experiment_filter();
        }

        if confirm_pressed() {
            self.confirm_archive_selection(data);
        }
        if cancel_pressed() {
            self.clear_overlay();
            self.runtime.status_text = archive_input_text::closed();
        }
    }

    fn confirm_archive_selection(&mut self, data: &GameData) {
        match ARCHIVE_TABS[self.ui.archive_tab] {
            "timeline" => self.handle_archive_timeline_submit(data),
            "disassembly" => {
                let recipes = self.available_disassembly_recipes(data);
                if let Some(recipe) = recipes.get(self.ui.archive_index).copied() {
                    self.disassemble_recipe(data, recipe);
                }
            }
            "duplication" => {
                let items = self.duplication_candidates(data);
                if let Some(item_id) = items.get(self.ui.archive_index).cloned() {
                    self.duplicate_item(data, &item_id);
                }
            }
            _ => {}
        }
    }

    fn handle_archive_timeline_submit(&mut self, data: &GameData) {
        if self.can_reconstruct_archive(data) {
            let milestone = &narrative_text().milestones.archive_revelation;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            self.trigger_archive_reconstruction_feedback(
                archive_input_text::timeline_restored_toast(),
            );
            self.runtime.status_text = archive_input_text::timeline_complete();
        } else {
            self.runtime.status_text = archive_input_text::timeline_incomplete();
        }
    }
}
