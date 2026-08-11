//! Which of the player's held units were gathered as wild variants, and what
//! that is worth at the bench.
//!
//! Gathering under the right sky used to change a string. The variant was
//! computed at pickup, written into the herb journal's best-seen line, and
//! thrown away — inventory counts units and had nowhere to record that one unit
//! of quarry lichen came up sparked. So `bonus_traits`, `elements` and the two
//! synthesis bonuses on all forty variants were dead, and `quality_bonus`
//! reached the journal but never the brew.
//!
//! Stock is tracked alongside the plain count rather than replacing it: the
//! inventory still says how many you hold, and this says how many of those were
//! the good ones. Brewing spends the best variant first and folds its bonuses
//! into the ingredient it stands in for, which is enough for every downstream
//! calculation — quality, traits, elements, volatility, synthesis — to pick the
//! difference up without knowing variants exist.

use super::gameplay_reagent_bottles::pour_bottle;
use super::GameplayState;
use crate::data::{GameData, ItemCategory, ItemDefinition, WildVariantDefinition};

impl GameplayState {
    /// Record that one gathered unit of `item_id` came up as `variant_id`.
    pub(super) fn note_variant_gathered(&mut self, item_id: &str, variant_id: &str) {
        if variant_id.is_empty() {
            return;
        }
        *self
            .progression
            .variant_stock
            .entry(item_id.to_owned())
            .or_default()
            .entry(variant_id.to_owned())
            .or_insert(0) += 1;
    }

    /// The best variant currently held for this ingredient, if any. "Best" is
    /// the largest quality bonus, which is also how the herb journal ranks them.
    pub(super) fn best_held_variant<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
    ) -> Option<&'a WildVariantDefinition> {
        let held = self.progression.variant_stock.get(item_id)?;
        let item = data.item(item_id)?;
        item.wild_variants
            .iter()
            .filter(|variant| held.get(&variant.id).copied().unwrap_or_default() > 0)
            .max_by_key(|variant| variant.quality_bonus)
    }

    /// The best strain currently in the bag for this ingredient, and how many
    /// of them there are. The journal has recorded the best strain ever *seen*
    /// since variants were authored; what it could not say is whether the
    /// player is carrying one right now, which is the half that decides whether
    /// to walk back out or brew with what is in the bag.
    pub(super) fn held_variant_summary(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(String, u32)> {
        let variant = self.best_held_variant(data, item_id)?;
        let count = self
            .progression
            .variant_stock
            .get(item_id)
            .and_then(|held| held.get(&variant.id))
            .copied()
            .unwrap_or_default();
        (count > 0).then(|| (variant.name.clone(), count))
    }

    /// Spend one held unit of `variant_id`, dropping the entry when it runs out
    /// so `best_held_variant` stops offering something that is gone.
    pub(super) fn spend_variant_unit(&mut self, item_id: &str, variant_id: &str) {
        let Some(held) = self.progression.variant_stock.get_mut(item_id) else {
            return;
        };
        if let Some(count) = held.get_mut(variant_id) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                held.remove(variant_id);
            }
        }
        if held.is_empty() {
            self.progression.variant_stock.remove(item_id);
        }
    }

    /// The ingredients a brew of `selected_items` would actually be made from,
    /// with the best held variant folded into each herb and the best held bottle
    /// folded into each potion. Returned owned because neither a variant-grade
    /// reagent nor a graded bottle is any `ItemDefinition` the data file holds.
    pub(super) fn brew_ingredients(
        &self,
        data: &GameData,
        selected_items: &[String],
    ) -> Vec<ItemDefinition> {
        // One unit of each variant can only be spent once, so a brew calling for
        // two of the same herb gets the variant for the first and the plain herb
        // for the second. Bottles work the same way, best first.
        let mut remaining = self.progression.variant_stock.clone();
        let mut pour = self.reagent_bottle_pour();
        selected_items
            .iter()
            .filter_map(|item_id| {
                let item = data.item(item_id)?;
                if item.category == ItemCategory::Potion {
                    return Some(pour_bottle(item, &mut pour));
                }
                let variant = self
                    .best_held_variant(data, item_id)
                    .filter(|variant| {
                        remaining
                            .get(item_id)
                            .and_then(|held| held.get(&variant.id))
                            .copied()
                            .unwrap_or_default()
                            > 0
                    })
                    .cloned();
                let Some(variant) = variant else {
                    return Some(item.clone());
                };
                if let Some(count) = remaining
                    .get_mut(item_id)
                    .and_then(|held| held.get_mut(&variant.id))
                {
                    *count = count.saturating_sub(1);
                }
                Some(apply_variant(item, &variant))
            })
            .collect()
    }

    /// Spend the variant units a brew of `selected_items` just used up. Mirrors
    /// the choice `brew_ingredients` made, so what was paid for is what was
    /// consumed.
    pub(super) fn spend_brew_variants(&mut self, data: &GameData, selected_items: &[String]) {
        for item_id in selected_items {
            let Some(variant_id) = self
                .best_held_variant(data, item_id)
                .map(|variant| variant.id.clone())
            else {
                continue;
            };
            self.spend_variant_unit(item_id, &variant_id);
        }
    }
}

/// An ingredient as the variant makes it. Every field a variant carries lands on
/// the reagent itself, so the rest of the brewer needs no notion of variants.
fn apply_variant(item: &ItemDefinition, variant: &WildVariantDefinition) -> ItemDefinition {
    let mut adjusted = item.clone();
    adjusted.quality = (adjusted.quality + variant.quality_bonus).min(100);
    for extra in &variant.bonus_traits {
        if !adjusted.traits.iter().any(|held| held == extra) {
            adjusted.traits.push(extra.clone());
        }
    }
    adjusted.elements.add_assign(&variant.elements);
    adjusted.synthesis_weight = adjusted
        .synthesis_weight
        .saturating_add(variant.synthesis_weight_bonus);
    adjusted.synthesis_value = adjusted
        .synthesis_value
        .saturating_add(variant.synthesis_value_bonus);
    adjusted
}

#[cfg(test)]
#[path = "gameplay_variant_stock/tests.rs"]
mod tests;
