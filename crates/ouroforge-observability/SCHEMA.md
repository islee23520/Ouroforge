# Ouroforge live observability bundle schema v1

Status: M116.1 contract source for `runs/live-observability/<run-id>/` bundles.
Closure classification for M116.1 PR 1: contract-complete.

Generated bundles are untrusted evidence by default. The repository ignores `runs/`; only explicitly scoped fixtures under this crate may be tracked.

## Bundle layout

A bundle root is `runs/live-observability/<run-id>/` for generated evidence, or a fixture directory that preserves the same file names.

Required artifacts:

- `manifest.json`
- `console.jsonl`
- `frame-stats.jsonl`
- `world-samples.jsonl`
- `events.json`
- `input-replay.json`
- `screenshots/`
- `verdict.md`

Optional artifacts may be added only by updating this schema with a new `schema_version` or a versioned optional field.

## `manifest.json`

Required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | string | Must be `live-observability-v1`. |
| `run_id` | string | Must match the bundle directory name. |
| `created_at` | string | RFC3339/ISO-like timestamp string ending in `Z`; validation checks shape, not wall-clock truth. |
| `target_url` | string | Must be local-only: `http://127.0.0.1:<port>/...` or `http://localhost:<port>/...`. |
| `run_kind` | string | One of `runtime`, `studio`, `authoring`. |
| `tool_versions` | object | Records harness/schema/tool versions. |
| `browser` | object | Records browser family/channel/version when known. |
| `retry_attempts` | number | Browser launch/connect attempts used by the runner. |
| `artifact_inventory` | array | Each required artifact path with digest metadata. |

Artifact inventory entries require:

- `path`: relative bundle path using `/` separators.
- `kind`: `json`, `jsonl`, `markdown`, `directory`, `png`, or `other`.
- `sha256`: lowercase 64-hex digest for files; omitted or `null` for directories and for `manifest.json` because the manifest inventories itself.
- `required`: boolean.

The inventory must include every required artifact path above.

## JSONL schemas

Every line in `console.jsonl`, `frame-stats.jsonl`, and `world-samples.jsonl` is a JSON object with:

- `schema_version`: `live-observability-v1`
- `timestamp`: timestamp string ending in `Z`

Additional fields are allowed only after this schema documents them. Downstream sampler additions must flow through this crate rather than inventing a second schema.

### Console line fields

Recommended current fields: `level`, `text`, `source`, `args`.

### Frame stats line fields

Recommended current fields: `frame`, `fps`, `delta_ms`, `dropped_frames`.

### World sample line fields

Recommended current fields: `tick`, `scene_id`, `player`, `goal_flags`, `recent_events`, `screenshot`.

## `events.json`

A JSON object with:

- `schema_version`: `live-observability-v1`
- `events`: array of event objects

## `input-replay.json`

A JSON object with:

- `schema_version`: `live-observability-v1`
- `steps`: array of replay step objects

## `verdict.md`

Human-readable report. M116.4 owns the generated verdict renderer, but M116.1 requires this file so missing verdicts fail validation.

## Retention and generated-state policy

- Generated live bundles stay under ignored `runs/live-observability/` unless copied into an explicitly scoped fixture directory.
- Browser profiles, screenshots, package output, and raw run output are generated/untrusted unless a review explicitly promotes a minimal fixture.
- Validators must not treat generated evidence as source authority.

## Local URL policy

Only these target URL prefixes are valid:

- `http://127.0.0.1:<port>/`
- `http://localhost:<port>/`

HTTPS, remote hosts, filesystem URLs, data URLs, and command-like URLs fail closed.
