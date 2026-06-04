# Source Patch Stale Target Guard v1

Source Patch Stale Target Guard v1 is an inert contract artifact for issue #703.
It records and validates stale-target and freshness evidence that a future
trusted source-apply path must inspect before any source write could be
considered.

## Artifact purpose

A stale target guard records:

| Field group | Purpose |
| --- | --- |
| `guardId`, `createdAt`, `status` | Identify the guard and whether the recorded guard state is fresh, stale, or blocked. |
| `transactionId`, `transactionRef` | Link the apply transaction being guarded. |
| `baseRef` | Record expected and observed branch/head state. |
| `evidenceFreshness` | Link preview, file-class, diff-integrity, sandbox, review, and transaction evidence plus freshness policy. |
| `targets` | Record target path, file class, expected before hash, observed hash, file status, and mode status. |
| `worktreeContextRef`, `guardResults` | Link worktree context evidence and guard outcomes. |
| `blockedReasons`, `guardrails` | Keep stale/blocking reasons and non-apply boundaries visible. |

See `examples/source-patch-stale-target-guard-v1/stale-target-guard.sample.json`.

## Non-apply boundary

The stale target guard artifact is metadata only. It does not authorize or perform:

- source patch apply;
- trusted file writes;
- branch merge, rebase, push, or auto-merge;
- command execution;
- browser/dashboard/Studio command bridges or trusted writes;
- dependency, CI/workflow, build-script, credential, network/cloud, release,
  export, or generated/local-state mutation.

## Governance anchors

Issue #1 remains the broad roadmap/vision anchor and issue #23 remains the
repo-memory/design context anchor. This guard model does not close, modify, or
narrow either issue.

## Implemented validation scope

The v1 validator fails closed when recorded or current target state is unsafe:

- expected before hash and observed/current hash mismatch;
- missing or changed target files;
- target paths that escape or resolve through symlinks before hashing;
- stale branch/head observations;
- file status or file mode mismatch evidence;
- missing, stale, unrelated, or mismatched linked preview, file-class,
  diff-integrity, sandbox, review, transaction, or worktree-context evidence.

The dashboard and Studio read models display this evidence read-only. They do not
turn the guard into a trusted apply implementation, and future trusted apply code
must explicitly call the current-target and linked-evidence checks immediately
before any separately authorized write.
