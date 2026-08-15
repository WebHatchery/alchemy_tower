use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use super::truncate_text_to_width;
use crate::art::{draw_texture_centered, ArtAssets};
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_selection_card(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    selected: bool,
    enabled: bool,
    title: &str,
    subtitle: &str,
    meta: &str,
) {
    draw_selection_card_content(x, y, w, h, selected, enabled, title, subtitle, meta, 18.0);
}

pub(crate) fn draw_item_selection_card(
    art: &ArtAssets,
    item_id: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    selected: bool,
    enabled: bool,
    title: &str,
    subtitle: &str,
    meta: &str,
) {
    draw_selection_card_content(x, y, w, h, selected, enabled, title, subtitle, meta, 58.0);
    if let Some(texture) = art.item_icon(item_id) {
        let icon_size = (h - 12.0).clamp(24.0, 42.0);
        draw_texture_centered(
            texture,
            vec2(x + 29.0, y + h * 0.5),
            vec2(icon_size, icon_size),
            if enabled {
                WHITE
            } else {
                Color::new(0.7, 0.7, 0.7, 0.72)
            },
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_selection_card_content(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    selected: bool,
    enabled: bool,
    title: &str,
    subtitle: &str,
    meta: &str,
    text_inset: f32,
) {
    let bg = if selected {
        Color::from_rgba(28, 34, 44, 214)
    } else {
        Color::from_rgba(16, 18, 26, 154)
    };
    let text_color = if enabled {
        dark::TEXT_BRIGHT
    } else {
        dark::TEXT_DIM
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle(x, y, 4.0, h, accent_color(selected, enabled));
    draw_rectangle_lines(x, y, w, h, 1.5, outline_color(selected));
    draw_selection_card_text(x, y, w, title, subtitle, meta, text_color, text_inset);
}

fn draw_selection_card_text(
    x: f32,
    y: f32,
    w: f32,
    title: &str,
    subtitle: &str,
    meta: &str,
    text_color: Color,
    text_inset: f32,
) {
    let meta_width = if meta.is_empty() {
        0.0
    } else {
        measure_ui_text(meta, None, 16, 1.0)
            .width
            .min((w * 0.34).max(56.0))
    };
    let title_width = (w - text_inset - 12.0 - meta_width).max(80.0);
    draw_ui_text(
        &truncate_text_to_width(title, title_width, 20.0),
        x + text_inset,
        y + 22.0,
        20.0,
        text_color,
    );
    if !subtitle.is_empty() {
        draw_ui_text(
            &truncate_text_to_width(subtitle, w - 24.0, 16.0),
            x + text_inset,
            y + 42.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
    if !meta.is_empty() {
        let safe_meta = truncate_text_to_width(meta, (w * 0.34).max(56.0), 16.0);
        let measured = measure_ui_text(&safe_meta, None, 16, 1.0);
        draw_ui_text(
            &safe_meta,
            x + w - measured.width - 12.0,
            y + 21.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
}

fn accent_color(selected: bool, enabled: bool) -> Color {
    if selected {
        Color::from_rgba(176, 226, 255, 144)
    } else if enabled {
        Color::from_rgba(255, 214, 132, 92)
    } else {
        Color::from_rgba(102, 108, 120, 82)
    }
}

fn outline_color(selected: bool) -> Color {
    if selected {
        Color::from_rgba(176, 226, 255, 92)
    } else {
        Color::from_rgba(160, 170, 190, 54)
    }
}
