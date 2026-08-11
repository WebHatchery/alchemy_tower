use super::GameplayState;

#[test]
fn herb_usage_names_known_recipes_and_points_at_the_rest() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);

    // Whisper Moss feeds the starter Healing Draught (known at new game) plus
    // other formulae that are still discovery-only.
    let text = state
        .herb_used_in_text(&data, "whisper_moss")
        .expect("whisper moss is used in recipes");
    assert!(text.contains("Healing Draught"), "got: {text}");
    assert!(
        text.contains("The nearest wants"),
        "undiscovered uses pointed at: {text}"
    );

    // Field Bloom is not in any starter recipe, so its uses read as
    // undiscovered rather than naming a formula.
    let field_bloom = state
        .herb_used_in_text(&data, "field_bloom")
        .expect("field bloom is used in recipes");
    assert!(!field_bloom.contains("Brews into:"), "got: {field_bloom}");

    // The southern pass herbs returned None here for two iterations, which
    // rendered as a blank where every other herb explains itself: nothing
    // brewed with them, so the journal had nothing to say. A herb the
    // player can pick should always be able to answer "what is this for".
    //
    // Answering it used to stop at a count — "used in formulae you have not
    // yet discovered" — which named the gap and not one thing to do about
    // it. Every entry now says where the missing half comes from.
    for herb_id in ["coldiron_lichen", "rimeflower", "field_bloom"] {
        let text = state
            .herb_used_in_text(&data, herb_id)
            .unwrap_or_else(|| panic!("{herb_id} should read as used somewhere"));
        assert!(
            text.contains("The nearest wants"),
            "{herb_id} uses should point somewhere: {text}"
        );
    }
}
