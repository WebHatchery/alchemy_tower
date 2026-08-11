use macroquad::prelude::{vec2, Vec2};

#[cfg(test)]
use super::GameplayState;
use crate::data::{GameData, WarpDefinition};

pub(super) fn schedule_start_minutes(time_window: &str) -> f32 {
    match time_window {
        "morning" => 6.0 * 60.0,
        "day" => 11.0 * 60.0,
        "evening" => 17.0 * 60.0,
        _ => 21.0 * 60.0,
    }
}

pub(super) fn warp_center(warp: &WarpDefinition) -> Vec2 {
    vec2(
        warp.rect.x + warp.rect.w * 0.5,
        warp.rect.y + warp.rect.h * 0.5,
    )
}

pub(super) fn matching_arrival_position(
    data: &GameData,
    source_area_id: &str,
    warp: &WarpDefinition,
) -> Option<Vec2> {
    let target_area = data.area(&warp.target_area)?;
    target_area
        .warps
        .iter()
        .find(|candidate| candidate.target_area == source_area_id)
        .map(warp_center)
}

pub(super) fn npc_motion_seed(id: &str) -> f32 {
    let mut value = 0u32;
    for byte in id.as_bytes() {
        value = value.wrapping_mul(33).wrapping_add(*byte as u32);
    }
    (value % 360) as f32 * 0.017453292
}

#[cfg(test)]
#[path = "gameplay_npc_pathing/tests.rs"]
mod tests;
