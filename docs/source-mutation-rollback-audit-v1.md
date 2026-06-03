# Source Mutation Rollback and Audit Contract v1

Source Mutation Rollback and Audit Contract v1 is a Source Mutation Design Gate
v1 control artifact. It defines the rollback evidence and audit expectations
that any future source mutation milestone must satisfy before source mutation
implementation could be considered.

This document is design/control scope only. It does not implement source mutation
apply, arbitrary patch apply, rollback execution, patch execution, browser
writes, command execution, source patch generators, or merge automation.

## Rollback contract boundary

Rollback evidence is mandatory review context, not a command to execute. A future
source patch preview or apply milestone must prove that recovery is possible
before an apply path is authorized, but this design gate does not write files,
restore files, run revert commands, or modify git state.

## Required file hash evidence

A rollback-ready source patch design must record deterministic hashes for every
targeted file:

| Field | Required | Purpose |
| --- | --- | --- |
| `path` | yes | Canonical repository-relative path; absolute/traversal/symlink-ambiguous paths are invalid. |
| `fileClass` | yes | File class from `docs/source-mutation-file-classes-v1.md`. |
| `beforeHash` | yes | Hash of the target content before the proposed mutation. |
| `previewHash` | yes | Hash of the patch preview target content or diff representation. |
| `afterHash` | conditional | Expected hash after a later separately authorized apply path; absent for design-only previews. |
| `rollbackHash` | conditional | Expected hash after rollback, normally matching `beforeHash`; required before any future apply implementation. |
| `hashAlgorithm` | yes | Explicit algorithm such as SHA-256. |
| `hashSource` | yes | Trusted producer context and base ref used to compute hashes. |

Hash mismatches must force hold/reject or preview regeneration. A later apply
milestone must not treat stale hashes as recoverable state.

## Patch artifact hash requirements

A future rollback-ready preview should hash the review artifacts themselves:

- patch preview artifact hash;
- diff summary or rendered patch hash;
- review decision artifact hash;
- linked evidence bundle hash or stable reference;
- rollback plan artifact hash; and
- generated-state audit artifact hash or stable reference.

The purpose is reviewer-visible provenance. These hashes do not authorize apply
or rollback execution in this design gate.

## Revert context requirements

A future source mutation apply milestone must design revert context before apply
is considered. At minimum the rollback plan should include:

1. base branch/ref and commit used to generate the preview;
2. target file list and before hashes;
3. patch preview id and patch artifact hash;
4. review decision id and reviewer context;
5. command context for any future trusted revert command;
6. generated/local cleanup expectations;
7. stale-target rejection policy;
8. symlink, hard-link, traversal, and ignored-root rejection policy;
9. failure-mode guidance for partial apply or partial rollback; and
10. human-readable recovery instructions.

The command context is copyable/reviewable text only unless a later explicit
implementation milestone defines trusted execution. Browser and Studio surfaces
must not execute revert commands.

## Rollback decision states

| State | Meaning | Source write authority |
| --- | --- | --- |
| `rollback_not_applicable` | Design-only preview or rejected proposal; no source mutation occurred. | None. |
| `rollback_required_before_apply` | Future apply consideration is blocked until rollback context exists. | None. |
| `rollback_ready_for_later_milestone` | Required rollback evidence is present for future review. | None in this design gate. |
| `rollback_blocked` | Hashes, refs, paths, generated-state audit, or revert context are stale/missing/unsafe. | None. |

No rollback state in this design gate restores files or applies source changes.

## No-apply / no-rollback-execution boundary

This rollback contract does not authorize:

- source mutation application;
- arbitrary patch apply;
- rollback execution or source file restoration;
- patch execution engines or schedulers;
- browser-side trusted writes or command bridges;
- dependency changes unless separately authorized;
- CI/workflow mutation, credentialed command execution, native export, plugin
  runtime, hosted/cloud/server/auth, distributed QA/Elixir, production editor,
  public launch automation, or Godot replacement claims.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.

## Audit trail contract

A future source mutation workflow must be auditable before any apply path is
considered. Audit records must explain what was proposed, what evidence supported
it, who reviewed it, what tests were expected or run, what rollback context
exists, and whether generated/local state stayed outside source control.

This audit contract is design-only. It does not add ledger event implementations,
journal writers, source mutation events, or source apply behavior.

## Ledger event requirements

A future implementation milestone should define ledger events for source patch
preview and review lifecycle transitions. At minimum, the design should account
for these event kinds:

| Event | Required references |
| --- | --- |
| `source_patch_preview_created` | preview id, proposal id, base ref, patch artifact hash, target file hashes, file classes, risk ids. |
| `source_patch_review_held` | preview id, reviewer context, hold reasons, missing evidence, stale refs, restricted classes. |
| `source_patch_review_rejected` | preview id, reviewer context, rejection reasons, evidence refs, generated-state audit ref. |
| `source_patch_review_accepted_for_later_apply_consideration` | preview id, reviewer context, evidence refs, test plan, rollback plan, explicit note that apply remains blocked. |
| `source_patch_rollback_context_recorded` | preview id, rollback plan hash, before hashes, rollback hashes, revert context, cleanup expectations. |
| `source_patch_generated_state_audited` | preview id, audit command/context, ignored roots, untracked roots, source-like fixture exceptions. |

Every event should be append-only, timestamped, linked to stable artifacts, and
safe to render in read-only Studio/dashboard surfaces without creating trusted
write authority.

## Journal entry requirements

Future journal/read-model entries should summarize source patch preview state for
humans without implying apply readiness. A journal entry should include:

- preview id and proposal id;
- current review state (`hold`, `reject`, or accepted for later apply
  consideration);
- source mutation apply status, which remains blocked under this design gate;
- linked failing/passing evidence;
- target file classes and risk IDs;
- test plan summary and known gaps;
- rollback/audit readiness summary;
- generated-state audit result; and
- #1/#23 governance anchor status when closing design-gate issues.

The journal must distinguish trusted Rust-generated artifacts from browser/CDP
observations and must expose missing or malformed evidence as warnings rather
than inferred passes.

## Test evidence requirements

Audit records for future source patch reviews should link test evidence rather
than merely naming commands. The expected evidence includes:

1. command text and working directory;
2. command authority/trust boundary;
3. exit status and timestamp;
4. targeted behavior or file class covered;
5. known gaps or skipped commands;
6. generated-state audit result after tests; and
7. stale evidence warning if base refs or target hashes changed after the test.

Browser or Studio displays may copy commands and show evidence summaries, but
must not execute tests.

## Generated-state audit requirements

A generated-state audit should be recorded for each future source patch review.
The audit should identify:

- ignored local roots such as `.omx/`, `.omc/`, `.claude/`, `.openchrome/`,
  `runs/`, and `target/`;
- newly untracked files;
- deterministic source-like fixtures intentionally added by the current issue;
- build outputs, caches, screenshots, evidence bundles, and run artifacts that
  must remain untracked;
- any generated-origin file promoted to source-like status, with rationale; and
- final source-control state before closure.

Generated-state audit records do not authorize committing ignored/local state.

## Audit closure constraints

A future source mutation-related issue cannot close unless its final evidence
comment or equivalent audit record states:

- merged PRs in required order;
- artifacts changed;
- verification commands and results;
- linked test evidence or known gaps;
- no-apply/no-source-mutation audit;
- rollback/audit readiness or explicit out-of-scope note;
- generated-state audit;
- non-goal/over-engineering/drift audit; and
- #1 and #23 remain open.
