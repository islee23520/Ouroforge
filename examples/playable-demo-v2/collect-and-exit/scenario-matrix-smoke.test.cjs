#!/usr/bin/env node
'use strict';

// Godot-Plus demo scenario matrix smoke (#787).
//
// Validates that the checked-in matrix maps required acceptance areas to
// pass/fail criteria, evidence artifacts, executable/read-only verification
// refs, and explicit forbidden-action boundaries. It does not execute trusted
// source writes, browser command bridges, installs, releases, or plugin code.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}

function exists(relativePath) {
  return fs.existsSync(path.join(fixtureDir, relativePath));
}

const matrix = readJson('scenarios/demo-scenario-matrix.json');
const project = readJson('ouroforge.project.json');

assert.equal(matrix.schemaVersion, 'demo-scenario-matrix-v1');
assert.equal(matrix.issue, 787);
assert.equal(matrix.projectId, 'collect_and_exit_demo');
assert.ok(project.scenarioPacks.some((pack) => pack.id === 'collect-and-exit-demo-matrix'));

const requiredScenarioIds = [
  'start-game',
  'move-player',
  'complete-level',
  'fail-restart',
  'enemy-interaction',
  'objective-update',
  'ui-state',
  'runtime-probe-state',
  'export-smoke',
  'studio-walkthrough',
  'plugin-validation',
  'evidence-bundle',
];
assert.deepEqual(matrix.scenarios.map((scenario) => scenario.id), requiredScenarioIds);

for (const [name, relativePath] of Object.entries(matrix.sourceRefs)) {
  assert.equal(exists(relativePath), true, `sourceRef ${name} resolves to ${relativePath}`);
}

const forbidden = new Set(matrix.forbiddenActions);
for (const required of [
  'directTrustedSourceWrite',
  'autoApply',
  'autoMerge',
  'browserCommandBridge',
  'arbitraryShellExecution',
  'dependencyInstall',
  'credentialedOperation',
  'publicDeploy',
  'releaseSigning',
  'storePublishing',
  'executablePluginRuntime',
  'pluginMarketplace',
  'networkPluginInstall',
]) {
  assert.equal(forbidden.has(required), true, `forbidden action ${required} is explicit`);
}

const categories = new Set(matrix.scenarios.map((scenario) => scenario.category));
for (const required of ['runtime', 'gameplay', 'behavior', 'ui', 'evidence', 'export', 'studio', 'plugin']) {
  assert.equal(categories.has(required), true, `category ${required} covered`);
}

const evidenceKinds = new Set();
for (const scenario of matrix.scenarios) {
  assert.ok(scenario.title, `${scenario.id}: title`);
  assert.ok(scenario.acceptanceCriteria.length > 0, `${scenario.id}: acceptance criteria`);
  assert.ok(scenario.passCriteria.length > 0, `${scenario.id}: pass criteria`);
  assert.ok(scenario.failCriteria.length > 0, `${scenario.id}: fail criteria`);
  assert.ok(scenario.evidenceArtifacts.length > 0, `${scenario.id}: evidence artifacts`);
  assert.ok(scenario.verificationRefs.length > 0, `${scenario.id}: verification refs`);
  for (const artifact of scenario.evidenceArtifacts) evidenceKinds.add(artifact);
  for (const ref of scenario.verificationRefs) {
    assert.equal(exists(ref), true, `${scenario.id}: verification ref ${ref} exists`);
    assert.equal(ref.includes('..'), false, `${scenario.id}: verification ref stays inside fixture`);
  }
  const joined = JSON.stringify(scenario).toLowerCase();
  for (const blocked of ['publish', 'credential', 'marketplace', 'auto-merge', 'automerge']) {
    if (joined.includes(blocked)) {
      assert.match(joined, /no|not|required|blocked|forbid|without|fail|none/, `${scenario.id}: ${blocked} mention is bounded`);
    }
  }
}

for (const requiredArtifact of [
  'world_state',
  'runtime_events',
  'frame_stats',
  'runtime_probe',
  'dashboard_read_model',
  'studio_inspector',
  'plugin_descriptor',
  'export_profile',
  'asset_provenance',
  'scenario_verdicts',
]) {
  assert.equal(evidenceKinds.has(requiredArtifact), true, `evidence artifact ${requiredArtifact} covered`);
}

for (const generatedName of ['runs', 'target', 'dashboard-data', 'dist', 'screenshots']) {
  assert.equal(exists(generatedName), false, `${generatedName} remains generated/untracked`);
}

assert.equal(matrix.qaSwarmUse.status, 'ready-for-read-only-regression-planning');
assert.match(matrix.qaSwarmUse.note, /does not grant auto-fix, auto-apply, merge, command bridge, dependency install, credential, release, or plugin runtime authority/);
assert.match(matrix.globalGuardrails.wordingBoundary, /not full Godot replacement/);

console.log(`collect-and-exit scenario matrix smoke passed; ${matrix.scenarios.length} scenarios verified`);
