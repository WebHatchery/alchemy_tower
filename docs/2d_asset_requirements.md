# Alchemy Tower — 2D Asset Requirements

## Purpose

Replace the game's procedural and generated placeholder visuals with a cohesive,
hand-authored 2D art set. The finished game should feel illustrated rather than
assembled from circles, rectangles, generated patterns, and temporary sprites.

This is a visual replacement project, not a redesign of world geometry or game
rules. Existing collision rectangles, interaction positions, routes, station
locations, and progression gates remain authoritative unless a visual review
identifies a genuine playability problem.

## Definition of done

The art replacement is complete when:

- Every player-visible gameplay graphic uses approved 2D artwork.
- No shipped screen relies on procedural shapes as finished art. Runtime geometry
  may remain for clipping, collision, hit testing, debug views, and invisible
  layout only.
- The game no longer depends on `tools/generate_art.py` or
  `assets/generated/` for production visuals.
- All required visual states are represented, including disabled, selected,
  pressed, locked, unlocked, restored, discovered, and progression-changed
  states where applicable.
- Player-facing text is rendered by the game and is not baked into artwork.
- Art remains readable at common desktop browser sizes and on touch screens.
- Missing production art fails clearly during validation instead of silently
  falling back to procedural placeholders.
- `publish.ps1` passes, the verification captures are updated, and
  `catalog_thumbnail.png` shows the final title screen.

## Scope

### Must be replaced

- Area backgrounds and the procedural blocker scenery drawn over them.
- Player, Crow, and townsfolk sprites.
- Stations, planters, habitats, shops, and other interactive props.
- Gather-node sprites and inventory/item icons.
- Warp, apply-target, interaction, selection, and world-marker graphics.
- Story flourishes and other scenery that appears as the world progresses.
- Brewing, gathering, warp, weather, and status-effect visuals.
- HUD chrome, panels, buttons, tabs, slots, frames, dividers, badges, prompts,
  scrollbars, and overlay decoration.
- Menu, Settings, Journal, Alchemy, Archive, Rune, dialogue, quest board, shop,
  ending, pause, and sleep-transition presentation.

### May remain procedural

- Collision and interaction geometry that is not drawn to the player.
- Camera movement, sprite animation timing, particle movement, fading, pulsing,
  and other animation logic.
- Text layout and font rendering.
- Debug-only outlines and diagnostics that are disabled in production.

### Outside this document

- Music, ambient sound, and sound effects.
- Narrative, balance, quest, or world-layout changes.
- A new game name, although no visual asset may bake in the current name.

## Art direction

Use a painterly storybook-fantasy style that can sit naturally beside the current
title-screen illustration: warm, detailed, lived-in, and magical without becoming
ornate high fantasy. Gameplay art must be simpler and more silhouette-led than
the title screen so it remains legible at actual play size.

The camera is top-down with a slight angle bias. Materials should feel grounded:
worn stone, aged timber, stained paper, glass, copper, brass, soil, cloth, and
weathered road surfaces. Magic should appear as a restrained accent rather than
covering every object in glow.

Core palette families:

- Tower: old stone, archive brass, oxidised copper, parchment, teal alchemy light.
- Town: warm timber, clay, muted cloth, market ochres, soft lamplight.
- Wild areas: distinct botanical and geological palettes per biome.
- Containment: cool blue-grey materials with humane, calm habitat lighting.
- Rune floor: charcoal, violet-white glyph light, iron, and heated channel orange.
- Observatory: deep blue-black, silver, cold glass, and restrained starlight.

Silhouette and value contrast must carry meaning before colour does. Avoid tiny
surface detail, noisy texture, pure-black shadows, bloom that obscures edges, and
colour-only distinctions between interactive states.

## Technical delivery standard

### File formats and locations

- Runtime images: PNG in sRGB.
- Transparent sprites: straight-alpha RGBA with clean transparent edges.
- Opaque backgrounds: RGB or RGBA PNG.
- Editable masters: layered PSD, Krita, or equivalent source files retained with
  named layers and no flattened-only source delivery.
- Production exports should live under `assets/art/`, grouped by `areas/`,
  `characters/`, `stations/`, `items/icons/`, `items/world/`, `effects/`, and
  `ui/`.
- Keep `assets/generated/` active only while an asset category is still being
  migrated. Update manifests to production paths category by category.
- Filenames use the existing snake_case data ID. Do not introduce a second name
  for an existing game object.

### General requirements

- Keep important content inside an 8% safe margin unless the asset is explicitly
  designed to bleed.
- Do not bake labels, button text, quantities, key names, or the game title into
  art.
- Do not use unlicensed stock, fonts, brushes, textures, or generated source
  material whose production use is unclear.
