use super::GameplayState;

#[test]
fn npc_arc_offers_one_beat_at_a_time() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let rowan = data
        .npc("rowan_herbalist")
        .expect("rowan should exist")
        .clone();
    let chain = rowan.quest_chain().to_vec();
    assert!(chain.len() >= 3, "rowan should carry a multi-beat arc");

    for (index, quest_id) in chain.iter().enumerate() {
        let active = state
            .npc_active_quest(&data, &rowan)
            .expect("an unfinished arc always has a live step");
        assert_eq!(
            &active.id, quest_id,
            "beat {index} should be the live step before it is completed"
        );
        assert!(
            !active.giver_intro_line.is_empty(),
            "{quest_id} should speak in Rowan's voice while on offer"
        );
        state.progression.completed_quests.insert(quest_id.clone());
    }

    assert!(
        state.npc_active_quest(&data, &rowan).is_none(),
        "a finished arc leaves nothing further to ask"
    );
    assert!(state.npc_has_been_helped(&rowan));
}

/// The town talks about what has happened, one thing per conversation, and
/// keeps moving forward. Walk Elric through his reactions: each new beat is
/// remarked on once it has been heard, and a beat that lands late does not
/// drag him back to an older subject.
#[test]
fn town_reactions_move_on_as_the_story_does() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let npc_id = "mayor_elric";

    assert_eq!(state.npc_phase1_followup_line(npc_id), None);

    state
        .progression
        .completed_quests
        .insert("healing_for_mira".to_owned());
    let after_healing = state
        .npc_phase1_followup_line(npc_id)
        .expect("elric reacts to the first delivered draught");

    // Until the player has actually heard it, that is still what he has to
    // say — a new beat does not overwrite one that was never spoken.
    state.push_journal_milestone("greenhouse_repaired", "Greenhouse", "");
    assert_eq!(state.npc_phase1_followup_line(npc_id), Some(after_healing));

    state.mark_followup_spoken(npc_id);
    let after_greenhouse = state
        .npc_phase1_followup_line(npc_id)
        .expect("elric reacts to the greenhouse");
    assert_ne!(after_healing, after_greenhouse);

    state.mark_followup_spoken(npc_id);
    state.push_journal_milestone("harvest_beds_turned", "Bed Rows", "");
    let after_harvest = state
        .npc_phase1_followup_line(npc_id)
        .expect("elric reacts to the square growing");
    assert_ne!(after_greenhouse, after_harvest);

    // An earlier beat arriving late must not drag the conversation back.
    state.mark_followup_spoken(npc_id);
    state
        .progression
        .completed_quests
        .insert("glow_for_rowan".to_owned());
    assert_eq!(state.npc_phase1_followup_line(npc_id), Some(after_harvest));
}

/// Thirty-six of the hundred and sixty authored reactions could never be
/// spoken: the selector took the highest-order earned line, and earning is
/// monotonic, so anything that came due alongside a later beat lost forever.
/// With every gate satisfied, a townsperson should work through all of their
/// lines rather than repeating their last one.
#[test]
fn every_reaction_a_townsperson_has_earned_eventually_gets_said() {
    use crate::content::narrative_text;

    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    // Satisfy everything: every quest done, every recordable beat recorded.
    for quest in &data.quests {
        state.progression.completed_quests.insert(quest.id.clone());
    }
    for reaction in &narrative_text().reactions {
        if !reaction.after_milestone.is_empty() {
            state.push_journal_milestone(&reaction.after_milestone, "", "");
        }
    }

    let npc_id = "ione_archivist";
    let authored = narrative_text()
        .reactions
        .iter()
        .filter(|reaction| reaction.npc_id == npc_id)
        .count();
    assert!(authored >= 4, "ione should have several lines");

    let mut heard = std::collections::HashSet::new();
    for _ in 0..authored {
        let line = state
            .npc_phase1_followup_line(npc_id)
            .expect("an earned line should be on offer");
        heard.insert(line);
        state.mark_followup_spoken(npc_id);
    }

    assert_eq!(
        heard.len(),
        authored,
        "ione repeated herself instead of working through her {authored} lines"
    );
}

/// `quest_ids` is the arc; `quest_id` is the older single-request field that
/// givers without an arc still use. Test the fallback on a stripped clone
/// rather than on whichever townsperson happens not to have an arc yet —
/// that list shrinks every time this loop writes one.
#[test]
fn single_quest_givers_still_resolve_without_a_chain() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let state = GameplayState::new(&data);
    let mut npc = data
        .npcs
        .iter()
        .find(|npc| !npc.quest_id.is_empty())
        .expect("some townsperson still carries a plain quest_id")
        .clone();
    npc.quest_ids.clear();

    assert_eq!(npc.quest_chain(), std::slice::from_ref(&npc.quest_id));
    assert_eq!(
        state
            .npc_active_quest(&data, &npc)
            .map(|quest| quest.id.as_str()),
        Some(npc.quest_id.as_str())
    );
}

