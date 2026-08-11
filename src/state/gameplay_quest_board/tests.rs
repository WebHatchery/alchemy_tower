use super::GameplayState;
use crate::data::{BottleBatchEntry, CraftedItemProfileEntry};

/// One Fine, restorative healing draught actually on the shelf. It has to be
/// a real bottle now, not merely a best-ever record: a request is checked
/// against what is being handed over.
fn stock_healing_draught(state: &mut GameplayState) {
    state.inventory.insert("healing_draught".to_owned(), 1);
    state.progression.bottle_stock.insert(
        "healing_draught".to_owned(),
        vec![BottleBatchEntry {
            item_id: "healing_draught".to_owned(),
            quality_score: 60,
            quality_band: "Fine".to_owned(),
            traits: vec!["restorative".to_owned()],
            count: 1,
        }],
    );
    state.progression.crafted_item_profiles.insert(
        "healing_draught".to_owned(),
        CraftedItemProfileEntry {
            item_id: "healing_draught".to_owned(),
            best_quality_score: 60,
            best_quality_band: "Fine".to_owned(),
            inherited_traits: vec!["restorative".to_owned()],
            effect_kinds: vec!["restore".to_owned()],
        },
    );
}

#[test]
fn repeatable_board_request_returns_after_cooldown() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let quest_id = "board_restorative_stash";
    let quest = data.quest(quest_id).expect("board quest should exist");
    assert!(quest.repeatable);

    state.progression.total_brews = 10;
    state.progression.started_quests.insert(quest_id.to_owned());
    stock_healing_draught(&mut state);

    // A ready request shows up as a deliverable board action.
    assert!(state
        .board_actions(&data)
        .iter()
        .any(|action| action.quest_id == quest_id && action.deliver));

    let coins_before = state.coins;
    state.deliver_board_quest(&data, quest_id);

    // Delivered: paid out, consumed, and NOT permanently completed. The
    // fixture bottle is Fine against a Serviceable order, so the payment
    // includes the bonus for beating the bar by a band.
    let bonus = state.quality_bonus_coins(
        quest,
        crate::state::gameplay::gameplay_support::quality_band_rank("Fine"),
    );
    assert!(bonus > 0, "a Fine bottle beats a Serviceable order");
    assert_eq!(state.coins, coins_before + quest.reward_coins + bonus);
    assert!(!state.progression.completed_quests.contains(quest_id));
    assert!(!state.progression.started_quests.contains(quest_id));
    assert_eq!(
        state.inventory.get(quest_id).copied().unwrap_or_default(),
        0
    );

    // On cooldown today, back on the board once the cooldown day arrives.
    assert!(!state
        .available_board_quests(&data)
        .contains(&quest_id.to_owned()));
    state.world.day_index += quest.repeat_cooldown_days.max(1);
    assert!(state
        .available_board_quests(&data)
        .contains(&quest_id.to_owned()));
}

#[test]
fn non_repeatable_delivery_completes_permanently() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    // star_elixir_for_ione is a one-shot NPC quest; deliver it through the
    // shared board delivery path to confirm the non-repeatable branch.
    let quest_id = "healing_for_mira";
    state.progression.started_quests.insert(quest_id.to_owned());
    stock_healing_draught(&mut state);

    state.deliver_board_quest(&data, quest_id);
    assert!(state.progression.completed_quests.contains(quest_id));
}

/// The repeatable layer used to be decoupled from the townsfolk entirely:
/// thirty-odd orders, none of which earned standing with anybody, however
/// many times they were run. Every order now names whose work it serves.
#[test]
fn a_board_delivery_earns_standing_with_whoever_it_was_for() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let quest = data
        .quest("board_restorative_stash")
        .expect("board quest should exist");
    assert_eq!(quest.rapport_npc_id, "wren_physician");

    state.progression.total_brews = 10;
    state.progression.started_quests.insert(quest.id.clone());
    stock_healing_draught(&mut state);

    let before = state.rapport_value(&quest.rapport_npc_id);
    state.deliver_board_quest(&data, &quest.id);
    assert_eq!(
        state.rapport_value(&quest.rapport_npc_id),
        before + 1,
        "the infirmary learned nothing from being supplied"
    );
}

/// A request can now wait on standing rather than on progress. Without the
/// gate being read, a confidant-only order would sit on the board from the
/// first day and the upper rapport tiers would go back to being labels.
#[test]
fn a_confidant_order_waits_until_the_standing_is_there() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let quest = data
        .quest("board_confidant_coldread_for_ione")
        .expect("the confidant order should exist");
    assert_eq!(
        quest.required_rapport,
        data.config.balance.rapport.confidant
    );

    // Everything but the standing.
    state
        .progression
        .unlocked_warps
        .insert(quest.required_unlocked_warp.clone());
    assert!(!state.quest_is_available(quest));

    state.progression.relationships.insert(
        quest.required_rapport_npc_id.clone(),
        quest.required_rapport,
    );
    assert!(state.quest_is_available(quest));
}
