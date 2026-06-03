# Asset Pipeline v1 Governance Handoff

Asset Pipeline v1 is complete as a bounded local content-authoring milestone. It
adds Rust-trusted local asset manifests, sprite atlas metadata, tileset/tilemap
authoring metadata, reference integrity, runtime asset loading evidence, asset
preview/read-model evidence, Studio asset inspection, asset-backed playable-demo
evidence, and Scenario Coverage v4 regression fixtures.

This handoff records the roadmap/#1 governance position after #342. It is not a
public launch approval, production-readiness claim, native export decision,
plugin/marketplace decision, or source mutation authorization.

## Completed milestone evidence

- Asset Pipeline scope/control contract: `docs/asset-pipeline-v1.md`
- Asset manifest validation: `docs/asset-manifest-v1.md`
- Sprite atlas metadata: `docs/sprite-atlas-manifest-v1.md`
- Tileset/tilemap authoring: `docs/tileset-tilemap-authoring-v2.md`
- Asset reference integrity: `docs/asset-reference-integrity-v1.md`
- Runtime asset loading evidence: `docs/runtime-asset-loading-evidence-v1.md`
- Asset preview evidence: `docs/asset-preview-evidence-v1.md`
- Studio asset inspector: `docs/studio-asset-inspector-v1.md`
- Asset-backed playable demo refresh:
  `docs/playable-demo-v2-collect-and-exit.md`
- Scenario Coverage v4: `docs/scenario-coverage-v4-asset-pipeline.md`

## Governance decision

- #1 remains open as the broad vision and evidence-native implementation-roadmap
  anchor.
- #23 remains open as the repo-memory/design context anchor.
- The recommended next implementation sequence is Visual Authoring v1
  (#343-#354), then Source Mutation Preview v1 (#356-#366) as inert
  preview/evidence work only, then Public Alpha Readiness (#367-#377), then
  Public Alpha Launch Governance (#378-#387).
- Native Export Design Gate and Plugin Design Gate remain possible later
  governance topics, but Asset Pipeline v1 completion does not authorize native
  export, packaged editor behavior, plugin runtime, marketplace behavior, dynamic
  extension loading, or package publishing.
- Source mutation apply remains blocked. Asset Pipeline v1 completion does not
  authorize arbitrary patch apply, branch mutation, dependency mutation,
  auto-merge, browser command bridges, or trusted browser writes.

## Reviewed drift surfaces

- `README.md` now includes Asset Pipeline v1 as completed local evidence and
  links this handoff.
- `docs/roadmap.md` now lists Asset Pipeline v1 under completed evidence-native
  milestones and moves the next recommendation to Visual Authoring v1.
- `docs/asset-pipeline-v1.md` now records completion while preserving non-goals.
- `docs/playable-demo-v2-collect-and-exit.md` and
  `docs/scenario-coverage-v4-asset-pipeline.md` remain local-first demo and
  regression evidence references, not public launch claims.
- `docs/public-readiness-audit.md`, `docs/public-launch-checklist.md`, and
  `docs/public-demo-evidence.md` remain public-readiness inputs only; visibility
  changes still require a separate manual maintainer decision.

## Non-goals preserved

Asset Pipeline v1 completion does not authorize:

- remote asset hosting, CDN, accounts, cloud storage, or browser upload flows;
- marketplace, plugin loader, dynamic extension API, or user asset store;
- native export, packaging, packaged editor, or production editor behavior;
- browser trusted file writes, command bridges, local server write APIs, hidden
  command execution, dependency installation, or credentialed commands;
- source mutation apply, arbitrary patch apply, auto-merge, auto-apply,
  auto-accept, or branch/dependency mutation;
- public launch automation, repository visibility changes, compatibility-stable
  engine API promises, broad production readiness, or Godot replacement claims.

## Verification expectation

Closure of #342 still requires latest-main verification, generated-state audit,
#1/#23 open-state evidence, a final issue comment, and no generated local state
committed. The handoff is a governance record, not a replacement for that final
issue evidence.
