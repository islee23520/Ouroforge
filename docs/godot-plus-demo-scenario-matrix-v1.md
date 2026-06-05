# Godot-Plus Demo Scenario Matrix v1

Issue: #787
Status: **GPD12.10 scenario matrix contract.** This document records the
acceptance-to-evidence matrix for the Godot-Plus Demonstration Game v1 vertical
slice (Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the GDD (#780),
scaffold (#781), core loop (#782), level set (#783), behavior (#784), HUD (#785),
and asset pack (#786). The legacy `examples/godot-plus-demo-v1/` tree is
superseded and is not used. #1 and #23 remain open.

## Matrix artifact

The matrix lives at:

```text
examples/playable-demo-v2/collect-and-exit/scenarios/demo-scenario-matrix.json
```

It is registered in `ouroforge.project.json` as scenario pack
`collect-and-exit-demo-scenario-matrix`. The artifact is read-only policy data for
QA/regression planning; it does not execute commands, mutate source, publish
builds, install plugins, or grant auto-fix authority.

## Covered scenarios

The matrix maps each required demo acceptance area to pass/fail criteria,
fixture/evidence references, and blocked actions:

| Scenario | Acceptance area | Primary evidence |
| --- | --- | --- |
| `demo-start-game` | start game | `e2e-smoke.test.cjs`, world-state + frame budget |
| `demo-move-player` | move player | deterministic input/world-state evidence |
| `demo-complete-level` | complete level | key/door/exit goal flags |
| `demo-fail-restart` | fail/restart | hazard lose state + explicit restart-evidence requirement |
| `demo-enemy-interaction` | enemy interaction | hazard drone dormant/armed/contact evidence |
| `demo-objective-update` | objective update | HUD/goal flag binding evidence |
| `demo-ui-state` | UI state | read-only HUD/dashboard evidence |
| `demo-runtime-probe-state` | runtime probe state | runtime-state-v1 level/entity/objective evidence |
| `demo-export-smoke` | export smoke | local web export/profile/package metadata only |
| `demo-studio-walkthrough` | Studio walkthrough | dashboard/cockpit read-only inspection checks |
| `demo-plugin-validation` | plugin validation | inert descriptor + Rust plugin tests |
| `demo-evidence-bundle` | evidence bundle | asset/read-model/matrix/governance evidence |

## Scenario fixtures

The scoped fixtures are existing canonical v2 files: the source scenario pack,
scene, level set, hazard behavior, HUD model, asset manifest/provenance, local
export profile/package metadata, and inert plugin descriptor. The matrix smoke
verifies every fixture reference resolves and every scenario has pass criteria,
fail criteria, evidence expectations, fixture refs, and verification refs.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs
```

The smoke proves the 12 required acceptance areas are covered, expected evidence
is explicit, fixture references resolve, forbidden actions are inherited from the
matrix guardrail list, QA swarm usage remains read-only planning, and conservative
wording is preserved.

## Boundaries

The matrix does **not** add QA swarm execution, autonomous fixes, direct trusted
Studio source writes, source mutation bypass, auto-apply, auto-merge,
self-approval, executable plugin runtime, marketplace/network plugin install,
public deployment, native/mobile/console export, signing, app-store/Steam/itch
publishing, dependency install/update, CI/workflow mutation, browser command
bridge, arbitrary shell execution, or a full Godot parity/replacement /
production-ready / commercial-release claim. #1 and #23 remain open.
