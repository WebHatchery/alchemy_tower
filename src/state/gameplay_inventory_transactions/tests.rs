use super::GameplayState;
use crate::data::{BottleBatchEntry, EffectKind, GameData};

fn stock_bottle(state: &mut GameplayState, item_id: &str, quality_score: u32, quality_band: &str) {
    state.inventory.insert(item_id.to_owned(), 1);
    state.progression.bottle_stock.insert(
        item_id.to_owned(),
        vec![BottleBatchEntry {
            item_id: item_id.to_owned(),
            quality_score,
            quality_band: quality_band.to_owned(),
            traits: Vec::new(),
            count: 1,
        }],
    );
}

fn drink_restore(data: &GameData, quality_score: u32, quality_band: &str) -> f32 {
    let mut state = GameplayState::new(data);
    state.vitality = 0.0;
    stock_bottle(&mut state, "healing_draught", quality_score, quality_band);
    state.consume_potion(data, "healing_draught");
    state.vitality
}

fn drink_timed(
    data: &GameData,
    item_id: &str,
    kind: EffectKind,
    quality_score: u32,
    quality_band: &str,
) -> f32 {
    let mut state = GameplayState::new(data);
    stock_bottle(&mut state, item_id, quality_score, quality_band);
    state.consume_potion(data, item_id);
    state
        .runtime
        .active_effects
        .iter()
        .find(|effect| effect.kind == kind)
        .map(|effect| effect.remaining_seconds)
        .expect("drinking should start the authored timed effect")
}

#[test]
fn a_better_healing_bottle_restores_more_of_the_day() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let crude = drink_restore(&data, 10, "Crude");
    let masterwork = drink_restore(&data, 95, "Masterwork");

    assert!((crude - 15.0).abs() < f32::EPSILON);
    assert!((masterwork - 30.0).abs() < f32::EPSILON);
    assert!(masterwork > crude);
}

#[test]
fn a_better_positive_brew_lasts_longer_without_lengthening_a_misfire() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let crude_glow = drink_timed(&data, "glow_potion", EffectKind::Glow, 10, "Crude");
    let masterwork_glow = drink_timed(&data, "glow_potion", EffectKind::Glow, 95, "Masterwork");
    assert!((crude_glow - 67.5).abs() < f32::EPSILON);
    assert!((masterwork_glow - 135.0).abs() < f32::EPSILON);
    assert!(masterwork_glow > crude_glow);

    let crude_misfire = drink_timed(&data, "murky_concoction", EffectKind::Misfire, 10, "Crude");
    let masterwork_misfire = drink_timed(
        &data,
        "murky_concoction",
        EffectKind::Misfire,
        95,
        "Masterwork",
    );
    assert!((crude_misfire - masterwork_misfire).abs() < f32::EPSILON);
}
