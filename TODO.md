# TODO — Alchemy Tower

Last verified: 2026-08-23. This file lists only work that remains open.

## Player experience

### Screen and flow fixes

- [x] Move each Settings button fully inside its panel and verify the layout at
  the supported viewport sizes.
- [ ] Audit the Journal screen's panel, tab, row, and footer styling against
  the shared game UI surfaces.
- [ ] Align the Journal screen's colors, typography, spacing, and selection
  states with the rest of the game.
- [ ] Verify that every Journal tab, entry, close control, and required detail
  view remains reachable with tap/click input.
- [ ] Reproduce the town wrap-up ordering change and identify the state or
  collection whose iteration order is unstable.
- [ ] Define and implement a stable ordering key for the town wrap-up entries.
- [ ] Add a regression test proving the town wrap-up order is deterministic
  across repeated runs and equivalent input states.
- [ ] Audit how players discover and open the bag/inventory during normal play.
- [ ] Add or improve a persistent visible bag/inventory affordance with the
  current item count.
- [ ] Improve inventory inspection so the selected item, quantity, uses, and
  relevant actions are legible and reachable by touch.
- [ ] Audit the Alchemy screen's material, formula, result, quality, and action
  hierarchy for the first-quest and later-game workflows.
- [ ] Improve the Alchemy screen's requirement, result, quality, and error
  feedback without hiding the primary brew action.
- [ ] Verify that the Alchemy screen's complete workflow is usable at supported
  desktop and touch viewport sizes.
- [ ] Add a visible in-game hint that identifies the healing-potion recipe and
  its required ingredients during the first quest.
- [ ] Link the first-quest guidance to the relevant recipe or Alchemy screen
  entry point, where the current UI supports that navigation.
- [ ] Add a progression check proving the healing-potion guidance appears at
  the right time and resolves after the required potion is made.
