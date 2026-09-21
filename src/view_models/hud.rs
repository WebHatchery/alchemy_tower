pub(crate) const HOTBAR_SLOT_COUNT: usize = 8;
pub(crate) type HudColor = [f32; 4];
pub(crate) type HudPoint = [f32; 2];

pub(crate) struct HudPotionSlot {
    pub(crate) key_label: &'static str,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount: u32,
}

pub(crate) struct HudGoal {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) detail: String,
    pub(crate) action: String,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount_text: String,
}

pub(crate) struct HudFeedbackView {
    pub(crate) position: HudPoint,
    pub(crate) radius: f32,
    pub(crate) color: HudColor,
    pub(crate) sparkle_points: [HudPoint; 8],
    pub(crate) burst_scale: f32,
}

/// One event banner: what just happened, in the words the caller wrote.
pub(crate) struct HudToastView {
    pub(crate) text: String,
    pub(crate) icon_key: String,
    pub(crate) color: HudColor,
    /// Fades out over the last of its life so a banner leaves rather than
    /// vanishing mid-sentence.
    pub(crate) alpha: f32,
}

pub(crate) struct HudView {
    pub(crate) vitality_label: String,
    pub(crate) vitality_text: String,
    pub(crate) clock_text: String,
    pub(crate) season_weather_text: String,
    pub(crate) day_text: String,
    pub(crate) sleep_warning_text: Option<String>,
    pub(crate) goal_prefix: String,
    pub(crate) goal: HudGoal,
    pub(crate) status_text: String,
    pub(crate) area_label: String,
    pub(crate) area_banner_alpha: f32,
    pub(crate) truncation_suffix: String,
    pub(crate) potions: [HudPotionSlot; HOTBAR_SLOT_COUNT],
    pub(crate) feedbacks: Vec<HudFeedbackView>,
    pub(crate) toasts: Vec<HudToastView>,
}
