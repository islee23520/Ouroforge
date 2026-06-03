# Asset Manifest v1

Asset Manifest v1 is the Rust-trusted local asset inventory for Asset Pipeline
v1 / Content Authoring Foundation. It lets Ouroforge validate project-local asset
files before later scene/runtime/dashboard/Studio work refers to them by stable
ids and integrity metadata.

The manifest is local-first. It does not fetch remote assets, upload files, run
browser writes, package assets, load plugins, or implement runtime asset loading.

## Manifest shape

```json
{
  "schemaVersion": "asset-manifest-v1",
  "id": "asset_manifest_v1_fixture",
  "assets": [
    {
      "id": "player_sprite",
      "type": "image",
      "path": "assets/sprites/player.png",
      "contentHash": {
        "algorithm": "fnv1a64-file-v1",
        "value": "0123456789abcdef"
      },
      "classification": "source_like",
      "dimensions": { "width": 16, "height": 16 },
      "license": "CC0 test fixture",
      "source": "Ouroforge deterministic fixture"
    }
  ]
}
```

## Fields

| Field | Required | Meaning |
| --- | --- | --- |
| `schemaVersion` | yes | Must be `asset-manifest-v1`. |
| `id` | yes | Stable local manifest id using bounded Ouroforge id characters. |
| `assets[]` | yes | Asset entries. Empty manifests are rejected. |
| `assets[].id` | yes | Stable local asset id; duplicate ids are rejected. |
| `assets[].type` | yes | One of `image`, `sprite_atlas`, `tileset`, `tilemap`, `audio`, or `font`. |
| `assets[].path` | yes | Project-relative file path; duplicate paths are rejected. |
| `assets[].contentHash` | yes | File integrity hash. Algorithm must be `fnv1a64-file-v1`; value is 16 lowercase hex characters. |
| `assets[].classification` | yes | `source_like` for tracked source assets or `generated` for generated/local evidence assets. |
| `dimensions` | image-like optional metadata | Width and height must both be greater than zero when present. Sprite atlas image references require image dimensions for bounds validation. |
| `atlas` | `sprite_atlas` only | Sprite atlas payload with `imageAssetId`, named frame rectangles, and optional animation frame refs. See `docs/sprite-atlas-manifest-v1.md`. |
| `durationMs` | audio optional metadata | Duration must be greater than zero when present. |
| `license`, `source`, `metadata` | optional | Human review notes only; not remote authority. |

## Validation command

Validate either an asset manifest file or a project root directory containing
`asset-manifest.json`:

```bash
cargo run -p ouroforge-cli -- asset validate path/to/asset-manifest.json
cargo run -p ouroforge-cli -- asset validate path/to/project-root
```

A successful validation prints a deterministic summary:

```text
Asset manifest valid: cli_asset_fixture
Manifest: path/to/asset-manifest.json
Assets: 2
Source-like assets: 1
Generated assets: 1
Sprite atlases: 0
Sprite atlas frames: 0
Sprite atlas animations: 0
Asset types: image=2
```

Invalid manifests exit non-zero and include the failing path or integrity reason,
for example `contentHash mismatch`, `missing file`, `unsupported extension`, or
`generated root runs`.

## Path and integrity rules

Asset validation rejects:

- absolute paths;
- `..` traversal or paths that escape the project root;
- hidden path components and local tool/runtime roots such as `.git`, `.omx`,
  `.omc`, `.openchrome`, and `.claude`;
- source-like assets under generated roots such as `runs`, `target`, or
  `dashboard-data`;
- missing files;
- duplicate asset ids or duplicate asset paths;
- sprite atlas entries with missing image refs, duplicate frame ids, out-of-bounds frame rectangles, or unknown animation frame refs;
- unsupported file extensions for the declared type;
- non-`fnv1a64-file-v1` hash algorithms;
- malformed hash strings; and
- content hashes that do not match the observed local file bytes.

Generated-classified assets may be declared under generated roots so later
issues can validate preview/evidence files, but generated/local outputs remain
ignored unless a future issue explicitly scopes a tiny deterministic fixture.

## Supported extensions

| Type | Extensions |
| --- | --- |
| `image` | `png`, `jpg`, `jpeg`, `svg`, `webp` |
| `sprite_atlas` | `json` |
| `tileset` | `json` |
| `tilemap` | `json` |
| `audio` | `ogg`, `mp3`, `wav` |
| `font` | `ttf`, `otf`, `woff`, `woff2` |

Tileset/tilemap payloads may include bounded authoring metadata used by
`docs/tileset-tilemap-authoring-v2.md`; runtime/dashboard consumers treat the
extracted cells as read-only evidence, not editor write authority. Asset
Reference Integrity v1 is documented in
`docs/asset-reference-integrity-v1.md`; it covers scene reference warning
evidence for missing refs, stale hashes, invalid types, and unresolved ids.

## Boundary and non-goals

Asset Manifest v1 keeps trusted validation in Rust and does not authorize:

- runtime asset loading;
- Studio/dashboard/browser trusted writes, uploads, command bridges, or local
  server execution bridges;
- remote asset fetches, hosting, CDN, cloud storage, accounts, or marketplace
  behavior;
- plugin loading, dynamic code loading, native export, packaging, or asset bundle
  export;
- source-code mutation, arbitrary patch apply, dependency mutation, auto-apply,
  or auto-merge;
- production editor, visual asset editor, public launch automation, or Godot
  replacement claims; or
- closing or modifying #1 or #23.
