const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const matrixRoot = path.join(root, 'scenario-coverage-v20');
const matrix = readJson('matrix.json');

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

assert.equal(matrix.schemaVersion, 'scenario-coverage-v20-evolve-depth-v1');
assert.equal(matrix.issue, 1297);
assert.equal(matrix.fixtureScoped, true);
assert.match(matrix.boundary, /Deterministic fixture coverage only/);
assert.match(matrix.boundary, /no subjective quality assertions/);
assert.match(matrix.boundary, /no auto-fix.*no auto-apply.*no auto-merge/);
assert.deepEqual(matrix.requiredFourGates, ['mechanical', 'runtime', 'visual', 'semantic']);
assertNoForbiddenClaims(matrix, 'matrix');

const proposalCases = matrix.proposalStateRefs.map(readJson);
const proposalCaseIds = new Set(proposalCases.map((entry) => entry.caseId));
for (const id of [
  'visual-linked-high-confidence-scene',
  'semantic-linked-high-confidence-data',
  'runtime-linked-high-confidence-data',
  'mechanical-linked-high-confidence-scenario',
  'visual-missing-evidence-no-proposal',
  'semantic-stale-ref-no-proposal',
  'bounded-type-enforcement-rejects-source-apply-for-visual',
]) {
  assert.ok(proposalCaseIds.has(id), `proposal coverage includes ${id}`);
}

for (const entry of proposalCases) {
  assert.equal(entry.schemaVersion, 'evolve-depth-proposal-regression-v20');
  assert.match(entry.failure.kind, /^(scenario_failed|behavior_assertion_failed|visual_gate_failed|semantic_gate_failed)$/);
  assert.ok(matrix.requiredFourGates.includes(entry.rationale.failing_gate_category));
  assert.match(entry.rationale.evidence_state, /^(linked|missing|stale)$/);
  assert.match(entry.rationale.confidence, /^(high|none)$/);
  assert.match(entry.rationale.allowed_mutation_type, /^(scene_only|data_only|scenario_only)$/);
  assert.equal(typeof entry.rationale.justifying_evidence_ref, 'string');
  assert.equal(Array.isArray(entry.rationale.evidence_artifact_ids), true);
  if (entry.expect.proposalCreated) {
    assert.equal(entry.rationale.evidence_state, 'linked', `${entry.caseId} proposal requires linked evidence`);
    assert.notEqual(entry.rationale.confidence, 'none', `${entry.caseId} proposal has evidence-derived confidence`);
    assert.equal(entry.rationale.backlog_only, false, `${entry.caseId} is not backlog-only`);
  } else {
    assert.equal(entry.rationale.backlog_only, true, `${entry.caseId} is backlog-only`);
  }
  if (entry.caseId.includes('visual')) {
    assert.equal(entry.rationale.failing_gate_category, 'visual');
  }
  if (entry.caseId.includes('semantic')) {
    assert.equal(entry.rationale.failing_gate_category, 'semantic');
  }
  assertNoForbiddenClaims(entry, entry.caseId);
}

const mapping = readJson(matrix.classMappingRef);
assert.equal(mapping.schemaVersion, 'evolve-depth-class-mapping-v20');
const byClass = Object.fromEntries(mapping.mappings.map((entry) => [entry.failureClass, entry]));
assert.deepEqual(byClass.scenario_failed, { failureClass: 'scenario_failed', gate: 'mechanical', boundedMutationType: 'scenario', allowedMutationType: 'scenario_only', backlogOnly: false });
assert.deepEqual(byClass.behavior_assertion_failed, { failureClass: 'behavior_assertion_failed', gate: 'runtime', boundedMutationType: 'data', allowedMutationType: 'data_only', backlogOnly: false });
assert.deepEqual(byClass.visual_gate_failed, { failureClass: 'visual_gate_failed', gate: 'visual', boundedMutationType: 'scene', allowedMutationType: 'scene_only', backlogOnly: false });
assert.deepEqual(byClass.semantic_gate_failed, { failureClass: 'semantic_gate_failed', gate: 'semantic', boundedMutationType: 'data', allowedMutationType: 'data_only', backlogOnly: false });
for (const cls of ['unsupported_gate_failure', 'unknown_failure', 'flaky_evidence']) {
  assert.equal(byClass[cls].backlogOnly, true, `${cls} stays backlog-only`);
  assert.equal(byClass[cls].boundedMutationType, 'none', `${cls} cannot fabricate bounded mutation type`);
}
assertNoForbiddenClaims(mapping, 'mapping');

