const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scenePath = path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json');
const scenarioPackPath = path.join(fixtureDir, 'scenarios', 'collect-and-exit.json');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
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

function pathValue(source, dottedPath) {
  return String(dottedPath || '')
    .split('.')
    .filter(Boolean)
    .reduce((value, segment) => {
      if (value === undefined || value === null) return undefined;
      if (/^\d+$/.test(segment)) return value[Number(segment)];
      return value[segment];
    }, source);
}

function assertScenarioAssertion(evidence, assertion) {
  const [target, contract] = Object.entries(assertion)[0];
  const actual = pathValue(evidence[target], contract.path);
  if (Object.prototype.hasOwnProperty.call(contract, 'equals')) {
    assert.deepEqual(actual, contract.equals, `${target}.${contract.path}`);
  }
  if (contract.exists === true) assert.notEqual(actual, undefined, `${target}.${contract.path} exists`);
  if (Object.prototype.hasOwnProperty.call(contract, 'greaterThan')) {
    assert.ok(actual > contract.greaterThan, `${target}.${contract.path} > ${contract.greaterThan}`);
  }
  if (Object.prototype.hasOwnProperty.call(contract, 'containsType')) {
    assert.ok(Array.isArray(actual), `${target}.${contract.path} is an array`);
    assert.ok(actual.some((entry) => entry && entry.type === contract.containsType), `${target}.${contract.path} contains ${contract.containsType}`);
  }
}

function generatedStateAudit() {
  const generatedNames = ['runs', 'target', 'dashboard-data'];
  for (const name of generatedNames) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked/generated`);
  }
}

(async () => {
  const scene = readJson(scenePath);
  const scenarioPack = readJson(scenarioPackPath);
  const api = createRuntime(scene);
  await api.whenReady();
  const initial = api.getWorldState();
  const startSave = api.createSave('demo-start');

  api.setInput({ right: true });
  const afterKey = api.step(40);
  const afterKeyEvents = api.getEvents();
  const afterExit = api.step(45);
  api.setInput({ right: false });

  const runtimeEventsBeforeRestore = api.getEvents();
  const evidence = {
    world_state: afterExit,
    frame_stats: api.getFrameStats(),
    runtime_events: { events: runtimeEventsBeforeRestore },
    runtime_save: startSave,
    animation_evidence: afterExit.entities
      .filter((entity) => entity.components && entity.components.animation)
      .map((entity) => ({ entityId: entity.id, ...entity.components.animation })),
    audio_evidence: afterExit.audioEvents,
    asset_loading: afterExit.assets,
    tilemap_evidence: afterExit.tilemaps,
    comparison: {
      beforeSceneId: initial.sceneId,
      afterSceneId: afterExit.sceneId,
      changedFlags: Object.keys(afterExit.componentModel.goalFlags)
        .filter((flag) => initial.componentModel.goalFlags[flag] !== afterExit.componentModel.goalFlags[flag])
        .sort(),
    },
  };

  assert.equal(afterKey.componentModel.goalFlags.key_collected, true);
  assert.equal(afterKey.componentModel.goalFlags.door_open, true);
  assert.equal(afterExit.componentModel.goalFlags.exit_reached, true);
  assert.equal(initial.metadata.title, 'Collect and Exit Vertical Slice Demo');
  assert.equal(initial.metadata.startState.checkpointSlot, 'demo-start');
  assert.equal(startSave.schemaVersion, 'runtime-save-artifact-v1');
  assert.equal(startSave.slotId, 'demo-start');
  assert.equal(startSave.policy.browserWriteAccess, 'none');
  assert.equal(startSave.state.flags.key_collected, false);
  assert.equal(afterExit.assetManifest.id, 'collect-and-exit-runtime-assets');
  assert.equal(afterExit.assetManifest.assetCount, 2);
  assert.equal(afterExit.assetManifest.errors.length, 0);
  assert.ok(afterExit.assets.some((asset) => asset.id === 'collect_and_exit_sheet' && asset.path === 'assets/sprites/collect-and-exit-sheet.png'));
  assert.equal(afterExit.audioEvents[0].asset, 'collect_sound');
  assert.equal(afterExit.tilemaps.tilemaps[0].id, 'collect_and_exit_level');
  assert.equal(afterExit.tilemaps.tilemaps[0].grid.width, 10);
  assert.equal(afterExit.tilemaps.tilemaps[0].authoring.triggerCells[0].trigger, 'key_collected');
  assert.equal(afterExit.tilemaps.tilemaps[0].authoring.goalCells[0].tileId, 'exit_marker');
  assert.ok(afterKeyEvents.some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'collect_key'));
  assert.ok(runtimeEventsBeforeRestore.some((event) => event.type === 'runtime.trigger.entered' && event.payload.triggerId === 'enter_exit'));
  assert.deepEqual(evidence.comparison.changedFlags, ['door_open', 'exit_reached', 'key_collected']);
  const restored = api.loadSave(startSave);
  assert.equal(restored.componentModel.goalFlags.key_collected, false, 'checkpoint restore resets collected key');
  assert.equal(restored.componentModel.goalFlags.exit_reached, false, 'checkpoint restore resets exit flag');
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.save.loaded'));
  evidence.runtime_events = { events: api.getEvents() };

  for (const group of scenarioPack.scenarioGroups) {
    for (const scenario of group.scenarios) {
      for (const assertion of scenario.assertions || []) assertScenarioAssertion(evidence, assertion);
    }
  }

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-smoke-'));
  const tempEvidence = path.join(tempDir, 'evidence-smoke.json');
  fs.writeFileSync(tempEvidence, JSON.stringify(evidence, null, 2));
  assert.equal(readJson(tempEvidence).world_state.componentModel.goalFlags.exit_reached, true);
  fs.rmSync(tempDir, { recursive: true, force: true });
  generatedStateAudit();

  console.log('collect-and-exit e2e evidence smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
