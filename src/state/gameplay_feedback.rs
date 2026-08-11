use super::gameplay_feedback_primitives::rgba;
use super::gameplay_runtime_types::GameSound;
use super::GameplayState;

impl GameplayState {
    pub(super) fn trigger_archive_reconstruction_feedback(
        &mut self,
        toast_text: impl Into<String>,
    ) {
        let archive_color = rgba(176, 226, 255, 255);
        self.push_event_toast_with_icon(toast_text, archive_color, "journal_note");
        self.trigger_world_feedback(self.player_feedback_position(), archive_color, true, 2.2);
        self.trigger_camera_shake(0.2, 5.2);
    }

    pub(super) fn trigger_duplication_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(216, 182, 255, 255), "best_quality");
    }

    pub(super) fn trigger_planter_mutation_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(188, 255, 220, 255), "best_quality");
    }

    pub(super) fn trigger_recipe_logged_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(176, 226, 255, 255), "recipe_logged");
    }

    pub(super) fn trigger_mastery_improved_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(255, 230, 170, 255), "best_quality");
    }

    pub(super) fn trigger_disassembly_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(214, 204, 170, 255), "recipe_logged");
    }

    pub(super) fn trigger_safe_sale_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(255, 214, 132, 255), "best_quality");
    }

    pub(super) fn trigger_quest_accepted_feedback(&mut self, toast_text: impl Into<String>) {
        let quest_color = rgba(255, 230, 170, 255);
        self.push_event_toast_with_icon(toast_text, quest_color, "quest_accepted");
        self.trigger_world_feedback(self.player_feedback_position(), quest_color, false, 1.2);
    }

    pub(super) fn trigger_quest_complete_feedback(&mut self, toast_text: impl Into<String>) {
        let quest_color = rgba(188, 255, 220, 255);
        self.push_event_toast_with_icon(toast_text, quest_color, "quest_complete");
        self.trigger_world_feedback(self.player_feedback_position(), quest_color, true, 1.8);
        self.trigger_camera_shake(0.14, 3.8);
        self.queue_sound(GameSound::WorkLanded);
    }

    pub(super) fn trigger_warp_travel_feedback(&mut self, position: [f32; 2]) {
        self.trigger_world_feedback(position, rgba(255, 245, 160, 255), false, 1.2);
    }

    pub(super) fn trigger_route_restored_feedback(
        &mut self,
        toast_text: impl Into<String>,
        position: [f32; 2],
    ) {
        let route_color = rgba(188, 255, 220, 255);
        self.push_event_toast_with_icon(toast_text, route_color, "route_restored");
        self.queue_sound(GameSound::RouteRestored);
        self.trigger_world_feedback(position, route_color, true, 2.0);
        self.trigger_camera_shake(0.18, 4.8);
    }

    pub(super) fn trigger_brew_result_feedback(
        &mut self,
        station_position: [f32; 2],
        stable_brew: bool,
        collapsed_brew: bool,
    ) {
        let brew_color = if collapsed_brew {
            rgba(196, 162, 255, 255)
        } else if stable_brew {
            rgba(188, 255, 220, 255)
        } else {
            rgba(255, 214, 132, 255)
        };
        self.trigger_world_feedback(
            station_position,
            brew_color,
            stable_brew || collapsed_brew,
            if stable_brew { 1.9 } else { 1.4 },
        );
        self.trigger_camera_shake(
            if stable_brew { 0.12 } else { 0.08 },
            if stable_brew { 3.6 } else { 2.0 },
        );
    }

    /// A townsperson's own words about work that beat what they asked for. It
    /// takes the best-quality mark rather than the delivery one: what is being
    /// remarked on is the grade, not the errand.
    pub(super) fn trigger_exceptional_delivery_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(255, 230, 170, 255), "best_quality");
    }

    pub(super) fn trigger_new_best_brew_feedback(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast_with_icon(toast_text, rgba(188, 255, 220, 255), "best_quality");
    }

    pub(super) fn trigger_greenhouse_unlock_feedback(&mut self, toast_text: impl Into<String>) {
        let unlock_color = rgba(200, 255, 200, 255);
        self.push_event_toast_with_icon(toast_text, unlock_color, "route_restored");
        self.trigger_world_feedback(self.player_feedback_position(), unlock_color, true, 2.1);
        self.trigger_camera_shake(0.2, 5.4);
    }

    pub(super) fn trigger_day_begin_feedback(&mut self) {
        self.trigger_world_feedback(
            self.player_feedback_position(),
            rgba(176, 226, 255, 255),
            true,
            1.5,
        );
        self.trigger_camera_shake(0.22, 6.0);
    }

    pub(super) fn trigger_journal_note_feedback(&mut self, toast_text: impl Into<String>) {
        let note_color = rgba(176, 226, 255, 255);
        self.push_event_toast_with_icon(toast_text, note_color, "journal_note");
        self.trigger_world_feedback(self.player_feedback_position(), note_color, true, 1.6);
        self.queue_sound(GameSound::JournalNote);
    }

    pub(super) fn queue_sound(&mut self, sound: GameSound) {
        self.runtime.pending_sounds.push(sound);
    }

    /// Hand over everything queued and leave the queue empty. Split out from
    /// the playing so the draining half can be tested — `AudioAssets` needs a
    /// device and cannot be built in a unit test.
    pub(super) fn take_pending_sounds(&mut self) -> Vec<GameSound> {
        std::mem::take(&mut self.runtime.pending_sounds)
    }

    /// Drained once a frame, where the audio assets actually live.
    pub(super) fn play_pending_sounds(&mut self, audio: &crate::audio::AudioAssets) {
        for sound in self.take_pending_sounds() {
            match sound {
                GameSound::JournalNote => audio.play_journal_note(),
                GameSound::WorkLanded => audio.play_work_landed(),
                GameSound::RouteRestored => audio.play_route_restored(),
                GameSound::CollapseHome => audio.play_collapse_home(),
            }
        }
    }
}

#[cfg(test)]
#[path = "gameplay_feedback/tests.rs"]
mod tests;
