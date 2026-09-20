# TODO

## UI_STYLE review — 2026-09-20

Audit/planning only; no game code changed. Read the updated `AGENTS.md`,
`UI_STYLE.md`, `CODE_STANDARDS.md`, `GAME_DEVELOPMENT_GUIDE.md`, `README.md`,
and the live design reference `docs/alchemy_system_design.md`. No
`PROJECT_AGENTS.md` or separately named GDD was found. The design calls for
readable early brewing and gradually deeper synthesis, with exploration as
its supply loop.

Evidence: inspected current rendering, input dispatch, view models, camera,
layout/scaling and menu/gameplay/pause transitions. Visually inspected existing
`docs/verification/ui_gameplay.png` (1904x961), `ui_gameplay_compact.png`
(1264x681), and 1280x720 `ui_brew.png`, `ui_inventory.png`, `ui_journal.png`,
`ui_shop.png`, `ui_rune.png`, `ui_archive_0_0.png`, `ui_toasts.png`, and
`ui_paused.png`. These are saved captures, not new runs of the current revision.
Archive/toast captures visibly predate the current touch labels; do not treat
obsolete key hints in them as current defects. No live browser, native gameplay,
or touch interactions were exercised in this audit. No minimum gameplay
viewport is declared in the README; 1280x720 is the native/design baseline,
not proof of phone usability.

Keep what already works: PAUSE is separate from exploration actions, save/load
live in pause, overlays have visible close controls, opening hints track shown
state, and the greenhouse journal is unlocked contextually. The art and station
copy are game-specific; no unmodified template demo screen was established.
The issues below concern composition and interaction, not an assumed template
origin. Do not remove costs, stock protections, urgent warnings or recovery.

### Confirmed findings and implementation tasks

Ordered by player impact and implementation dependencies. Establish the brief
and shared responsive geometry before completing dependent screen changes;
touch-access blockers can be fixed independently using that geometry. For all
UI tasks below, verify 1280x720 and the declared minimum actual canvas size,
plus 1920x1080, 640x360 landscape and 360x640 portrait as stress cases. Decide
and document support for the latter sizes rather than silently shrinking them.
Check embedded browser and native layouts, and actual rendered tap sizes.
A practical project target is at least 44x44 logical screen pixels for controls;
this is an audit recommendation, not a numeric requirement in UI_STYLE.md.

- [ ] **UI-01 — Recompose the default exploration HUD around the valley and the next action.**
  - **Screen/files:** Exploration, fresh and established saves; `README.md`,
    `src/ui/hud.rs::draw_hud_view`, `hud_density.rs::visible_panels`,
    `hud_header.rs`, `hud_status_goal.rs`, `hud_side.rs`, `hud_belt.rs`,
    `src/state/gameplay_hud_view.rs`, `src/settings.rs`.
  - **Observed:** Saved gameplay captures and current Full defaults show a large
    title/location banner, vitality medallion, coin panel, full quest note,
    clock/compass, bag/effects/journal plaque and eight-slot belt. Many equally
    ornate regions compete with the player and room; the empty belt and
    Effects/None plaque consume space before they help. Bag/journal entry points
    are duplicated. Quiet HUD exists but is opt-in and retains ornate surfaces.
  - **Change:** Record UI_STYLE §1 briefs for exploration, brewing, collection
    inspection and transactions in the README, including normal/minimum sizes.
    Make the default composition world-first: one compact status group and one
    contextual action/support area. Replace the permanent title with brief area
    arrival feedback; collapse the quest essay to its next actionable objective
    with tap-to-open details. Consolidate bag/journal access, show active effects
    only when present, and collapse unused belt slots while keeping potion access
    discoverable. Relocate coin detail to relevant trade/upgrade decisions and
    inspection. Remove redundant nested brass frames before tuning decoration.
    Preserve a separate, quiet PAUSE route and existing preference compatibility.
  - **Acceptance:** Normal exploration has no more than 2–3 strongly competing
    regions, including the world. The player, nearby target and ACT are apparent
    first; vitality/time pressure and the next objective remain understandable.
    Quiet and Full preferences both meet the attention budget.
  - **Verify:** Capture entry, town, a dense gathering route, empty/full belt and
    active effects at the common size matrix. Tap movement → ACT → bag/journal →
    close → PAUSE → resume. Check no utility is grouped with the gameplay action.

