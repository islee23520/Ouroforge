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
    fetch: () => Promise.reject(new Error('fetch disabled in physics test')),
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

const platformerScene = {
  schemaVersion: '1',
  id: 'physics-runtime-rules',
  bounds: { width: 160, height: 96 },
  entities: [
    {
      id: 'player',
      sprite: { color: '#5eead4' },
      components: {
        transform: { x: 16, y: 64 },
        velocity: { x: 0, y: 0 },
        size: { width: 16, height: 16 },
        controllable: true,
        input: { scheme: 'keyboard', moveSpeed: 3, jumpImpulse: 6, allowedActions: ['move', 'jump'] },
        collider: {
          shape: 'aabb',
          body: 'dynamic',
          offset: { x: 0, y: 0 },
          size: { width: 16, height: 16 },
          collisionGroup: 'actors',
          collisionMask: ['world'],
        },
      },
    },
    {
      id: 'floor',
      sprite: { color: '#475569' },
      components: {
        transform: { x: 0, y: 80 },
        velocity: { x: 0, y: 0 },
        size: { width: 160, height: 16 },
        controllable: false,
        collider: {
          shape: 'aabb',
          body: 'static',
          offset: { x: 0, y: 0 },
          size: { width: 160, height: 16 },
          collisionGroup: 'world',
        },
      },
    },
  ],
};

const api = createRuntime();
let state = api.loadScene(platformerScene);
assert.equal(state.physics.gravity, 1);
assert.equal(state.physics.grounded.player, false);

state = api.step(1);
let player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.y, 64, 'solid floor resolves the one-frame gravity overlap');
assert.equal(player.components.velocity.y, 0, 'ground contact zeroes downward velocity');
assert.equal(state.physics.grounded.player, true);
assert.ok(state.collisions.some((event) => event.type === 'runtime.collision.contact' && event.normal.y === -1));

api.setInput({ up: true });
state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.velocity.y, -5, 'jump impulse is applied then gravity advances deterministically');
assert.equal(player.components.transform.y, 59);
assert.equal(state.physics.grounded.player, false);
assert.ok(api.getEvents().some((event) => event.type === 'runtime.physics.jump'));

state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.velocity.y, -4);
assert.equal(player.components.transform.y, 55);
