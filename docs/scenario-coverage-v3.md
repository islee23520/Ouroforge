# Scenario Coverage v3

Scenario Coverage v3 is the Engine Expressiveness v2 regression suite for issue
#320. It keeps feature-specific checks separate from the integrated playable demo
so failures are attributable by feature area while preserving local-first,
Rust-trusted, browser-observable boundaries.

## Source fixture

The source fixture lives in `examples/engine-expressiveness-v2-regression/`:

- `ouroforge.project.json` references the source scene, Seed, scenario pack,
  asset root, and generated-state roots.
- `scenes/expressiveness-v2.scene.json` is a deterministic one-screen scene used
  by the evidence smoke.
- `seeds/engine-expressiveness-v2-regression.yaml` contains the focused Scenario
  DSL checks.
- `scenarios/expressiveness-v2-regression.json` groups the same checks as a
  project-level scenario pack.
- `evidence-smoke.test.cjs` drives the runtime fixture, evaluates all assertions
  against in-memory evidence, writes temporary verdict evidence under the OS temp
  directory, then deletes it.

## Coverage matrix

| Feature area | Scenario id | Evidence focus | Dashboard/read-model contract | Gap note |
| --- | --- | --- | --- | --- |
| Component schema | `component-schema-v2-regression` | `world_state.componentModel.*` counts and version | Existing dashboard/cockpit world-state links stay read-only. | None. |
| Triggers/flags | `trigger-flags-v1-regression` | `world_state.componentModel.goalFlags.*`, `runtime_events.events` | Runtime events link to verdict evidence without browser-side commands. | None. |
| Multi-scene transition | `multi-scene-transition-v1-contract` | `world_state.sceneId`, `runtime_events.events`, `transition_evidence.*` | Transition evidence remains manifest-bounded and read-only in dashboard/cockpit summaries. | No arbitrary scene loading, streaming, or editor transition graph scope. |
| HUD values | `hud-values-v1-regression` | `world_state.componentModel.hudValues.*` | Dashboard/cockpit render HUD read-model state as escaped read-only data. | None. |
| Collision layers | `collision-layers-v2-regression` | `world_state.collisionRules.*`, `collision_evidence.contacts` | Contact evidence remains linked from verdicts. | None. |
| Animation event | `animation-event-v1-regression` | `animation_evidence.*`, `runtime_events.events` animation state records | Animation evidence is data-only; no visual AI judgment. | No animation graph/editor scope. |
| Audio event | `audio-event-v1-regression` | `audio_evidence.*` request metadata and intent records | Audio evidence remains intent-only; no playback assertion. | No audio engine/playback controls. |
| Playable loop | `playable-demo-loop-v2-regression` | key, door, exit, and HUD world-state evidence | Scenario verdicts link to generated evidence paths without committing runs. | None. |

## Verification

Focused source/evidence smoke:

```bash
node examples/engine-expressiveness-v2-regression/evidence-smoke.test.cjs
```

Broad read-model compatibility checks:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

From a Cargo-enabled environment, also run the normal Rust gates and validate the
new source contracts:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p ouroforge-cli -- project validate examples/engine-expressiveness-v2-regression
cargo run -p ouroforge-cli -- seed validate examples/engine-expressiveness-v2-regression/seeds/engine-expressiveness-v2-regression.yaml
```

## Generated-state policy

Scenario Coverage v3 fixtures are source-like inputs. Generated `runs/`,
`dashboard-data/`, `target/`, screenshots, temporary smoke verdicts, and local
review artifacts must remain untracked. Browser surfaces may display exported
state but must not execute commands, write trusted files, or mutate local source.

## Non-goals

This suite does not implement native export, plugin/runtime extension APIs,
marketplace features, hosted services, source-code mutation, browser-side trusted
writes, visual scripting, a production editor, public launch automation, or a
replacement for an existing general-purpose game engine. Roadmap anchors #1 and
#23 remain open by design.
