use super::GameplayState;
use crate::data::{GameData, ItemCategory, ItemDefinition};

#[path = "gameplay_duplication_text.rs"]
mod duplication_text;

impl GameplayState {
    pub(super) fn duplication_candidates(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(duplication_item_allowed)
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            let left_cost = self.duplication_cost_for_item(data, left);
            let right_cost = self.duplication_cost_for_item(data, right);
            left_cost
                .cmp(&right_cost)
                .then(data.item_name(left).cmp(data.item_name(right)))
        });
        items
    }

    pub(super) fn can_duplicate_item(&self, data: &GameData, item_id: &str) -> bool {
        let Some(item) = data.item(item_id) else {
            return false;
        };
        duplication_item_allowed(item)
            && self.inventory.get(item_id).copied().unwrap_or_default() > 0
            && self.coins >= self.duplication_cost_for_item(data, item_id)
            && self.duplication_catalyst_item_id(data).is_some()
    }

    pub(super) fn duplicate_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            self.runtime.status_text = duplication_text::unstable();
            return;
        };
        if !duplication_item_allowed(item) {
            self.runtime.status_text = duplication_text::resists(&item.name);
            return;
        }
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            self.runtime.status_text = duplication_text::missing_source(&item.name);
            return;
        }

        let cost = self.duplication_cost_for_item(data, item_id);
        if self.coins < cost {
            self.runtime.status_text =
                duplication_text::need_coins(&item.name, cost.saturating_sub(self.coins));
            return;
        }

        let Some(catalyst_item_id) = self.duplication_catalyst_item_id(data) else {
            self.runtime.status_text = duplication_text::need_catalyst();
            return;
        };

        self.coins = self.coins.saturating_sub(cost);
        self.take_from_inventory(&catalyst_item_id, 1);
        if item.category == ItemCategory::Potion {
            if !self.duplicate_worst_held_bottle(data, item_id) {
                self.runtime.status_text = duplication_text::missing_source(&item.name);
                return;
            }
        } else {
            *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        }
        self.note_inventory_observation(data, item_id);

        self.trigger_duplication_feedback(duplication_text::toast(&item.name));
        self.runtime.status_text =
            duplication_text::duplicated(data, &item.name, cost, &catalyst_item_id);
    }

    /// The starlight catalyst the console would spend.
    ///
    /// It used to take the *highest-quality* one held, and duplication reads
    /// nothing from a catalyst's quality — it is spent, not measured. So the
    /// console reached past a 24-coin shard you can buy at two counters for
    /// Mira's `counterkept_shard`, which is a friendship gift, sold nowhere and
    /// gathered nowhere, and burned it for exactly the same result. The
    /// planter had the same shape of bug and the same answer: take the least
    /// valuable thing that qualifies.
    pub(super) fn duplication_catalyst_item_id(&self, data: &GameData) -> Option<String> {
        self.inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .filter_map(|(item_id, _)| {
                let item = data.item(item_id)?;
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == "starlight")
                    .then_some((item_id.clone(), item.base_value))
            })
            .min_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)))
            .map(|entry| entry.0)
    }

    /// Exceptional bottles cost at least one coin more than the copy could be
    /// sold for. The catalyst remains an additional material cost, but this
    /// keeps the console from becoming a mint even when it copies Masterwork.
    pub(super) fn duplication_cost_for_item(&self, data: &GameData, item_id: &str) -> u32 {
        let Some(item) = data.item(item_id) else {
            return u32::MAX;
        };
        let base = duplication_cost(item);
        if item.category != ItemCategory::Potion {
            return base;
        }
        base.max(self.sell_price(data, item_id).saturating_add(1))
    }
}

fn duplication_item_allowed(item: &ItemDefinition) -> bool {
    matches!(
        item.category,
        ItemCategory::Ingredient | ItemCategory::Catalyst | ItemCategory::Potion
    )
}

pub(super) fn duplication_cost(item: &ItemDefinition) -> u32 {
    item.base_value + u32::from(item.rarity) * 10
}

#[cfg(test)]
#[path = "gameplay_duplication/tests.rs"]
mod tests;
