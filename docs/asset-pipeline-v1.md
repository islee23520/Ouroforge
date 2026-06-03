# Asset Pipeline v1 Scope and Contract

Asset Pipeline v1 / Content Authoring Foundation is the next local-first,
Rust-trusted, evidence-native milestone after Engine Expressiveness v2 and Source
Mutation Design Gate v1. It brings project game content assets into the same
bounded review loop as scenes, scenarios, runs, comparisons, journals, and
read-only Studio inspection.

This is a scope/control contract. It does not implement asset schemas, runtime
loading, preview generation, Studio asset behavior, browser uploads, file writes,
marketplaces, plugins, native export, remote hosting, source mutation, or
production-editor capabilities.

## Why this milestone comes next

Engine Expressiveness v2 made the local 2D runtime more expressive with bounded
scene components, collision/physics rules, triggers, HUD, animation/audio event
evidence, manifest-declared transitions, the collect-and-exit demo, scenario
coverage v3, and read-only Studio inspection.

Source Mutation Design Gate v1 completed a conservative design/control pass for
future source patch work and kept source mutation apply blocked. That outcome
makes Asset Pipeline v1 the safer next implementation foundation: it expands
content authoring through local project assets and generated evidence without
crossing into source-code mutation, browser-trusted writes, plugins, hosting, or
native export.

## Target outcome

Asset Pipeline v1 should make project assets reviewable, reproducible, and
inspectable:

```text
project asset files -> manifest/hash classification -> sprite/tileset/tilemap/audio/font refs
  -> runtime loading evidence -> preview evidence -> read-only Studio inspection
  -> playable demo asset refresh -> scenario coverage -> roadmap/#1 refresh
```

The milestone should answer:

1. Which local asset files belong to a project?
2. Which scene/tilemap/audio/font references are valid and hash-pinned?
3. Which generated evidence proves runtime loading and preview behavior?
4. Which read-only Studio views help inspect assets without writing files?
5. Which checks prove generated/local state remains untracked?

## Dependency order

Follow-up Asset Pipeline v1 issues should be implemented in this order:

1. **Asset Manifest v1** — define project-local asset manifest schema, asset ids,
   relative paths, file types, integrity hashes, CLI validation, and fixture policy
   (see `docs/asset-manifest-v1.md`).
2. **Sprite Atlas Manifest v1** — define sprite atlas metadata, frame regions,
   animation frame refs, deterministic ordering, and validation boundaries (see
   `docs/sprite-atlas-manifest-v1.md`).
3. **Tileset and Tilemap Authoring v2** — extend tilemap authoring around
   manifest-declared tilesets, layers, collision tags, and source-like fixtures.
   See `docs/tileset-tilemap-authoring-v2.md` for the schema, runtime evidence,
   dashboard read-model, and explicit non-goals.
4. **Asset Reference Integrity v1** — validate scene, animation, audio, font,
   tilemap, and scenario references against manifest ids and hashes.
5. **Runtime Asset Loading Evidence v1** — record generated evidence that runtime
   loading resolved declared local assets without remote fetches or browser
   authority.
6. **Asset Preview Evidence v1** — generate bounded preview evidence for sprites,
   atlases, tilesets, tilemaps, audio metadata, and missing/malformed refs.
7. **Studio Asset Inspector v1** — render exported asset/preview/reference state
   as escaped read-only Studio data with no uploads, writes, or commands.
8. **Playable Demo Asset Refresh** — update the local collect-and-exit demo to
   use manifest-backed assets and evidence without broad production claims.
9. **Scenario Coverage v4** — add regression coverage for asset manifest,
   loading, preview, reference integrity, malformed assets, and read-only Studio
   display contracts.
10. **Roadmap and #1 Governance Refresh after Asset Pipeline v1** — record
    milestone outcome, preserve #1/#23 open, and decide next sequencing.

Later issues may split implementation details further, but they should not skip
or merge these dependency categories without an explicit governance comment.

## Local asset fixture policy

Asset Pipeline v1 may add tiny deterministic source-like fixtures only when they
are explicitly scoped by a follow-up issue. Allowed fixture classes include:

| Fixture class | Boundary |
| --- | --- |
| Small PNG sprites/atlases | Local source fixtures only; deterministic dimensions and hashes. |
| Small JSON manifests | Schema examples for asset ids, refs, hashes, and metadata. |
| Tiny tilemap/tileset files | Local source fixtures tied to manifest ids and scene refs. |
| Minimal audio/font metadata fixtures | Prefer metadata/reference fixtures; binary audio/font additions require explicit issue scope. |
| Scenario packs for assets | Source-like regression fixtures only; generated run evidence remains ignored. |

Fixtures must avoid large binary blobs, private/proprietary assets, remote URLs,
license ambiguity, generated caches, and platform-specific outputs. Every tracked
fixture should have a clear purpose, deterministic validation path, and bounded
size.

## Generated preview/output policy

Generated asset previews, runtime load reports, transformed images, extracted
metadata, dashboard exports, run artifacts, local caches, and temporary project
outputs remain generated/local state. They should live under ignored generated
roots such as `runs/`, `target/`, or explicitly documented generated dashboard
outputs unless a follow-up issue scopes a tiny deterministic fixture as tracked
source-like data.

Generated evidence should record enough provenance for review:

- source asset id and path;
- manifest hash and observed file hash;
- command/run id that produced the evidence;
- missing/malformed/unsupported state when applicable;
- runtime/preview status and bounded diagnostics; and
- cleanup/generated-state expectations.

## Rust-trusted / browser-read-only boundary

Rust or another explicitly trusted local CLI boundary owns validation,
persistence, manifest parsing, integrity checks, and generated evidence writing.
Browser/Studio surfaces may display exported asset data and preview evidence only
as escaped read-only state. They must not upload assets, write trusted files,
execute commands, install packages, fetch remote assets, edit manifests, promote
assets, or act as a command bridge.

## Verification gates for follow-up issues

Every Asset Pipeline v1 implementation issue should define focused verification
for its changed surface plus the broad gates:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short --ignored
```

If dashboard or Studio UI files change, the issue should also run the relevant
Node checks:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Closure evidence should include #1/#23 state, no generated-state drift, fixture
size/purpose audit when fixtures are added, and proof that browser surfaces remain
read-only.

## Explicit non-goals

Asset Pipeline v1 does not authorize:

- asset marketplace, plugin loading, dynamic extension APIs, or user asset store;
- remote hosting, cloud asset storage, CDN integration, accounts, auth, or
  browser upload flows;
- native export, packaged editor, production-editor claims, compatibility-stable
  engine API promises, public launch automation, or Godot replacement claims;
- browser trusted file writes, command bridges, local server write APIs, hidden
  command execution, dependency installation, or credentialed commands;
- source mutation apply, arbitrary patch apply, auto-merge, auto-apply,
  auto-accept, or source patch generator behavior;
- large or licensed third-party binary asset drops without explicit governance;
  or
- closing, replacing, or narrowing #1 or #23.

## Closure policy for this milestone

Asset Pipeline v1 is complete only when its ordered follow-up issues are merged,
latest-main verification passes, generated/local artifacts remain untracked, and
the roadmap/#1 governance refresh records the outcome. Until then, this document
is a planning contract, not a claim that the asset pipeline is production-ready.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. Both remain open unless a separate explicit governance decision
says otherwise.
