//! World collision, responsive player-follow camera, and NPC draw positions.

use super::gameplay_npc::npc_motion_seed;
use super::gameplay_npc_types::NpcRuntimeState;
use super::gameplay_world_types::{CAMERA_PADDING, PLAYER_RADIUS};
use super::GameplayState;
use crate::data::{AreaDefinition, NpcDefinition};
use macroquad::prelude::{get_time, screen_height, screen_width, vec2, Vec2};

impl GameplayState {
    pub(super) fn resolve_area_collision(&self, area: &AreaDefinition, candidate: Vec2) -> Vec2 {
        let clamped = vec2(
            candidate
                .x
                .clamp(PLAYER_RADIUS, area.size[0] - PLAYER_RADIUS),
            candidate
                .y
                .clamp(PLAYER_RADIUS, area.size[1] - PLAYER_RADIUS),
        );
        if !self.collides(area, clamped) {
            return clamped;
        }
        let x_only = vec2(clamped.x, self.world.player.position.y);
        if !self.collides(area, x_only) {
            return x_only;
        }
        let y_only = vec2(self.world.player.position.x, clamped.y);
        if !self.collides(area, y_only) {
            return y_only;
        }
        self.world.player.position
    }

    pub(super) fn collides(&self, area: &AreaDefinition, point: Vec2) -> bool {
        area.blockers.iter().any(|blocker| {
            let nearest = vec2(
                point.x.clamp(blocker.x, blocker.x + blocker.w),
                point.y.clamp(blocker.y, blocker.y + blocker.h),
            );
            point.distance_squared(nearest) < PLAYER_RADIUS * PLAYER_RADIUS
        })
    }

    pub(super) fn camera_offset(&self, area: &AreaDefinition) -> Vec2 {
        let viewport = vec2(screen_width(), screen_height());
        let area_size = vec2(area.size[0], area.size[1]);
        let centered = (viewport - area_size) * 0.5;
        let padding = Vec2::splat(CAMERA_PADDING);
        // Collapse each axis to the room's center when its padded bounds fit.
        let min_offset = (viewport - area_size - padding).min(centered);
        let max_offset = padding.max(centered);
        let mut offset =
            (viewport * 0.5 - self.world.player.position).clamp(min_offset, max_offset);
        if macroquad_toolkit::settings::screen_shake_enabled() {
            offset += self.runtime.camera_shake.offset();
        }
        offset
    }

    pub(super) fn npc_draw_position(&self, npc: &NpcDefinition, runtime: &NpcRuntimeState) -> Vec2 {
        if macroquad_toolkit::settings::reduced_motion_enabled()
            || !runtime.moving
            || runtime.direction.length_squared() <= 0.0
        {
            return runtime.position;
        }
        let perpendicular = vec2(-runtime.direction.y, runtime.direction.x);
        let seed = npc_motion_seed(&npc.id);
        let sway = ((get_time() as f32 * 4.5) + seed).sin() * 1.6;
        runtime.position + perpendicular * sway
    }
}
