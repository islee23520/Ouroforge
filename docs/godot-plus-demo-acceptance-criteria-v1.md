# Godot-Plus Demo Acceptance Criteria v1

Issue: #780
Status: **GPD12.3.2 acceptance contract only.** This document defines the
measurable, testable acceptance criteria matrix for the Godot-Plus Demonstration
Game v1 vertical slice described in `docs/godot-plus-demo-gdd-v1.md`. It does not
implement gameplay, assets, QA, export, source apply, plugins, or Studio writes,
and it does not modify or close #1/#23.

## Canonical target

Acceptance is evaluated against the canonical fixture
`examples/playable-demo-v2/collect-and-exit/` and its scenario pack
`scenarios/collect-and-exit.json` (Seed `seeds/collect-and-exit.yaml`). The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used for acceptance.

## How to read this matrix

Each criterion is **testable**: it names a concrete pass signal and the evidence
source (a scenario assertion, a Node smoke, a Rust CLI/validation artifact, or a
read-only read model). "Verifying issue" is the later issue that owns producing
that evidence; #780 only fixes the contract.

## Acceptance criteria matrix

| # | Criterion | Pass signal (measurable) | Evidence source | Verifying issue | Boundary |
| --- | --- | --- | --- | --- | --- |
| A1 | **Gameplay loop complete** | `goalFlags.key_collected = true`, `goalFlags.door_open = true`, and `goalFlags.exit_reached = true` are reached on the win path | `collect-key-hud-contract` scenario assertions + `e2e-smoke.test.cjs` | #782, #783 | Vertical slice only; no full game |
| A2 | **Deterministic scenario pass** | The scenario pack evaluates the same verdict on repeated runs from source inputs (no flakiness, no network) | `scenarios/collect-and-exit.json` via runtime smoke | #787, #788 | Deterministic local evidence; no hosted QA |
| A3 | **Deterministic failure path** | A blocked-gate run (no key) leaves `door_open = false` / `exit_reached = false`; (planned #784) hazard contact sets `player_alive = false` with a non-pass verdict | Failure scenario result + journal/dashboard context | #784, #787 | Failure evidence never auto-applies a fix |
| A4 | **Objective visible (HUD/state)** | `componentModel.hudValues` exists and key/goal/health states are inspectable; `metadata.title` and `startState.checkpointSlot` are present | scenario `world_state` assertions + read-model smoke | #785 | Browser/Studio surfaces read-only or draft-only |
| A5 | **Animation + audio intent evidence** | `animation_evidence.0.mode = "sprite_frame"` and `audio_evidence.0.name = "player_spawn"` are produced | scenario assertions + runtime evidence | #782, #785, #786 | Intent metadata only; no shipped audio/video mix |
| A6 | **Frame / performance budget** | `runtimeFrameBudgetStatus = within-budget` against the scene `runtimeDebug.frameBudget` defaults | `frame_stats` scenario assertion | #794 | Bounded demo budget; not a production profiler claim |
| A7 | **Save / load (start-state) evidence** | A `runtime.save.loaded` event is recorded after restoring the `demo-start` checkpoint | `runtime_events` scenario assertion + `e2e-smoke.test.cjs` | #782 | Save artifact is browser-observable evidence; trusted persistence stays Rust/local |
| A8 | **Studio-inspectable** | Dashboard and Studio read models render the demo evidence shape without trusted writes | `evidence-read-model-smoke.test.cjs`, `asset-evidence-smoke.test.cjs` | #790 | Read-only/draft-only; no command bridge, no trusted browser write |
| A9 | **Exportable / packageable** | A local web package/bundle smoke or checksum artifact verifies locally | export/package verification artifact | #791 | No native/mobile/console/store export, signing, publishing, or deployment |
| A10 | **Evidence-backed (not screenshot-only)** | Every accepted claim links to scenario verdicts, run ids, or read-model evidence refs (recorded in PR/issue, not committed as generated state) | PR/issue evidence + scenario results | #788, #789 | Generated runs/dashboards stay untracked unless fixture-scoped |
| A11 | **Review-gated source apply** | Any agentic source change is represented as draft → independent review decision → Safe Source Mutation Apply transaction → rollback metadata → rerun comparison | source-apply transaction + review decision evidence | #789, #790 | No self-approval, auto-apply, auto-merge, reviewer bypass, or hidden trusted writes |
| A12 | **Plugin descriptor usage (inert)** | The demo references a validated, inert plugin/extension descriptor displayed as escaped read-only metadata | plugin registry/descriptor evidence | #792 | No executable plugin runtime, marketplace, or network install/update |
| A13 | **Asset-reference integrity** | `asset-manifest.json` validates and runtime asset refs resolve to manifest ids with stable hashes | `asset validate` CLI + `asset-evidence-smoke.test.cjs` | #786 | Tiny deterministic fixtures only; no asset upload/fetch/import |
| A14 | **Conservative comparison / wording** | Comparison docs name supported / unsupported / local-only rows and avoid replacement/parity/production claims | comparison matrix doc | #793 | No broad Godot superiority claim |
| A15 | **Governance preserved** | Final evidence confirms #1 and #23 remain open and repeats the no-overclaim/no-production/no-replacement boundary | issue comments + verification commands | #797 | #1/#23 not modified or closed without a separate explicit decision |

## Verification commands (contract)

Implementation issues prove the criteria with the established checks (run from a
full toolchain):

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs
git diff --check
```

## Non-acceptance examples

The vertical slice is **not** accepted by: screenshots alone; a design document
without scenario evidence; a manually played run without reproducible inputs; an
unreviewed source mutation; a browser-side trusted write; an executable plugin; a
hosted deployment; a native/mobile/store package; or any language claiming full
Godot replacement, full Godot parity, production-ready engine/editor, secure
sandbox, or commercial release readiness.
