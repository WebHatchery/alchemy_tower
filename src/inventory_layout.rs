use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

const PANEL_W: f32 = 980.0;
const PANEL_H: f32 = 590.0;
const PANEL_MARGIN: f32 = 24.0;

pub(crate) const INVENTORY_LIST_X: f32 = 20.0;
pub(crate) const INVENTORY_LIST_W: f32 = 360.0;
pub(crate) const INVENTORY_LIST_TOP: f32 = 122.0;
pub(crate) const INVENTORY_ROW_H: f32 = 52.0;
pub(crate) const INVENTORY_VISIBLE_ROWS: usize = 7;
pub(crate) const INVENTORY_DETAIL_X: f32 = 410.0;
pub(crate) const INVENTORY_DETAIL_TOP: f32 = 122.0;

pub(crate) fn inventory_panel_rect() -> Rect {
    let width = (ui_w() - PANEL_MARGIN * 2.0).clamp(PANEL_W, 1120.0);
    let height = (ui_h() - PANEL_MARGIN * 2.0).clamp(PANEL_H, 680.0);
    Rect::new(
        ((ui_w() - width) * 0.5).max(0.0),
        ((ui_h() - height) * 0.5).max(0.0),
        width,
        height,
    )
}

pub(crate) fn inventory_row_rect(index: usize) -> Rect {
    let panel = inventory_panel_rect();
    Rect::new(
        panel.x + INVENTORY_LIST_X,
        panel.y + INVENTORY_LIST_TOP + index as f32 * INVENTORY_ROW_H,
        INVENTORY_LIST_W,
        INVENTORY_ROW_H - 6.0,
    )
}

pub(crate) fn inventory_previous_rect() -> Rect {
    let panel = inventory_panel_rect();
    Rect::new(
        panel.x + panel.w - 468.0,
        panel.y + panel.h - 46.0,
        110.0,
        30.0,
    )
}

pub(crate) fn inventory_next_rect() -> Rect {
    let panel = inventory_panel_rect();
    Rect::new(
        panel.x + panel.w - 350.0,
        panel.y + panel.h - 46.0,
        110.0,
        30.0,
    )
}

pub(crate) fn inventory_use_rect() -> Rect {
    let panel = inventory_panel_rect();
    Rect::new(
        panel.x + panel.w - 206.0,
        panel.y + panel.h - 46.0,
        178.0,
        30.0,
    )
}

pub(crate) fn inventory_hud_rect() -> Rect {
    Rect::new(crate::ui_scale::ui_w() - 104.0, 214.0, 84.0, 78.0)
}
