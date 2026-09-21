//! Deterministic scene setup used by the verification capture harness.

use super::game_state::GameState;
use crate::data::{GameData, PlayerGender};
use crate::state::{GameplayState, MenuState};

pub(super) fn build(data: &GameData, requested_scene: &str) -> GameState {
    let scene = match requested_scene.strip_suffix("+quiet") {
        Some(base) => {
            crate::ui::set_quiet_hud(true);
            base
        }
        None => requested_scene,
    };
    build_scene(data, scene)
}

fn build_scene(data: &GameData, scene: &str) -> GameState {
    // The suffix changes only the stable output filename; the captured scene
    // itself is the same deterministic sample at the requested viewport.
    let scene = scene
        .strip_suffix(":minimum")
        .or_else(|| scene.strip_suffix(":stress"))
        .or_else(|| scene.strip_suffix(":wide"))
        .unwrap_or(scene);
    match scene {
        "gameplay" => gameplay(data),
        "gameplay:female" => gameplay_with_gender(data, PlayerGender::Female),
        "gender" => GameState::Menu(MenuState::new_gender_select()),
        "settings" => GameState::Menu(MenuState::new_settings()),
        "paused" => GameState::pause(GameplayState::new(data)),
        "ending" => sample(data, GameplayState::open_full_ending),
        "inventory" => sample(data, |state| state.open_inventory_sample(data)),
        "brew" => sample(data, |state| state.open_alchemy_sample_brew(data)),
        "compound" => sample(data, |state| state.open_compound_brew_sample(data)),
        "toasts" => sample(data, |state| state.show_event_toasts_sample(data)),
        "rune" => sample(data, |state| state.open_rune_bench_sample(data)),
        "shop" => sample(data, |state| state.open_shop_sample(data)),
        "board" => sample(data, |state| state.open_quest_board_sample(data)),
        "brews_journal" => sample(data, |state| state.open_brews_journal_sample(data)),
        other if other.starts_with("afterword:") => build_afterword(data, other),
        other if other.starts_with("dialogue") => build_dialogue(data, other),
        other if other.starts_with("area:") => build_area(data, other),
        other if other.starts_with("archive") => build_archive(data, other),
        other if other.starts_with("notes") => build_notes(data, other),
        other if other.starts_with("journal") => build_journal(data, other),
        _ => GameState::new_menu(),
    }
}

fn gameplay(data: &GameData) -> GameState {
    GameState::from_gameplay(GameplayState::new(data))
}

fn gameplay_with_gender(data: &GameData, gender: PlayerGender) -> GameState {
    GameState::from_gameplay(GameplayState::new_with_gender(data, gender))
}

fn sample<F>(data: &GameData, configure: F) -> GameState
where
    F: FnOnce(&mut GameplayState),
{
    let mut gameplay = GameplayState::new(data);
    configure(&mut gameplay);
    GameState::from_gameplay(gameplay)
}

fn build_afterword(data: &GameData, scene: &str) -> GameState {
    let npc_id = scene.strip_prefix("afterword:").unwrap_or("crow_guide");
    sample(data, |state| {
        state.open_dialogue_after_everything(data, npc_id)
    })
}

fn build_dialogue(data: &GameData, scene: &str) -> GameState {
    let target = scene.strip_prefix("dialogue:").unwrap_or("mira_apothecary");
    let mut parts = target.split(':');
    let npc_id = parts.next().unwrap_or("mira_apothecary");
    let beat = parts
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let window = parts.next().unwrap_or("morning");
    sample(data, |state| {
        state.set_time_window(window);
        if beat > 0 {
            state.open_dialogue_at_arc_beat(data, npc_id, beat);
        } else {
            state.open_dialogue_with(npc_id);
        }
    })
}

fn build_area(data: &GameData, scene: &str) -> GameState {
    let target = scene.strip_prefix("area:").unwrap_or_default();
    let (area_id, rest) = target.split_once(':').unwrap_or((target, ""));
    let (day, time_window) = rest
        .split_once(':')
        .map(|(day, window)| (day.parse::<u32>().unwrap_or(0), window))
        .unwrap_or((rest.parse::<u32>().unwrap_or(0), "morning"));
    sample(data, |state| {
        state.preview_area(data, area_id, day, time_window)
    })
}

fn build_archive(data: &GameData, scene: &str) -> GameState {
    let target = scene.strip_prefix("archive:").unwrap_or("2:7");
    let (tab, index) = target
        .split_once(':')
        .map(|(tab, index)| {
            (
                tab.parse::<usize>().unwrap_or(2),
                index.parse::<usize>().unwrap_or(7),
            )
        })
        .unwrap_or((2, 7));
    sample(data, |state| state.open_archive_sample(data, tab, index))
}

fn build_notes(data: &GameData, scene: &str) -> GameState {
    let index = scene
        .strip_prefix("notes:")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    sample(data, |state| state.open_notes_sample(data, index))
}

fn build_journal(data: &GameData, scene: &str) -> GameState {
    let herb_id = scene
        .strip_prefix("journal:")
        .filter(|value| !value.is_empty())
        .unwrap_or("tumbled_glass");
    sample(data, |state| {
        state.open_journal_sample(data);
        state.select_journal_herb(data, herb_id);
    })
}
