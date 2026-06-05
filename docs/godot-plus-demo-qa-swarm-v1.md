# Godot-Plus Demo QA / Playtest Swarm v1

Issue: #788
Status: **GPD12.10 QA-swarm contract.** This document records the bounded
autonomous QA/playtest swarm for the Godot-Plus Demonstration Game v1 vertical
slice (Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on #780–#787. The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## Swarm plan

`qa/qa-swarm-plan.json` (schema `demo-qa-swarm-plan-v1`) defines a bounded swarm
of up to three workers. Each worker replays a deterministic input route against
the runtime, runs to a stop condition or step budget, captures evidence, and
records a classified verdict.

| Route | Worker | Input | Budget | Stop when | Verdict | Classification |
| --- | --- | --- | --- | --- | --- | --- |
| `qa-win-path` | worker-1 | hold right | 250 | `exit_reached` | pass | `pass` |
| `qa-objective-blocked` | worker-2 | idle | 120 | budget | fail | `objective_blocked` |
| `qa-hazard-contact` | worker-3 | hold right (drone armed) | 250 | `player_alive == false` | fail | `hazard_contact_loss` |

The hazard route reuses the merged `behaviors/hazard-drone.json` archetype (#784).

## Evidence captured

Per route the swarm captures `world_state` (objective flags, scene id),
`frame_stats` (`runtimeFrameBudgetStatus`), and `runtime_events` (event count) and
records the classified verdict and the step at which it stopped. The aggregated
playtest report is written to a temp directory outside the repository and removed
before exit (generated evidence stays untracked).

## No auto-fix / no source mutation

The plan policy is explicit: `autoFix: false`, `autoApply: false`,
`sourceMutation: "forbidden"`. The smoke captures SHA-256 hashes of watched source
files (scene, plan, hazard archetype) before the swarm and asserts they are
unchanged afterward — QA produces evidence and classified verdicts only; it never
fixes, applies, or mutates source.

## Determinism

The swarm requires deterministic outcomes: the smoke reruns the hazard route and
asserts an identical classification and stop step.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs
```

The smoke runs all routes, asserts each verdict/classification, confirms the frame
budget holds, proves determinism, and proves no source mutation. It writes only to
a temp dir and fails closed on any committed generated root.

## Boundaries

The QA swarm reuses the existing runtime and demo contracts and performs no
auto-fix, auto-apply, source mutation, trusted browser write, command execution,
network access, committed generated output, production/native/store export, or full
Godot parity / replacement / production-ready / commercial-release claim. #1 and
#23 remain open.
