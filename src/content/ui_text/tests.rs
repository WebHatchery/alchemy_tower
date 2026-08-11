use std::path::Path;

/// Key families the game builds at runtime from an item id rather than
/// naming outright, so no literal for them appears in the source.
const COMPOSED_PREFIXES: [&str; 2] = ["journal_herb_summary_", "journal_potion_recap_"];

fn all_rust_source() -> String {
    fn walk(dir: &Path, out: &mut String) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    out.push_str(&text);
                    out.push('\n');
                }
            }
        }
    }
    let mut source = String::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut source,
    );
    source
}

/// Fifty-two strings in `ui_text.json` were written, translated in spirit,
/// and asked for by nothing: superseded HUD and gather lines, four
/// `effect_name_*` keys for a composer that was never built, and two brew
/// failure messages that contradicted the deliberate-overfire design the
/// game settled on.
///
/// Copy is looked up by string, so an orphan costs nothing at runtime and
/// is invisible without a check like this one — it just makes the file
/// harder to read and the next person unsure which line is the live one.
#[test]
fn every_line_of_copy_is_asked_for_by_something() {
    let source = all_rust_source();
    let orphans = super::ui_text()
        .copy
        .keys()
        .filter(|key| {
            !COMPOSED_PREFIXES
                .iter()
                .any(|prefix| key.starts_with(prefix))
        })
        .filter(|key| !source.contains(&format!("\"{key}\"")))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        !super::ui_text().copy.is_empty(),
        "no copy loaded at all — ui_text.json is not reaching the game"
    );
    assert!(
        orphans.is_empty(),
        "copy nothing in the game ever asks for:\n{orphans:#?}"
    );
}

/// The composed families are the one place a key can be live without its
/// name appearing in the source, so they need the opposite check: a
/// `journal_herb_summary_<id>` for an item that does not exist is a line
/// nothing will ever look up.
#[test]
fn composed_copy_keys_name_real_items() {
    let data = crate::data::load_embedded().expect("embedded game data should load");
    let stranded = super::ui_text()
        .copy
        .keys()
        .filter_map(|key| {
            COMPOSED_PREFIXES
                .iter()
                .find_map(|prefix| key.strip_prefix(prefix))
                .map(|item_id| (key.clone(), item_id.to_owned()))
        })
        .filter(|(_, item_id)| data.item(item_id).is_none())
        .map(|(key, _)| key)
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        stranded.is_empty(),
        "copy composed for items that do not exist:\n{stranded:#?}"
    );
}
