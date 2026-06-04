# Source Patch Apply Transaction v1

Source Patch Apply Transaction v1 is an inert contract artifact for issue #702.
It records the evidence a future trusted source-apply path must consume before
any source write could be considered. This PR unit adds the artifact shape,
fixture, and documentation only; it does not implement trusted apply, validation
readiness, rollback execution, command execution, browser writes, or merge
automation.

## Artifact purpose

A transaction links the evidence chain that must stay separate across the source
mutation pipeline:

| Field group | Purpose |
| --- | --- |
| `transactionId`, `createdAt`, `status` | Identify the transaction and whether it remains blocked or is ready for a future trusted apply gate. |
| `evidence` | Link the patch preview, sandbox report, independent review decision, file-class report, and diff-integrity report. |
| `baseRef` | Record preview and trusted-apply commits plus stale-target policy. |
| `targets` | Record target path, file class, review level, before hash, expected after hash, and sandbox after hash. |
| `diffSummary` | Preserve human-readable diff summary and diff-integrity evidence linkage. |
| `rollbackRef` | Link rollback plan, pre-apply branch/commit, target before hashes, and cleanup policy. |
| `verificationCommands` | Store allowlisted, copyable commands that a trusted runner may execute later; the artifact never executes them. |
| `postApplyScenarioRefs` | Declare evidence refs that a later apply path must create after trusted writes. |
| `blockedReasons`, `guardrails` | Keep non-apply and read-only boundaries visible to reviewers and UI surfaces. |

See `examples/source-patch-apply-transaction-v1/apply-transaction.sample.json`.

## Non-apply boundary

The transaction artifact is metadata only. It does not authorize or perform:

- source patch apply;
- trusted file writes;
- branch merge, rebase, push, or auto-merge;
- command execution;
- browser/dashboard/Studio command bridges or trusted writes;
- dependency, CI/workflow, build-script, credential, network/cloud, release,
  export, or generated/local-state mutation.

Validation and readiness enforcement are intentionally left to the next PR unit
(SA15.4.2). Read-model/export compatibility is intentionally left to SA15.4.3.

## Governance anchors

Issue #1 remains the broad roadmap/vision anchor and issue #23 remains the
repo-memory/design context anchor. This transaction model does not close, modify,
or narrow either issue.

## Validation readiness (SA15.4.2)

`validate_source_patch_apply_transaction_artifact` checks transaction readiness
without applying, writing, merging, or running commands. A transaction is blocked
when required preview/review/sandbox/file-class/diff-integrity refs are missing,
refs escape artifact roots, target hashes are malformed or inconsistent,
rollback before-hash coverage is incomplete, target paths are duplicated or not
classified as explicitly allowed source-like file classes, verification commands
are not allowlisted copyable metadata, post-apply evidence refs are absent, or
non-apply/read-only guardrails are missing.

A passed validation means only that the transaction metadata is complete enough
for a future separately scoped trusted apply implementation to consider. It does
not itself authorize source writes.
