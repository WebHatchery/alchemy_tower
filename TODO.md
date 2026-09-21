# Remaining work

- [ ] **Add a 640x360 stress-layout pass.** Reflow the HUD and any modal
  overlays that still overlap at the stress viewport, then replace the stress
  evidence capture.

- [ ] **Exercise live transactional touch flows.** Verify select → inspect →
  commit and recovery for shop, rune, archive, quest, dialogue, and save/load
  in an exposed native or browser game session; fix only reproduced issues.

- [ ] **Finish the library/test boundary migration.** Move remaining test-only
  helpers and in-source suites into `tests/` where practical without losing
  focused regression coverage.

- [ ] **Finish production module documentation.** Add missing `//!` purpose
  comments to gameplay/UI child modules and resolve newly found same-scope
  shadowing.
