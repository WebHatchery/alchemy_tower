use super::{
    GameplayState, TUTORIAL_ALCHEMY_OPEN, TUTORIAL_CROW_INTRO, TUTORIAL_HINT_KEYS,
    TUTORIAL_JOURNAL, TUTORIAL_POTIONS,
};
use crate::content::input_bindings;

/// The hint layer was invisible for the life of the project, so nothing had
/// ever checked that a hint naming a key names the *bound* one. Three of
/// them said "Press J" and "with E" as literals while the rest of the HUD
/// reads `input_bindings.json`, and one of those keys is rebindable in the
/// same file that draws the control tags.
#[test]
fn a_hint_that_names_a_key_names_the_one_that_is_bound() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let bindings = input_bindings();

    for (key, placeholder, expected) in [
        (
            TUTORIAL_JOURNAL,
            "{journal}",
            bindings.global.journal.as_str(),
        ),
        (
            TUTORIAL_ALCHEMY_OPEN,
            "{alchemy}",
            bindings.alchemy.open.as_str(),
        ),
        (
            TUTORIAL_POTIONS,
            "{quick_potions}",
            bindings.global.quick_potions[0].as_str(),
        ),
    ] {
        // The copy has to *ask* for the binding. Checking only the rendered
        // string would pass against the literal it replaced: "Press J to
        // open the field journal" contains the bound key by coincidence.
        assert!(
            crate::content::ui_copy(key).contains(placeholder),
            "{key} does not ask for {placeholder}; it spells the key out"
        );
        let text = state.tutorial_hint_text(key);
        assert!(
            text.contains(expected),
            "{key} does not name the bound key {expected:?}: {text}"
        );
        assert!(
            !text.contains('{'),
            "{key} has a placeholder nothing filled in: {text}"
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
