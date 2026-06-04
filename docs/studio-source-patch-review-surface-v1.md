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


## Implemented read-only bundle surface

SMP1.9.1 through SMP1.9.3 implemented the first read-only source patch
evidence-bundle surface for dashboard and authoring cockpit inspection. The
implemented surface consumes Rust-exported mutation artifact data, including
patch summary, file-class summary, risk ids, blocked reasons, linked evidence,
sandbox dry-run summary, required test summary, review summary, refs, guardrails,
and forbidden-action notices.

The implementation remains display-only:

- dashboard and cockpit render escaped text and artifact refs only;
- required test commands are displayed as inert text and are not executed;
- no apply, merge, accept, reject, refresh, rerun, install, export, launch,
  trusted-write, local-server, or command-bridge controls are present;
- source patch apply to the trusted worktree remains unimplemented and
  explicitly forbidden;
- generated preview, sandbox, report, dashboard, and run artifacts remain
  untracked unless a separate fixture-scoped issue authorizes them.

The source-like fixture for the implemented bundle shape is
`examples/source-patch-evidence-bundle-v1/source-patch-evidence-bundle.sample.json`.
Its README records the focused smoke checks for Rust validation/export and
dashboard/cockpit rendering.

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

## Read-only display data contract

A future dashboard export or Studio read model may expose a `source_patch_review`
object. The object is display-only and must be generated outside the browser by a
trusted local command or by already-existing generated evidence export flows. The
browser consumes it as inert JSON and renders escaped text; the browser must not
repair, normalize into authority, write back, or execute any field.

### Top-level shape

| Field | Required | Meaning | Display behavior |
| --- | --- | --- | --- |
| `schema_version` | yes | Contract version, initially `source_patch_review.v1`. | Unknown versions render as `unsupported_contract`. |
| `preview` | yes | Patch preview identity and artifact metadata. | Missing or malformed preview renders hold state. |
| `risk` | yes | Risk level and reasons derived from review evidence. | Unknown risk renders as `hold`, not pass. |
| `file_classes` | yes | Target file classes and allowed/disallowed status. | Forbidden/unknown classes render blocking warnings. |
| `diff_summary` | yes | Bounded counts and summary text for display. | Escaped text only; no editable patch area. |
| `evidence_links` | yes | Named generated evidence refs required for review. | Missing required refs render hold warnings. |
| `required_tests` | yes | Copyable check-only commands and expected refs. | Commands render as inert text, never buttons that execute. |
| `reviewer_checklist` | yes | Audit items the reviewer must verify. | Checklist does not write durable decisions. |
| `forbidden_actions` | yes | Blocked actions and reasons. | Always visible; cannot be dismissed as review noise. |
| `boundary_notice` | yes | Read-only/no-apply/no-command copy. | Render near heading and commands. |

### Field requirements

`preview` should include `preview_id`, `proposal_id`, `base_ref`,
`base_commit`, `patch_artifact_ref`, `patch_artifact_hash`, `created_at`, and
`stale_status`. `stale_status` is one of `current`, `stale`, `unknown`, or
`missing`; anything except `current` holds review.

`risk` should include `level` (`low`, `medium`, `high`, `critical`, or
`unknown`), `reasons`, `requires_independent_review`, and `decision_state`
(`not_reviewed`, `hold`, `rejected`, `accepted_for_future_evaluation`, or
`unknown`). The display must avoid wording that implies accepted review applies a
patch.

`file_classes` entries should include `path`, `class`, `status` (`allowed`,
`forbidden`, `unknown`, or `needs_governance`), `reason`, and `evidence_ref`.
Forbidden, unknown, or needs-governance entries block readiness.

`evidence_links` entries should include `kind`, `ref`, `required`, and `status`
(`present`, `missing`, `malformed`, or `stale`). Required missing/malformed/stale
links produce warning cards and keep the panel in hold state.

`required_tests` entries should include `label`, `command_text`, `working_dir`,
`allowed_by_policy_ref`, `expected_evidence_ref`, and `status`. `command_text` is
copyable text only and may not become a trusted browser command or URL handler.

`forbidden_actions` must include at least source patch apply, arbitrary patch
apply, auto-merge, auto-apply, auto-accept, browser trusted writes, browser
command bridge, hidden command execution, credentialed commands, network access,
install scripts, CI/workflow mutation, dependency mutation, native export, plugin
runtime, hosted/cloud/server/auth, public launch automation, Godot replacement
claims, and closure/modification of #1 or #23.

### Example display-only payload

```json
{
  "schema_version": "source_patch_review.v1",
  "preview": {
    "preview_id": "preview-2026-06-03-example",
    "proposal_id": "proposal-example",
    "base_ref": "main",
    "base_commit": "18ad4d8",
    "patch_artifact_ref": "runs/source-patch-previews/example/patch.json",
    "patch_artifact_hash": "sha256:example",
    "created_at": "2026-06-03T00:00:00Z",
    "stale_status": "unknown"
  },
  "risk": {
    "level": "unknown",
    "reasons": ["Example payload has no real evidence."],
    "requires_independent_review": true,
    "decision_state": "hold"
  },
  "file_classes": [
    {
      "path": "docs/example.md",
      "class": "documentation",
      "status": "allowed",
      "reason": "Example display row only.",
      "evidence_ref": "runs/source-patch-previews/example/classification.json"
    }
  ],
  "diff_summary": {
    "files_changed": 1,
    "additions": 3,
    "deletions": 0,
    "summary_text": "Escaped display summary only; not an editable patch."
  },
  "evidence_links": [
    {
      "kind": "patch_preview",
      "ref": "runs/source-patch-previews/example/patch.json",
      "required": true,
      "status": "missing"
    }
  ],
  "required_tests": [
    {
      "label": "Rust tests",
      "command_text": "cargo test",
      "working_dir": ".",
      "allowed_by_policy_ref": "docs/source-mutation-sandbox-boundary-v1.md",
      "expected_evidence_ref": "runs/source-patch-previews/example/test-log.txt",
      "status": "not_run"
    }
  ],
  "reviewer_checklist": [
    "No source mutation apply occurred.",
    "Generated state remains untracked.",
    "#1 and #23 remain open."
  ],
  "forbidden_actions": [
    "source_patch_apply",
    "browser_command_bridge",
    "browser_trusted_write"
  ],
  "boundary_notice": "Read-only preview. Studio cannot apply, merge, write files, or execute commands."
}
```

The example is intentionally incomplete and `hold`-oriented. It is a static
contract illustration, not a fixture that grants evaluation authority.

## Rendering rules for future prototypes

If a later issue adds a tiny deterministic prototype, it must:

1. render all fields as escaped text or inert links;
2. show boundary notices near the heading and required tests;
3. provide copy controls only if they copy plain command text to the clipboard,
   never execute commands or call a local bridge;
4. render forbidden actions and missing evidence as persistent warnings;
5. avoid apply, merge, accept, reject, rerun, refresh, install, export, launch,
   or write buttons; and
6. keep generated dashboard data untracked and removable.

Node/UI gates are required only when such UI files change. This contract-only
section changes no browser implementation.
