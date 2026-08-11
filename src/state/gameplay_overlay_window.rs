//! Shared row-windowing for overlay lists.
//!
//! Several overlays draw fixed-height section boxes with no scrolling. Every one
//! of them was written when its list held two or three entries and quietly
//! overran its box once the content grew — the rune drafts list did, and the
//! quest board did the moment the board carried more than a handful of requests.

/// First row of the window that keeps `selected` visible. Clamping to the first
/// `window` entries instead would silently make every later entry unreachable,
/// which is worse than the overflow it fixes.
pub(super) fn visible_window_start(selected: usize, total: usize, window: usize) -> usize {
    if total <= window || window == 0 {
        return 0;
    }
    selected.saturating_sub(window - 1).min(total - window)
}

#[cfg(test)]
#[path = "gameplay_overlay_window/tests.rs"]
mod tests;

/// Rows shown at once by the archive's section lists and the brew journal.
pub(super) const ARCHIVE_PAGE_ROWS: usize = 6;

/// Where a paged list starts, and the line telling the player what fraction of
/// it they are looking at.
///
/// Four of the archive's five lists used to take the first six rows while the
/// selection index ranged over the whole list — so a player could select, and
/// act on, an entry that was neither drawn nor highlighted. Only the experiments
/// list paged correctly. This is that list's arithmetic, extracted so the other
/// four cannot drift from it again.
pub(super) fn paged_window(selected: usize, total: usize, rows: usize) -> (usize, Option<String>) {
    if total == 0 || rows == 0 {
        return (0, None);
    }
    let page = selected / rows;
    let start = page * rows;
    let page_count = total.div_ceil(rows);
    let text = (page_count > 1).then(|| {
        crate::content::ui_format(
            "overlay_page_of",
            &[
                ("page", &(page + 1).to_string()),
                ("count", &page_count.to_string()),
            ],
        )
    });
    (start, text)
}

#[cfg(test)]
#[path = "gameplay_overlay_window/paged_tests.rs"]
mod paged_tests;
