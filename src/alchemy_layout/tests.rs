use super::*;
use macroquad::prelude::vec2;

#[test]
fn alchemy_touch_controls_stay_inside_the_content_panel() {
    let panel = Rect::new(0.0, 0.0, ALCHEMY_CONTENT_W, ALCHEMY_CONTENT_H);
    let controls = [
        material_row_rect_at(0.0, 0.0, 0),
        material_row_rect_at(0.0, 0.0, AL_MAT_VISIBLE_ROWS - 1),
        alchemy_slot_rect_at(0.0, 0.0, 0),
        alchemy_slot_rect_at(0.0, 0.0, 2),
        catalyst_rect_at(0.0, 0.0),
        sort_rect_at(0.0, 0.0),
        clear_rect_at(0.0, 0.0),
        repeat_rect_at(0.0, 0.0),
        brew_rect_at(0.0, 0.0),
        alchemy_close_rect_at(0.0, 0.0, ALCHEMY_CONTENT_W),
    ];
    for rect in controls {
        assert!(
            panel.contains(vec2(rect.x, rect.y))
                && panel.contains(vec2(rect.x + rect.w, rect.y + rect.h)),
            "alchemy control escaped the panel: {rect:?}"
        );
    }
}

#[test]
fn alchemy_primary_brew_action_is_separate_from_the_setup_controls() {
    let brew = brew_rect_at(0.0, 0.0);
    assert!(brew.y > AL_SLOT_BOX_Y + AL_SLOT_BOX_H);
    assert!(brew.w >= 100.0 && brew.h >= 28.0);
}
