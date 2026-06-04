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
  assert.equal(state.audioEvents[0].kind, 'audio_request');
  assert.equal(state.audioEvents[0].requestId, 'audio-0-1');
  assert.equal(state.audioEvents[0].sceneId, state.sceneId);
  assert.equal(state.audioEvents[0].name, 'player_spawn');
  assert.equal(state.audioEvents[0].asset, 'collect_sound');
  assert.equal(state.entities.find((entity) => entity.id === 'player').components.animation.mode, 'sprite_frame');
  assert.equal(state.assetManifest.id, 'collect-and-exit-runtime-assets');
  assert.equal(state.assetManifest.assetCount, 2);
  assert.equal(state.renderBreakdown.schemaVersion, 'ouroforge.scene-render-breakdown.v1');
  assert.equal(state.renderBreakdown.frameId, 'tick-0');
  assert.equal(state.renderBreakdown.sceneId, state.sceneId);
  assert.ok(state.renderBreakdown.elements.some((element) => element.entityId === 'player'));
  assert.ok(Array.isArray(state.renderBreakdown.absenceDiagnostics));
  assert.deepEqual(Array.from(state.renderBreakdown.readOnlyInspection.disallowedActions), ['trusted writes', 'command bridge', 'live mutation']);
  assert.equal(api.getFrameStats().renderBreakdownFrameId, state.renderBreakdown.frameId);
  assert.equal(state.assetManifest.errors.length, 0);
  assert.ok(state.assets.some((asset) => asset.id === 'collect_and_exit_sheet' && asset.path === 'assets/sprites/collect-and-exit-sheet.png'));
  assert.equal(state.tilemaps.tilemaps[0].id, 'collect_and_exit_level');
  assert.equal(state.tilemaps.tilemaps[0].grid.width, 10);
  assert.equal(state.tilemaps.tilemaps[0].authoring.triggerCells[0].trigger, 'key_collected');
  assert.equal(state.tilemaps.tilemaps[0].authoring.goalCells[0].tileId, 'exit_marker');

  api.setInput({ right: true });
  state = api.step(40);
  assert.equal(state.componentModel.goalFlags.key_collected, true);
  assert.equal(state.componentModel.goalFlags.door_open, true);
  assert.equal(state.entities.find((entity) => entity.id === 'key').sprite.visible, false);
  assert.equal(state.renderBreakdown.frameId, 'tick-40');
  assert.ok(state.renderBreakdown.absenceDiagnostics.some((diag) => diag.entityId === 'key' && diag.reason === 'hidden'));
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
