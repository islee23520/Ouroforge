# Trust Gradient Demo v1

Issue: #1481 — Trust Gradient Demo v1, under #1 Era E Milestone 22 (Trust
Gradient). Authorized by the design gate (`docs/trust-gradient-design.md`).

This is a deterministic, fixture-scoped demonstration of bounded, reversible,
audited auto-apply. It runs with no network and no live browser, reuses the
existing apply/rollback/audit decision rules (no demo-only writer), and shows
four behaviors end to end:

1. **Low-risk auto-apply** — a low-risk scene-only data proposal that passes all
   four gates on rerun, at high confidence, within the risk budget, and backed
   by a one-command rollback, auto-applies and is recorded in the append-only
   audit log.
2. **High-risk manual fallback** — a source-affecting proposal is classified
   manual-only and never auto-applies, even at high confidence with passing
   gates; it falls back to manual review-gated apply.
3. **Budget-exhaustion fallback** — a low-risk, eligible proposal is refused
   because the explicit risk budget is exhausted, and falls back to manual
   review.
4. **Kill-switch halt** — with the emergency kill switch engaged, autonomy is
   halted: an otherwise eligible proposal still falls back to manual review,
   restoring the default no-auto-apply posture.

## Files

- `examples/trust-gradient-v1/demo/low-risk-auto-apply.json`
- `examples/trust-gradient-v1/demo/high-risk-manual-fallback.json`
- `examples/trust-gradient-v1/demo/budget-exhausted-fallback.json`
- `examples/trust-gradient-v1/demo/kill-switch-halt.json`
- `examples/trust-gradient-v1/demo/demo-smoke.test.cjs` — the smoke test.

Each fixture records its `request`, its documented `expectedOutcome`, and a
conservative `boundary`. The low-risk fixture also records the
`expectedRollbackCommand` and the append-only `auditEntry` produced on apply.

## Running

```bash
node examples/trust-gradient-v1/demo/demo-smoke.test.cjs
```

The smoke test reimplements the fail-closed bounded auto-apply decision rules in
Node and asserts every fixture produces its documented outcome, that the applied
scenario yields exactly one rollback command and consumes budget, that the audit
entry is complete and reversible, and that the kill-switch scenario records an
engaged switch with a reason. It deterministically reproduces from a fresh clone.

## Boundary

Bounded, reversible, audited autonomy is **not** auto-merge, self-approval,
reviewer bypass, or a quality/production-ready guarantee, and makes no current
Godot replacement claim. Nothing source-affecting auto-applies. Trusted
decisions are owned by Rust/local; demo evidence is fixture-scoped and the
demonstration performs no trusted write, command execution, or network access.
#1 and #23 remain open.
