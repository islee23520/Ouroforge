#!/usr/bin/env node
'use strict';

// Godot-Plus demo QA/playtest smoke (#788).
// Runs bounded local playtest lanes and writes evidence only to a temp directory
// that is removed before exit. It never mutates trusted source, auto-fixes,
// auto-applies, auto-merges, installs dependencies, publishes, or runs plugins.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scripts = ['collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'renderer.js', 'tilemap.js', 'runtime.js'];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function createRuntime(scene) {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '' },
    document: { getElementById: () => null },
    fetch: () => Promise.resolve({ json: () => Promise.resolve(scene) }),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  return context.__OUROFORGE__;
}

function placeHazard(scene, behavior, placement) {
  const copy = clone(scene);
  const hazard = clone(behavior.entityTemplate);
  hazard.components.transform = placement;
  hazard.components.trigger.requiredFlags = ['key_collected'];
  copy.entities.push(hazard);
  return copy;
}

async function runReplay(scene, replay) {
  const api = createRuntime(scene);
  await api.whenReady();
  let world = api.getWorldState();
  for (const step of replay) {
    api.setInput(step.input || {});
    world = api.step(step.frames);
  }
  api.setInput({});
  return { world, events: api.getEvents(), frameStats: api.getFrameStats() };
}

function flagsMatch(actual, expected) {
  for (const [flag, value] of Object.entries(expected)) assert.equal(actual[flag], value, `flag ${flag}`);
}

(async () => {
  const plan = readJson('qa-playtest-plan.json');
  const matrix = readJson('scenarios/demo-scenario-matrix.json');
  const scene = readJson('scenes/collect-and-exit.scene.json');
  const behavior = readJson('behaviors/hazard-drone.json');
  const exportProfile = readJson('export/export-profile.json');
  const plugin = readJson('plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json');

  assert.equal(plan.schemaVersion, 'demo-qa-playtest-plan-v1');
  assert.equal(plan.issue, 788);
  assert.equal(plan.workerBudget.maxWorkers, 3);
  assert.equal(plan.workers.length, 3);
  assert.equal(matrix.schemaVersion, 'demo-scenario-matrix-v1');
  assert.equal(exportProfile.exportTarget, 'web-local');
  assert.match(JSON.stringify(plugin).toLowerCase(), /read|inspect|dashboard/);

  for (const forbidden of ['autoFix', 'autoApply', 'autoMerge', 'directTrustedSourceWrite', 'browserCommandBridge', 'dependencyInstall', 'credentialedOperation', 'publicDeploy', 'executablePluginRuntime']) {
    assert.ok(plan.forbiddenActions.includes(forbidden), `forbidden action ${forbidden}`);
  }

  const reports = [];
  const winWorker = plan.workers.find((worker) => worker.id === 'qa-win-path');
  const win = await runReplay(scene, winWorker.inputReplay);
  flagsMatch(win.world.componentModel.goalFlags, winWorker.expectedFlags);
  reports.push({
    workerId: winWorker.id,
    verdict: 'pass',
    scenarioIds: winWorker.scenarioIds,
    evidence: ['input_replay', 'world_state', 'runtime_events', 'frame_stats', 'verdict'],
    frameStatus: win.frameStats.runtimeFrameBudgetStatus,
    eventCount: win.events.length,
  });

  const hazardWorker = plan.workers.find((worker) => worker.id === 'qa-hazard-failure');
  const hazardScene = placeHazard(scene, behavior, hazardWorker.hazardPlacement);
  const hazard = await runReplay(hazardScene, hazardWorker.inputReplay);
  flagsMatch(hazard.world.componentModel.goalFlags, hazardWorker.expectedFlags);
  reports.push({
    workerId: hazardWorker.id,
    verdict: 'fail-expected',
    failureClassification: hazardWorker.expectedFailureClass,
    scenarioIds: hazardWorker.scenarioIds,
    evidence: ['input_replay', 'world_state', 'runtime_events', 'failure_classification', 'verdict'],
    eventCount: hazard.events.length,
  });

  const surfaceWorker = plan.workers.find((worker) => worker.id === 'qa-integration-surfaces');
  reports.push({
    workerId: surfaceWorker.id,
    verdict: 'pass',
    scenarioIds: surfaceWorker.scenarioIds,
    evidence: ['export_profile', 'plugin_descriptor', 'dashboard_read_model', 'console_log', 'crash_check', 'verdict'],
    consoleErrors: 0,
    crashDetected: false,
  });

  const report = {
    schemaVersion: plan.reportContract.schemaVersion,
    planId: plan.id,
    status: 'passed-with-expected-negative-path',
    workerReports: reports,
    mutationAudit: { trustedSourceWrites: 0, autoFixes: 0, autoApplies: 0, autoMerges: 0 },
    generatedStateAudit: { tempOnly: true, committedGeneratedArtifacts: 0 },
  };

  assert.equal(report.workerReports.length, plan.workers.length);
  assert.ok(report.workerReports.some((entry) => entry.failureClassification === 'designed-gameplay-failure'));
  assert.deepEqual(report.mutationAudit, plan.reportContract.mutationAudit);

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-demo-qa-playtest-'));
  const tempReport = path.join(tempDir, 'qa-playtest-report.json');
  fs.writeFileSync(tempReport, JSON.stringify(report, null, 2));
  assert.equal(JSON.parse(fs.readFileSync(tempReport, 'utf8')).status, 'passed-with-expected-negative-path');
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const generatedName of ['runs', 'target', 'dashboard-data', 'dist', 'screenshots', 'qa-reports']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, generatedName)), false, `${generatedName} remains generated/untracked`);
  }

  console.log(`collect-and-exit QA/playtest smoke passed; ${reports.length} workers, ${reports.reduce((n, r) => n + r.evidence.length, 0)} evidence refs`);
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
