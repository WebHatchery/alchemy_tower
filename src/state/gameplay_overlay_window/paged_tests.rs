use super::{paged_window, ARCHIVE_PAGE_ROWS};

/// The failure this replaces: a selection past the first page was drawn on
/// no page at all, so nothing appeared highlighted and the detail panel
/// described a row the player could not see.
#[test]
fn every_selection_lands_on_the_page_that_is_drawn() {
    for total in 1..60usize {
        for selected in 0..total {
            let (start, _) = paged_window(selected, total, ARCHIVE_PAGE_ROWS);
            assert!(
                selected >= start && selected < start + ARCHIVE_PAGE_ROWS,
                "selection {selected} of {total} is not on the page starting at {start}"
            );
        }
    }
}

#[test]
fn a_single_page_is_not_announced() {
    assert_eq!(
        paged_window(0, ARCHIVE_PAGE_ROWS, ARCHIVE_PAGE_ROWS).1,
        None
    );
    assert!(paged_window(0, ARCHIVE_PAGE_ROWS + 1, ARCHIVE_PAGE_ROWS)
        .1
        .is_some());
}
