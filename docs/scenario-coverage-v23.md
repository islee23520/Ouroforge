# Scenario Coverage v23: Trust Gradient Regression Suite

Issue: #1483 — Scenario Coverage v23: Trust Gradient Regression Suite, under #1
Era E Milestone 22 (Trust Gradient). Authorized by the design gate
(`docs/trust-gradient-design.md`).

Scenario Coverage v23 locks Trust Gradient v1 behavior with an enumerated,
fixture-scoped regression matrix. It asserts states and shapes only — no
subjective quality and no flaky/timing assertions — so a breaking change to
risk-tier classification, bounded auto-apply, the audit log, or the kill switch
fails CI. It also pins the default-off backward-compatibility guarantee: with
autonomy off (the default), review-gated manual apply is unchanged.

## Regression matrix

| Scenario id | Coverage | Expected result |
| --- | --- | --- |
| `TG23.tier-low-eligible` | Low-risk scene-only data, source-free scope, high confidence, all gates pass, fresh refs. | tier `low`, `auto-apply-eligible`. |
| `TG23.tier-medium-manifest` | Review-required data (manifest). | tier `medium`, `manual-only`. |
| `TG23.tier-high-source` | Source-affecting kind. | tier `high`, `manual-only`. |
| `TG23.tier-high-ambiguous` | Ambiguous kind. | tier `high`, `manual-only`. |
| `TG23.tier-source-affecting-scope-overrides-kind` | Scene-only kind but a source-affecting scope path. | tier `high`, `manual-only` (scope overrides kind, fail-closed). |
| `TG23.tier-missing-confidence-manual` | Scene-only but no confidence. | `manual-only` (conservative default). |
| `TG23.apply-success` | Eligible, low, high confidence, gates pass, in budget, rollback-backed. | `auto-applied`, one-command rollback present, budget consumed. |
| `TG23.apply-rollback-present` | Eligible apply at the budget boundary. | `auto-applied`, one-command rollback present. |
| `TG23.apply-budget-exhausted` | Eligible but the risk budget is exhausted. | `manual-fallback`, no rollback command. |
| `TG23.apply-ineligible` | High-risk / manual-only proposal. | `manual-fallback`. |
| `TG23.apply-gate-regression` | Eligible but a gate fails on rerun. | `manual-fallback`. |
| `TG23.audit-complete-valid` | Append-only audit log with contiguous sequences and intact rollback handles. | valid. |
| `TG23.audit-tampered-gap-invalid` | Audit log with a sequence gap. | invalid (tamper-evident). |
| `TG23.audit-broken-rollback-invalid` | Audit entry with an empty rollback ref. | invalid. |
| `TG23.audit-kill-switch-engaged-halts` | Audit log with the kill switch engaged and a reason. | valid and autonomy halted. |
| `TG23.backward-compat-autonomy-off` | Otherwise-eligible proposal with autonomy off (default). | `manual-fallback` — review-gated manual apply unchanged. |

## Files

- `examples/trust-gradient-v1/coverage-v23/coverage-matrix.json` — the enumerated matrix.
- `crates/ouroforge-core/tests/scenario_coverage_v23_trust_gradient.rs` — the
  trusted Rust guard (owns the regression gate).
- `examples/trust-gradient-v1/scenario-coverage-v23-trust-gradient.test.cjs` — the
  browser-facing demo mirror.

## Running

```bash
cargo test -p ouroforge-core --test scenario_coverage_v23_trust_gradient
node examples/trust-gradient-v1/scenario-coverage-v23-trust-gradient.test.cjs
```

The Rust test reads the shared `coverage-matrix.json` and drives each case through
the production `classify_mutation_risk_tier`, `decide_auto_apply`, and
`AutoApplyAuditLog::validate` paths, so a breaking change to any trusted Trust
Gradient path fails CI — the regression guard is Rust-owned. The Node runner
mirrors the same matrix for the browser-facing demo (asserting states and shapes
only); because a Node reimplementation cannot catch a Rust regression, it is a
documentation mirror, not the trusted gate. The backward-compatibility golden
re-decides the autonomy-off request and requires manual fallback, pinning the
default-off guarantee without timing assertions.

## Boundary

Coverage asserts states and shapes only; it adds no subjective quality score and
no flaky/timing assertions. The default-off autonomy posture is provably
unchanged. Bounded, reversible, audited autonomy is not auto-merge,
self-approval, or a quality/production-ready guarantee. Rust/local owns
persistence and serialization; fixtures are fixture-scoped. #1 and #23 remain
open.
