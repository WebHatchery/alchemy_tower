use super::GameplayState;
use crate::data::{BottleBatchEntry, CraftedItemProfileEntry};

#[test]
fn quest_text_describes_the_bottles_held_now_not_the_best_ever_made() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut quest = data
        .quests
        .iter()
        .find(|quest| quest.required_item_id == "healing_draught")
        .expect("some request should want a healing draught")
        .clone();
    quest.required_amount = 1;
    quest.minimum_quality_band = "Masterwork".to_owned();
    quest.required_trait = "restorative".to_owned();
    quest.required_traits.clear();
    quest.required_effect_kind.clear();
    quest.required_effect_kinds.clear();

    state.progression.crafted_item_profiles.insert(
        quest.required_item_id.clone(),
        CraftedItemProfileEntry {
            item_id: quest.required_item_id.clone(),
            best_quality_score: 95,
            best_quality_band: "Masterwork".to_owned(),
            inherited_traits: vec!["restorative".to_owned()],
            effect_kinds: Vec::new(),
        },
    );
    state.inventory.insert(quest.required_item_id.clone(), 1);
    state.progression.bottle_stock.insert(
        quest.required_item_id.clone(),
        vec![BottleBatchEntry {
            item_id: quest.required_item_id.clone(),
            quality_score: 10,
            quality_band: "Crude".to_owned(),
            traits: Vec::new(),
            count: 1,
        }],
    );

    let summary = state.quest_requirement_summary(&data, &quest);
    assert!(
        summary.contains("Masterwork"),
        "quality was hidden: {summary}"
    );
    assert!(
        summary.contains("restorative"),
        "traits were hidden: {summary}"
    );
    assert_ne!(summary, super::ready_requirement_summary());
}
