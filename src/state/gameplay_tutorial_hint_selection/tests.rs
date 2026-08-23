use super::{GameplayState, TUTORIAL_CROW_INTRO, TUTORIAL_HINT_KEYS};

/// Browser hints have to name the button a player can actually touch. A
/// keyboard legend does not help a player with no keyboard and makes a visible
/// control look optional.
#[test]
fn tutorial_hints_name_visible_touch_controls() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    for key in TUTORIAL_HINT_KEYS {
        let text = state.tutorial_hint_text(key);
        assert!(
            !["Press ", "Tab", "Esc", "Enter", "Space", "F5", "F9", "F11"]
                .iter()
                .any(|keyboard| text.contains(keyboard)),
            "{key} still presents a keyboard-only instruction: {text}"
        );
        assert!(
            !text.contains('{'),
            "{key} left a placeholder unfilled: {text}"
        );
    }
}

/// Every hint has to have words behind it. A key with no line reads as an
/// empty banner, and a line formatted with a substitution its copy has no
/// placeholder for — which is what `tutorial_potions` did with the belt
/// keys — throws the value away without anything failing.
#[test]
fn every_hint_has_something_to_say() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    for key in TUTORIAL_HINT_KEYS {
        let text = state.tutorial_hint_text(key);
        assert!(!text.is_empty(), "{key} has no copy at all");
        assert!(
            !text.starts_with("[missing"),
            "{key} is missing from ui_text: {text}"
        );
        assert!(
            !text.contains('{'),
            "{key} left a placeholder unfilled: {text}"
        );
    }
}

/// The opening three fire on no condition at all, and the flags saying they
/// had been shown lived in runtime state — which is rebuilt on load. So a
/// player forty hours in was introduced to the crow, told how to save and
/// told how to open the journal every single time they opened a save. It
/// cost nothing while the banners were invisible and became a defect the
/// moment they were not.
#[test]
fn a_hint_already_seen_does_not_come_back_after_a_load() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    let (first, _) = state
        .take_next_tutorial_hint(&data)
        .expect("a new game should have something to say");
    assert!(
        first.contains("Crow"),
        "the opening line should be the crow's"
    );
    assert!(state
        .progression
        .shown_tutorial_hints
        .contains(TUTORIAL_CROW_INTRO));

    let snapshot = super::super::gameplay_save_snapshot::build_save_snapshot(&state, &data);
    let mut reloaded = GameplayState::new(&data);
    super::super::gameplay_save_restore::apply_save_snapshot(&mut reloaded, &data, snapshot)
        .expect("the save should load");
    assert!(
        reloaded
            .progression
            .shown_tutorial_hints
            .contains(TUTORIAL_CROW_INTRO),
        "the hint came back after a save and load"
    );
    assert!(
        reloaded
            .take_next_tutorial_hint(&data)
            .is_none_or(|(text, _)| !text.contains("Crow: Nothing grows")),
        "the crow introduced themselves again to a returning player"
    );
}