- [ ] **UI-02 — Replace blanket overlay shrinking with responsive content and controls.**
  - **Screen/files:** HUD and gameplay overlays; `src/ui_scale.rs`,
    `src/alchemy_layout.rs`, `src/inventory_layout.rs`, `src/journal_layout.rs`,
    `src/archive_layout.rs`, `src/ui/overlay_layout.rs`, `src/ui/touch_controls.rs`,
    `src/state/gameplay_overlay_input_dispatch.rs` and overlay renderers.
  - **Observed (code-confirmed, phone rendering unverified):** `virtual_ui()`
    scales a 1280x720 canvas uniformly below the baseline. At 640x360, 26-pixel
    heat controls become 13 pixels high and 18-pixel text becomes 9; at 360-pixel
    portrait width they shrink further. Movement controls remain in real screen
    space, creating two competing sizing systems. Letterboxing prevents some
    overlaps but does not establish readability or usable targets.
  - **Change:** Use the UI-01 support contract to introduce shared breakpoint
    layouts with a readable type scale and minimum rendered target dimensions.
    Reflow brewing to material selection/setup/result steps or stacked sections;
    use list/detail navigation for inventory, journal and archive. Keep primary
    actions and Close reachable; give long content bounded scrolling/paging.
    Keep drawing and picking on the same toolkit coordinate conversion instead
    of adding independent screen-space fixes. Reserve space for touch controls.
  - **Acceptance:** At every supported canvas size players can read essential
    costs/warnings and operate every control without zooming the browser, using
    a keyboard, or tapping miniature targets; no clipped or obscured action.
  - **Verify:** Resize and rotate with an overlay open; tap first/last list rows,
    heat controls, tabs, primary action and Close. Check native display scaling,
    browser device scaling and embedded canvas dimensions, not just image sizes.

- [ ] **UI-03 — Frame the playable world within the usable space after HUD subtraction.**
  - **Screen/files:** Exploration camera; `src/state/gameplay_camera.rs::camera_offset`,
    `gameplay_draw.rs`, `src/ui/world_scene.rs`, `src/ui/prompts.rs` and touch layout.
  - **Observed:** The wide saved entry capture centers a roughly 960x720 room in
    a 1904x961 window with large dark side areas, while ornate HUD elements remain
    prominent. Current camera logic centers small rooms and follows in large
    ones, but uses the entire window, fixed world scale and padding rather than
    the unobscured play rectangle. Compact captures place panels over room space.
    The earlier centering fix is already present; do not reintroduce it as work.
  - **Change:** After UI-01/02, derive a safe play rectangle from HUD/touch bands
    and choose a bounded default world zoom/framing appropriate to small rooms
    and outdoor routes. Enlarge meaningful room content where useful without
    cutting off travel context. Centralize transformed world markers and picking
    if zoom is added; assess the toolkit camera before adding local machinery.
  - **Acceptance:** World content dominates normal play; the player and immediate
    interaction target remain clear of controls, with useful approach/exit context.
    Wide layouts gain purposeful framing rather than merely emptier corners.
  - **Verify:** Entry, town and large wild areas at center and all travel edges;
    approach cauldron, bed, NPC, gather node and locked/unlocked warp at each size.
    Recheck marker alignment and interaction selection after resize/zoom.

- [ ] **UI-04 — Make every long action list reachable by touch and bound drawing to its viewport.**
  - **Screen/files:** Alchemy materials, shop stock/sales, rune drafts and archive
    lists; `src/state/gameplay_alchemy_input.rs`, `gameplay_alchemy_mouse_input.rs`,
    `gameplay_shop_input.rs`, `gameplay_shop_overlay.rs`, `gameplay_rune_input.rs`,
    `gameplay_archive_input.rs`, `gameplay_overlay_window.rs`,
    `src/ui/overlay_alchemy_sections.rs`, `overlay_shop.rs`, `overlay_rune.rs`,
    `overlay_layout.rs` and corresponding archive renderers/layout.
  - **Observed:** Materials show four rows, rune drafts five and archive lists six;
    their pointer handlers can select only the current window, with no visible
    next-page path. Selecting the last visible row cannot advance beyond it.
    Shop renders from the beginning and stops after drawing beyond its available
    height; its handler still tests every stock row. The saved shop capture shows
    the sixth row underneath the footer. Current dispatch adds Close, not paging.
    This task incorporates the previous alchemy PREVIOUS/NEXT backlog item.
  - **Change:** Add labelled PREVIOUS/NEXT or discoverable touch scrolling and
    range indicators; use one bounded visible-row model for rendering and input.
    Test row fit before drawing, reserve footer/action space and disable hit
    regions outside the list. Keep selection visible after sort, buy/sell,
    consumption, filtering and tab changes. Provide a tap-accessible sort control
    where sorting is offered rather than only a mode label/keyboard shortcut.
  - **Acceptance:** A touch-only player can reach, inspect and act on the first,
    middle and last eligible entries; hidden rows never accept transactions and
    no row overlaps the footer. Empty/single-page lists remain clear.
  - **Verify:** More than 4 materials, 5 rune drafts, 6 archive records and a large
    sell inventory; page both ways, change filter/sort, consume the last selected
    item and close. Repeat using taps at normal/minimum sizes after UI-02.

