use crate::data::{ElementProfile, ItemCategory, ItemDefinition};

use super::{synthesis_efficiency_bonus, total_synthesis_weight, weighted_quality_average};

fn test_item(quality: u32, synthesis_weight: u32, synthesis_value: u32) -> ItemDefinition {
    ItemDefinition {
        id: format!("item_{quality}_{synthesis_weight}_{synthesis_value}"),
        name: "Test Item".to_owned(),
        category: ItemCategory::Ingredient,
        base_value: 1,
        color: [0, 0, 0, 255],
        description: String::new(),
        quality,
        rarity: 1,
        elements: ElementProfile::default(),
        traits: Vec::new(),
        source_conditions: Vec::new(),
        wild_variants: Vec::new(),
        synthesis_weight,
        synthesis_value,
        catalyst_tags: Vec::new(),
        effects: Vec::new(),
    }
}

#[test]
fn weighted_quality_average_uses_synthesis_weight() {
    let light = test_item(20, 1, 2);
    let heavy = test_item(80, 3, 2);
    let ingredients = vec![&light, &heavy];

    assert_eq!(weighted_quality_average(&ingredients), 65);
}

#[test]
fn synthesis_efficiency_bonus_drops_for_heavier_mixes() {
    let light_a = test_item(20, 1, 3);
    let light_b = test_item(20, 1, 3);
    let heavy_a = test_item(20, 3, 3);
    let heavy_b = test_item(20, 3, 3);

    let light_mix = vec![&light_a, &light_b];
    let heavy_mix = vec![&heavy_a, &heavy_b];

    assert_eq!(
        synthesis_efficiency_bonus(&light_mix, total_synthesis_weight(&light_mix)),
        6
    );
    assert_eq!(
        synthesis_efficiency_bonus(&heavy_mix, total_synthesis_weight(&heavy_mix)),
        2
    );
}
