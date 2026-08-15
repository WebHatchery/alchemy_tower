use super::draw_wrapped_text;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::view_models::journal::JournalBrewsTabView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_journal_brews_tab_view(
    view: &JournalBrewsTabView,
    art: &ArtAssets,
    x: f32,
    y: f32,
    _w: f32,
    h: f32,
) {
    draw_ui_text(view.title, x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
    if let Some(page_text) = &view.page_text {
        draw_ui_text(page_text, x + 240.0, y + 136.0, 16.0, dark::TEXT_DIM);
    }
    let mut brew_y = y + 168.0;
    if view.entries.is_empty() {
        draw_ui_text(&view.empty_text, x + 20.0, brew_y, 20.0, dark::TEXT_DIM);
        return;
    }
    for entry in &view.entries {
        let row_top = brew_y - 8.0;
        draw_rectangle(
            x + 12.0,
            row_top,
            1000.0,
            126.0,
            Color::from_rgba(17, 19, 27, 104),
        );
        draw_line(
            x + 12.0,
            row_top + 126.0,
            x + 1012.0,
            row_top + 126.0,
            1.0,
            Color::from_rgba(160, 170, 190, 42),
        );
        if let Some(texture) = art.item_icon(&entry.item_id) {
            draw_texture_centered(
                texture,
                vec2(x + 38.0, brew_y + 18.0),
                vec2(38.0, 38.0),
                WHITE,
            );
        }
        draw_ui_text(&entry.title, x + 64.0, brew_y, 20.0, dark::TEXT_BRIGHT);
        brew_y += 20.0;
        draw_ui_text(&entry.state_line, x + 64.0, brew_y, 17.0, dark::TEXT_DIM);
        brew_y += 18.0;
        draw_wrapped_text(
            &entry.recap,
            x + 64.0,
            brew_y,
            500.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        if let Some(effects_text) = &entry.effects_text {
            draw_ui_text(
                effects_text,
                x + 590.0,
                row_top + 26.0,
                18.0,
                dark::TEXT_DIM,
            );
            if let Some(traits_text) = &entry.traits_text {
                draw_ui_text(traits_text, x + 590.0, row_top + 48.0, 18.0, dark::TEXT_DIM);
            }
        }
        let mut detail_y = row_top + 70.0;
        if let Some(best_brew_text) = &entry.best_brew_text {
            draw_ui_text(best_brew_text, x + 590.0, detail_y, 18.0, dark::TEXT_DIM);
            detail_y += 22.0;
        }
        if let Some(formula_text) = &entry.formula_text {
            draw_ui_text(formula_text, x + 590.0, detail_y, 18.0, dark::TEXT_DIM);
            detail_y += 22.0;
        }
        if let Some(successful_brews_text) = &entry.successful_brews_text {
            draw_ui_text(
                successful_brews_text,
                x + 590.0,
                detail_y,
                18.0,
                dark::TEXT_DIM,
            );
        }
        brew_y = row_top + 134.0;
        if brew_y > y + h - 40.0 {
            break;
        }
    }
}
