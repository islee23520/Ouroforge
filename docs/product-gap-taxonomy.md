# Product gap taxonomy and severity model

Machine-readable source of truth: `docs/product-gap-taxonomy.json`.

This document renders the single category/severity enum source for #2351, #2381, #2383, and #2390. Do not copy or fork the enum values into separate schemas; consumers must reuse the ids from the JSON file.

## Severity levels

### `blocking`

Prevents the scoped user/product flow from being completed or makes the claim materially false.

Observable example: Runtime cannot be launched from the documented entry point; editor cannot open the project; agentic apply lacks review gate evidence.

### `major`

The flow can complete only with significant workaround, missing diagnostics, or evidence gaps that would mislead product-observed closure.

Observable example: Gameplay can finish, but console errors, missing frame stats, or absent replay prevent a product-observed pass.

### `polish`

The core flow works and evidence exists, but usability, clarity, presentation, or ergonomics fall below practical quality expectations.

Observable example: HUD wording or visual alignment is confusing but does not block the observed scenario.

## Categories

### `runtime_ux` — Runtime UX

Can a user launch, understand, and recover from the running game loop without fixture-specific knowledge?

Default owner: `runtime` — Runtime/data-plane owner: Rust contracts plus browser runtime evidence, depending on surface.

Example finding: A user cannot find how to start/restart/play the loop.

### `renderer_quality` — Renderer Quality

Does the rendered output communicate game state clearly enough for practical play and review?

Default owner: `runtime` — Runtime/data-plane owner: Rust contracts plus browser runtime evidence, depending on surface.

Example finding: Sprites, camera, scaling, or visual ordering make gameplay unreadable.

### `input_control` — Input / Control

Do input mappings, responsiveness, replay, and control affordances support the claimed gameplay/editor action?

Default owner: `runtime` — Runtime/data-plane owner: Rust contracts plus browser runtime evidence, depending on surface.

Example finding: Required actions are missing, laggy, undiscoverable, or not replayable.

### `scene_authoring` — Scene Authoring

Can scenes, levels, entities, components, and layout edits be authored and reviewed through the scoped product flow?

Default owner: `studio` — Studio/editor owner: presentation/control-plane UX with Rust-owned artifact truth and gated writes.

Example finding: A level change requires hand-editing fixture JSON despite an editor-authoring claim.

### `asset_workflow` — Asset Workflow

Can assets be referenced, previewed, validated, and diagnosed in the scoped product flow?

Default owner: `studio` — Studio/editor owner: presentation/control-plane UX with Rust-owned artifact truth and gated writes.

Example finding: Missing/invalid assets fail silently or previews cannot be tied to source refs.

### `behavior_scripting` — Behavior / Scripting

Can gameplay behavior be expressed, constrained, verified, and debugged without unsafe arbitrary code expansion?

Default owner: `core` — Core gameplay/data owner: Rust validation and deterministic evidence contracts.

Example finding: A behavior claim depends on opaque scripts or has no deterministic assertion path.

### `editor_workflow` — Editor Workflow

Can Studio/editor users navigate projects, inspect state, draft changes, and understand gated writes?

Default owner: `studio` — Studio/editor owner: presentation/control-plane UX with Rust-owned artifact truth and gated writes.

Example finding: The editor surface displays data but gives no usable workflow for the claimed task.

### `agentic_mutation` — Agentic Mutation

Can proposal, review, apply/rerun, rollback, and backlog steps be observed and bounded end to end?

Default owner: `agentic` — Agentic-loop owner: proposal/review/apply/rerun evidence and human gate preservation.

Example finding: An agent proposes changes without before/after evidence or human review gate proof.

### `qa_evaluator_depth` — QA / Evaluator Depth

Do evaluators catch mechanical, runtime, visual, semantic, regression, and product-observed failures with honest gaps?

Default owner: `qa` — QA/evaluator owner: failure taxonomy, scenario coverage, verdict quality, and gap escalation.

Example finding: Green tests miss a visible product failure or collapse multiple failure modes into one vague verdict.

### `export_package_reality` — Export / Package Reality

Does export/package evidence reflect a runnable local artifact without implying store submission or release authority?

Default owner: `export` — Export/package owner: local package descriptors, provenance, checksums, and runnable smoke evidence.

Example finding: A bundle descriptor exists but no local smoke proves it runs.

### `dogfood_game_quality` — Dogfood Game Quality

Does a real dogfood game demonstrate coherent play, pacing, feedback, and human-owned fun/release judgment?

Default owner: `product` — Dogfood/product owner: real-game slice quality, human fun/taste verdicts, and release-go/no-go boundaries.

Example finding: A dogfood slice is mechanically passable but confusing, unfun, or lacks human verdict evidence.

## Finding record contract

Every downstream gap ledger, backlog item, classifier output, and dogfood production-log finding must include:

- `id`
- `category`
- `severity`
- `owner`
- `evidenceRefs`
- `observedBehavior`
- `expectedBehavior`
- `productImpact`
- `recommendedBacklogAction`

## Classification rules

- Use exactly one category id from categoryEnum and one severity id from severityEnum per primary finding.
- Use blocking when the claimed product flow cannot complete or required evidence is absent for product-observed closure.
- Use major when the flow completes only with misleading gaps, diagnostics failures, or substantial workaround.
- Use polish only when product-observed completion remains honest and the issue is usability/presentation refinement.
- Record gaps as backlog items rather than changing failed product observations to green.

## Sample application

The collect-and-exit sample application lives at
[`docs/product-gap-taxonomy-sample-collect-and-exit.md`](product-gap-taxonomy-sample-collect-and-exit.md)
with machine-readable findings at
[`docs/product-gap-taxonomy-sample-collect-and-exit.json`](product-gap-taxonomy-sample-collect-and-exit.json).
