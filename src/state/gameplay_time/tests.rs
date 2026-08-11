use super::GameplayState;

#[test]
fn waiting_across_a_window_refreshes_the_ground_without_changing_area() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let node_id = "archive_inkgall_01";
    state.world.current_area_id = "archive_floor".to_owned();
    state.set_clock_minutes(1261.0);

    let day = (0..40)
        .find(|day| {
            state.world.day_index = *day;
            state.refresh_available_nodes(&data);
            state.world.available_nodes.contains(node_id)
        })
        .expect("the night ink should have a valid day in the campaign cycle");
    state.world.day_index = day;
    let two_minutes = state.world.day_length_seconds * 2.0 / (24.0 * 60.0);

    state.set_clock_minutes(1259.0);
    state.refresh_available_nodes(&data);
    assert_eq!(state.current_time_window(), "evening");
    assert!(!state.world.available_nodes.contains(node_id));
    state.advance_clock(&data, two_minutes);
    assert_eq!(state.current_time_window(), "night");
    assert!(
        state.world.available_nodes.contains(node_id),
        "night ground did not arrive when night did"
    );

    state.set_clock_minutes(359.0);
    state.refresh_available_nodes(&data);
    assert!(state.world.available_nodes.contains(node_id));
    state.advance_clock(&data, two_minutes);
    assert_eq!(state.current_time_window(), "morning");
    assert!(
        !state.world.available_nodes.contains(node_id),
        "night ground remained after morning arrived"
    );
}
