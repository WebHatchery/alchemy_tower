use super::{locked_summary_text, GameplayState, VISIBLE_BOARD_ROWS};

/// The locked box is 836px wide at 16px, which is about 95 characters, and
/// 54px tall at an 18px line height, which is three lines. Measured against
/// captures: every locked request at once ran through two sections below it,
/// and even two summaries spilled the third line.
const LOCKED_TEXT_BUDGET: usize = 280;

#[test]
fn the_locked_list_stays_inside_its_box() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    // A fresh save is the worst case: nearly every board request is locked.
    let locked = state.locked_board_quest_summaries(&data);
    assert!(
        locked.len() > 1,
        "this only proves anything while several requests are locked"
    );

    let text = locked_summary_text(&locked);
    assert!(
        text.chars().count() <= LOCKED_TEXT_BUDGET,
        "locked list is {} characters, over the {LOCKED_TEXT_BUDGET} its box shows:\n{text}",
        text.chars().count()
    );
    assert!(
        text.contains(&(locked.len() - 1).to_string()),
        "the summary must say how many it is not showing, not drop them silently"
    );
}

#[test]
fn the_available_list_never_draws_more_cards_than_fit() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let view = state.quest_board_overlay_view(&data);
    assert!(
        view.entries.len() <= VISIBLE_BOARD_ROWS,
        "{} cards drawn into a box that holds {VISIBLE_BOARD_ROWS}",
        view.entries.len()
    );
}
