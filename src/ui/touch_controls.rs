//! Always-visible touch controls for exploration.
//!
//! Macroquad maps a single browser touch to the primary pointer, so the same
//! hit regions serve taps and mouse clicks. The controls deliberately remain in
//! real screen space: they stay usable while the HUD scales down on a phone.

use super::draw_action_button;
use macroquad::prelude::{screen_height, screen_width, Rect};

const MOVE_SIZE: f32 = 48.0;
const MOVE_GAP: f32 = 4.0;

pub(crate) fn touch_move_up_rect() -> Rect {
    let (x, y) = move_origin();
    Rect::new(x + MOVE_SIZE + MOVE_GAP, y, MOVE_SIZE, MOVE_SIZE)
}

pub(crate) fn touch_move_left_rect() -> Rect {
    let (x, y) = move_origin();
    Rect::new(x, y + MOVE_SIZE + MOVE_GAP, MOVE_SIZE, MOVE_SIZE)
}

pub(crate) fn touch_move_right_rect() -> Rect {
    let (x, y) = move_origin();
    Rect::new(
        x + (MOVE_SIZE + MOVE_GAP) * 2.0,
        y + MOVE_SIZE + MOVE_GAP,
        MOVE_SIZE,
        MOVE_SIZE,
    )
}

pub(crate) fn touch_move_down_rect() -> Rect {
    let (x, y) = move_origin();
    Rect::new(
        x + MOVE_SIZE + MOVE_GAP,
        y + (MOVE_SIZE + MOVE_GAP) * 2.0,
        MOVE_SIZE,
        MOVE_SIZE,
    )
}

pub(crate) fn touch_journal_rect() -> Rect {
    Rect::new(20.0, move_origin().1 - 42.0, 190.0, 34.0)
}

pub(crate) fn touch_pause_rect() -> Rect {
    Rect::new(screen_width() - 148.0, 18.0, 128.0, 40.0)
}

pub(crate) fn draw_touch_controls() {
    draw_action_button(touch_move_up_rect(), "UP", 16.0);
    draw_action_button(touch_move_left_rect(), "L", 18.0);
    draw_action_button(touch_move_right_rect(), "R", 18.0);
    draw_action_button(touch_move_down_rect(), "DN", 16.0);
    draw_action_button(touch_journal_rect(), "BAG & JOURNAL", 13.0);
    draw_action_button(touch_pause_rect(), "PAUSE", 16.0);
}

fn move_origin() -> (f32, f32) {
    (20.0, (screen_height() - 180.0).max(64.0))
}
