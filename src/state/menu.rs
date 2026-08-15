//! Title screen state.

use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};
use crate::view_models::menu::MenuScreenView;

#[path = "menu_fullscreen.rs"]
mod menu_fullscreen;
#[path = "menu_input.rs"]
mod menu_input;

use self::menu_fullscreen::{apply_fullscreen_enabled, saved_fullscreen_enabled};
use self::menu_input::{
    selected_gender_action, selected_settings_action, selected_title_action, GenderAction,
    SettingsAction, TitleAction,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleMode {
    Actions,
    GenderSelect,
    Settings,
}

pub(crate) struct MenuState {
    mode: TitleMode,
    status_text: String,
    fullscreen_enabled: bool,
    quiet_hud: bool,
}

impl MenuState {
    pub(crate) fn new() -> Self {
        Self {
            mode: TitleMode::Actions,
            status_text: String::new(),
            fullscreen_enabled: saved_fullscreen_enabled(),
            quiet_hud: crate::ui::quiet_hud_enabled(),
        }
    }

    pub(crate) fn new_gender_select() -> Self {
        let mut state = Self::new();
        state.mode = TitleMode::GenderSelect;
        state
    }

    pub(crate) fn update(&mut self, data: &GameData) -> Option<StateTransition> {
        if self.mode == TitleMode::Settings {
            self.update_settings();
            return None;
        }
        if self.mode == TitleMode::GenderSelect {
            return self.update_gender_select(data);
        }

        match selected_title_action() {
            Some(TitleAction::NewGame) => {
                self.mode = TitleMode::GenderSelect;
                self.status_text.clear();
                None
            }
            Some(TitleAction::LoadGame) => self.load_game(data),
            Some(TitleAction::Settings) => {
                self.mode = TitleMode::Settings;
                self.status_text.clear();
                None
            }
            None => None,
        }
    }

    fn update_gender_select(&mut self, data: &GameData) -> Option<StateTransition> {
        match selected_gender_action() {
            Some(GenderAction::Back) => {
                self.mode = TitleMode::Actions;
                None
            }
            Some(GenderAction::Start(gender)) => Some(StateTransition::EnterGameplay(
                GameplayState::new_with_gender(data, gender),
            )),
            None => None,
        }
    }

    fn update_settings(&mut self) {
        match selected_settings_action() {
            Some(SettingsAction::Back) => {
                self.mode = TitleMode::Actions;
            }
            Some(SettingsAction::ToggleFullscreen) => {
                self.fullscreen_enabled = !self.fullscreen_enabled;
                apply_fullscreen_enabled(self.fullscreen_enabled);
                self.status_text = ui_copy(if self.fullscreen_enabled {
                    "menu_fullscreen_on_status"
                } else {
                    "menu_fullscreen_off_status"
                })
                .to_owned();
            }
            Some(SettingsAction::ToggleQuietHud) => {
                self.quiet_hud = !self.quiet_hud;
                crate::ui::set_quiet_hud(self.quiet_hud);
                self.status_text = ui_copy(if self.quiet_hud {
                    "menu_quiet_hud_on_status"
                } else {
                    "menu_quiet_hud_off_status"
                })
                .to_owned();
            }
            None => {}
        }
    }

    fn load_game(&mut self, data: &GameData) -> Option<StateTransition> {
        if !GameplayState::saved_progress_exists() {
            self.status_text = ui_copy("menu_load_unavailable").to_owned();
            return None;
        }

        let mut gameplay = GameplayState::new(data);
        if gameplay.load_progress(data) {
            return Some(StateTransition::EnterGameplay(gameplay));
        }
        self.status_text = gameplay.pause_status_text().to_owned();
        None
    }

    pub(crate) fn draw(&self, data: &GameData, art: &ArtAssets) {
        crate::ui::draw_menu_screen(data, art, &self.menu_screen_view());
    }

    fn menu_screen_view(&self) -> MenuScreenView {
        MenuScreenView {
            showing_settings: self.mode == TitleMode::Settings,
            showing_gender_select: self.mode == TitleMode::GenderSelect,
            title: ui_copy("menu_title").to_owned(),
            subtitle: ui_copy("menu_subtitle").to_owned(),
            new_game_label: ui_copy("menu_new_game").to_owned(),
            load_game_label: ui_copy("menu_load_game").to_owned(),
            settings_label: ui_copy("menu_settings").to_owned(),
            settings_title: ui_copy("menu_settings_title").to_owned(),
            settings_hint: ui_copy("menu_settings_hint").to_owned(),
            fullscreen_label: ui_copy(if self.fullscreen_enabled {
                "menu_fullscreen_on"
            } else {
                "menu_fullscreen_off"
            })
            .to_owned(),
            quiet_hud_label: ui_copy(if self.quiet_hud {
                "menu_quiet_hud_on"
            } else {
                "menu_quiet_hud_off"
            })
            .to_owned(),
            settings_back_label: ui_copy("menu_settings_back").to_owned(),
            gender_title: ui_copy("menu_gender_title").to_owned(),
            gender_hint: ui_copy("menu_gender_hint").to_owned(),
            female_label: ui_copy("menu_gender_female").to_owned(),
            male_label: ui_copy("menu_gender_male").to_owned(),
            gender_back_label: ui_copy("menu_gender_back").to_owned(),
            status_text: self.status_text.clone(),
        }
    }
}
