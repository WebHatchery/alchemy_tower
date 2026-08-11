use super::{DIALOGUE_FOOTER_SPACE, DIALOGUE_LINE_HEIGHT, DIALOGUE_TEXT_TOP};

/// Measured off a capture: Mayor Elric's 360-character beat wrapped to
/// exactly four lines in the 980-wide panel, so the panel fits roughly 90
/// characters per line. Deliberately conservative — a long word can wrap
/// early and cost a line.
const CHARS_PER_LINE: usize = 90;
/// The shortest window the game is laid out for.
const REFERENCE_SCREEN_HEIGHT: f32 = 720.0;

/// A conversation's body is a townsperson's beat with their earned reaction
/// appended, which reaches ~660 characters late in the story. The panel was
/// a fixed 216 tall — four lines, about 360 characters — so the back third
/// of every arc ran its closing sentences through the footer and off the
/// bottom of the box. The panel now grows with its text; this checks the
/// worst pairing the content can actually produce still fits on screen.
#[test]
fn the_longest_thing_a_townsperson_can_say_still_fits_the_panel() {
    use crate::content::narrative_text;
    use crate::data::load_embedded;
    use std::collections::HashMap;

    let data = load_embedded().expect("embedded game data should load");

    let mut longest_beat: HashMap<&str, usize> = HashMap::new();
    for npc in &data.npcs {
        let own = [npc.dialogue_complete.as_str()]
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        let entry = longest_beat.entry(npc.id.as_str()).or_default();
        *entry = (*entry).max(own);
    }
    for quest in &data.quests {
        for line in [&quest.giver_intro_line, &quest.giver_active_line] {
            let entry = longest_beat.entry(quest.giver_npc_id.as_str()).or_default();
            *entry = (*entry).max(line.chars().count());
        }
    }

    let mut longest_reaction: HashMap<&str, usize> = HashMap::new();
    for reaction in &narrative_text().reactions {
        let entry = longest_reaction
            .entry(reaction.npc_id.as_str())
            .or_default();
        *entry = (*entry).max(reaction.line.chars().count());
    }

    let usable = REFERENCE_SCREEN_HEIGHT - 56.0 - DIALOGUE_TEXT_TOP - DIALOGUE_FOOTER_SPACE;
    let max_lines = (usable / DIALOGUE_LINE_HEIGHT).floor() as usize;
    let budget = max_lines * CHARS_PER_LINE;

    let mut over = Vec::new();
    for (npc_id, beat) in &longest_beat {
        // "+ 1" for the separator the followup format inserts.
        let worst = beat + longest_reaction.get(npc_id).copied().unwrap_or(0) + 1;
        if worst > budget {
            over.push(format!("{npc_id}: {worst} chars"));
        }
    }
    over.sort();

    assert!(
        over.is_empty(),
        "conversations that cannot fit even a full-height panel (budget {budget}): {over:?}"
    );
}
