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

## Dependency-change restrictions

Dependency manifests and lockfiles are restricted by default and must not be
changed by a generic source patch preview or future source mutation flow. A
preview that touches dependency configuration must be `hold` or `reject` unless a
separate explicit design/governance issue authorizes dependency review scope.

A dependency-related review must, at minimum, identify:

- the manifest or lockfile path;
- direct and transitive dependency impact;
- supply-chain and build-script risk;
- affected verification commands;
- rollback plan for manifest and lockfile state; and
- why the change belongs in the current milestone instead of a dedicated
  dependency issue.

Without that separate authorization, dependency mutation remains blocked.

## Test plan requirements

A source patch preview's test plan must be reviewer-visible and copyable. It must
not be executed by the preview artifact, browser, or Studio. The test plan should
include:

1. targeted checks for the changed behavior or data;
2. stale-target and base-ref freshness checks;
3. generated-state audit commands;
4. file-class and risk-specific verification;
5. broad gates when trusted code, tests, evidence readers, or public wording are
   affected;
6. explicit commands that are safe to run locally; and
7. known gaps when a command is intentionally not run.

If a preview changes tests, evidence readers, review gates, rollback contracts,
or command allowlists, the review level must be elevated because the patch could
hide failures or spoof confidence.

## Rollback requirements

A source patch preview is not eligible for later apply consideration unless it
states rollback expectations. A future implementation issue may define exact
formats, but the review gate requires the design to cover:

- pre-change branch/ref and target hashes;
- affected file list and file classes;
- review decision reference;
- evidence bundle or audit reference;
- cleanup expectations for generated/local state;
- stale rollback rejection behavior;
- symlink, hard-link, traversal, and ignored-root rejection behavior; and
- human-readable recovery instructions.

Rollback metadata is review evidence only in this design gate. It does not apply,
restore, or write files.

## Generated-state audit requirements

Every future source patch review should record whether local/generated roots are
untracked or ignored. At minimum, closure evidence should confirm that generated
state such as `.omx/`, `.omc/`, `.claude/`, `.openchrome/`, `runs/`, and
`target/` was not committed unless a later issue explicitly scopes a deterministic
source-like fixture.

## Closure constraints for future source patch review issues

A future source patch review issue cannot close until:

- every preview has explicit file-class approval or a hold/reject reason;
- linked evidence is present or missing-evidence is called out;
- dependency changes are absent or separately authorized;
- test plan and rollback expectations are recorded;
- generated-state audit is recorded;
- no browser apply/write/command bridge exists;
- source mutation apply remains blocked unless the issue is an explicit later
  apply implementation milestone; and
- #1 and #23 remain open unless a separate governance decision says otherwise.
