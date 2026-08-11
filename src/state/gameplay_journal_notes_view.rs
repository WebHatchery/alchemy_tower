use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalMilestoneStatusView, JournalNoteRowView, JournalNotesTabView,
};

/// Titles are one line each, with the selected beat's paragraph beneath them.
/// Six leaves room for the longest beat in the game underneath — see
/// `the_longest_recorded_beat_fits_the_panel`, which does that arithmetic
/// against the renderer's own numbers.
pub(crate) const VISIBLE_NOTE_ROWS: usize = 6;

impl GameplayState {
    pub(super) fn journal_notes_tab_view(&self, data: &GameData) -> JournalNotesTabView {
        // Newest first: the thing that just happened is what a player opening
        // the journal is usually looking for.
        let notes = self
            .progression
            .journal_milestones
            .iter()
            .rev()
            .collect::<Vec<_>>();
        let total = notes.len();
        let selected = self.ui.journal_index.min(total.saturating_sub(1));
        let start = visible_window_start(selected, total, VISIBLE_NOTE_ROWS);

        JournalNotesTabView {
            title: ui_copy("overlay_tower_notes"),
            active_title: ui_copy("overlay_progress_active"),
            milestones_title: ui_copy("overlay_progress_milestones"),
            notes_title: ui_copy("overlay_recorded_notes"),
            active_summary: self
                .active_quest_summary(data)
                .unwrap_or_else(|| self.next_goal_summary(data)),
            milestone_rows: self
                .milestone_status_lines(data)
                .into_iter()
                .map(|(label, detail, ready)| JournalMilestoneStatusView {
                    title: ui_format(
                        "journal_milestone_status_title",
                        &[
                            ("label", label),
                            (
                                "status",
                                if ready {
                                    ui_copy("overlay_progress_ready")
                                } else {
                                    ui_copy("overlay_progress_locked")
                                },
                            ),
                        ],
                    ),
                    detail,
                })
                .collect(),
            note_rows: notes
                .iter()
                .enumerate()
                .skip(start)
                .take(VISIBLE_NOTE_ROWS)
                .map(|(index, milestone)| JournalNoteRowView {
                    title: milestone.title.clone(),
                    selected: index == selected,
                })
                .collect(),
            note_range_text: (total > VISIBLE_NOTE_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        ("last", &(start + VISIBLE_NOTE_ROWS).min(total).to_string()),
                        ("total", &total.to_string()),
                    ],
                )
            }),
            note_detail: notes.get(selected).map(|milestone| milestone.text.clone()),
        }
    }
}

#[cfg(test)]
#[path = "gameplay_journal_notes_view/tests.rs"]
mod tests;
