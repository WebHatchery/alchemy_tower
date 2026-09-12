use super::{
    draw_journal_row, draw_journal_section_box, draw_journal_section_title, draw_wrapped_text,
};
use crate::view_models::journal::JournalRapportTabView;
use macroquad::prelude::Rect;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_journal_rapport_tab_view(
    view: &JournalRapportTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_journal_section_title(x + 20.0, y + 136.0, view.title, view.page_text.as_deref());
    draw_journal_section_box(x + 18.0, y + 152.0, w - 36.0, (h - 208.0).max(180.0));
    let mut rapport_y = y + 168.0;
    for row in &view.rows {
        draw_journal_row(
            Rect::new(x + 24.0, rapport_y - 20.0, w - 48.0, 112.0),
            row.selected,
        );
        draw_ui_text(&row.title, x + 36.0, rapport_y, 20.0, dark::TEXT_BRIGHT);
        rapport_y += 20.0;
        draw_ui_text(&row.now_text, x + 36.0, rapport_y, 17.0, dark::TEXT_DIM);
        rapport_y += 18.0;
        draw_ui_text(&row.later_text, x + 36.0, rapport_y, 17.0, dark::TEXT_DIM);
        rapport_y += 18.0;
        draw_wrapped_text(
            &row.usually_text,
            x + 36.0,
            rapport_y,
            w - 56.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        rapport_y += 34.0;
        if rapport_y > y + h - 56.0 {
            break;
        }
    }
}
