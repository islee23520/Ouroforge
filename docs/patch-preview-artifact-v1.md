# Patch Preview Artifact v1

Patch Preview Artifact v1 is a Source Mutation Design Gate v1 control artifact.
It defines a reviewable, evidence-linked preview shape for future source mutation
proposals without applying any source patch. The artifact is data-only: it may
explain proposed file changes, file classes, risk, evidence, expected behavior,
required tests, reviewer checks, and blocked reasons, but it does not write
source files, execute commands, approve changes, or merge patches.

This document is design/control scope only. Source mutation application remains
blocked until a later explicit implementation milestone authorizes an apply path
with review, rollback, sandbox, stale-target, and generated-state controls.

## Artifact identity

A patch preview artifact should include:

| Field | Required | Description |
| --- | --- | --- |
| `schemaVersion` | yes | Version string for this preview contract, for example `patch-preview.v1`. |
| `patchPreviewId` | yes | Stable id unique within the producing run or review bundle. |
| `proposalId` | yes | Upstream proposal/review item this preview explains. |
| `createdAt` | yes | Timestamp from the trusted producer context. |
| `producer` | yes | Tool/agent label and version/context when available. |
| `sourceMutationApplyStatus` | yes | Must be `blocked`; any `applied`, `merged`, or command-executed state is invalid for this design gate. |
| `baseRef` | yes | Branch/commit/hash context the preview was generated against. |
| `staleTargetPolicy` | yes | How target hash or latest-main drift forces regeneration before review. |

## Target files

Each target file entry should include:

| Field | Required | Description |
| --- | --- | --- |
| `path` | yes | Repository-relative canonical path. Absolute paths, traversal, symlink ambiguity, and ignored generated roots are invalid. |
| `beforeHash` | yes | Hash of the target content used to produce the preview. |
| `afterHash` | optional | Expected hash if the preview is later applied by a separately authorized workflow. Absence is allowed for design-only previews. |
| `fileClass` | yes | File class from `docs/source-mutation-file-classes-v1.md`. |
| `reviewLevel` | yes | Required review level for the class. |
| `classificationStatus` | yes | `potentially_allowed_later`, `restricted_separate_approval`, or `blocked_by_design`. |
| `classificationRationale` | yes | Reviewer-facing reason for the class decision. |
| `blockedReasons` | conditional | Required when class is restricted/blocked, target is stale, path is unsafe, or evidence is missing. |

## Diff summary

The preview should make the proposed change reviewable without requiring hidden
patch application:

- `summary`: concise human-readable change summary.
- `diffStats`: files changed, additions, deletions, binary/opaque indicators,
  generated-origin indicators, and truncation status.
- `hunks`: bounded text hunk summaries or explicit omitted-content warnings.
- `largeDiffWarning`: required when a diff is truncated or too large for full
  reviewer display.
- `binaryOrOpaqueWarning`: required for binary, minified, or opaque files.

The diff summary is evidence for review only. It is not an executable patch and
must not be treated as permission to write files.

## Risk and evidence fields

| Field | Required | Description |
| --- | --- | --- |
| `riskLevel` | yes | `low`, `medium`, `high`, or `critical`, derived from file classes and threat-model risk IDs. |
| `riskIds` | yes | References to `docs/source-mutation-threat-model-v1.md`, for example `STM-07`. |
| `linkedEvidence` | yes | Existing run, comparison, review, regression, or handoff artifacts used to justify the proposal. |
| `expectedBehaviorChange` | yes | What should change if a later authorized workflow applies the patch. |
| `requiredTests` | yes | Commands or checks expected before any later review decision; commands are copyable context, not executed by the artifact. |
| `reviewerChecklist` | yes | Human/reviewer checks for class, risk, evidence, stale targets, generated-state, rollback, and non-goal drift. |
| `rollbackExpectations` | yes | Required rollback/audit references a later implementation must provide before apply is considered. |

## Validation expectations

A future validator/read-model should reject or mark blocked previews when:

- `sourceMutationApplyStatus` is not `blocked`;
- any target path is absolute, traverses outside the repo, points into ignored
  local/generated state, or is symlink/hard-link ambiguous;
- a target lacks a file class, review level, or classification rationale;
- a blocked/restricted class lacks blocked reasons;
- risk IDs are absent or do not match known threat-model identifiers;
- expected behavior change, required tests, reviewer checklist, or rollback
  expectations are missing;
- diff content is truncated without an explicit warning;
- generated-origin content lacks promotion rationale; or
- base refs or target hashes are stale relative to the review context.

A future read-model may summarize valid, blocked, stale, and malformed preview
states for dashboards or Studio, but it must remain read-only and must not apply
patches or execute commands.

## No-apply boundary

Patch Preview Artifact v1 does not authorize:

- source mutation application;
- arbitrary patch apply;
- patch execution engines or schedulers;
- automatic apply, automatic merge, automatic accept, or self-approval;
- browser-side trusted writes or command bridges;
- dependency changes, CI/workflow mutation, build script mutation, or
  credentialed command execution;
- plugin runtime, hosted/cloud/server/auth, native export, production editor,
  public launch automation, or Godot replacement claims.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.
