use crate::content::ui_format;

pub(super) fn best_record(best_quality_band: &str) -> String {
    ui_format("inventory_best", &[("band", best_quality_band)])
}

pub(super) fn quest_reference(count: usize) -> String {
    ui_format("inventory_ref_quest", &[("count", &count.to_string())])
}

pub(super) fn recipe_reference(count: usize) -> String {
    ui_format("inventory_ref_recipe", &[("count", &count.to_string())])
}

pub(super) fn reserved_reference(count: u32) -> String {
    ui_format("inventory_ref_reserved", &[("count", &count.to_string())])
}

pub(super) fn safe_reference() -> String {
    ui_format("inventory_ref_safe", &[])
}

pub(super) fn reference_summary(parts: &[String]) -> String {
    let reference_separator = ui_format("inventory_reference_separator", &[]);
    parts
        .iter()
        .map(|part| part.as_str())
        .collect::<Vec<_>>()
        .join(&reference_separator)
}
