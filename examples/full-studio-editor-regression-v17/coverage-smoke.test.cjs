const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cockpit = require('../authoring-cockpit/cockpit.js');

const matrixPath = path.join(__dirname, 'coverage-matrix.fixture.json');
const demoPath = path.join(__dirname, '..', 'full-studio-editor-demo-v1', 'demo.fixture.json');
const docPath = path.join(__dirname, '..', '..', 'docs', 'scenario-coverage-v17-full-studio-editor.md');
const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));
const demo = JSON.parse(fs.readFileSync(demoPath, 'utf8'));
const doc = fs.readFileSync(docPath, 'utf8');

const requiredSuccess = [
  'project overview',
  'scene tree',
  'entity inspector',
  'draft edit preview',
  'Safe Source Apply handoff preview',
  'visual canvas',
  'asset browser',
  'scenario/evidence panel',
  'evidence timeline',
  'export panel',
  'plugin panel',
  'workspace persistence',
  'command palette',
];
const requiredFailures = [
  'invalid scene reference',
  'missing asset',
  'malformed plugin descriptor',
  'stale source apply target',
  'blocked direct trusted write',
  'blocked publish/deploy command',
  'blocked shell command',
  'broken evidence bundle',
  'invalid workspace state',
  'large fixture budget exceeded',
];

assert.equal(matrix.schemaVersion, 'scenario-coverage-v17-full-studio-editor-v1');
assert.equal(matrix.issue, 775);
assert.equal(matrix.status, 'ready');
assert.equal(matrix.roadmapAnchor, '#1');
assert.equal(matrix.memoryAnchor, '#23');
assert.equal(matrix.generatedStatePolicy.trackedFixtureOnly, true);
assert.equal(matrix.generatedStatePolicy.noWorkspaceArtifactsCommitted, true);
assert.match(doc, /Issue: #775/);
assert.match(doc, /#1 and #23 remain open/);
assert.doesNotMatch(doc, /production-ready editor is available|current Godot replacement is implemented|full Godot editor parity is implemented|auto-apply enabled|auto-merge enabled|plugin runtime enabled|native export ready|secure sandbox is guaranteed/);

const successNames = matrix.successScenarios.map((scenario) => scenario.surface);
assert.deepEqual(successNames, requiredSuccess);
const failureKinds = matrix.failureScenarios.map((scenario) => scenario.blockedKind);
assert.deepEqual(failureKinds, requiredFailures);

for (const scenario of matrix.successScenarios) {
  assert.ok(scenario.id.startsWith('success.'), `success id must be namespaced: ${scenario.id}`);
  assert.ok(Array.isArray(scenario.evidence) && scenario.evidence.length > 0, `${scenario.id} needs evidence refs`);
  assert.match(JSON.stringify(scenario), /expectedText|workspace layout/);
}
for (const scenario of matrix.failureScenarios) {
  assert.ok(scenario.id.startsWith('failure.'), `failure id must be namespaced: ${scenario.id}`);
  assert.match(scenario.diagnostic, /\w/);
  assert.ok(Array.isArray(scenario.blockedControls) && scenario.blockedControls.length > 0, `${scenario.id} must name blocked controls`);
}

const rendered = [
  cockpit.renderProjectOverviewSurface(demo),
  cockpit.renderStudioSceneTreeInspectorSurface(demo),
  cockpit.renderEntityComponentInspectorSurface(demo),
  cockpit.renderStudioDraftAuthoringSurface(demo),
  cockpit.renderStudioSourceApplyHandoffSurface(demo),
  cockpit.renderStudioSceneCanvasSurface(demo),
  cockpit.renderStudioAssetBrowserSurface(demo),
  cockpit.renderStudioScenarioPanelSurface(demo),
  cockpit.renderEvidenceTimelineSurface(demo.evidence_timeline || demo),
  cockpit.renderStudioExportPackageInspectionSurface(demo),
  cockpit.renderStudioPluginPanelSurface(demo),
  cockpit.renderStudioCommandPaletteSurface(demo),
  cockpit.renderFullStudioEditorDemoSurface(demo),
].join('\n');

for (const scenario of matrix.successScenarios) {
  if (scenario.expectedText === 'workspace layout') continue;
  assert.match(rendered, new RegExp(scenario.expectedText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')), `${scenario.id} expected text`);
}

assert.equal(cockpit.resolveStudioCommand('apply-source').allowed, false);
assert.equal(cockpit.resolveStudioCommand('execute-plugin').allowed, false);
assert.equal(cockpit.resolveStudioCommand('publish').allowed, false);
assert.doesNotMatch(rendered, /trustedWriteCommand|executeCommand|browserCommandBridge|publishCommand|deployCommand|signCommand|uploadCommand|installPluginCommand|pluginExecuteCommand|marketplaceCommand|networkInstall/);

module.exports = { matrix, demo };

if (require.main === module) {
  console.log(JSON.stringify({ issue: matrix.issue, success: matrix.successScenarios.length, failure: matrix.failureScenarios.length }));
}
