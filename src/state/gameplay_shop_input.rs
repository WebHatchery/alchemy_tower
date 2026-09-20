use super::GameplayState;
use crate::data::{GameData, StationKind};
use crate::input::{
    confirm_pressed, left_mouse_pressed, mouse_position_point, rect_contains_point,
    select_next_pressed, select_previous_pressed, sort_pressed, switch_next_pressed,
    switch_previous_pressed,
};
use crate::ui::{shop_visible_rows, SHOP_BUY_VISIBLE_ROWS, SHOP_SELL_VISIBLE_ROWS};

impl GameplayState {
    pub(super) fn handle_shop_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            self.clear_overlay();
            return;
        };
        if station.kind != StationKind::Shop {
            self.clear_overlay();
            return;
        }

        if left_mouse_pressed() {
            let point = mouse_position_point();
            if rect_contains_point(crate::ui::shop_tab_rect(true), point) {
                self.ui.shop_buy_tab = true;
                self.ui.shop_index = 0;
                return;
            }
            if rect_contains_point(crate::ui::shop_tab_rect(false), point) {
                self.ui.shop_buy_tab = false;
                self.ui.shop_index = 0;
                return;
            }
            if rect_contains_point(crate::ui::shop_sort_rect(), point) {
                self.cycle_inventory_sort_mode();
                self.ui.shop_index = 0;
                return;
            }
            let total = self.shop_entry_count(data, station);
            let page_rows = shop_visible_rows(self.ui.shop_buy_tab);
            if total > page_rows && rect_contains_point(crate::ui::shop_previous_rect(), point) {
                let start = visible_shop_start(self.ui.shop_index, total, page_rows);
                self.ui.shop_index = start.saturating_sub(page_rows);
                return;
            }
            if total > page_rows && rect_contains_point(crate::ui::shop_next_rect(), point) {
                let start = visible_shop_start(self.ui.shop_index, total, page_rows);
                self.ui.shop_index = (start + page_rows).min(total.saturating_sub(1));
                return;
            }
            let count = if self.ui.shop_buy_tab {
                station.stock.len().min(SHOP_BUY_VISIBLE_ROWS)
            } else {
                self.sell_candidates(data).len().min(SHOP_SELL_VISIBLE_ROWS)
            };
            let start = visible_shop_start(
                self.ui.shop_index,
                self.shop_entry_count(data, station),
                page_rows,
            );
            for offset in 0..count.min(self.shop_entry_count(data, station).saturating_sub(start)) {
                if !rect_contains_point(
                    crate::ui::shop_entry_rect(offset, self.ui.shop_buy_tab),
                    point,
                ) {
                    continue;
                }
                let index = start + offset;
                if self.ui.shop_index == index {
                    self.confirm_shop_selection(data, station);
                } else {
                    self.ui.shop_index = index;
                }
                return;
            }
        }

        if switch_previous_pressed() || switch_next_pressed() {
            self.ui.shop_buy_tab = !self.ui.shop_buy_tab;
            self.ui.shop_index = 0;
        }
        if select_previous_pressed() {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if select_next_pressed() {
            let max_index = if self.ui.shop_buy_tab {
                station.stock.len().saturating_sub(1)
            } else {
                self.sell_candidates(data).len().saturating_sub(1)
            };
            self.ui.shop_index = (self.ui.shop_index + 1).min(max_index);
        }
        if sort_pressed() {
            self.cycle_inventory_sort_mode();
            self.ui.shop_index = 0;
        }
        if confirm_pressed() {
            self.confirm_shop_selection(data, station);
        }

        let max_index = if self.ui.shop_buy_tab {
            station.stock.len().saturating_sub(1)
        } else {
            self.sell_candidates(data).len().saturating_sub(1)
        };
        self.ui.shop_index = self.ui.shop_index.min(max_index);
    }

    fn shop_entry_count(&self, data: &GameData, station: &crate::data::StationDefinition) -> usize {
        if self.ui.shop_buy_tab {
            station.stock.len()
        } else {
            self.sell_candidates(data).len()
        }
    }

    fn confirm_shop_selection(
        &mut self,
        data: &GameData,
        station: &crate::data::StationDefinition,
    ) {
        if self.ui.shop_buy_tab {
            if let Some(stock) = station.stock.get(self.ui.shop_index) {
                self.buy_item(data, &stock.item_id, stock.price);
            }
        } else {
            let sellable = self.sell_candidates(data);
            if let Some(item_id) = sellable.get(self.ui.shop_index) {
                self.sell_item(data, item_id);
            }
        }
    }
}

fn visible_shop_start(selected: usize, total: usize, rows: usize) -> usize {
    super::gameplay_overlay_window::paged_window(selected, total, rows).0
}
