# Source Patch Review Gate v1

Source Patch Review Gate v1 defines the review requirements that any future
source patch preview must satisfy before source mutation implementation could be
considered. It is a Source Mutation Design Gate v1 control artifact and does not
implement source mutation apply, arbitrary patch apply, review-gated source
apply, patch execution, browser writes, command execution, or merge automation.

The gate starts from the premise that a patch preview is inspectable evidence,
not authority to change files. A future apply milestone remains blocked unless a
separate issue defines trusted implementation, rollback, sandbox, stale-target,
and audit behavior.

## Required review gate checklist

A future source patch preview is not review-ready unless it records all of the
following:

| Requirement | Purpose | Review-ready evidence |
| --- | --- | --- |
| Linked failing/passing evidence | Prove why the patch is proposed and what behavior should change or remain stable. | Run, comparison, regression, review, or design evidence references with stale/missing warnings. |
| Explicit file-class approval | Prevent class drift into unsafe files. | File-class label, classification status, review level, and rationale from `docs/source-mutation-file-classes-v1.md`. |
| Threat-model risk labels | Make source mutation risk visible to reviewers. | Risk IDs from `docs/source-mutation-threat-model-v1.md`, plus derived risk level. |
| Patch preview artifact | Make the proposed change reviewable without applying it. | `docs/patch-preview-artifact-v1.md` fields including target hashes, diff summary, blocked reasons, expected behavior, required tests, and rollback expectations. |
| Test plan | State the checks a human or trusted runner would execute before any later decision. | Copyable commands and rationale; no browser or preview artifact executes them. |
| Rollback plan | Show how a later authorized apply path would recover. | Pre-change refs, target hashes, review decision refs, cleanup expectations, and stale rollback rejection rules. |
| Generated-state audit | Prevent ignored/local artifacts from becoming reviewed source. | `git status --short --ignored`-style evidence or equivalent, with generated/local roots kept untracked. |
| Reviewer decision | Require explicit human/reviewer outcome. | `accept`, `reject`, or `hold` with reviewer identity/context, reason, evidence refs, and timestamp. |
| Governance anchor audit | Preserve roadmap and memory anchors. | #1 and #23 confirmed open before issue closure or future source-patch decision. |

## Reviewer decision states

| State | Meaning | Source write authority |
| --- | --- | --- |
| `hold` | Preview is incomplete, stale, restricted, missing evidence, or waiting on separate governance approval. | None. |
| `reject` | Preview is unsafe, out of scope, malformed, or not worth pursuing. | None. |
| `accept_for_later_apply_consideration` | Reviewer agrees the preview may be considered by a future separately authorized apply milestone. | None in this design gate. |

No decision state in Source Patch Review Gate v1 applies a patch or writes source
files. Even an accepted preview remains blocked until a later implementation
milestone explicitly authorizes an apply path.

## Self-approval and automation limits

- The proposer or generating agent cannot be the sole reviewer.
- A browser/Studio display cannot accept, apply, merge, or execute commands.
- Generated evidence cannot silently upgrade a preview to accepted.
- Auto-merge, auto-apply, auto-accept, and hidden command execution are forbidden.
- Dependency, CI/workflow, build-script, secret, plugin, hosted/cloud/server/auth,
  native export, public launch, and production-editor scope cannot be approved by
  this gate.

## No-apply boundary

This review gate is a design/control artifact only. It does not authorize:

- source mutation application;
- arbitrary patch apply;
- review-gated source apply implementation;
- patch execution engines or schedulers;
- browser-side trusted writes or command bridges;
- dependency changes unless separately authorized;
- CI/workflow mutation, credentialed command execution, native export, plugin
  runtime, hosted/cloud/server/auth, distributed QA/Elixir, production editor,
  public launch automation, or Godot replacement claims.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.
