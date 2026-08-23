use macroquad::prelude::{screen_height, screen_width, Rect};

const SCREEN_MARGIN: f32 = 24.0;
const SETTINGS_PANEL_HEIGHT: f32 = 300.0;
const SETTINGS_COMPACT_TITLE_MIN_HEIGHT: f32 = 462.0;
const SETTINGS_FULL_TITLE_MIN_HEIGHT: f32 = 528.0;

#[derive(Clone, Copy, Debug)]
pub(crate) struct SettingsLayout {
    pub(crate) panel: Rect,
    pub(crate) fullscreen_toggle: Rect,
    pub(crate) quiet_hud_toggle: Rect,
    pub(crate) back_button: Rect,
    pub(crate) show_menu_title: bool,
}

pub(crate) fn title_button_rect(index: usize) -> Rect {
    let button_width = if screen_width() < 760.0 { 250.0 } else { 320.0 };
    let button_height = if screen_height() < 500.0 { 40.0 } else { 48.0 };
    let gap = if screen_height() < 500.0 { 8.0 } else { 12.0 };
    let button_count = 3.0;
    let total_height = button_height * button_count + gap * (button_count - 1.0);
    let min_y: f32 = if screen_height() < 500.0 {
        152.0
    } else {
        248.0
    };
    let max_y = (screen_height() - total_height - 58.0).max(24.0);
    let start_y = (screen_height() * 0.5).clamp(min_y.min(max_y), max_y.max(min_y));

    Rect::new(
        screen_width() * 0.5 - button_width * 0.5,
        start_y + index as f32 * (button_height + gap),
        button_width,
        button_height,
    )
}

pub(crate) fn status_y() -> f32 {
    let last_button = title_button_rect(2);
    (last_button.y + last_button.h + 28.0).min(screen_height() - 28.0)
}

pub(crate) fn settings_rect() -> Rect {
    settings_layout_for_viewport(screen_width(), screen_height()).panel
}

pub(crate) fn fullscreen_toggle_rect() -> Rect {
    settings_layout_for_viewport(screen_width(), screen_height()).fullscreen_toggle
}

pub(crate) fn quiet_hud_toggle_rect() -> Rect {
    settings_layout_for_viewport(screen_width(), screen_height()).quiet_hud_toggle
}

pub(crate) fn settings_back_rect() -> Rect {
    settings_layout_for_viewport(screen_width(), screen_height()).back_button
}

pub(crate) fn settings_show_menu_title() -> bool {
    settings_layout_for_viewport(screen_width(), screen_height()).show_menu_title
}

pub(crate) fn settings_layout_for_viewport(width: f32, height: f32) -> SettingsLayout {
    let target_width: f32 = if width < 760.0 { 320.0 } else { 420.0 };
    let panel_width = target_width.min((width - SCREEN_MARGIN * 2.0).max(0.0));
    let panel_height = SETTINGS_PANEL_HEIGHT.min((height - SCREEN_MARGIN * 2.0).max(0.0));
    let show_menu_title = if height < 500.0 {
        height >= SETTINGS_COMPACT_TITLE_MIN_HEIGHT
    } else {
        height >= SETTINGS_FULL_TITLE_MIN_HEIGHT
    };
    let panel_y = if show_menu_title {
        let preferred = height * 0.5 - panel_height * 0.5 + 42.0;
        let min_y = menu_title_bottom(height) + 18.0;
        let max_y = height - SCREEN_MARGIN - panel_height;
        preferred.clamp(min_y, max_y)
    } else {
        (height - panel_height) * 0.5
    };
    let panel = Rect::new(
        width * 0.5 - panel_width * 0.5,
        panel_y,
        panel_width,
        panel_height,
    );
    let control_x = panel.x + 24.0;
    let control_width = (panel.w - 48.0).max(0.0);

    SettingsLayout {
        panel,
        fullscreen_toggle: Rect::new(control_x, panel.y + panel.h - 152.0, control_width, 44.0),
        quiet_hud_toggle: Rect::new(control_x, panel.y + panel.h - 102.0, control_width, 44.0),
        back_button: Rect::new(control_x, panel.y + panel.h - 52.0, control_width, 38.0),
        show_menu_title,
    }
}

fn menu_title_bottom(height: f32) -> f32 {
    if height < 500.0 {
        120.0
    } else {
        186.0
    }
}

pub(crate) fn gender_select_rect() -> Rect {
    let width = 540.0_f32.min(screen_width() - 40.0);
    let height = 350.0_f32.min(screen_height() - 210.0);
    Rect::new(
        screen_width() * 0.5 - width * 0.5,
        (screen_height() * 0.5 - height * 0.5 + 62.0).max(174.0),
        width,
        height,
    )
}

pub(crate) fn gender_choice_rect(index: usize) -> Rect {
    let panel = gender_select_rect();
    let gap = 18.0;
    let width = (panel.w - 48.0 - gap) * 0.5;
    Rect::new(
        panel.x + 24.0 + index as f32 * (width + gap),
        panel.y + panel.h - 106.0,
        width,
        52.0,
    )
}

pub(crate) fn gender_back_rect() -> Rect {
    let panel = gender_select_rect();
    Rect::new(
        panel.x + 24.0,
        panel.y + panel.h - 44.0,
        panel.w - 48.0,
        34.0,
    )
}

#[cfg(test)]
mod tests;
