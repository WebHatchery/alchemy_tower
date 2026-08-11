use super::GameplayState;
use crate::data::GameData;

#[test]
fn town_to_tower_path_routes_through_plains() {
    let data = GameData::fallback();
    let state = GameplayState::new(&data);

    let path = state
        .area_path(&data, "town_square", "tower_entry")
        .expect("town should connect to tower");

    assert_eq!(
        path,
        vec!["town_to_plains".to_owned(), "plains_to_entry".to_owned()]
    );
}

#[test]
fn town_to_forest_path_routes_through_plains() {
    let data = GameData::fallback();
    let state = GameplayState::new(&data);

    let path = state
        .area_path(&data, "town_square", "moonlit_forest")
        .expect("town should connect to east forest");

    assert_eq!(
        path,
        vec!["town_to_plains".to_owned(), "plains_to_forest".to_owned()]
    );
}
