use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use crate::alchemy_layout::{AL_BOX_BOTTOM_MARGIN, AL_FORM_BOX_Y, AL_FORM_TITLE_Y, AL_LW, AL_LX};
use crate::view_models::alchemy::AlchemyFormulaePanelView;
use macroquad::prelude::Color;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_alchemy_formulae_panel_view(
    view: &AlchemyFormulaePanelView,
    x: f32,
    y: f32,
    h: f32,
) {
    let box_top = y + AL_FORM_BOX_Y;
    let box_bottom = y + h - AL_BOX_BOTTOM_MARGIN;
    draw_overlay_section_title(x + AL_LX, y + AL_FORM_TITLE_Y, view.title, None);
    draw_overlay_section_box(x + AL_LX - 2.0, box_top, AL_LW, box_bottom - box_top);

    let text_x = x + AL_LX + 6.0;
    let avail_w = AL_LW - 24.0;

    if view.rows.is_empty() {
        draw_wrapped_text(
            &view.empty_text,
            text_x,
            box_top + 24.0,
            avail_w,
            18.0,
            18.0,
            dark::TEXT_DIM,
        );
        return;
    }

    let mut ky = box_top + 24.0;
    let mut shown = 0usize;
    for row in &view.rows {
        // Reserve room for a title line plus up to two detail lines; stop before
        // a recipe would spill past the box instead of overrunning it.
        if ky + 62.0 > box_bottom {
            break;
        }
        draw_ui_text(&row.title, text_x, ky, 18.0, dark::TEXT_BRIGHT);
        ky += 22.0;
        for line in wrap_lines(&row.detail, avail_w, 15, 2) {
            draw_ui_text(&line, text_x, ky, 15.0, dark::TEXT_DIM);
            ky += 18.0;
        }
        ky += 12.0;
        shown += 1;
    }

    if shown < view.rows.len() {
        draw_ui_text(
            &format!("+{} more (browse to see)", view.rows.len() - shown),
            text_x,
            (box_bottom - 8.0).min(ky),
            15.0,
            Color::from_rgba(186, 174, 145, 220),
        );
    }
}

/// Word-wrap `text` to at most `max_lines` lines that each fit `max_w`, adding
/// an ellipsis if content is dropped.
fn wrap_lines(text: &str, max_w: f32, font: u16, max_lines: usize) -> Vec<String> {
    let mut lines = macroquad_toolkit::ui::wrap_text(text, max_w, font as f32);
    let overflow = lines.len() > max_lines;
    lines.truncate(max_lines);
    if overflow {
        if let Some(last) = lines.last_mut() {
            let ellipsis_width = measure_ui_text("…", None, font, 1.0).width;
            let clipped = macroquad_toolkit::ui::truncate_text_to_width(
                last,
                (max_w - ellipsis_width).max(0.0),
                font as f32,
            );
            *last = format!("{}…", clipped.trim_end_matches('.'));
        }
    }
    lines
}
