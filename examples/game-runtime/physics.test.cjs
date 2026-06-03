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


function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function sceneWithPlayerInput(input) {
  const scene = clone(platformerScene);
  scene.entities[0].components.input = input;
  return scene;
}

function settleOnFloor(api, scene) {
  api.setInput({ left: false, right: false, up: false, down: false });
  let state = api.loadScene(scene);
  state = api.step(1);
  const player = state.entities.find((entity) => entity.id === 'player');
  assert.equal(player.components.transform.y, 64, 'solid floor resolves the one-frame gravity overlap');
  assert.equal(player.components.velocity.y, 0, 'ground contact zeroes downward velocity');
  assert.equal(state.physics.grounded.player, true);
  return state;
}

function assertGroundedJump(api, scene, message) {
  settleOnFloor(api, scene);
  api.setInput({ up: true });
  const state = api.step(1);
  const player = state.entities.find((entity) => entity.id === 'player');
  assert.equal(player.components.velocity.y, -5, message);
  assert.equal(player.components.transform.y, 59);
  assert.equal(state.physics.grounded.player, false);
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.physics.jump'));
  api.setInput({ up: false });
  return state;
}

const platformerScene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'physics-rules-v2.json'), 'utf8'));
const floorCollider = platformerScene.entities.find((entity) => entity.id === 'floor').components.collider;
const worldLayer = platformerScene.collisionRules.layers.find((layer) => layer.id === 'world');
assert.deepEqual(floorCollider.collisionMask, ['actors'], 'floor collider must not rely on wildcard masks');
assert.deepEqual(worldLayer.collidesWith, ['actors'], 'world layer declares actor contact for the floor fixture');


const defaultLayerScene = {
  schemaVersion: '1',
  id: 'physics-runtime-default-layer',
  bounds: { width: 64, height: 32 },
  collisionRules: { defaultLayer: 'terrain' },
  entities: [
    {
      id: 'player',
      sprite: { color: '#5eead4' },
      components: {
        transform: { x: 0, y: 0 },
        velocity: { x: 0, y: 0 },
        size: { width: 16, height: 16 },
        controllable: false,
        input: { scheme: 'keyboard', moveSpeed: 24, allowedActions: ['move'] },
        collider: {
          shape: 'aabb',
          body: 'dynamic',
          offset: { x: 0, y: 0 },
          size: { width: 16, height: 16 },
        },
      },
    },
    {
      id: 'wall',
      sprite: { color: '#475569' },
      components: {
        transform: { x: 20, y: 0 },
        velocity: { x: 0, y: 0 },
        size: { width: 16, height: 16 },
        controllable: false,
        collider: {
          shape: 'aabb',
          body: 'static',
          offset: { x: 0, y: 0 },
          size: { width: 16, height: 16 },
          collisionMask: ['terrain'],
        },
      },
    },
  ],
};

const explicitGroupScene = clone(defaultLayerScene);
explicitGroupScene.id = 'physics-runtime-explicit-layer';
explicitGroupScene.entities[0].components.collider.collisionGroup = 'actors';

const api = createRuntime();
let state = api.loadScene(platformerScene);
assert.equal(state.physics.gravity, 1);
assert.equal(state.physics.grounded.player, false);

state = settleOnFloor(api, platformerScene);
let player = state.entities.find((entity) => entity.id === 'player');
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

assertGroundedJump(
  api,
  sceneWithPlayerInput({ scheme: 'keyboard', moveSpeed: 3, jumpImpulse: 6 }),
  'omitted allowedActions permits jump when jumpImpulse is finite',
);

assertGroundedJump(
  api,
  sceneWithPlayerInput({ scheme: 'keyboard', moveSpeed: 3, jumpImpulse: 6, allowedActions: [] }),
  'empty allowedActions permits jump when jumpImpulse is finite',
);

settleOnFloor(
  api,
  sceneWithPlayerInput({ scheme: 'keyboard', moveSpeed: 3, jumpImpulse: 6, allowedActions: ['attack'] }),
);
const jumpEventCount = api.getEvents().filter((event) => event.type === 'runtime.physics.jump').length;
api.setInput({ up: true });
state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.velocity.y, 0, 'non-empty allowedActions without jump disables jump');
assert.equal(player.components.transform.y, 64);
assert.equal(state.physics.grounded.player, true);
assert.equal(api.getEvents().filter((event) => event.type === 'runtime.physics.jump').length, jumpEventCount);

api.setInput({ left: false, right: false, up: false, down: false });
state = api.loadScene(defaultLayerScene);
api.setInput({ right: true });
state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 4, 'ungrouped dynamic collider uses collisionRules.defaultLayer for masks');
assert.ok(state.collisions.some((event) => event.type === 'runtime.collision.contact' && event.pairId === 'player:wall'));
assert.equal(state.collisionRules.defaultLayer, 'terrain');

state = api.loadScene(explicitGroupScene);
api.setInput({ right: true });
state = api.step(1);
player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 24, 'explicit collisionGroup is not replaced by collisionRules.defaultLayer');
assert.equal(state.collisions.length, 0);
