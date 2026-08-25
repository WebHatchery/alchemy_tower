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
