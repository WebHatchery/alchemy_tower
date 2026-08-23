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
