use alchemy_tower::{game::Game, window_conf};
use macroquad::prelude::next_frame;
use macroquad_toolkit::capture;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when ALCHEMY_TOWER_CAPTURE_PATH is set, seed a
    // scene, simulate deterministic frames, write a PNG, and exit.
    if let Some(configs) = capture::CaptureConfig::all_from_env("ALCHEMY_TOWER") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |_dt| {
                game.update();
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
