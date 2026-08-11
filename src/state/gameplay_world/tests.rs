use super::GameplayState;

/// Season, weather and hour are three independent gates and a daily roll is
/// a fourth. It is easy to author a node whose conditions can never all be
/// true at once, and such a node is invisible content: it costs art, data
/// and a route entry, and never appears. Walk a full cycle of every gate and
/// insist each node turns up at least once.
#[test]
fn every_gather_node_can_actually_spawn() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for quest in &data.quests {
        state.progression.completed_quests.insert(quest.id.clone());
    }
    // Ground can now also wait on a journal beat — treating a slumped root
    // wall opens the bank above it — so a run that has finished everything
    // has recorded those too.
    for area in &data.areas {
        for node in &area.gather_nodes {
            if !node.required_journal_milestone.is_empty() {
                state.push_journal_milestone(&node.required_journal_milestone, "", "");
            }
        }
    }

    // Season advances every 5 days and weather every 4, so a 20-day sweep
    // covers every pairing; the extra days vary the per-node daily roll.
    let day_length = state.world.day_length_seconds;
    let never_spawns = data
        .areas
        .iter()
        .flat_map(|area| area.gather_nodes.iter().map(move |node| (area, node)))
        .filter(|(area, node)| {
            state.world.current_area_id = area.id.clone();
            for day in 0..60u32 {
                for fraction in [0.3, 0.5, 0.75, 0.95] {
                    state.world.day_index = day;
                    state.world.day_clock_seconds = day_length * fraction;
                    state.refresh_available_nodes(&data);
                    if state.world.available_nodes.contains(&node.id) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(area, node)| format!("{} in {}", node.id, area.id))
        .collect::<Vec<_>>();

    assert!(
        never_spawns.is_empty(),
        "gather nodes that can never appear:\n{never_spawns:#?}"
    );
}

/// `every_gather_node_can_actually_spawn` only asks whether a node appears
/// at all. That is a low bar: a node can clear it and still make the player
/// wait weeks. This measures the two things they actually feel — how long
/// until it first turns up, and how often it turns up after that.
///
/// The per-node daily roll is `day * 31` mixed with the node id, which is a
/// linear sequence rather than noise, and season and weather cycle every 20
/// days. Those periods interact: the lake's stillwater pearl qualifies on
/// five days of the first cycle and its roll is too high on every one of
/// them, so the only source of a catalyst three morphs need is absent until
/// day 21. That is the behaviour this pins — the floors are precedent, set
/// just outside the worst node that already ships, not an ambition.
#[test]
fn every_gather_node_turns_up_soon_enough_and_often_enough() {
    const SWEEP_DAYS: u32 = 100;
    const LATEST_FIRST_APPEARANCE: u32 = 28;
    const MINIMUM_DAYS_IN_SWEEP: usize = 6;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for quest in &data.quests {
        state.progression.completed_quests.insert(quest.id.clone());
    }
    // Ground can now also wait on a journal beat — treating a slumped root
    // wall opens the bank above it — so a run that has finished everything
    // has recorded those too.
    for area in &data.areas {
        for node in &area.gather_nodes {
            if !node.required_journal_milestone.is_empty() {
                state.push_journal_milestone(&node.required_journal_milestone, "", "");
            }
        }
    }

    let day_length = state.world.day_length_seconds;
    let mut complaints = Vec::new();
    for area in &data.areas {
        for node in &area.gather_nodes {
            state.world.current_area_id = area.id.clone();
            let mut first = None;
            let mut seen = 0usize;
            for day in 0..SWEEP_DAYS {
                // Any hour of that day counts: the player can wait for dusk.
                let available = [0.15, 0.35, 0.6, 0.85, 0.97].iter().any(|fraction| {
                    state.world.day_index = day;
                    state.world.day_clock_seconds = day_length * fraction;
                    state.refresh_available_nodes(&data);
                    state.world.available_nodes.contains(&node.id)
                });
                if available {
                    seen += 1;
                    first.get_or_insert(day);
                }
            }

            match first {
                None => complaints.push(format!("{} in {}: never", node.id, area.id)),
                Some(day) if day > LATEST_FIRST_APPEARANCE => complaints.push(format!(
                    "{} in {}: first appears on day {day}",
                    node.id, area.id
                )),
                _ => {}
            }
            if seen < MINIMUM_DAYS_IN_SWEEP {
                complaints.push(format!(
                    "{} in {}: only {seen} of {SWEEP_DAYS} days",
                    node.id, area.id
                ));
            }
        }
    }
    complaints.sort();

    assert!(
        complaints.is_empty(),
        "gather nodes the player cannot reasonably find:
{complaints:#?}"
    );
}

/// Seasons are deliberately unequal — winter should be leaner than spring,
/// and the charred hollow exists to give that leanness a destination rather
/// than erase it. This is a floor, not a balance target: it catches a pass
/// that starves a quarter of the year without noticing, which is exactly how
/// winter got down to 62% of spring before anyone counted.
#[test]
fn no_season_is_starved_of_gatherable_ground() {
    const SEASONS: [&str; 4] = ["spring", "summer", "autumn", "winter"];
    const FLOOR: f32 = 0.5;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let counts = SEASONS.map(|season| {
        data.areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .filter(|node| node.seasons.is_empty() || node.seasons.iter().any(|s| s == season))
            .count()
    });

    let best = *counts.iter().max().expect("four seasons");
    let worst = *counts.iter().min().expect("four seasons");
    let ratio = worst as f32 / best as f32;

    assert!(
        ratio >= FLOOR,
        "the leanest season has only {worst} nodes against {best} in the richest \
             ({ratio:.2} of it); per season: {:?}",
        SEASONS.iter().zip(counts.iter()).collect::<Vec<_>>()
    );
}

/// The same floor for weather. Wind sat at 23 nodes against clear's 40
/// before the plains were given a reason to be walked in it — a whole
/// condition a player could ignore for a whole game. Leanness is fine; an
/// axis nothing uses is not.
#[test]
fn no_weather_is_starved_of_gatherable_ground() {
    const WEATHERS: [&str; 4] = ["clear", "mist", "rain", "windy"];
    const FLOOR: f32 = 0.5;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let counts = WEATHERS.map(|weather| {
        data.areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .filter(|node| node.weathers.is_empty() || node.weathers.iter().any(|w| w == weather))
            .count()
    });

    let best = *counts.iter().max().expect("four weathers");
    let worst = *counts.iter().min().expect("four weathers");
    let ratio = worst as f32 / best as f32;

    assert!(
        ratio >= FLOOR,
        "the leanest weather has only {worst} nodes against {best} in the richest \
             ({ratio:.2} of it); per weather: {:?}",
        WEATHERS.iter().zip(counts.iter()).collect::<Vec<_>>()
    );
}
