use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_journal_brews_tab(
        &self,
        data: &GameData,
        art: &ArtAssets,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        let view = self.journal_brews_tab_view(data);
        crate::ui::draw_journal_brews_tab_view(&view, art, x, y, w, h);
    }
}
