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
    fetch: () => Promise.reject(new Error('fetch disabled in scene3d collision test')),
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

const api = createRuntime();
const state = api.loadScene({
  schemaVersion: '1',
  id: 'runtime-scene3d-collision-smoke',
  sceneKind: '3d',
  bounds: { width: 320, height: 180 },
  entities: [],
  scene3d: {
    version: '1',
    colliders: [
      { id: 'player-box', shape: 'box', body: 'dynamic', size: { x: 2, y: 2, z: 2 }, collisionGroup: 'actors', collisionMask: ['triggers'] },
      { id: 'goal-trigger', shape: 'box', body: 'trigger', trigger: true, size: { x: 2, y: 2, z: 2 }, collisionGroup: 'triggers' },
      { id: 'invalid-box', shape: 'capsule', body: 'static', size: { x: 1, y: 1, z: 1 } },
    ],
    nodes: [
      { id: 'player', colliderRef: 'player-box', localTransform: { translation: { x: 0, y: 0, z: 0 } } },
      { id: 'goal', colliderRef: 'goal-trigger', localTransform: { translation: { x: 1, y: 0, z: 0 } } },
      { id: 'invalid', colliderRef: 'invalid-box', localTransform: { translation: { x: 8, y: 0, z: 0 } } },
    ],
  },
});

assert.equal(state.scene3dCollision.present, true);
assert.equal(state.scene3dCollision.triggerCount, 1);
assert.equal(state.scene3dCollision.invalidColliderCount, 1);
assert.equal(state.scene3dCollisions[0].type, 'runtime.scene3d.collision.trigger');
assert.ok(state.collisions.some((event) => event.type === 'runtime.scene3d.collision.trigger'));

const stepped = api.step();
assert.equal(stepped.scene3dCollision.triggerCount, 1);
assert.ok(stepped.collisions.some((event) => event.type === 'runtime.scene3d.collision.trigger'));
assert.ok(api.getEvents().some((event) => event.type === 'runtime.scene3d.collision.trigger'));
assert.ok(stepped.collisionEvents.some((event) => event.type === 'runtime.scene3d.collision.trigger'));

console.log('scene3d collision runtime smoke test passed');
