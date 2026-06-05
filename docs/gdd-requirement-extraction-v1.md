# GDD Requirement Extraction v1

Issue: #646  
Status: requirement extraction artifact, validation, fixtures, and display read model contract.

`gdd-requirement-extraction-v1` records manually or structurally extracted requirements from a GDD/design brief. It is a traceability artifact, not a prototype generator. Every requirement must cite a declared source section and a short source excerpt so reviewers can distinguish evidence from inference.

## Artifact shape

Top-level fields:

- `schemaVersion`: `gdd-requirement-extraction-v1`.
- `extractionId`: bounded local id.
- `status`: `ready`, `partial`, or `blocked`.
- `sourceArtifactRef`: local `examples/`, `docs/`, or `seeds/` reference to the source brief.
- `sourceSections`: source ids, titles, refs, and optional source text.
- `requirements`: extracted requirement rows.
- `boundary`: required wording that manual/structured extraction comes first, LLM extraction is advisory only, output is untrusted until Rust/local validation, later apply is review-gated, there is no autonomous unrestricted game creation, and the artifact is not prototype generation authority.

Requirement rows include `id`, `category`, `sourceSectionRef`, `sourceExcerpt`, `priority`, `confidence`, `ambiguityFlags`, `dependencyLinks`, `conflictsWith`, `blockedReasons`, and `evidenceBoundary`.

Allowed categories are deliberately bounded: `mechanic`, `control`, `entity`, `level`, `ui-hud`, `win-loss`, `constraint`, `non-goal`, `acceptance`, and `evidence`. Priorities are `must`, `should`, `could`, and `wont`. Confidence is a bounded reviewer signal from `0.0` to `1.0`; it is not a correctness, fun, or quality guarantee.

## Validation gates

Rust/local validation rejects:

- missing or duplicate source section ids;
- duplicate requirement ids;
- missing source refs;
- source excerpts absent from the cited source section text;
- invented/unlinked requirements without a source excerpt;
- dependency/conflict links to unknown requirements or self-links;
- conflicting requirements without visible blockers;
- ambiguous or low-confidence requirements without visible blockers;
- blocked artifacts with no blocked requirements;
- ready artifacts that still contain blockers;
- boundary wording that implies command bridges, browser trusted writes, auto-apply, auto-merge, native export, plugin runtime, asset generation, production readiness, or current Godot replacement scope.

## Fixtures

Valid/visible-state fixtures:

- `examples/gdd-requirement-extraction-v1/requirements.valid.fixture.json`
- `examples/gdd-requirement-extraction-v1/requirements.partial.fixture.json`
- `examples/gdd-requirement-extraction-v1/requirements.blocked.fixture.json`

Invalid fixtures cover missing source refs, duplicate ids, invented/no excerpt, excerpt drift, conflict without blockers, low confidence without blockers, and unsafe authority wording under `examples/gdd-requirement-extraction-v1/invalid/`.

## Read model

`GddRequirementExtractionArtifact::read_model()` produces display-only counts by status/category, blocker/ambiguity/dependency/conflict totals, validation summary text, compatibility notes, and the original boundary. Browser/dashboard/Studio consumers must treat this as inspection data only; it gives no trusted persistence, generation, command, or apply authority.

## Boundaries

This contract enables evidence-gated prototype planning later; it does not generate a prototype, apply a scene/behavior/source change, execute scripts, load plugins, generate assets, open a command bridge, perform browser trusted writes, or claim autonomous unrestricted game creation.

GDD-derived output remains untrusted until Rust/local validation and a later review-gated apply path explicitly accepts it. Generated prototype drafts, plans, reviews, applies, runs, evidence, screenshots, dashboard exports, temp projects, and local tool state remain ignored unless explicitly fixture-scoped.

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor.
