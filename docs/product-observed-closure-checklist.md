# Product-observed closure checklist

This is the canonical artifact-level checklist for future Ouroforge
`product-observed complete` claims. It complements the completion semantics in
[`docs/product-observed-completion.md`](product-observed-completion.md): that doc
owns the terms and closure template; this document owns the per-artifact pass/fail
criteria.

A practical runtime, Studio, gameplay, asset-workflow, export, or agentic-loop
claim must reference each stable item id below. Docs-only, VM-only, static
fixture-only, schema-only, and read-model-only evidence fails product-observed
closure even when it remains valid `contract-complete` evidence.

## Checklist items

<a id="po-check-live-url"></a>
### `po-check-live-url` — live product entry point

Pass requires a concrete live URL, local product command, or editor/workspace
entry point that a reviewer can exercise. The entry point must identify the
build/project/version and must not be merely a static fixture path.

Fail examples:
- only linking a README, schema, fixture JSON, or unit test;
- citing a generated path that was deleted before review;
- omitting the exact URL/command used.

<a id="po-check-console"></a>
### `po-check-console` — console and runtime diagnostics

Pass requires captured console/runtime diagnostics for the exercised product
surface, including an explicit clean/error summary. Expected warnings may pass
only when they are enumerated and justified.

Fail examples:
- no console capture;
- only Rust unit-test output for a browser/editor claim;
- ignoring runtime errors because the fixture tests passed.

<a id="po-check-screenshot"></a>
### `po-check-screenshot` — screenshots or equivalent visual captures

Pass requires screenshots, video frames, or equivalent visual captures that show
the claimed runtime/editor/gameplay state. Non-visual claims may mark this N/A
only with an explanation and substitute structured product-surface evidence.

Fail examples:
- no capture for a visual runtime or Studio claim;
- stale screenshot from an unrelated run;
- screenshot stored only in ignored local state with no reproducible reference.

<a id="po-check-replay"></a>
### `po-check-replay` — input replay or interaction transcript

Pass requires a replay file, scripted input sequence, or human interaction
transcript sufficient to reproduce the observed state.

Fail examples:
- a scenario fixture exists but was not replayed in the product surface;
- only manual narrative without timestamps/actions;
- a replay exists but does not match the observed run/build.

<a id="po-check-world-sample"></a>
### `po-check-world-sample` — world-state samples

Pass requires sampled state from the exercised product surface: player/object
position, inventory, scene id, score/HUD values, editor selection/draft state, or
other claim-specific state.

Fail examples:
- source fixture state only;
- no after-state captured;
- state sample cannot be tied to the live entry point and replay.

<a id="po-check-event-sample"></a>
### `po-check-event-sample` — event and assertion samples

Pass requires relevant events, scenario assertions, editor actions, or agentic
loop decisions emitted by the exercised product surface, with pass/fail status.

Fail examples:
- static scenario pack without observed event output;
- event output from a different fixture/run;
- no assertion status for a claimed gameplay or loop outcome.

<a id="po-check-frame-stats"></a>
### `po-check-frame-stats` — frame/performance stats where relevant

Pass requires frame timing, budget, update count, render stats, or equivalent
product-surface performance diagnostics when the claim involves runtime,
rendering, editor responsiveness, playback, or export smoke execution.

Fail examples:
- no frame/runtime stats for a runtime or rendering claim;
- only a unit-test duration;
- performance values reported without thresholds or verdict.

<a id="po-check-before-after"></a>
### `po-check-before-after` — before/after comparison where relevant

Pass requires before/after evidence when the claim changes state, assets,
scenes, generated proposals, source-like drafts, exports, or agentic loop output.
The comparison must identify what changed and whether the result passed.

Fail examples:
- only after-state for an apply/rerun claim;
- compare output exists but is unlinked;
- no rollback/backlog note for a failed after-state.

<a id="po-check-verdict"></a>
### `po-check-verdict` — human-readable verdict

Pass requires a concise verdict that separates mechanical pass/fail from product
usability, fun/taste, release, market, or quality judgment. The verdict must
state `product-observed complete`, `product-observed FAIL`, or why the checklist
is N/A for a contract-only issue.

