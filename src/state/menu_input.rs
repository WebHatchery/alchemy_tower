use crate::data::PlayerGender;
use crate::input::{cancel_pressed, confirm_pressed, fullscreen_pressed, rect_clicked};
use crate::menu_layout::{
    fullscreen_toggle_rect, gender_back_rect, gender_choice_rect, quiet_hud_toggle_rect,
    settings_back_rect, title_button_rect,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TitleAction {
    NewGame,
    LoadGame,
    Settings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SettingsAction {
    Back,
    ToggleFullscreen,
    ToggleQuietHud,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GenderAction {
    Back,
    Start(PlayerGender),
}

pub(super) fn selected_title_action() -> Option<TitleAction> {
    if confirm_pressed() || rect_clicked(title_button_rect(0)) {
        return Some(TitleAction::NewGame);
    }

    if rect_clicked(title_button_rect(1)) {
        return Some(TitleAction::LoadGame);
    }

    rect_clicked(title_button_rect(2)).then_some(TitleAction::Settings)
}

pub(super) fn selected_settings_action() -> Option<SettingsAction> {
    if cancel_pressed() || rect_clicked(settings_back_rect()) {
        return Some(SettingsAction::Back);
    }

    if rect_clicked(quiet_hud_toggle_rect()) {
        return Some(SettingsAction::ToggleQuietHud);
    }

    (fullscreen_pressed() || rect_clicked(fullscreen_toggle_rect()))
        .then_some(SettingsAction::ToggleFullscreen)
}

pub(super) fn selected_gender_action() -> Option<GenderAction> {
    if cancel_pressed() || rect_clicked(gender_back_rect()) {
        return Some(GenderAction::Back);
    }
    if confirm_pressed() || rect_clicked(gender_choice_rect(0)) {
        return Some(GenderAction::Start(PlayerGender::Female));
    }
    rect_clicked(gender_choice_rect(1)).then_some(GenderAction::Start(PlayerGender::Male))
}
