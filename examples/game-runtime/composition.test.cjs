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
    fetch: () => Promise.reject(new Error('fetch disabled in composition test')),
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

const scene = {
  schemaVersion: '1',
  id: 'composition-runtime',
  bounds: { width: 320, height: 180 },
  componentDefaults: {
    velocity: { x: 0, y: 0 },
    size: { width: 10, height: 12 },
    controllable: false,
  },
  entities: [
    {
      id: 'ship',
      sprite: { color: '#5eead4' },
      components: {
        transform: { x: 100, y: 50 },
        velocity: { x: 0, y: 0 },
        size: { width: 32, height: 16 },
        controllable: true,
      },
    },
    {
      id: 'turret',
      parent: 'ship',
      sprite: { color: '#facc15' },
      components: { transform: { x: 8, y: -4 } },
    },
    {
      id: 'muzzle',
      parent: 'turret',
      sprite: { color: '#f97316' },
      components: { transform: { x: 3, y: 1 } },
    },
  ],
};

const api = createRuntime();
const state = api.loadScene(scene);
const ship = state.entities.find((entity) => entity.id === 'ship');
const turret = state.entities.find((entity) => entity.id === 'turret');
const muzzle = state.entities.find((entity) => entity.id === 'muzzle');

assert.equal(ship.components.transform.x, 100);
assert.equal(ship.components.transform.y, 50);
assert.equal(turret.components.transform.x, 108);
assert.equal(turret.components.transform.y, 46);
assert.equal(muzzle.components.transform.x, 111);
assert.equal(muzzle.components.transform.y, 47);
assert.equal(turret.components.localTransform.x, 8);
assert.equal(turret.components.localTransform.y, -4);
assert.equal(muzzle.composition.worldTransform.x, 111);
assert.equal(muzzle.composition.worldTransform.y, 47);
assert.equal(turret.components.size.width, 10);
assert.equal(turret.components.size.height, 12);
assert.equal(turret.components.controllable, false);
assert.equal(state.componentDefaults.size.width, 10);


assert.equal(state.composition.version, '1');
const turretProbe = state.composition.entities.find((entity) => entity.entityId === 'turret');
const muzzleProbe = state.composition.entities.find((entity) => entity.entityId === 'muzzle');
assert.equal(turretProbe.parent, 'ship');
assert.equal(turretProbe.localTransform.x, 8);
assert.equal(turretProbe.localTransform.y, -4);
assert.equal(turretProbe.worldTransform.x, 108);
assert.equal(turretProbe.worldTransform.y, 46);
assert.equal(muzzleProbe.parent, 'turret');
assert.equal(muzzleProbe.worldTransform.x, 111);
assert.equal(muzzleProbe.worldTransform.y, 47);
