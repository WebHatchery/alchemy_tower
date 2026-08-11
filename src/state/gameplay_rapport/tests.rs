use super::GameplayState;

#[test]
fn friendship_gift_granted_once_at_friend_tier() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let npc = data
        .npc("mira_apothecary")
        .expect("mira should exist")
        .clone();
    assert!(!npc.friendship_line.is_empty());
    assert!(npc.friendship_reward_coins > 0);

    // Below the friend tier: no gift.
    assert!(!state.try_grant_friendship_gift(&data, &npc));

    // At the friend tier: gift is handed over exactly once.
    state
        .progression
        .relationships
        .insert(npc.id.clone(), data.config.balance.rapport.friend);
    let coins_before = state.coins;
    assert!(state.try_grant_friendship_gift(&data, &npc));
    assert_eq!(state.coins, coins_before + npc.friendship_reward_coins);
    assert!(state.has_reached_friendship(&npc.id));
    assert_eq!(
        state
            .inventory
            .get(&npc.friendship_reward_item_id)
            .copied()
            .unwrap_or_default(),
        npc.friendship_reward_amount
    );

    // Not repeatable.
    assert!(!state.try_grant_friendship_gift(&data, &npc));
}

#[test]
fn rapport_tiers_track_standing() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let npc = "mira_apothecary";
    assert_eq!(state.rapport_tier_label(&data, npc, 0), "Stranger");
    assert_eq!(state.rapport_tier_label(&data, npc, 1), "Acquaintance");
    assert_eq!(
        state.rapport_tier_label(&data, npc, data.config.balance.rapport.friend),
        "Friend"
    );
    assert_eq!(state.rapport_tier_label(&data, npc, 6), "Confidant");
}

/// The friend tier arrives at rapport 3, which a three-beat arc passes
/// halfway through its second request. Without a second payoff the
/// relationship track finishes long before the relationship does.
#[test]
fn the_parting_gift_waits_for_the_whole_arc() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let npc = data
        .npc("rowan_herbalist")
        .expect("rowan should exist")
        .clone();
    assert!(
        !npc.trusted_line.is_empty(),
        "rowan should have a parting gift"
    );
    let chain = npc.quest_chain().to_vec();
    assert!(chain.len() >= 3);

    // Every beat but the last: still nothing.
    for quest_id in chain.iter().take(chain.len() - 1) {
        state.progression.completed_quests.insert(quest_id.clone());
        assert!(
            !state.try_grant_trusted_gift(&data, &npc),
            "the parting gift arrived while {quest_id} was still the last thing done"
        );
    }

    state
        .progression
        .completed_quests
        .insert(chain.last().expect("a last beat").clone());
    let coins_before = state.coins;
    assert!(state.try_grant_trusted_gift(&data, &npc));
    assert_eq!(state.coins, coins_before + npc.trusted_reward_coins);
    assert_eq!(
        state
            .inventory
            .get(&npc.trusted_reward_item_id)
            .copied()
            .unwrap_or_default(),
        npc.trusted_reward_amount
    );
    assert!(state.has_reached_trust(&npc.id));

    // Once only.
    assert!(!state.try_grant_trusted_gift(&data, &npc));
}

/// Every townsperson who can be befriended should also have somewhere for
/// that to end, and the gift should be something their own arc produced.
#[test]
fn everyone_with_a_friendship_has_a_parting_gift_too() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut missing = Vec::new();
    for npc in &data.npcs {
        if npc.friendship_line.is_empty() {
            continue;
        }
        if npc.trusted_line.is_empty() {
            missing.push(format!("{} befriends but never parts", npc.id));
        }
        if !npc.trusted_reward_item_id.is_empty()
            && data.item(&npc.trusted_reward_item_id).is_none()
        {
            missing.push(format!(
                "{} gives {}, which is not an item",
                npc.id, npc.trusted_reward_item_id
            ));
        }
    }
    assert!(
        missing.is_empty(),
        "incomplete rapport arcs:
{missing:#?}"
    );
}

/// Board orders now pay rapport, so the number alone can be carried to the
/// top of the ladder by supply runs. The top tier is supposed to mean the
/// player saw everything this person asked for through, so it wants the arc
/// finished as well — a reliable supplier is a confidant, not kin.
#[test]
fn the_top_tier_is_only_reachable_by_finishing_an_arc() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let npc = data
        .npc("rowan_herbalist")
        .expect("rowan should exist")
        .clone();

    // Accepting and completing three beats is +1 and +2 apiece.
    assert_eq!(
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.kin - 1),
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.confidant)
    );
    // The number on its own does not buy the top tier.
    assert_eq!(
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.kin),
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.confidant),
        "supply runs alone should not make somebody kin"
    );

    // Seeing the arc through does.
    for quest_id in npc.quest_chain() {
        state.progression.completed_quests.insert(quest_id.clone());
    }
    assert!(state.try_grant_trusted_gift(&data, &npc));
    assert_ne!(
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.kin),
        state.rapport_tier_label(&data, &npc.id, data.config.balance.rapport.confidant)
    );
}
