use super::GameplayState;
use crate::content::{ui_copy, ui_text};
use crate::view_models::archive::ArchiveChromeView;

impl GameplayState {
    pub(super) fn archive_chrome_view(&self) -> ArchiveChromeView {
        ArchiveChromeView {
            title: ui_copy("overlay_archive_title"),
            subtitle: ui_text().overlays.archive_subtitle.clone(),
            tabs: self
                .archive_tabs()
                .iter()
                .map(|tab| archive_tab_label(tab))
                .collect(),
            footer_text: archive_footer_text(self.archive_tab_id()),
        }
    }
}

fn archive_tab_label(tab: &str) -> &'static str {
    ui_copy(match tab {
        "timeline" => "overlay_archive_tab_timeline",
        "experiments" => "overlay_archive_tab_experiments",
        "mastery" => "overlay_archive_tab_mastery",
        "morphs" => "overlay_archive_tab_morphs",
        "disassembly" => "overlay_archive_tab_disassembly",
        _ => "overlay_archive_tab_duplication",
    })
}

fn archive_footer_text(tab: &str) -> String {
    ui_copy(match tab {
        "timeline" => "overlay_archive_footer_timeline",
        "disassembly" | "duplication" => "overlay_archive_footer_confirm",
        "experiments" => "overlay_archive_footer_filter",
        "mastery" | "morphs" => "overlay_archive_footer_browse",
        _ => "overlay_archive_footer_close",
    })
    .to_owned()
}
