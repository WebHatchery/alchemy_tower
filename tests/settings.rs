//! Preference compatibility and runtime behavior.
use alchemy_tower::settings::{self, Preferences};

#[test]
fn older_preferences_keep_defaults_and_round_trip_quiet_hud() {
    let mut value: Preferences =
        serde_json::from_str(r#"{"master_volume":0.5,"quiet_hud":true}"#).unwrap();
    value.common.sfx_volume = 0.4;
    let restored: Preferences =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert!(restored.quiet_hud);
    assert!(restored.common.screen_shake);
    assert!(!restored.common.reduced_motion);
    assert!((restored.common.effective_sfx_volume() - 0.2).abs() < 0.001);
    assert!(serde_json::from_str::<Preferences>(r#"{"master_volume":"bad"}"#).is_err());
}

#[test]
fn storage_receives_bounded_values_and_failure_leaves_runtime_unchanged() {
    let mut value = Preferences::default();
    value.common.master_volume = f32::NAN;
    value.common.sfx_volume = 2.0;
    value.quiet_hud = true;
    let error = value
        .save_with(|saved| {
            assert_eq!(saved.common.master_volume, 1.0);
            assert_eq!(saved.common.sfx_volume, 1.0);
            Err("storage unavailable".into())
        })
        .unwrap_err();
    assert_eq!(error, "storage unavailable");
    assert!(!settings::current().quiet_hud);
}

#[test]
fn reduced_motion_overrides_shake_and_freezes_only_cosmetic_time() {
    let mut value = Preferences::default();
    value.common.reduced_motion = true;
    settings::apply(value);
    assert!(!macroquad_toolkit::settings::screen_shake_enabled());
    assert_eq!(settings::effect_time(12.0), 0.0);
    settings::apply(Preferences::default());
    assert_eq!(settings::effect_time(12.0), 12.0);
    assert!(macroquad_toolkit::settings::screen_shake_enabled());
}
