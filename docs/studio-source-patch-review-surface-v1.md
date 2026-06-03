# Studio Source Patch Review Surface Design v1

Studio Source Patch Review Surface Design v1 is a Source Mutation Design Gate v1
control artifact. It defines a future read-only Studio/dashboard review surface
for inert source patch previews. It does not implement source mutation apply,
arbitrary patch apply, merge automation, command execution, browser trusted
writes, command bridges, schedulers, worktree automation, or file writes.

The surface exists to help reviewers inspect risk, file classes, diff summaries,
evidence links, required checks, and forbidden actions before any future source
patch evaluation or source mutation apply path is considered. It is an
information architecture and trust-boundary design, not a production editor or
source-mutation readiness claim.

## Read-only information architecture

A future Studio panel should be organized as a review aid with these sections:

| Section | Required content | Boundary |
| --- | --- | --- |
| Preview identity | Patch preview id, proposal id, base ref, patch artifact hash, generated artifact path, and stale/unknown state. | Identifies review material only; no apply or merge authority. |
| Risk summary | Risk level, risk reasons, blocked/hold/pass review state, and whether independent review is required. | Displays review status; does not accept/reject on its own. |
| File class matrix | Target paths, file class, allowed/disallowed status, ownership notes, and source-like/generated classification. | Forbidden or unknown classes render as warnings, not override controls. |
| Diff summary | Bounded file counts, line counts, touched symbols or headings when available, and patch size warnings. | Escaped text only; no editable diff, patch editor, or browser file write. |
| Evidence links | Links to preview artifact, classification, review decision, rollback plan, sandbox preflight, allowed command policy, and test evidence. | Links point to generated/local evidence; missing links stay visible as blockers. |
| Required tests | Copyable check-only commands and expected evidence refs from the governing issue. | Commands are inert text; Studio never executes them. |
| Reviewer checklist | No-apply/no-source-mutation audit, generated-state audit, #1/#23 state, design-scope drift audit, and independent reviewer separation. | Checklist is display guidance; durable decisions remain Rust/CLI/review-ledger owned. |
| Forbidden actions | Explicit list of blocked actions: apply, merge, auto-accept, command execution, browser writes, network/credential/install-script use. | Always visible even when a preview is low risk. |

## Trust-boundary notices

Every future Studio source patch review surface must show unambiguous boundary
copy near the panel heading and near any copyable command text:

- "Read-only source patch review preview. Studio cannot apply, merge, accept,
  reject, write files, or execute commands."
- "Copyable commands are inert reviewer guidance only; run checks manually in an
  authorized local terminal if allowed by the issue."
- "Missing evidence, forbidden file classes, stale hashes, or sandbox failures
  block evaluation rather than broadening permissions."
- "#1 and #23 remain open; this panel does not narrow roadmap or repo-memory
  governance issues."

The notice must be rendered as normal text, not as a dismissible toast that can
hide the boundary during review.

## Warning and empty states

A future surface should fail visible and conservative:

| State | Display requirement | Forbidden fallback |
| --- | --- | --- |
| Missing preview artifact | Show `missing_preview` with expected evidence refs and no commands except inspection guidance. | Do not infer patch content from free text or repository state. |
| Malformed preview artifact | Show parse/validation error and artifact path. | Do not partially apply or render unescaped diff content. |
| Forbidden file class | Show blocked file class, path, reason, and required governance approval. | Do not add override/apply controls. |
| Missing rollback/audit evidence | Show hold state and required evidence refs. | Do not mark review ready based on risk level alone. |
| Missing sandbox/command-policy evidence | Show hold state and no-credential/no-network/no-install-script requirements. | Do not run checks from the browser or broaden command scope. |
| Stale base/hash | Show expected and observed refs/hashes when available. | Do not rebase, refresh, regenerate, or patch from Studio. |
| Unknown risk | Show `unknown`/`hold` state with reviewer action required. | Do not default unknown to low risk. |

## No-apply browser boundary

Studio and dashboard implementations remain browser-read-only for trusted state.
They may display escaped exported data and inert copyable text. They must not:

- apply source patches or scene patches;
- create, modify, delete, or move trusted repository files;
- create worktrees or branches;
- run shell commands, tests, package managers, install scripts, or network calls;
- accept, reject, merge, auto-apply, auto-accept, auto-merge, or promote changes;
- mutate CI/workflows, dependency manifests, hosted/cloud/server/auth code,
  plugin runtimes, native export paths, public launch automation, or Godot
  replacement claims; or
- close, modify, or reinterpret #1 or #23.

## Conservative wording requirements

Public and UI copy must use conservative terms such as "preview", "read-only",
"review aid", "hold", "blocked", "copy command", and "evidence required". It
must not claim production source mutation, safe auto-apply, code generation
readiness, public launch readiness, or replacement for a production editor.
