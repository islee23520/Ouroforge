const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const dashboard = require(path.join(repoRoot, 'examples', 'evidence-dashboard', 'dashboard.js'));
const cockpit = require(path.join(repoRoot, 'examples', 'authoring-cockpit', 'cockpit.js'));

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function previewFromManifest(manifest) {
  const records = manifest.assets.map((asset) => ({
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
    evidence_refs: ['evidence/asset-preview/asset-pipeline-v1-regression.json'],
    boundary: 'Read-only asset preview metadata; generated previews remain local/untracked and browser surfaces must not upload, write, fetch remote assets, or execute commands.',
    empty_state: '',
  };
}

function loadingFromManifest(manifest) {
  const records = manifest.assets
    .filter((asset) => asset.type === 'image' || asset.type === 'audio')
    .map((asset, index) => ({
      attemptId: `load-${asset.id}`,
      assetId: asset.id,
      assetType: asset.type,
      path: asset.path,
      status: 'loaded',
      loadDurationMs: index + 1,
      failureReason: null,
      width: asset.dimensions?.width || null,
      height: asset.dimensions?.height || null,
    }));
  return {
    present: true,
    attempt_count: records.length,
    loaded_count: records.length,
    failed_count: 0,
    rejected_count: 0,
    fallback_count: 0,
    records,
    evidence_refs: ['evidence/runtime-assets/asset-pipeline-v1-regression.json'],
    boundary: 'Read-only runtime asset loading evidence; browser surfaces display escaped local evidence only and never upload, write trusted state, or execute commands.',
    empty_state: '',
  };
}

function integrityFromInvalidFixtures() {
  const warnings = [
    { kind: 'stale_asset_hash', assetId: 'asset_regression_sheet', path: 'invalid/hash-mismatch.asset-manifest.json', message: 'hash mismatch fixture remains explicit evidence' },
    { kind: 'missing_asset_file', assetId: 'asset_regression_sheet', path: 'invalid/missing-asset.asset-manifest.json', message: 'missing asset fixture remains explicit evidence' },
  ];
  return {
    present: true,
    status: 'warnings',
    warning_count: warnings.length,
    resolved_count: 0,
    warnings,
    evidence_refs: ['evidence/asset-integrity/asset-pipeline-v1-regression.json'],
    boundary: 'Read-only Rust validation evidence. No remote fetch, browser upload, trusted write, or command execution.',
    empty_state: '',
  };
}

function inspectorFrom(manifest, loading, preview, integrity) {
  const runtimeById = new Map();
  for (const record of loading.records) {
    if (!runtimeById.has(record.assetId)) runtimeById.set(record.assetId, []);
    runtimeById.get(record.assetId).push(record.status);
  }
  const warningsById = new Map();
  for (const warning of integrity.warnings) {
    if (!warningsById.has(warning.assetId)) warningsById.set(warning.assetId, []);
    warningsById.get(warning.assetId).push(warning.kind);
  }
  return {
    present: true,
    status: 'warnings',
    asset_count: manifest.assets.length,
    preview_count: preview.preview_count,
    runtime_attempt_count: loading.attempt_count,
    loaded_count: loading.loaded_count,
    failed_count: loading.failed_count,
    warning_count: integrity.warning_count,
    atlas_frame_count: preview.atlas_frame_count,
    tilemap_count: preview.tilemap_count,
    evidence_refs: ['evidence/asset-inspector/asset-pipeline-v1-regression.json'],
    boundary: 'Read-only Studio asset inspector data assembled from exported evidence; browser surfaces must not upload, write, fetch remote assets, edit manifests, or execute commands.',
    empty_state: '',
    assets: manifest.assets.map((asset) => ({
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
      warnings: warningsById.get(asset.id) || [],
    })),
  };
}

const manifest = readJson(path.join(fixtureDir, 'asset-manifest.json'));
const asset_preview = previewFromManifest(manifest);
const asset_loading = loadingFromManifest(manifest);
const asset_integrity = integrityFromInvalidFixtures();
const asset_inspector = inspectorFrom(manifest, asset_loading, asset_preview, asset_integrity);
const run = {
  summary: { id: 'asset-pipeline-v1-regression-smoke' },
  artifacts: [],
  asset_preview,
  asset_loading,
  asset_integrity,
  asset_inspector,
};

const dashboardHtml = [
  dashboard.renderAssetIntegrity(run),
  dashboard.renderAssetLoading(run),
  dashboard.renderAssetPreview(run),
].join('\n');
assert.ok(dashboardHtml.includes('Asset preview evidence refs'));
assert.ok(dashboardHtml.includes('asset_regression_tilemap'));
assert.ok(dashboardHtml.includes('Read-only'));
assert.ok(!dashboardHtml.includes('<script>'));

const cockpitHtml = [
  cockpit.renderRuntimeAssetLoadingSurface(run),
  cockpit.renderAssetPreviewEvidenceSurface(run),
  cockpit.renderStudioAssetInspectorSurface(run),
].join('\n');
assert.ok(cockpitHtml.includes('Runtime asset loading'));
assert.ok(cockpitHtml.includes('Asset inspector'));
assert.ok(cockpitHtml.includes('asset_regression_atlas'));
assert.ok(cockpitHtml.includes('upload files') || cockpitHtml.includes('upload assets'));
assert.ok(!cockpitHtml.includes('<script>'));

for (const name of ['runs', 'target', 'dashboard-data']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay generated/untracked`);
}

console.log('asset pipeline v1 dashboard compatibility smoke passed');
