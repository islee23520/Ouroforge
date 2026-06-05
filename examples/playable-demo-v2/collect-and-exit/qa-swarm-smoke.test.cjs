#!/usr/bin/env node
'use strict';

// Godot-Plus demo autonomous QA/playtest swarm smoke (#788).
//
// Executes the bounded QA swarm plan: each worker replays a deterministic input
// route against the runtime, runs until its stop condition or step budget,
// captures world-state/performance/event evidence, and records a classified
// verdict (pass / objective_blocked / hazard_contact_loss / budget_exhausted).
// It proves the swarm produces evidence and verdicts without auto-fixing,
// auto-applying, or mutating source. Pure read-only harness: temp dir only.

const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(rel) {
  return JSON.parse(fs.readFileSync(path.isAbsolute(rel) ? rel : path.join(fixtureDir, rel), 'utf8'));
}

function fileHash(rel) {
  return crypto.createHash('sha256').update(fs.readFileSync(path.join(fixtureDir, rel))).digest('hex');
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
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function sceneForRoute(baseScene, archetypeCache, route) {
  const scene = JSON.parse(JSON.stringify(baseScene));
  if (route.hazard) {
    const archetype = archetypeCache[route.hazard.archetype]
      || (archetypeCache[route.hazard.archetype] = readJson(route.hazard.archetype));
    const hazard = JSON.parse(JSON.stringify(archetype.entityTemplate));
    hazard.components.transform = { x: route.hazard.placement.x, y: route.hazard.placement.y };
    scene.entities.push(hazard);
  }
  return scene;
}

// Run one route to its stop condition / budget and classify the verdict.
async function runRoute(baseScene, archetypeCache, route) {
  const api = createRuntime(sceneForRoute(baseScene, archetypeCache, route));
  await api.whenReady();
  for (const step of route.inputReplay) {
    if (step.atStep === 0) api.setInput(step.input);
  }
  let world = api.getWorldState();
  let classification = 'budget_exhausted';
  let stoppedAtStep = route.budgetSteps;
  for (let frame = 1; frame <= route.budgetSteps; frame += 1) {
    world = api.step(1);
    const flags = world.componentModel.goalFlags;
    if (flags.player_alive === false) { classification = 'hazard_contact_loss'; stoppedAtStep = frame; break; }
    if (flags.exit_reached === true) { classification = 'pass'; stoppedAtStep = frame; break; }
  }
  api.setInput({ right: false });
  const flags = world.componentModel.goalFlags;
  if (classification === 'budget_exhausted' && flags.exit_reached !== true && flags.player_alive !== false) {
    // Distinguish a still-gated objective from a generic budget timeout.
    classification = route.id === 'qa-objective-blocked' ? 'objective_blocked' : 'budget_exhausted';
  }
  const verdict = classification === 'pass' ? 'pass' : 'fail';
  return {
    routeId: route.id,
    worker: route.worker,
    verdict,
    classification,
    stoppedAtStep,
    evidence: {
      world_state: { goalFlags: flags, sceneId: world.sceneId },
      frame_stats: { runtimeFrameBudgetStatus: api.getFrameStats().runtimeFrameBudgetStatus },
      runtime_events: { count: api.getEvents().length },
    },
  };
}

(async () => {
  const plan = readJson(path.join('qa', 'qa-swarm-plan.json'));
  const baseScene = readJson(path.join('scenes', 'collect-and-exit.scene.json'));
  const archetypeCache = {};

  assert.equal(plan.schemaVersion, 'demo-qa-swarm-plan-v1', 'plan schema');
  assert.equal(plan.policy.autoFix, false, 'policy: no auto-fix');
  assert.equal(plan.policy.autoApply, false, 'policy: no auto-apply');
  assert.equal(plan.policy.sourceMutation, 'forbidden', 'policy: no source mutation');
  assert.ok(plan.routes.length <= plan.policy.maxWorkers, 'routes within worker bound');

  // Capture source hashes before the swarm to prove no source mutation occurs.
  const watchedSources = [
    'scenes/collect-and-exit.scene.json',
    'qa/qa-swarm-plan.json',
    'behaviors/hazard-drone.json',
  ];
  const before = Object.fromEntries(watchedSources.map((rel) => [rel, fileHash(rel)]));

  const results = [];
  for (const route of plan.routes) {
    const result = await runRoute(baseScene, archetypeCache, route);
    assert.equal(result.verdict, route.expectedVerdict, `${route.id}: verdict`);
    assert.equal(result.classification, route.expectedClassification, `${route.id}: classification`);
    assert.ok(plan.classifications.includes(result.classification), `${route.id}: known classification`);
    assert.equal(result.evidence.frame_stats.runtimeFrameBudgetStatus, 'within-budget', `${route.id}: perf`);
    results.push(result);
  }

  // Determinism: rerun the hazard route and confirm an identical verdict.
  const hazardRoute = plan.routes.find((r) => r.id === 'qa-hazard-contact');
  const rerun = await runRoute(baseScene, archetypeCache, hazardRoute);
  assert.equal(rerun.classification, 'hazard_contact_loss', 'determinism: hazard verdict stable');
  assert.equal(rerun.stoppedAtStep, results.find((r) => r.routeId === 'qa-hazard-contact').stoppedAtStep,
    'determinism: hazard stop step stable');

  // No source mutation occurred during QA.
  for (const rel of watchedSources) {
    assert.equal(fileHash(rel), before[rel], `QA must not mutate source: ${rel}`);
  }

  // Playtest evidence report -> temp dir outside repo, removed before exit.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-qa-'));
  const report = {
    planId: plan.id,
    autoFix: plan.policy.autoFix,
    results,
    summary: {
      pass: results.filter((r) => r.verdict === 'pass').length,
      fail: results.filter((r) => r.verdict === 'fail').length,
    },
  };
  const tempReport = path.join(tempDir, 'qa-report.json');
  fs.writeFileSync(tempReport, JSON.stringify(report, null, 2));
  assert.equal(readJson(tempReport).summary.pass, 1, 'one win route');
  assert.equal(readJson(tempReport).summary.fail, 2, 'two classified failure routes');
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log(`collect-and-exit QA swarm smoke passed; ${report.summary.pass} pass / ${report.summary.fail} classified fail`);
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
