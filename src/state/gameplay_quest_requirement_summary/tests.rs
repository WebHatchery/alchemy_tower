use super::effect_requirement_summary;
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
fn multi_effect_requirement_summary_shows_band_threshold() {
    let mut quest = test_quest();
    quest.required_effect_kinds = vec!["glow".to_owned(), "restore".to_owned()];
    quest.minimum_effect_matches = 1;

    assert_eq!(
        effect_requirement_summary(&quest),
        "effects 1/2 glow, restore"
    );
}
