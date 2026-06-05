# Behavior Apply Transaction v1

Behavior apply transactions are review-gated evidence records for #620. They do
not execute behavior changes by themselves. The trusted boundary remains in local
Rust validation and any future apply path must preserve the same gates.

A ready transaction records:

- `draftId` and `transactionId` provenance;
- an accepted, non-self `reviewDecision`;
- target hash binding with matching `expectedBeforeHash` and
  `observedBeforeHash` plus an `afterHash` preview;
- generated `transactionOutputRef` evidence, not a source scene path;
- `rollbackMetadata` with a before hash and rollback evidence ref;
- `rerunCommand` context with an allowlist policy id;
- evidence refs for reviewed scenario or validation artifacts; and
- trusted-boundary text that keeps no arbitrary scripting, no auto-apply, no
  self-approval, no browser trusted writes, and no command bridge explicit.

The read model `ouroforge.behavior-apply-transaction-read-model.v1` is display
only. It summarizes review, hash freshness, rollback, rerun, transaction-output,
and evidence refs for dashboards or reports without writing trusted files,
executing commands, applying behavior changes, or granting browser authority.

Generated behavior apply transactions belong under ignored/local roots such as
`runs/...` or `.omx/...` unless a fixture is intentionally checked in for tests.
