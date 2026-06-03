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
| `beforeHash` | Expected hash for stale-draft detection. Generic fixtures may use `sha256:<64 hex>`; scene preflight uses the trusted scene hash form `fnv1a64-canonical-json-v1:<16 hex>`; tilemap preflight may use `fnv1a64-file-v1:<16 hex>` when checking the current asset file hash. |
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
VA1.4 adds tilemap-specific operation metadata, Rust-owned preflight, and
preview-only summaries/collision/trigger metadata for bounded tilemap draft
operations. It still does not authorize tilemap file writes or browser apply.
VA1.5 adds asset-reference operation metadata, Rust-owned manifest/integrity
preflight, and preview-only summaries for bounded sprite, sprite-frame, audio,
font, and tilemap tileset references. It still does not authorize asset writes,
remote fetches, Studio authoring UI, or apply behavior.

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

## Tilemap Visual Edit Draft v1 compatibility

Tilemap drafts may include a `tilemapOperation` object on each operation:

| Field | Purpose |
| --- | --- |
| `tilemapOperation.kind` | Bounded tilemap draft category: `tile_set`, `tile_remove`, `rectangle_fill`, `layer_visibility_change`, `layer_config_change`, `tile_property_reference_change`, `collision_preview`, or `trigger_preview`. |
| `tilemapOperation.layerId` | Existing tilemap layer id. Missing layers fail preflight before preview output. |
| `tilemapOperation.x` / `y` | Cell coordinate for point operations and rectangle origin. Coordinates must be inside the tilemap bounds. |
| `tilemapOperation.width` / `height` | Rectangle size for rectangle fills. Sizes must be non-zero, in bounds, and within the Rust operation cell limit. |
| `tilemapOperation.tileId` | Existing tileset tile id where an operation needs a tile reference. Unknown tile ids fail preflight. |
| `tilemapOperation.tilesetAssetId` | Optional expected tileset asset id. If present, it must match the tilemap's tileset reference. |
| `tilemapOperation.visible` / `metadata` | Draft-only visibility/config/property metadata for preview summaries. |

Tilemap draft preflight is Rust-owned and side-effect-free: it checks the draft
shape, target type/id, current before hash, tilemap schema/integrity, layer ids,
tile ids, point/rectangle bounds, and operation size limits before returning
preview records. Tilemap preview records are inert read-model data with operation
ids, affected-cell counts, before/after preview hashes, summaries, and
collision/trigger cell metadata. Preview generation mutates only cloned
in-memory tilemap manifests and does not write tilemap files, execute browser
commands, create transaction artifacts, or apply reviews.

Read-only dashboards, journals, and Studio surfaces may display tilemap preview
summaries, affected cell counts, hashes, and collision/trigger metadata as
escaped diagnostics. They must not claim that a tilemap draft has been applied or
reviewed, and they must not execute preview/apply commands from browser
JavaScript. Review-gated apply remains a separate trusted CLI flow.

## Asset Reference Edit Draft v1 compatibility

Asset-reference drafts may include an `assetReferenceOperation` object on each
operation:

| Field | Purpose |
| --- | --- |
| `assetReferenceOperation.kind` | Bounded asset reference category: `sprite_asset_reference`, `sprite_frame_reference`, `audio_event_asset_reference`, `font_asset_reference`, or `tilemap_tileset_reference`. |
| `assetReferenceOperation.targetReferencePath` | Draft address of the reference being changed. It must match the operation envelope `path` during preflight. |
| `assetReferenceOperation.replacementAssetId` | Manifest asset id proposed as the replacement reference. Missing ids fail preflight. |
| `assetReferenceOperation.expectedAssetType` | Optional expected manifest type. If present, it must match both the operation kind and the manifest entry type. |
| `assetReferenceOperation.expectedContentHash` | Optional manifest content hash expectation. If present, it must match the manifest entry before preview summaries are produced. |
| `assetReferenceOperation.frameId` | Required for `sprite_frame_reference`; the frame id must exist in the sprite atlas payload. |
| `assetReferenceOperation.eventId` | Required for `audio_event_asset_reference`; used only as preview/read-model context in this issue. |
| `assetReferenceOperation.metadata` / `summary` | Draft-only display metadata. It must remain inert and must not authorize writes. |

Asset-reference preflight is Rust-owned and side-effect-free except for explicit
local file integrity reads supplied by the caller. It checks the draft shape,
target type/id, operation payload consistency, replacement asset id, operation
kind/type compatibility, optional expected type/hash, local file integrity, and
kind-specific frame/event requirements before returning preview records.

Asset-reference preview records are inert read-model data with operation ids,
manifest id, target reference paths, replacement asset ids, observed asset type,
content hash, optional frame/event ids, summaries, and an explicit guardrail
string. Preview generation does not write asset files, fetch remote assets,
execute browser commands, create transaction artifacts, or apply reviews.

Read-only dashboards, journals, and Studio surfaces may display asset-reference
preview summaries and manifest/hash diagnostics as escaped diagnostics. They must
not claim that an asset reference draft has been applied or reviewed, and they
must not execute preview/apply commands from browser JavaScript. Review-gated
apply remains a separate trusted CLI flow.

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
