//! Menu and responsive settings geometry shared with touch input.
use macroquad::prelude::{screen_height, screen_width, Rect};

/// Bounds shared by settings drawing and touch input.
#[derive(Clone, Copy, Debug)]
pub struct SettingsLayout {
    pub panel: Rect,
    pub controls: [Rect; 8],
    pub back_button: Rect,
    pub status_y: f32,
    pub show_menu_title: bool,
}

pub fn title_button_rect(index: usize) -> Rect {
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

pub fn status_y() -> f32 {
    let last_button = title_button_rect(2);
    (last_button.y + last_button.h + 28.0).min(screen_height() - 28.0)
}

pub fn settings_layout() -> SettingsLayout {
    settings_layout_for_viewport(screen_width(), screen_height())
}

pub fn settings_layout_for_viewport(width: f32, height: f32) -> SettingsLayout {
    let columns = if width >= 580.0 { 2 } else { 1 };
    let rows = 8 / columns;
    let compact = height < 600.0 || columns == 1;
    let gap = if compact { 6.0 } else { 10.0 };
    let button_height = if compact { 44.0 } else { 50.0 };
    let header = 76.0;
    let footer = 74.0;
    let panel_height = header + rows as f32 * button_height + (rows - 1) as f32 * gap + footer;
    let panel_width = (if columns == 2 { 760.0_f32 } else { 400.0 }).min(width - 24.0);
    let show_menu_title = height >= panel_height + 222.0;
    let panel_y = if show_menu_title {
        ((height - panel_height) * 0.5 + 72.0).max(204.0)
    } else {
        (height - panel_height) * 0.5
    };
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        panel_y,
        panel_width,
        panel_height,
    );
    let control_width = (panel.w - 32.0 - (columns - 1) as f32 * gap) / columns as f32;
    let controls = std::array::from_fn(|index| {
        Rect::new(
            panel.x + 16.0 + (index % columns) as f32 * (control_width + gap),
            panel.y + header + (index / columns) as f32 * (button_height + gap),
            control_width,
            button_height,
        )
    });
    SettingsLayout {
        panel,
        controls,
        back_button: Rect::new(panel.x + 16.0, panel.bottom() - 54.0, panel.w - 32.0, 44.0),
        status_y: panel.bottom() - 60.0,
        show_menu_title,
    }
}

pub fn gender_select_rect() -> Rect {
    let width = 540.0_f32.min(screen_width() - 40.0);
    let height = 350.0_f32.min(screen_height() - 210.0);
    Rect::new(
        screen_width() * 0.5 - width * 0.5,
        (screen_height() * 0.5 - height * 0.5 + 62.0).max(174.0),
        width,
        height,
    )
}

pub fn gender_choice_rect(index: usize) -> Rect {
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

pub fn gender_back_rect() -> Rect {
    let panel = gender_select_rect();
    Rect::new(
        panel.x + 24.0,
        panel.y + panel.h - 44.0,
        panel.w - 48.0,
        34.0,
    )
}
