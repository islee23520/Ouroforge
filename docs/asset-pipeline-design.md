# Asset Generation and Asset-QA v1 Scope and Design Gate

Issue: **#1634**

Asset Generation and Asset-QA v1 defines how Ouroforge gains a **verified visual-asset
function** (sprites, tilesets, UI art, animation, VFX) without asset slop or license risk.
A generated asset is a **proposal** carrying license/provenance, routed through the
existing review/apply/trust-gradient path; it is promoted only after a function-specific
**asset-QA gate** passes. The function is a **composition** of surfaces that already
exist — the existing review/apply/trust-gradient path (`source_apply_*` /
`trust_gradient_*`), the evaluator four-gate aggregation, the `compare` run-comparison
surface, the provenance bundle, and the existing Asset Pipeline v1 manifest/loader/atlas
surface (`asset validate`, `world_state.assetManifest.*`). It builds **no second asset
engine** and adds **no executable behavior** in this issue.

This is a scope/design-gate document for Era G Milestone 36. Following the project idiom
(ADR #92, Native Export Design Gate #168, Trust Gradient #1476). It adds no executable
behavior, no auto-fix, no auto-apply, no auto-merge, no self-approval, no reviewer bypass,
and no trusted browser/Studio mutation. It defines contracts, boundaries, and the follow-up
sequence only.

## Decision: bounded GO

The default for generated assets is **DEFER**. This gate records a **bounded GO** for one
narrowly-scoped path and **DEFER** for everything outside it.

**GO (bounded):** Generated visual assets are permitted **only** as proposals that

1. enter through the existing review/apply/trust-gradient path (never a direct trusted
   write), and
2. carry complete, verifiable **license/provenance**, and
3. pass the function-specific **asset-QA gate** (style-consistency, format/resolution
   validity, visual-regression vs baseline, and license/provenance completeness) which
   composes with the existing evaluator four gates, and
4. receive a human review/apply decision before promotion.

All four conditions are mandatory and the gate **fails closed**: a missing license, a
missing provenance chain, a failed QA check, or a blocked/invalid input means the asset is
**not** promoted.

**DEFER (default for everything else):** auto-apply of generated assets; promotion of
unlicensed/uncredited/unverified-style assets; any art-direction, taste, "looks good", or
"is fun" automation; a hosted/paid asset store or marketplace transaction layer (Layer-3,
DEFER per #1508); and any new generation engine/runtime/writer. Distributed/Elixir remains
NO-GO per ADR #92.

## Goals

- Define the **generated-asset policy**: license/provenance, attribution, and allowed
  sources, with an explicit GO/DEFER.
- Define the **asset-proposal contract**: generated assets are proposals carrying
  license/provenance, routed through the existing review/apply/trust-gradient path.
- Define the **asset-QA gate contract**: style-consistency, format/resolution validity,
  visual-regression vs baseline, and license/provenance completeness, composing with the
  existing four gates.
- Define the **asset import/atlas contract**, reusing the existing runtime asset
  manifest/loader.
- Define the follow-up dependency order and closure gates; preserve #1 and #23 as open
  governance anchors.

## Non-goals

- No implementation of the Rust/local generation-proposal model, asset-QA gate, import
  path, demo, coverage suite, or governance refresh in this issue. Those belong to
  #1635–#1640.
- No new generation engine, asset runtime, asset writer, comparison engine, evaluator, or
  manifest/loader. The function reuses the existing review/apply/trust-gradient path,
  evaluator four gates, `compare`, provenance bundle, and Asset Pipeline v1
  manifest/loader/atlas (`asset validate`).
- No direct trusted write from generation, role agents, the producer, or any
  browser/Studio surface; proposals only, through the existing path. Browser/Studio
  surfaces remain read-only for trusted state.
- No autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted write;
  high-risk and source-affecting changes are never auto-applied.
- No promotion of any unlicensed, uncredited, or unverified-style generated
  asset/audio/content; license/provenance and the asset-QA gate are mandatory.
- No automated quality/fun/taste claim; "looks good / sounds good / is fun" and
  art/audio/UX/narrative direction remain human decisions.
- No claim of a production-ready engine, Godot replacement/parity, or autonomous shipping
  of finished games.
- No shipping (native/store export), hosted/cloud, real-player telemetry, live-ops, or a
  hosted/paid asset store absent an explicit Layer-3 GO (DEFER per Milestone 26 / #1508);
  distributed/Elixir remains NO-GO per ADR #92.
- No engine/content/system breadth beyond what a specific loop-produced rung
  (Milestone 24) justifies.
- No generated runs/assets/content/release artifacts committed unless explicitly
  fixture-scoped.

## Generated-asset policy

Every generated visual asset is governed by this policy before it can be promoted.

### License and provenance

| Field | Requirement |
| --- | --- |
| `license` | An explicit, recognized license identifier (e.g. `CC0-1.0`, `CC-BY-4.0`, project-owned). Absent or unrecognized → **fail closed**, not promoted. |
| `attribution` | Required when the license demands it (e.g. CC-BY). A missing required attribution is a promotion blocker. |
| `source` | The provenance chain: generator/model identity, prompt or input reference, and any upstream-asset references. Recorded through the existing provenance bundle (`provenance_bundle.rs`); no new provenance format. |
| `allowedSource` | The source must be on the **allowed-sources** list (project-owned, explicitly-licensed generator output, or licensed third-party input). An off-list source is **DEFER**, not promoted. |

License/provenance completeness is **mandatory** and is one of the four asset-QA checks
below. Unknown, malformed, or missing fields are never coerced into "licensed"; they
produce an explicit failure.

### Allowed sources

- Project-owned original assets.
- Generator/model output where the generator's terms grant the project a usable license
  **and** the provenance chain (model, prompt/input) is recorded.
- Licensed third-party input whose license is recorded and compatible.

Anything else (unknown-license scraped assets, "style-of" outputs that launder a
copyrighted style without a license, off-list sources) is **DEFER** and is never promoted.

## Asset-proposal contract

A generated asset is a **proposal**, never a direct trusted write.

| Step | What happens | Reused surface |
| --- | --- | --- |
| 1. Propose | Generation emits an asset proposal carrying the asset bytes (fixture-scoped), its target manifest entry, and its license/provenance metadata. | generation proposal output (proposal-only) |
| 2. Route | The proposal enters the existing review/apply/trust-gradient path. It is never a direct trusted write. | `source_apply_*` / `trust_gradient_*` review/apply path |
| 3. Gate | The asset-QA gate runs (below). A failure blocks promotion; the gate fails closed. | evaluator four gates + `compare` + provenance bundle |
| 4. Review | A human review/apply decision promotes or rejects the proposal. High-risk/source-affecting changes are never auto-applied. | existing review decision / trust-gradient tier |
| 5. Import | On promotion, the asset is bound through the existing asset manifest/loader (import/atlas contract below). | Asset Pipeline v1 manifest/loader (`asset validate`) |
| 6. Surface | The proposal, its QA verdict, and its provenance are surfaced **read-only**. | dashboard / cockpit read-only panels, `window.__OUROFORGE__` probe |

The proposal path introduces no new apply, review, or trust-gradient engine; it reuses the
existing one and adds the asset-QA gate as a precondition to promotion.

## Asset-QA gate contract

The asset-QA gate is the function-specific verification gate. It **composes with** the
existing evaluator four gates (static, runtime, visual, semantic) aggregated via
`declared-gate-and` with `undeclaredGatePolicy: neutral`; it does **not** add a fifth
parallel aggregator. Style-consistency and visual-regression register through the existing
**visual** gate surface; format/resolution and license/provenance completeness register as
declared checks. An undeclared asset-QA dimension stays neutral, never silently passing.

| Check | What it verifies | Reused surface | Fail-closed behavior |
| --- | --- | --- | --- |
| Style-consistency | The asset matches the project's declared style baseline (palette, scale, perspective) within tolerance. | visual gate + `compare` baseline diff | Out-of-style → fail; not promoted. Style is matched against a declared baseline; the gate does **not** assert the style is "good". |
| Format/resolution validity | Format, dimensions, atlas frame bounds, and integrity (hash/schema) are valid for the target manifest entry. | Asset Pipeline v1 manifest/atlas validation (`asset validate`) | Invalid format/oversize/out-of-bounds frame → fail; not promoted. |
| Visual-regression vs baseline | The asset does not regress against the prior accepted baseline for its slot. | `compare` (`compare_runs`, `write_run_comparison_artifact`, `RunComparison`, `RunComparisonComparability`) | A missing/malformed/non-comparable baseline produces an explicit `insufficient-data`/`unsupported` verdict, never a silent pass. |
| License/provenance completeness | License, required attribution, source, and allowed-source are present and verifiable per the policy above. | provenance bundle (`provenance_bundle.rs`) | Any missing/unrecognized field → fail; not promoted. |

Rules:

- Promotion requires **all four** asset-QA checks to pass **and** the existing four gates to
  pass **and** a human review/apply decision. The gate fails closed on blocked writes,
  invalid inputs, and missing license.
- The gate detects regressions and verifies license/format/style-baseline conformance; it
  does **not** assert the asset is good, beautiful, on-brand-by-taste, or fun. Those remain
  human decisions.
- Unknown, malformed, stale, or non-comparable inputs are never coerced into a pass; they
  produce an explicit `insufficient-data` or `unsupported` verdict.

## Asset import and atlas contract

Promoted assets are bound through the **existing** Asset Pipeline v1 manifest/loader; this
function adds no second importer.

| Concern | Reused surface | Boundary |
| --- | --- | --- |
| Manifest binding | The existing asset manifest and `world_state.assetManifest.*` / `assetManifestId` runtime fields, validated by Rust (path/hash/schema). | Rust-trusted local path/hash/schema validation only; no remote fetch, no browser trust escalation. |
| Atlas/frame | The existing atlas frame-bounds validation and animation evidence smoke. | No sprite editor or atlas generation; bounds validation only. |
| Tilemap | The existing tile authoring extraction (`tilemaps[*].authoring.*Cells`). | Read-only tile authoring evidence; no visual editor. |
| Runtime load | The existing runtime asset loader (`assets.load(world, world.assetManifest)`, `assets.manifestSummary()`). | Browser observes local manifest refs read-only; it does not upload/write/fetch remote assets. |
| CLI | The existing `asset validate` command. | Local validation only. |

A promoted generated asset is just a manifest entry the existing loader already
understands; nothing in the runtime asset path changes.

## Artifact shape

Future implementation issues (#1635 onward) own the Rust structs and serializers. This
contract fixes the additive artifact shape they must preserve. The asset-proposal/QA
artifact **references** existing manifest, comparison, and provenance artifacts; it does
not duplicate their contents.

```json
{
  "schemaVersion": "asset-generation-qa-v1",
  "projectId": "collect-and-exit",
  "proposalId": "asset-proposal-789",
  "assetKind": "sprite | tileset | ui-art | animation | vfx",
  "target": {
    "manifestEntry": "assets/sprites/hero.png",
    "proposalRef": "review/apply proposal reference (never a direct trusted write)"
  },
  "license": {
    "license": "CC0-1.0 | CC-BY-4.0 | project-owned",
    "attribution": "required-string-or-null",
    "source": "generator/model + prompt/input + upstream refs",
    "allowedSource": true
  },
  "assetQa": {
    "styleConsistency": "pass | fail | insufficient-data | unsupported",
    "formatResolution": "pass | fail",
    "visualRegression": "pass | fail | insufficient-data | unsupported",
    "licenseProvenance": "pass | fail",
    "compareRef": "runs/run-789/evidence/compare/asset-789.json",
    "manifestValidationRef": "runs/run-789/evidence/asset/asset-789.json",
    "provenanceRef": "runs/run-789/evidence/provenance/asset-789.json"
  },
  "gateComposition": {
    "operator": "declared-gate-and",
    "undeclaredGatePolicy": "neutral"
  },
  "verdict": "promotable | blocked | insufficient-data | unsupported",
  "reviewDecisionRef": "human review/apply decision (required before promotion)"
}
```

The artifact is additive and backward-compatible. It introduces no breaking change to the
existing run, manifest, comparison, provenance, evidence, evaluator, or dashboard
contracts.

## Reuse contract

The function composes existing surfaces. It introduces no parallel engine. If a future
follow-up appears to need a new generation engine, asset runtime, importer, comparison
engine, evaluator, or provenance format, that is a signal to extend the existing surface —
not to add a parallel system — unless the issue includes an explicit, justified migration
note.

| Concern | Reused surface |
| --- | --- |
| Proposal / apply / trust tier | `source_apply_*` / `trust_gradient_*` review/apply/trust-gradient path |
| Gate aggregation | Evaluator four gates (static, runtime, visual, semantic), `declared-gate-and`, `undeclaredGatePolicy: neutral` |
| Visual-regression diff | `compare` (`compare_runs`, `write_run_comparison_artifact`, `RunComparison`, `RunComparisonComparability`) |
| License / provenance | `provenance_bundle.rs` and existing evidence/trace writers |
| Import / atlas / manifest | Asset Pipeline v1 manifest/loader/atlas, `world_state.assetManifest.*`, `asset validate` |
| Surface | Dashboard/cockpit read-only panels and the `window.__OUROFORGE__` probe / runtime |

## Language boundary

- **Rust/local** owns trusted validation, persistence, the generation-proposal model, the
  asset-QA/curation/orchestration/provenance/compliance logic, evidence writing,
  run/project binding, the review/apply/trust-gradient path, and CLI behavior.
- **TypeScript/JavaScript** owns the deterministic runtime (including in-game UI/HUD/menus),
  the `window.__OUROFORGE__` probe, browser-local read-only inspection, and the static
  dashboard/cockpit read-only surfacing of asset proposals/verdicts where explicitly
  scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and must not own
  any core Era G/H contract.
- No new language/runtime is introduced. Distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Compatibility

- The function is **additive**. Existing runtime, probe, evaluator four-gate aggregation,
  Asset Pipeline v1 manifest/loader/atlas, evolve/campaign, `compare`, provenance-bundle,
  dashboard, cockpit, and CLI contracts remain backward-compatible unless a later issue
  includes an explicit migration note.
- Generated assets, runs, comparison artifacts, traces, previews, screenshots, videos,
  browser state, and local dashboard exports remain **untracked** unless intentionally
  fixture-scoped.
- Existing manual and proposal workflows remain valid; this function describes how
  generated assets are proposed, gated, and imported — it does not change how non-generated
  assets are authored or validated.

## Follow-up dependency order and closure gates

| Order | Issue | Closure gate |
| --- | --- | --- |
| 1 | #1634 Asset Generation and Asset-QA v1 Scope and Design Gate (this issue) | This document exists; records the generated-asset policy and the bounded GO/DEFER; defines the asset-proposal, asset-QA, and import/atlas contracts reusing existing surfaces; passes the governance/wording audit; #1 and #23 remain open. |
| 2 | #1635 Asset Generation Proposal Model v1 | Adds the Rust/local generation-proposal model routed through the existing review/apply/trust-gradient path, carrying license/provenance. Proposal-only; no direct trusted write. **Prerequisite: #1593 merged.** |
| 3 | #1636 Asset-QA Gate v1 | Adds the Rust/local asset-QA gate (style-consistency, format/resolution validity, visual-regression vs baseline via `compare`, license/provenance completeness) composing with the existing four gates. Fails closed. |
| 4 | #1637 Asset Import and Atlas Path v1 | Binds promoted assets through the existing manifest/loader/atlas (`asset validate`); no second importer. |
| 5 | #1638 Asset Generation and QA Demo v1 | Fixture-scoped demo: a licensed in-style asset passes and imports; an unlicensed, out-of-style, or regressing asset is blocked with a replayable verdict. |
| 6 | #1639 Scenario Coverage v34: Asset Pipeline Regression Suite | Enumerated, fixture-scoped regression suite for promotable/blocked/insufficient-data/unsupported and boundary cases. Continues Scenario Coverage numbering from v33 (Era F) onward. |
| 7 | #1640 Roadmap and #1 Governance Refresh after Asset Generation and Asset-QA v1 | Updates roadmap/#1 governance after the prior gates are merged and verified; confirms #1 and #23 remain open. |

Dependency chain:

```text
#1634 scope -> #1635 -> #1636 -> #1637 -> #1638 -> #1639 -> #1640
```

## Wording audit

Allowed wording:

- "verified visual-asset function"
- "generated asset is a proposal carrying license/provenance"
- "promoted only after the asset-QA gate passes"
- "reuses the review/apply/trust-gradient path, the four gates, `compare`, the provenance
  bundle, and Asset Pipeline v1; no second asset engine"
- "license/provenance and asset-QA are mandatory before promotion; the gate fails closed"
- "regression and conformance detection only, human-in-the-loop"
- "Rust/local owns the proposal model and verdict serialization; browser/Studio read-only"

Disallowed wording:

- "the gate proves an asset is good/beautiful/on-brand/fun"
- "production-ready asset pipeline" / "asset quality score"
- "Godot replacement/parity evidence"
- "automatic apply/merge authority for generated assets"
- "browser/Studio trusted write authority"
- "new generation/asset/comparison engine"
- "hosted/paid asset store" (Layer-3, DEFER)

## Governance

- Before starting, before merge or closure, and after merge or closure, verify #1634 state
  and confirm #1 and #23 remain open.
- #1 remains the roadmap anchor and #23 remains the memory anchor; this issue must not
  close or modify either anchor.
- Generation stays proposal-only through the existing review/apply/trust-gradient path;
  never a direct trusted write. Browser/Studio surfaces stay read-only. Generated
  assets/audio/content require license/provenance **and** the asset-QA gate before
  promotion.
- Genre/engine growth stays demand-driven (Milestone 24): no engine breadth beyond this
  rung's gate; cloud/hosted/marketplace/store monetization stays Layer-3 (DEFER per #1508);
  distributed/Elixir remains NO-GO per ADR #92.
- Future follow-up issues must cite this contract when changing the generated-asset policy,
  the asset-proposal model, the asset-QA gate, the import/atlas path, demos, regression
  suites, or governance.

**#1 and #23 remain open.**
