# Ouroforge Architecture

Ouroforge is organized around inspectable local artifacts rather than hidden orchestration state.

## Core loop

```text
Seed → Run → Ledger + Evidence → Scenario Results → Verdict → Journal → Mutation Proposal
```

### Seed

A Seed describes the goal, target constraints, acceptance criteria, and Scenario DSL checks. Seeds live in `seeds/` and are snapshotted into every run.

### Run

A run is a directory under `runs/` containing:

- `run.json` — run metadata;
- `seed.snapshot.yaml` — immutable Seed copy;
- `ledger.jsonl` — append-only event log;
- `evidence/index.json` — indexed evidence artifacts;
- `verdict.json` — evaluator output;
- `journal.md` — deterministic human-readable run summary;
- `mutation/proposals.json` — proposed changes for failed verdicts when applicable.

### Ledger and evidence

Ledger events explain what happened. Evidence artifacts point to files under the
run evidence tree and carry metadata for workers, scenarios, screenshots,
runtime probes, and scenario results. After Foundation Hardening v1, ledger IO
lives in `ouroforge-ledger` and evidence artifact/index IO lives in
`ouroforge-evidence`; `ouroforge-core` re-exports those contracts for existing
callers.

### Crate boundary after Foundation Hardening v1

Foundation Hardening v1 (#1301-#1306) realized the first Suggested Repository
Structure seams without changing behavior or serialization. The current local
core stack is deliberately acyclic:

```text
ouroforge-ledger <- ouroforge-evidence <- ouroforge-evaluator <- ouroforge-core <- ouroforge-cli
```

- `ouroforge-ledger` owns the append-only event/record log helpers.
- `ouroforge-evidence` owns evidence artifact models, path validation, index IO,
  and artifact registration.
- `ouroforge-evaluator` owns verdict models, mechanical scenario evidence
  checks, explicit console/performance evaluator checks, runtime invariant
  evaluation, visual and semantic gate evaluation, gate aggregation, and
  top-level run verdict orchestration.
- `ouroforge-core` remains the harness/runtime/orchestration crate and keeps a
  re-export facade so existing public paths remain behavior-compatible. Its
  `evaluate_run` entry point is a facade plus behavior-runtime adapter, avoiding
  an `ouroforge-evaluator -> ouroforge-core` dependency cycle.

The extraction reduced `crates/ouroforge-core/src/lib.rs` from approximately
89k lines at the start of the milestone to 89,047 lines after #1305, while
creating `ouroforge-ledger` (96 lines), `ouroforge-evidence` (130 lines), and
`ouroforge-evaluator` (2,960 lines). Golden parity remained byte-identical, so
this was an architectural hygiene milestone only: no runtime behavior, public
capability, serialization, production-readiness, or Godot-replacement claim
changed. Mutation, evolve, runtime, behavior, and seed clusters intentionally
remain in `ouroforge-core` for a possible later A.H2.

### Browser/runtime boundary

The local MVP uses Chrome DevTools Protocol against local runtime pages. Browser workers capture screenshots and runtime probe JSON. This is local development automation, not a hosted sandbox.

### Scenario DSL and evaluator

Scenario DSL steps drive the runtime probe API, capture world/frame state, and
emit scenario result artifacts. The `ouroforge-evaluator` crate reads evidence
and scenario results to produce deterministic pass/fail/pending verdicts,
including the bounded mechanical, runtime, visual, and semantic gate categories
from Evaluator Depth v1. Rust/local validation owns trusted gate logic and
verdict serialization; browser and Studio surfaces inspect exported evidence
only.

### Journal and mutation proposals

The journal renders the Seed, observed evidence, verdict, and next mutation. Failed verdicts can create evidence-linked mutation proposals. Mutation proposals are records only; they do not apply edits automatically.

## UI boundaries

### Evidence dashboard

`examples/evidence-dashboard` is a read-only static UI over exported dashboard data. It does not mutate run artifacts.

### Authoring cockpit

`examples/authoring-cockpit` is a minimal static cockpit for the current game-runtime scene. Browser edits are in-memory and show the Rust-validated `ouroforge scene edit` command. Direct browser file writes are intentionally not supported.

## Scene edit model

Rust owns scene read/edit validation. Supported edit paths are deliberately minimal:

- `sprite.color`
- `components.transform.x`
- `components.transform.y`
- `components.velocity.x`
- `components.velocity.y`
- `components.size.width`
- `components.size.height`
- `components.controllable`

Unsupported broad editor concepts should become explicit future issues rather than hidden abstractions.
