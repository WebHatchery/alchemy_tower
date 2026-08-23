use super::GameplayState;
use crate::data::{GameData, StationKind};
use crate::input::{
    confirm_pressed, left_mouse_pressed, mouse_position_point, rect_contains_point,
    select_next_pressed, select_previous_pressed, sort_pressed, switch_next_pressed,
    switch_previous_pressed,
};

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
            let count = if self.ui.shop_buy_tab {
                station.stock.len()
            } else {
                self.sell_candidates(data).len()
            };
            for index in 0..count {
                if !rect_contains_point(
                    crate::ui::shop_entry_rect(index, self.ui.shop_buy_tab),
                    point,
                ) {
                    continue;
                }
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
