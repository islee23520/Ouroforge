# Godot-Plus Demo Performance / Stability Budget v1

Issue: **#794**

These budgets are bounded regression guards for the Signal Gate / `collect-and-exit` vertical slice. They are local fixture expectations, not shipped-game SLAs, hosted telemetry, native export performance claims, or production profiler claims.

| Budget | Measurement / source | Limit / expectation | Regression guard |
| --- | --- | --- | --- |
| Frame budget | `examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json` `metadata.runtimeDebug.frameBudget` and `metadata.runtimeDebug.frameTimings` | Source fixture budget is `totalMs <= 20`; current fixture timing must stay `<= frameBudget.totalMs`. | `examples/godot-plus-demo-performance-v794/performance-budget-smoke.test.cjs` asserts timing is inside the scene budget. |
| Load-time budget | Fresh-clone deterministic fixture smokes for scenario, e2e, asset pack, evidence read model, and plugin usage | No network install/update or generated export is required for the local checks; startup remains deterministic through checked-in fixtures. | `examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs` |
| Console / crash-free budget | Node smoke execution and Rust export/package contract | Focused demo smokes complete without uncaught exceptions; Rust contract completes without panics/failures. | `node --check` for focused smokes; direct smoke execution; `cargo test -p ouroforge-core --test godot_plus_demo_export_package_contract -- --test-threads=1` |
| QA / playtest stability | Bounded QA/playtest plan and QA swarm fixture | QA plan enumerates bounded checks/workers and stays fixture-scoped. | `examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs` |
| Export verification stability | Local web export profile, package metadata, generated-output ignore guard | Export remains local-web-only; profile/package metadata contain no publish, deploy, sign, upload, native/mobile/console, or production release path; generated `/dist/` remains ignored. | `crates/ouroforge-core/tests/godot_plus_demo_export_package_contract.rs`; root `.gitignore` `/dist/`; `examples/playable-demo-v2/collect-and-exit/export/export-profile.json` |
| Boundary / wording stability | Conservative Godot-plus wording, Studio walkthrough, plugin descriptor evidence, and Safe Source Mutation Apply handoff evidence | No-overclaim, no-commercial-release, no-direct-trusted-write, review-gated source apply, plugin descriptor, Studio walkthrough, and #1/#23 governance evidence remain explicit. | `docs/godot-plus-demo-capability-comparison-matrix-v1.md`; `docs/godot-plus-demo-studio-walkthrough-v1.md`; `docs/godot-plus-demo-plugin-usage-v1.md`; `docs/safe-source-mutation-apply-v1.md`; `examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs` |
| Generated-state stability | Fixture-scoped source/evidence files and generated-output boundary wording | Generated demo outputs, exports, QA runs, evidence, screenshots, videos, temp servers, package bundles, and local state remain untracked unless explicitly fixture-scoped. | `examples/godot-plus-demo-closure-v791-794/godot-plus-open-issues-audit.test.cjs`; `git status --short --ignored`; `git diff --check` |

## Known gaps

- No shipped-game SLA, production profiler integration, hosted load testing, large device/browser matrix, multiplayer soak, or release-candidate certification.
- No native/mobile/console export performance measurement.
- No public deploy, signing, store packaging, upload, release automation, or commercial release readiness.
- No executable plugin runtime, marketplace, network plugin install/update, dependency install, or arbitrary command bridge.
- No direct trusted source writes from Studio; source mutation remains review-gated through Safe Source Mutation Apply style handoff fixtures.

## Governance

- Before starting, before merge or closure, and after merge or closure, verify #794 state and confirm #1 and #23 remain open.
- Budget language must remain fixture-scoped; do not promote these rows into production performance, native export, public release, hosted load-testing, broad Godot parity, or replacement claims.
- If future measurements are added, keep them tied to explicit fixture commands or generated artifacts that are ignored unless intentionally fixture-scoped.
- Protected issues #1 and #23 must remain open.

**#1 and #23 remain open.**
