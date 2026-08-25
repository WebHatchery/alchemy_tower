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
