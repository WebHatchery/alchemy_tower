use super::trait_requirement_met;
use crate::data::QuestDefinition;

fn test_quest() -> QuestDefinition {
    QuestDefinition {
        id: "quest".to_owned(),
        title: "Quest".to_owned(),
        description: String::new(),
        required_item_id: "item".to_owned(),
        required_amount: 1,
        reward_coins: 1,
        giver_npc_id: "npc".to_owned(),
        minimum_quality_band: String::new(),
        required_trait: String::new(),
        required_traits: Vec::new(),
        minimum_trait_matches: 0,
        required_effect_kind: String::new(),
        required_effect_kinds: Vec::new(),
        minimum_effect_matches: 0,
        prerequisite_quests: Vec::new(),
        required_unlocked_warp: String::new(),
        minimum_total_brews: 0,
        required_mastered_recipe: String::new(),
        required_journal_milestone: String::new(),
        coin_cost: 0,
        rapport_npc_id: String::new(),
        required_rapport_npc_id: String::new(),
        required_rapport: 0,
        completion_milestones: Vec::new(),
        giver_intro_line: String::new(),
        giver_active_line: String::new(),
        repeatable: false,
        repeat_cooldown_days: 0,
    }
}

#[test]
fn legacy_single_trait_requirement_still_works() {
    let mut quest = test_quest();
    quest.required_trait = "restorative".to_owned();

    assert!(trait_requirement_met(&quest, &["restorative".to_owned()]));
    assert!(!trait_requirement_met(&quest, &["luminous".to_owned()]));
}

#[test]
fn multi_trait_requirement_supports_thresholds() {
    let mut quest = test_quest();
    quest.required_traits = vec![
        "restorative".to_owned(),
        "calm".to_owned(),
        "luminous".to_owned(),
    ];
    quest.minimum_trait_matches = 2;

    assert!(trait_requirement_met(
        &quest,
        &["restorative".to_owned(), "calm".to_owned()]
    ));
    assert!(!trait_requirement_met(&quest, &["restorative".to_owned()]));
}
