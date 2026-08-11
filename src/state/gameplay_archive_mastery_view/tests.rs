use super::{GameplayState, ARCHIVE_PAGE_ROWS};

/// Four of the archive's five lists took the first six rows while the
/// selection ranged over the whole list, so a player could select — and in
/// the disassembly and duplication tabs *act on* — a row that was neither
/// drawn nor highlighted. Every list must show what it says is selected.
#[test]
fn every_archive_list_shows_the_row_it_says_is_selected() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for recipe in &data.recipes {
        state.progression.known_recipes.insert(recipe.id.clone());
        state
            .progression
            .recipe_mastery
            .insert(recipe.id.clone(), 3);
    }
    for item in &data.items {
        state.inventory.insert(item.id.clone(), 2);
    }

    let known = state.progression.known_recipes.len();
    assert!(
        known > ARCHIVE_PAGE_ROWS,
        "this only proves anything with more rows than one page"
    );

    for index in 0..known {
        state.ui.archive_index = index;
        let mastery = state.archive_mastery_section_view(&data);
        assert!(
            mastery.entries.iter().any(|entry| entry.selected),
            "mastery list shows nothing selected at index {index}"
        );
        assert!(mastery.entries.len() <= ARCHIVE_PAGE_ROWS);

        let disassembly = state.archive_disassembly_section_view(&data);
        if !disassembly.entries.is_empty() {
            assert!(
                disassembly.entries.iter().any(|entry| entry.selected),
                "disassembly list shows nothing selected at index {index}"
            );
        }

        let duplication = state.archive_duplication_section_view(&data);
        if !duplication.entries.is_empty() {
            assert!(
                duplication.entries.iter().any(|entry| entry.selected),
                "duplication list shows nothing selected at index {index}"
            );
        }
    }
}

#[test]
fn a_list_longer_than_a_page_says_so() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let mut state = GameplayState::new(&data);
    for recipe in &data.recipes {
        state.progression.known_recipes.insert(recipe.id.clone());
    }
    assert!(
        state
            .archive_mastery_section_view(&data)
            .page_text
            .is_some(),
        "a multi-page list must tell the player it has more"
    );
}
