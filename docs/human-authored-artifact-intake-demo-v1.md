# Human-Authored Artifact Intake Demo v1

Issue: #2059 — Era M, Milestone 76

This fixture-scoped demo records the human-authored artifact intake path without adding a new data store, write path, hosted Studio, or Elixir-owned artifact semantics.

Demo cases:

- **Autonomous default**: no human-authored artifact is supplied, the loop completes without human input, and CLI fallback remains sufficient.
- **Gated human intake**: local Studio captures a human-authored card candidate as intervention-as-evidence, then routes it to the Rust `human-artifact-intake validate` path.
- **Blocked human intake**: a failed evaluator gate blocks review/apply readiness while preserving evidence and avoiding raw apply.

Invariant evidence:

- Studio is read + gated-write only.
- Every human write is routed through review/apply, scene/source-apply, evaluator, and evidence/provenance gates.
- Rust = data plane; Elixir/OTP + Phoenix LiveView = control + presentation.
- Human intervention remains optional; the autonomous loop completes without human input.
- #1 and #23 remain open.
