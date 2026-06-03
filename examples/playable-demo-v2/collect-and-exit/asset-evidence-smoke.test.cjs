const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const dashboard = require(path.join(repoRoot, 'examples', 'evidence-dashboard', 'dashboard.js'));
const cockpit = require(path.join(repoRoot, 'examples', 'authoring-cockpit', 'cockpit.js'));
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

function previewFromManifest(projectManifest) {
  const records = projectManifest.assets.map((asset) => ({
    assetId: asset.id,
    assetType: asset.type,
    sourcePath: asset.path,
    previewKind: asset.type === 'image' || asset.type === 'sprite_atlas' ? 'thumbnail' : 'metadata',
    image: asset.dimensions || null,
    atlasFrames: (asset.atlas?.frames || []).map((frame) => ({ frameId: frame.id, rect: frame.rect })),
    tilemap: asset.tilemap ? {
      tilesetAssetId: asset.tilemap.tilesetAssetId,
      width: asset.tilemap.width,
      height: asset.tilemap.height,
      layerCount: asset.tilemap.layers.length,
      tileCount: asset.tilemap.layers.reduce((count, layer) => count + (Array.isArray(layer.data) ? layer.data.length : 0), 0),
    } : null,
    audio: asset.type === 'audio' ? { durationMs: asset.durationMs } : null,
  }));
  return {
    present: true,
    preview_count: records.length,
    warning_count: 0,
    atlas_frame_count: records.reduce((count, record) => count + record.atlasFrames.length, 0),
    tilemap_count: records.filter((record) => record.tilemap).length,
    audio_count: records.filter((record) => record.audio).length,
    image_count: records.filter((record) => record.image).length,
    font_count: 0,
    records,
    warnings: [],
    evidence_refs: ['temp-dashboard-data.json'],
    boundary: 'Read-only generated smoke metadata; browser surfaces display escaped local evidence only and never upload, write, fetch remote assets, or execute commands.',
    empty_state: '',
  };
}

function loadingFromWorldState(worldState) {
  const records = (worldState.assets || []).map((asset) => ({
    attemptId: asset.attemptId,
    assetId: asset.id,
    assetType: asset.kind,
    path: asset.path,
    status: asset.status,
    loadDurationMs: asset.loadDurationMs,
    failureReason: asset.failureReason,
    width: asset.width,
    height: asset.height,
  }));
  return {
    present: true,
    attempt_count: records.length,
    loaded_count: records.filter((record) => record.status === 'loaded').length,
    failed_count: records.filter((record) => record.status === 'failed').length,
    rejected_count: records.filter((record) => record.status === 'rejected').length,
    fallback_count: 0,
    records,
    evidence_refs: ['temp-runtime-world-state.json'],
    boundary: 'Read-only runtime loading evidence; browser surfaces display escaped local evidence only and never upload, write trusted state, or execute commands.',
    empty_state: '',
  };
}

function inspectorFrom(projectManifest, loading, preview) {
  const runtimeById = new Map();
  for (const record of loading.records) {
    if (!runtimeById.has(record.assetId)) runtimeById.set(record.assetId, []);
    runtimeById.get(record.assetId).push(record.status);
  }
  return {
    present: true,
    status: 'ok',
    asset_count: projectManifest.assets.length,
    preview_count: preview.preview_count,
    runtime_attempt_count: loading.attempt_count,
    loaded_count: loading.loaded_count,
    failed_count: loading.failed_count,
    warning_count: 0,
    atlas_frame_count: preview.atlas_frame_count,
    tilemap_count: preview.tilemap_count,
    evidence_refs: ['temp-dashboard-data.json'],
    boundary: 'Read-only Studio asset inspector data assembled from generated smoke evidence; browser surfaces must not upload, write, fetch remote assets, edit manifests, or execute commands.',
    empty_state: '',
    assets: projectManifest.assets.map((asset) => ({
      asset_id: asset.id,
      asset_type: asset.type,
      source_path: asset.path,
      content_hash: `${asset.contentHash.algorithm}:${asset.contentHash.value}`,
      runtime_statuses: runtimeById.get(asset.id) || [],
      atlas_frame_count: asset.atlas?.frames?.length || 0,
      tilemap: asset.tilemap ? {
        tilesetAssetId: asset.tilemap.tilesetAssetId,
        width: asset.tilemap.width,
        height: asset.tilemap.height,
        layerCount: asset.tilemap.layers.length,
        tileCount: asset.tilemap.layers.reduce((count, layer) => count + (Array.isArray(layer.data) ? layer.data.length : 0), 0),
      } : null,
      warnings: [],
    })),
  };
}

(async () => {
  const scene = readJson(path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json'));
  const projectManifest = readJson(path.join(fixtureDir, 'asset-manifest.json'));
  const api = createRuntime(scene);
  await api.whenReady();
  api.setInput({ right: true });
  api.step(40);
  const worldState = api.step(45);

  assert.equal(worldState.sceneId, 'collect-and-exit-scene');
  assert.equal(worldState.assetManifest.id, 'collect-and-exit-runtime-assets');
  assert.ok(worldState.assets.some((asset) => asset.id === 'collect_and_exit_sheet' && asset.status === 'loaded'));
  assert.equal(worldState.tilemaps.tilemaps[0].id, 'collect_and_exit_level');

  const asset_loading = loadingFromWorldState(worldState);
  const asset_preview = previewFromManifest(projectManifest);
  const asset_inspector = inspectorFrom(projectManifest, asset_loading, asset_preview);
  const run = {
    summary: { id: 'temp-collect-and-exit-asset-smoke', status: 'passed' },
    project: { id: 'collect_and_exit_demo', manifestPath: 'ouroforge.project.json' },
    asset_loading,
    asset_preview,
    asset_inspector,
  };
  const payload = { schema: 'ouroforge-dashboard-v1', runs: [run] };

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-demo-asset-evidence-'));
  const tempDashboard = path.join(tempDir, 'dashboard-data.json');
  fs.writeFileSync(tempDashboard, JSON.stringify(payload, null, 2));
  const parsed = readJson(tempDashboard);
  assert.equal(parsed.runs[0].asset_inspector.asset_count, 5);
  assert.equal(parsed.runs[0].asset_preview.atlas_frame_count, 6);
  assert.equal(parsed.runs[0].asset_preview.tilemap_count, 1);

  const dashboardLoading = dashboard.renderAssetLoading(run);
  assert.match(dashboardLoading, /collect_and_exit_sheet/);
  assert.match(dashboardLoading, /loaded/i);
  const dashboardPreview = dashboard.renderAssetPreview(run);
  assert.match(dashboardPreview, /collect_and_exit_atlas/);
  assert.match(dashboardPreview, /collect_and_exit_tilemap/);
  const studioInspector = cockpit.renderStudioAssetInspectorSurface(run);
  assert.match(studioInspector, /Asset inspector/);
  assert.match(studioInspector, /collect_and_exit_sheet/);
  assert.match(studioInspector, /collect_and_exit_atlas/);
  assert.match(studioInspector, /collect_and_exit_tilemap/);
  assert.match(studioInspector, /Runtime load evidence/);
  assert.match(studioInspector, /browser surfaces must not upload, write, fetch remote assets, edit manifests, or execute commands/);

  fs.rmSync(tempDir, { recursive: true, force: true });
  for (const generatedName of ['runs', 'dashboard-data', 'asset-previews']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, generatedName)), false, `${generatedName} must stay generated/untracked`);
  }

  console.log('collect-and-exit asset evidence dashboard/Studio smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
