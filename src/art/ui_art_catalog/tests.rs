use super::ui_art_catalog;

/// The fallback icon has to name a real one, or a toast raised with no icon
/// key — every `push_event_toast` call — draws nothing beside itself. This
/// key sat in the file unread for the whole project, so it has never once
/// been checked against the list directly underneath it.
#[test]
fn the_default_toast_icon_names_one_that_exists() {
    let catalog = ui_art_catalog();
    assert!(
        !catalog.toast_icons.is_empty(),
        "no toast icons are registered at all"
    );
    assert!(
        catalog
            .toast_icons
            .iter()
            .any(|icon| icon.key == catalog.default_toast_icon),
        "default_toast_icon is {:?}, which is not in toast_icons",
        catalog.default_toast_icon
    );
}
