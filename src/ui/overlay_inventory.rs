use super::{
    draw_action_button, draw_overlay_footer, draw_overlay_section_box, draw_overlay_section_title,
    draw_overlay_subtitle, draw_panel, draw_wrapped_text,
};
use crate::art::{draw_texture_centered, ArtAssets};
use crate::inventory_layout::{
    inventory_next_rect, inventory_panel_rect, inventory_previous_rect, inventory_use_rect,
    INVENTORY_DETAIL_TOP, INVENTORY_DETAIL_X, INVENTORY_LIST_TOP,
};
use crate::view_models::inventory::InventoryOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_inventory_overlay_view(view: &InventoryOverlayView, art: &ArtAssets) {
    let panel = inventory_panel_rect();
    draw_panel(panel.x, panel.y, panel.w, panel.h, view.title);
    draw_overlay_subtitle(panel.x, panel.y, view.subtitle);

    draw_overlay_section_title(panel.x + 20.0, panel.y + 104.0, view.held_title, None);
    draw_overlay_section_box(
        panel.x + 18.0,
        panel.y + INVENTORY_LIST_TOP - 8.0,
        364.0,
        402.0,
    );
    if let Some(page_text) = &view.page_text {
        draw_ui_text(
            page_text,
            panel.x + 202.0,
            panel.y + 104.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
    draw_ui_text(
        &view.sort_text,
        panel.x + 254.0,
        panel.y + 104.0,
        15.0,
        dark::TEXT_DIM,
    );

    if view.items.is_empty() {
        draw_ui_text(
            &view.empty_text,
            panel.x + 38.0,
            panel.y + INVENTORY_LIST_TOP + 32.0,
            18.0,
            dark::TEXT_DIM,
        );
    } else {
        for (index, item) in view.items.iter().enumerate() {
            let row = crate::inventory_layout::inventory_row_rect(index);
            draw_inventory_row(row, item, art);
        }
    }

    let detail_w = panel.w - INVENTORY_DETAIL_X - 20.0;
    draw_overlay_section_title(
        panel.x + INVENTORY_DETAIL_X,
        panel.y + 104.0,
        view.detail_title,
        None,
    );
    draw_overlay_section_box(
        panel.x + INVENTORY_DETAIL_X - 2.0,
        panel.y + INVENTORY_DETAIL_TOP - 8.0,
        detail_w,
        402.0,
    );
    if let Some(detail) = &view.detail {
        draw_inventory_detail(
            detail,
            art,
            panel.x + INVENTORY_DETAIL_X + 18.0,
            panel.y + INVENTORY_DETAIL_TOP + 24.0,
            detail_w - 36.0,
            view.uses_title,
        );
    } else {
        draw_ui_text(
            &view.empty_text,
            panel.x + INVENTORY_DETAIL_X + 18.0,
            panel.y + INVENTORY_DETAIL_TOP + 32.0,
            18.0,
            dark::TEXT_DIM,
        );
    }

    draw_overlay_footer(panel.x, panel.y, panel.w, panel.h, view.footer_text);
    draw_action_button(inventory_previous_rect(), view.previous_label, 0.0);
    draw_action_button(inventory_next_rect(), view.next_label, 0.0);
    if view.detail.as_ref().is_some_and(|detail| detail.can_use) {
        draw_action_button(inventory_use_rect(), view.use_label, 0.0);
    }
}

fn draw_inventory_row(
    rect: Rect,
    item: &crate::view_models::inventory::InventoryItemView,
    art: &ArtAssets,
) {
    let fill = if item.selected {
        Color::from_rgba(42, 54, 68, 238)
    } else {
        Color::from_rgba(17, 19, 27, 190)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle(
        rect.x,
        rect.y,
        5.0,
        rect.h,
        if item.selected {
            Color::from_rgba(242, 205, 126, 230)
        } else {
            Color::from_rgba(95, 118, 132, 150)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::from_rgba(160, 170, 190, if item.selected { 100 } else { 45 }),
    );
    if let Some(texture) = art.item_icon(&item.item_id) {
        draw_texture_centered(
            texture,
            vec2(rect.x + 28.0, rect.y + rect.h * 0.5),
            vec2(30.0, 30.0),
            WHITE,
        );
    }
    draw_ui_text(
        &item.title,
        rect.x + 52.0,
        rect.y + 21.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
    draw_ui_text(
        &item.meta,
        rect.x + 52.0,
        rect.y + 40.0,
        14.0,
        dark::TEXT_DIM,
    );
}

fn draw_inventory_detail(
    detail: &crate::view_models::inventory::InventoryDetailView,
    art: &ArtAssets,
    x: f32,
    y: f32,
    w: f32,
    uses_title: &str,
) {
    if let Some(texture) = art.item_icon(&detail.item_id) {
        draw_texture_centered(texture, vec2(x + 28.0, y + 24.0), vec2(48.0, 48.0), WHITE);
    }
    draw_ui_text(&detail.title, x + 64.0, y + 22.0, 24.0, dark::TEXT_BRIGHT);
    draw_ui_text(&detail.quantity_text, x + 64.0, y + 46.0, 18.0, dark::TEXT);
    draw_ui_text(&detail.category_text, x, y + 82.0, 16.0, dark::TEXT_DIM);
    draw_wrapped_text(&detail.description, x, y + 112.0, w, 17.0, 20.0, dark::TEXT);
    draw_ui_text(uses_title, x, y + 214.0, 20.0, dark::TEXT_BRIGHT);
    draw_wrapped_text(
        &detail.uses_text,
        x,
        y + 240.0,
        w,
        17.0,
        20.0,
        dark::TEXT_DIM,
    );
    draw_ui_text(&detail.action_text, x, y + 314.0, 18.0, dark::ACCENT);
}
