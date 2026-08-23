use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

pub(super) fn standard_overlay_panel_rect() -> Rect {
    Rect::new(180.0, 90.0, ui_w() - 360.0, ui_h() - 180.0)
}

/// Kept outside every panel so it remains a consistent, generous exit target
/// regardless of the overlay currently open.
pub(crate) fn overlay_close_rect() -> Rect {
    Rect::new(ui_w() - 136.0, 18.0, 116.0, 38.0)
}

pub(crate) fn overlay_primary_action_rect() -> Rect {
    Rect::new(ui_w() - 176.0, ui_h() - 60.0, 156.0, 36.0)
}

pub(crate) fn shop_panel_rect() -> Rect {
    Rect::new(160.0, 88.0, ui_w() - 320.0, ui_h() - 176.0)
}

pub(crate) fn shop_tab_rect(buying: bool) -> Rect {
    let panel = shop_panel_rect();
    Rect::new(
        panel.x + if buying { 20.0 } else { 140.0 },
        panel.y + 88.0,
        112.0,
        30.0,
    )
}

pub(crate) fn shop_entry_rect(index: usize, buying: bool) -> Rect {
    let panel = shop_panel_rect();
    let banner_offset = if buying { 0.0 } else { 38.0 };
    Rect::new(
        panel.x + 32.0,
        panel.y + 172.0 + banner_offset + index as f32 * 60.0,
        panel.w - 64.0,
        52.0,
    )
}

pub(crate) fn standard_overlay_entry_rect(index: usize, top: f32) -> Rect {
    let panel = standard_overlay_panel_rect();
    Rect::new(
        panel.x + 32.0,
        panel.y + top + index as f32 * 64.0,
        panel.w - 64.0,
        58.0,
    )
}

pub(crate) fn archive_list_entry_rect(index: usize) -> Rect {
    let panel = crate::archive_layout::archive_panel_rect();
    Rect::new(
        panel.x + 20.0,
        panel.y + 130.0 + index as f32 * 64.0,
        360.0,
        58.0,
    )
}

pub(crate) fn archive_filter_rect() -> Rect {
    let panel = crate::archive_layout::archive_panel_rect();
    Rect::new(panel.x + 206.0, panel.y + 96.0, 112.0, 32.0)
}
