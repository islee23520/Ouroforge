const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '../../..');
const demo = path.join(root, 'examples/evolve-loop-depth-v1/demo');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(demo, relativePath), 'utf8'));
}

function readText(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), 'utf8');
}

function assertExists(relativePath) {
  assert.equal(fs.existsSync(path.join(demo, relativePath)), true, `${relativePath} exists`);
}

function assertFixtureEvidenceExists(runDir, refs) {
  for (const ref of refs) {
    assertExists(path.join(runDir, ref));
  }
}

const seed = readText('seeds/evolve-loop-depth-v1-demo.yaml');
const docs = readText('docs/evolve-loop-depth-v1-demo.md');
const before = readJson('before-run/verdict.json');
const after = readJson('after-run/verdict.json');
const proposals = readJson('before-run/mutation/proposals.json');
const operation = readJson('apply/operation.scene-only.json');
const transaction = readJson('after-run/transactions/scene-edit-depth-demo-align-player-x.json');
const applications = readJson('before-run/mutation/scene-applications.json');
const comparison = readJson('comparison/run-comparison-before-run--after-run.json');

assert.match(seed, /visual-depth-demo/);
assert.match(docs, /offline and fixture-scoped/);
assert.match(docs, /#215/);
assert.match(docs, /scene-only-mutation-v1/);
const forbiddenClaimPattern = new RegExp([
  ['production', 'ready'].join('-'),
  ['current Godot', 'replacement'].join(' '),
  ['auto', 'fix enabled'].join('-'),
  ['auto', 'apply enabled'].join('-'),
  ['auto', 'merge enabled'].join('-'),
  ['browser-owned trusted', 'persistence'].join(' '),
].join('|'), 'i');
assert.doesNotMatch(docs, forbiddenClaimPattern);

assert.equal(before.status, 'failed');
assert.equal(before.gateCategories.visual.status, 'fail');
assert.equal(before.gateCategories.mechanical.status, 'pass');
assert.equal(before.gateCategories.runtime.status, 'pass');
assert.equal(before.gateCategories.semantic.status, 'pass');
assert.equal(before.failures[0].kind, 'visual_gate_failed');
assert.match(before.failures[0].path, /visual-comparison\.json/);
assertFixtureEvidenceExists('before-run', before.evidence_refs);
assertFixtureEvidenceExists('before-run', before.visual[0].evidenceRefs);

assert.equal(after.status, 'passed');
for (const gate of ['mechanical', 'runtime', 'visual', 'semantic']) {
  assert.equal(after.gateCategories[gate].status, 'pass', `${gate} passes after rerun`);
}
assertFixtureEvidenceExists('after-run', after.evidence_refs);

assert.equal(proposals.proposals.length, 1);
const proposal = proposals.proposals[0];
assert.equal(proposal.status, 'proposed');
assert.equal(proposal.rationale.failing_gate_category, 'visual');
assert.equal(proposal.rationale.evidence_state, 'linked');
assert.equal(proposal.rationale.bounded_mutation_type, 'scene');
assert.equal(proposal.rationale.allowed_mutation_type, 'scene_only');
assert.equal(proposal.rationale.backlog_read_only, true);
assert.equal(proposal.rationale.justifying_evidence_ref, before.failures[0].path);
assert.ok(proposal.rationale.evidence_artifact_ids.includes(proposal.evidence_id));
assertFixtureEvidenceExists('before-run', [proposal.rationale.justifying_evidence_ref]);

assert.equal(operation.schemaVersion, 'scene-only-mutation-v1');
assert.equal(operation.proposalId, proposal.id);
assert.equal(operation.validationRequired, true);
assert.equal(operation.edit.entityId, 'player');
assert.equal(operation.edit.path, 'components.transform.x');
assert.equal(operation.edit.value, 96);
assert.equal(operation.expectedBeforeSceneHash.algorithm, 'fnv1a64-canonical-json-v1');

assert.equal(transaction.schemaVersion, 'ouroforge.scene-edit-transaction.v1');
assert.equal(transaction.edit.path, operation.edit.path);
assert.deepEqual(transaction.beforeSceneHash, operation.expectedBeforeSceneHash);
assert.equal(transaction.validationResult.status, 'passed');
assert.notEqual(transaction.beforeSceneHash.value, transaction.afterSceneHash.value);

assert.equal(applications.applications.length, 1);
assert.equal(applications.applications[0].proposalId, proposal.id);
assert.equal(applications.applications[0].transactionId, transaction.id);
assert.equal(applications.applications[0].status, 'applied');

assert.equal(comparison.classification, 'improved');
assert.equal(comparison.comparability.state, 'comparable');
const deltas = Object.fromEntries(comparison.fourGateDeltas.map((delta) => [delta.gate, delta]));
assert.deepEqual(Object.keys(deltas).sort(), ['mechanical', 'runtime', 'semantic', 'visual']);
assert.equal(deltas.visual.before_status, 'fail');
assert.equal(deltas.visual.after_status, 'pass');
assert.equal(deltas.visual.transition, 'fail_to_pass');
for (const gate of ['mechanical', 'runtime', 'semantic']) {
  assert.equal(deltas[gate].transition, 'unchanged_pass', `${gate} unchanged pass`);
}
assert.ok(
  comparison.semantic.reasons.some((reason) => reason.summary.includes('fail_to_pass')),
  'semantic diff summarizes visual fail_to_pass',
);

console.log('evolve loop depth v1 demo smoke passed');
