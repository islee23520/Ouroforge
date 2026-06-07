# Deck-Roguelike Game Class v1

Deck-Roguelike Game Class v1 is a scope and contract document for the next
genre rung under #1 Era F Milestone 31: a deck roguelike. Climbing this rung
requires deterministic randomness, so this document defines two coupled
contracts — the seeded stochastic determinism contract and the deck-roguelike
game-class contract — and pins both to existing runtime, probe, and
replay-digest surfaces.

This document adds no executable behavior, fixtures, runtime features, Studio
surfaces, browser authority, or engine capability. It is a governance contract
for the follow-up implementation issues listed at the end. The current runtime
is deterministic only because it contains no randomness; a roguelike needs
seeded randomness that still preserves replay and regression. This contract
defines how that randomness is introduced without weakening determinism.

## Relationship to the Milestone 24 ladder

This rung records under the Game Complexity Ladder
([`game-complexity-ladder-v1.md`](game-complexity-ladder-v1.md)). The ladder's
rung rule, evidence gates, engine-growth demand rule, and #1/#23 governance
audit apply unchanged. Deck-Roguelike Game Class v1 is a demand-driven rung:
it may be claimed only after a loop-produced demo proves the class with
four-gate evidence and a Milestone 20 loop-coverage verdict, exactly as the
ladder requires. This document does not claim the rung; it scopes the contracts
the rung's follow-up issues must satisfy.

The deck-roguelike rung sits above the ladder's structural rungs (collect-and-exit
through multi-scene objective game) because it adds a new *capability axis* —
seeded stochastic state — rather than new spatial breadth. It does not authorize
renderer, physics, audio, or animation breadth; card/relic/run state is data and
deterministic logic, not new engine depth.

## Scope

The contract applies to:

- the seeded stochastic determinism contract for any randomness the runtime
  introduces;
- the deck-roguelike game-class contract (cards, relics, runs) as
  probe-exposed, seed-reproducible state;
- how snapshot/restore and replay-digest divergence detection behave once
  randomness is seeded;
- how the rung records under the Milestone 24 ladder;
- the dependency order and closure gates for the follow-up issues.

The contract governs game-class claims, roadmap wording, runtime/probe/replay
ownership, and future engine-growth requests connected to the deck-roguelike
rung. A contract does not authorize implementation by itself; each follow-up
issue must still scope concrete Rust/local changes and verification.

## Non-goals

Deck-Roguelike Game Class v1 does not authorize:

- a new engine, runtime, writer, or parallel game loop; the deck-roguelike class
  must extend the existing runtime, probe, and replay-digest surfaces;
- unseeded randomness, wall-clock seeding, ambient entropy, or any randomness
  that cannot be reproduced from an explicit seed;
- direct trusted writes from generation or any browser/Studio surface; proposals
  flow only through the existing review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- hosted/cloud/paid capability, a marketplace transaction layer, or
  distributed/Elixir orchestration (Layer-3; DEFER per #1508, NO-GO per ADR #92,
  [`distributed-elixir-design.md`](distributed-elixir-design.md));
- new renderer, physics, audio, animation, or 3D breadth beyond what this rung's
  gate justifies;
- committing generated runs, genre, evidence, or registry artifacts unless they
  are explicitly fixture-scoped;
- any claim of a production-ready engine, Godot replacement or parity, or that
  generated games are good, fun, or shippable;
- closing, modifying, replacing, or narrowing #1 or #23 without a separate
  explicit governance decision.

## Seeded stochastic determinism contract

The deck-roguelike rung introduces randomness for the first time. To preserve
replay and regression, randomness must be deterministic with respect to an
explicit seed.

### Seed authority

- Every run that uses randomness MUST derive all of its randomness from a single
  explicit run seed. The seed is part of the run's declared inputs, not an
  ambient or implicit value.
- A run MUST NOT read randomness from wall-clock time, host entropy,
  `Math.random()`, process state, or any source that is not reproducible from
  the declared seed.
- The seed is recorded in the run's evidence and probe state so a reviewer can
  reproduce the run from inputs alone.

### Determinism guarantee

- Identical seed plus identical declared inputs plus identical runtime version
  MUST produce an identical run: the same sequence of stochastic outcomes and
  the same replay-state digest at every compared frame.
- Stochastic draws MUST come from an explicit, seeded pseudo-random stream whose
  advancement is part of the runtime's deterministic state. The stream position
  is captured by snapshot/restore and contributes to the replay-state digest, so
  a divergence in random advancement is detected like any other state divergence.
- The digest contract is unchanged: the runtime continues to use the existing
  `runtime-replay-digest-v1` schema with `fnv1a64-canonical-json-v1` 16-character
  hex values (`examples/game-runtime/runtime.js`, `replayStateDigest`). The
  seeded random stream's state is included in the digested runtime state; no new
  digest scheme is introduced.

### Reproducibility evidence

- A seeded run MUST be reproducible from `{ seed, declared inputs, runtime
  version }` alone, with no hidden state.
- The probe MUST expose the active seed and the current random-stream position
  read-only, so browser-local inspection can confirm reproducibility without any
  trusted write.
