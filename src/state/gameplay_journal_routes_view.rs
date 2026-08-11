use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalHerbMemoriesView, JournalHerbMemoryView, JournalHerbRowView, JournalRouteProgressView,
    JournalRouteRowView, JournalRoutesTabView,
};

/// Route rows are one line each, with the selected route's description beneath
/// them — the same shape as the herb column beside it. Drawn as title plus full
/// paragraph, the column had room for two of seventeen.
const VISIBLE_ROUTE_ROWS: usize = 7;

/// Herb rows are one line each and the block beneath belongs to the selected
/// row. Drawn at full detail for every herb, the column had room for one.
const VISIBLE_HERB_ROWS: usize = 5;

impl GameplayState {
    pub(super) fn journal_routes_tab_view(&self, data: &GameData) -> JournalRoutesTabView {
        let route_total = data.gathering_routes.len();
        // Routes ride the same index as the herb list rather than claiming a
        // second key: walking the herbs walks the routes past them too.
        let route_selected = self.ui.journal_index.min(route_total.saturating_sub(1));
        let route_start = visible_window_start(route_selected, route_total, VISIBLE_ROUTE_ROWS);
        let locked_lines = self
            .locked_warps(data)
            .into_iter()
            .take(2)
            .map(|warp| {
                ui_format(
                    "overlay_route_locked_line",
                    &[
                        ("label", &warp.label),
                        ("requirements", &self.warp_lock_text(data, warp)),
                    ],
                )
            })
            .collect::<Vec<_>>();

        JournalRoutesTabView {
            title: ui_copy("overlay_known_routes"),
            progress_title: ui_copy("overlay_progress_routes"),
            route_rows: data
                .gathering_routes
                .iter()
                .skip(route_start)
                .take(VISIBLE_ROUTE_ROWS)
                .enumerate()
                .map(|(offset, route)| JournalRouteRowView {
                    title: route.name.clone(),
                    selected: route_start + offset == route_selected,
                })
                .collect(),
            route_range_text: (route_total > VISIBLE_ROUTE_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(route_start + 1).to_string()),
                        (
                            "last",
                            &(route_start + VISIBLE_ROUTE_ROWS)
                                .min(route_total)
                                .to_string(),
                        ),
                        ("total", &route_total.to_string()),
                    ],
                )
            }),
            route_detail: data
                .gathering_routes
                .get(route_selected)
                .map(|route| route.description.clone()),
            herb_memories: self.journal_herb_memories_view(data),
            route_progress: JournalRouteProgressView {
                all_restored_text: locked_lines
                    .is_empty()
                    .then(|| ui_copy("overlay_routes_all_restored").to_owned()),
                locked_lines,
            },
        }
    }

    fn journal_herb_memories_view(&self, data: &GameData) -> JournalHerbMemoriesView {
        let herb_memories = self.herb_memories(data);
        if herb_memories.is_empty() {
            return JournalHerbMemoriesView {
                title: ui_copy("overlay_herb_memories"),
                empty_text: ui_copy("journal_memory_no_herbs").to_owned(),
                range_text: None,
                rows: Vec::new(),
                detail: None,
            };
        }

        let total = herb_memories.len();
        let selected = self.ui.journal_index.min(total - 1);
        let start = visible_window_start(selected, total, VISIBLE_HERB_ROWS);
        let entries = herb_memories
            .into_iter()
            .map(|entry| {
                let route_id = if entry.learned {
                    &entry.learned_route_id
                } else {
                    &entry.first_seen_route_id
                };
                let route_label = data
                    .route(route_id)
                    .map(|route| route.name.as_str())
                    .unwrap_or_else(|| ui_copy("journal_memory_unknown_place"));
                let route_copy_key = if entry.learned {
                    "journal_memory_learned_at"
                } else {
                    "journal_memory_observed_at"
                };
                JournalHerbMemoryView {
                    title: data.item_name(&entry.item_id).to_owned(),
                    state_line: ui_format(
                        "journal_memory_state_line",
                        &[("state", ui_copy(self.herb_memory_state_key(&entry.item_id)))],
                    ),
                    route_line: ui_format(route_copy_key, &[("route", route_label)]),
                    summary: self.journal_herb_lead(data, &entry.item_id),
                    // Learned means the conditions are known exactly. Short of
                    // that the entry carries what the valley says about the
                    // herb, which is enough to know when to go looking and not
                    // enough to save the trip.
                    conditions: if entry.learned {
                        self.learned_gathering_conditions(data, &entry.item_id)
                    } else {
                        self.heard_gathering_conditions(data, &entry.item_id)
                    }
                    .unwrap_or_else(|| ui_copy("journal_memory_conditions_unknown").to_owned()),
                    used_in_text: self.herb_used_in_text(data, &entry.item_id),
                    best_specimen_text: (entry.best_quality > 0).then(|| {
                        ui_format(
                            "journal_memory_best_specimen",
                            &[
                                ("quality", &entry.best_quality.to_string()),
                                ("band", &entry.best_quality_band),
                            ],
                        )
                    }),
                    // What was seen once, and — the part that decides whether to
                    // walk back out — whether one is in the bag now.
                    variant_text: self
                        .held_variant_summary(data, &entry.item_id)
                        .map(|(name, count)| {
                            ui_format(
                                "journal_memory_variant_held",
                                &[("variant", &name), ("count", &count.to_string())],
                            )
                        })
                        .or_else(|| {
                            (!entry.variant_name.is_empty()).then(|| {
                                ui_format(
                                    "journal_memory_variant",
                                    &[("variant", &entry.variant_name)],
                                )
                            })
                        }),
                    note_text: (entry.learned && !entry.note.is_empty())
                        .then(|| entry.note.clone()),
                }
            })
            .collect::<Vec<_>>();

        JournalHerbMemoriesView {
            title: ui_copy("overlay_herb_memories"),
            empty_text: String::new(),
            range_text: (total > VISIBLE_HERB_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        ("last", &(start + VISIBLE_HERB_ROWS).min(total).to_string()),
                        ("total", &total.to_string()),
                    ],
                )
            }),
            rows: entries
                .iter()
                .enumerate()
                .skip(start)
                .take(VISIBLE_HERB_ROWS)
                .map(|(index, entry)| JournalHerbRowView {
                    title: entry.title.clone(),
                    state_line: entry.state_line.clone(),
                    selected: index == selected,
                })
                .collect(),
            detail: entries.into_iter().nth(selected),
        }
    }

    /// Names the brews this ingredient feeds. Only recipes the player has
    /// discovered are named; still-unknown uses are never named, so the journal
    /// teaches what a gathered herb is *for* without giving away the catalogue.
    ///
    /// They are pointed at, though. Counting them and stopping there — which is
    /// all this line did for fifty-nine of the game's sixty-two formulae — left
    /// the only route to a formula being to guess its exact reagents. The hint
    /// says where the missing half of the nearest one comes from and nothing
    /// else; see [`super::gameplay_formula_hint`].
    fn herb_used_in_text(&self, data: &GameData, item_id: &str) -> Option<String> {
        let mut known = Vec::new();
        let mut undiscovered = 0u32;
        for recipe in &data.recipes {
            if recipe
                .ingredients
                .iter()
                .any(|ingredient| ingredient.item_id == item_id)
            {
                if self.recipe_is_known(&recipe.id) {
                    known.push(recipe.name.clone());
                } else {
                    undiscovered += 1;
                }
            }
        }

        if known.is_empty() && undiscovered == 0 {
            return None;
        }
        let hint = self
            .undiscovered_formula_hint(data, item_id)
            .unwrap_or_else(|| ui_copy("journal_formula_hint_unknown").to_owned());
        if known.is_empty() {
            return Some(ui_format(
                "journal_memory_used_in_unknown",
                &[("hint", &hint)],
            ));
        }

        let recipes = known.join(", ");
        Some(if undiscovered > 0 {
            ui_format(
                "journal_memory_used_in_more",
                &[("recipes", &recipes), ("hint", &hint)],
            )
        } else {
            ui_format("journal_memory_used_in", &[("recipes", &recipes)])
        })
    }
}

#[cfg(test)]
#[path = "gameplay_journal_routes_view/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "gameplay_journal_routes_view/window_tests.rs"]
mod window_tests;
