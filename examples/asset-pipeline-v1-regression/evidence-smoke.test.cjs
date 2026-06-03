const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scenePath = path.join(fixtureDir, 'scenes', 'asset-pipeline-regression.scene.json');
const scenarioPackPath = path.join(fixtureDir, 'scenarios', 'asset-pipeline-v1-regression.json');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

class LoadingImage {
  constructor() {
    this.naturalWidth = 64;
    this.naturalHeight = 32;
  }

  set src(value) {
    this._src = value;
    if (this.onload) this.onload();
  }

  get src() {
    return this._src;
  }
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function createRuntime(scene) {
  const context = {
    console,
    Image: LoadingImage,
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

function countAt(source, dottedPath) {
  const value = pathValue(source, dottedPath);
  if (Array.isArray(value)) return value.length;
  if (value && typeof value === 'object') return Object.keys(value).length;
  return 0;
}

function assertScenarioAssertion(evidence, assertion) {
  const [target, contract] = Object.entries(assertion)[0];
  const actual = pathValue(evidence[target], contract.path);
  if (Object.prototype.hasOwnProperty.call(contract, 'equals')) {
    assert.deepEqual(actual, contract.equals, `${target}.${contract.path}`);
  }
  if (contract.exists === true) assert.notEqual(actual, undefined, `${target}.${contract.path} exists`);
  if (Object.prototype.hasOwnProperty.call(contract, 'countGreaterThan')) {
    assert.ok(countAt(evidence[target], contract.path) > contract.countGreaterThan, `${target}.${contract.path} count > ${contract.countGreaterThan}`);
  }
}

function generatedStateAudit() {
  for (const name of ['runs', 'target', 'dashboard-data']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay generated/untracked`);
  }
}

(async () => {
  const scene = readJson(scenePath);
  const scenarioPack = readJson(scenarioPackPath);
  const api = createRuntime(scene);
  await api.whenReady();
  const worldState = api.step(2);
  const runtimeEvents = api.getEvents();
  const frameStats = api.getFrameStats();
  const evidence = {
    world_state: worldState,
    frame_stats: frameStats,
    runtime_events: { events: runtimeEvents },
    animation_evidence: worldState.entities
      .filter((entity) => entity.components && entity.components.animation)
      .map((entity) => ({ entityId: entity.id, ...entity.components.animation })),
    audio_evidence: worldState.audioEvents,
    asset_loading: worldState.assets,
    tilemap_evidence: worldState.tilemaps,
  };

  assert.equal(worldState.sceneId, 'asset-pipeline-regression-scene');
  assert.equal(worldState.assetManifest.id, 'asset-pipeline-regression-runtime-assets');
  assert.equal(worldState.assetManifest.assetCount, 2);
  assert.equal(worldState.assetManifest.errors.length, 0);
  assert.ok(worldState.assets.some((asset) => asset.id === 'asset_regression_sheet' && asset.status === 'loaded'));
  assert.ok(worldState.audioEvents.some((event) => event.name === 'asset_probe_spawn' && event.asset === 'asset_regression_audio'));
  assert.equal(worldState.tilemaps.tilemaps[0].authoring.triggerCells[0].trigger, 'missing_asset_observed');
  assert.ok(worldState.tilemaps.tilemaps[0].authoring.hazardCells.some((cell) => cell.tileId === 'hazard'));

  const scenarioResults = [];
  for (const group of scenarioPack.scenarioGroups) {
    for (const scenario of group.scenarios) {
      for (const assertion of scenario.assertions || []) assertScenarioAssertion(evidence, assertion);
      scenarioResults.push({
        scenarioId: scenario.id,
        status: 'passed',
        evidenceRefs: [
          `evidence/scenarios/${scenario.id}/world-state.json`,
          `evidence/scenarios/${scenario.id}/scenario-result.json`,
        ],
        verdictRef: `evidence/scenarios/${scenario.id}/verdict.json`,
      });
    }
  }

  assert.deepEqual(
    scenarioResults.map((result) => result.scenarioId),
    [
      'manifest-validation',
      'hash-mismatch-regression',
      'missing-asset-regression',
      'atlas-frame-validation',
      'tile-collision-extraction',
      'runtime-asset-load-evidence',
      'studio-read-model-compatibility',
    ],
  );

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-asset-pipeline-evidence-'));
  const summaryPath = path.join(tempDir, 'asset-pipeline-v1-regression-evidence.json');
  fs.writeFileSync(summaryPath, JSON.stringify({
    schemaVersion: 'asset-pipeline-v1-regression-smoke-v1',
    scenarioPackId: scenarioPack.id,
    scenarioResults,
    generatedBoundary: 'temporary evidence only; repository runs/dashboard/previews remain untracked',
  }, null, 2));
  const summary = readJson(summaryPath);
  assert.equal(summary.scenarioPackId, 'asset-pipeline-v1-regression');
  assert.equal(summary.scenarioResults.length, 7);
  assert.ok(summary.scenarioResults.every((result) => result.status === 'passed'));
  assert.ok(summary.scenarioResults.every((result) => result.evidenceRefs.length >= 2 && result.verdictRef.endsWith('verdict.json')));
  fs.rmSync(tempDir, { recursive: true, force: true });
  assert.equal(fs.existsSync(tempDir), false, 'temporary evidence directory removed');
  generatedStateAudit();

  console.log('asset pipeline v1 regression evidence smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