#[test]
fn quest_gated_nodes_stay_bare_until_their_beat_is_done() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    let town = data.area("town_square").expect("town square should exist");
    let gated = town
        .gather_nodes
        .iter()
        .find(|node| !node.required_completed_quest.is_empty())
        .expect("the turned bed rows should be quest-gated");
    let quest_id = gated.required_completed_quest.clone();
    let node_id = gated.id.clone();

    state.world.current_area_id = "town_square".to_owned();
    // Walk a full season/weather/time cycle so the node's own conditions are
    // never the reason it is absent.
    let mut seen_before = false;
    for day in 0..20 {
        state.world.day_index = day;
        state.refresh_available_nodes(&data);
        seen_before |= state.world.available_nodes.contains(&node_id);
    }
    assert!(
        !seen_before,
        "gated ground should stay bare before its beat"
    );

    state.progression.completed_quests.insert(quest_id);
    let mut seen_after = false;
    for day in 0..20 {
        state.world.day_index = day;
        state.refresh_available_nodes(&data);
        seen_after |= state.world.available_nodes.contains(&node_id);
    }
    assert!(
        seen_after,
        "the turned row should grow once the beat is done"
    );
}

/// Every phrase authored on a townsperson should be something a player can
/// actually hear. The town-recovery observation used to be checked first and
/// returned outright, so from the moment the greenhouse reopened it was the
/// only thing eight of them could say — `post_help_relief` reached three,
/// and `dialogue_complete` nobody at all.
///
/// Walks each townsperson through the states the game puts them in and
/// collects everything they say, then asks whether anything authored was
/// never heard.
///
/// The states now include **the hour**, which they did not when the
/// schedule was only a sprite position. A guard that walks a class of thing
/// has to be revisited when a new writer joins that class, and nothing
/// enforces that — this loop has now been caught by it twice.
#[test]
fn every_line_a_townsperson_has_is_reachable() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut silent = Vec::new();

    for npc in &data.npcs {
        if npc.id == "crow_guide" {
            continue;
        }
        let chain = npc.quest_chain().to_vec();
        let mut heard = std::collections::HashSet::new();

        // Before anything; after a brew; then each arc beat accepted and
        // finished in turn; with and without the town having recovered.
        // Every hour band, because where somebody is standing is now part
        // of what they say.
        for minutes in [8.0 * 60.0, 13.0 * 60.0, 18.0 * 60.0, 23.0 * 60.0] {
            for recovered in [false, true] {
                let mut state = GameplayState::new(&data);
                state.set_clock_minutes(minutes);
                if recovered {
                    state.push_journal_milestone("greenhouse_repaired", "", "");
                }
                let collect =
                    |state: &GameplayState, heard: &mut std::collections::HashSet<String>| {
                        let selection = state.npc_dialogue_selection(&data, npc);
                        heard.insert(selection.start.to_owned());
                        heard.insert(selection.progress.to_owned());
                        heard.insert(selection.complete.to_owned());
                    };

                collect(&state, &mut heard);
                state.progression.total_brews = 1;
                collect(&state, &mut heard);
                for quest_id in &chain {
                    state.progression.started_quests.insert(quest_id.clone());
                    collect(&state, &mut heard);
                    state.progression.started_quests.remove(quest_id);
                    state.progression.completed_quests.insert(quest_id.clone());
                    collect(&state, &mut heard);
                }
            }
        }

        let phase1 = &npc.phase1_dialogue;
        for (label, line) in [
            ("dialogue_complete", npc.dialogue_complete.as_str()),
            ("intro", phase1.intro.as_str()),
            ("pre_help_concern", phase1.pre_help_concern.as_str()),
            ("active_request", phase1.active_request.as_str()),
            ("post_help_relief", phase1.post_help_relief.as_str()),
            (
                "town_recovery_observation",
                phase1.town_recovery_observation.as_str(),
            ),
        ] {
            if !line.is_empty() && !heard.contains(line) {
                silent.push(format!("{}: {label}", npc.id));
            }
        }
        for entry in &npc.schedule {
            if !entry.while_here_line.is_empty() && !heard.contains(&entry.while_here_line) {
                silent.push(format!("{}: while_here_line/{}", npc.id, entry.time_window));
            }
        }
    }

    silent.sort();
    assert!(
        silent.is_empty(),
        "lines authored on a townsperson that nothing can ever say:
{silent:#?}"
    );
}

/// A townsperson standing somewhere they do not work has a reason to be
/// there, and should be able to say it.
///
/// Ten of the thirty-six scheduled stops are away from home, and the
/// schedule had been moving people between rooms since before this loop
/// started with nothing following them: Mira at the lake shore at dusk gave
/// the same words as Mira behind her counter. A player who walks somewhere
/// specifically to find somebody should be told why they came.
///
/// The crow is exempt on purpose. Its four lines are a tutorial ladder tied
/// to progression, an away line would shadow rungs of it, and the crow does
/// not live anywhere — which is the joke and the exemption.
#[test]
fn a_townsperson_away_from_home_has_a_reason_to_be_there() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut unexplained = Vec::new();
    let mut away = 0usize;

    for npc in &data.npcs {
        if npc.id == "crow_guide" {
            continue;
        }
        for entry in &npc.schedule {
            if entry.area_id == npc.area_id {
                continue;
            }
            away += 1;
            if entry.while_here_line.is_empty() {
                unexplained.push(format!(
                    "{} is in {} at {} for no stated reason",
                    npc.id, entry.area_id, entry.time_window
                ));
            }
        }
    }

    assert!(away > 0, "nobody in this town goes anywhere");
    assert!(
        unexplained.is_empty(),
        "scheduled stops nobody can explain:
{unexplained:#?}"
    );
}
