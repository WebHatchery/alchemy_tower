# Remaining work

- [ ] **Finish responsive overlay composition.** Replace the remaining blanket
  virtual-UI shrinking below the 1280x720 design size with breakpoint layouts
  for the supported 960x540 landscape viewport; reflow alchemy and long-list
  screens, preserve readable essential text, and verify rendered touch targets.

- [ ] **Complete the brewing screen review.** Make the material/setup/preview
  hierarchy clear at the supported sizes and remove any dense or duplicated
  process information that still competes with BREW.

- [ ] **Exercise late and transactional flows.** Verify touch select → inspect
  → commit for shop, rune, archive, quest, dialogue, save/load, and failure
  recovery flows at the supported sizes; add explicit archive actions or fix
  only issues reproduced by the review.

- [ ] **Refresh acceptance evidence and publish.** Replace stale equivalent
  captures in `docs/verification/`, cover the supported-size and stress matrix
  plus touch interactions, record remaining browser/native limitations, and
  run `./publish.ps1` after the implementation pass.

- [ ] **Move the remaining alchemy balance into typed data.** Transfer quality
  bands, mastery thresholds, scoring/volatility values, and timing IDs out of
  Rust constants; make all callers use the same authored configuration and
  extend semantic validation and regression coverage.

- [ ] **Finish the library/test boundary migration.** Move remaining test-only
  helpers and in-source suites into `tests/` where practical without losing
  focused regression coverage.

- [ ] **Finish production module documentation.** Add missing `//!` purpose
  comments to the remaining gameplay/UI child modules and resolve any newly
  found same-scope shadowing.

- [ ] **Separate capture setup from normal initialization.** Extract the
  capture-specific setup still embedded in `src/state/gameplay_init.rs` while
  keeping normal game startup and capture scenes independently testable.
