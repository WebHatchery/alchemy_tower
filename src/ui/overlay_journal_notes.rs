use super::{
    draw_journal_detail_box, draw_journal_row, draw_journal_section_title, draw_wrapped_text,
};
use crate::content::ui_copy;
use crate::view_models::journal::JournalNotesTabView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

/// The notes tab's geometry. Exported because the guard that keeps the longest
/// recorded beat inside the panel has to do the same arithmetic, and a test
/// carrying its own copy of the numbers stops being a guard the first time one
/// of them changes.
pub(crate) const NOTE_ROW_STEP: f32 = 22.0;
pub(crate) const NOTE_DETAIL_FONT: f32 = 18.0;
pub(crate) const NOTE_DETAIL_LINE_HEIGHT: f32 = 20.0;
/// Both columns start here, level with the Active Work box. The record used to
/// begin below the milestone rows, which left it about eighty pixels of a panel
/// whose beats run to seven lines.
pub(crate) const COLUMN_TOP: f32 = 168.0;
/// A column heading to its first row.
pub(crate) const COLUMN_HEADING_STEP: f32 = 32.0;
/// The gap between the last note title and the selected beat's paragraph.
pub(crate) const NOTE_DETAIL_GAP: f32 = 14.0;
/// The panel's own bottom margin.
pub(crate) const NOTES_BOTTOM_MARGIN: f32 = 40.0;
/// How much of the panel's inner width the left column takes. The rest, less a
/// gutter, belongs to the record.
pub(crate) const LEFT_COLUMN_FRACTION: f32 = 0.44;
pub(crate) const COLUMN_GUTTER: f32 = 24.0;

/// Where the record column starts and how wide it is, given the panel.
pub(crate) fn note_column(x: f32, w: f32) -> (f32, f32) {
    let inner = w - 40.0;
    let left = inner * LEFT_COLUMN_FRACTION;
    (
        x + 20.0 + left + COLUMN_GUTTER,
        inner - left - COLUMN_GUTTER,
    )
}

/// Where the selected beat's paragraph starts, given the panel and how many
/// titles are listed above it.
pub(crate) fn note_detail_top(y: f32, rows: usize) -> f32 {
    y + COLUMN_TOP + COLUMN_HEADING_STEP + rows as f32 * NOTE_ROW_STEP + NOTE_DETAIL_GAP
}

pub(crate) fn draw_journal_notes_tab_view(
    view: &JournalNotesTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_journal_section_title(x + 20.0, y + 136.0, view.title, None);

    // Two columns. Everything used to run the panel's full width: the milestone
    // rows ended at y+480 and the recorded notes began at y+448, so the last
    // milestone's detail was overprinted by the first note's title on every
    // full record, in the shipped game.
    let left_width = (w - 40.0) * LEFT_COLUMN_FRACTION;
    draw_journal_section_title(x + 20.0, y + COLUMN_TOP, view.active_title, None);
    draw_journal_detail_box(Rect::new(x + 20.0, y + 182.0, left_width, 96.0));
    draw_wrapped_text(
        &view.active_summary,
        x + 34.0,
        y + 206.0,
        left_width - 28.0,
        18.0,
        20.0,
        dark::TEXT_DIM,
    );

    draw_journal_section_title(x + 20.0, y + 300.0, view.milestones_title, None);
    let mut milestone_y = y + 332.0;
    for row in &view.milestone_rows {
        draw_journal_row(
            Rect::new(x + 20.0, milestone_y - 20.0, left_width, 26.0),
            row.title.contains(ui_copy("overlay_progress_ready")),
        );
        draw_ui_text(&row.title, x + 32.0, milestone_y, 20.0, dark::TEXT_BRIGHT);
        milestone_y += 20.0;
        draw_wrapped_text(
            &row.detail,
            x + 20.0,
            milestone_y,
            left_width,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        milestone_y += 46.0;
    }

    // Titles listed, the selected beat written out beneath them — the same
    // shape the routes tab and the archive console use. This drew the last five
    // beats one after another, advancing a fixed 74px per entry while the text
    // was laid out to its real wrapped height, so the entries overlapped; and
    // everything older than the fifth could not be reached at all.
    let (note_x, note_width) = note_column(x, w);
    let mut note_y = y + COLUMN_TOP;
    draw_journal_section_title(
        note_x,
        note_y,
        view.notes_title,
        view.note_range_text.as_deref(),
    );
    note_y += COLUMN_HEADING_STEP;
    for row in &view.note_rows {
        let colour = if row.selected {
            dark::TEXT_BRIGHT
        } else {
            dark::TEXT_DIM
        };
        draw_journal_row(
            Rect::new(note_x, note_y - 20.0, note_width, 26.0),
            row.selected,
        );
        draw_ui_text(&row.title, note_x + 12.0, note_y, 20.0, colour);
        note_y += NOTE_ROW_STEP;
    }

    let Some(detail) = &view.note_detail else {
        return;
    };
    // One definition of where the paragraph starts, shared with the guard that
    // checks the longest beat still fits.
    let note_y = note_detail_top(y, view.note_rows.len());
    // Bounds-checked on the block's *measured* height, not on where it starts.
    // Checking the start is how the herb entry came to be running its last line
    // through the panel below it.
    let lines = super::text::wrapped_lines(detail, note_width, NOTE_DETAIL_FONT).len() as f32;
    if note_y + lines * NOTE_DETAIL_LINE_HEIGHT > y + h - NOTES_BOTTOM_MARGIN {
        return;
    }
    draw_wrapped_text(
        detail,
        note_x,
        note_y,
        note_width,
        NOTE_DETAIL_FONT,
        NOTE_DETAIL_LINE_HEIGHT,
        dark::TEXT_DIM,
    );
}
