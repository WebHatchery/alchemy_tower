use super::GameplayState;

#[test]
fn the_archive_waits_for_iones_complete_recovered_record() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);

    for quest_id in &data.config.archive_required_completed_quests {
        if quest_id != "restored_record_for_ione" {
            state.progression.completed_quests.insert(quest_id.clone());
        }
    }
    for milestone_id in &data.config.archive_required_journal_milestones {
        if ![
            "record_reconciled",
            "eleven_months_restored",
            "the_previous_hand",
        ]
        .contains(&milestone_id.as_str())
        {
            state.push_journal_milestone(milestone_id, "", "");
        }
    }

    assert!(!state.can_reconstruct_archive(&data));

    state
        .progression
        .completed_quests
        .insert("restored_record_for_ione".to_owned());
    assert!(
        !state.can_reconstruct_archive(&data),
        "the quest flag alone should not stand in for its recovered evidence"
    );

    state.push_journal_milestone("record_reconciled", "", "");
    assert!(!state.can_reconstruct_archive(&data));
    state.push_journal_milestone("eleven_months_restored", "", "");
    assert!(!state.can_reconstruct_archive(&data));
    state.push_journal_milestone("the_previous_hand", "", "");

    assert!(state.can_reconstruct_archive(&data));
}
