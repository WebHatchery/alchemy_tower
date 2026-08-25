# Alchemy Tower concept art

This is a visual-development pass for the existing game. These images are
concept references, not runtime-ready exports: the game remains responsible for
all player-facing text, interaction markers, and gameplay-safe composition.

## Shared visual language

- Painterly storybook fantasy with grounded, lived-in materials.
- Old stone, aged paper, tarnished brass, oxidised copper, worn timber, and
  clear hand-blown glass.
- Teal alchemy light is a restrained accent, never the whole image.
- Recovery is visible in the land and in repaired objects: the tower is tended,
  not conquered.
- Silhouette and value contrast should remain readable at game scale.

## Studies

### Existing anchor

![Title-screen anchor](title_screen_anchor.png)

The current title-screen illustration establishes the warm laboratory, valley,
and player silhouette that the new studies extend.

### New environment studies

![Tower exterior at dawn](tower_exterior_dawn.png)

The exterior establishes the tower's modest scale, repaired greenhouse annex,
lit path, recovering terraces, orchard rows, and the valley waterfall.

![Greenhouse floor](greenhouse_floor.png)

The greenhouse shows recovery in progress: thriving beds sit beside bare seed
trays, with a clear central aisle and a practical copper still.

![Town square at golden hour](town_square_golden_hour.png)

The town square makes the social side of restoration visible: a usable well,
request board, small market stalls, and the tower present in the distance while
the road fills in again.

![Archive floor](archive_floor.png)

The archive is cooler and more measured. Reading benches, instruments, scraped
pages, and the open view outward support the story's diagnose-before-fixing
rhythm.

![Observatory at blue hour](observatory_blue_hour.png)

The observatory resolves the visual arc with a working instrument and a living
valley below. Its magic is precise and quiet rather than a spectacle.

### Key instrument sheet

![Key instruments](key_instruments_sheet.png)

The cauldron, greenhouse still, and cloudglass focus share a craft language:
heavy bases, repairable fittings, small teal cores, and materials that show use.

### Cast and world studies

![Character cast](character_cast_sheet.png)

The cast sheet gives the nine townsfolk, the player, and the Crow distinct
silhouettes built from the same practical valley wardrobe. Tools carry more
identity than costume ornament.

![Wild biome color keys](wild_biome_color_keys.png)

The route strip separates the gathering areas by value and palette before small
items or markers are added: plains, moonlit forest, lake shore, sunscar desert,
and southern pass.

![Ingredient prop sheet](ingredient_prop_sheet.png)

The ingredient sheet sets a compact visual vocabulary for gathered materials:
botanical texture, mineral transparency, and a small number of readable color
accents.

![Journal spread mood](journal_spread_mood.png)

The journal study keeps large quiet page areas for runtime copy while using
pressed specimens, brass corners, route sketches, and a teal bead to connect the
overlay to the world.

![Restoration progression](restoration_progression_triptych.png)

The triptych is a direct visual check on the story's central rule: the same
terrace moves from dormant, to newly tended, to flowering. The tower is a quiet
instrument in every state; the ground carries the change.

![Tower floor cutaway](tower_floor_cutaway.png)

The cutaway gives the tower a single architectural spine across entry,
greenhouse, containment, archive, rune workshop, and observatory. It is useful
for checking that each floor feels like a room in the same building.

![Arrival keyframe](arrival_keyframe.png)

The arrival keyframe keeps the player and Crow small in the landscape and makes
the first promise of the game visual: there is work to do, but the place is
already worth caring for.

![Potion bottle family](potion_bottle_family.png)

The bottle family is a first pass at silhouette-led alchemy feedback. Different
forms should remain identifiable when the runtime reduces them to small journal
or belt icons.

![Containment floor](containment_floor.png)

The containment study keeps the floor cool and precise while making every
habitat feel humane. Glass, water, moss, and observation tools carry the mood;
the room does not need spectacle to feel magical.

