//! Persistent tower preferences backed by the toolkit settings model and storage.
use macroquad_toolkit::{persistence, settings::GameSettings};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

const GAME: &str = "alchemy_tower";
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    #[serde(flatten)]
    pub common: GameSettings,
    pub quiet_hud: bool,
}

impl Preferences {
    pub fn sanitize(&mut self) {
        self.common.sanitize();
    }
    pub fn save_with(
        &mut self,
        save: impl FnOnce(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.sanitize();
        save(self)
    }
    pub fn save(&mut self) -> Result<(), String> {
        self.save_with(|value| persistence::save_json_key(GAME, "settings", value))
    }
}
thread_local! {
    static CURRENT: RefCell<Preferences> = RefCell::new(Preferences::default());
}
pub fn current() -> Preferences {
    CURRENT.with(|value| value.borrow().clone())
}
pub fn apply(value: Preferences) {
    value.common.apply_effects();
    CURRENT.with(|current| *current.borrow_mut() = value);
}
pub fn load() -> Result<(), String> {
    if !persistence::json_key_exists(GAME, "settings") {
        return Ok(());
    }
    let mut value: Preferences = persistence::load_json_key(GAME, "settings")?;
    value.sanitize();
    // Browsers require a fresh user gesture to enter fullscreen after a reload.
    #[cfg(target_arch = "wasm32")]
    {
        value.common.fullscreen = false;
    }
    apply(value);
    Ok(())
}

/// Freeze cosmetic animation while simulation and status changes continue.
pub fn effect_time(time: f32) -> f32 {
    if macroquad_toolkit::settings::reduced_motion_enabled() {
        0.0
    } else {
        time
    }
}
