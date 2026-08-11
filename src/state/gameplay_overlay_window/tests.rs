use super::visible_window_start;

#[test]
fn the_selection_is_always_inside_the_window() {
    for window in 1..6usize {
        for total in 1..40usize {
            for selected in 0..total {
                let start = visible_window_start(selected, total, window);
                assert!(
                    selected >= start && selected < start + window,
                    "selection {selected} of {total} fell outside a {window}-row window at {start}"
                );
                assert!(
                    start + window <= total.max(window),
                    "window ran past the list"
                );
            }
        }
    }
}

#[test]
fn a_short_list_never_scrolls() {
    assert_eq!(visible_window_start(0, 3, 5), 0);
    assert_eq!(visible_window_start(2, 3, 5), 0);
}