- [ ] **UI-05 — Make the brew setup and predicted consequence one coherent decision.**
  - **Screen/files:** Entry and advanced alchemy; `src/alchemy_layout.rs`,
    `src/ui/overlay_alchemy_actions.rs`, `overlay_alchemy_preview.rs`,
    `overlay_alchemy_slots.rs`, `overlay_alchemy_formulae.rs`,
    `src/state/gameplay_alchemy_preview_view.rs`,
    `gameplay_alchemy_preview_detail_text.rs`, `gameplay_alchemy_overlay_view.rs`.
  - **Observed:** `ui_brew.png` repeats Healing Draught as known result and output,
    shows quality/mastery/traits plus multiple pass/match diagnostics, and keeps
    station prose and a full tutorial footer visible. Brew is a 110x28 control
    beside Sort/Clear/Repeat while framed preview text dominates. Catalyst,
    timing and morph information appear alongside the first simple recipe.
  - **Change:** After UI-02/04, compose a dominant setup/result area with a single
    output identity, amount, quality band, live risk and a prominent BREW action.
    Place consumed inputs and actionable shortages beside that action. Put sort
    with the material list and make Clear/Repeat secondary setup tools. Replace
    successful diagnostic prose with a concise readiness state; disclose detailed
    elements/traits/mastery/process reasoning through a labelled details control.
    Introduce advanced explanations when relevant to discovered recipes or the
    selected setup; never hide a process control or risk a current recipe needs.
    Keep Close visually separate and remove redundant section frames/headings.
  - **Acceptance:** A new player can answer what is consumed, what may result,
    why brewing is blocked/risky, and where to tap; experienced players can still
    inspect all earned details. Known/unknown output rules remain intact.
  - **Verify:** First healing brew, empty setup, shortage, unstable/salvage brew,
    quest-quality requirement and unlocked catalyst/morph setup at normal/minimum
    sizes. Tap select → fill/clear → adjust → inspect details → brew → inspect result.

- [ ] **UI-06 — Separate durable player state from temporary event feedback.**
  - **Screen/files:** Exploration and action results; `src/ui/hud_belt.rs::draw_status_strip`,
    `hud_toasts.rs::draw_event_toasts`, `hud_status_time.rs`,
    `src/state/gameplay_runtime_types.rs`, `gameplay_hud_view.rs`,
    `gameplay_overlay_status.rs`, and status/event producers.
  - **Observed:** Runtime `status_text` is overwritten by event messages such as
    closing a window or sleeping; the strip renders any nonempty value without
    expiration. Separate temporary toasts already exist. Saved toast imagery
    shows three central banners over the world; current renderer still stacks
    banners upward from the belt. Persistent warnings are separately generated
    in `sleep_warning_text` and must survive this cleanup.
  - **Change:** Route ordinary action/closure acknowledgements to expiring,
    prioritised feedback instead of a permanent status strip. Keep urgent current
    low-vitality/night warnings with their state/action, not on the event timer.
    Coalesce routine bursts and queue story feedback so it does not cover the
    player/target. Preserve important outcomes in updated inventory, quest,
    route or journal state after their toast ends; add history only where absent.
  - **Acceptance:** Quiet play has no stale event sentence. Results are noticed
    without multiple equally urgent banners; critical consequences remain
    retrievable and current dangers persist until resolved.
  - **Verify:** Gather repeatedly, deliver a request that also unlocks a route,
    close overlays, run low on vitality, reach night and recover by potion/sleep.
    Wait for events to expire, then retrieve outcomes; check reduced motion too.