- Generated run digests and divergence reports remain generated evidence,
  untracked unless explicitly fixture-scoped, consistent with the existing
  `evidence/runtime-state/replay/` retention note.

## Deck-roguelike game-class contract

The deck-roguelike game class is defined as seeded, probe-exposed, deterministic
state on top of the existing runtime. It is data and deterministic logic, not a
new engine.

### Class shape

A deck-roguelike run, at minimum, exposes:

- **Cards** — a declared card pool and a player deck; draw, play, and discard are
  deterministic transformations of runtime state driven by the seeded random
  stream (for example, shuffles and draws).
- **Relics** — declared persistent run modifiers that alter rules or outcomes;
  relics are deterministic state, observable through the probe.
- **Runs** — a bounded sequence of encounters/choices with an explicit
  start, progression, and win/loss terminal state, all reconstructible from the
  seed and declared inputs.

### Reuse and ownership boundary

- The game class MUST be implemented by extending the existing runtime, probe,
  and replay-digest surfaces. No parallel engine, runtime, or writer is
  introduced.
- Rust/local owns trusted validation, persistence, the class/registry/telemetry
  logic, evidence and provenance writing, run/project binding, and the
  review/apply/trust-gradient path.
- TypeScript/JavaScript owns the deterministic runtime, the `window.__OUROFORGE__`
  probe, and browser-local read-only inspection. Browser/Studio surfaces stay
  read-only; they never perform a trusted write or apply.
- Card/relic/run state MUST be seed-reproducible and probe-exposed so scenario
  assertions can distinguish run progress, blocked/illegal actions, and terminal
  outcomes deterministically.

### Probe exposure

- The probe exposes deck-roguelike state (deck, hand, discard, relics, run
  progress, seed, random-stream position) read-only for inspection and scenario
  assertion. Probe exposure adds observation, not authority: it never mutates
  trusted state and never bridges to a command surface.

## Snapshot/restore and replay-digest divergence with seeded randomness

Seeded randomness must round-trip through the existing snapshot and replay
machinery without a new mechanism.

- **Snapshot/restore** — the seeded random-stream position is part of captured
  runtime state. `snapshot()` captures it and `restore(snapshotId)` restores it,
  so a restored run continues drawing the identical sequence it would have drawn
  without the snapshot. Snapshot/restore therefore preserves the determinism
  guarantee across save/load.
- **Replay-digest divergence** — because the random-stream position is part of
  the digested runtime state, any divergence in stochastic advancement surfaces
  through the existing `replayStateDigest` / `runtime-replay-divergence-v1`
  comparison (`examples/game-runtime/runtime.js`). A mismatch between the
  expected and actual digest at a frame is reported as a first-divergence with
  its frame id and tick, exactly as for non-stochastic state. No separate
  randomness-divergence path is added.
- **Regression** — seed-pinned fixtures let the regression suite assert that a
  given seed yields a stable digest across the run, so reordering or
  miscounting random draws is caught as a digest divergence rather than silently
  passing.

## Dependency order and closure gates

The Milestone 31 follow-up chain is:

```text
#1599 scope -> #1600 -> #1601 -> #1602 -> #1603 -> #1604
```

1. **#1599 — Deck-Roguelike Game Class v1 Scope and Contract** (this issue).
2. **#1600 — Seeded Stochastic Determinism v1** — the seeded random-stream
   implementation and reproducibility evidence.
3. **#1601 — Deck-Roguelike Game Class and Runtime v1** — the card/relic/run
   state on top of the seeded stream and existing runtime.
4. **#1602 — Deck-Roguelike Game Class Demo v1** — a loop-produced demo proving
   the class with four-gate evidence.
5. **#1603 — Scenario Coverage v31: Deck-Roguelike Game Class Regression Suite**
   — seed-pinned regression coverage; continue Scenario Coverage numbering from
   Era E onward.
6. **#1604 — Roadmap and #1 Governance Refresh after Deck-Roguelike Game Class
   v1** — records the rung outcome and reconfirms anchors.

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged, or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one bounded contract, capability, demo, coverage suite, or
  governance step — independently verifiable behavior is not combined into one
  PR;
- all randomness is seeded and replay-stable; no unseeded randomness is added;
- any engine growth cites the specific rung gate and evidence gap per the
  ladder's engine-growth demand rule;
- four-gate evidence and a Milestone 20 loop-coverage verdict are present for any
  rung claim;
- generated-state and trust-boundary audits pass: run outputs, dashboard
  exports, and local tool state stay untracked unless explicitly fixture-scoped;
- public wording stays conservative — no auto-merge, quality, fun, production, or
  Godot-replacement claim;
- latest-main verification before issue closure;
- #1 and #23 are checked and remain open anchors.

## Governance audit for #1 and #23

#1 remains the broad vision and roadmap anchor. #23 remains the repo-memory and
design-context anchor. This contract preserves both as open anchors and does not
modify, close, replace, or narrow either issue. Any future work that proposes to
change either anchor must be a separate explicit governance decision that
identifies the replacement source of truth, records maintainer approval, and
does not imply this contract authorized the change.
