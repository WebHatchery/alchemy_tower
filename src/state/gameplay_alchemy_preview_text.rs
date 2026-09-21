use crate::alchemy::BrewResolution;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;

pub(super) fn output_line(data: &GameData, preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_output",
        &[
            ("item", data.item_name(&preview.output_item_id)),
            ("amount", &preview.output_amount.to_string()),
        ],
    )
}

pub(super) fn quality_line(preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_quality_forecast",
        &[
            ("quality", &preview.quality_score.to_string()),
            ("band", preview.quality_band),
            ("mastery", preview.mastery_stage),
        ],
    )
}

/// Instability meter, shown only once the brew actually carries risk (an
/// overcharge or volatile ingredients). A calm on-spec brew stays uncluttered.
pub(super) fn instability_line(preview: &BrewResolution<'_>) -> Option<String> {
    if preview.recipe.is_none() || preview.instability == 0 {
        return None;
    }
    let meter = ui_format(
        "overlay_alchemy_instability",
        &[("value", &preview.instability.to_string())],
    );
    if preview.destabilized {
        Some(format!(
            "{meter} {}",
            ui_copy("overlay_alchemy_instability_break")
        ))
    } else {
        Some(meter)
    }
}

pub(super) fn traits_line(preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_traits",
        &[(
            "traits",
            &if preview.inherited_traits.is_empty() {
                ui_copy("overlay_alchemy_traits_none").to_owned()
            } else {
                preview.inherited_traits.join(", ")
            },
        )],
    )
}

pub(super) fn read_line(preview: &BrewResolution<'_>, known: bool, stable_preview: bool) -> String {
    ui_format(
        "overlay_alchemy_read",
        &[(
            "text",
            ui_copy(if preview.recipe.is_none() && known {
                "overlay_alchemy_read_found"
            } else if preview.recipe.is_none() {
                "overlay_alchemy_read_fallback"
            } else if !known {
                "overlay_alchemy_read_unlogged"
            } else if stable_preview {
                "overlay_alchemy_read_stable"
            } else {
                "overlay_alchemy_read_degraded"
            }),
        )],
    )
}
