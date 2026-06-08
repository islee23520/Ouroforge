# Scenario Coverage v50: Post-Launch Patch Regression Suite

Issue: #1848. This suite locks Post-Launch Patch, Re-Verify, and Save-Migration Loop v1 behavior with deterministic state/shape-only regressions.

Fixture root: `examples/post-launch-patch-v1/scenario-coverage-v50/`.

## Enumerated cases

1. `v50.patch-reverify.pass` — the full patch gate set passes and re-package evidence can be derived locally.
2. `v50.patch-reverify.fail` — a non-passing patch gate blocks re-package.
3. `v50.save-migration.forward` — an old save migrates forward and replay digests are preserved.
4. `v50.save-migration.incompatible` — an incompatible save fails closed with explicit evidence and no migrated state.
5. `v50.non-patched-build-save-backcompat` — a non-patched existing web build and legacy save profile remain valid.

## Boundaries

- Coverage asserts states/shapes only; it has no timing, live browser, network, Steam upload, signing, release, or subjective fun assertion.
- Rust/local owns trusted validation for patch gates, re-package evidence, save migration, digest integrity, and compatibility.
- Browser/Studio/Electron/Steamworks surfaces remain read-only and have no trusted write authority.
- Generated runs, package descriptors, migrated saves, and build artifacts remain untracked unless fixture-scoped.
- No auto-merge, self-approval, reviewer bypass, automated score, production-ready claim, broad engine-parity claim, or automated taste metric is authorized.
- #1 and #23 remain open.

## Smoke command

```bash
cargo test -p ouroforge-core --test scenario_coverage_v50_post_launch_patch --jobs 2
```
