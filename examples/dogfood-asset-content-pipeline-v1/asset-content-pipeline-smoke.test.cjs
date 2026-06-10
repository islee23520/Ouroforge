const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const read = (repoPath) => fs.readFileSync(path.join(repoRoot, repoPath), 'utf8');
const json = (repoPath) => JSON.parse(read(repoPath));
const exists = (repoPath) => fs.existsSync(path.join(repoRoot, repoPath));

const reportPath = '.omx/dogfood-validation/asset-content-pipeline.md';
const statusPath = '.omx/dogfood-validation/asset-content-pipeline.status.json';
const report = read(reportPath);
const normalized = report.replace(/\s+/g, ' ').trim();
const status = json(statusPath);

for (const repoPath of [
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  '.omx/dogfood-validation/gameplay-runtime-stress.md',
  '.omx/dogfood-validation/studio-ux-validation.md',
  reportPath,
  statusPath,
]) {
  assert.ok(exists(repoPath), `expected dogfood artifact exists: ${repoPath}`);
}

for (const section of [
  'Metadata',
  'Purpose',
  'Merged prerequisite evidence',
  'Commands executed',
  'Asset/content evidence summary',
  'Bounded evidence and generated-state boundary',
  'Gaps and conservative wording',
  'Verification commands for this PR',
  'Non-goals and guardrails',
]) {
  assert.match(report, new RegExp(`## ${section}`), `report includes ${section}`);
}

for (const required of [
  'dogfood-asset-content-pipeline-v1',
  'collect-and-exit-local-rc-candidate',
  'bounded-local-asset-content-pipeline-evidence',
  '#2334 MERGED',
  '#2335 MERGED',
  '#2336 MERGED',
  '#2337 MERGED',
  '#2339 MERGED',
  '#2340 MERGED',
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  '.omx/dogfood-validation/gameplay-runtime-stress.md',
  '.omx/dogfood-validation/studio-ux-validation.md',
  'asset pack smoke OK; 5 assets verified',
  'asset pipeline v1 regression evidence smoke passed',
  'asset pipeline v1 dashboard compatibility smoke passed',
  'scenario_coverage_v34_asset_pipeline',
  '6 Rust tests passed',
  'No new asset generator',
]) {
  assert.ok(normalized.includes(required.replace(/\s+/g, ' ')), `report records ${required}`);
}

const manifest = json('examples/playable-demo-v2/collect-and-exit/asset-manifest.json');
const provenance = json('examples/playable-demo-v2/collect-and-exit/asset-provenance.json');
assert.equal(manifest.schemaVersion, 'asset-manifest-v1');
assert.equal(provenance.schemaVersion, 'demo-asset-provenance-v1');
assert.equal(manifest.assets.length, 5);
assert.equal(provenance.copyrightRisk, 'none');
const byId = new Map(manifest.assets.map((asset) => [asset.id, asset]));
for (const asset of manifest.assets) {
  assert.ok(exists(path.join('examples/playable-demo-v2/collect-and-exit', asset.path)), `asset file exists: ${asset.id}`);
  assert.equal(asset.classification, 'source_like', `${asset.id} source-like`);
  assert.ok(asset.license, `${asset.id} license`);
  assert.ok(asset.source, `${asset.id} source`);
}
const atlas = manifest.assets.find((asset) => asset.type === 'sprite_atlas');
assert.ok(byId.has(atlas.atlas.imageAssetId), 'atlas image ref resolves');
const tilemap = manifest.assets.find((asset) => asset.type === 'tilemap');
assert.ok(byId.has(tilemap.tilemap.tilesetAssetId), 'tilemap tileset ref resolves');

const regressionManifest = json('examples/asset-pipeline-v1-regression/asset-manifest.json');
assert.equal(regressionManifest.schemaVersion, 'asset-manifest-v1');
assert.ok(regressionManifest.assets.some((asset) => asset.type === 'sprite_atlas'));
assert.ok(regressionManifest.assets.some((asset) => asset.type === 'tilemap'));
assert.ok(exists('examples/asset-pipeline-v1-regression/invalid/hash-mismatch.asset-manifest.json'));
assert.ok(exists('examples/asset-pipeline-v1-regression/invalid/missing-asset.asset-manifest.json'));

assert.equal(status.schemaVersion, 'dogfood-asset-content-pipeline-status-v1');
assert.equal(status.blocker, 'B7');
assert.equal(status.status, 'ready_for_verifier');
assert.equal(status.evidenceClassification, 'bounded-local-asset-content-pipeline-evidence');
assert.equal(status.protectedIssues['1'], 'OPEN');
assert.equal(status.protectedIssues['23'], 'OPEN');
assert.equal(status.forbiddenScopeIntroduced, false);
assert.equal(status.assetEvidence.verifiedAssetCount, 5);
assert.equal(status.assetEvidence.rustTestsPassed, 6);
for (const artifact of status.trackedArtifacts) assert.ok(exists(artifact), `tracked artifact exists: ${artifact}`);
for (const prereq of status.mergedPrerequisites) {
  assert.equal(prereq.state, 'MERGED', `${prereq.blocker} merged`);
  assert.ok(exists(prereq.artifact), `${prereq.blocker} artifact exists`);
}

for (const guardrail of [
  /#1 and #23 remain open/i,
  /Era Q M102-M106 remain deferred\/non-goal/i,
  /No product asset pipeline feature/i,
  /No production-ready, store-ready, commercial release/i,
]) {
  assert.match(report, guardrail, `guardrail present: ${guardrail}`);
}

for (const forbiddenOverclaim of [
  /production-ready\s+(?:asset|content|pipeline)/i,
  /store-ready\s+(?:asset|content|pipeline)/i,
  /commercial release ready/i,
  /full Godot parity is verified|claims full Godot parity/i,
  /Godot replacement status is verified/i,
  /M102(?:–|-| to )M106\s+(?:active|implemented|complete|ready)/i,
  /trusted browser writes are allowed/i,
  /remote upload enabled/i,
  /asset marketplace ready/i,
]) {
  assert.doesNotMatch(report, forbiddenOverclaim, `forbidden overclaim absent: ${forbiddenOverclaim}`);
}

console.log('dogfood asset/content pipeline smoke passed');