- Preserve transparent padding and alignment between animation frames.
- Use linear texture filtering for painterly production assets. Pixel snapping
  may still be used for layout, but the art must not rely on nearest-neighbour
  filtering.
- Each asset must be checked at in-game scale, not only at source resolution.
- Visual decoration must not obscure interaction targets, paths, gather nodes,
  characters, status values, or touch controls.

### Character sheets

- Canvas: 320 × 256 px, transparent.
- Frame: 64 × 64 px.
- Layout: 5 columns × 4 rows.
- Rows, top to bottom: down, left, right, up.
- Column 1: idle. Columns 2–5: four-frame walk cycle.
- Keep feet/pivot placement consistent in every frame; the current renderer
  centres the frame on the character's world position.
- Include a readable contact shadow either consistently in every frame or as a
  separate shared sprite. Do not mix approaches between characters.
- Faces may be simple at play scale, but posture, hair/head shape, clothing,
  carried tools, and palette must make every character recognisable without a
  name label.

### Area backgrounds

- Paint each area at its exact world-space aspect ratio and native pixel size.
  The current 1920 × 1080 placeholders are stretched into several different
  aspect ratios and must not be used as production templates.
- Backgrounds contain non-changing floor, terrain, walls, and noninteractive
  scenery.
- Collision-aligned blocker art may be painted into the background if it does
  not need to overlap moving entities. Any foreground occlusion must be exported
  as a separate transparent layer and drawn after the relevant entities.
- Paths and open movement space must remain visually obvious without debug
  outlines.
- Keep a quiet value range behind characters and interaction markers.
- Do not paint permanent versions of progression scenery into the base plate.

| Area ID | Native canvas | Visual requirement |
| --- | ---: | --- |
| `tower_entry` | 960 × 720 | Restored entry laboratory, old shelves, cauldron work area, bed, worn research surfaces |
| `greenhouse_floor` | 960 × 720 | Glass ribs, warm moisture, cultivated beds, hopeful reclaimed tower space |
| `containment_floor` | 960 × 720 | Humane habitat rings, secure research architecture, calm cool lighting |
| `rune_workshop_floor` | 960 × 720 | Engraved channels, forge surfaces, glyph tools, violet and brass accents |
| `archive_floor` | 960 × 720 | Heavy stacks, parchment drawers, reading machinery, scholarly mystery |
| `observatory_floor` | 960 × 720 | Astral lens architecture, brass circles, cold glass, restrained final-act wonder |
| `north_plains` | 1280 × 900 | Open field roads, meadow pockets, low obstacles, clear crossroads |
| `town_square` | 1280 × 900 | Warm social hub, market paths, civic buildings, readable shop and board areas |
| `rock_fields` | 1200 × 900 | Broken quarry lanes, mineral shelves, dust seams, strong rock silhouettes |
| `moonlit_forest` | 1440 × 960 | Winding paths, damp glades, canopy pockets, charred hollow, cool night identity |
| `lake_shore` | 1200 × 900 | Curved shoreline, reed beds, shallow water, cool stone coves |
| `sunscar_desert` | 1280 × 960 | Dune lanes, exposed stone scars, sparse shelter, hard sunlight |
| `tropical_rainforest` | 1280 × 960 | Layered jungle paths, root arches, broad leaves, wet clearings |
| `southern_pass` | 1280 × 960 | High switchback road, grey-green scree, exposed rock, cloud-line weather |

### Stations and world props

- Export on transparent canvases matching the current assigned size: 96 × 96,
  112 × 112, or 128 × 128 px.
- Preserve the current centre-based placement and keep the interaction footprint
  visually plausible.
- Every station family needs a unique silhouette before colour and glow are
  applied.
- Planters and habitats must show meaningful state changes where the game exposes
  those states. Do not depend on a text label to distinguish empty, growing,
  ready, occupied, repaired, or upgraded states.
- The production set contains 24 current station IDs. The authoritative list and
  size for each is `assets/data/sprites/stations.json`.

Required families include:

- Brewing: entry cauldron, greenhouse still, containment cold bench, archive
  reading bench, and rune forge bench.
- Tower work: rune workbench, archive console, and observatory focus.
- Growing: four greenhouse planters, containment cold bed, and observatory cloud
  frame, with required state variants.
- Habitats: moth, slug, bloomwing, shrimp, and silverfish.
- Civic and trade: request board, apothecary counter, well stock, and roadside
  trader.
- Rest: entry bed.

### Characters

Eleven complete character sheets are required:

