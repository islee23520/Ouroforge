# Godot-Plus Demo Asset Pack v1

Issue: #786
Status: **GPD12.9 asset-pack contract.** This document records the minimal,
provenance-safe asset pack for the Godot-Plus Demonstration Game v1 vertical slice
(Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on #780–#785. The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## Asset pack

The pack reuses the existing deterministic CC0 fixtures already integrated with
`asset-manifest.json` (`asset-manifest-v1`), each carrying an `fnv1a64-file-v1`
content hash, classification `source_like`, and license/source notes:

| Asset | Type | License |
| --- | --- | --- |
| `collect_and_exit_sheet` | image (sprite sheet) | CC0-1.0 |
| `collect_and_exit_atlas` | sprite_atlas | CC0-1.0 |
| `collect_and_exit_tileset` | tileset | CC0-1.0 |
| `collect_and_exit_tilemap` | tilemap | CC0-1.0 |
| `collect_sound` | audio | CC0-1.0 |

To keep the pack minimal and avoid asset bloat / copyright risk, **no new binary
assets** are added. The demo HUD is text-based; UI icons reuse existing atlas
frames (key/door/exit) rather than new icon binaries.

## Provenance / license

`asset-provenance.json` (`demo-asset-provenance-v1`) is a consolidated provenance
record: every asset is an `authored-in-repo`, deterministic CC0-1.0 fixture with
`copyrightRisk: none`. No external, third-party, or copyrighted assets are used,
and no asset upload/fetch/import or marketplace behavior is introduced.

## Hash / missing / duplicate checks

`asset-pack-smoke.test.cjs` is the missing/duplicate/hash-integrity audit. For
every manifest asset it asserts:

- the file exists;
- the committed `fnv1a64-file-v1` hash matches the file bytes (recomputed in
  Node, matching the Rust `fnv1a64`);
- there are no duplicate asset ids or paths;
- license, source, and `source_like` classification are present;
- the provenance record covers the asset with `copyrightRisk: none`;
- atlas/tileset/tilemap cross-references resolve.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
node examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs
```

Rust (`asset validate`) remains the trusted hash/integrity validator; the Node
smoke adds missing/duplicate/provenance coverage.

## Boundaries

The asset pack is minimal, fixture-scoped, and provenance-safe. It adds no
external/copyrighted assets, no new binaries, no asset upload/fetch/import, no
marketplace, no committed generated output, no trusted browser write, no
production/native/store export, and no full Godot parity / replacement /
production-ready / commercial-release claim. #1 and #23 remain open.
