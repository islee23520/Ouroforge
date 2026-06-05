#!/usr/bin/env node
'use strict';

// Demo Scenario Matrix v1 smoke (#787).
// Validates the read-only acceptance-to-evidence matrix for the canonical
// playable-demo-v2 Collect and Exit fixture. The smoke performs no command
// execution beyond local validation and never writes generated state.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const matrixPath = path.join(fixtureDir, 'scenarios', 'demo-scenario-matrix.json');
const projectPath = path.join(fixtureDir, 'ouroforge.project.json');

const requiredAreas = new Set([
  'start game',
  'move player',
  'complete level',
  'fail/restart',
  'enemy interaction',
  'objective update',
  'UI state',
  'runtime probe state',
  'export smoke',
  'Studio walkthrough',
  'plugin validation',
  'evidence bundle',
]);

const blockedTokens = [
  'commercial release',
  'public deployment',
  'native/mobile/console export',
  'app-store/Steam/itch publishing',
  'signing or credentialed upload',
  'full Godot replacement or parity claim',
  'production-ready claim',
  'direct trusted Studio source write',
  'source mutation bypass',
  'auto-apply',
  'auto-merge',
  'self-approval',
  'executable plugin runtime',
  'marketplace or network plugin install/update',
  'dependency install/update',
  'CI/workflow mutation',
  'browser command bridge',
  'arbitrary shell execution',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function assertRepoRefExists(ref) {
  if (ref.startsWith('#')) return;
  if (/^[a-z]+ test /.test(ref) || ref.startsWith('node ') || ref.startsWith('cargo ')) return;
  const candidates = [
    path.join(fixtureDir, ref),
    path.join(repoRoot, ref),
  ];
  assert.ok(candidates.some((candidate) => fs.existsSync(candidate)), `fixture ref exists: ${ref}`);
}

const matrix = readJson(matrixPath);
const project = readJson(projectPath);

assert.equal(matrix.schemaVersion, 'demo-scenario-matrix-v1');
assert.equal(matrix.issue, 787);
assert.equal(matrix.scope.fixtureRoot, 'examples/playable-demo-v2/collect-and-exit');
assert.deepEqual(matrix.governance.protectedIssuesMustRemainOpen, [1, 23]);
assert.equal(matrix.restartEvidenceRequired, true);

for (const token of blockedTokens) {
  assert.ok(matrix.blockedActions.includes(token), `blocked action listed: ${token}`);
}

assert.ok(
  project.scenarioPacks.some((pack) => pack.id === matrix.id && pack.path === 'scenarios/demo-scenario-matrix.json'),
  'project manifest registers the demo scenario matrix pack'
);

assert.ok(Array.isArray(matrix.scenarioFixtures) && matrix.scenarioFixtures.length >= 8, 'scenario fixtures are declared');
for (const fixture of matrix.scenarioFixtures) {
  assert.ok(fixture.id && fixture.path && fixture.type && fixture.purpose, `fixture shape: ${fixture.id}`);
  assertRepoRefExists(fixture.path);
}

assert.equal(matrix.scenarios.length, requiredAreas.size, 'one scenario per required acceptance area');
const seenIds = new Set();
const coveredAreas = new Set();
let scenariosWithForbiddenGuard = 0;
for (const scenario of matrix.scenarios) {
  assert.ok(scenario.id && /^demo-/.test(scenario.id), `scenario id prefix: ${scenario.id}`);
  assert.equal(seenIds.has(scenario.id), false, `unique scenario id: ${scenario.id}`);
  seenIds.add(scenario.id);
  coveredAreas.add(scenario.area);

  assert.ok(requiredAreas.has(scenario.area), `required area covered: ${scenario.area}`);
  assert.ok(scenario.description && scenario.description.length > 20, `${scenario.id}: description`);
  assert.ok(Array.isArray(scenario.acceptanceCriteria) && scenario.acceptanceCriteria.length > 0, `${scenario.id}: acceptance criteria`);
  assert.ok(Array.isArray(scenario.passCriteria) && scenario.passCriteria.length > 0, `${scenario.id}: pass criteria`);
  assert.ok(Array.isArray(scenario.failCriteria) && scenario.failCriteria.length > 0, `${scenario.id}: fail criteria`);
  assert.ok(Array.isArray(scenario.evidenceExpectations) && scenario.evidenceExpectations.length > 0, `${scenario.id}: evidence expectations`);
  assert.ok(Array.isArray(scenario.fixtureRefs) && scenario.fixtureRefs.length > 0, `${scenario.id}: fixture refs`);
  assert.ok(Array.isArray(scenario.verificationRefs) && scenario.verificationRefs.length > 0, `${scenario.id}: verification refs`);
  assert.ok(Array.isArray(scenario.forbiddenActions), `${scenario.id}: forbidden actions array`);

  for (const expectation of scenario.evidenceExpectations) {
    assert.ok(expectation.kind && expectation.path && Object.hasOwn(expectation, 'expected'), `${scenario.id}: evidence expectation shape`);
  }
  for (const ref of scenario.fixtureRefs) assertRepoRefExists(ref);
  for (const action of scenario.forbiddenActions) {
    assert.ok(matrix.blockedActions.includes(action), `${scenario.id}: forbidden action inherits matrix guardrail: ${action}`);
  }
  if (scenario.forbiddenActions.length > 0) scenariosWithForbiddenGuard += 1;
}

assert.deepEqual([...coveredAreas].sort(), [...requiredAreas].sort(), 'all required scenario areas covered');
assert.ok(scenariosWithForbiddenGuard >= 8, 'most scenarios carry explicit forbidden-action guardrails');
assert.match(matrix.qaSwarmReadiness.notes, /does not authorize autonomous fixes/i);
assert.match(matrix.governance.wordingBoundary, /no Godot replacement\/parity\/production-ready\/commercial-release claim/i);

console.log(`demo scenario matrix smoke passed; ${matrix.scenarios.length} scenarios validated`);
