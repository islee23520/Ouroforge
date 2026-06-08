'use strict';

// Runtime contract test for the Deck-Roguelike Game Class v1 (#1601).
// Mirror of the Rust contract test
// crates/ouroforge-core/tests/deck_roguelike_game_class_contract.rs.
//
// Validates, through the existing runtime probe / replay-digest surfaces, that:
// a seeded deck run is probe-exposed and observable, identical seeds reproduce
// the same run (digest-stable), different seeds diverge detectably, and a
// malformed deck fails closed at scene load.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'deck-roguelike.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in deck roguelike test')),
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

function readScene(file) {
  return JSON.parse(fs.readFileSync(path.join(runtimeDir, file), 'utf8'));
}

// Re-realm values produced inside the vm context so deepStrictEqual compares by
// value rather than by the vm context's Object.prototype identity.
function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

// Greedy deterministic driver: each turn, play every affordable attack card
// from the front of the hand, then end the turn. Drives the same trajectory for
// the same seed, so it is a deterministic action sequence.
function driveRun(api) {
  let guard = 0;
  let view = api.getWorldState().deckRoguelike;
  while (view && view.status === 'playing' && guard < 64) {
    guard += 1;
    let played = true;
    while (played) {
      played = false;
      const hand = view.hand;
      for (let i = 0; i < hand.length; i += 1) {
        const card = view.cards[hand[i]];
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

const scene = readScene('deck-roguelike-scene-v1.json');

// --- Probe world-state exposure ----------------------------------------------
{
  const api = createRuntime();
  api.loadScene(scene);
  const view = norm(api.getWorldState().deckRoguelike);
  assert.equal(view.schemaVersion, 'ouroforge.deck-roguelike-state.v1');
  assert.equal(view.status, 'playing');
  assert.equal(view.seed, 12345);
  assert.equal(view.turn, 1);
  // Seeded opening hand and remaining draw pile are deterministic for seed 12345.
  assert.deepEqual(view.hand, ['strike', 'strike', 'bash', 'defend', 'defend']);
  assert.deepEqual(view.drawPile, ['strike', 'strike', 'strike', 'bash', 'defend']);
  // The run-start relic grants 3 starting block; energy is the per-turn budget.
  assert.equal(view.player.block, 3);
  assert.equal(view.player.energy, 3);
  assert.equal(view.enemy.hp, 20);
  assert.deepEqual(view.enemy.intent, { type: 'attack', value: 8 });
  // The world-state is read-only inspection evidence, never a trusted write path.
  assert.deepEqual(view.readOnlyInspection.disallowedActions, [
    'trusted writes', 'command bridge', 'live mutation',
  ]);
  assert.equal(view.readOnlyInspection.trustedEmitter, 'browser-runtime-deck-roguelike-world-state');
  assert.equal(view.cardRogueliteSubstrate.schemaVersion, 'ouroforge.card-roguelite-substrate-probe.v1');
  assert.equal(view.cardRogueliteSubstrate.variant, 'deck-roguelike-classic');
  assert.match(view.cardRogueliteSubstrate.digest.value, /^[0-9a-f]{16}$/);
  assert.equal(view.cardRogueliteSubstrate.readOnlyInspection.trustedEmitter, 'browser-runtime-card-roguelite-substrate-probe');
}

// --- A seeded run is playable and observable ---------------------------------
{
  const api = createRuntime();
  api.loadScene(scene);
  const final = norm(driveRun(api));
  assert.equal(final.status, 'won', 'the seeded deck run reaches a win');
  assert.equal(final.enemy.hp, 0);
  assert.ok(final.player.hp > 0, 'the player survives the encounter');
}

// --- Same-seed reproducibility (digest-stable via the replay digest) ----------
{
  const a = createRuntime();
  a.loadScene(scene);
  driveRun(a);
  const digestA = a.replayStateDigest('deck-frame');

  const b = createRuntime();
  b.loadScene(scene);
  driveRun(b);
  const digestB = b.replayStateDigest('deck-frame');

  assert.match(digestA.digest.value, /^[0-9a-f]{16}$/);
  assert.equal(digestA.digest.value, digestB.digest.value, 'identical seed reproduces a digest-stable run');
  const matched = b.compareReplayDigest(digestA, 'deck-frame');
  assert.equal(matched.status, 'matched');
  assert.equal(matched.firstDivergence, null);
}

// --- Different seed diverges detectably ---------------------------------------
{
  const a = createRuntime();
  a.loadScene(scene);
  const digestA = a.replayStateDigest('deck-frame');

  const divergentScene = norm(scene);
  divergentScene.seed = 999;
  divergentScene.deckRoguelike.seed = 999;
  const b = createRuntime();
  b.loadScene(divergentScene);
  const handB = norm(b.getWorldState().deckRoguelike).hand;
  assert.notDeepEqual(handB, ['strike', 'strike', 'bash', 'defend', 'defend'], 'a different seed shuffles differently');
  const digestB = b.replayStateDigest('deck-frame');
  assert.notEqual(digestA.digest.value, digestB.digest.value, 'different seeds diverge in the digest');
  const diverged = b.compareReplayDigest(digestA, 'deck-frame');
  assert.equal(diverged.status, 'diverged');
}

// --- Malformed deck fails closed at load --------------------------------------
{
  const malformed = readScene('deck-roguelike-invalid-malformed-deck.json');
  const api = createRuntime();
  assert.throws(
    () => api.loadScene(malformed),
    /deck references undeclared card "phantom"/,
    'a deck citing an undeclared card must fail closed',
  );
}

console.log('deck-roguelike.test.cjs: all deck-roguelike game class cases passed');
