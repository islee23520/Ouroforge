'use strict';

// Deck-Roguelike Game Class Demo smoke test (Milestone 31, #1602).
//
// Demonstrates, deterministically and offline, that the demo records a
// seed-reproducible deck-roguelike run with passing four-gate + loop-coverage
// evidence and a Milestone 24 ladder rung. It:
//   1. validates the demo source refs and evidence-fixture shapes,
//   2. drives the seeded run live through the existing runtime probe twice and
//      asserts an identical replay-state digest (and a divergent digest for a
//      different seed) — the live seed-reproducibility proof,
//   3. asserts the four-gate states and the rung-record linkage.
// No network, no live browser, no generated state.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const root = path.resolve(__dirname, '../../..');
const demoRoot = path.join(root, 'examples/deck-roguelike-game-class-v1/demo');
const runtimeDir = path.join(root, 'examples/game-runtime');

function readJson(absPath) {
  return JSON.parse(fs.readFileSync(absPath, 'utf8'));
}

function assertRepoRef(relativePath) {
  assert(fs.existsSync(path.join(root, relativePath)), `expected repo ref to exist: ${relativePath}`);
}

// --- 1. Source refs and manifest -------------------------------------------------
const manifest = readJson(path.join(demoRoot, 'ouroforge.project.json'));
assert.equal(manifest.schemaVersion, 'project-manifest-v1');
assert.equal(manifest.project.id, 'deck_roguelike_game_class_demo');
for (const group of [manifest.scenes, manifest.seeds, manifest.scenarioPacks]) {
  for (const ref of group) {
    assert(fs.existsSync(path.join(demoRoot, ref.path)), `manifest ref exists: ${ref.path}`);
  }
}

// --- 2. Live seed reproducibility through the runtime probe ----------------------
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'deck-roguelike.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in deck demo smoke test')),
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

function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

const scene = readJson(path.join(demoRoot, 'scenes/deck-roguelike-demo.scene.json'));

// Greedy deterministic driver: each turn play every affordable attack card from
// the front of the hand, then end the turn. Deterministic for a fixed seed.
function driveRun(api) {
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

function runDemo(seededScene) {
  const api = createRuntime();
  api.loadScene(seededScene);
  const opening = norm(api.getWorldState().deckRoguelike);
  const final = norm(driveRun(api));
  const digest = api.replayStateDigest('deck-demo-frame');
  return { api, opening, final, digest };
}

const a = runDemo(scene);
const b = runDemo(scene);

// Opening hand and outcome are deterministic for seed 12345.
assert.equal(a.opening.seed, 12345);
assert.deepEqual(a.opening.hand, ['strike', 'strike', 'bash', 'defend', 'defend']);
assert.equal(a.opening.readOnlyInspection.trustedEmitter, 'browser-runtime-deck-roguelike-world-state');
assert.equal(a.final.status, 'won', 'the seeded demo run reaches a win');
assert.equal(a.final.enemy.hp, 0);
assert.ok(a.final.player.hp > 0, 'the player survives the encounter');

// Same seed reproduces a digest-stable run.
assert.match(a.digest.digest.value, /^[0-9a-f]{16}$/);
assert.equal(a.digest.digest.value, b.digest.digest.value, 'identical seed reproduces a digest-stable run');
const matched = b.api.compareReplayDigest(a.digest, 'deck-demo-frame');
assert.equal(matched.status, 'matched');

// A different seed shuffles differently and diverges in the digest.
const divergentScene = norm(scene);
divergentScene.seed = 999;
divergentScene.deckRoguelike.seed = 999;
const c = runDemo(divergentScene);
assert.notDeepEqual(c.opening.hand, a.opening.hand, 'a different seed shuffles to a different opening hand');
assert.notEqual(c.digest.digest.value, a.digest.digest.value, 'different seeds diverge in the digest');

// The scenario pack (validated by the trusted ScenarioPack contract) records
// the read-only probe assertions for the seeded run.
const scenarioPack = readJson(path.join(demoRoot, 'scenarios/deck-roguelike-demo.json'));
assert.equal(scenarioPack.schemaVersion, 'scenario-pack-v1');
const seededScenario = scenarioPack.scenarioGroups[0].scenarios[0];
assert.equal(seededScenario.id, 'deck-roguelike-seeded-run');
assert.ok(
  seededScenario.assertions.some(
    (entry) => entry.world_state && entry.world_state.path === 'deckRoguelike.seed' && entry.world_state.equals === 12345,
  ),
  'scenario pack asserts the seeded run through the read-only probe',
);

// --- 3. Four-gate states, loop coverage, and rung linkage ------------------------
const verdict = readJson(path.join(demoRoot, 'fixtures/evidence/four-gate-verdict.fixture.json'));
assert.equal(verdict.schemaVersion, 'four-gate-verdict-v1');
assert.equal(verdict.fixtureScoped, true);
assert.equal(verdict.verdict, 'pass');
assert.deepEqual(
  verdict.gates.map((gate) => gate.id),
  ['mechanical', 'runtime', 'visual', 'semantic'],
);
for (const gate of verdict.gates) {
  assert.equal(gate.status, 'pass', `gate ${gate.id} passes`);
  for (const ref of gate.evidenceRefs) assertRepoRef(ref);
}

const coverage = readJson(path.join(demoRoot, 'fixtures/evidence/loop-coverage.fixture.json'));
assert.equal(coverage.schemaVersion, 'loop-coverage-metric-v1');
assert.equal(coverage.fixtureScoped, true);
assert.equal(coverage.summary.status, 'computed');
assert.equal(coverage.summary.coverageFraction, 1.0);

const loop = readJson(path.join(demoRoot, 'fixtures/loop/deck-roguelike-loop-run.fixture.json'));
assert.deepEqual(loop.loopShape, ['seed', 'build', 'observe', 'verify', 'journal', 'evolve']);
for (const stage of loop.stages) assertRepoRef(stage.artifactRef);

const rung = readJson(path.join(demoRoot, 'rung-demo.fixture.json'));
assert.equal(rung.schemaVersion, 'game-complexity-ladder-rung-demo-v1');
assert.equal(rung.rungGate.ladderId, 'game-complexity-ladder-v1');
assert.equal(rung.rungGate.rungId, 'deck-roguelike');
assert.equal(rung.rungGate.status, 'satisfied');
assert.equal(rung.loopCoverage.verdict, 'pass');
assert.equal(rung.guardrails.deterministic, true);
assert.equal(rung.guardrails.generatedState, false);
assert.equal(rung.guardrails.modifiesIssue1, false);
assert.equal(rung.guardrails.modifiesIssue23, false);
for (const evidence of rung.loopProducedEvidence) assertRepoRef(evidence.ref);

console.log('deck-roguelike demo-smoke.test.cjs: all demo cases passed');
