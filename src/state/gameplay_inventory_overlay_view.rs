use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, ItemCategory};
use crate::inventory_layout::INVENTORY_VISIBLE_ROWS;
use crate::view_models::inventory::{InventoryDetailView, InventoryItemView, InventoryOverlayView};

impl GameplayState {
    pub(super) fn inventory_overlay_view(&self, data: &GameData) -> InventoryOverlayView {
        let item_ids = self.sorted_inventory_items(data);
        let selected = self
            .ui
            .inventory_index
            .min(item_ids.len().saturating_sub(1));
        let start = visible_window_start(selected, item_ids.len(), INVENTORY_VISIBLE_ROWS);
        let rows = item_ids
            .iter()
            .enumerate()
            .skip(start)
            .take(INVENTORY_VISIBLE_ROWS)
            .map(|(index, item_id)| InventoryItemView {
                item_id: item_id.clone(),
                title: data.item_name(item_id).to_owned(),
                meta: self.inventory_row_meta(data, item_id),
                selected: index == selected,
            })
            .collect();

        InventoryOverlayView {
            title: ui_copy("overlay_inventory_title"),
            subtitle: ui_copy("overlay_inventory_subtitle"),
            held_title: ui_copy("overlay_inventory_held_title"),
            detail_title: ui_copy("overlay_inventory_detail_title"),
            uses_title: ui_copy("overlay_inventory_uses_title"),
            sort_text: ui_format(
                "overlay_sort_mode",
                &[("mode", self.inventory_sort_label())],
            ),
            empty_text: ui_copy("overlay_inventory_empty").to_owned(),
            page_text: (item_ids.len() > INVENTORY_VISIBLE_ROWS).then(|| {
                ui_format(
                    "inventory_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        (
                            "last",
                            &(start + INVENTORY_VISIBLE_ROWS)
                                .min(item_ids.len())
                                .to_string(),
                        ),
                        ("total", &item_ids.len().to_string()),
                    ],
                )
            }),
            items: rows,
            detail: item_ids
                .get(selected)
                .and_then(|item_id| self.inventory_detail_view(data, item_id)),
            footer_text: ui_copy("overlay_inventory_footer"),
            use_label: ui_copy("overlay_inventory_use_button"),
            previous_label: ui_copy("overlay_inventory_previous"),
            next_label: ui_copy("overlay_inventory_next"),
        }
    }

    fn inventory_row_meta(&self, data: &GameData, item_id: &str) -> String {
        let amount = self.inventory.get(item_id).copied().unwrap_or_default();
        let mut meta = ui_format("inventory_held", &[("amount", &amount.to_string())]);
        let reserved = self.reserved_count(item_id);
        if reserved > 0 {
            meta.push_str(" · ");
            meta.push_str(&ui_format(
                "inventory_row_reserved",
                &[("count", &reserved.to_string())],
            ));
        } else if self.active_quest_reference_count(data, item_id) > 0 {
            meta.push_str(" · ");
            meta.push_str(ui_copy("inventory_row_quest"));
        } else if self.known_recipe_reference_count(data, item_id) > 0 {
            meta.push_str(" · ");
            meta.push_str(ui_copy("inventory_row_recipe"));
        }
        meta
    }

    fn inventory_detail_view(&self, data: &GameData, item_id: &str) -> Option<InventoryDetailView> {
        let item = data.item(item_id)?;
        let amount = self.inventory.get(item_id).copied().unwrap_or_default();
        let references = self.inventory_reference_summary(data, item_id);
        let uses_text = if references.is_empty() {
            ui_copy("inventory_no_known_uses").to_owned()
        } else {
            ui_format("inventory_uses", &[("uses", &references)])
        };
        let can_use = item.category == ItemCategory::Potion;
        let action_text = if can_use {
            ui_copy("inventory_action_potion").to_owned()
        } else if self.reserved_count(item_id) > 0 {
            ui_copy("inventory_action_reserved").to_owned()
        } else {
            ui_copy("inventory_action_hold").to_owned()
        };

        Some(InventoryDetailView {
            item_id: item_id.to_owned(),
            title: item.name.clone(),
            quantity_text: ui_format("inventory_quantity", &[("amount", &amount.to_string())]),
            category_text: self.item_facts_text(data, item_id, amount),
            description: item.description.clone(),
            uses_text,
            action_text,
            can_use,
        })
    }
}

#[cfg(test)]
#[path = "gameplay_inventory_overlay_view/tests.rs"]
mod tests;
