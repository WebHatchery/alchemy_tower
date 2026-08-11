use super::{GameplayState, VISIBLE_HERB_ROWS, VISIBLE_ROUTE_ROWS};

fn seeded_state(data: &crate::data::GameData) -> GameplayState {
    let mut state = GameplayState::new(data);
    state.open_journal_sample(data);
    state
}

/// Both columns draw into fixed boxes with no scrollbar. Emitting more rows
/// than fit is how this tab came to be hiding ten routes and twenty-odd
/// herbs without saying so.
#[test]
fn neither_column_emits_more_rows_than_its_box_holds() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = seeded_state(&data);

    for index in 0..40 {
        state.ui.journal_index = index;
        let view = state.journal_routes_tab_view(&data);
        assert!(
            view.route_rows.len() <= VISIBLE_ROUTE_ROWS,
            "{} route rows at index {index}",
            view.route_rows.len()
        );
        assert!(
            view.herb_memories.rows.len() <= VISIBLE_HERB_ROWS,
            "{} herb rows at index {index}",
            view.herb_memories.rows.len()
        );
    }
}

/// Walking the list must actually reach the far end of it, and must always
/// show the thing it says is selected.
#[test]
fn walking_the_list_keeps_the_selection_visible_and_reaches_the_end() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = seeded_state(&data);
    let herb_total = state.herb_memories(&data).len();
    assert!(
        herb_total > VISIBLE_HERB_ROWS,
        "this only proves anything with more herbs than fit"
    );

    let mut last_row_seen = String::new();
    for index in 0..herb_total {
        state.ui.journal_index = index;
        let view = state.journal_routes_tab_view(&data);
        assert!(
            view.herb_memories.rows.iter().any(|row| row.selected),
            "nothing selected at index {index}"
        );
        assert!(
            view.route_rows.iter().any(|row| row.selected),
            "no route selected at index {index}"
        );
        assert!(view.herb_memories.detail.is_some(), "no detail at {index}");
        if let Some(row) = view.herb_memories.rows.last() {
            last_row_seen = row.title.clone();
        }
    }

    state.ui.journal_index = 0;
    let first = state.journal_routes_tab_view(&data);
    let first_page_last = first
        .herb_memories
        .rows
        .last()
        .map(|row| row.title.clone())
        .unwrap_or_default();
    assert_ne!(
        last_row_seen, first_page_last,
        "walking to the end never moved past the first page"
    );
}

/// Measured off `screenshots/hud/journal_hearsay.png`: the 600-wide herb
/// column wraps at about 88 characters per line at font 16. Deliberately
/// generous — a long word wraps early and costs a line, so a real entry
/// takes at least as many lines as this arithmetic says.
const CHARS_PER_LINE: usize = 88;
/// The shortest window the game is laid out for.
const REFERENCE_SCREEN_HEIGHT: f32 = 720.0;

/// The detail box has room for about four lines, and every entry was
/// leading with a description that wraps to three of them — so the
/// gathering conditions ran down through the Tower Access panel and the
/// "brews into" line fell off the bottom without a mark to say so. Those
/// two are the whole reason to open this tab.
///
/// This checks the worst entry the content can produce still gets both,
/// using the same layout numbers the renderer uses.
#[test]
fn every_herb_entry_gets_its_conditions_and_its_uses() {
    use crate::ui::{
        HERB_DETAIL_BLOCK_GAP, HERB_DETAIL_LINE_HEIGHT, HERB_DETAIL_TOP_GAP, HERB_LINE_STEP,
        HERB_ROW_STEP,
    };

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = seeded_state(&data);
    // The panel is `journal_panel_rect()` at the reference height, and the
    // herb detail is bounded by `y + h - 170` with the rows above it.
    let panel_y = 72.0;
    let panel_h = REFERENCE_SCREEN_HEIGHT - 144.0;
    let bottom_limit = panel_y + panel_h - 170.0;
    let rows_end = panel_y + 136.0 + 32.0 + super::VISIBLE_HERB_ROWS as f32 * HERB_ROW_STEP;
    let block_height = |text: &str| {
        let lines = text.len().div_ceil(CHARS_PER_LINE).max(1) as f32;
        lines * HERB_DETAIL_LINE_HEIGHT + HERB_DETAIL_BLOCK_GAP
    };

    let total = state.herb_memories(&data).len();
    let mut clipped = Vec::new();
    for index in 0..total {
        state.ui.journal_index = index;
        let view = state.journal_routes_tab_view(&data);
        let Some(entry) = view.herb_memories.detail else {
            continue;
        };
        let mut y = rows_end + HERB_DETAIL_TOP_GAP + HERB_LINE_STEP;
        y += block_height(&entry.conditions);
        if let Some(used_in) = &entry.used_in_text {
            y += block_height(used_in);
        }
        if y > bottom_limit {
            clipped.push(format!(
                "{}: needs {y:.0} against a {bottom_limit:.0} floor",
                entry.title
            ));
        }
    }

    clipped.sort();
    clipped.dedup();
    assert!(
        clipped.is_empty(),
        "herb entries whose conditions or uses fall out of the box:
{clipped:#?}"
    );
}

#[test]
fn the_counts_appear_only_when_something_is_out_of_sight() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let seeded = seeded_state(&data);
    let view = seeded.journal_routes_tab_view(&data);
    assert!(view.route_range_text.is_some(), "17 routes need a count");
    assert!(
        view.herb_memories.range_text.is_some(),
        "a full shelf needs a count"
    );

    let empty = GameplayState::new(&data);
    assert!(
        empty
            .journal_routes_tab_view(&data)
            .herb_memories
            .range_text
            .is_none(),
        "an empty shelf should not claim to be hiding anything"
    );
}