![Rune workshop floor](rune_workshop_floor.png)

The rune floor provides the strongest contrast in the set: charcoal stone,
contained violet-white glyphs, and heated orange channels. The glow is local to
the work surfaces, keeping the rest of the room legible.

![UI chrome sheet](ui_chrome_sheet.png)

The UI sheet translates the same material language into reusable blank panels,
tabs, slots, badges, and touch buttons. It is a reference for hand-authored
chrome only; all copy and state text remains runtime-rendered.

![Valley route map](valley_route_map.png)

The route map ties the art direction together at world scale: the tower is the
instrument on the hillside, the town is the social center below, and the roads
lead outward to distinct gathering palettes.

![Town prop sheet](town_prop_sheet.png)

The town prop sheet gives the social spaces a practical vocabulary of their own:
requests, water, remedies, and the lantern that tells the road the valley is
open again.

![Valley restored at evening](valley_restored_evening.png)

The ending keyframe is the final palette target: flowers and orchard rows in the
foreground, an active town below, a clear river, and the tower reduced to one
warm part of a larger living landscape.

![Weather and lighting variants](weather_lighting_variants.png)

The variant study holds the route and tower silhouette steady across clear
morning, rain, and moonlit night. It is a reminder that readability should come
from value, path shape, and local lamps before color or glow.

![Archive evidence still life](archive_evidence_still_life.png)

The archive still life turns the story's measurements into visible props: an
interrupted record, mirrorsalt, comparison trays, and quiet counters. It keeps
the mystery grounded in absence and evidence instead of spectacle.

![Player variants](player_variants_sheet.png)

The player variants share one coat, satchel, bottle belt, and teal tool language
while allowing different presentation. The Crow remains the stable companion
silhouette between them.

## Prompt set used

The image-generation pass used the `stylized-concept` use case with one prompt
per study:

1. Restored tower exterior at dawn, with recovering terraces and valley road.
2. Restored greenhouse floor, showing cultivation and recovery in progress.
3. Archive floor, showing measurement tools and the reconstructed record.
4. Observatory floor at blue hour, showing careful use and a living valley.
5. Town square at golden hour, showing reconnection and the return of ordinary
   traffic.
6. Three-instrument prop sheet: entry cauldron, greenhouse still, and
   cloudglass focus.
7. Character cast sheet for the player, nine townsfolk, and the Crow.
8. Five-panel wild biome color-key strip for gathering routes.
9. Ingredient prop sheet for eight recurring gathered materials.
10. Open journal spread with blank space for runtime notes and recipes.
11. Restoration progression triptych showing dormant, first-tended, and
    flowering ground.
12. Tower cutaway showing the six functional floors as one repaired structure.
13. Arrival keyframe with the player and Crow approaching the tower.
14. Potion bottle family for the main effect silhouettes.
15. Containment floor with humane habitats and observation tools.
16. Rune workshop floor with controlled glyph and forge lighting.
17. Blank UI chrome sheet for panels, tabs, slots, badges, and touch buttons.
18. Valley route map connecting the tower, town, river, and gathering regions.
19. Town prop sheet for the request board, apothecary counter, well, and
    waystation lantern.
20. Ending keyframe showing the restored valley at evening.
21. Weather and lighting variants for the tower-entry route.
22. Archive evidence still life for the interrupted record and measurements.
23. Player variants sheet for the shared alchemist design and Crow companion.

Each prompt required no readable text, logos, watermark, or UI, and asked for
the same material palette and restrained teal magic.

## Follow-up production targets

1. Translate the strongest silhouettes into the exact native area canvases in
   `docs/2d_asset_requirements.md`.
2. Simplify each scene into a gameplay-safe base plate plus optional foreground
   occlusion layers.
3. Derive transparent station and prop sprites from the instrument sheet.
4. Keep the archive's measurement language and the observatory's restraint when
   designing restored-state variants.
