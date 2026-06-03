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
    fetch: () => Promise.reject(new Error('fetch disabled in components v2 test')),
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

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'scene-components-v2.json'), 'utf8'));
const api = createRuntime();
let state = api.loadScene(scene);

const player = state.entities.find((entity) => entity.id === 'player');
const coin = state.entities.find((entity) => entity.id === 'coin');
const hud = state.entities.find((entity) => entity.id === 'hud_coin');

assert.equal(player.components.status.hitPoints, 3);
assert.equal(player.components.input.moveSpeed, 3);
assert.equal(player.components.cameraTarget.weight, 2);
assert.equal(coin.components.trigger.id, 'collect_coin');
assert.equal(coin.components.goalFlag.flag, 'coin_collected');
assert.equal(hud.components.uiText.text, 'Coin: 0/1');
assert.equal(state.componentDefaults.status.hitPoints, 1);
assert.equal(state.componentDefaults.input.moveSpeed, 2);
assert.deepEqual(JSON.parse(JSON.stringify(state.componentModel.counts)), {
  status: 3,
  input: 3,
  trigger: 1,
  goalFlag: 1,
  cameraTarget: 1,
  uiText: 1,
});
assert.equal(state.componentModel.goalFlags.alive, true);
assert.equal(state.componentModel.goalFlags.coin_collected, false);

const presetFlagScene = structuredClone(scene);
presetFlagScene.entities.push({
  id: 'gate_state',
  sprite: { color: '#ffffff', visible: false },
  components: {
    transform: { x: 0, y: 0 },
    velocity: { x: 0, y: 0 },
    size: { width: 1, height: 1 },
    goalFlag: { flag: 'gate_open', label: 'Gate open', value: true },
  },
});
const presetCoin = presetFlagScene.entities.find((entity) => entity.id === 'coin');
presetCoin.components.transform.x = 34;
presetCoin.components.collider = {
  shape: 'aabb',
  body: 'static',
  offset: { x: 0, y: 0 },
  size: { width: 12, height: 12 },
  sensor: false,
  trigger: true,
  collisionGroup: 'triggers',
};
presetCoin.components.trigger.requiredFlags = ['gate_open'];
const presetPlayer = presetFlagScene.entities.find((entity) => entity.id === 'player');
presetPlayer.components.collider.collisionMask = ['triggers'];
state = api.loadScene(presetFlagScene);
assert.equal(state.componentModel.goalFlags.gate_open, true);
const presetEventCount = api.getEvents().length;
state = api.step(1);
assert.equal(state.componentModel.goalFlags.coin_collected, true);
assert.equal(state.entities.find((entity) => entity.id === 'coin').sprite.visible, false);
assert.ok(api.getEvents().slice(presetEventCount).some((event) => event.type === 'runtime.trigger.entered'));

state = api.loadScene(scene);
assert.equal(state.componentModel.goalFlags.coin_collected, false);
api.setInput({ right: true });
state = api.step(1);
assert.equal(state.entities.find((entity) => entity.id === 'player').components.velocity.x, 3);

const triggerScene = structuredClone(scene);
const triggerCoin = triggerScene.entities.find((entity) => entity.id === 'coin');
triggerCoin.components.transform.x = 34;
triggerCoin.components.collider = {
  shape: 'aabb',
  body: 'static',
  offset: { x: 0, y: 0 },
  size: { width: 12, height: 12 },
  sensor: false,
  trigger: true,
  collisionGroup: 'triggers',
};
const triggerPlayer = triggerScene.entities.find((entity) => entity.id === 'player');
triggerPlayer.components.collider.collisionMask = ['triggers'];
state = api.loadScene(triggerScene);
const triggerEventCount = api.getEvents().length;
state = api.step(1);
assert.equal(state.componentModel.goalFlags.coin_collected, true);
assert.equal(state.entities.find((entity) => entity.id === 'coin').sprite.visible, false);
assert.ok(api.getEvents().slice(triggerEventCount).some((event) => event.type === 'runtime.trigger.entered'));
