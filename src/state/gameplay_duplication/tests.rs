use super::GameplayState;
use crate::data::{BottleBatchEntry, GameData};

#[test]
fn duplication_consumes_catalyst_and_coins() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);

    state.coins = 99;
    state.inventory.insert("glow_potion".to_owned(), 1);
    state.inventory.insert("starlight_shard".to_owned(), 1);

    state.duplicate_item(&data, "glow_potion");

    assert_eq!(
        state
            .inventory
            .get("glow_potion")
            .copied()
            .unwrap_or_default(),
        2
    );
    assert_eq!(
        state
            .inventory
            .get("starlight_shard")
            .copied()
            .unwrap_or_default(),
        0
    );
    assert_eq!(state.coins, 63);
}

/// A copy must never be worth more than it cost to make.
///
/// The band multipliers were being applied to raw materials as well as to
/// brews, and a catalyst's `quality` is potency rather than craft — so
/// Tarn's `elevenyear_amber` (quality 82, paying 200%) sold for 640 against
/// a 360 duplication cost. Two more catalysts were the same shape. The
/// console cannot be a mint: whatever the fix, this is the rule.
#[test]
fn a_copy_never_sells_for_more_than_it_cost_to_make() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut mints = Vec::new();

    for item in &data.items {
        if !super::duplication_item_allowed(item) {
            continue;
        }
        state.inventory.insert(item.id.clone(), 1);
        let paid_back = state.sell_price(&data, &item.id);
        state.take_from_inventory(&item.id, 1);
        let cost = super::duplication_cost(item);
        if paid_back >= cost {
            mints.push(format!(
                "{}: copy costs {cost}, sells for {paid_back}",
                item.id
            ));
        }
    }

    assert!(
        mints.is_empty(),
        "things the console can mint coins from:
{mints:#?}"
    );
}

/// Duplication reads nothing from a catalyst's quality — the shard is spent,
/// not measured — and it used to take the *best* one held. Mira's
/// `counterkept_shard` is a friendship gift, sold nowhere and gathered
/// nowhere, and the console burned it in preference to a 24-coin shard two
/// counters sell, for exactly the same result.
#[test]
fn duplication_spends_the_shard_you_can_replace() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    let gift = data
        .item("counterkept_shard")
        .expect("Mira's gift catalyst should exist");
    let stock = data
        .item("starlight_shard")
        .expect("the buyable starlight shard should exist");
    assert!(
        gift.quality > stock.quality && gift.base_value > stock.base_value,
        "the gift has to be both better and dearer for this to mean anything"
    );

    state.inventory.insert(gift.id.clone(), 1);
    state.inventory.insert(stock.id.clone(), 1);

    assert_eq!(
        state.duplication_catalyst_item_id(&data).as_deref(),
        Some(stock.id.as_str()),
        "the console reached past the replaceable shard for the gift"
    );
}

#[test]
fn duplication_copies_the_worst_live_bottle_without_flattening_its_grade() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);
    state.coins = 999;
    state.inventory.insert("glow_potion".to_owned(), 2);
    state.inventory.insert("starlight_shard".to_owned(), 1);
    state.progression.bottle_stock.insert(
        "glow_potion".to_owned(),
        vec![
            BottleBatchEntry {
                item_id: "glow_potion".to_owned(),
                quality_score: 15,
                quality_band: "Crude".to_owned(),
                traits: vec!["faint".to_owned()],
                count: 1,
            },
            BottleBatchEntry {
                item_id: "glow_potion".to_owned(),
                quality_score: 90,
                quality_band: "Masterwork".to_owned(),
                traits: vec!["luminous".to_owned()],
                count: 1,
            },
        ],
    );

    state.duplicate_item(&data, "glow_potion");

    let batches = state.live_batches("glow_potion");
    assert_eq!(state.inventory["glow_potion"], 3);
    assert_eq!(batches.len(), 2);
    assert_eq!(batches[0].quality_score, 15);
    assert_eq!(batches[0].traits, vec!["faint"]);
    assert_eq!(batches[0].count, 2, "the Crude source was not the copy");
    assert_eq!(batches[1].quality_score, 90);
    assert_eq!(batches[1].count, 1);
}

#[test]
fn even_a_masterwork_copy_costs_more_than_it_can_be_sold_for() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);
    state.inventory.insert("glow_potion".to_owned(), 1);
    state.progression.bottle_stock.insert(
        "glow_potion".to_owned(),
        vec![BottleBatchEntry {
            item_id: "glow_potion".to_owned(),
            quality_score: 90,
            quality_band: "Masterwork".to_owned(),
            traits: vec!["luminous".to_owned()],
            count: 1,
        }],
    );

    let sale = state.sell_price(&data, "glow_potion");
    let cost = state.duplication_cost_for_item(&data, "glow_potion");
    assert!(cost > sale, "copy costs {cost}, but sells for {sale}");
}
