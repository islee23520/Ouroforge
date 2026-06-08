# Steam Desktop Export Demo v1

Issue: #1841. This document records the deterministic, fixture-scoped Steam desktop export demo for Era I Milestone 54.

The demo composes existing contracts instead of introducing a new runtime, browser session, or Steam connection:

- `examples/steam-desktop-export-v1/demo/build-manifest.fixture.json` and `depot-config.fixture.json` validate the packaged desktop artifact descriptor path from #1838.
- `examples/steam-desktop-export-v1/demo/steamworks-wiring.fixture.json`, `no-steam-fallback.fixture.json`, and `daily-seed-leaderboard.fixture.json` validate mocked Steamworks feature wiring from #1839.
- `examples/steam-desktop-export-v1/demo/store-assets.fixture.json` validates Steam store asset proposals from #1840, reusing the Milestone 36 asset generation proposal pipeline.
- `examples/steam-desktop-export-v1/demo/demo-index.fixture.json` ties the fixture set together for the smoke test.

## Deterministic behavior

From a fresh clone, the smoke test parses only committed fixtures and derives the package descriptor from canonical JSON. It does not require network access, a live browser, Steam credentials, a real Steam SDK connection, signing, upload, or a release button.

## Boundary

Rust/local owns trusted validation and deterministic descriptor/provenance checks. Browser, Studio, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state. Generated desktop artifacts, depot builds, screenshots, trailer frames, and store assets remain untracked unless they are fixture-scoped proposals. Human Ring-3 action is required for Steam account work, signing, content survey, store submission, and release.

The demo asserts behavior and gate states only. It does not claim production readiness, a fun/quality verdict, engine-parity marketing, automated release authority, autonomous merge authority, self-review authority, reviewer bypass, or market demand.

## Verification

The required smoke test is:

```bash
cargo test -p ouroforge-core --test steam_desktop_export_demo_contract --jobs 2
```

Full issue verification also runs workspace formatting, tests, clippy, dashboard/cockpit JavaScript checks, diff checks, and confirms #1 and #23 remain open.
