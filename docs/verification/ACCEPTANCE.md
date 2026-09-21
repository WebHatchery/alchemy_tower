# Acceptance evidence

Reviewed 2026-09-21 after the responsive overlay and brewing-preview pass.

## Viewport matrix

- `ui_*_minimum.png`: supported 960x540 landscape overlays. Brewing, inventory,
  archive, shop, rune, journal, and gameplay samples were regenerated and
  visually checked for readable text, contained panels, and visible touch
  controls.
- `ui_*` overlay samples: 1280x720 design-size reference captures.
- `ui_gameplay_wide.png`: 1920x1080 wide capture.
- `ui_gameplay_stress.png`: 640x360 stress capture. This remains a known
  limitation: the design-sized HUD intentionally overlaps at this viewport
  until a dedicated stress-size reflow is implemented.

## Interaction coverage

State-transition and transaction behavior remains covered by the Rust test
suite, including brewing, archive records, inventory use, shop/quest delivery,
save/load, rune recipes, and failed brew recovery. The native Computer Use
helper did not expose the launched game window in this environment, so live
Windows touch select -> inspect -> commit coverage could not be performed and
is not claimed here.

The captures use the native Macroquad verification harness. Browser-hosted
touch behavior still requires a browser session with an exposed game surface.
