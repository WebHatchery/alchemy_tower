use super::{infer_trait_output, SALVAGE_OUTPUT_ITEM_IDS};

/// The salvage list is duplicated knowledge by necessity — the match maps
/// traits to ids and cannot be derived from the constant. Keep them in step
/// by driving every branch and checking where it lands.
#[test]
fn every_salvage_branch_lands_in_the_declared_list() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let probes = [
        "whisper_moss",  // healing
        "arcane_dust",   // luminous
        "ember_root",    // vigor / volatile
        "quarry_lichen", // neither, falls through
    ];
    for probe in probes {
        let output = infer_trait_output(&data, &[probe.to_owned()]);
        assert!(
            SALVAGE_OUTPUT_ITEM_IDS.contains(&output),
            "{probe} salvaged to {output}, which is not in the declared salvage list"
        );
        assert!(
            data.item(output).is_some(),
            "salvage output {output} is not an item"
        );
    }
}