Fail examples:
- saying `complete` without classification;
- treating fixture green as practical usability;
- hiding a failed product observation behind passing tests.

<a id="po-check-generated-state"></a>
### `po-check-generated-state` — generated-state audit

Pass requires a generated-state audit showing that runs, screenshots, browser
profiles, dashboard exports, packages, temp projects, and observability outputs
remain ignored/generated unless explicitly fixture-scoped.

Fail examples:
- committed run/browser/screenshot output without fixture authorization;
- no `git status --short --ignored` or equivalent audit when product evidence was
  generated;
- generated evidence cannot be found, reproduced, or tied to the closure verdict.

## Canonical FAIL example: current collect-and-exit page

The current collect-and-exit fixture remains useful contract evidence but is not
yet product-observed practical engine usability. The README documents source
fixtures and smoke commands, and it explicitly says generated runs, dashboard
data, screenshots, previews, and temporary smoke outputs are not committed.
That is correct for contract evidence, but it means the product-observed artifact
bundle is absent.

Reference: `examples/playable-demo-v2/collect-and-exit/README.md`.

| Stable id | Current evidence | Result | Gap to pass |
| --- | --- | --- | --- |
| `po-check-live-url` | README and source fixture paths; no committed live URL or reproducible product entry-point artifact for M115 review. | FAIL | Provide exact live URL or local product command with build/project identity. |
| `po-check-console` | Smoke commands are listed, but no current console/runtime diagnostic capture is committed for this checklist application. | FAIL | Attach console/runtime diagnostics and clean/error summary from the exercised product surface. |
| `po-check-screenshot` | README says screenshots and temporary smoke outputs remain untracked. | FAIL | Attach screenshots/video frames or equivalent captures tied to the reviewed run. |
| `po-check-replay` | Scenario/smoke fixtures exist, but no product-surface replay artifact is recorded in this checklist application. | FAIL | Record replay/scripted inputs or timestamped interaction transcript. |
| `po-check-world-sample` | Source fixture state and smoke descriptions exist; no current live world-state sample is attached. | FAIL | Capture player/object/HUD/scene state from the exercised product surface. |
| `po-check-event-sample` | Scenario pack and smoke descriptions exist; no current event/assertion sample output is attached here. | FAIL | Attach event/assertion output from the product-surface run. |
| `po-check-frame-stats` | Frame-budget defaults are described; no current frame/performance stats artifact is attached. | FAIL | Capture frame/update/render stats with thresholds and verdict. |
| `po-check-before-after` | Prior generated before/after ids are historical ignored smoke evidence, not a current linked product-observed comparison. | FAIL | Attach before/after comparison for any claimed changed state or mark N/A with reason. |
| `po-check-verdict` | README is contract-fixture documentation and does not provide an M115 product-observed verdict. | FAIL | Record `product-observed complete` or `product-observed FAIL` with itemized rationale. |
| `po-check-generated-state` | README states generated output remains untracked; no current audit output is attached to this checklist application. | FAIL | Include generated-state audit output tied to the evidence bundle. |

Canonical result: `product-observed FAIL`; preserve the fixture as
`contract-complete` evidence only.

## Passing pattern for collect-and-exit

The same collect-and-exit page could pass a future product-observed claim only if
a later issue records an evidence bundle with:

- exact live URL or local product command and build/project identity
  (`po-check-live-url`);
- clean console/runtime diagnostics (`po-check-console`);
- screenshots or equivalent captures showing start, key pickup, and exit state
  (`po-check-screenshot`);
- replay or interaction transcript that reproduces the run (`po-check-replay`);
- world-state and event/assertion samples for key pickup and exit completion
  (`po-check-world-sample`, `po-check-event-sample`);
- frame/performance stats with scoped thresholds (`po-check-frame-stats`);
- before/after comparison when a change is claimed (`po-check-before-after`);
- explicit verdict separating mechanical pass/fail from fun/release judgment
  (`po-check-verdict`);
- generated-state audit showing no unscoped generated artifacts entered trusted
  source (`po-check-generated-state`).
