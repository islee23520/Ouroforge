const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const matrixPath = path.join(fixtureDir, 'coverage-matrix.fixture.json');
const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));

const requiredAreas = [
  'intent_validation',
  'generation_plan_validation',
  'layout_constraints',
  'tilemap_terrain_draft',
  'entity_objective_encounter_placement',
  'reachability_pathing',
  'objective_completion_proof',
  'difficulty_pacing_heuristic',
  'visual_semantic_diff',
  'agent_generated_level_draft',
  'review_gated_level_apply',
  'studio_dashboard_read_model',
];

assert.equal(matrix.schemaVersion, 'agentic-level-design-regression-suite-v1');
assert.equal(matrix.issue, 641);
assert.equal(matrix.status, 'ready');
assert.deepEqual(matrix.rows.map((row) => row.area), requiredAreas);

for (const row of matrix.rows) {
  assert.ok(row.validFixture, `${row.area} needs a valid fixture`);
  assert.ok(fs.existsSync(path.join(repoRoot, row.validFixture)), `${row.area} valid fixture exists`);
  assert.ok(Array.isArray(row.edgeFixtures) && row.edgeFixtures.length >= 1, `${row.area} needs edge/stale/missing coverage`);
  assert.ok(Array.isArray(row.malformedFixtures) && row.malformedFixtures.length >= 1, `${row.area} needs malformed coverage`);
  for (const fixture of [...row.edgeFixtures, ...row.malformedFixtures]) {
    assert.ok(fs.existsSync(path.join(repoRoot, fixture)), `${row.area} referenced fixture exists: ${fixture}`);
  }
  assert.match(row.testFilter, /cargo test|node /, `${row.area} has runnable test filter`);
  assert.doesNotMatch(row.expectedBoundary, /production-ready|Godot replacement|autonomous full game/i);
}

for (const phrase of [
  'no autonomous full game generation',
  'no browser trusted writes',
  'no command bridge',
  'no auto-apply',
  'no auto-merge',
  'no self-approval',
  'no production editor',
  'generated outputs remain untracked',
]) {
  assert.ok(matrix.guardrails.includes(phrase), `guardrail present: ${phrase}`);
}

for (const root of matrix.generatedStatePolicy.untrackedRoots) {
  assert.equal(fs.existsSync(path.join(repoRoot, root)), false, `${root} must remain generated/untracked`);
}

const coverageDoc = fs.readFileSync(path.join(repoRoot, 'docs', 'scenario-coverage-v10-agentic-level-design.md'), 'utf8');
assert.match(coverageDoc, /Scenario Coverage v10/);
assert.match(coverageDoc, /malformed, missing, stale, unsupported,\s+and blocked/);
assert.match(coverageDoc, /no autonomous full game generation/);
assert.match(coverageDoc, /no browser trusted writes/);
assert.match(coverageDoc, /#1 and #23 remain open/);

console.log('agentic level design regression coverage smoke passed');
