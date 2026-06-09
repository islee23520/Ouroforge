# Scenario Coverage v66 — Proposal Amendment and Re-Verify

Coverage v66 locks the Era M/M75 Proposal Amendment and Re-Verify regression surface for amend-before-approve intervention.

Required invariants covered:

- Human amendments are recorded as `intervention-as-evidence` and routed through existing gates before review/apply readiness.
- Studio remains `read + gated-write`; it captures/routes but does not own artifact semantics or perform trusted artifact writes.
- The suite rejects `no raw-bypass` regressions: no raw write, no trusted Studio write, no auto-apply, and no bypass token in evidence.
- The autonomous loop completes without human input; human intervention remains opt-in and never mandatory.
- Rust = data plane for artifact truth, validation, determinism, evaluator evidence, provenance, and gate decisions.
- Elixir/OTP + Phoenix LiveView = control + presentation for local single-user capture and rendering only.
- CLI fallback remains intact, hosted/multi-user Studio remains deferred, and #1 and #23 remain open.

Verification anchors:

- `crates/ouroforge-core/tests/scenario_coverage_v66_proposal_amendment.rs`
- `studio/executor/test/ouroforge_executor/scenario_coverage_v66_test.exs`
