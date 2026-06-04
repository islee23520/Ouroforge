# Review-Gated Level Apply v1

Issue: #638 - Review-Gated Level Apply v1.

Review-Gated Level Apply v1 records the trusted local validation contract for
turning an untrusted generated level draft into reviewable scene, tilemap, and
placement transaction outputs. A ready artifact requires accepted non-self review,
fresh target hashes, safe transaction-output refs, rollback metadata,
rerun evidence, and generated-state audit evidence.

This is an inert contract and read model. It does not add a browser command
bridge, hidden command execution, arbitrary script execution, or autonomous
trusted writes.

## Artifact Shape

The `review-gated-level-apply-v1` artifact includes:

- transaction, draft, intent, and plan ids;
- accepted review decision metadata with reviewer and draft author ids;
- target refs, before/observed hashes, expected after hashes, and safe
  transaction output refs under `evidence/level-apply/<transaction-id>/`;
- rollback metadata with pre-apply branch, commit, and target before hashes;
- rerun command metadata that remains allowlist-scoped;
- agent draft, review decision, diff, rollback, rerun, and generated-state
  evidence refs;
- ready, missing-review, rejected, stale, and blocked statuses.

## Boundary

Ready means ready for a separately scoped trusted local apply, not that this
artifact applies anything by itself.

- No browser trusted writes.
- No auto-apply.
- No auto-merge.
- No self-approval.
- No autonomous full game generation.
- No production editor or full visual level editor claim.

## Non-Goals

- No browser command bridge or local server bridge.
- No arbitrary script execution.
- No unrestricted source mutation.
- No native export, plugin runtime, hosted/cloud behavior, production-ready
  claim, or Godot replacement claim.
