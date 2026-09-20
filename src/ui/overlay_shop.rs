use super::{
    draw_action_button, draw_item_selection_card, draw_overlay_backdrop, draw_overlay_footer,
    draw_overlay_section_box, draw_overlay_section_title, draw_overlay_subtitle, draw_overlay_tab,
    draw_panel, draw_state_banner,
};
use crate::art::ArtAssets;
use crate::view_models::shop::ShopOverlayView;
use macroquad::prelude::*;

pub(crate) fn draw_shop_overlay_view(view: &ShopOverlayView, art: &ArtAssets) {
    draw_overlay_backdrop();
    let panel = crate::ui::shop_panel_rect();
    let x = panel.x;
    let y = panel.y;
    let w = panel.w;
    let h = panel.h;
    draw_panel(x, y, w, h, &view.station_name);
    draw_overlay_subtitle(x, y, &view.subtitle);
    draw_overlay_tab(
        Rect::new(x + 20.0, y + 88.0, 112.0, 30.0),
        &view.buy_tab_label,
        view.buy_tab_active,
    );
    draw_overlay_tab(
        Rect::new(x + 140.0, y + 88.0, 112.0, 30.0),
        &view.sell_tab_label,
        view.sell_tab_active,
    );
    draw_overlay_section_title(
        x + 20.0,
        y + 148.0,
        &view.stock_title,
        view.range_text.as_deref(),
    );
    draw_action_button(crate::ui::shop_sort_rect(), &view.sort_text, 0.0);
    draw_overlay_section_box(x + 20.0, y + 162.0, w - 40.0, h - 224.0);

    let mut row_y = y + 196.0;
    if let Some(safe_banner) = &view.safe_sell_banner {
        draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, safe_banner, false);
        row_y += 38.0;
    }
    if view.entries.is_empty() {
        draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &view.empty_text, false);
    } else {
        for (index, entry) in view.entries.iter().enumerate() {
            draw_item_selection_card(
                art,
                &entry.item_id,
                crate::ui::shop_entry_rect(index, view.buy_tab_active).x,
                crate::ui::shop_entry_rect(index, view.buy_tab_active).y,
                crate::ui::shop_entry_rect(index, view.buy_tab_active).w,
                52.0,
                entry.selected,
                entry.enabled,
                &entry.title,
                &entry.detail,
                &entry.meta,
            );
        }
    }
    draw_overlay_footer(x, y, w, h, &view.footer_text);
    if view.range_text.is_some() {
        draw_action_button(crate::ui::shop_previous_rect(), &view.previous_label, 0.0);
        draw_action_button(crate::ui::shop_next_rect(), &view.next_label, 0.0);
    }
}
