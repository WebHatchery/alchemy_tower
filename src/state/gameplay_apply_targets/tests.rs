use super::GameplayState;
use crate::data::{ApplyTargetDefinition, BottleBatchEntry, GameData};

fn a_target(data: &GameData) -> ApplyTargetDefinition {
    data.areas
        .iter()
        .flat_map(|area| area.apply_targets.iter())
        .next()
        .cloned()
        .unwrap_or_else(|| panic!("the world should carry something a brew can be used on"))
}

/// The premise, end to end: hold nothing and the target stays as it is;
/// hold the right brew and it is treated, the bottle is gone, and the beat
/// is recorded for whatever is waiting on it.
#[test]
fn a_brew_poured_on_the_world_changes_it() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let target = a_target(&data);

    assert!(!state.target_is_treated(&target));
    assert!(
        !state.treat_target(&data, &target),
        "an empty bag should treat nothing"
    );

    // Find something that does the job and stock one.
    let bottle = data
        .items
        .iter()
        .find(|item| {
            item.category == crate::data::ItemCategory::Potion
                && item
                    .effects
                    .iter()
                    .any(|effect| effect.kind.to_string() == target.required_effect_kind)
        })
        .expect("some potion should do what the target asks");
    state.inventory.insert(bottle.id.clone(), 1);

    assert!(state.treat_target(&data, &target));
    assert!(state.target_is_treated(&target));
    assert_eq!(
        state.inventory.get(&bottle.id).copied().unwrap_or_default(),
        0,
        "treating should have spent the bottle"
    );
    for milestone in &target.completion_milestones {
        assert!(state.has_journal_milestone(&milestone.id));
    }
}

/// A target that names a grade will not take a worse bottle, or the
/// quality system stops at the bench door.
#[test]
fn a_graded_target_refuses_a_worse_bottle() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut target = a_target(&data);
    target.minimum_quality_band = "Masterwork".to_owned();

    let bottle = data
        .items
        .iter()
        .find(|item| {
            item.category == crate::data::ItemCategory::Potion
                && item
                    .effects
                    .iter()
                    .any(|effect| effect.kind.to_string() == target.required_effect_kind)
        })
        .expect("some potion should do what the target asks");
    state.inventory.insert(bottle.id.clone(), 1);

    assert!(
        state.bottle_for_target(&data, &target).is_none(),
        "a plain bottle should not satisfy a Masterwork demand"
    );
}

#[test]
fn a_graded_target_spends_the_worst_bottle_that_actually_qualifies() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let mut target = a_target(&data);
    target.minimum_quality_band = "Masterwork".to_owned();

    let bottle = data
        .items
        .iter()
        .find(|item| {
            item.category == crate::data::ItemCategory::Potion
                && item
                    .effects
                    .iter()
                    .any(|effect| effect.kind.to_string() == target.required_effect_kind)
        })
        .expect("some potion should do what the target asks");
    state.inventory.insert(bottle.id.clone(), 2);
    state.progression.bottle_stock.insert(
        bottle.id.clone(),
        vec![
            BottleBatchEntry {
                item_id: bottle.id.clone(),
                quality_score: 10,
                quality_band: "Crude".to_owned(),
                traits: Vec::new(),
                count: 1,
            },
            BottleBatchEntry {
                item_id: bottle.id.clone(),
                quality_score: 95,
                quality_band: "Masterwork".to_owned(),
                traits: Vec::new(),
                count: 1,
            },
        ],
    );

    assert_eq!(
        state.bottle_for_target(&data, &target),
        Some(bottle.id.clone())
    );
    assert!(state.treat_target(&data, &target));
    assert_eq!(state.inventory.get(&bottle.id), Some(&1));
    let remaining = &state.progression.bottle_stock[&bottle.id];
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].quality_band, "Crude");
}

#[test]
fn a_treatment_opens_its_same_area_ground_immediately() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let area = data
        .area("moonlit_forest")
        .expect("the forest should exist");
    let target = area
        .apply_targets
        .iter()
        .find(|target| target.id == "forest_startled_roost")
        .expect("the startled roost should exist")
        .clone();
    let milestone = "forest_roost_settled";
    let node_id = "forest_settled_roost_01";
    state.world.current_area_id = area.id.clone();
    state.set_clock_minutes(1320.0);

    // Find a day on which season, weather and daily roll all allow the node;
    // then remove the temporary gate so the treatment is the only change.
    state.push_journal_milestone(milestone, "", "");
    let day = (0..40)
        .find(|day| {
            state.world.day_index = *day;
            state.refresh_available_nodes(&data);
            state.world.available_nodes.contains(node_id)
        })
        .expect("the settled roost should have a valid spawn day");
    state
        .progression
        .journal_milestones
        .retain(|entry| entry.id != milestone);
    state.world.day_index = day;
    state.refresh_available_nodes(&data);
    assert!(!state.world.available_nodes.contains(node_id));

    let bottle = data
        .items
        .iter()
        .find(|item| {
            item.category == crate::data::ItemCategory::Potion
                && item
                    .effects
                    .iter()
                    .any(|effect| effect.kind.to_string() == target.required_effect_kind)
        })
        .expect("a misfire bottle should settle the roost");
    state.inventory.insert(bottle.id.clone(), 1);

    assert!(state.treat_target(&data, &target));
    assert!(
        state.world.available_nodes.contains(node_id),
        "the player had to leave and re-enter before restored ground appeared"
    );
}
