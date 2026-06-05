# Gameplay Logic Demo Evidence Flow

Fixture-scoped expected evidence for #623 GL10.13.2.

The files in this directory model the deterministic run/evidence flow for the
structured gameplay logic demo without granting browser or arbitrary-code
authority. Local generated runs, dashboard exports, screenshots, browser
profiles, and temp evidence stay ignored by `.gitignore`; these JSON files are
tracked only as fixture-scoped expected evidence for tests and review.

Flow:

1. `behaviors/gameplay-logic-demo.behavior.json` describes key, door, dash,
   patrol/hazard, and win-condition behavior.
2. `runtime-events.fixture.json` records the expected event sequence.
3. `scenario-outcome.fixture.json` records the expected scenario outcomes.
4. `behavior-evidence-bundle.fixture.json` links behavior definitions, runtime
   events, scenario outcomes, draft/review/apply metadata, rollback metadata,
   and rerun comparison evidence for journal/dashboard inspection.

Boundary: this is read-only structured behavior evidence. It does not add eval,
dynamic import, plugin loading, command bridges, browser trusted writes,
auto-apply, hosted/cloud behavior, native export, production-stable scripting
API claims, production engine claims, or Godot replacement claims.
