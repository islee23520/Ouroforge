const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const read = (repoPath) => fs.readFileSync(path.join(repoRoot, repoPath), 'utf8');
const json = (repoPath) => JSON.parse(read(repoPath));
const exists = (repoPath) => fs.existsSync(path.join(repoRoot, repoPath));

const reportPath = '.omx/dogfood-validation/export-release-readiness.md';
const statusPath = '.omx/dogfood-validation/export-release-readiness.status.json';
const report = read(reportPath);
const normalized = report.replace(/\s+/g, ' ').trim();
const status = json(statusPath);

for (const repoPath of [
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  reportPath,
  statusPath,
]) {
  assert.ok(exists(repoPath), `expected tracked dogfood artifact exists: ${repoPath}`);
}

for (const section of [
  'Metadata',
  'Purpose',
  'Merged prerequisite evidence',
  'Local/manual package evidence',
  'Pipeline-to-package provenance join',
  'Package probe and performance evidence',
  'Generated-state cleanup and retention boundary',
  'Readiness verdict',
  'Verification commands',
  'Non-goals and guardrails',
]) {
  assert.match(report, new RegExp(`## ${section}`), `report includes ${section}`);
}

for (const required of [
  'dogfood-export-release-readiness-v1',
  'collect-and-exit-local-rc-candidate',
  'local-manual-rc-evidence-only',
  '#2334 MERGED',
  '#2335 MERGED',
  '#2336 MERGED',
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  'examples/playable-demo-v2/collect-and-exit/export/export-profile.json',
  'examples/playable-demo-v2/collect-and-exit/export/package-metadata.json',
  'explicit gap',
  'runtimeProbeMode: preserve',
  'exportTarget: web-local',
  'examples/godot-plus-demo-performance-v794/performance-budget-smoke.test.cjs',
  'docs/build-export-packaging-demo-v1.md',
  'Generated package outputs, dashboards, screenshots, run folders, bundles, checksums, and verification logs remain under ignored/generated roots',
  'pass-local-manual-evidence-gate',
  'No generated package artifact is committed by B4',
]) {
  assert.ok(normalized.includes(required.replace(/\s+/g, ' ')), `report records ${required}`);
}

const exportProfile = json('examples/playable-demo-v2/collect-and-exit/export/export-profile.json');
assert.equal(exportProfile.schemaVersion, 'export-profile-v1');
assert.equal(exportProfile.projectId, 'collect_and_exit_demo');
assert.equal(exportProfile.exportTarget, 'web-local');
assert.equal(exportProfile.runtimeProbeMode, 'preserve');
assert.match(exportProfile.boundary, /no publish, deploy, sign, or upload/i);
assert.ok(exists(exportProfile.entryScene), `entry scene exists: ${exportProfile.entryScene}`);
assert.ok(exists(exportProfile.packageMetadataRef), `package metadata ref exists: ${exportProfile.packageMetadataRef}`);

const packageMetadata = json('examples/playable-demo-v2/collect-and-exit/export/package-metadata.json');
assert.equal(packageMetadata.schemaVersion, 'export-package-metadata-v1');
assert.equal(packageMetadata.projectId, 'collect_and_exit_demo');
assert.equal(packageMetadata.version, exportProfile.version);
assert.equal(packageMetadata.entryScene, 'scenes/collect-and-exit.scene.json');

const packageText = JSON.stringify(packageMetadata);
for (const forbiddenField of ['signingKey', 'uploadUrl', 'publishTarget', 'steamDepotId', 'credential', 'token']) {
  assert.doesNotMatch(packageText, new RegExp(forbiddenField, 'i'), `package metadata omits ${forbiddenField}`);
}

assert.equal(status.schemaVersion, 'dogfood-export-release-readiness-status-v1');
assert.equal(status.blocker, 'B4');
assert.equal(status.status, 'ready_for_verifier');
assert.equal(status.readinessClassification, 'local-manual-rc-evidence-only');
assert.equal(status.protectedIssues['1'], 'OPEN');
assert.equal(status.protectedIssues['23'], 'OPEN');
assert.equal(status.forbiddenScopeIntroduced, false);
assert.equal(status.packageEvidence.retainedRcArtifact, 'explicit_gap_no_durable_origin_main_package_artifact');
assert.equal(status.packageEvidence.pipelineToPackageJoin, true);

for (const prereq of status.mergedPrerequisites) {
  assert.equal(prereq.state, 'MERGED', `${prereq.blocker} is merged`);
  assert.ok(exists(prereq.artifact), `${prereq.blocker} artifact exists: ${prereq.artifact}`);
}
for (const artifact of status.trackedArtifacts) assert.ok(exists(artifact), `tracked artifact exists: ${artifact}`);

for (const guardrail of [
  /#1 and #23 remain open/i,
  /Era Q M102–M106 remain deferred\/non-goal/i,
  /No release automation, signing, notarization, upload, publishing/i,
  /No production-ready, store-ready, commercial release/i,
]) {
  assert.match(report, guardrail, `guardrail present: ${guardrail}`);
}

for (const forbiddenOverclaim of [
  /production-ready\s+(?:export|package|release|artifact)/i,
  /store-ready\s+(?:export|package|release|artifact)/i,
  /commercial release ready/i,
  /Steam depot (?:configured|ready|uploaded|published)/i,
  /credential flow (?:implemented|enabled|configured)/i,
  /M102(?:–|-| to )M106\s+(?:active|implemented|complete|ready)/i,
]) {
  assert.doesNotMatch(report, forbiddenOverclaim, `forbidden overclaim absent: ${forbiddenOverclaim}`);
}

console.log('dogfood export/release readiness smoke passed');
