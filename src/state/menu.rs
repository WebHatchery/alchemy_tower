//! Title screen state.

use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};
use crate::view_models::menu::MenuScreenView;

#[path = "menu_input.rs"]
mod menu_input;

use self::menu_input::{
    selected_gender_action, selected_settings_action, selected_title_action, GenderAction,
    SettingsAction, TitleAction,
};
use crate::settings::{self, Preferences};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleMode {
    Actions,
    GenderSelect,
    Settings,
}

pub(crate) struct MenuState {
    mode: TitleMode,
    status_text: String,
    preferences: Preferences,
}

impl MenuState {
    pub(crate) fn new() -> Self {
        Self {
            mode: TitleMode::Actions,
            status_text: String::new(),
            preferences: settings::current(),
        }
    }

    pub(crate) fn new_gender_select() -> Self {
        let mut state = Self::new();
        state.mode = TitleMode::GenderSelect;
        state
    }

    pub(crate) fn new_settings() -> Self {
        let mut state = Self::new();
        state.mode = TitleMode::Settings;
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
                self.status_text.clear();
            }
            Some(SettingsAction::Row(row)) => self.change_setting(row),
            None => {}
        }
    }

    fn change_setting(&mut self, row: usize) {
        let mut next = self.preferences.clone();
        match row {
            0 => next.common.fullscreen = !next.common.fullscreen,
            1 => next.quiet_hud = !next.quiet_hud,
            2 => next.common.screen_shake = !next.common.screen_shake,
            3 => next.common.reduced_motion = !next.common.reduced_motion,
            4 => next.common.master_volume = cycle_volume(next.common.master_volume),
            5 => next.common.sfx_volume = cycle_volume(next.common.sfx_volume),
            6 => next.common.show_fps = !next.common.show_fps,
            7 => next = Preferences::default(),
            _ => return,
        }
        if let Err(error) = next.save() {
            self.status_text = format!("{} {error}", ui_copy("menu_settings_save_error"));
            return;
        }
        if next.common.fullscreen != self.preferences.common.fullscreen {
            macroquad::prelude::set_fullscreen(next.common.fullscreen);
        }
        crate::ui::set_quiet_hud(next.quiet_hud);
        settings::apply(next.clone());
        self.preferences = next;
        self.status_text = ui_copy("menu_settings_saved").to_owned();
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
            settings_title: ui_copy("menu_settings").to_owned(),
            settings_hint: ui_copy(if cfg!(target_arch = "wasm32") {
                "menu_audio_web"
            } else {
                "menu_settings_hint"
            })
            .to_owned(),
            setting_labels: std::array::from_fn(|index| self.setting_label(index)),
            settings_back_label: ui_copy("menu_settings_back").to_owned(),
            gender_title: ui_copy("menu_gender_title").to_owned(),
            gender_hint: ui_copy("menu_gender_hint").to_owned(),
            female_label: ui_copy("menu_gender_female").to_owned(),
            male_label: ui_copy("menu_gender_male").to_owned(),
            gender_back_label: ui_copy("menu_gender_back").to_owned(),
            status_text: self.status_text.clone(),
        }
    }
    fn setting_label(&self, row: usize) -> String {
        let common = &self.preferences.common;
        let (key, enabled) = match row {
            0 => ("menu_fullscreen", common.fullscreen),
            1 => ("menu_quiet_hud", self.preferences.quiet_hud),
            2 => ("menu_shake", common.screen_shake),
            3 => ("menu_reduced_motion", common.reduced_motion),
            4 | 5 => {
                let (key, volume) = if row == 4 {
                    ("menu_master", common.master_volume)
                } else {
                    ("menu_sfx", common.sfx_volume)
                };
                return format!("{}: {:.0}%", ui_copy(key), volume * 100.0);
            }
            6 => ("menu_fps", common.show_fps),
            _ => return ui_copy("menu_defaults").to_owned(),
        };
        format!(
            "{}: {}",
            ui_copy(key),
            ui_copy(if enabled { "menu_on" } else { "menu_off" })
        )
    }
}

fn cycle_volume(value: f32) -> f32 {
    if value >= 0.99 {
        0.0
    } else {
        (value + 0.1).min(1.0)
    }
}
