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
    fetch: () => Promise.reject(new Error('fetch disabled in replay digest test')),
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

function runDeterministicReplay() {
  const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'action-map-v1.json'), 'utf8'));
  const api = createRuntime();
  api.loadScene(scene);
  api.setInput({ keys: { d: true }, actions: { interact: true } });
  api.step(2);
  return api;
}

const first = runDeterministicReplay();
const expected = first.replayStateDigest('frame-2');
assert.equal(expected.schemaVersion, 'runtime-replay-digest-v1');
assert.equal(expected.policy.browserWriteAccess, 'none');
assert.equal(expected.policy.rootKind, 'generated_evidence');
assert.match(expected.digest.value, /^[0-9a-f]{16}$/);

const second = runDeterministicReplay();
const matched = second.compareReplayDigest(expected, 'frame-2');
assert.equal(matched.schemaVersion, 'runtime-replay-divergence-v1');
assert.equal(matched.status, 'matched');
assert.equal(matched.firstDivergence, null);
assert.equal(matched.actual.value, expected.digest.value);

second.setInput({ keys: { d: false, a: true }, actions: { interact: false } });
second.step(1);
const diverged = second.compareReplayDigest(expected.digest, 'frame-3');
assert.equal(diverged.status, 'diverged');
assert.equal(diverged.firstDivergence.frameId, 'frame-3');
assert.notEqual(diverged.actual.value, expected.digest.value);
assert.ok(second.getEvents().some((event) => event.type === 'runtime.replay.digest_compared'));
const worldState = second.getWorldState();
assert.equal(worldState.runtimeState.schemaVersion, 'runtime-state-read-model-v1');
assert.match(worldState.runtimeState.digest.value, /^[0-9a-f]{16}$/);
assert.ok(worldState.runtimeEvents.some((event) => event.type === 'runtime.replay.digest_compared'));

assert.throws(() => second.replayStateDigest('../source'), /path-safe generated evidence file stem/);
assert.throws(() => second.compareReplayDigest({ algorithm: 'bad', value: '123' }), /expected digest/);

console.log('replay digest runtime smoke test passed');
