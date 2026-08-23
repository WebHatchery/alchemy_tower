use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::view_models::journal::JournalChromeView;

impl GameplayState {
    pub(super) fn journal_chrome_view(&self) -> JournalChromeView {
        JournalChromeView {
            title: ui_copy("overlay_journal_title"),
            current_conditions_text: ui_format(
                "overlay_current_conditions",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                ],
            ),
            tabs: self.journal_tabs(),
            footer_text: ui_copy("overlay_journal_footer").to_owned(),
        }
    }
}