- [ ] **UI-07 — Give journal routes and herb memories independent selection and contextual detail.**
  - **Screen/files:** Journal Routes; `src/state/gameplay_journal_routes_view.rs`,
    `gameplay_overlay_input_dispatch.rs::handle_journal_overlay_inputs`,
    `gameplay_overlay_input_dispatch.rs::journal_row_at_point`,
    `src/ui/overlay_journal_routes.rs`, `src/journal_layout.rs` and UI state.
  - **Observed:** Both lists share `journal_index`; selecting/browsing a herb also
    changes the selected route and vice versa. The saved dense journal shows two
    separately framed lists/details plus a Tower Access block; the latter takes
    only the first two locked warps rather than details tied to the current route.
  - **Change:** Use independent selection/page state with clearly labelled list
    focus, or separate Routes/Herbs views on compact screens. Show destination
    requirements beside the selected route and offer access to all locked route
    requirements. Remove the unrelated always-present Tower Access block once
    its useful content has a contextual home. Retain undiscovered/learned distinctions.
  - **Acceptance:** Inspecting an herb never silently changes the chosen route;
    players can plan a trip and find the relevant access costs without unrelated
    selections jumping or arbitrary two-item truncation.
  - **Verify:** Unequal long route/herb lists, empty memories, locked/open routes,
    tab changes and reopen. Tap first/last entries independently and page each
    list at normal/minimum sizes; verify requirements agree with world prompts.

- [ ] **UI-08 — Replace repeated instructions and encoded item facts with contextual help and readable summaries.**
  - **Screen/files:** Bag, journal, shop and station overlays;
    `src/ui/overlay_inventory.rs`, `overlay_shop.rs`, `overlay_journal_chrome.rs`,
    `src/state/gameplay_inventory_overlay_view.rs`, `gameplay_shop_overlay.rs`,
    `gameplay_tutorial_hint_selection.rs`, `assets/data/ui_text.json` and
    item/reference text builders.
  - **Observed:** Saved bag/shop views expose compressed facts such as `q20 r1 x3`,
    `quest 1 recipe 1` and repeated bracket tags; bag quantity/reference facts
    recur in both row and detail. Persistent subtitles/footer prose explain
    obvious controls on every visit. Opening hints already track shown state,
    so this is not a request to replace that working mechanism.
  - **Change:** Give each fact one home: rows use name/count plus the relevant
    price or reservation warning, selected details use explicit quality/rarity
    and named uses. Preserve quest stock protections. Replace permanent generic
    instructions with first-use guidance naming exact visible controls and a
    visible Help action that reopens them. Retain useful station-specific effects
    as concise context or inspectable detail, not repeated tutorial wallpaper.
  - **Acceptance:** Players understand an item without decoding abbreviations;
    costs and keep-stock warnings remain visible before use/sale. Returning
    players see their decision, while new players can discover and reopen help.
  - **Verify:** Long item names, large counts, reserved quest items, empty lists,
    first/repeat visits and reloaded saves. Touch through Help → dismiss → inspect
    → use/trade → close; check every replaced keyboard hint has a visible equivalent.

### Further inspection and acceptance evidence — not verified defects

- [ ] **UI-09 — Verify transaction intent and remaining dense/late-game flows before accepting the UI pass.**
  - **Areas/files:** `src/state/gameplay_shop_input.rs`, `gameplay_rune_input.rs`,
    `gameplay_archive_input.rs`, quest/dialogue handlers, `src/ui/overlay_dialogue.rs`,
    menu/pause input and the capture setup in `src/game.rs`.
  - **Inspection needed:** Shop/rune/archive currently execute when tapping the
    already selected row, including a default selection; code establishes that
    behavior, but this audit did not establish whether players understand it or
    whether touch-release/drag interactions cause unintended actions. Dense
    dialogue, archive warnings, quest lists, failure recovery and settings were
    not interactively checked. Old archive captures cannot prove current defects.
  - **Action:** Exercise select/inspect/commit separately with touch, including
    pointer movement/cancellation. If intent is ambiguous, make row taps inspect
    and provide an explicit BUY/SELL/IMBUE/DISASSEMBLE/DUPLICATE action with cost,
    consumed items and disabled reason beside it; keep Close separate. Check
    quest/archive paging beyond the confirmed lists and fix only reproduced gaps.
    Coordinate with the existing release-based input task below, not a duplicate
    input rewrite. Verify unsupported advanced information is disclosed only at
    its actual progression point; do not infer normal unlocks from seeded captures.
  - **Acceptance/verification:** Document outcomes at the declared sizes for new
    game → gather → brew → deliver → unlock → research, plus save/load/resume,
    failed action and recovery. A touch player can inspect without unintended
    resource use, complete supported flows and always exit safely. Capture any
    reproduced issue and turn it into a scoped task before broader changes.

