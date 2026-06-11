# Preview Session v1 (M131.1, Era X)

Issue: #2518. Parent SSOT: #2517 (Era X — Live Preview Loop v1). Design
authority: the #2517 Q1 ratification (Model B — Rust-validated ephemeral
preview proposals).

Preview Session v1 defines the contract for the long-lived local validation
process behind `ouroforge preview serve`. An untrusted preview surface (the
browser runtime page, the Studio cockpit) sends **manipulation intents**; the
session validates and normalizes them through the **same code paths used by
review-gated scene apply preflight** and answers with **normalized deltas**
that consumers apply mechanically.

## Invariants

- **Apply-path parity.** Intent validation uses `apply_scene_edit` +
  `validate_scene` — the exact pair exercised by
  `preview_scene_edit_transaction`. The parity golden suite
  (`crates/ouroforge-core/tests/preview_session_parity.rs`) locks
  byte-identical outcomes (after-scene hash on pass, rejection on fail)
  between the preview path and the apply-preflight path across every
  allowlisted scene-edit path. This suite is a permanent regression gate.
- **In-memory only.** A session holds the scene document in memory. The serve
  path performs no filesystem writes except logs under ignored roots; the
  scene file is only read (at session start and on `sceneReload`). Nothing
  reaches the trusted worktree except via the existing review-gated CLI apply.
- **No write authority for surfaces.** Intents carry zero file/shell
  authority. There is no command bridge: the browser sends intents and
  receives deltas, nothing else.
- **Atomic intents.** All edits normalized from one intent apply
  all-or-nothing; a rejected intent leaves session state and sequence
  untouched.
- **Deterministic deltas.** Identical session histories produce byte-identical
  deltas (`deltaId` derives from the canonical JSON of the delta seed).

## Schemas

### `ouroforge.preview-intent.v1`

| Field | Notes |
| --- | --- |
| `schemaVersion` | must be `ouroforge.preview-intent.v1` |
| `intentId` | client-supplied identifier, echoed in the delta |
| `sessionId` | must match the active session |
| `kind` | `parameterSet` \| `entityTransform` \| `sceneReload` |

- `parameterSet`: `entityId`, `path` (must be in `supported_scene_edit_paths()`), `value`.
- `entityTransform`: `entityId`, optional integer `x`/`y` (at least one);
  normalizes to `components.transform.{x,y}` edits applied atomically.
- `sceneReload`: no payload; discards in-memory state and re-reads the base
  scene from disk.

### `ouroforge.preview-delta.v1`

| Field | Notes |
| --- | --- |
| `schemaVersion` | `ouroforge.preview-delta.v1` |
| `deltaId` | deterministic digest of the delta seed |
| `sessionId`, `intentId` | correlation |
| `sequence` | session sequence after this delta (unchanged on rejection) |
| `kind` | echo of the intent kind |
| `status` | `applied` \| `rejected` |
| `edits` | fully normalized `SceneEdit` list; consumers apply mechanically |
| `beforeSceneHash` / `afterSceneHash` | canonical scene hashes; `afterSceneHash` absent on rejection |
| `errors` | rejection diagnostics |

Validation rejections (allowlist miss, unknown entity, type mismatch, invalid
candidate scene) are `rejected` deltas with diagnostics. Envelope errors
(wrong schema version, session mismatch) are hard errors, not deltas.

## Boundaries

This contract does not authorize browser trusted writes, command bridges,
auto-apply to the trusted worktree, remote/non-loopback binding, multi-client
sessions, transcript capture (M131.3), or any runtime client behavior
(M131.2). Scene mutation authority remains exclusively with the existing
review-gated apply flow.