| ID | Readable identity |
| --- | --- |
| `player_tower_alchemist` | Practical young male alchemist, travel cloak, ingredient satchel, visible belt vials |
| `player_tower_alchemist_female` | Practical young female alchemist, braided hair, matching travel gear and animation pivots |
| `crow_guide` | Pale-marked magical crow with a distinct silhouette at 64 px |
| `mira_apothecary` | Confident shopkeeper, apron, warm salmon and honey palette |
| `rowan_herbalist` | Field-worn herbalist, gathering tools, green travel layers |
| `mayor_elric` | Modest civic coat, brass trim, practical authority rather than royalty |
| `ione_archivist` | Cool blue and parchment palette, notes or satchel, precise posture |
| `brin_groundskeeper` | Soil-marked work clothes, gloves, sturdy gardener silhouette |
| `lyra_keeper` | Pale blue utility coat, feed pouch or habitat tools, calm posture |
| `tarn_wayfarer` | Weathered road warden, heavy travelling coat and hood |
| `wren_physician` | Plain working linen, dark apron, tired and practical bearing |

### Gather nodes and item icons

- Inventory icon canvas: 64 × 64 px, transparent.
- World-node canvas: 64 × 64 px, transparent.
- Inventory icons should fill roughly 70–82% of the canvas and remain recognisable
  at 24–32 px display size.
- World nodes need a clear ground contact point and enough local context to belong
  to their biome without disappearing into the background.
- Potion families may share bottle construction, but every final icon must be a
  pre-rendered 2D asset with a distinguishable silhouette, liquid treatment,
  stopper, label/mark, or attachment. Colour swaps alone are insufficient.
- Ingredient variants must read as related to the base item while visibly showing
  their special condition.
- Rune, catalyst, creature harvest, botanical, mineral, and potion families each
  need a consistent internal visual grammar.

Current production quantity:

- 179 unique inventory icons, one for every item ID.
- 74 unique world-node sprites used by 96 placed gather nodes.
- The authoritative requirements are the `assets/data/sprites/item_icons*.json`,
  `gatherables.json`, and `gatherable_variants.json` entries.

### Progression scenery and world markers

The current game draws many of these as runtime rectangles, lines, circles, and
glows. Replace their visible form with authored sprites while retaining data-driven
placement and progression rules.

- 61 blocker instances across eight visual families: shelves, houses, panels,
  grass, quarry rock, forest, reeds, dunes, and rainforest growth. Static blocker
  scenery may be incorporated into area plates where layering permits.
- 26 warp placements with at least locked, available, and restored/open visual
  states. Warps must show direction and availability without relying only on
  colour.
- 6 apply targets with untreated, eligible/nearby, and treated states. The four
  potion effect kinds should have related but distinguishable motifs.
- 23 progression flourishes. Replace shape lists with one or more local prop
  sprites per flourish; do not use full-area transparent images for sparse props.
- Interaction, gather, station, NPC, selection, and focus markers. Markers must
  remain visible over every biome and at touch viewing distance.

### Effects and weather

At minimum, replace the current gather sparkle, brew bubbles, and warp glow with
authored sprite animation or sprite particles. Add compatible sprites for rain,
dust, mist, magical status effects, low vitality, successful actions, failed
brews, and major progression feedback where those effects are currently rendered
as primitives.

Animation movement, timing, alpha, and colour tint may remain runtime-driven.
The visible particle or effect shape must come from a 2D asset. Effects should
reinforce feedback without covering controls or item/result text.

### UI kit

Build a reusable 9-slice-capable UI kit instead of painting each screen as a
single image. It must support arbitrary text length, localisation-safe resizing,
desktop scaling, and large touch targets.

Required components:

- Large, medium, small, modal, tooltip, and side-panel frames.
- Header, footer, divider, corner, edge, plaque, and tab treatments.
- Buttons in normal, focused/hovered, pressed, selected, and disabled states.
- Checkbox/toggle, slider, scrollbar, page control, close button, and back button.
- List row, selection card, inventory slot, potion-belt slot, recipe slot, and
  empty-slot states.
- Vitality, coin, clock/day, season/weather, goal, bag, effects, journal, compass,
  minimap, status strip, and toast containers.
- Interaction prompt plate and world-label plate.
- Journal tab icons for routes, notes, brews, greenhouse, and rapport.
- Toast icons for journal note, recipe logged, quest accepted, quest complete,
  route restored, and best quality.
- Icons for touch-visible game actions. No UI art may require a keyboard label.

All actionable controls must have a minimum 44 × 44 px touch hit area at final
render scale. Artwork may be smaller inside that hit area, but the visual state
must clearly communicate that the whole target is tappable. Hover cannot be the
only state that reveals meaning.

### Screen-specific requirements

