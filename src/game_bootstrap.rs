use super::game_state::GameState;
use super::Game;
use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::load_embedded_or_fallback;

pub(super) async fn load_game() -> Game {
    let data = load_embedded_or_fallback();
    let art = ArtAssets::load(&data)
        .await
        .unwrap_or_else(|error| panic!("Failed to load art assets: {error}"));
    let audio = load_audio().await;

    Game {
        data,
        art,
        audio,
        state: Some(GameState::new_menu()),
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_audio() -> AudioAssets {
    // Browser audio decoding can be unavailable until a user gesture, and it
    // must never keep the touch-first game from reaching its menu. The sound
    // hooks accept empty variation sets, so WebGL continues silently instead.
    eprintln!("WebGL audio is disabled so unsupported browser decoding cannot block startup.");
    AudioAssets::without_sounds()
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_audio() -> AudioAssets {
    AudioAssets::load()
        .await
        .unwrap_or_else(|error| panic!("Failed to load audio assets: {error}"))
}
