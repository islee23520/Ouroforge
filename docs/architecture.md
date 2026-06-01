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

Ledger events explain what happened. Evidence artifacts point to files under the run evidence tree and carry metadata for workers, scenarios, screenshots, runtime probes, and scenario results.

### Browser/runtime boundary

The local MVP uses Chrome DevTools Protocol against local runtime pages. Browser workers capture screenshots and runtime probe JSON. This is local development automation, not a hosted sandbox.

### Scenario DSL and evaluator

Scenario DSL steps drive the runtime probe API, capture world/frame state, and emit scenario result artifacts. The evaluator reads evidence and scenario results to produce deterministic pass/fail/pending verdicts.

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
