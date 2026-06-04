const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const renderer = require(path.join(runtimeDir, 'renderer.js'));
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function createRuntime(scene) {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '' },
    document: { getElementById: () => null },
    fetch: () => Promise.resolve({ json: () => Promise.resolve(scene) }),
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

function assertNoGeneratedFixtureState() {
  for (const name of ['runs', 'target', 'dashboard-data']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
  }
}

function rendererCameraRegression() {
  const activeRenderer = renderer.normalizeRenderer({
    viewport: { width: 160, height: 90 },
    camera: { x: 80, y: 30 },
    layers: [
      { id: 'background', order: -10, parallaxFactor: 50, visible: true },
      { id: 'actors', order: 0, parallaxFactor: 100, visible: true },
      { id: 'hud', order: 10, parallaxFactor: 100, visible: true, cameraParticipation: false },
    ],
    debug: { showBounds: true, showCamera: true, showEntityIds: true },
  });
  const world = { sceneId: 'renderer-camera-regression' };
  const entities = [
    { id: 'sky', sprite: { layer: 'background', order: 0, color: '#88f' }, components: { transform: { x: 0, y: 0 }, size: { width: 16, height: 16 } } },
    { id: 'player', sprite: { layer: 'actors', order: 0, color: '#fff' }, components: { transform: { x: 80, y: 30 }, size: { width: 16, height: 16 } } },
    { id: 'hud', sprite: { layer: 'hud', order: 0, color: '#fff' }, components: { transform: { x: 8, y: 8 }, size: { width: 16, height: 16 } } },
  ];
  const queue = renderer.renderQueue({ world: { ...world, entities }, renderer: activeRenderer });
  assert.equal(queue.validation.status, 'ready');
  assert.deepEqual(queue.renderables.filter((item) => item.sourceKind === 'entity').map((item) => item.sourceId), ['sky', 'player', 'hud']);
  assert.equal(renderer.cameraOffsetForLayer(activeRenderer, 'background').x, 40);
  assert.equal(renderer.cameraOffsetForLayer(activeRenderer, 'actors').x, 80);
  assert.equal(renderer.cameraOffsetForLayer(activeRenderer, 'hud').x, 0);
  assert.ok(queue.renderables.some((item) => item.sourceKind === 'debug-overlay' && item.primitiveKind === 'debug_camera'));
}

async function collisionInputSaveReplayRegression() {
  const collectScene = readJson('examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json');
  const collectApi = createRuntime(collectScene);
  await collectApi.whenReady();
  const startSave = collectApi.createSave('regression-start');
  collectApi.setInput({ right: true });
  collectApi.step(40);
  const keyEvents = collectApi.getEvents();
  const afterExit = collectApi.step(45);
  const events = collectApi.getEvents();
  assert.equal(afterExit.componentModel.goalFlags.key_collected, true);
  assert.equal(afterExit.componentModel.goalFlags.exit_reached, true);
  assert.ok(keyEvents.some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'collect_key'));
  assert.ok(events.some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'enter_exit'));
  let restored = collectApi.loadSave(startSave);
  assert.equal(restored.componentModel.goalFlags.key_collected, false);
  assert.equal(restored.componentModel.goalFlags.exit_reached, false);

  const actionScene = readJson('examples/game-runtime/action-map-v1.json');
  const actionApi = createRuntime(actionScene);
  let state = actionApi.loadScene(actionScene);
  assert.equal(state.sceneId, 'action-map-v1-fixture');
  actionApi.setInput({ keys: { d: true }, actions: { interact: true } });
  state = actionApi.step(2);
  assert.equal(state.actionState.move_right, true);
  assert.equal(state.actionState.interact, true);
  const snapshot = actionApi.snapshot();
  const digest = actionApi.replayStateDigest('input-regression');
  actionApi.setInput({ keys: { d: false, a: true }, actions: { interact: false } });
  state = actionApi.step(3);
  const divergence = actionApi.compareReplayDigest(digest, 'input-regression-after');
  assert.equal(divergence.status, 'diverged');
  restored = actionApi.restore(snapshot.snapshotId);
  assert.equal(restored.actionState.move_right, true);
  assert.equal(restored.actionState.interact, true);
  assert.ok(actionApi.getEvents().some((event) => event.type === 'runtime.snapshot.restored'));
}

(async () => {
  rendererCameraRegression();
  await collisionInputSaveReplayRegression();
  assertNoGeneratedFixtureState();
  console.log('production 2d runtime feature regressions passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
