use super::quality_band_rank;
use super::GameplayState;
use crate::data::{BottleBatchEntry, CraftedItemProfileEntry, GameData, QuestDefinition};

fn masterwork_request(data: &GameData) -> QuestDefinition {
    let mut quest = data
        .quests
        .iter()
        .find(|quest| quest.required_item_id == "healing_draught")
        .expect("some request should want a healing draught")
        .clone();
    quest.minimum_quality_band = "Masterwork".to_owned();
    quest.required_amount = 1;
    quest.required_trait = String::new();
    quest.required_traits = Vec::new();
    quest
}

fn bottle(band: &str, score: u32, count: u32) -> BottleBatchEntry {
    BottleBatchEntry {
        item_id: "healing_draught".to_owned(),
        quality_score: score,
        quality_band: band.to_owned(),
        traits: Vec::new(),
        count,
    }
}

/// The bug this whole module exists for: the gate read
/// `crafted_item_profiles`, which is a best-ever record, so one Masterwork
/// brew satisfied every later Masterwork request permanently — including
/// ones handed a Crude bottle brewed afterwards.
#[test]
fn a_masterwork_record_does_not_qualify_a_crude_bottle() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let quest = masterwork_request(&data);

    // The player has brewed a Masterwork at some point in the past...
    state.progression.crafted_item_profiles.insert(
        "healing_draught".to_owned(),
        CraftedItemProfileEntry {
            item_id: "healing_draught".to_owned(),
            best_quality_score: 95,
            best_quality_band: "Masterwork".to_owned(),
            inherited_traits: Vec::new(),
            effect_kinds: vec!["restore".to_owned()],
        },
    );
    // ...but what is on the shelf right now is Crude.
    state.inventory.insert("healing_draught".to_owned(), 1);
    state
        .progression
        .bottle_stock
        .insert("healing_draught".to_owned(), vec![bottle("Crude", 10, 1)]);

    assert_eq!(state.qualifying_bottle_count(&data, &quest), 0);
    assert!(!state.quest_requirements_met(&data, &quest));
}

/// Handing over the worst bottle that still meets the request is the
/// difference between a quality system and a tax on brewing well.
#[test]
fn delivering_spends_the_worst_bottle_that_still_qualifies() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut quest = masterwork_request(&data);
    quest.minimum_quality_band = "Fine".to_owned();

    state.inventory.insert("healing_draught".to_owned(), 3);
    state.progression.bottle_stock.insert(
        "healing_draught".to_owned(),
        vec![
            bottle("Crude", 10, 1),
            bottle("Fine", 55, 1),
            bottle("Masterwork", 95, 1),
        ],
    );

    assert_eq!(state.qualifying_bottle_count(&data, &quest), 2);
    state.spend_bottles_for_quest(&data, &quest, 1);
    *state.inventory.get_mut("healing_draught").expect("held") -= 1;

    let left = &state.progression.bottle_stock["healing_draught"];
    assert!(
        left.iter().any(|batch| batch.quality_band == "Masterwork"),
        "the best bottle should still be on the shelf"
    );
    assert!(
        !left.iter().any(|batch| batch.quality_band == "Fine"),
        "the Fine one was the worst that qualified and should have gone"
    );
}

/// Bottles leave the shelf a dozen ways that know nothing about batches —
/// sold, drunk, imbued, fed to a planter. If a stale batch outlived the
/// bottle it described, the next plain bottle bought to replace it would
/// inherit that grade and the original bug would be back by another route.
#[test]
fn a_batch_cannot_outlive_the_bottle_it_described() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let quest = masterwork_request(&data);

    state.inventory.insert("healing_draught".to_owned(), 1);
    state.progression.bottle_stock.insert(
        "healing_draught".to_owned(),
        vec![bottle("Masterwork", 95, 1)],
    );
    assert_eq!(state.qualifying_bottle_count(&data, &quest), 1);

    // Sold or drunk — through `take_from_inventory`, which every removal
    // path now uses. Then a plain one is bought to replace it, leaving the
    // count exactly where it started: the case a lazy read-time trim cannot
    // see, because nothing about the totals looks wrong afterwards.
    state.take_from_inventory("healing_draught", 1);
    assert!(state.progression.bottle_stock.is_empty());
    state.inventory.insert("healing_draught".to_owned(), 1);

    assert_eq!(
        state.qualifying_bottle_count(&data, &quest),
        0,
        "a shop bottle inherited the grade of one that was already gone"
    );
}

