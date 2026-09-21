use super::*;
use crate::data::{ElementProfile, ItemCategory, ItemDefinition};

fn recipe(required_heat: i32, required_stirs: u32) -> RecipeDefinition {
    RecipeDefinition {
        id: "test".to_owned(),
        name: "Test".to_owned(),
        station_id: "entry_cauldron".to_owned(),
        ingredients: Vec::new(),
        output_item_id: "out".to_owned(),
        output_amount: 1,
        description: String::new(),
        required_heat,
        required_stirs,
        unstable_output_item_id: "murky_concoction".to_owned(),
        discovery_milestones: Vec::new(),
        lore_note: String::new(),
        minimum_quality: 0,
        preferred_traits: Vec::new(),
        guaranteed_traits: Vec::new(),
        minimum_elements: ElementProfile::default(),
        catalyst_tag: String::new(),
        catalyst_quality_bonus: 0,
        required_timing: String::new(),
        required_sequence: Vec::new(),
        morph_targets: Vec::new(),
        starter_known: false,
    }
}

fn item(traits: &[&str]) -> ItemDefinition {
    ItemDefinition {
        id: "item".to_owned(),
        name: "Item".to_owned(),
        category: ItemCategory::Ingredient,
        base_value: 1,
        color: [0, 0, 0, 255],
        description: String::new(),
        quality: 30,
        rarity: 1,
        elements: ElementProfile::default(),
        traits: traits.iter().map(|t| (*t).to_owned()).collect(),
        source_conditions: Vec::new(),
        wild_variants: Vec::new(),
        synthesis_weight: 1,
        synthesis_value: 1,
        catalyst_tags: Vec::new(),
        effects: Vec::new(),
    }
}

#[test]
fn on_spec_brew_is_calm() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let tuning = &data.config.balance.alchemy.volatility;
    let mastery_threshold = data
        .config
        .balance
        .alchemy
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default();
    let recipe = recipe(2, 2);
    let calm = item(&["pure"]);
    let ingredients = vec![&calm];
    let instability = brew_instability(
        &recipe,
        &ingredients,
        None,
        false,
        2,
        2,
        0,
        mastery_threshold,
        tuning,
    );
    assert_eq!(instability, 0);
    assert!(!is_destabilized(instability, tuning.instability_limit));
    assert_eq!(overcharge_potency(&recipe, 2, 2, tuning), 0);
}

#[test]
fn overcharge_trades_potency_for_instability() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let tuning = &data.config.balance.alchemy.volatility;
    let mastery_threshold = data
        .config
        .balance
        .alchemy
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default();
    let recipe = recipe(2, 2);
    let plain = item(&[]);
    let ingredients = vec![&plain];
    // Four stirs over the requirement.
    let potency = overcharge_potency(&recipe, 2, 6, tuning);
    let instability = brew_instability(
        &recipe,
        &ingredients,
        None,
        false,
        2,
        6,
        0,
        mastery_threshold,
        tuning,
    );
    assert_eq!(potency, 4 * tuning.overcharge_stir_potency);
    assert_eq!(instability, 4 * tuning.stir_instability);
}

#[test]
fn pushing_too_far_destabilizes() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let tuning = &data.config.balance.alchemy.volatility;
    let mastery_threshold = data
        .config
        .balance
        .alchemy
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default();
    let recipe = recipe(2, 2);
    let plain = item(&[]);
    let ingredients = vec![&plain];
    // Ten stirs over the requirement clears the collapse threshold.
    let instability = brew_instability(
        &recipe,
        &ingredients,
        None,
        false,
        2,
        12,
        0,
        mastery_threshold,
        tuning,
    );
    assert!(is_destabilized(instability, tuning.instability_limit));
}

#[test]
fn stabilizers_widen_the_overcharge_window() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let tuning = &data.config.balance.alchemy.volatility;
    let mastery_threshold = data
        .config
        .balance
        .alchemy
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default();
    let recipe = recipe(2, 2);
    let volatile = item(&["volatile"]);
    let pure = item(&["pure", "calm"]);

    let volatile_only = vec![&volatile];
    let steadied = vec![&volatile, &pure];

    let risky = brew_instability(
        &recipe,
        &volatile_only,
        None,
        false,
        2,
        8,
        0,
        mastery_threshold,
        tuning,
    );
    let steadier = brew_instability(
        &recipe,
        &steadied,
        None,
        false,
        2,
        8,
        0,
        mastery_threshold,
        tuning,
    );
    assert!(steadier < risky);
}

#[test]
fn mastery_and_catalyst_settle_the_brew() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let tuning = &data.config.balance.alchemy.volatility;
    let mastery_threshold = data
        .config
        .balance
        .alchemy
        .mastery_stages
        .last()
        .map(|stage| stage.minimum_brews)
        .unwrap_or_default();
    let recipe = recipe(2, 2);
    let volatile = item(&["volatile"]);
    let ingredients = vec![&volatile];
    let catalyst = item(&["pure"]);

    let raw = brew_instability(
        &recipe,
        &ingredients,
        None,
        false,
        2,
        6,
        0,
        mastery_threshold,
        tuning,
    );
    let settled = brew_instability(
        &recipe,
        &ingredients,
        Some(&catalyst),
        true,
        2,
        6,
        6,
        mastery_threshold,
        tuning,
    );
    assert!(settled < raw);
}
