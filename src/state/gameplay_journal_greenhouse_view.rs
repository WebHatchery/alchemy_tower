use super::gameplay_overlay_window::visible_window_start;
use super::gameplay_support::planter_stage_label;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, StationKind};
use crate::view_models::journal::{JournalGreenhouseBedView, JournalGreenhouseTabView};

const VISIBLE_GREENHOUSE_ROWS: usize = 6;

impl GameplayState {
    pub(super) fn journal_greenhouse_tab_view(&self, data: &GameData) -> JournalGreenhouseTabView {
        let all_beds = self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.kind == StationKind::Planter)
            .map(|station| JournalGreenhouseBedView {
                title: station.name.clone(),
                summary: self.journal_planter_summary(data, station),
                selected: false,
            })
            .collect::<Vec<_>>();
        let selected = self.ui.journal_index.min(all_beds.len().saturating_sub(1));
        let start = visible_window_start(selected, all_beds.len(), VISIBLE_GREENHOUSE_ROWS);
        let beds = all_beds
            .into_iter()
            .enumerate()
            .skip(start)
            .take(VISIBLE_GREENHOUSE_ROWS)
            .map(|(index, mut bed)| {
                bed.selected = index == selected;
                bed
            })
            .collect();

        JournalGreenhouseTabView {
            title: ui_copy("overlay_greenhouse_beds"),
            empty_text: ui_copy("overlay_greenhouse_empty").to_owned(),
            page_text: (start + VISIBLE_GREENHOUSE_ROWS < visible_station_count(data, self)).then(
                || {
                    ui_format(
                        "journal_showing_range",
                        &[
                            ("first", &(start + 1).to_string()),
                            (
                                "last",
                                &(start + VISIBLE_GREENHOUSE_ROWS)
                                    .min(visible_station_count(data, self))
                                    .to_string(),
                            ),
                            ("total", &visible_station_count(data, self).to_string()),
                        ],
                    )
                },
            ),
            beds,
        }
    }

    fn journal_planter_summary(
        &self,
        data: &GameData,
        station: &crate::data::StationDefinition,
    ) -> String {
        self.progression
            .planter_states
            .get(&station.id)
            .map(|state| {
                if state.planted_item_id.is_empty() {
                    ui_copy("overlay_greenhouse_none").to_owned()
                } else if state.ready {
                    if state.mutation_note.is_empty() {
                        ui_format(
                            "overlay_planter_ready",
                            &[("item", data.item_name(&state.planted_item_id))],
                        )
                    } else {
                        ui_format(
                            "overlay_greenhouse_ready_meta",
                            &[
                                ("item", data.item_name(&state.planted_item_id)),
                                ("mutation", &state.mutation_note),
                            ],
                        )
                    }
                } else {
                    let growth_target = station
                        .planter_harvest_days
                        .max(1)
                        .saturating_sub(state.mutation_growth_bonus_days)
                        .max(1);
                    let item = data.item_name(&state.planted_item_id);
                    let stage = planter_stage_label(state.growth_days, growth_target);
                    if state.mutation_note.is_empty() {
                        ui_format(
                            "overlay_greenhouse_growing",
                            &[("item", item), ("stage", stage)],
                        )
                    } else {
                        ui_format(
                            "overlay_greenhouse_growing_meta",
                            &[
                                ("item", item),
                                ("stage", stage),
                                ("mutation", &state.mutation_note),
                            ],
                        )
                    }
                }
            })
            .unwrap_or_else(|| ui_copy("overlay_greenhouse_none").to_owned())
    }
}

fn visible_station_count(data: &GameData, state: &GameplayState) -> usize {
    state
        .visible_stations(data)
        .into_iter()
        .filter(|station| station.kind == StationKind::Planter)
        .count()
}
