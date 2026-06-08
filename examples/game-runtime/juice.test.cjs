const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'juice.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in juice test')),
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

function stable(value) {
  return JSON.parse(JSON.stringify(value));
}

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'juice-scene-v1.json'), 'utf8'));
const api = createRuntime();
let state = api.loadScene(scene);

assert.equal(state.juice.schemaVersion, 'ouroforge.runtime-juice-probe.v1');
assert.equal(state.juice.primitiveCount, 4);
assert.equal(state.juice.emittedCount, 2, 'scene_loaded emits tween and sfx primitives deterministically');
assert.equal(state.juice.activeCount, 2);
assert.ok(state.juice.boundary.includes('feel/fun judgment remains human'));
assert.ok(state.juice.readOnlyInspection.disallowedActions.includes('trusted writes'));

const spawnPop = state.juice.emitted.find((event) => event.primitiveId === 'spawn-pop');
assert.equal(spawnPop.kind, 'tween');
assert.equal(spawnPop.sample.property, 'scale');
assert.equal(spawnPop.sample.value, 0.85);
assert.equal(spawnPop.boundary.includes('not a fun/quality verdict'), true);

const sfx = state.juice.emitted.find((event) => event.primitiveId === 'pickup-sfx');
assert.equal(sfx.kind, 'sfx');
assert.deepEqual(stable(sfx.sample.audioIntent), {
  name: 'pickup',
  action: 'play',
  kind: 'ui',
  bus: 'ui',
  asset: 'assets/audio/spawn.ogg',
});
assert.ok(state.audioEvents.some((event) => event.sourceFeedbackId === sfx.feedbackId && event.playback === 'intent'));

const beforeStepDigest = state.runtimeState.digest;
state = api.step(1);
assert.notEqual(state.runtimeState.digest, beforeStepDigest);
const tickShake = state.juice.emitted.find((event) => event.primitiveId === 'tick-shake' && event.tick === 1);
assert.ok(tickShake, 'tick trigger emits shake feedback');
assert.deepEqual(stable(tickShake.sample.offset), { x: 3, y: -3 });
assert.equal(state.runtimeFrameBudget.counts.juiceEventCount, state.juice.emittedCount);
assert.ok(api.getEvents().some((event) => event.type === 'runtime.juice.feedback' && event.payload.primitiveId === 'tick-shake'));

const snapshot = api.snapshot();
api.step(3);
const diverged = stable(api.getWorldState().juice);
api.restore(snapshot.snapshotId);
const restored = stable(api.getWorldState().juice);
assert.notDeepEqual(diverged, restored, 'snapshot restore returns to the saved juice state');
const replayed = stable(api.step(3).juice);
api.restore(snapshot.snapshotId);
const replayedAgain = stable(api.step(3).juice);
assert.deepEqual(replayedAgain, replayed, 'juice feedback snapshot/restore parity is deterministic');

const second = createRuntime();
second.loadScene(scene);
second.step(1);
assert.deepEqual(
  stable(second.getWorldState().juice.emitted.map((event) => ({ primitiveId: event.primitiveId, kind: event.kind, tick: event.tick, sample: event.sample }))),
  stable(state.juice.emitted.map((event) => ({ primitiveId: event.primitiveId, kind: event.kind, tick: event.tick, sample: event.sample }))),
  'fresh runtimes emit identical feedback evidence for the same scene and steps',
);

console.log('juice primitives runtime smoke test passed');
