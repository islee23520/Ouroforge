const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const matrixPath = path.join(root, 'scenario-coverage-v19/matrix.json');
const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));

function readJson(ref) {
  return JSON.parse(fs.readFileSync(path.join(root, 'scenario-coverage-v19', ref), 'utf8'));
}

assert.equal(matrix.schemaVersion, 'scenario-coverage-v19-evaluator-depth-v1');
assert.equal(matrix.issue, 1288);
assert.equal(matrix.fixtureScoped, true);
assert.match(matrix.boundary, /Deterministic fixture coverage only/);
assert.match(matrix.boundary, /no subjective quality assertions/);
assert.match(matrix.boundary, /no .*auto-fix.*auto-apply.*auto-merge/);

const visualStates = new Set(matrix.visualGateStates.map((entry) => entry.state));
for (const state of ['pass', 'fail', 'missing-baseline', 'missing-screenshot', 'threshold-not-declared', 'stale-ref']) {
  assert.ok(visualStates.has(state), `visual coverage includes ${state}`);
}
assert.equal(matrix.visualGateStates.filter((entry) => entry.caseId === 'under-threshold')[0].state, 'pass');

for (const entry of matrix.visualGateStates) {
  const fixture = readJson(entry.fixtureRef);
  assert.equal(fixture.schemaVersion, 'visual-comparison-evidence-v1');
  assert.equal(typeof fixture.scenarioId, 'string');
  assert.equal(typeof fixture.checkpointId, 'string');
  if (entry.expect.outcome) assert.equal(fixture.outcome, entry.expect.outcome);
  if (Object.prototype.hasOwnProperty.call(entry.expect, 'changedPixels')) {
    assert.equal(fixture.pixelDiffSummary.changedPixels, entry.expect.changedPixels);
  }
  if (entry.expect.thresholdDeclared === true) assert.ok(fixture.thresholds.length > 0);
  if (entry.expect.thresholdDeclared === false) assert.equal(fixture.thresholds.length, 0);
  if (entry.expect.missingSide === 'before') assert.equal(typeof fixture.before.missingReason, 'string');
  if (entry.expect.missingSide === 'after') assert.equal(typeof fixture.after.missingReason, 'string');
  if (entry.expect.staleRef) assert.equal(fixture.comparisonId, 'visual_gate_stale_ref');
  assert.doesNotMatch(JSON.stringify(fixture), /aesthetic score|fun score|production-ready|auto-apply enabled|auto-merge enabled/i);
}

const semanticStates = new Set(matrix.semanticGateStates.map((entry) => entry.state));
for (const state of ['pass', 'fail', 'unsupported', 'missing-target-state', 'malformed-invariant', 'unsafe-expression', 'stale-ref']) {
  assert.ok(semanticStates.has(state), `semantic coverage includes ${state}`);
}

for (const entry of matrix.semanticGateStates) {
  const fixture = readJson(entry.fixtureRef);
  assert.equal(fixture.schemaVersion, 'runtime-invariant-model-v1');
  assert.equal(typeof fixture.scenarioId, 'string');
  const invariant = fixture.invariants[0] || {};
  if (entry.expect.invariantType) assert.equal(invariant.invariantType, entry.expect.invariantType);
  if (entry.expect.targetPath) assert.equal(invariant.targetPath, entry.expect.targetPath);
  if (entry.expect.unsupportedType) assert.equal(invariant.invariantType, 'no_impossible_state');
  if (entry.expect.malformed) assert.equal(invariant.invariantType, 'player_in_bounds');
  if (entry.expect.unsafeExpression) assert.ok(Object.prototype.hasOwnProperty.call(invariant, 'expression'));
  if (entry.expect.staleRunId) assert.notEqual(fixture.runId, '__RUN_ID__');
  assert.doesNotMatch(JSON.stringify(fixture), /new Function|browser trusted write|auto-apply enabled|auto-merge enabled/i);
}

const legacyGolden = readJson(matrix.legacyGoldenRef);
assert.deepEqual(legacyGolden, {
  status: 'passed',
  summary: '1 scenario result(s) passed with consistent evidence.',
  failures: [],
  evidence_refs: ['evidence/scenarios/collect-and-exit/scenario-result.json'],
  metadata: {
    evaluator: 'ouroforge-evaluator-v0',
    scenario_results: 1,
    suite_summaries: 0,
    behavior_evaluator_results: 0,
    visual_gate_results: 0,
    semantic_gate_results: 0,
  },
});
assert.equal(Object.prototype.hasOwnProperty.call(legacyGolden, 'visual'), false);
assert.equal(Object.prototype.hasOwnProperty.call(legacyGolden, 'semantic'), false);
assert.equal(Object.prototype.hasOwnProperty.call(legacyGolden, 'gateCategories'), false);

assert.deepEqual(matrix.fourCategoryVerdictShape.requiredCategories, ['mechanical', 'runtime', 'visual', 'semantic']);
assert.deepEqual(matrix.fourCategoryVerdictShape.aggregation, { operator: 'declared-gate-and', undeclaredGatePolicy: 'neutral' });

for (const demoRun of ['demo/visual-fail-run/verdict.json', 'demo/semantic-fail-run/verdict.json']) {
  const verdict = JSON.parse(fs.readFileSync(path.join(root, demoRun), 'utf8'));
  assert.equal(typeof verdict.gateCategories, 'object', `${demoRun} exposes gateCategories`);
  for (const category of matrix.fourCategoryVerdictShape.requiredCategories) {
    assert.equal(typeof verdict.gateCategories[category].declared, 'boolean', `${demoRun} ${category}.declared`);
    assert.match(verdict.gateCategories[category].status, /^(pass|fail)$/, `${demoRun} ${category}.status`);
    assert.equal(Number.isInteger(verdict.gateCategories[category].resultCount), true, `${demoRun} ${category}.resultCount`);
    assert.equal(Number.isInteger(verdict.gateCategories[category].failureCount), true, `${demoRun} ${category}.failureCount`);
  }
  assert.deepEqual(verdict.gateCategories.aggregation, matrix.fourCategoryVerdictShape.aggregation);
}

const docs = fs.readFileSync(path.join(root, '../../docs/scenario-coverage-v19.md'), 'utf8');
assert.match(docs, /Scenario Coverage v19/);
assert.match(docs, /visual(?: and semantic)? gate states/i);
assert.match(docs, /semantic gate states/i);
assert.match(docs, /legacy two-gate verdict/);
assert.match(docs, /#1 and #23 remain open/);
assert.doesNotMatch(docs, /subjective quality score enabled|production-ready claim enabled|current Godot replacement is implemented|auto-apply enabled|auto-merge enabled/i);

console.log('scenario coverage v19 evaluator depth smoke passed');