- [ ] **UI-10 — Refresh visual evidence and run the shared acceptance review after implementation.**
  - **Files/state:** `docs/verification/`, legacy `screenshots/`, `README.md`,
    capture harness in `src/main.rs`/`src/game.rs`, and `publish.ps1`.
  - **Gap:** Saved captures have mixed revisions and do not establish current
    minimum-size, embedded-browser or touch usability. Existing screenshot
    consolidation work is merged here; no completion history was removed.
  - **Action:** Capture supported first-use, calm exploration, full belt/effects,
    urgent/failure, dense list and expanded advanced brewing states after UI-01–09.
    Use the declared size matrix, record actual canvas size and tested interactions,
    and check every UI_STYLE §9 criterion. Replace equivalent captures directly
    in `docs/verification/`; consolidate useful legacy screenshots there, remove
    superseded duplicates and update references. Do not call synthetic capture
    success proof of touch interaction or ordinary progression.
  - **Acceptance/verification:** Evidence shows the 2–3-region budget, readable
    text and targets, useful world framing, separated navigation, accessible
    details/help and durable outcomes. Run `.\publish.ps1` with no parameters
    after meaningful implementation changes; report results and precise remaining
    browser/native/touch limitations. This documentation-only audit requires no
    game build or publish and makes no claim that these checks already passed.

## Existing code and architecture backlog

The existing open tasks below are retained. Alchemy list access and screenshot
consolidation were merged into UI-04 and UI-10 above. There were no completed
checkboxes in the starting file.

- [ ] Expose intentional gameplay APIs through `src/lib.rs`, have `main.rs` consume the library, and migrate all tests and test-only helpers from `src/` into `tests/`. Preserve regression coverage; review suites exceeding five cases per feature and consolidate related inputs or explain distinct coverage (§11.3–11.4).
- [ ] Change `publish.ps1` validation from `cargo test --bin alchemy_tower` to include library and integration tests, including `tests/code_standards.rs`; correct that test's obsolete “non-test lines” comment to describe total physical lines (§2.2, §8.3).
- [ ] Remove generic parsing/fallback wrappers in `src/data/embedded_json.rs`, `src/content/embedded_json.rs`, and `src/art/embedded_json.rs`; use toolkit JSON APIs directly, retain source labels, and move any missing generic fallback capability into the toolkit (§5.3).
- [ ] Add semantic validation to `GameData::from_parts` before accepting loaded content: check references, recipe/station/rune/mutation IDs, and balance invariants. Startup currently checks only selected duplicate IDs; reuse the rules in the existing data regression suites and cover malformed input (§5.3).
- [ ] Move remaining balance values and player-facing literals into typed JSON configuration: start with quality/mastery thresholds and scoring in `src/alchemy/quality.rs`, heat bounds in `src/state/gameplay_alchemy_types.rs` and mouse handlers, tutorial timing, and labels in `src/ui/touch_controls.rs`. Make keyboard and touch handlers consume the same settings (§5.3).
- [ ] Use toolkit release-based input for discrete overlay actions currently triggered by `left_mouse_pressed`, starting with alchemy brewing and setup controls; preserve held movement and document any intentional press-triggered action (§7.4).
- [ ] Remove the blanket `allow(dead_code)` and discarded `data` argument in `src/state/gameplay_progression.rs`; remove other unused `_data`, `_h`, and `_index` parameters and update callers. Narrow and explain remaining Clippy allowances, including those in `src/main.rs` (§1.4, §10.2).
- [ ] Extract capture-scene dispatch from the over-200-line `Game::begin_capture_scene` into cohesive helpers, and separate capture setup from normal initialization in `src/state/gameplay_init.rs`; keep functions within 100 lines (§2.1, §4.1).
- [ ] Add missing `//!` purpose comments to production modules, including `src/input.rs`, `src/data/game_data.rs`, and the gameplay/UI child modules; remove same-scope variable shadowing such as `sort_label` in `src/ui/overlay_alchemy_sections.rs` (§9.2, §10.3).
