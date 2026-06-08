# Post-Launch Patch Demo v1

Issue: #1847. This deterministic fixture-scoped demo composes the existing post-launch patch re-verify loop with save migration:

1. Load `examples/post-launch-patch-v1/patch-reverify.pass.fixture.json`.
2. Re-run the Rust/local re-verify gate model and derive a local Steam desktop package descriptor from the existing Steam export fixtures.
3. Load `examples/save-migration-v1/legacy-save.v0.fixture.json`.
4. Migrate the old save forward through the existing save/restore and replay-digest contract.

The demo manifest is `examples/post-launch-patch-v1/demo/manifest.fixture.json`.

## Determinism and boundaries

- The demo is offline and fixture-scoped: no network, no live browser, and no generated run/build artifact is tracked outside the fixture tree.
- Rust/local owns trusted validation, re-package evidence, save migration, and digest integrity.
- Browser/Studio/Electron/Steamworks surfaces remain read-only and do not perform direct trusted writes.
- The release decision remains human/Ring-3; this demo does not automate a Release button, Steam upload, signing, account creation, content survey, wishlist, market-demand, quality, or fun verdict.
- The demo asserts behavior and gate states only: a patch re-verifies before re-package, and an old save migrates forward with replay digests preserved.
- #1 remains open.
- #23 remains open.

## Smoke command

```bash
cargo test -p ouroforge-core --test post_launch_patch_demo_contract --jobs 2
```

Expected result: the demo manifest validates, the patch re-package status is `repackaged-after-reverify`, the save migration status is `migrated`, the migration preserves replay digests, and both outcomes serialize deterministically across repeated runs.
