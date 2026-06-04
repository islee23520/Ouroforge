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
    fetch: () => Promise.reject(new Error('fetch disabled in save/load test')),
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
const api = createRuntime();
let state = api.loadScene(scene);
assert.equal(state.sceneId, 'action-map-v1-fixture');

api.setInput({ keys: { d: true }, actions: { interact: true } });
state = api.step(2);
let player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 38);
assert.equal(state.actionState.move_right, true);
assert.equal(state.actionState.interact, true);

const snapshot = api.snapshot();
api.setInput({ keys: { d: false }, actions: { interact: false } });
state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 38);
state = api.restore(snapshot.snapshotId);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 38, 'snapshot restore preserves scoped entity state');
assert.equal(state.rawInput.keys.d, true, 'snapshot restore preserves raw key state');
assert.equal(state.actionState.interact, true, 'snapshot restore preserves action override state');

const save = api.createSave('slot-1');
assert.equal(save.schemaVersion, 'runtime-save-artifact-v1');
assert.equal(save.state.schemaVersion, 'runtime-state-v1');
assert.equal(save.policy.browserWriteAccess, 'none');
assert.equal(save.policy.trustedWriter, 'rust-local-runtime-save-v1');
assert.match(save.state.digest.value, /^[0-9a-f]{16}$/);

api.setInput({ keys: { d: false, a: true }, actions: { interact: false } });
state = api.step(3);
player = state.entities.find((entity) => entity.id === 'player');
assert.notEqual(player.components.transform.x, save.state.entities[0].transform.x);
state = api.loadSave(save);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, save.state.entities[0].transform.x, 'save load restores entity transform');
assert.equal(state.tick, save.state.tick, 'save load restores tick');
assert.equal(state.rawInput.keys.d, true, 'save load restores raw key state');
assert.equal(state.actionState.interact, true, 'save load restores action override state');
assert.ok(api.getEvents().some((event) => event.type === 'runtime.save.loaded'));
assert.throws(() => api.loadSave({ schemaVersion: 'bad' }), /runtime save artifact schemaVersion/);
assert.throws(() => api.createSave('../source'), /path-safe generated evidence file stem/);
assert.throws(() => api.loadSave('../source'), /path-safe generated evidence file stem/);

console.log('save/load runtime smoke test passed');
