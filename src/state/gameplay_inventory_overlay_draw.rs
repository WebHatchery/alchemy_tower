use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_inventory_overlay(&self, data: &GameData, art: &ArtAssets) {
        let view = self.inventory_overlay_view(data);
        crate::ui::draw_inventory_overlay_view(&view, art);
    }
}
