# Product gap taxonomy sample: collect-and-exit

Machine-readable sample: `docs/product-gap-taxonomy-sample-collect-and-exit.json`.

Taxonomy source: `docs/product-gap-taxonomy.json`.

Sample classification: `product-observed fail`.

This sample applies #2350 categories and severities to the current collect-and-exit evidence bundle referenced by `examples/playable-demo-v2/collect-and-exit/README.md` and the M115 checklist. It is not a live browser dogfood run.

## Findings

### `ce-gap-001`

- Category: `runtime_ux`
- Severity: `blocking`
- Owner: `runtime`
- Evidence refs:
  - `examples/playable-demo-v2/collect-and-exit/README.md`
  - `docs/product-observed-closure-checklist.md`
- Observed behavior: The fixture README lists validation and smoke commands, but the M115 checklist application has no live URL or reproducible product entry-point artifact.
- Expected behavior: A product-observed runtime claim provides an exact live URL or local product command with build/project identity.
- Product impact: A reviewer cannot independently observe practical runtime usability from the committed bundle.
- Recommended backlog action: Create a live dogfood runtime harness that records URL/command identity, diagnostics, replay, world/event samples, frame stats, verdict, and generated-state audit.

### `ce-gap-002`

- Category: `renderer_quality`
- Severity: `major`
- Owner: `runtime`
- Evidence refs:
  - `examples/playable-demo-v2/collect-and-exit/README.md`
  - `docs/product-observed-closure-checklist.md#po-check-screenshot`
- Observed behavior: The README explicitly keeps screenshots and temporary smoke outputs untracked; no current visual capture is attached to the checklist application.
- Expected behavior: Runtime visual claims include screenshots or equivalent captures tied to the reviewed run.
- Product impact: Rendering quality and readability cannot be assessed from source fixtures alone.
- Recommended backlog action: Capture start, key pickup, and exit-state screenshots or equivalent frames for the product-observed run.

### `ce-gap-003`

- Category: `input_control`
- Severity: `major`
- Owner: `runtime`
- Evidence refs:
  - `examples/playable-demo-v2/collect-and-exit/README.md`
  - `docs/product-observed-closure-checklist.md#po-check-replay`
- Observed behavior: Scenario/smoke fixtures exist, but no product-surface replay artifact or timestamped interaction transcript is attached to the checklist application.
- Expected behavior: Gameplay claims provide replay/scripted inputs that reproduce the observed state.
- Product impact: Input/control usability cannot be distinguished from a fixture-only smoke path.
- Recommended backlog action: Record a replay or interaction transcript for collect-and-exit and tie it to world/event samples.

### `ce-gap-004`

- Category: `qa_evaluator_depth`
- Severity: `major`
- Owner: `qa`
- Evidence refs:
  - `docs/scenario-coverage-v96-product-observed-rebaseline.md`
  - `docs/roadmap/milestone-classification-ledger.json`
- Observed behavior: Historical smoke and ledger evidence classify the page as contract-complete, while product-observed checklist items fail.
- Expected behavior: QA/verdict output must keep fixture-green and product-observed-fail states separate.
- Product impact: Without this split, future closures could overclaim practical usability from green fixture tests.
- Recommended backlog action: Have #2351/#2381 consume this taxonomy and emit product-observed failure categories directly in gap/verdict outputs.

### `ce-gap-005`

- Category: `dogfood_game_quality`
- Severity: `blocking`
- Owner: `product`
- Evidence refs:
  - `docs/product-observed-closure-checklist.md#po-check-verdict`
  - `examples/playable-demo-v2/collect-and-exit/README.md`
- Observed behavior: No current human-readable dogfood verdict separates mechanical pass/fail from fun, clarity, pacing, or release judgment for the page.
- Expected behavior: A dogfood usability claim includes an explicit product-observed verdict and human-owned quality/fun boundaries.
- Product impact: The fixture cannot be used as evidence that Ouroforge is practically usable for real game production.
- Recommended backlog action: Record a dogfood verdict with human quality/taste/release boundaries once live evidence exists.

## Boundary and overlap review

Each finding selects the category that best identifies the user-facing failure mode; adjacent categories may be mentioned in evidence but not duplicated as separate primary categories unless a distinct backlog action exists.

- runtime_ux owns missing launch/entry-point evidence; renderer_quality owns missing visual proof; input_control owns missing replay/interaction proof.
- qa_evaluator_depth owns the meta-failure where fixture-green could be mistaken for product-observed pass.
- dogfood_game_quality owns the absence of a real-game human verdict, not low-level runtime mechanics.
