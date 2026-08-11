use super::text::wrapped_lines;
use super::{draw_panel, draw_wrapped_text};
use crate::view_models::dialogue::DialogueOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

/// Gap between the panel's top edge and the first line of speech.
pub(crate) const DIALOGUE_TEXT_TOP: f32 = 58.0;
pub(crate) const DIALOGUE_LINE_HEIGHT: f32 = 26.0;
/// Room below the last line for the divider and the controls prompt.
pub(crate) const DIALOGUE_FOOTER_SPACE: f32 = 44.0;

pub(crate) fn draw_dialogue_overlay_view(view: &DialogueOverlayView) {
    let sw = crate::ui_scale::ui_w();
    let sh = crate::ui_scale::ui_h();
    draw_rectangle(0.0, 0.0, sw, sh, Color::from_rgba(0, 0, 0, 130));

    // Bottom-anchored speech panel, centered and clamped so it never inverts its
    // width on a narrow window or slides off the top of a short one.
    let w = (sw - 360.0).clamp(440.0, 980.0).min(sw - 32.0);

    // Height follows the words. This was a fixed 216 — four lines — while the
    // body is a townsperson's beat *plus* their earned reaction appended, which
    // runs past 600 characters late in the game. Every conversation in the last
    // third of the story was quietly running its final sentences through the
    // footer and off the panel. Sizing to the wrapped line count fixes the whole
    // family at once instead of trimming prose until it happens to fit.
    let line_count = wrapped_lines(&view.body, w - 44.0, 20.0).len().max(1);
    let h = (DIALOGUE_TEXT_TOP + line_count as f32 * DIALOGUE_LINE_HEIGHT + DIALOGUE_FOOTER_SPACE)
        .clamp(216.0, (sh - 56.0).max(216.0));
    let x = ((sw - w) * 0.5).max(0.0);
    let y = (sh - h - 40.0).max(16.0);
    draw_panel(x, y, w, h, &view.title);

    // Just the character's words — the old Now/Later/Usually schedule readout
    // lived here and made every conversation read like a debug tracker. That
    // routine info now lives in the Journal's rapport tab where it belongs.
    draw_wrapped_text(
        &view.body,
        x + 22.0,
        y + 58.0,
        w - 44.0,
        20.0,
        26.0,
        dark::TEXT_BRIGHT,
    );

    // A dim divider plus a highlighted prompt makes the exit/continue controls
    // an obvious, always-visible affordance rather than an easy-to-miss line.
    let footer_y = y + h - 34.0;
    draw_line(
        x + 20.0,
        footer_y - 6.0,
        x + w - 20.0,
        footer_y - 6.0,
        1.0,
        Color::from_rgba(223, 184, 111, 70),
    );
    draw_ui_text(
        &view.footer,
        x + 20.0,
        footer_y + 14.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}

#[cfg(test)]
mod tests;