/// A counter used to pay the same for a Masterwork and a Crude brew of the
/// same recipe, so the entire brewing-well half of the game was worth
/// nothing to anyone but a quest giver.
#[test]
fn a_better_bottle_fetches_a_better_price() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    state.inventory.insert("healing_draught".to_owned(), 1);
    state
        .progression
        .bottle_stock
        .insert("healing_draught".to_owned(), vec![bottle("Crude", 10, 1)]);
    let crude = state.sell_price(&data, "healing_draught");

    state.progression.bottle_stock.insert(
        "healing_draught".to_owned(),
        vec![bottle("Masterwork", 95, 1)],
    );
    let masterwork = state.sell_price(&data, "healing_draught");

    assert!(
        masterwork > crude,
        "a masterwork fetched {masterwork} against a crude bottle's {crude}"
    );
}

/// Selling parts with the worst bottle held — the same order
/// `reconcile_bottle_stock` trims in — so clearing shelf space can never
/// cost the player their best work by accident, and the price says so.
#[test]
fn selling_prices_the_bottle_it_would_actually_part_with() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    state.inventory.insert("healing_draught".to_owned(), 2);
    state.progression.bottle_stock.insert(
        "healing_draught".to_owned(),
        vec![bottle("Crude", 10, 1), bottle("Masterwork", 95, 1)],
    );
    let mixed = state.sell_price(&data, "healing_draught");

    state.inventory.insert("healing_draught".to_owned(), 1);
    state
        .progression
        .bottle_stock
        .insert("healing_draught".to_owned(), vec![bottle("Crude", 10, 1)]);
    assert_eq!(
        mixed,
        state.sell_price(&data, "healing_draught"),
        "holding a masterwork should not raise the price of the crude one being sold"
    );
}

/// Beating the grade a request asked for pays, and is noticed. Without
/// this the rational play is always to hand over the worst bottle that
/// clears the bar, which makes the quality system a bar rather than a
/// reason to brew well.
#[test]
fn exceeding_a_request_pays_more_and_is_remembered() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let mut quest = masterwork_request(&data);
    quest.minimum_quality_band = "Serviceable".to_owned();
    quest.reward_coins = 100;

    let asked = quality_band_rank("Serviceable");
    assert_eq!(state.quality_bonus_coins(&quest, asked), 0);
    assert!(state.quality_bonus_coins(&quest, 4) > 0);
    assert!(!state.delivery_was_exceptional(&quest, asked));
    assert!(state.delivery_was_exceptional(&quest, 4));

    // A request with no stated bar has nothing to beat.
    quest.minimum_quality_band = String::new();
    assert_eq!(state.quality_bonus_coins(&quest, 4), 0);
    assert!(!state.delivery_was_exceptional(&quest, 4));
}

/// Coin and standing have paid for good work since the quality pass and
/// nobody said anything about it. This checks the remark reaches the
/// player's screen, and — the half that matters — that it stays quiet for a
/// delivery that merely cleared the bar. Praise for everything is praise
/// for nothing.
#[test]
fn beating_the_bar_is_remarked_on_and_meeting_it_is_not() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let npc_id = "wren_physician";
    let expected = data
        .npcs
        .iter()
        .find(|npc| npc.id == npc_id)
        .map(|npc| npc.exceptional_delivery_line.clone())
        .expect("Wren should have a line for work that beat the order");

    state.remark_on_exceptional_delivery(&data, npc_id);
    let toasts = state.build_hud_toasts();
    assert_eq!(toasts.len(), 1, "nothing was said about exceptional work");
    assert_eq!(toasts[0].text, expected);

    // Nobody has a word for a bottle that simply met the specification.
    let mut plain = GameplayState::new(&data);
    plain.remark_on_exceptional_delivery(&data, "quest_board");
    assert!(
        plain.build_hud_toasts().is_empty(),
        "the board is not a person and has no opinion"
    );
}

/// Everyone who can receive a delivery needs one, or the remark is a thing
/// that happens for some townsfolk and silently does not for others. Both
/// paths matter: an arc request handed over face to face, and a board order
/// whose beneficiary the prose names.
#[test]
fn everyone_who_takes_a_delivery_has_something_to_say_about_good_work() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut receivers = std::collections::BTreeSet::new();
    for quest in &data.quests {
        if quest.giver_npc_id != "quest_board" {
            receivers.insert(quest.giver_npc_id.clone());
        }
        if !quest.rapport_npc_id.is_empty() {
            receivers.insert(quest.rapport_npc_id.clone());
        }
    }

    let silent = receivers
        .iter()
        .filter(|npc_id| {
            data.npcs
                .iter()
                .find(|npc| &&npc.id == npc_id)
                .is_none_or(|npc| npc.exceptional_delivery_line.is_empty())
        })
        .collect::<Vec<_>>();

    assert!(!receivers.is_empty(), "nobody receives deliveries at all");
    assert!(
        silent.is_empty(),
        "townsfolk who take deliveries and never remark on good work: {silent:?}"
    );
}
