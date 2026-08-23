use super::GameplayState;
use crate::data::GameData;
use crate::input::{
    hud_mouse_position_point, left_mouse_pressed, quick_potion_pressed, rect_contains_point,
};

impl GameplayState {
    pub(super) fn handle_potion_inputs(&mut self, data: &GameData) {
        let potions = self.quick_potions(data);
        for (index, item_id) in potions.iter().take(3).enumerate() {
            if quick_potion_pressed(index) || self.touch_potion_pressed(index) {
                self.consume_potion(data, item_id);
                return;
            }
        }
    }

    fn touch_potion_pressed(&self, index: usize) -> bool {
        left_mouse_pressed()
            && rect_contains_point(
                crate::ui::hud_potion_slot_rect(index),
                hud_mouse_position_point(),
            )
    }
}
