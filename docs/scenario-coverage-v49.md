# Scenario Coverage v49: Steam Desktop Export Regression Suite

Issue: #1842. Scenario Coverage v49 locks the Steam Desktop Export and Steamworks v1 state shapes from #1838, #1839, #1840, and #1841.

The suite is state/shape-only. It uses committed fixtures and deterministic Rust validators; it does not launch a browser, connect to Steam, use network access, sign, upload, or release anything.

## Matrix

Fixture: `examples/steam-desktop-export-v1/scenario-coverage-v49/matrix.fixture.json`.

| Case | Fixtures | Expected state |
| --- | --- | --- |
| `v49.build-depot.valid` | #1838 build manifest + depot config | deterministic package descriptor with `human-ring3-required` release authority |
| `v49.steamworks.valid` | #1839 wiring, no-Steam fallback, daily-seed leaderboard payload | mocked Steamworks features wired over the existing runtime with trusted local evidence |
| `v49.store-assets.valid` | #1840 store asset plan | proposal-only Steam spec assets through the Milestone 36 asset generation path |
| `v49.demo.composed` | #1841 deterministic demo index | descriptor, wiring, fallback, leaderboard, and store assets compose without network/live browser/Steam |
| `v49.web-build-standalone-backcompat` | standalone web entry + asset manifest golden | existing web runtime remains valid without Electron or Steamworks |

## Boundary

Rust/local owns trusted validation. Browser, Studio, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state. Generated desktop artifacts, depot builds, screenshots, trailer frames, and store assets stay untracked unless fixture-scoped. Steam account work, signing, content survey, store submission, and release are human Ring-3 responsibilities.

This coverage suite asserts behavior states and schema shapes only. It does not claim production readiness, fun/quality, engine parity, automated release, autonomous merge authority, self-review authority, reviewer bypass, or market demand.

## Verification

```bash
cargo test -p ouroforge-core --test scenario_coverage_v49_steam_desktop_export --jobs 2
```

Full issue verification also confirms #1 and #23 remain open and runs workspace fmt, tests, clippy, dashboard/cockpit JavaScript checks, diff checks, and crate-scoped tests.
