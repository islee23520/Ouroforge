# Gameplay Logic Demo Docs and Read-Model Compatibility

Scope for #623 GL10.13.3: document the commands, expected evidence, known gaps,
cleanup policy, and dashboard/Studio read-model compatibility for the structured
Gameplay Logic Demo v1 fixture.

## Commands

Run from the repository root:

```bash
node examples/gameplay-logic-demo-v1/schema-smoke.test.cjs
node examples/gameplay-logic-demo-v1/evidence-flow-smoke.test.cjs
node examples/gameplay-logic-demo-v1/read-model-compatibility-smoke.test.cjs
```

Broader release checks used for PR evidence:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

## Expected evidence

- `behaviors/gameplay-logic-demo.behavior.json` is the structured behavior
  definition for key pickup, door flag/state logic, dash ability, patrol/hazard,
  and win-condition outcomes.
- `scenarios/gameplay-logic-demo.scenario-pack.json` and
  `scenarios/gameplay-logic-demo.behavior-assertions.json` describe scenario
  coverage and behavior assertion IDs.
- `evidence/runtime-events.fixture.json` records expected event types.
- `evidence/scenario-outcome.fixture.json` records passed scenario outcomes.
- `evidence/behavior-evidence-bundle.fixture.json` links behavior definitions,
  runtime event evidence, scenario outcome evidence, draft/review/apply metadata,
  rollback metadata, and rerun comparison evidence for read-only inspection.
- `journal/behavior-evidence-journal.fragment.md` records the expected journal
  fragment shape for behavior evidence lifecycle display.

## Dashboard compatibility

The static dashboard consumes a `behavior_evidence` read model with:

- `present`, `status`, `bundle_count`, `malformed_count`, lifecycle counts, and
  conservative `boundary` text.
- `bundles[]` entries containing `bundle_id`, `status`, `path`,
  `lifecycle_ref_count`, observed failures, next-step hypotheses, linked
  evidence refs, and guardrails.

The compatibility smoke renders the demo bundle through
`renderBehaviorEvidenceLifecycle` and verifies that the panel remains escaped,
read-only, and free of command/apply/browser-authority controls.

## Studio compatibility

The authoring cockpit consumes two read-only surfaces:

- `behavior_evidence` for the behavior evidence lifecycle panel.
- `behavior_inspection` for behavior list, event/signal, state-machine,
  ability/action, and review/apply status panels.

The compatibility smoke loads the demo inspection descriptor refs and renders
those Studio panels with the existing Milestone 10 behavior model, event/signal,
state-machine, ability/action, draft, review, and apply fixtures.

## Cleanup policy

Generated local output must remain untracked:

- `runs/`
- `dashboard-data/`
- `screenshots/`
- `browser-profiles/`
- `tmp-evidence/`

Fixture-scoped JSON under `evidence/` is tracked only as deterministic expected
read-model input. Local run output, dashboard exports, screenshots, browser
profiles, and temp evidence should be deleted or left ignored after review.

## Known gaps

- This demo does not add arbitrary executable scripting, eval, dynamic import,
  plugin loading, command bridges, browser trusted writes, source mutation,
  auto-apply, auto-merge, self-approval, hosted/cloud behavior, native export,
  production-stable scripting APIs, production engine maturity, or Godot
  replacement claims.
- Dashboard and Studio surfaces are read-only display compatibility checks; Rust
  remains the trusted owner for validation, behavior draft/apply validation,
  generated evidence contracts, and persistence.
- GL10.13.3 does not broaden compatibility beyond the fixture-scoped Milestone 10
  structured behavior/read-model contracts listed above.
