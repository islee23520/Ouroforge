const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json' },
    document: { getElementById: () => null },
    fetch: (scenePath) => {
      assert.equal(scenePath, '/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json');
      const absolute = path.join(repoRoot, scenePath.replace(/^\//, ''));
      return Promise.resolve({ json: () => Promise.resolve(JSON.parse(fs.readFileSync(absolute, 'utf8'))) });
    },
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

(async () => {
  const api = createRuntime();
  let state = await api.whenReady();

  assert.equal(state.sceneId, 'collect-and-exit-scene');
  assert.equal(state.componentModel.counts.hudValue, 3);
  assert.equal(state.componentModel.goalFlags.key_collected, false);
  assert.equal(state.componentModel.goalFlags.door_open, undefined);
  assert.equal(state.componentModel.goalFlags.exit_reached, false);
  assert.equal(state.audioEvents[0].name, 'player_spawn');
  assert.equal(state.entities.find((entity) => entity.id === 'player').components.animation.mode, 'sprite_frame');

  api.setInput({ right: true });
  state = api.step(40);
  assert.equal(state.componentModel.goalFlags.key_collected, true);
  assert.equal(state.componentModel.goalFlags.door_open, true);
  assert.equal(state.entities.find((entity) => entity.id === 'key').sprite.visible, false);
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'collect_key'));

  state = api.step(45);
  assert.equal(state.componentModel.goalFlags.exit_reached, true);
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'enter_exit'));
  assert.equal(state.physics.grounded.player, true);

  console.log('playable demo v2 runtime smoke test passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
