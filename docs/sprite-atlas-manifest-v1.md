# Sprite Atlas Manifest v1

Sprite Atlas Manifest v1 extends `asset-manifest-v1` with a declarative
`sprite_atlas` asset subtype. It records which local image asset contains a set
of named frames and which animation clips reference those frames.

This contract is validation/read-only metadata only. It does not pack images,
generate atlases, load runtime assets, edit files in the browser, upload assets,
or provide marketplace/plugin/native-export behavior.

## Manifest shape

A sprite atlas is an asset entry with `type: "sprite_atlas"` and an `atlas`
payload:

```json
{
  "id": "player_sheet_atlas",
  "type": "sprite_atlas",
  "path": "assets/atlases/player-sheet.atlas.json",
  "contentHash": {
    "algorithm": "fnv1a64-file-v1",
    "value": "1111111111111111"
  },
  "classification": "source_like",
  "atlas": {
    "imageAssetId": "player_sheet_image",
    "frames": [
      { "id": "idle_0", "rect": { "x": 0, "y": 0, "width": 16, "height": 16 } },
      { "id": "idle_1", "rect": { "x": 16, "y": 0, "width": 16, "height": 16 } }
    ],
    "animations": [
      {
        "id": "idle",
        "frames": [
          { "frameId": "idle_0", "durationMs": 120 },
          { "frameId": "idle_1", "durationMs": 120 }
        ]
      }
    ]
  }
}
```

The referenced `imageAssetId` must name an `image` asset in the same asset
manifest. The image asset must include `dimensions` so Rust validation can prove
frame rectangles are within the image bounds.

## Validation

`cargo run -p ouroforge-cli -- asset validate <project-or-manifest>` validates
sprite atlas entries as part of Asset Manifest v1 validation.

Validation rejects:

- a `sprite_atlas` asset without an `atlas` payload;
- an `atlas` payload on non-`sprite_atlas` asset types;
- missing or non-image `imageAssetId` references;
- image references without dimensions;
- empty `frames` arrays;
- duplicate frame ids;
- zero-width or zero-height frame rectangles;
- frame rectangles outside the referenced image bounds;
- empty animation `frames` arrays;
- zero-duration animation frame entries; and
- animation frame refs that do not match declared frame ids.

Validation also keeps all Asset Manifest v1 path/hash rules: local relative paths,
no traversal, no hidden/local tool roots, source-like assets outside generated
roots, supported extensions, existing files, duplicate asset rejection, and
matching `fnv1a64-file-v1` hashes.

## Read-only summary

The CLI prints deterministic read-only counts for atlas metadata:

```text
Sprite atlases: 1
Sprite atlas frames: 2
Sprite atlas animations: 1
```

These are summaries of already validated local manifest data. They are not editor
state, generated preview output, runtime load evidence, or browser authority.

## Fixtures

Tracked schema fixtures live under `examples/sprite-atlas-manifest-v1/`:

- `asset-manifest.valid.json` contains one image asset and one sprite atlas asset
  with two frames and one animation.
- `invalid/` contains schema-level rejection fixtures.

The fixture JSON files are source-like contracts only. They do not include
runtime packing output or generated previews.

## Non-goals

Sprite Atlas Manifest v1 does not authorize:

- runtime sprite atlas packing, texture loading, or animation playback behavior;
- generated atlas build steps, generated previews, or committed run output;
- Studio/dashboard/browser editing, uploads, writes, command bridges, or local
  server execution bridges;
- remote asset hosting, CDN, cloud storage, account systems, marketplaces, or
  plugin loading;
- native export, packaging, asset bundle export, production editor behavior, or
  public launch automation;
- source-code mutation, arbitrary patch apply, dependency mutation, auto-apply,
  or auto-merge; or
- closing or modifying #1 or #23.
