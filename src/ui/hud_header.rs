use super::hud_atmosphere::*;
use super::hud_primitives::*;
use super::truncate_text_to_width;
use super::HudView;
use macroquad::prelude::*;

pub(super) fn draw_hud_vignette() {
    let sw = screen_width();
    let sh = screen_height();
    draw_hud_atmosphere(sw, sh);

    for band in 0..6 {
        let t = band as f32 / 5.0;
        let alpha = (95.0 * (1.0 - t)) as u8;
        let inset = band as f32 * 12.0;
        draw_rectangle(0.0, inset, sw, 12.0, Color::from_rgba(0, 0, 0, alpha));
        draw_rectangle(
            0.0,
            sh - inset - 12.0,
            sw,
            12.0,
            Color::from_rgba(0, 0, 0, alpha),
        );
        draw_rectangle(
            inset,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
        draw_rectangle(
            sw - inset - 12.0,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
    }

    draw_edge_foliage(sw, sh);
}

/// Area names are useful when arriving, but they should not remain a permanent
/// header that competes with the room. The runtime supplies a short-lived alpha.
pub(super) fn draw_arrival_banner(view: &HudView) {
    if view.area_banner_alpha <= 0.0 {
        return;
    }
    let width = 330.0;
    let rect = Rect::new(super::hud_w() * 0.5 - width * 0.5, 18.0, width, 42.0);
    let alpha = view.area_banner_alpha;
    draw_beveled_rect(
        Rect::new(rect.x + 3.0, rect.y + 5.0, rect.w, rect.h),
        10.0,
        Color::new(0.0, 0.0, 0.0, 0.42 * alpha),
    );
    draw_beveled_rect(rect, 10.0, Color::new(0.20, 0.16, 0.11, 0.90 * alpha));
    draw_beveled_rect_lines(rect, 10.0, 1.2, Color::new(0.95, 0.78, 0.44, 0.78 * alpha));
    draw_centered_text_shadowed(
        &truncate_text_to_width(&view.area_label, width - 24.0, 20.0),
        rect.x,
        rect.y + 27.0,
        rect.w,
        20.0,
        Color::new(0.98, 0.92, 0.77, alpha),
    );
}