- Title/menu: retain the illustrated mood, but separate the background from all
  title and button text so the game can be renamed without repainting.
- Settings/pause: controls must remain fully inside the panel at every supported
  scale and show obvious touch states.
- Journal: use the same material language as the rest of the game, give all five
  sections a coherent hierarchy, and keep entries scannable at touch distance.
- Alchemy: make ingredient selection, slots, formula choice, process controls,
  preview, and brew action visually distinct; artwork must clarify the workflow
  rather than add decoration around an unclear layout.
- Archive and Rune: share the core overlay kit while retaining their own room
  identity through restrained thematic accents.
- Dialogue, quest board, and shop: preserve speaker/item focus and keep primary
  actions reachable without a keyboard.
- Ending: support pagination and variable earned content without baked text.

## Asset register summary

The current known minimum is:

| Category | Quantity | Delivery shape |
| --- | ---: | --- |
| Area backgrounds | 14 | Native-size opaque PNG plates |
| Character sheets | 10 | 320 × 256 transparent sheets |
| Stations | 24 base IDs | 96, 112, or 128 px transparent sprites, plus state variants |
| Inventory icons | 179 | 64 × 64 transparent icons |
| World gather nodes | 74 unique | 64 × 64 transparent sprites |
| Existing effect sprites | 3 minimum | Animated frames or sprite particles |
| Journal tab icons | 5 | 32 × 32 or larger source, exported for runtime |
| Toast icons | 6 | 32 × 32 or larger source, exported for runtime |
| Progression flourishes | 23 | Local transparent prop sprites |
| Apply targets | 6 placements | Reusable stateful target sprites |
| Warp placements | 26 | Reusable locked/available/restored state set |
| Procedural blocker instances | 61 | Baked scenery or reusable environment sprites |
| UI kit | 1 coherent set | 9-slice panels, controls, slots, markers, and icons |

Quantities are a baseline, not a cap. State variants, animation frames, foreground
layers, and responsive UI pieces increase the final export count.

## Migration phases

### Phase 0 — style and integration proof

Create a playable vertical slice before commissioning the full set:

- One `town_square` background section or complete plate.
- Player, Mira, and Crow character sheets.
- Request board, apothecary counter, and one gather node.
- Healing Draught plus five representative ingredient/potion icons.
- Core panel, button, slot, prompt, and marker components.
- One progression flourish and one weather/effect treatment.

Integrate the proof in-game and verify scale, filtering, pivots, contrast, memory,
and touch readability. Approval of source artwork alone is not enough.

### Phase 1 — complete first-hour path

Replace the title/menu presentation as needed, `tower_entry`, `north_plains`, and
`town_square`; the player, Crow, Mira, Rowan, and Elric; all first-hour stations,
gather nodes, icons, markers, and effects; and the Settings, HUD, Journal, and
Alchemy UI required to finish the first quest without a keyboard.

### Phase 2 — remaining world and characters

Complete the other eleven area plates, remaining five townsfolk, all stations,
all gather-node sprites, all item icons, biome scenery, warps, and apply targets.

### Phase 3 — progression and specialist screens

Complete all progression flourishes, station state variants, weather/effect
sets, Archive, Rune, quest board, shop, dialogue, ending, and late-game HUD/UI
states.

### Phase 4 — production cleanup

- Point every texture manifest entry at `assets/art/`.
- Remove production fallbacks to generated or primitive visuals.
- Stop publishing generated visual assets.
- Retire `tools/generate_art.py` after confirming no production path uses it.
- Update every affected verification image in `docs/verification/` without
  creating duplicate captures of the same state.
- Replace `catalog_thumbnail.png` with a final title-screen capture.
- Run `publish.ps1` and fix all missing-art, layout, and rendering failures.

## Review and acceptance checklist

Each batch is accepted only after all applicable checks pass:

- Correct ID, path, dimensions, colour mode, and transparency.
- No unexpected opaque fringe or colour halo on transparent edges.
- Correct pivot and no animation jitter.
- Strong silhouette at actual display size.
- Sufficient contrast over every background where the asset appears.
- Interactive and decorative objects are visually distinct.
- Locked, disabled, selected, and completed states are not colour-only changes.
- UI stretches cleanly and text remains inside its panel.
- Touch controls remain obvious and at least 44 × 44 px.
- No baked text, keyboard command, quantity, or current game title.
- Desktop and browser captures match the intended composition.
- Asset loads through the production manifest with no procedural fallback.

The final review must cover a new game, first quest, each world biome, every tower
floor, Settings, Journal, Alchemy, Archive, Rune workbench, shop, quest board,
dialogue, pause, sleep/collapse recovery, ending, and post-ending play.
