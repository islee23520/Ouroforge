# Foundation Hardening v1 Scope and Contract

Issue: #1301
Roadmap anchor: #1 Milestone A.H (Roadmap Alignment Addendum).
Status: complete after #1306 governance refresh; mechanical refactor only, no executable behavior added.

Foundation Hardening v1 was the architectural-hygiene milestone that decomposed the ~89k-line `crates/ouroforge-core/src/lib.rs` toward the Suggested Repository Structure crate seams. It is a mechanical refactor only: it moves cohesive type/function clusters into new crates, preserves the public API via re-export, and changes no runtime behavior. The primary acceptance gate is that golden run evidence/verdict bytes are unchanged.

v1 extracts three crates (`ouroforge-ledger`, `ouroforge-evidence`, `ouroforge-evaluator`); other clusters (mutation, evolve, runtime, behavior, seed) remain in `ouroforge-core` for a later A.H2. It does NOT add features, change behavior, alter serialization, fix bugs opportunistically, or introduce a dependency cycle. Discovered bugs must be filed as separate issues, not fixed inside an extraction PR.

This document remains the canonical contract and closure record for the v1 extraction sequence. It added no executable behavior.

## Target crates and contents

- `ouroforge-ledger`: the append-only event/record log, including Journal records.
- `ouroforge-evidence`: evidence artifact models and IO.
- `ouroforge-evaluator`: evaluate/verdict/scenario/visual/semantic gate logic.

## Acyclic dependency direction

The dependency direction is strictly acyclic, bottom to top:

```text
ouroforge-ledger <- ouroforge-evidence <- ouroforge-evaluator <- ouroforge-core <- ouroforge-cli
```

Each extraction PR moves exactly one crate and must not introduce a dependency cycle.

## Re-export facade rule

`ouroforge-core` re-exports moved types (`pub use ...`) so the public API and the import paths used by `ouroforge-cli` and tests stay behavior-compatible. Downstream churn is limited to import paths; no consumer signature changes.

## Golden-parity gate

Representative demo run evidence/verdict snapshots must be byte-identical before and after each extraction. The full `cargo test --workspace` stays green and `cargo clippy --all-targets --all-features -- -D warnings` stays clean. The golden baseline established by #1302 is the safety net that every extraction PR must preserve.

## Bounded v1 scope

v1 is bounded to exactly three crates: `ouroforge-ledger`, `ouroforge-evidence`, and `ouroforge-evaluator`. The following clusters explicitly remain in `ouroforge-core` for a later A.H2 and are out of scope for v1:

- mutation
- evolve
- runtime
- behavior
- seed

## Follow-up issue sequence

The extractions are strictly sequential (they touch the same `lib.rs`/workspace and must each preserve golden parity); they do not run in parallel.

1. Foundation Hardening v1 Scope and Contract (this issue) — #1301
2. Refactor Parity Golden Baseline v1 — #1302
3. Extract ouroforge-ledger crate — #1303
4. Extract ouroforge-evidence crate — #1304
5. Extract ouroforge-evaluator crate — #1305
6. Roadmap and #1 Governance Refresh after Foundation Hardening v1 — #1306

```text
#1301 scope -> #1302 golden baseline -> #1303 ledger -> #1304 evidence -> #1305 evaluator -> #1306 governance
```

## Verification and closure gates

This was a scope/contract issue and added no executable behavior. PR evidence included the canonical `docs/foundation-hardening-v1.md`, the crate boundary / dependency-direction / re-export / golden-parity definitions, the bounded-v1 cluster list, and a #1/#23 governance audit. Each follow-up extraction PR kept `cargo test --workspace` green, kept `cargo clippy --all-targets --all-features -- -D warnings` clean, preserved the #1302 golden parity byte-for-byte, and moved one bounded crate slice. #1306 recorded the final governance refresh after #1305 closure.

## Explicit non-goals

- No feature addition, behavior change, serialization change, or `while we're here` logic/signature change.
- No opportunistic bug fixing inside an extraction PR; discovered bugs are filed as separate issues.
- No dependency cycle; each extraction PR moves exactly one crate.
- No extraction of the mutation/evolve/runtime/behavior/seed clusters in v1 (reserved for A.H2).
- No browser trusted writes; the browser remains read-only for trusted state.
- No generated runs/artifacts tracked unless explicitly fixture-scoped.
- No new capability, production-ready, or current Godot replacement claim.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.


## Completion record after #1306

Foundation Hardening v1 completed through the planned sequence: #1301 scope,
#1302 golden parity baseline, #1303 `ouroforge-ledger`, #1304
`ouroforge-evidence`, #1305 `ouroforge-evaluator`, and #1306 governance. The
realized dependency direction remains acyclic:

```text
ouroforge-ledger <- ouroforge-evidence <- ouroforge-evaluator <- ouroforge-core <- ouroforge-cli
```

At the #1306 audit, `crates/ouroforge-core/src/lib.rs` measured 89,047 lines,
while the extracted crates measured 96 lines (`ouroforge-ledger`), 130 lines
(`ouroforge-evidence`), and 2,960 lines (`ouroforge-evaluator`). Golden run
verdict parity remained byte-identical via `refactor_parity_golden`, full
workspace tests and clippy passed, and #1/#23 remained open. The recommended
next hardening direction is a separately scoped A.H2 candidate for one of the
remaining `ouroforge-core` clusters (mutation/evolve, runtime, behavior, or
seed); this completion does not authorize that work by itself.
