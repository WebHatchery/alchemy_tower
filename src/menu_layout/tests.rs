use super::*;

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
        (1280.0, 720.0),
        (960.0, 540.0),
        (640.0, 480.0),
        (640.0, 360.0),
    ] {
        let layout = settings_layout_for_viewport(width, height);

        assert!(rect_is_on_screen(layout.panel, width, height));
        assert!(rect_is_inside(layout.fullscreen_toggle, layout.panel));
        assert!(rect_is_inside(layout.quiet_hud_toggle, layout.panel));
        assert!(rect_is_inside(layout.back_button, layout.panel));
        assert!(rect_is_on_screen(layout.fullscreen_toggle, width, height));
        assert!(rect_is_on_screen(layout.quiet_hud_toggle, width, height));
        assert!(rect_is_on_screen(layout.back_button, width, height));
    }
}

#[test]
fn settings_hides_the_menu_heading_when_there_is_not_room_for_both() {
    assert!(settings_layout_for_viewport(640.0, 480.0).show_menu_title);
    assert!(!settings_layout_for_viewport(640.0, 360.0).show_menu_title);
}
