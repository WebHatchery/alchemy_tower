//! High-level game loop orchestration.

use macroquad::prelude::{clear_background, Color};

#[path = "game_bootstrap.rs"]
mod game_bootstrap;
#[path = "game_capture.rs"]
mod game_capture;
#[path = "game_state.rs"]
mod game_state;
#[path = "game_transition.rs"]
mod game_transition;

use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::GameData;

use self::game_state::GameState;
use self::game_transition::apply_transition;

pub struct Game {
    data: GameData,
    art: ArtAssets,
    audio: AudioAssets,
    state: Option<GameState>,
}

impl Game {
    pub async fn new() -> Self {
        game_bootstrap::load_game().await
    }

    pub fn update(&mut self) {
        let Some(mut current_state) = self.state.take() else {
            return;
        };
        let transition = current_state.update(&self.data, &self.audio);

        self.state = Some(match transition {
            Some(next) => apply_transition(current_state, next),
            None => current_state,
        });
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(22, 24, 30, 255));

        let Some(state) = self.state.as_ref() else {
            return;
        };
        state.draw(&self.data, &self.art);
        if crate::settings::current().common.show_fps {
            macroquad::prelude::draw_rectangle(8.0, 8.0, 100.0, 28.0, macroquad::prelude::BLACK);
            macroquad_toolkit::ui::draw_ui_text(
                &format!("{} FPS", macroquad::prelude::get_fps()),
                16.0,
                28.0,
                20.0,
                macroquad::prelude::WHITE,
            );
        }
    }

    /// Seed a specific scene for the screenshot harness.
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.state = Some(game_capture::build(&self.data, scene));
    }
}
