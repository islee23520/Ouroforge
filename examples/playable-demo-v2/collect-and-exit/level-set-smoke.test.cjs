#!/usr/bin/env node
'use strict';

// Godot-Plus demo level set smoke (#783).
//
// Loads the deterministic Collect-and-Exit level set and, for each level, applies
// the level's deterministic spawn/key/gate configuration over the base scene,
// plays it to a win through the existing runtime, and evaluates the level's
// per-level scenario assertions. Also proves the difficulty ramp is monotonic,
// the level set is bounded (3-5 levels), and the runtime can identify the current
// level state. Pure read-only harness: writes only to a temp dir outside the repo
// and removes it before exit.

const assert = require('node:assert/strict');
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

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
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

function setEntityTransform(scene, entityId, pos) {
  const entity = scene.entities.find((candidate) => candidate.id === entityId);
  assert.ok(entity, `level scene must contain entity ${entityId}`);
  entity.components = entity.components || {};
  entity.components.transform = { x: pos.x, y: pos.y };
}

// Build a deterministic level scene by applying the level config over the base.
function buildLevelScene(baseScene, level) {
  const scene = JSON.parse(JSON.stringify(baseScene));
  setEntityTransform(scene, 'player', level.spawn);
  setEntityTransform(scene, 'key', level.key);
  setEntityTransform(scene, 'door', level.door);
  scene.metadata = scene.metadata || {};
  scene.metadata.levelId = level.id;
  scene.metadata.levelTitle = level.title;
  scene.metadata.startState = scene.metadata.startState || {};
  scene.metadata.startState.spawn = { x: level.spawn.x, y: level.spawn.y };
  if (level.frameBudget) {
    scene.metadata.runtimeDebug = scene.metadata.runtimeDebug || {};
    scene.metadata.runtimeDebug.frameBudget = level.frameBudget;
  }
  return scene;
}

function evaluateAssertion(goalFlags, assertion) {
  const segments = String(assertion.path).split('.');
  let actual = { goalFlags };
  for (const segment of segments) actual = actual === undefined ? undefined : actual[segment];
  assert.deepEqual(actual, assertion.equals, `assertion ${assertion.path}`);
}

(async () => {
  const levelSet = readJson(path.join(fixtureDir, 'levels', 'level-set.json'));
  const baseScene = readJson(path.join(fixtureDir, levelSet.baseScene));

  assert.equal(levelSet.schemaVersion, 'demo-level-set-v1', 'level set schema');
  assert.ok(
    levelSet.levels.length >= 3 && levelSet.levels.length <= 5,
    'level set must be bounded to 3-5 levels'
  );

  const summaries = [];
  let previousTier = 0;

  for (const level of levelSet.levels) {
    // Each level has objective metadata, difficulty, deterministic spawn, scenario.
    assert.ok(level.objective && level.objective.length > 0, `${level.id}: objective`);
    assert.ok(Number.isInteger(level.difficultyTier), `${level.id}: difficulty tier`);
    assert.ok(level.spawn && Number.isFinite(level.spawn.x), `${level.id}: deterministic spawn`);
    assert.ok(level.scenario && Array.isArray(level.scenario.assertions), `${level.id}: scenario`);
    assert.ok(level.difficultyTier > previousTier, `${level.id}: difficulty ramp is monotonic`);
    previousTier = level.difficultyTier;

    const scene = buildLevelScene(baseScene, level);
    const api = createRuntime(scene);
    await api.whenReady();

    // Runtime can identify the current level state.
    const initial = api.getWorldState();
    assert.equal(initial.metadata.levelId, level.id, `${level.id}: current level identifiable`);

    // Play the level to a win deterministically.
    api.setInput({ right: true });
    const world = api.step(200);
    api.setInput({ right: false });
    const goalFlags = world.componentModel.goalFlags;

    // Per-level scenario coverage.
    for (const assertion of level.scenario.assertions) evaluateAssertion(goalFlags, assertion);

    // Frame budget stays within bounds (tighter for higher tiers).
    assert.equal(
      api.getFrameStats().runtimeFrameBudgetStatus,
      'within-budget',
      `${level.id}: frame budget held`
    );

    summaries.push({
      id: level.id,
      tier: level.difficultyTier,
      scenarioId: level.scenario.id,
      won: goalFlags.exit_reached === true,
    });
  }

  assert.ok(summaries.every((entry) => entry.won), 'every level is winnable');

  // Evidence shape can be written to a temp dir outside the repo, then removed.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-levels-'));
  const tempEvidence = path.join(tempDir, 'level-set.json');
  fs.writeFileSync(tempEvidence, JSON.stringify({ levelSetId: levelSet.id, summaries }, null, 2));
  assert.equal(readJson(tempEvidence).summaries.length, levelSet.levels.length);
  fs.rmSync(tempDir, { recursive: true, force: true });

  // Generated-state audit.
  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log(`collect-and-exit level set smoke passed; ${summaries.length} levels winnable`);
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
