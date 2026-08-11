use super::GameplayState;
use crate::data::{GameData, WildVariantDefinition};

impl GameplayState {
    /// The wild variant this ingredient would come up as under the current sky,
    /// if any. Gathering records the variant's *id* against the stock it just
    /// put in the bag, so the bench can spend it later — the name alone was
    /// enough while this only ever fed a journal line.
    pub(super) fn matching_wild_variant<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
    ) -> Option<&'a WildVariantDefinition> {
        data.item(item_id)?.wild_variants.iter().find(|variant| {
            variant
                .required_conditions
                .iter()
                .all(|condition| self.condition_matches(condition))
        })
    }

    pub(super) fn current_item_quality_snapshot(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(u32, String)> {
        let item = data.item(item_id)?;
        let variant = self.matching_wild_variant(data, item_id);
        let quality = item.quality + variant.map(|variant| variant.quality_bonus).unwrap_or(0);
        let variant_name = variant
            .map(|variant| variant.name.clone())
            .unwrap_or_default();
        Some((quality.min(100), variant_name))
    }

    pub(super) fn condition_matches(&self, condition: &str) -> bool {
        let condition = condition.to_ascii_lowercase();
        condition.contains(self.current_season())
            || condition.contains(self.current_weather())
            || condition.contains(self.current_time_window())
    }
}

#[cfg(test)]
#[path = "gameplay_item_conditions/tests.rs"]
mod tests;
