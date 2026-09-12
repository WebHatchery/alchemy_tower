use super::{draw_journal_row, draw_journal_section_box, draw_journal_section_title};
use crate::view_models::journal::JournalGreenhouseTabView;
use macroquad::prelude::Rect;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_journal_greenhouse_tab_view(
    view: &JournalGreenhouseTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_journal_section_title(x + 20.0, y + 136.0, view.title, view.page_text.as_deref());
    draw_journal_section_box(x + 18.0, y + 152.0, w - 36.0, (h - 208.0).max(180.0));
    let mut bed_y = y + 168.0;
    for bed in &view.beds {
        draw_journal_row(
            Rect::new(x + 24.0, bed_y - 20.0, w - 48.0, 46.0),
            bed.selected,
        );
        draw_ui_text(&bed.title, x + 36.0, bed_y, 22.0, dark::TEXT_BRIGHT);
        bed_y += 22.0;
        draw_ui_text(&bed.summary, x + 36.0, bed_y, 18.0, dark::TEXT_DIM);
        bed_y += 30.0;
        if bed_y > y + h - 56.0 {
            break;
        }
    }
    if view.beds.is_empty() {
        draw_ui_text(&view.empty_text, x + 20.0, bed_y, 20.0, dark::TEXT_DIM);
    }
}
