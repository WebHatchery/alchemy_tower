use super::GameplayState;
use crate::data::GameData;

#[test]
fn the_belt_offers_potions_dearest_first_and_nothing_else() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);
    state.inventory.insert("healing_draught".to_owned(), 1);
    state.inventory.insert("glow_potion".to_owned(), 2);
    state.inventory.insert("sunleaf".to_owned(), 5);

    let potions = state.quick_potions(&data);

    assert_eq!(
        potions,
        vec!["glow_potion".to_owned(), "healing_draught".to_owned()]
    );
}

/// A counter must never pay more for a thing than it charges for one.
///
/// The apothecary sold a starlight shard for 28 and bought it back for 33:
/// five coins a click, unbounded, no travel and no cost but the keypress.
/// The shard's authored quality of 62 puts it in the Excellent band, which
/// pays 140% — so the value multipliers the sell-price work introduced
/// quietly turned one hand-authored price into a faucet, at exactly the
/// moment the commissions gave coins somewhere to go.
///
/// Prices stay hand-authored; this only asks that each one clears what the
/// counter would pay back.
#[test]
fn no_counter_pays_more_for_a_thing_than_it_charges() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut faucets = Vec::new();

    for station in &data.stations {
        for stocked in &station.stock {
            // A bought unit carries no batch, so it sells for what the item
            // itself is worth — which is exactly what a buy-and-sell loop
            // would be trading.
            state.inventory.insert(stocked.item_id.clone(), 1);
            let paid_back = state.sell_price(&data, &stocked.item_id);
            state.take_from_inventory(&stocked.item_id, 1);
            if paid_back >= stocked.price {
                faucets.push(format!(
                    "{}: {} costs {} and sells back for {paid_back}",
                    station.id, stocked.item_id, stocked.price
                ));
            }
        }
    }

    assert!(
        faucets.is_empty(),
        "counters that pay more than they charge:
{faucets:#?}"
    );
}

#[test]
fn drinking_a_potion_spends_the_bottle_and_starts_its_effects() {
    let data = GameData::fallback();
    let mut state = GameplayState::new(&data);
    state.inventory.insert("glow_potion".to_owned(), 1);

    state.consume_potion(&data, "glow_potion");

    assert!(!state.inventory.contains_key("glow_potion"));
    assert!(!state.runtime.active_effects.is_empty());
}
