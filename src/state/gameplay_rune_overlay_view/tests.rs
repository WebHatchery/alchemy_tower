/// A draft row gives its description one line and truncates the rest with
/// an ellipsis. Measured off a capture: the nine original drafts run 55 to
/// 83 characters and read in full, while a 141-character line came back cut
/// mid-sentence. The budget is the house style, not the truncation point —
/// a row that only just fits is a row that stops fitting when the font or
/// the panel width next changes.
const RUNE_DESCRIPTION_BUDGET: usize = 120;

#[test]
fn every_rune_draft_description_reads_in_full() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let overlong = data
        .rune_recipes
        .iter()
        .filter(|recipe| recipe.description.chars().count() > RUNE_DESCRIPTION_BUDGET)
        .map(|recipe| {
            format!(
                "{}: {} chars",
                recipe.id,
                recipe.description.chars().count()
            )
        })
        .collect::<Vec<_>>();

    assert!(
        overlong.is_empty(),
        "draft rows that will be cut off mid-sentence: {overlong:#?}"
    );
}
