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
- Tap BAG & JOURNAL to inspect your notes and PAUSE to save, load, resume, or
  return to the menu.
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
