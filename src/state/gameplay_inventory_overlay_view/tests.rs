use super::GameplayState;

#[test]
fn inventory_view_explains_quantity_uses_and_potion_action() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    state.inventory.insert("healing_draught".to_owned(), 3);
    state
        .progression
        .started_quests
        .insert("healing_for_mira".to_owned());

    let view = state.inventory_overlay_view(&data);
    let detail = view.detail.expect("the held potion should be selected");
    assert!(detail.title.contains("Healing"));
    assert!(detail.quantity_text.contains('3'));
    assert!(detail.uses_text.contains("quest"));
    assert!(detail.can_use);
    assert!(detail.action_text.contains("USE"));
}

#[test]
fn inventory_view_keeps_the_selected_row_inside_the_visible_window() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for item in &data.items {
        state.inventory.insert(item.id.clone(), 1);
    }
    state.ui.inventory_index = usize::MAX;

    let view = state.inventory_overlay_view(&data);
    assert_eq!(view.items.iter().filter(|item| item.selected).count(), 1);
    assert!(view.page_text.is_some());
}
