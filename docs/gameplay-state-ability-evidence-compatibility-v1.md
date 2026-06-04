# Gameplay State/Ability Evidence Compatibility v1

Issue: #614 — State Machine and Ability Action Model v1. This document defines
GL10.4.3 evidence/read-model compatibility for
[`Gameplay State Machine v1`](gameplay-state-machine-v1.md) and
[`Gameplay Ability Action v1`](gameplay-ability-action-v1.md).

The compatibility contract exposes read-only summaries for state-machine and
ability/action status so scenario, dashboard, Studio, and probe consumers can
reference state and ability status without interpreting executable scripts. It is
not runtime dispatch, arbitrary script execution, a production-stable scripting
API, a plugin loader, a command bridge, browser trusted writes, source apply,
auto-apply, auto-merge, self-approval, native export, hosted service, or a
current Godot replacement claim.

## Read-model schemas

- `gameplay-state-machine-read-model.v1` summarizes state pack status, machine
  counts, state ids, transition ids, initial states, target refs, trigger kinds,
  guard kinds, action kinds, linked evidence refs, blocked reasons, and an
  explicit read-only boundary.
- `gameplay-ability-action-read-model.v1` summarizes ability pack status,
  ability/action ids, runtime status counts, target refs, trigger kinds, effect
  kinds, cost kinds, linked evidence refs, blocked reasons, and an explicit
  read-only boundary.

Both read models are generated from validated fixture/source-like artifacts and
remain display/evidence data. They do not write trusted state, dispatch actions,
change runtime state, or apply source mutations.

## Compatibility fixture

[`examples/gameplay-state-ability-evidence-v1/read-model-compatibility.fixture.json`](../examples/gameplay-state-ability-evidence-v1/read-model-compatibility.fixture.json)
records the expected compatibility surface between the two read models:

- state read-model refs can point at state-machine fixtures;
- ability read-model refs can point at ability/action fixtures;
- consumers must show missing or malformed read models visibly rather than
  inferring successful runtime behavior;
- browser/dashboard/Studio consumers remain read-only or draft-only for trusted
  state.

## Malformed and missing behavior

Malformed or missing state/ability artifacts fail at Rust/local validation before
read-model generation. Consumers should preserve explicit missing/malformed
status and linked evidence references instead of treating stale or absent files as
present runtime behavior.

## Non-goals

GL10.4.3 does not authorize arbitrary JS/Rust/Python/Lua/WASM execution, `eval`,
dynamic import, production-stable scripting APIs, secure-sandbox claims, native
export, plugin runtime, hosted cloud/server/auth behavior, source apply,
auto-merge, auto-apply, self-approval, public launch automation, shipped-game
maturity, or current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
