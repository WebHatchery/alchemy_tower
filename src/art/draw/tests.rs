use super::character_frame_index;
use macroquad::prelude::vec2;

#[test]
fn character_sheet_rows_cover_all_viewing_directions() {
    assert_eq!(
        character_frame_index(vec2(0.0, 1.0), false, 0.0),
        (0.0, 0.0)
    );
    assert_eq!(
        character_frame_index(vec2(-1.0, 0.0), false, 0.0),
        (0.0, 1.0)
    );
    assert_eq!(
        character_frame_index(vec2(1.0, 0.0), false, 0.0),
        (0.0, 2.0)
    );
    assert_eq!(
        character_frame_index(vec2(0.0, -1.0), false, 0.0),
        (0.0, 3.0)
    );
}

#[test]
fn character_sheet_walk_columns_cycle_after_idle() {
    assert_eq!(character_frame_index(vec2(0.0, 1.0), true, 0.0), (1.0, 0.0));
    assert_eq!(
        character_frame_index(vec2(0.0, 1.0), true, 1.0 / 7.0),
        (2.0, 0.0)
    );
    assert_eq!(
        character_frame_index(vec2(0.0, 1.0), true, 3.0 / 7.0),
        (4.0, 0.0)
    );
    assert_eq!(
        character_frame_index(vec2(0.0, 1.0), true, 4.0 / 7.0),
        (1.0, 0.0)
    );
}
