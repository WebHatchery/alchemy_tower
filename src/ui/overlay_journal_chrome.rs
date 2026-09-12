use crate::art::{draw_texture_centered, ArtAssets};
use crate::journal_layout::{journal_next_rect, journal_previous_rect};
use macroquad::prelude::*;

pub(crate) fn draw_journal_tabs(
    tabs: &[&str],
    selected_index: usize,
    tab_rects: &[Rect],
    art: &ArtAssets,
) {
    for (index, tab) in tabs.iter().enumerate() {
        let Some(rect) = tab_rects.get(index) else {
            break;
        };
        super::draw_overlay_tab(*rect, tab, selected_index == index);
        if let Some(texture) = art.journal_tab_by_label(tab) {
            draw_texture_centered(
                texture,
                vec2(rect.x + 16.0, rect.y + rect.h * 0.5),
                vec2(18.0, 18.0),
                WHITE,
            );
        }
    }
}

pub(crate) fn draw_journal_footer(footer_text: &str, x: f32, y: f32, w: f32, h: f32) {
    super::draw_overlay_footer(x, y, w, h, footer_text);
    super::draw_action_button(journal_previous_rect(), "PREVIOUS", 13.0);
    super::draw_action_button(journal_next_rect(), "NEXT", 15.0);
}

pub(crate) fn draw_journal_section_title(x: f32, y: f32, title: &str, meta: Option<&str>) {
    super::draw_overlay_section_title(x, y, title, meta);
}

pub(crate) fn draw_journal_section_box(x: f32, y: f32, w: f32, h: f32) {
    super::draw_overlay_section_box(x, y, w, h);
}

pub(crate) fn draw_journal_row(rect: Rect, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::from_rgba(42, 54, 68, 224)
        } else {
            Color::from_rgba(17, 19, 27, 154)
        },
    );
    draw_rectangle(
        rect.x,
        rect.y,
        4.0,
        rect.h,
        if selected {
            Color::from_rgba(242, 205, 126, 225)
        } else {
            Color::from_rgba(95, 118, 132, 135)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::from_rgba(160, 170, 190, if selected { 92 } else { 40 }),
    );
}

pub(crate) fn draw_journal_detail_box(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(25, 28, 38, 218),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::from_rgba(160, 170, 190, 72),
    );
}
