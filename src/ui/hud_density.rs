use std::sync::atomic::{AtomicBool, Ordering};

/// How much of the HUD is drawn.
///
/// The tower's frames are the most finished art in the game and the world they
/// surround is procedurally generated, which is the wrong way round: the ornate
/// panelling reads as the subject. Quiet mode is the answer that does not
/// require redrawing anything — keep what a player needs to act on and drop the
/// framing, so the valley is what fills the screen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HudDensity {
    Full,
    Quiet,
}

/// One drawable region of the HUD.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum HudPanel {
    /// Vitality can end the working day, so it survives quiet mode.
    VitalityMedallion,
    /// The journal holds the same thing in more detail.
    GoalNote,
    /// The clock decides whether ground is gatherable and when you collapse.
    TimePanel,
    /// Actionable: the bottles a keypress away.
    PotionBelt,
    /// What just happened, and the only warning before a collapse.
    StatusStrip,
    /// The banners that announce a beat, a delivery, a route reopening. They
    /// are the payoff channel, not framing, so quiet keeps them.
    EventToasts,
}

/// What each density draws. Both modes keep the same actionable information;
/// Full mode no longer makes ornamental chrome compete with the valley.
pub(crate) fn visible_panels(density: HudDensity) -> &'static [HudPanel] {
    match density {
        HudDensity::Full => &[
            HudPanel::VitalityMedallion,
            HudPanel::GoalNote,
            HudPanel::TimePanel,
            HudPanel::PotionBelt,
            HudPanel::StatusStrip,
            HudPanel::EventToasts,
        ],
        HudDensity::Quiet => &[
            HudPanel::VitalityMedallion,
            HudPanel::GoalNote,
            HudPanel::TimePanel,
            HudPanel::PotionBelt,
            HudPanel::EventToasts,
        ],
    }
}

static QUIET_HUD: AtomicBool = AtomicBool::new(false);

pub(crate) fn hud_density() -> HudDensity {
    if QUIET_HUD.load(Ordering::Relaxed) {
        HudDensity::Quiet
    } else {
        HudDensity::Full
    }
}

pub(crate) fn set_quiet_hud(quiet: bool) {
    QUIET_HUD.store(quiet, Ordering::Relaxed);
}

#[cfg(test)]
mod tests;
