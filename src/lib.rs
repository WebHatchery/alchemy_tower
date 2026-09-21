//! Public game crate used by the native/WebGL runner and integration checks.

pub mod game;
pub mod menu_layout;
pub mod settings;

mod alchemy;
mod alchemy_layout;
mod archive_layout;
mod art;
mod audio;
mod content;
mod data;
mod input;
mod inventory_layout;
mod journal_layout;
mod pause_layout;
mod save;
mod state;
mod ui;
mod ui_scale;
mod view_models;

use macroquad::prelude::Conf;
use macroquad_toolkit::capture;

/// Window configuration shared by the native and browser runners.
pub fn window_conf() -> Conf {
    capture::capture_window_conf("ALCHEMY_TOWER", content::ui_copy("window_title"), 1280, 720)
}
