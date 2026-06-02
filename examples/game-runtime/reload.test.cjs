const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js',
  'snapshot.js',
  'assets.js',
  'animation.js',
  'audio.js',
  'renderer.js',
  'tilemap.js',
  'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in reload test')),
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

const baseScene = {
  schemaVersion: '1',
  id: 'reload-before',
  bounds: { width: 320, height: 180 },
  assetManifest: {
    schemaVersion: '1',
    id: 'before-assets',
    assets: [{ id: 'before-sprite', kind: 'sprite', path: 'assets/sprites/player.svg' }],
  },
  entities: [{
    id: 'player',
    sprite: { color: '#5eead4', asset: 'before-sprite' },
    components: {
      transform: { x: 32, y: 72 },
      velocity: { x: 0, y: 0 },
      size: { width: 16, height: 16 },
      controllable: true,
    },
  }],
};

const reloadedScene = {
  schemaVersion: '1',
  id: 'reload-after',
  bounds: { width: 160, height: 90 },
  entities: [{
    id: 'avatar',
    sprite: { color: '#facc15', asset: 'after-sprite' },
    components: {
      transform: { x: 8, y: 12 },
      velocity: { x: 0, y: 0 },
      size: { width: 12, height: 10 },
      controllable: true,
    },
  }],
};

const api = createRuntime();
api.loadScene(baseScene);
api.setInput({ right: true });
api.step(3);
api.snapshot();
assert.equal(api.getWorldState().tick, 3);
assert.equal(api.getWorldState().snapshots.length, 1);

const state = api.reload({
  schemaVersion: 'ouroforge.scene-reload.v0',
  scene: reloadedScene,
  assetManifest: {
    schemaVersion: '1',
    id: 'after-assets',
    assets: [{ id: 'after-sprite', kind: 'sprite', path: 'assets/sprites/after.svg' }],
  },
});

assert.equal(state.sceneId, 'reload-after');
assert.equal(state.tick, 0);
assert.equal(state.bounds.width, 160);
assert.equal(state.entities.length, 1);
assert.equal(state.entities[0].id, 'avatar');
assert.equal(state.entities[0].components.transform.x, 8);
assert.equal(state.collisions.length, 0);
assert.equal(state.collisionEvents.length, 0);
assert.equal(state.audioEvents.length, 0);
assert.equal(state.snapshots.length, 0);
assert.equal(state.assetManifest.id, 'after-assets');
assert.equal(state.assetManifest.assetCount, 1);
assert.equal(state.assets[0].id, 'after-sprite');
assert.equal(api.getFrameStats().fixedDeltaMs, 16);

assert.throws(
  () => api.reload({ schemaVersion: 'wrong', scene: reloadedScene }),
  /reload payload schemaVersion must be ouroforge\.scene-reload\.v0/,
);
