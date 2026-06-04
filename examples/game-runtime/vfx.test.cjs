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
    fetch: () => Promise.reject(new Error('fetch disabled in vfx test')),
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

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'scene.json'), 'utf8'));
const player = scene.entities.find((entity) => entity.id === 'player');
player.components.vfx = {
  emitters: [
    { id: 'run-dust', kind: 'trail', trigger: 'tick', particleCount: 4, lifetimeFrames: 6, color: '#94a3b8', asset: 'player-sprite', layer: 'actors' },
    { id: 'disabled-spark', kind: 'spark', trigger: 'tick', disabled: true, particleCount: 3, lifetimeFrames: 5, color: '#facc15' },
    { id: 'clamped', kind: 'burst', trigger: 'tick', particleCount: 999, lifetimeFrames: 999, color: '#ffffff' },
  ],
};

const api = createRuntime();
let state = api.loadScene(scene);
assert.equal(state.vfxEvents.length, 0);
state = api.step(1);
assert.equal(state.vfxEvents.length, 2, 'enabled tick emitters produce bounded VFX events');
const dust = state.vfxEvents.find((event) => event.emitterId === 'run-dust');
assert.equal(dust.schemaVersion, 'runtime-vfx-event-v1');
assert.equal(dust.particleCount, 4);
assert.equal(dust.expiresAtTick, state.tick + 6);
assert.equal(dust.readOnlyEvidence, true);
assert.equal(state.vfxEvents.some((event) => event.emitterId === 'disabled-spark'), false);
const clamped = state.vfxEvents.find((event) => event.emitterId === 'clamped');
assert.equal(clamped.particleCount, 64);
assert.equal(clamped.lifetimeFrames, 120);
assert.ok(api.getEvents().some((event) => event.type === 'runtime.vfx.emitted' && event.payload.emitterId === 'run-dust'));
assert.equal(state.componentModel.counts.vfx, 1);

console.log('vfx runtime smoke test passed');
