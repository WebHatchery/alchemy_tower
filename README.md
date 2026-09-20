# Alchemy Tower

Alchemy Tower is a cozy exploration and potion-making game about restoring an abandoned tower and reconnecting with the town around it.

You gather ingredients, learn recipes, help townsfolk, and unlock new tower floors. The focus is discovery, preparation, and gentle progression rather than combat.

## Gameplay

- Explore the tower, town, and surrounding wilds.
- Gather herbs and ingredients under different conditions.
- Brew potions through station-based alchemy.
- Complete town requests and build rapport with locals.
- Restore tower floors to unlock new spaces and routines.

## Goal

Turn the abandoned tower into a working magical home while learning the rhythms of the valley and expanding your recipe knowledge.

## Controls

- Use the on-screen arrows to move.
- Tap ACT beside an object, person, or route to interact.
- Tap BAG or JOURNAL to inspect carried items or notes, and PAUSE to save, load,
  resume, or return to the menu.
- Keyboard shortcuts are optional extras; every required action has a visible
  touch control.

## Current Scope

Playable exploration, gathering, brewing, requests, tower restoration, inventory flow, and save/load progression.

## Design Reference

`docs/alchemy_system_design.md` is the live specification for the brewing engine — element profiles, traits, quality bands, mastery, morph paths, and the instability fallback. Open work is tracked in `TODO.md`.

## Player settings and toolkit review

Settings shows fullscreen, Quiet HUD, screen shake, reduced motion, master and
effects volume, FPS display and reset together. Wide windows use two columns;
portrait windows use one stack. Tap a volume to advance by 10%, wrapping from
100% to mute. Changes save immediately, and a failed save leaves the active
preferences unchanged. Reset settings restores preferences only; it does not
erase game progress.

Preferences use the toolkit `GameSettings` model and JSON persistence, with
Quiet HUD as a game-specific extension. Reduced motion applies the toolkit
policy and freezes local weather, marker and alchemy animation, NPC sway and
collapse flashing. Screen shake respects both effect preferences. Native audio
multiplies each effect's authored volume by master and effects volume. Browser
audio remains disabled by the existing startup implementation; the settings panel
says so. Browser fullscreen requires a new tap after reload.

The toolkit also provides `SettingsPanel`, `SettingsSession`, display previews,
UI/text scaling, autosave controls, audio groups, control preferences and camera
preferences. The existing menu action/render separation and tower styling are
retained. Music, voice, background muting, remapping, camera controls, autosave
and user scaling are not exposed until the game integrates those consumers;
showing those toolkit fields alone would create ineffective controls.

## UI screen briefs and support contract

The normal design canvas is 1280x720. The supported responsive range is 960x540
and larger in landscape; 1920x1080 is a wide-layout check. 640x360 landscape
and 360x640 portrait remain stress cases until the responsive overlay pass
provides a readable stacked layout, so those sizes are expected to letterbox
and are not claimed as supported gameplay canvases.

Exploration: the current decision is “what should I do next in this place?”
The dominant focus is the valley and the nearby target. The primary action is
the visible ACT prompt; vitality, time pressure, and the next objective stay
near the edges without competing with the world. Bag, Journal, and PAUSE are
separate visible touch controls. Area names appear briefly on arrival rather
than as a permanent title. Empty potion slots collapse to one discoverable
slot, while active potions remain available from the belt.

Brewing: the current decision is “what will this setup consume and produce?”
The dominant focus is the selected materials, slots, and predicted outcome.
Heat, stirs, timing, shortages, and BREW remain beside the setup; advanced
traits, mastery, and morph reasoning are supporting detail. Close is a separate
navigation action. At the minimum supported landscape size, the panel keeps its
readable fixed design geometry; smaller stress sizes are recorded as known
limitations pending the stacked overlay work.

Collection inspection: the current decision is “which carried item matters for
the work ahead?” The list is the supporting area and the selected detail is the
dominant comparison. Rows show name, held count, and only the warning that
affects selection; explicit uses and quality appear in the detail view.

Transactions: the current decision is “is this safe and affordable to buy or
sell?” The selected item, price, held/keep-stock warning, and explicit action
are kept together. Navigation and Close remain visually separate from the
trade decision.
