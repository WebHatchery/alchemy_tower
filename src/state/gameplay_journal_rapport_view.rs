use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{JournalRapportRowView, JournalRapportTabView};

const VISIBLE_RAPPORT_ROWS: usize = 3;

impl GameplayState {
    pub(super) fn journal_rapport_tab_view(&self, data: &GameData) -> JournalRapportTabView {
        let selected = self.ui.journal_index.min(data.npcs.len().saturating_sub(1));
        let start = visible_window_start(selected, data.npcs.len(), VISIBLE_RAPPORT_ROWS);
        JournalRapportTabView {
            title: ui_copy("overlay_town_rapport"),
            page_text: (data.npcs.len() > VISIBLE_RAPPORT_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        (
                            "last",
                            &(start + VISIBLE_RAPPORT_ROWS)
                                .min(data.npcs.len())
                                .to_string(),
                        ),
                        ("total", &data.npcs.len().to_string()),
                    ],
                )
            }),
            rows: data
                .npcs
                .iter()
                .enumerate()
                .skip(start)
                .take(VISIBLE_RAPPORT_ROWS)
                .map(|(index, npc)| {
                    let rapport = self
                        .progression
                        .relationships
                        .get(&npc.id)
                        .copied()
                        .unwrap_or_default();
                    let role = if npc.role.is_empty() {
                        ui_copy("overlay_rapport_empty")
                    } else {
                        npc.role.as_str()
                    };
                    JournalRapportRowView {
                        title: ui_format(
                            "overlay_rapport_line",
                            &[
                                ("name", &npc.name),
                                ("role", role),
                                ("value", &rapport.to_string()),
                                ("standing", self.rapport_tier_label(data, &npc.id, rapport)),
                            ],
                        ),
                        now_text: ui_format(
                            "overlay_now",
                            &[("text", &self.npc_now_hint(data, npc))],
                        ),
                        later_text: ui_format(
                            "overlay_later",
                            &[("text", &self.npc_later_hint(data, npc))],
                        ),
                        usually_text: ui_format(
                            "overlay_usually",
                            &[("text", &self.npc_usual_hint(data, npc))],
                        ),
                        selected: index == selected,
                    }
                })
                .collect(),
        }
    }
}
