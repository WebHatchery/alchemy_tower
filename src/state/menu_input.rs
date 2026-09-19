use crate::data::PlayerGender;
use crate::input::{cancel_pressed, confirm_pressed, fullscreen_pressed, rect_clicked};
use crate::menu_layout::{
    gender_back_rect, gender_choice_rect, settings_layout, title_button_rect,
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
    Row(usize),
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
    let layout = settings_layout();
    if cancel_pressed() || rect_clicked(layout.back_button) {
        return Some(SettingsAction::Back);
    }
    if fullscreen_pressed() {
        return Some(SettingsAction::Row(0));
    }
    layout
        .controls
        .iter()
        .position(|rect| rect_clicked(*rect))
        .map(SettingsAction::Row)
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
