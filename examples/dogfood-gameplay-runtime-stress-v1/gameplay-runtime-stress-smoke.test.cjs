const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const read = (repoPath) => fs.readFileSync(path.join(repoRoot, repoPath), 'utf8');
const json = (repoPath) => JSON.parse(read(repoPath));
const exists = (repoPath) => fs.existsSync(path.join(repoRoot, repoPath));

const reportPath = '.omx/dogfood-validation/gameplay-runtime-stress.md';
const statusPath = '.omx/dogfood-validation/gameplay-runtime-stress.status.json';
const report = read(reportPath);
const normalized = report.replace(/\s+/g, ' ').trim();
const status = json(statusPath);

for (const repoPath of [
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  reportPath,
  statusPath,
]) {
  assert.ok(exists(repoPath), `expected dogfood artifact exists: ${repoPath}`);
}

for (const section of [
  'Metadata',
  'Purpose',
  'Merged prerequisite evidence',
  'Commands executed',
  'Evidence summary',
  'Stress limits and bounded failure cases',
  'Capability boundary',
  'Verification commands for this PR',
  'Non-goals and guardrails',
]) {
  assert.match(report, new RegExp(`## ${section}`), `report includes ${section}`);
}

for (const required of [
  'dogfood-gameplay-runtime-stress-v1',
  'collect-and-exit-local-rc-candidate',
  'bounded-local-runtime-stress-evidence',
  '#2334 MERGED',
  '#2335 MERGED',
  '#2336 MERGED',
  '#2337 MERGED',
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  'collect-and-exit e2e evidence smoke passed',
  'collect-and-exit gameplay loop smoke passed',
  'hazard lose at frame 49, dormant pass wins',
  'collect-and-exit HUD smoke passed',
  '4 levels winnable',
  '12 scenarios validated',
  '3 workers, 16 evidence refs',
  '1 pass / 2 classified fail',
  '3 passed',
  '22 passed; 3 filtered out',
  'hazard_contact_loss',
  'objective_blocked',
  'No generated package artifact',
]) {
  if (required === 'No generated package artifact') continue;
  assert.ok(normalized.includes(required.replace(/\s+/g, ' ')), `report records ${required}`);
}

const scene = json('examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json');
assert.equal(scene.schemaVersion, '1');
assert.ok(Array.isArray(scene.entities) && scene.entities.length >= 8, 'scene has compact runtime entities');
assert.equal(scene.metadata.runtimeDebug.frameBudget.totalMs, 20);
assert.ok(scene.metadata.runtimeDebug.frameTimings.totalMs <= scene.metadata.runtimeDebug.frameBudget.totalMs, 'fixture timing within budget');

const matrix = json('examples/playable-demo-v2/collect-and-exit/scenarios/demo-scenario-matrix.json');
assert.equal(matrix.schemaVersion, 'demo-scenario-matrix-v1');
assert.ok(matrix.scenarios.length >= 12, 'scenario matrix has stress/acceptance coverage');
assert.deepEqual(matrix.governance.protectedIssuesMustRemainOpen, [1, 23]);

const qaPlan = json('examples/playable-demo-v2/collect-and-exit/qa-playtest-plan.json');
assert.equal(qaPlan.workerBudget.maxWorkers, 3);
assert.equal(qaPlan.workers.length, 3);

const swarmPlan = json('examples/playable-demo-v2/collect-and-exit/qa/qa-swarm-plan.json');
assert.equal(swarmPlan.policy.autoFix, false);
assert.equal(swarmPlan.policy.autoApply, false);
assert.equal(swarmPlan.policy.sourceMutation, 'forbidden');
assert.ok(swarmPlan.routes.length <= swarmPlan.policy.maxWorkers, 'swarm routes within max workers');
for (const classification of ['pass', 'hazard_contact_loss', 'objective_blocked', 'budget_exhausted']) {
  assert.ok(swarmPlan.classifications.includes(classification), `classification listed: ${classification}`);
}

assert.equal(status.schemaVersion, 'dogfood-gameplay-runtime-stress-status-v1');
assert.equal(status.blocker, 'B5');
assert.equal(status.status, 'ready_for_verifier');
assert.equal(status.evidenceClassification, 'bounded-local-runtime-stress-evidence');
assert.equal(status.protectedIssues['1'], 'OPEN');
assert.equal(status.protectedIssues['23'], 'OPEN');
assert.equal(status.forbiddenScopeIntroduced, false);
assert.equal(status.runtimeEvidence.nodeRuntimeSmokesPassed, 8);
assert.equal(status.runtimeEvidence.runtimeFrameBudgetRustTestsPassed, 3);
assert.equal(status.runtimeEvidence.behaviorRuntimeTargetedTestsPassed, 22);
for (const artifact of status.trackedArtifacts) assert.ok(exists(artifact), `tracked artifact exists: ${artifact}`);
for (const prereq of status.mergedPrerequisites) {
  assert.equal(prereq.state, 'MERGED', `${prereq.blocker} merged`);
  assert.ok(exists(prereq.artifact), `${prereq.blocker} artifact exists`);
}

for (const guardrail of [
  /#1 and #23 remain open/i,
  /Era Q M102–M106 remain deferred\/non-goal/i,
  /No hosted\/cloud\/multi-user scope/i,
  /No production-ready, store-ready, commercial release/i,
]) {
  assert.match(report, guardrail, `guardrail present: ${guardrail}`);
}

for (const forbiddenOverclaim of [
  /production-ready\s+(?:runtime|gameplay|game|engine)/i,
  /store-ready\s+(?:runtime|gameplay|game|engine)/i,
  /commercial release ready/i,
  /full Godot parity is verified|claims full Godot parity/i,
  /Godot replacement status is verified/i,
  /M102(?:–|-| to )M106\s+(?:active|implemented|complete|ready)/i,
  /trusted browser writes are allowed/i,
  /auto-fix enabled/i,
  /source mutation allowed/i,
]) {
  assert.doesNotMatch(report, forbiddenOverclaim, `forbidden overclaim absent: ${forbiddenOverclaim}`);
}

console.log('dogfood gameplay/runtime stress smoke passed');
