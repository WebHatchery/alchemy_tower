use alchemy_tower::menu_layout::*;
use macroquad::prelude::Rect;

fn rect_is_inside(inner: Rect, outer: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.x + inner.w <= outer.x + outer.w
        && inner.y + inner.h <= outer.y + outer.h
}

fn rect_is_on_screen(rect: Rect, width: f32, height: f32) -> bool {
    rect_is_inside(rect, Rect::new(0.0, 0.0, width, height))
}

#[test]
fn settings_controls_stay_inside_their_panel_at_supported_viewports() {
    for (width, height) in [
        (360.0, 640.0),
        (320.0, 568.0),
        (1280.0, 720.0),
        (960.0, 540.0),
        (640.0, 480.0),
        (640.0, 360.0),
    ] {
        let layout = settings_layout_for_viewport(width, height);

        assert!(rect_is_on_screen(layout.panel, width, height));
        let mut controls = layout.controls.to_vec();
        controls.push(layout.back_button);
        for (index, button) in controls.iter().enumerate() {
            assert!(rect_is_inside(*button, layout.panel));
            assert!(button.h >= 44.0);
            assert!(button.y >= layout.panel.y + 76.0);
            for other in controls.iter().skip(index + 1) {
                assert!(!button.overlaps(other));
            }
        }
    }
}

#[test]
fn settings_hides_the_menu_heading_when_there_is_not_room_for_both() {
    assert!(settings_layout_for_viewport(640.0, 720.0).show_menu_title);
    assert!(!settings_layout_for_viewport(640.0, 360.0).show_menu_title);
}
