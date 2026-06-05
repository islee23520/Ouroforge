# Refactor Parity Golden Baseline v1

Issue: #1302
Part of: Foundation Hardening v1 (#1301), #1 Milestone A.H.
Status: parity tooling only; no behavior, serialization, or feature change.

This baseline is the safety net created first under Foundation Hardening v1. It captures byte-level golden snapshots of representative deterministic demo-run verdict output, so every later crate extraction (#1303 `ouroforge-ledger`, #1304 `ouroforge-evidence`, #1305 `ouroforge-evaluator`) can prove it changed no runtime behavior. Each extraction is a pure mechanical refactor; if these snapshots stay byte-identical, the move preserved behavior.

## Representative runs

The baseline pins the deterministic `verdict.json` emitted by `evaluate_run` for three representative cases:

- `mechanical-pass` — a single passing scenario result; overall verdict `passed`.
- `mechanical-fail` — a single failing scenario result; overall verdict `failed`.
- `visual-gates` — a passing scenario with three declared visual acceptance comparisons (`pass`, over-threshold `fail`, and `missing-baseline`); overall verdict `failed`.

These exercise the evaluator gate logic (mechanical/runtime scenario evaluation plus the visual gate) that moves into `ouroforge-evaluator`, and the evidence models consumed from what becomes `ouroforge-evidence`.

## Golden snapshots

The committed golden snapshots are the only fixture-scoped artifacts:

```text
examples/refactor-parity-golden-v1/mechanical-pass.verdict.golden.json
examples/refactor-parity-golden-v1/mechanical-fail.verdict.golden.json
examples/refactor-parity-golden-v1/visual-gates.verdict.golden.json
```

`verdict.json` is byte-deterministic: it contains only relative evidence references, scenario states, evidence-derived reasons, and static evaluator metadata. It carries no timestamp, absolute path, or process id. (By contrast, `evidence/index.json` records an `addedAtUnixMs` timestamp and is therefore intentionally **not** part of the golden baseline.)

## Parity runner

`crates/ouroforge-core/tests/refactor_parity_golden.rs` is the runner. It runs as part of `cargo test` / `cargo test --workspace`. For each case it:

1. Builds a deterministic fixture run under the system temp dir (never the repo).
2. Calls `evaluate_run` and reads the produced `verdict.json`.
3. Regenerates the same case a second time into an independent temp run dir and asserts the two byte streams are identical — a determinism guard against any timestamp/path/process leak.
4. Diffs the bytes against the committed golden and fails on any byte difference.

The runner never "fixes" or normalizes engine output; the golden is exactly what the engine emits.

## Procedure for each extraction PR

Each Foundation Hardening extraction PR (#1303/#1304/#1305) must:

1. Keep the golden snapshots unchanged — do not edit files under `examples/refactor-parity-golden-v1/`.
2. Run `cargo test --workspace` (which includes `refactor_parity_golden`) and confirm it is green.
3. Run `cargo clippy --all-targets --all-features -- -D warnings` clean.

If the parity runner fails, the extraction changed behavior or serialization and is out of scope for a mechanical refactor; the change must be reverted or the discrepancy filed as a separate bug, never papered over by regenerating the golden.

## Regenerating the baseline

Regeneration is intentionally explicit and is only appropriate when an authorized, separately reviewed behavior change deliberately alters verdict output (never inside an extraction PR):

```bash
OUROFORGE_PARITY_CAPTURE=1 cargo test --test refactor_parity_golden
```

This rewrites the golden files from the current engine output. The capture run still asserts cross-run determinism.

## Baseline state

At baseline, `cargo test --workspace` is green and `cargo clippy --all-targets --all-features -- -D warnings` is clean.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open; this issue does not close or modify either.
