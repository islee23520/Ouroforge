const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function plain(value) {
  return JSON.parse(JSON.stringify(value));
}

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in camera test')),
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

const cameraScene = {
  schemaVersion: '1',
  id: 'camera-runtime-follow-test',
  bounds: { width: 640, height: 360 },
  activeCameraId: 'follow-player',
  cameras: [{
    id: 'follow-player',
    followTarget: 'player',
    position: { x: 0, y: 0 },
    viewport: { width: 160, height: 90 },
    clampBounds: { x: 0, y: 0, width: 240, height: 120 },
    deadZone: { width: 32, height: 24 },
    zoom: 100,
  }],
  renderer: {
    version: '1',
    camera: { x: 0, y: 0 },
    viewport: { width: 160, height: 90 },
    layers: [
      { id: 'sky', order: -10, parallaxFactor: 50 },
      { id: 'actors', order: 0 },
      { id: 'hud', order: 10, cameraParticipation: false },
    ],
  },
  entities: [
    {
      id: 'player',
      sprite: { color: '#5eead4', layer: 'actors' },
      components: {
        transform: { x: 220, y: 96 },
        velocity: { x: 0, y: 0 },
        size: { width: 16, height: 16 },
        controllable: false,
      },
    },
    {
      id: 'cloud',
      sprite: { color: '#94a3b8', layer: 'sky' },
      components: { transform: { x: 100, y: 20 }, size: { width: 32, height: 16 } },
    },
    {
      id: 'coin_counter',
      sprite: { color: '#ffffff', layer: 'hud' },
      components: { transform: { x: 12, y: 8 }, size: { width: 64, height: 12 } },
    },
  ],
};

const api = createRuntime();
let state = api.loadScene(cameraScene);

assert.equal(state.activeCameraId, 'follow-player');
assert.deepEqual(plain(state.renderer.camera), { x: 80, y: 30 }, 'camera follows target and clamps to configured bounds');
assert.deepEqual(plain(state.renderer.viewport), { width: 160, height: 90 });
assert.deepEqual(plain(state.cameras[0].position), { x: 80, y: 30 });
assert.equal(state.cameras[0].followTarget, 'player');
assert.equal(state.cameras[0].clampBounds.width, 240);

assert.deepEqual(
  plain(state.renderBreakdown.elements.find((element) => element.entityId === 'player').transform),
  { x: 220, y: 96 },
  'render breakdown preserves world position while world-to-screen remains renderer-owned',
);
assert.deepEqual(
  plain(state.renderer.layers.find((layer) => layer.id === 'sky')),
  { id: 'sky', order: -10, visible: true, parallaxFactor: 50, cameraParticipation: true },
);

assert.deepEqual(
  plain(api.getWorldState().renderQueue.layers).map((layer) => [layer.id, layer.order]),
  [['sky', -10], ['actors', 0], ['hud', 10]],
);

const rendererApi = require('./renderer.js');
assert.deepEqual(plain(rendererApi.cameraOffsetForLayer(state.renderer, 'actors')), { x: 80, y: 30 });
assert.deepEqual(plain(rendererApi.cameraOffsetForLayer(state.renderer, 'sky')), { x: 40, y: 15 });
assert.deepEqual(plain(rendererApi.cameraOffsetForLayer(state.renderer, 'hud')), { x: 0, y: 0 });
assert.deepEqual(plain(rendererApi.worldToScreen({ x: 220, y: 96 }, state.renderer, 'actors')), {
  x: 140,
  y: 66,
  layer: 'actors',
  cameraOffset: { x: 80, y: 30 },
});
assert.deepEqual(plain(rendererApi.worldToScreen({ x: 100, y: 20 }, state.renderer, 'sky')), {
  x: 60,
  y: 5,
  layer: 'sky',
  cameraOffset: { x: 40, y: 15 },
});
assert.deepEqual(plain(rendererApi.worldToScreen({ x: 12, y: 8 }, state.renderer, 'hud')), {
  x: 12,
  y: 8,
  layer: 'hud',
  cameraOffset: { x: 0, y: 0 },
});

const unclampedScene = JSON.parse(JSON.stringify(cameraScene));
unclampedScene.id = 'camera-runtime-unclamped-follow-test';
unclampedScene.cameras[0].clampBounds = { x: 0, y: 0, width: 640, height: 360 };
unclampedScene.entities[0].components.transform = { x: 184, y: 82 };
state = api.loadScene(unclampedScene);
assert.deepEqual(plain(state.renderer.camera), { x: 112, y: 45 }, 'unclamped camera centers on the followed target');

console.log('camera runtime follow and transform test passed');
