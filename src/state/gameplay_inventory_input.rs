use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::data::{GameData, ItemCategory};
use crate::input::{
    left_mouse_pressed, mouse_position_point, rect_contains_point, select_next_pressed,
    select_previous_pressed, sort_pressed,
};
use crate::inventory_layout::{
    inventory_next_rect, inventory_previous_rect, inventory_row_rect, inventory_use_rect,
    INVENTORY_VISIBLE_ROWS,
};

impl GameplayState {
    pub(super) fn handle_inventory_overlay_inputs(&mut self, data: &GameData) {
        let items = self.sorted_inventory_items(data);
        self.ui.inventory_index = self.ui.inventory_index.min(items.len().saturating_sub(1));

        if left_mouse_pressed() {
            let mouse = mouse_position_point();
            if rect_contains_point(inventory_previous_rect(), mouse) {
                self.ui.inventory_index = self.ui.inventory_index.saturating_sub(1);
                return;
            }
            if rect_contains_point(inventory_next_rect(), mouse) {
                self.ui.inventory_index =
                    (self.ui.inventory_index + 1).min(items.len().saturating_sub(1));
                return;
            }
            if items.is_empty() {
                return;
            }
            let start =
                visible_window_start(self.ui.inventory_index, items.len(), INVENTORY_VISIBLE_ROWS);
            for offset in 0..INVENTORY_VISIBLE_ROWS.min(items.len() - start) {
                if rect_contains_point(inventory_row_rect(offset), mouse) {
                    self.ui.inventory_index = start + offset;
                    return;
                }
            }
            if rect_contains_point(inventory_use_rect(), mouse) {
                self.use_selected_inventory_potion(data, &items);
                return;
            }
        }

        if select_previous_pressed() {
            self.ui.inventory_index = self.ui.inventory_index.saturating_sub(1);
        }
        if select_next_pressed() {
            self.ui.inventory_index =
                (self.ui.inventory_index + 1).min(items.len().saturating_sub(1));
        }
        if sort_pressed() {
            self.cycle_inventory_sort_mode();
            self.ui.inventory_index = 0;
        }
    }

    fn use_selected_inventory_potion(&mut self, data: &GameData, items: &[String]) {
        let Some(item_id) = items.get(self.ui.inventory_index) else {
            return;
        };
        if data
            .item(item_id)
            .is_some_and(|item| item.category == ItemCategory::Potion)
        {
            self.consume_potion(data, item_id);
        }
    }
}
