use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_rune_overlay(&self, data: &GameData, art: &ArtAssets) {
        if let Some(view) = self.rune_overlay_view(data) {
            crate::ui::draw_rune_overlay_view(&view, art);
        }
    }
}
