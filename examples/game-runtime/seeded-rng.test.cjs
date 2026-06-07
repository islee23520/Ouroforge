'use strict';

// Seeded stochastic determinism runtime test (Era F Milestone 31, #1600).
// Validates, against the existing runtime replay-digest and snapshot/restore
// surfaces, that: identical seeds produce identical runs (digest-stable),
// different seeds diverge detectably, and snapshot/restore survives RNG draws.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in seeded rng test')),
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

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'action-map-v1.json'), 'utf8'));
const cases = JSON.parse(
  fs.readFileSync(path.join(runtimeDir, '..', 'seeded-rng-v1', 'seeded-rng-cases.json'), 'utf8'),
).cases;

function runWithSeed(seed, draws) {
  const api = createRuntime();
  api.loadScene(scene);
  api.seedRng(seed);
  const sequence = [];
  for (let i = 0; i < draws; i += 1) sequence.push(api.nextRandom().raw);
  const digest = api.replayStateDigest('seeded-frame');
  return { api, sequence, digest };
}

// --- Determinism guardrails ---------------------------------------------------
{
  const first = runWithSeed(cases.sameSeed.seedA, 1).api.rngState();
  assert.equal(first.schemaVersion, 'runtime-seeded-rng-v1');
  assert.equal(first.algorithm, 'mulberry32');
  // loadScene seeds from scene.seed (default 0) before the explicit seedRng call.
  const fresh = createRuntime();
  fresh.loadScene(scene);
  assert.equal(fresh.rngState().seed, 0, 'unseeded scene defaults to seed 0, never wall-clock');
}

// --- Case 1: same-seed parity (digest-stable, identical draw sequence) ---------
{
  const a = runWithSeed(cases.sameSeed.seedA, cases.sameSeed.draws);
  const b = runWithSeed(cases.sameSeed.seedB, cases.sameSeed.draws);
  assert.deepEqual(a.sequence, b.sequence, 'same seed yields identical draw sequence');
  assert.equal(a.digest.schemaVersion, 'runtime-replay-digest-v1');
  assert.match(a.digest.digest.value, /^[0-9a-f]{16}$/);
  assert.equal(a.digest.digest.value, b.digest.digest.value, 'same seed yields a digest-stable run');
  // compareReplayDigest confirms the parity through the existing divergence surface.
  const matched = b.api.compareReplayDigest(a.digest, 'seeded-frame');
  assert.equal(matched.schemaVersion, 'runtime-replay-divergence-v1');
  assert.equal(matched.status, 'matched');
  assert.equal(matched.firstDivergence, null);
}

// --- Case 2: different-seed divergence (detected via the replay digest) --------
{
  const a = runWithSeed(cases.differentSeed.seedA, cases.differentSeed.draws);
  const b = runWithSeed(cases.differentSeed.seedB, cases.differentSeed.draws);
  assert.notDeepEqual(a.sequence, b.sequence, 'different seeds produce different draws');
  assert.notEqual(a.digest.digest.value, b.digest.digest.value, 'different seeds diverge in the digest');
  const diverged = b.api.compareReplayDigest(a.digest, 'seeded-frame');
  assert.equal(diverged.status, 'diverged');
  assert.equal(diverged.firstDivergence.frameId, 'seeded-frame');
  assert.notEqual(diverged.actual.value, a.digest.digest.value);
}

// --- Case 3: snapshot/restore across a draw -----------------------------------
{
  const { seed, drawsBeforeSnapshot, drawsAfter } = cases.snapshotAcrossDraw;
  const api = createRuntime();
  api.loadScene(scene);
  api.seedRng(seed);
  for (let i = 0; i < drawsBeforeSnapshot; i += 1) api.nextRandom();
  const captured = api.snapshot();
  const rngAtSnapshot = api.rngState();

  const continuation = [];
  for (let i = 0; i < drawsAfter; i += 1) continuation.push(api.nextRandom().raw);
  const digestAfter = api.replayStateDigest('seeded-after').digest.value;

  api.restore(captured.snapshotId);
  assert.deepEqual(api.rngState(), rngAtSnapshot, 'restore returns the RNG stream to the snapshot position');

  const replay = [];
  for (let i = 0; i < drawsAfter; i += 1) replay.push(api.nextRandom().raw);
  assert.deepEqual(replay, continuation, 'restored stream redraws the identical sequence');
  assert.equal(
    api.replayStateDigest('seeded-after').digest.value,
    digestAfter,
    'replay digest after restore matches the uninterrupted run',
  );
}

console.log('seeded-rng.test.cjs: all seeded determinism cases passed');
