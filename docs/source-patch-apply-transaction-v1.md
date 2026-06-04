# Source Patch Apply Transaction v1

Source Patch Apply Transaction v1 is an inert contract artifact for issue #702.
It records the evidence a future trusted source-apply path must consume before
any source write could be considered. The merged v1 scope adds the artifact
shape, shape validation, linked-evidence inspection, generated dashboard export,
and read-only display compatibility; it does not implement trusted apply,
rollback execution, command execution, browser writes, or merge automation.

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

Shape validation, linked-evidence inspection, generated dashboard export, and
read-model compatibility are included in v1. These checks remain metadata-only:
they do not execute commands, apply patches, merge branches, or write trusted
files.

## Governance anchors

Issue #1 remains the broad roadmap/vision anchor and issue #23 remains the
repo-memory/design context anchor. This transaction model does not close, modify,
or narrow either issue.

## Validation readiness (SA15.4.2)

`validate_source_patch_apply_transaction_artifact` checks transaction shape
without applying, writing, merging, or running commands.
`inspect_source_patch_apply_transaction_artifact_with_evidence_root` additionally
checks that linked evidence files exist, are readable JSON, contain the expected
ids, and carry accepted/passed-style status metadata. A transaction is blocked
when required preview/review/sandbox/file-class/diff-integrity refs are missing,
refs escape artifact roots, target hashes are malformed or inconsistent,
rollback before-hash coverage is incomplete, target paths are duplicated or not
classified as explicitly allowed source-like file classes, verification commands
are not allowlisted copyable metadata, post-apply evidence refs are absent, or
non-apply/read-only guardrails are missing.

A `shape_valid_pending_linked_evidence` result means only that transaction
metadata is internally complete. A `linked_evidence_ready_no_apply_authority`
result means linked evidence also passed read-only presence/id/status inspection.
Neither result authorizes source writes.

## Read-model compatibility (SA15.4.3)

`source_patch_apply_transaction_read_model` summarizes transaction readiness,
targets, evidence refs, blockers, allowed inspection actions, and forbidden apply
or command actions for dashboard/Studio display. Dashboard and cockpit surfaces
export and may render `source-patch-apply-transaction` mutation artifacts read-only; they do
not expose apply, merge, command execution, browser command bridge, or trusted
write controls.