const deltaCases = matrix.rerunDeltaRefs.map(readJson);
const deltaIds = new Set(deltaCases.map((entry) => entry.caseId));
for (const id of ['visual-fail-to-pass', 'semantic-fail-to-pass', 'visual-pass-to-fail-regression', 'non-comparable-different-scenario', 'non-comparable-missing-after-verdict']) {
  assert.ok(deltaIds.has(id), `rerun delta coverage includes ${id}`);
}
for (const entry of deltaCases) {
  assert.equal(entry.schemaVersion, 'evolve-depth-rerun-delta-regression-v20');
  assert.match(entry.classification, /^(improved|regressed|not_comparable)$/);
  assert.match(entry.comparability.state, /^(comparable|non-comparable)$/);
  if (entry.comparability.state === 'comparable') {
    const gates = Object.fromEntries(entry.fourGateDeltas.map((delta) => [delta.gate, delta]));
    assert.deepEqual(Object.keys(gates).sort(), matrix.requiredFourGates.slice().sort(), `${entry.caseId} records all gates`);
    for (const gate of matrix.requiredFourGates) {
      assert.match(gates[gate].before_status, /^(pass|fail)$/);
      assert.match(gates[gate].after_status, /^(pass|fail)$/);
      assert.match(gates[gate].transition, /^(unchanged_pass|fail_to_pass|pass_to_fail)$/);
    }
  } else {
    assert.equal(entry.fourGateDeltas.length, 0, `${entry.caseId} does not invent deltas`);
    assert.equal(typeof entry.comparability.reason, 'string');
  }
  assertNoForbiddenClaims(entry, entry.caseId);
}

const legacy = readJson(matrix.legacyGoldenRef);
assert.equal(legacy.schemaVersion, 'evolve-v0-golden');
assert.equal(legacy.proposalArtifact.status, 'proposed');
assert.equal(legacy.proposalArtifact.proposal.mutation_type, 'scenario');
assert.equal(Object.prototype.hasOwnProperty.call(legacy.proposalArtifact.proposal, 'rationale'), false);
assert.equal(Object.prototype.hasOwnProperty.call(legacy.comparisonArtifact, 'fourGateDeltas'), false);
assert.equal(Object.prototype.hasOwnProperty.call(legacy.comparisonArtifact, 'gateCategories'), false);
assert.deepEqual(legacy.absentFields, ['rationale.failing_gate_category', 'rationale.bounded_mutation_type', 'fourGateDeltas', 'gateCategories']);
assertNoForbiddenClaims(legacy, 'legacy golden');

const docs = fs.readFileSync(path.join(root, '../../docs/scenario-coverage-v20.md'), 'utf8');
assert.match(docs, /Scenario Coverage v20/);
assert.match(docs, /proposal citation/i);
assert.match(docs, /class-to-bounded-type/i);
assert.match(docs, /four-gate rerun deltas/i);
assert.match(docs, /legacy evolve v0 golden/i);
assert.match(docs, /#1 and #23 remain open/);
assert.doesNotMatch(docs, /subjective quality score enabled|production-ready claim enabled|current Godot replacement is implemented|auto-apply enabled|auto-merge enabled/i);

console.log('scenario coverage v20 evolve depth smoke passed');
