use super::{toast_text_width, TOAST_MAX_LINES};

/// Measured off `screenshots/hud/event_toasts.png`: the banner's text column
/// is 618 wide at the reference window and wraps at about 60 characters per
/// line at font 18. Deliberately generous — a long word wraps early.
const CHARS_PER_LINE: usize = 60;
/// The shortest window the game is laid out for.
const REFERENCE_SCREEN_WIDTH: f32 = 1280.0;

/// A banner leaves after a couple of seconds and holds three lines. Anything
/// longer is cut mid-sentence — the same failure the epilogue and the route
/// pane each shipped once, and the crow's opening instruction shipped in
/// this box the day it became visible.
///
/// Both families of authored line go through this box, so both are walked:
/// the first version of this guard covered only the townsfolk's remarks and
/// the tutorial hints overran it the same afternoon.
#[test]
fn everything_raised_in_a_banner_fits_it() {
    use crate::state::GameplayState;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let budget = TOAST_MAX_LINES * CHARS_PER_LINE;
    assert!(
        toast_text_width(REFERENCE_SCREEN_WIDTH) > 0.0,
        "the banner has no room for text at all"
    );

    let mut overrun = data
        .npcs
        .iter()
        .filter(|npc| npc.exceptional_delivery_line.len() > budget)
        .map(|npc| {
            format!(
                "{}: {} characters against {budget}",
                npc.id,
                npc.exceptional_delivery_line.len()
            )
        })
        .collect::<Vec<_>>();
    overrun.extend(
        state
            .tutorial_hint_texts()
            .into_iter()
            .filter(|(_, text)| text.len() > budget)
            .map(|(key, text)| format!("{key}: {} characters against {budget}", text.len())),
    );

    assert!(
        overrun.is_empty(),
        "lines that will not fit the banner they are raised in: {overrun:#?}"
    );
}
