use super::super::gameplay_runtime_types::GameSound;
use super::GameplayState;

/// The game had five sounds — footsteps, a pickup, a bench opening, a stir
/// and a brew result — and every one of them is an *input*. Everything the
/// player was working towards happened in silence: a beat recorded, a
/// request finished, a route opened, a day run out. Each of those already
/// raised a toast, so the moment was identified; it just made no sound.
#[test]
fn the_moments_worth_hearing_now_make_a_sound() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    state.push_journal_milestone("a_beat", "A Beat", "Something happened.");
    assert!(
        state
            .runtime
            .pending_sounds
            .contains(&GameSound::JournalNote),
        "recording a journal beat made no sound"
    );

    state.runtime.pending_sounds.clear();
    state.trigger_quest_complete_feedback("delivered");
    assert!(
        state
            .runtime
            .pending_sounds
            .contains(&GameSound::WorkLanded),
        "finishing what the valley asked for made no sound"
    );

    state.runtime.pending_sounds.clear();
    state.trigger_route_restored_feedback("opened", [0.0, 0.0]);
    assert!(
        state
            .runtime
            .pending_sounds
            .contains(&GameSound::RouteRestored),
        "opening a route made no sound"
    );
}

/// Queued rather than played on the spot, because the code that knows a
/// moment happened is nowhere near the code that owns the speakers. That
/// only works if the queue is emptied every frame — otherwise it is a leak
/// that also, eventually, plays a hundred sounds at once.
#[test]
fn taking_the_queue_empties_it() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    // Titled, because an untitled beat is a capture scene seeding a gate
    // rather than something the player did, and those deliberately make no
    // noise and raise no banner.
    for index in 0..12 {
        state.push_journal_milestone(
            &format!("beat_{index}"),
            &format!("Beat {index}"),
            "Something happened.",
        );
    }

    let taken = state.take_pending_sounds();
    assert_eq!(taken.len(), 12, "the queue should hand over everything");
    assert!(
        state.take_pending_sounds().is_empty(),
        "the queue kept a copy of what it handed over"
    );
}

/// Every celebratory moment in the game raises a toast, and for the whole
/// life of the project the function that received them ignored its own
/// arguments: `_text`, `_color`, `_icon_key`, and a struct holding nothing
/// but a countdown. Thirteen authored strings, six generated icons and a
/// whole tutorial hint layer went into it and none of it could ever appear.
///
/// This checks the words survive the trip from the trigger to the view the
/// HUD draws — the trip that was severed.
#[test]
fn a_toast_carries_the_words_it_was_raised_with() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    state.trigger_quest_complete_feedback("The winter stores are laid in.");
    let toasts = state.build_hud_toasts();

    assert_eq!(toasts.len(), 1, "the toast never reached the HUD view");
    assert_eq!(toasts[0].text, "The winter stores are laid in.");
    assert_eq!(
        toasts[0].icon_key, "quest_complete",
        "the icon the trigger asked for was dropped"
    );
    assert!(toasts[0].alpha > 0.0, "a fresh banner should be visible");
    assert!(
        toasts[0].color[..3] != [0.0, 0.0, 0.0],
        "the event's colour was dropped"
    );
}

/// Three landing at once is normal — a delivery can record a beat, open a
/// route and finish an arc in the same keypress. The newest has to be first
/// in the list, which is the one drawn nearest the status strip, and the
/// stack is capped so a busy moment cannot bury the world behind banners.
#[test]
fn the_newest_banner_is_first_and_the_stack_is_capped() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    for index in 0..5 {
        state.trigger_quest_complete_feedback(format!("beat {index}"));
    }

    let toasts = state.build_hud_toasts();
    assert_eq!(toasts.len(), 3, "the stack should be capped at three");
    assert_eq!(toasts[0].text, "beat 4", "the newest should be first");
}
