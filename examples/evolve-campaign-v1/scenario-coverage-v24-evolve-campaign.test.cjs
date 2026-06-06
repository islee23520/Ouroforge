const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const matrixRoot = path.join(root, 'scenario-coverage-v24');

function readJson(ref) {
  return JSON.parse(fs.readFileSync(path.join(matrixRoot, ref), 'utf8'));
}

function assertNoForbiddenClaims(value, label) {
  assert.doesNotMatch(
    JSON.stringify(value),
    /subjective quality score enabled|fun score enabled|aesthetic score enabled|production-ready claim enabled|current Godot replacement is implemented|auto-fix enabled|auto-apply enabled|auto-merge enabled|browser-owned trusted persistence enabled|reviewer bypass enabled/i,
    `${label} keeps conservative wording`,
  );
}

const FOUR_GATES = ['mechanical', 'runtime', 'visual', 'semantic'];
const STOP_REASONS = new Set(['acceptance-reached', 'budget-exhausted', 'no-progress']);

const matrix = readJson('matrix.json');

assert.equal(matrix.schemaVersion, 'scenario-coverage-v24-evolve-campaign-v1');
assert.equal(matrix.issue, 1491);
assert.equal(matrix.fixtureScoped, true);
assert.match(matrix.boundary, /Deterministic fixture coverage only/);
assert.match(matrix.boundary, /no subjective quality assertions/);
assert.match(matrix.boundary, /no auto-fix, no auto-apply, no auto-merge/);
assert.deepEqual(matrix.requiredFourGates, FOUR_GATES);
assertNoForbiddenClaims(matrix, 'matrix');

// Enumerated termination coverage must include every termination mode.
const terminationReasons = new Set(matrix.terminationCases.map((entry) => entry.expectStopReason));
for (const reason of ['acceptance-reached', 'budget-exhausted', 'no-progress']) {
  assert.ok(terminationReasons.has(reason), `termination coverage includes ${reason}`);
}

// Enumerated convergence outcomes must include converged and not-converged.
const outcomes = new Set(matrix.terminationCases.map((entry) => entry.expectOutcome));
assert.ok(outcomes.has('converged'), 'coverage includes a converged outcome');
assert.ok(outcomes.has('not-converged'), 'coverage includes a not-converged outcome');

// Backward-compat: single-shot evolve must remain a valid campaign.
assert.ok(matrix.backwardCompatCases.length >= 1, 'backward-compat coverage present');
const singleShot = matrix.backwardCompatCases.find(
  (entry) => entry.caseId === 'v24.backward-compat-single-shot-evolve',
);
assert.ok(singleShot, 'single-shot evolve backward-compat case present');

function assertGateStatus(status, label) {
  assert.match(status, /^(pass|fail|unsupported)$/, `${label} gate status is a known state`);
}

function assertCampaignShape(entry, fixture) {
  assert.equal(fixture.schemaVersion, 'evolve-campaign-v1', `${entry.caseId} schema`);
  assert.equal(typeof fixture.campaignId, 'string');
  assert.equal(typeof fixture.seedRef, 'string');
  assert.deepEqual(fixture.acceptanceTarget, FOUR_GATES, `${entry.caseId} acceptance target`);
  assert.ok(fixture.iterations.length >= 1, `${entry.caseId} has a baseline iteration`);
  assert.ok(
    fixture.iterations.length <= fixture.budget.maxIterations,
    `${entry.caseId} iterations within iteration budget`,
  );
  let totalCost = 0;
  fixture.iterations.forEach((iteration, position) => {
    assert.equal(iteration.index, position, `${entry.caseId} iteration ${position} is gap-free`);
    assert.equal(typeof iteration.hypothesis, 'string');
    assert.equal(typeof iteration.mutationRef, 'string');
    assert.match(iteration.decision, /^(manual-review|auto-apply)$/, `${entry.caseId} decision`);
    const gates = iteration.fourGate.map((verdict) => verdict.gate).sort();
    assert.deepEqual(gates, [...FOUR_GATES].sort(), `${entry.caseId} records all four gates`);
    iteration.fourGate.forEach((verdict) => assertGateStatus(verdict.status, entry.caseId));
    assert.ok(iteration.evidenceRefs.length >= 1, `${entry.caseId} iteration links evidence`);
    totalCost += iteration.costUnits;
  });
  assert.ok(totalCost <= fixture.budget.maxCostUnits, `${entry.caseId} cost within budget`);
  assert.ok(STOP_REASONS.has(fixture.termination.reason), `${entry.caseId} stop reason known`);
  assert.equal(
    fixture.termination.reason,
    entry.expectStopReason,
    `${entry.caseId} stop reason matches matrix`,
  );
  if (fixture.termination.reason === 'acceptance-reached') {
    assert.equal(typeof fixture.termination.acceptedIteration, 'number');
    assert.equal(fixture.termination.diagnosis, undefined, `${entry.caseId} converged has no diagnosis`);
  } else {
    assert.equal(typeof fixture.termination.diagnosis, 'string', `${entry.caseId} carries diagnosis`);
  }
  assertNoForbiddenClaims(fixture, entry.caseId);
}

for (const entry of [...matrix.terminationCases, ...matrix.backwardCompatCases]) {
  assertCampaignShape(entry, readJson(entry.fixture));
}

console.log('scenario-coverage-v24-evolve-campaign: all cases asserted');
