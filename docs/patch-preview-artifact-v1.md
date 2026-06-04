# Patch Preview Artifact v1

Patch Preview Artifact v1 is a Source Mutation Design Gate v1 control artifact.
It defines a reviewable, evidence-linked preview shape for future source mutation
proposals without applying any source patch. The artifact is data-only: it may
explain proposed file changes, file classes, risk, evidence, expected behavior,
required tests, reviewer checks, and blocked reasons, but it does not write
source files, execute commands, approve changes, or merge patches.

This document is design/control scope only. The implemented schema structs and fixture-backed serde tests for issue #358 SMP1.4.1 use the existing `patch-preview.v1` schema string and remain preview-only. Source mutation application remains
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
| `artifactHash` | yes | Hash of the serialized preview artifact or deterministic fixture hash used to detect stale/mismatched preview evidence. |

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
- `diffText`: optional bounded unified diff text for review; it is inert data,
  not an executable patch.
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

## Dashboard and Studio read-only display contract

A future dashboard or Studio surface may display patch preview artifacts as an
inspection aid only. The display contract is intentionally read-only and must not
be interpreted as product implementation scope for #326.

### Required display sections

A read-only display should show:

- preview identity: `patchPreviewId`, `proposalId`, `schemaVersion`, producer,
  created time, and base ref;
- source mutation status: a prominent blocked banner when
  `sourceMutationApplyStatus` is `blocked`;
- target files: canonical paths, before hashes, file classes, review levels,
  classification statuses, and classification rationale;
- blocked reasons: per-file and preview-level reasons, including stale target,
  restricted/blocked file class, missing evidence, generated-state ambiguity, or
  source mutation apply not authorized;
- risk summary: risk level and threat-model risk IDs;
- evidence links: run/comparison/review/regression/design references used to
  justify the preview;
- expected behavior change: reviewer-facing statement of the proposed effect;
- required tests: copyable commands only, with clear wording that the browser
  does not execute them;
- reviewer checklist: stale-target, generated-state, rollback, file-class,
  risk, and non-goal drift checks; and
- rollback expectations: whether rollback/audit references are missing or ready
  for a future separately authorized apply milestone.

### Allowed interactions

Read-only surfaces may support:

- expand/collapse preview details;
- filter by status, file class, risk level, or blocked reason;
- copy paths, evidence references, risk IDs, and test commands;
- link to local/static evidence files already exported by Rust-owned tooling; and
- show malformed, stale, blocked, or missing-evidence warnings.

### Forbidden interactions

Read-only surfaces must not include controls or APIs for:

- applying patches;
- editing source files;
- writing trusted preview/review decisions;
- executing test commands or shell commands;
- merging branches or opening/closing PRs;
- accepting, auto-accepting, auto-merging, or self-approving previews;
- mutating dependency manifests, CI workflows, secrets, build scripts, or plugin
  loaders; or
- treating browser/CDP observations as trusted write authority.

### Display states

| State | Meaning | Display requirement |
| --- | --- | --- |
| `blocked` | Preview is reviewable but source mutation apply is not authorized. | Show blocked banner and disable/hide any apply-like affordance. |
| `stale` | Base ref or target hashes no longer match review context. | Show regeneration-required warning. |
| `malformed` | Required schema fields, classifications, risk IDs, evidence, or warnings are missing. | Show invalid preview warning and no review-ready indication. |
| `restricted` | One or more files require separate governance approval. | Show hold/reject guidance and required approval rationale. |
| `read_only_ready` | Preview is well-formed enough for inspection. | Still show no-apply boundary; only inspection/copy interactions are allowed. |

### Wording guardrails

The UI copy should say "preview", "blocked", "read-only", "copy command", and
"requires separate review/apply milestone". It should not say "apply now",
"approved", "safe to merge", "production ready", "source mutation ready", or
"trusted browser write".

If a future issue prototypes this display in dashboard or Studio code, that issue
must run the relevant Node gates and repeat the no-apply/no-command-bridge audit.
This issue adds only the display contract.

## Implemented validation and CLI read model

Issue #358 adds an inert Rust schema, validation report, compact read model, and
read-only CLI inspection path for `patch-preview.v1` artifacts.

The CLI commands are inspection-only:

```bash
ouroforge patch-preview validate examples/patch-preview-artifact-v1/patch-preview.sample.json
ouroforge patch-preview show examples/patch-preview-artifact-v1/patch-preview.sample.json
```

- `validate` parses the artifact, runs required-field checks, source file class
  validation, and patch diff integrity validation, then prints
  `source-patch-preview-validation-v1` JSON. It exits non-zero when validation is
  blocked.
- `show` prints `source-patch-preview-read-model-v1` JSON for read-only display.
  It does not apply patches, run required tests, create sandboxes, write trusted
  source files, merge branches, or execute commands.

Required tests are still copyable metadata only. A passed validation means the
preview artifact is internally consistent enough for later review/read-model
surfaces; it is not apply, merge, sandbox, or command authority.
