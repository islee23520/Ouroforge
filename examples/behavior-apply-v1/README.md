# Behavior Apply Transaction v1 fixtures

Fixture-scoped examples for #620 GL10.10.1. These files model the inert
transaction/apply contract only; they do not perform trusted writes.

- `valid/behavior-apply.ready.json` — ready transaction contract with draft,
  review decision, target hashes, rollback metadata, rerun command context, safe generated
  transaction output ref, evidence refs, and trusted-boundary text.
- `valid/behavior-apply.stale.json` — stale target state that remains visibly
  blocked before trusted apply.
- `invalid/behavior-apply.unsafe-output.json` — transaction output escaping the
  generated `runs/` root.
- `invalid/behavior-apply.output-collision.json` — transaction output trying to masquerade as
  a scene source artifact.
- `invalid/behavior-apply.unsupported-behavior.json` — unsupported proposed
  behavior that must not become apply-ready.

Trusted apply execution, actual rollback writing, and post-apply evidence
read models are deferred to later #620 PR units. No arbitrary script execution, command bridge, browser
trusted writes, auto-apply, auto-merge, or self-approval is introduced here.
