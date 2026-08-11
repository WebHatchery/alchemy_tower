use super::GameplayState;
use crate::content::{narrative_text, ui_copy, NarrativeEpilogueBeat};
use crate::view_models::ending::EndingOverlayView;

/// The panel holds roughly fourteen lines at the design size and the fixed
/// paragraph already uses five, so the opening page has room for three beats.
pub(crate) const MAX_EPILOGUE_BEATS: usize = 3;

/// Later pages carry no fixed paragraph, so they have room for one more beat
/// than the opener. Five overran the box by 134 characters — the budget test
/// below is what says so.
const LATER_PAGE_BEATS: usize = 4;

impl GameplayState {
    /// The epilogue beats this run earned, heaviest first.
    ///
    /// All of them. The panel used to show the top three and stop, and since
    /// reaching the ending at all earns two of the highest beats outright, only
    /// one slot was ever really contested — nine of the twelve were invisible
    /// even to a player who had done everything. The box cannot grow, so the
    /// ending is read a few beats at a time instead.
    fn earned_epilogue_beats(&self) -> Vec<&'static NarrativeEpilogueBeat> {
        let mut earned = narrative_text()
            .epilogue_beats
            .iter()
            .filter(|beat| self.epilogue_beat_earned(beat))
            .collect::<Vec<_>>();
        earned.sort_by_key(|beat| std::cmp::Reverse(beat.order));
        earned
    }

    /// Beats shown on `page`, and how many pages there are in total.
    fn epilogue_page(&self, page: usize) -> (Vec<&'static NarrativeEpilogueBeat>, usize) {
        let earned = self.earned_epilogue_beats();
        let after_first = earned.len().saturating_sub(MAX_EPILOGUE_BEATS);
        let pages = 1 + after_first.div_ceil(LATER_PAGE_BEATS);
        let page = page.min(pages - 1);

        let shown = if page == 0 {
            earned.into_iter().take(MAX_EPILOGUE_BEATS).collect()
        } else {
            earned
                .into_iter()
                .skip(MAX_EPILOGUE_BEATS + (page - 1) * LATER_PAGE_BEATS)
                .take(LATER_PAGE_BEATS)
                .collect()
        };
        (shown, pages)
    }

    /// How many pages the epilogue runs to, so the input handler knows when
    /// confirming should turn a page and when it should close the game out.
    pub(super) fn epilogue_page_count(&self) -> usize {
        self.epilogue_page(0).1
    }

    pub(super) fn ending_overlay_view(&self) -> EndingOverlayView {
        let narrative = narrative_text();
        let page = self.ui.ending_page;
        let (shown, pages) = self.epilogue_page(page);

        // The opening paragraph sets the scene once; turning the page continues
        // the list rather than starting it again.
        let mut body = if page == 0 {
            narrative.overlays.observatory_epilogue.clone()
        } else {
            String::new()
        };
        for beat in shown {
            if !body.is_empty() {
                body.push_str("\n\n");
            }
            body.push_str(&beat.line);
        }

        let footer = if page + 1 < pages {
            ui_copy("overlay_ending_more").to_owned()
        } else {
            narrative.overlays.observatory_footer.clone()
        };

        EndingOverlayView {
            title: ui_copy("overlay_ending_title").to_owned(),
            body,
            footer,
        }
    }

    fn epilogue_beat_earned(&self, beat: &NarrativeEpilogueBeat) -> bool {
        beat.after_milestones
            .iter()
            .all(|milestone_id| self.has_journal_milestone(milestone_id))
    }
}

#[cfg(test)]
#[path = "gameplay_ending_overlay_view/tests.rs"]
mod tests;
