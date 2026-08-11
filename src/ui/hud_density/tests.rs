use super::{visible_panels, HudDensity, HudPanel};

/// Quiet mode is for seeing the world, so it has to actually remove
/// something — and it has to keep the things that are not decoration.
#[test]
fn quiet_drops_the_framing_and_keeps_what_you_act_on() {
    let full = visible_panels(HudDensity::Full);
    let quiet = visible_panels(HudDensity::Quiet);

    assert!(
        quiet.len() < full.len(),
        "quiet mode draws as much as the full HUD, so it is not quiet"
    );
    for panel in quiet {
        assert!(
            full.contains(panel),
            "{panel:?} appears only in quiet mode, which cannot be right"
        );
    }

    // Vitality can end the day and the clock decides whether the ground is
    // even gatherable. Losing either to a display preference would make the
    // option a trap rather than a view.
    for required in [
        HudPanel::VitalityMedallion,
        HudPanel::TimePanel,
        HudPanel::StatusStrip,
        HudPanel::PotionBelt,
        HudPanel::EventToasts,
    ] {
        assert!(
            quiet.contains(&required),
            "quiet mode hides {required:?}, which the player needs to act"
        );
    }

    // And the framing is what should go.
    for ornament in [HudPanel::TitleBanner, HudPanel::MinimapFrame] {
        assert!(
            !quiet.contains(&ornament),
            "quiet mode still draws {ornament:?}"
        );
    }
}

/// The full HUD is the default: a display preference should never change
/// what a new player sees before they have opted into anything.
#[test]
fn the_full_hud_is_what_you_get_without_asking() {
    super::set_quiet_hud(false);
    assert_eq!(super::hud_density(), HudDensity::Full);
    super::set_quiet_hud(true);
    assert_eq!(super::hud_density(), HudDensity::Quiet);
    super::set_quiet_hud(false);
}
