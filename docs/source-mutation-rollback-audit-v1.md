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
