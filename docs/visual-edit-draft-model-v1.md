# Visual Edit Draft Model v1

Status: **implemented as an inert draft artifact model for Visual Authoring v1**.
This document records the draft lifecycle, generated-state policy, and read-model
compatibility notes for issue #344. It does not authorize Studio trusted writes,
local command execution, transaction conversion, review-gated apply, or a
production visual editor.

Visual Edit Draft v1 gives Studio, CLI code, and agents a shared JSON shape for
intended visual edits before any trusted write is possible. Drafts are generated
or copied data until a later Rust-trusted command validates them and turns them
into transaction previews.

## Artifact shape

A draft artifact uses `schemaVersion: "visual-edit-draft-v1"` and includes:

| Field | Purpose |
| --- | --- |
| `draftId` | Stable draft id for evidence, review, and diagnostics. |
| `target.type` | Bounded target enum: `scene`, `tilemap`, or `asset-reference`. |
| `target.path` / `target.id` | Project-relative target path and optional manifest/read-model id. |
| `proposedOperations[]` | Inert operation envelopes with id, kind, path, optional summary, optional JSON value, and optional target-specific metadata such as `sceneOperation`. |
| `beforeHash` | Expected hash for stale-draft detection. Generic fixtures may use `sha256:<64 hex>`; scene preflight uses the trusted scene hash form `fnv1a64-canonical-json-v1:<16 hex>`. |
| `expectedAfterSummary` | Human-readable summary of the intended after state. |
| `linkedEvidence[]` | Project-relative evidence refs that justify or explain the draft. |
| `author` | Human/agent/Studio/system source metadata. |
| `validationStatus` | `unvalidated`, `partial`, or `blocked`. |
| `blockedReasons[]` | Required when status is `blocked`; absent for `unvalidated`. |

The tracked fixtures in `examples/visual-edit-draft-v1/` cover valid scene,
tilemap, and asset-reference drafts, plus partial, blocked, and invalid examples.

## Lifecycle

```text
in-memory Studio draft or generated draft JSON
  -> Visual Edit Draft v1 artifact
  -> Rust validation (ids, paths, hashes, operation envelope, blocked reasons)
  -> later transaction preview issue
  -> later visual diff / review gate / apply issue
```

VA1.2 defines the artifact and generic validation. VA1.3 adds scene-specific
operation metadata, scene hash preflight, and preview-only transaction generation
for currently supported scalar scene edits. It still does not authorize trusted
writes, visual diff rendering, review-gated apply, or Studio authoring UI.

## Read-model compatibility

Read-only surfaces may treat Visual Edit Draft v1 as an inert source record. A
read model can safely display:

- draft id, schema version, target type/path/id;
- operation ids, kinds, paths, and summaries;
- expected after summary;
- linked evidence refs;
- validation status and blocked reasons; and
- author type/id/source.

Read-only surfaces must not infer that a draft is applied, accepted, reviewed, or
safe to write. In particular:

- `validationStatus: "unvalidated"` means no trusted transaction preview exists.
- `validationStatus: "partial"` means the draft is incomplete or waiting for
  more evidence/checks.
- `validationStatus: "blocked"` means the draft should remain display-only and
  must include reviewer-facing blocked reasons.
- `linkedEvidence[]` is advisory provenance, not proof that an apply is allowed.
- `beforeHash` is a stale-draft guard for later Rust validation, not a browser
  permission token.

Dashboard, Journal, and Studio read models should preserve unknown future draft
records as display-only diagnostics unless a later issue explicitly adds a
trusted parser/export path. Browser surfaces may copy draft JSON or commands for
a human to run elsewhere, but they must not write files or execute commands.


## Scene Visual Edit Draft v1 compatibility

Scene drafts may include a `sceneOperation` object on each operation:

| Field | Purpose |
| --- | --- |
| `sceneOperation.kind` | Bounded scene draft category such as `transform_move`, `sprite_color_change`, `collider_size_change`, `hud_text_change`, `hud_value_change`, or `camera_target_selection`. Other declared categories remain rejected until the underlying scene edit transaction model supports them. |
| `sceneOperation.entityId` | Existing scene entity id to preview against. Missing entities fail preflight before preview output. |
| `sceneOperation.sceneEditPath` | Existing Rust scene edit path. Supported preview paths currently match `supported_scene_edit_paths()` in Rust: `sprite.color`, transform/velocity/size scalar fields, `components.controllable`, status hit point fields, input speed/jump fields, `components.cameraTarget.weight`, and `components.uiText.text`. |
| `sceneOperation.value` | JSON value for the scene edit. Rust validates the value through the same preview path used by scene edit transactions. |
| `sceneOperation.summary` | Optional display-only authoring note. |

Scene draft preflight is Rust-owned and side-effect-free: it checks the draft
shape, target type/id, current scene hash, operation support, entity presence,
value type, and candidate scene validity before returning transaction previews.
Transaction previews preserve `beforeSceneHash`, compute `afterSceneHash` for
valid edits, and include rollback metadata, but they do not write scene files or
transaction artifacts. Review-gated apply remains a separate trusted CLI flow.

Read-only dashboards, journals, and Studio surfaces may display scene draft
operation summaries and preview metadata as escaped diagnostics. They must not
claim that a draft has been applied or reviewed, and they must not execute the
preview or apply commands from browser JavaScript. Unsupported scene categories
such as sprite frame changes, collider toggles, and trigger/flag configuration
are intentionally visible as draft categories but fail closed until separately
supported by the Rust transaction model.

## Generated-state policy

Visual edit drafts, transaction previews, visual diffs, dashboard exports, run
outputs, and local caches are generated/local state unless a specific issue
scopes a tiny deterministic fixture as tracked source-like data.

Tracked fixtures are allowed only when they are deterministic, small, and used by
schema or validation tests. Generated run evidence should remain under ignored
roots such as `runs/`, `target/`, or other documented generated roots.

Closure audits should include:

```bash
git diff --check
git status --short --ignored
```

## Boundaries and non-goals

Visual Edit Draft v1 does not authorize:

- Studio trusted file writes, browser uploads, local command bridges, local server
  write APIs, hidden command execution, dependency installation, or credentialed
  commands;
- source mutation, source patch generation, arbitrary patch apply, auto-merge,
  auto-apply, auto-accept, self-approval, or branch automation;
- transaction conversion, visual diff rendering, review-gated apply, rollback, or
  evidence bundle writes outside later explicitly scoped issues;
- production editor, visual scripting, plugin runtime, marketplace, native
  export, hosted/cloud/server/auth infrastructure, public launch automation, or
  Godot replacement claims; or
- closing, replacing, narrowing, or superseding #1 or #23.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. Both remain open unless a separate explicit governance decision
says otherwise.
