use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

pub(crate) fn archive_panel_rect() -> Rect {
    Rect::new(150.0, 70.0, ui_w() - 300.0, ui_h() - 140.0)
}

pub(crate) fn archive_tab_rect_at(x: f32, y: f32, index: usize) -> Rect {
    Rect::new(x + 20.0 + index as f32 * 148.0, y + 54.0, 136.0, 30.0)
}

pub(crate) fn archive_previous_rect() -> Rect {
    let panel = archive_panel_rect();
    Rect::new(
        panel.x + panel.w - 266.0,
        panel.y + panel.h - 46.0,
        116.0,
        32.0,
    )
}

pub(crate) fn archive_next_rect() -> Rect {
    let previous = archive_previous_rect();
    Rect::new(previous.x + previous.w + 8.0, previous.y, 116.0, previous.h)
}
