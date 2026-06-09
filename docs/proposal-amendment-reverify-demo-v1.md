# Proposal Amendment and Re-Verify Demo v1

Issue: #2055 — Era M, Milestone 75

Status: **scripted demo** for amend-before-approve.

## Demo paths

The demo covers two paths:

1. **Autonomous default** — no human intervention is supplied, the loop completes
   without the Studio surface, and CLI fallback remains valid.
2. **Human amendment** — the local Studio control/presentation surface captures a
   human edit as intervention-as-evidence, routes it to
   `ouroforge proposal-amendment validate`, and reports review/apply readiness
   only after review/apply, scene/source-apply, evaluator, and design-integrity
   gates pass.

A blocked variant proves failed re-verification keeps blocker evidence visible and
does not perform a raw apply.

## Invariant shown

Every human write in the demo is read + gated-write: captured by Elixir/Phoenix
control + presentation, validated by the Rust data plane, recorded with before /
after / provenance refs, and routed through existing gates. The demo performs no
trusted artifact writes, no raw bypass, no auto-apply, and no hosted or
collaborative Studio behavior.

Run with:

```bash
cd studio/executor
mix test --only demo test/ouroforge_executor/proposal_amendment_demo_test.exs
```
