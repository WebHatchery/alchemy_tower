pub(super) use super::hud_botanical_icons::{
    draw_flower, draw_leaf_cluster, draw_leaf_cluster_scaled,
};
use super::hud_palette::*;
use macroquad::prelude::*;

pub(super) fn draw_sun_icon(center: Vec2, radius: f32) {
    let color = Color::from_rgba(242, 173, 56, 255);
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0;
        let inner = center + vec2(angle.cos(), angle.sin()) * (radius + 4.0);
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius + 12.0);
        draw_line(inner.x, inner.y, outer.x, outer.y, 2.0, color);
    }
    draw_circle(center.x, center.y, radius, color);
    draw_circle(
        center.x - 4.0,
        center.y - 4.0,
        radius * 0.35,
        Color::from_rgba(255, 232, 143, 230),
    );
}

pub(super) fn draw_bottle_silhouette(rect: Rect, alpha: f32) {
    let color = Color::new(parchment().r, parchment().g, parchment().b, alpha);
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 17.0;
    draw_rectangle(cx - 5.0, top, 10.0, 13.0, color);
    draw_rectangle(cx - 12.0, top + 13.0, 24.0, 23.0, color);
    draw_circle(cx, top + 37.0, 12.0, color);
    draw_rectangle_lines(
        cx - 12.0,
        top + 13.0,
        24.0,
        24.0,
        1.0,
        Color::new(
            brass_light().r,
            brass_light().g,
            brass_light().b,
            alpha * 0.8,
        ),
    );
}
