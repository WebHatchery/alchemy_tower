use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_shop_overlay(&self, data: &GameData, art: &ArtAssets) {
        if let Some(view) = self.shop_overlay_view(data) {
            crate::ui::draw_shop_overlay_view(&view, art);
        }
    }
}
