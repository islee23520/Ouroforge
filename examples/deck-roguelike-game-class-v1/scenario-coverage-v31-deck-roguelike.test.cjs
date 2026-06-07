'use strict';

// Scenario Coverage v31 — Deck-Roguelike Game Class regression suite (#1603).
//
// Drives the enumerated regression cases live through the existing runtime,
// probe, and replay-digest surfaces and asserts states/shapes and digest
// goldens only (no flaky or timing assertions). Covers seeded determinism
// (same/different seed), snapshot-across-draw, and deck-roguelike run
// reproducibility, plus a backward-compatibility golden proving the prior
// non-stochastic grid-puzzle classes remain digest-stable.
//
// Rust mirror / CI gate:
// crates/ouroforge-core/tests/scenario_coverage_v31_deck_roguelike.rs.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const root = path.resolve(__dirname, '../..');
const runtimeDir = path.join(root, 'examples/game-runtime');
const coverageDir = path.join(root, 'examples/deck-roguelike-game-class-v1/scenario-coverage-v31');

function readJson(absPath) {
  return JSON.parse(fs.readFileSync(absPath, 'utf8'));
}

function makeRuntime(extraModule) {
  const scripts = [
    'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
    'renderer.js', 'tilemap.js', extraModule, 'runtime.js',
  ];
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in v31 coverage')),
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

function readScene(repoRef) {
  return readJson(path.join(root, repoRef));
}

// Re-realm values produced inside the vm context so deepStrictEqual compares by
// value rather than by the vm context's Object.prototype identity.
function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

function deckScene(sceneRef, seed) {
  const scene = readScene(sceneRef);
  if (seed !== undefined) {
    scene.seed = seed;
    scene.deckRoguelike.seed = seed;
  }
  return scene;
}

// Greedy deterministic driver: each turn play every affordable attack card from
// the front of the hand, then end the turn.
function driveDeck(api) {
  let guard = 0;
  let view = api.getWorldState().deckRoguelike;
  while (view && view.status === 'playing' && guard < 64) {
    guard += 1;
    let played = true;
    while (played) {
      played = false;
      for (let i = 0; i < view.hand.length; i += 1) {
        const card = view.cards[view.hand[i]];
        if (card.type === 'attack' && card.cost <= view.player.energy) {
          view = api.deckRoguelikePlayCard(i);
          played = true;
          break;
        }
      }
      if (view.status !== 'playing') break;
    }
    if (view.status !== 'playing') break;
    view = api.deckRoguelikeEndTurn();
  }
  return view;
}

const cases = readJson(path.join(coverageDir, 'cases.fixture.json'));
assert.equal(cases.schemaVersion, 'deck-roguelike-scenario-coverage-v31');
assert.equal(cases.fixtureScoped, true);
const frameId = cases.frameId;
const byKind = (kind) => cases.cases.filter((entry) => entry.kind === kind);

// Every enumerated case kind must be present, so the suite cannot silently drop
// a regression dimension.
for (const kind of ['seeded-determinism', 'seeded-divergence', 'snapshot-restore', 'run-reproducibility']) {
  assert.ok(byKind(kind).length >= 1, `coverage enumerates a ${kind} case`);
}

// --- Seeded determinism: same seed is reproducible -------------------------------
for (const testCase of byKind('seeded-determinism')) {
  const scene = deckScene(cases.sceneRef, testCase.seed);
  const apiA = makeRuntime('deck-roguelike.js');
  apiA.loadScene(scene);
  const openingA = norm(apiA.getWorldState().deckRoguelike.hand);
  assert.deepEqual(openingA, testCase.expect.openingHand, `${testCase.id}: opening hand`);
  const finalA = driveDeck(apiA);
  assert.equal(finalA.status, testCase.expect.finalStatus, `${testCase.id}: final status`);
  assert.equal(finalA.enemy.hp, testCase.expect.enemyHp, `${testCase.id}: enemy hp`);
  assert.equal(finalA.player.hp > 0, testCase.expect.playerSurvives, `${testCase.id}: player survives`);
  const digestA = apiA.replayStateDigest(frameId).digest.value;
  assert.equal(digestA, testCase.expect.runtimeDigest, `${testCase.id}: runtime digest golden`);
}

// --- Seeded divergence: a different seed shuffles and diverges --------------------
for (const testCase of byKind('seeded-divergence')) {
  const apiA = makeRuntime('deck-roguelike.js');
  apiA.loadScene(deckScene(cases.sceneRef, testCase.seedA));
  const apiB = makeRuntime('deck-roguelike.js');
  apiB.loadScene(deckScene(cases.sceneRef, testCase.seedB));
  assert.deepEqual(norm(apiA.getWorldState().deckRoguelike.hand), testCase.expect.openingHandA, `${testCase.id}: hand A`);
  assert.deepEqual(norm(apiB.getWorldState().deckRoguelike.hand), testCase.expect.openingHandB, `${testCase.id}: hand B`);
  const digestA = apiA.replayStateDigest(frameId).digest.value;
  const digestB = apiB.replayStateDigest(frameId).digest.value;
  assert.equal(digestA, testCase.expect.runtimeDigestA, `${testCase.id}: digest A golden`);
  assert.equal(digestB, testCase.expect.runtimeDigestB, `${testCase.id}: digest B golden`);
  assert.notEqual(digestA, digestB, `${testCase.id}: digests diverge`);
}

// --- Snapshot across a draw is deterministic on restore --------------------------
for (const testCase of byKind('snapshot-restore')) {
  const api = makeRuntime('deck-roguelike.js');
  api.loadScene(deckScene(cases.sceneRef, testCase.seed));
  for (let i = 0; i < testCase.playBeforeSnapshot; i += 1) api.deckRoguelikePlayCard(0);
  const snapshot = api.snapshot();
  const atSnapshot = api.replayStateDigest(frameId).digest.value;

  const runContinuation = () => {
    for (const step of testCase.continuation) {
      if (step === 'end-turn') api.deckRoguelikeEndTurn();
      else if (step.startsWith('play-card-')) api.deckRoguelikePlayCard(Number(step.slice('play-card-'.length)));
    }
    return api.replayStateDigest(frameId).digest.value;
  };

  const afterContinuation = runContinuation();
  assert.equal(
    testCase.expect.stateAdvancesBetweenSnapshotAndContinuation,
    atSnapshot !== afterContinuation,
    `${testCase.id}: state advances`,
  );
  api.restore(snapshot.snapshotId);
  const restored = api.replayStateDigest(frameId).digest.value;
  assert.equal(
    testCase.expect.restoreReturnsToSnapshotDigest,
    restored === atSnapshot,
    `${testCase.id}: restore returns to snapshot digest`,
  );
  const replayed = runContinuation();
  assert.equal(
    testCase.expect.continuationReproducesDigest,
    replayed === afterContinuation,
    `${testCase.id}: continuation reproduces digest`,
  );
}

// --- Run reproducibility: two identical-seed runs produce an identical digest ----
for (const testCase of byKind('run-reproducibility')) {
  const digests = [0, 1].map(() => {
    const api = makeRuntime('deck-roguelike.js');
    api.loadScene(deckScene(cases.sceneRef, testCase.seed));
    const final = driveDeck(api);
    assert.equal(final.status, testCase.expect.finalStatus, `${testCase.id}: final status`);
    return api.replayStateDigest(frameId).digest.value;
  });
  assert.equal(digests[0], digests[1], `${testCase.id}: two runs identical`);
  assert.equal(digests[0], testCase.expect.runtimeDigest, `${testCase.id}: digest golden`);
}

// --- Backward compatibility: non-stochastic classes remain digest-stable ---------
const golden = readJson(path.join(coverageDir, 'non-stochastic-digest.golden.json'));
assert.equal(golden.schemaVersion, 'deck-roguelike-backward-compat-golden-v31');
const gridFrameId = golden.frameId;
for (const testCase of golden.cases) {
  const api = makeRuntime('grid-puzzle.js');
  const scene = readScene(testCase.sceneRef);
  api.loadScene(scene);
  let moves = testCase.moves || [];
  if (testCase.movesFromIntendedSolution) moves = scene.gridPuzzle.intendedSolution;
  for (const direction of moves) {
    api.setInput({
      left: direction === 'left',
      right: direction === 'right',
      up: direction === 'up',
      down: direction === 'down',
    });
    api.step(1);
  }
  assert.equal(api.getWorldState().gridPuzzle.status, testCase.expectedStatus, `${testCase.id}: status`);
  assert.equal(
    api.replayStateDigest(gridFrameId).digest.value,
    testCase.expectedDigest,
    `${testCase.id}: non-stochastic digest is stable (additive deck key did not perturb it)`,
  );
}

console.log('scenario-coverage-v31-deck-roguelike.test.cjs: all v31 regression cases passed');
