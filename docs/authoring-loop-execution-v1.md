# Authoring Loop Execution v1

Authoring Loop Execution v1 adds an explicit Rust-trusted step runner for Authoring Loop Plan v1 artifacts.

The runner is intentionally bounded:

- CLI-only: `cargo run -p ouroforge-cli -- loop step <plan> --step <step-id>`.
- One explicit step at a time.
- Validates the plan and prerequisites before write-producing steps.
- Updates step status through the plan state machine.
- Records loop ledger events under generated `runs/authoring-loop-ledgers/<loop-id>/ledger.jsonl`.
- Emits a JSON `authoring-loop-step-execution-v1` summary for review surfaces.

## Supported step boundary

The runner reuses existing Rust functions rather than shelling out:

- `run-scenario-pack` creates project-bound run artifacts.
- `compare-runs` writes an existing run comparison artifact.
- `generate-proposal` invokes the deterministic proposal path for failed runs.
- `apply-accepted-scene-mutation` uses the existing review-gated scene-only apply path and requires a concrete accepted `reviewDecisionId`.
- `rerun` creates a transaction-bound rerun artifact.
- `promote-regression` requires accepted regression-promotion decision evidence before writing promotion output.

`record-review-decision` remains a manual/review step. The runner stops rather than fabricating review decisions.

## Browser and Studio boundary

Dashboard and cockpit surfaces may display `loop_execution` / `loopExecution` summaries as escaped read-only evidence. They do not execute loop steps, write plans, apply mutations, record decisions, promote regressions, run schedulers, or bridge browser actions to shell commands.

## Generated state

Loop ledgers, runs, comparisons, transactions, proposal artifacts, promotion records, and dashboard exports are generated local state. Keep them untracked unless a later issue explicitly creates deterministic fixture-scoped placeholders.

## Deferred work

Resume/retry semantics, evidence bundles, handoff contracts, and Studio loop cockpit workflows remain deferred to #307+.
