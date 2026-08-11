//! What a finished bottle is worth when it goes back into the pot.
//!
//! Second-order brewing — a bench that takes bottles as reagents — was blind to
//! how well its inputs were made. Every potion in the data files leaves
//! `quality` unset, so the schema default of 20 stood in for a Crude draught and
//! a Masterwork one alike, and a compound brew leaned entirely on process
//! bonuses and the catalyst to reach a band. Brewing the input well changed
//! nothing.
//!
//! The bottles already carry what they were brewed at, in `bottle_stock`. This
//! folds that into the reagent the way `gameplay_variant_stock` folds a wild
//! variant: the pot is filled with the *best* bottle held, its quality and the
//! traits it came out with land on the ingredient, and the brew spends that
//! batch rather than the worst on the shelf. Nothing downstream needs to know
//! bottles are graded.
//!
//! Elements are deliberately not folded. A batch records quality and traits
//! because that is what a brew resolves; a potion's element profile is authored
//! on the item and is the same for every bottle of it.

use std::collections::BTreeMap;

use super::GameplayState;
use crate::data::{BottleBatchEntry, GameData, ItemCategory, ItemDefinition};

/// The bottles a brew could pour, per item id, best last so a pour can take
/// them off the end.
pub(super) type BottlePour = BTreeMap<String, Vec<BottleBatchEntry>>;

impl GameplayState {
    /// The graded bottles on the shelf, ready to be poured one at a time.
    pub(super) fn reagent_bottle_pour(&self) -> BottlePour {
        self.progression
            .bottle_stock
            .keys()
            .map(|item_id| {
                let mut batches = self.live_batches(item_id);
                batches.sort_by_key(|batch| batch.quality_score);
                (item_id.clone(), batches)
            })
            .collect()
    }

    /// Spend the graded bottles a brew of `selected_items` just poured. Mirrors
    /// the choice `reagent_bottle_pour` made — best first — so the bottle that
    /// improved the brew is the one that leaves the shelf, rather than the worst
    /// one `reconcile_bottle_stock` would otherwise trim.
    pub(super) fn spend_brew_bottles(&mut self, data: &GameData, selected_items: &[String]) {
        for item_id in selected_items {
            if !is_potion(data, item_id) {
                continue;
            }
            self.reconcile_bottle_stock(item_id);
            let Some(batches) = self.progression.bottle_stock.get_mut(item_id) else {
                continue;
            };
            if let Some(best) = batches.iter_mut().max_by_key(|batch| batch.quality_score) {
                best.count = best.count.saturating_sub(1);
            }
            batches.retain(|batch| batch.count > 0);
            if batches.is_empty() {
                self.progression.bottle_stock.remove(item_id);
            }
        }
    }

    /// The quality a reagent would actually go into the pot at. Ingredients are
    /// worth what the data file says; a bottle is worth what it was brewed at.
    /// The materials list reads this, because a row promising quality 20 beside
    /// a bench that pours a Masterwork solution is a lie about the only decision
    /// the late tier asks the player to make.
    pub(super) fn reagent_quality(&self, data: &GameData, item_id: &str) -> u32 {
        let base = data
            .item(item_id)
            .map(|item| item.quality)
            .unwrap_or_default();
        if !is_potion(data, item_id) {
            // A herb gathered under the right sky goes into the pot better than
            // the data file's number, and the bench spends the best one held —
            // so this is the figure that decides the brew, and it was the only
            // one the player could not see.
            return base
                + self
                    .best_held_variant(data, item_id)
                    .map(|variant| variant.quality_bonus)
                    .unwrap_or_default();
        }
        self.live_batches(item_id)
            .iter()
            .map(|batch| batch.quality_score)
            .max()
            .unwrap_or(base)
    }
}

fn is_potion(data: &GameData, item_id: &str) -> bool {
    data.item(item_id)
        .is_some_and(|item| item.category == ItemCategory::Potion)
}

/// Take the best remaining bottle of this item out of the pour and fold it into
/// the reagent. Bottles with no batch — bought, gifted, granted — are plain
/// examples of the item and go in as authored.
pub(super) fn pour_bottle(item: &ItemDefinition, pour: &mut BottlePour) -> ItemDefinition {
    let Some(batches) = pour.get_mut(&item.id) else {
        return item.clone();
    };
    let Some(best) = batches.last_mut() else {
        return item.clone();
    };
    let mut poured = item.clone();
    poured.quality = best.quality_score;
    for inherited in &best.traits {
        if !poured.traits.iter().any(|held| held == inherited) {
            poured.traits.push(inherited.clone());
        }
    }
    best.count = best.count.saturating_sub(1);
    batches.retain(|batch| batch.count > 0);
    poured
}

#[cfg(test)]
#[path = "gameplay_reagent_bottles/tests.rs"]
mod tests;
