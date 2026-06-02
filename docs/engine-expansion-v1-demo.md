# Engine Expansion v1 Integration Demo Evidence

Issue: #170 — Engine Expansion v1 Integration Demo

Status: final audit documentation for the bounded Engine Expansion v1 milestone.
Ouroforge remains a local, evidence-native prototype. This document does not make
production, public-launch, broad compatibility, or Godot replacement claims.

## Demo entrypoints

- Integration seed: `seeds/engine-expansion-v1-demo.yaml`
- Playable template seed: `seeds/platformer.yaml`
- Runtime scene: `examples/game-runtime/scene.json`
- Evidence dashboard: `examples/evidence-dashboard/`
- Studio/cockpit inspection: `examples/authoring-cockpit/`

## Source issue map

| Capability | Source issue | Demo proof |
| --- | --- | --- |
| Milestone scope/guardrails | #157 | This doc and `docs/engine-expansion-v1.md` preserve local/browser-first boundaries. |
| Renderer layers/camera/debug state | #158 | Integration seed asserts `renderer.version` and rendered entity ordering. |
| Tilemap data/collision layers | #159 | Integration seed asserts `tilemaps.version`, tilemap id, and collision layer. |
| Asset manifest/local asset refs | #160 | Integration seed asserts manifest enabled/count and loaded asset evidence. |
| Sprite-frame animation | #161 | Integration seed asserts `sprite_frame` mode and deterministic frame index. |
| Audio intent evidence | #162 | Integration seed asserts `player_spawn` with `playback: intent`. |
| Physics/collision v2 | #163 | Integration seed asserts trigger/contact event evidence and wall normal. |
| Runtime hot reload boundary | #164 | Integration seed asserts reload boundary state remains observable and empty before reload. |
| Scene composition/defaults | #165 | Integration seed asserts `player-badge` parent and composed transform evidence. |
| Studio Runtime Authoring v2 | #166 | Cockpit README documents static scene/entity/component and command inspection. |
| Playable template | #167 | `seeds/platformer.yaml` and `seeds/engine-expansion-v1-demo.yaml` run with workers=4. |
| Native export design gate | #168 | `docs/native-export-design.md` records NO-GO now. |
| Plugin system design gate | #169 | `docs/plugin-system-design.md` records NO-GO now. |
| Final integration | #170 | This document plus PR/issue audit evidence. |

## Verification workflow

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- seed validate seeds/engine-expansion-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
BEFORE_RUN=$(ls -td runs/run-* | sed -n '2p')
AFTER_RUN=$(ls -td runs/run-* | head -1)
test -n "$BEFORE_RUN"
test -n "$AFTER_RUN"
cargo run -p ouroforge-cli -- compare "$BEFORE_RUN" "$AFTER_RUN"
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

## Latest #170 verification evidence

Fresh merged-branch verification for EE14.3:

- Integration run: `runs/run-1780397215365-83807` — workers 4/4 passed, scenarios 2/2 passed, verdict passed.
- Platformer/template run: `runs/run-1780397228155-84262` — workers 4/4 passed, scenarios 2/2 passed, verdict passed.
- Comparison artifact: `runs/comparisons/run-comparison-run-1780397215365-83807--run-1780397228155-84262.json` — classification `no_change`.
- Integration scenario refs: `evidence/scenarios/feature-surface-smoke/scenario-result-1780397223706.json`, `evidence/scenarios/objective-contact-integration/scenario-result-1780397223754.json`.
- Platformer scenario refs: `evidence/scenarios/bootstrap-smoke/scenario-result-1780397236679.json`, `evidence/scenarios/objective-contact/scenario-result-1780397236725.json`.
- Dashboard export: `examples/evidence-dashboard/dashboard-data.json` generated locally and not committed.
- Node dashboard/cockpit checks: `node --check` plus smoke tests passed for dashboard and cockpit.
- Rust checks: `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` passed.

Generated `runs/` and dashboard export data remain local/generated state and must
not be committed.

## Evidence-native loop coverage

- **Seed**: `seeds/engine-expansion-v1-demo.yaml` and `seeds/platformer.yaml` are
  Rust-validated Seed files.
- **Run**: CLI runs create local `runs/run-*` directories with worker evidence.
- **Observe**: Browser workers and scenarios capture screenshots, world state,
  frame stats, console logs, performance metrics, CDP summaries, and scenario
  result artifacts.
- **Verify**: Evaluator verdicts link scenario results and evidence refs; compare
  creates read-only before/after run comparison artifacts.
- **Journal**: Runs update journal artifacts through the existing CLI flow.
- **Evolve compatibility**: The integration demo preserves the artifact shape
  consumed by existing journal/evolve/compare flows; #170 does not add new evolve
  behavior or accept mutations.

## Known gaps and next milestone candidates

Known gaps are deliberately documented, not implemented in #170:

- no native export implementation; #168 is NO-GO now;
- no plugin system, marketplace, registry, or dynamic loading; #169 is NO-GO now;
- no distributed QA/Elixir implementation;
- no server, database, cloud, auth, telemetry, native shell, updater, installer,
  signing, notarization, or marketplace workflow;
- no polished production game, campaign, multiplayer, complex enemy AI, or public
  launch claim;
- no broad Godot compatibility or replacement claim;
- no direct browser file writes or browser-side trusted persistence.

Candidate future milestone work should be opened as separate scoped issues with
fresh evidence gates, not hidden inside this integration demo.

## Guardrail checklist

- [x] Integration uses completed features only.
- [x] Known gaps are documented rather than implemented.
- [x] Generated evidence remains untracked.
- [x] Dashboard and cockpit remain inspect-only/read-only/in-memory surfaces.
- [x] Native export, plugin, distributed, server/cloud/auth, and public-release
      scope remains excluded.
- [x] Final audit maps demo capabilities back to closed source issues.
