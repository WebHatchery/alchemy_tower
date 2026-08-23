use super::GameplayState;

impl GameplayState {
    pub(super) fn draw_ending_overlay(&self) {
        let view = self.ending_overlay_view();
        let label = if self.ui.ending_page + 1 < self.epilogue_page_count() {
            "NEXT"
        } else {
            "FINISH"
        };
        crate::ui::draw_ending_overlay_view(&view);
        crate::ui::draw_overlay_primary_action(label);
    }
}
