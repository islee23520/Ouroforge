const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cockpit = require('../authoring-cockpit/cockpit.js');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const docPath = path.join(repoRoot, 'docs', 'godot-plus-demo-studio-walkthrough-v1.md');
const fixturePath = path.join(fixtureDir, 'walkthrough-v790.fixture.json');

const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
const doc = fs.readFileSync(docPath, 'utf8');
const run = fixture.run;

assert.equal(fixture.issue, 790);
assert.equal(fixture.demoRoot, 'examples/playable-demo-v2/collect-and-exit');
assert.equal(fixture.generatedState.trackedFixtureOnly, true);

assert.match(doc, /Studio Demo Authoring Walkthrough v1/);
assert.match(doc, /Signal Gate|collect-and-exit/i);
assert.match(doc, /read-only|draft-only/i);
assert.match(doc, /does not execute|trusted write/i);
assert.match(doc, /#1 and #23 remain open/);
assert.doesNotMatch(doc, /production-ready Godot replacement|full Godot parity achieved|commercial release ready|universal superiority/i);

const scene = fixture.scene;
const selectedEntity = fixture.selectedEntity;
const scenePath = fixture.scenePath;

const markup = [
  cockpit.renderProjectOverviewSurface(run),
  cockpit.renderTree(scene, selectedEntity),
  cockpit.renderInspector(scene, selectedEntity, scenePath),
  cockpit.renderStudioSceneCanvasSurface(run),
  cockpit.renderStudioAssetBrowserSurface(run),
  cockpit.renderStudioScenarioPanelSurface(run),
  cockpit.renderEvidenceTimelineSurface(run),
  cockpit.renderStudioExportPackageInspectionSurface(run),
  cockpit.renderStudioPluginPanelSurface(run),
  cockpit.renderStudioDraftAuthoringSurface(run),
  cockpit.renderStudioSourceApplyHandoffSurface(run),
  cockpit.renderFullStudioEditorDemoSurface(run),
].join('\n');

const requiredPanels = [
  /Project overview|project-overview/i,
  /Scene tree|studio-scene-tree/i,
  /Inspector|entity inspector/i,
  /Visual scene canvas|studio-scene-canvas/i,
  /Asset browser/i,
  /Scenario|playtest/i,
  /Evidence timeline/i,
  /Export.*package|export-package/i,
  /Plugin|extension panel/i,
  /Draft authoring|draft edit/i,
  /Safe Source Apply|source apply handoff/i,
];

for (const panel of requiredPanels) {
  assert.match(markup, panel, `walkthrough markup includes ${panel}`);
}

assert.doesNotMatch(
  markup,
  /trustedWriteCommand|executeCommand|browserCommandBridge|auto-apply enabled|auto-merge enabled/i,
);

const demoModel = cockpit.fullStudioEditorDemoModel(run);
assert.equal(demoModel.present, true);
assert.equal(demoModel.issue, 790);
assert.equal(demoModel.missingPanels.length, 0);

console.log(
  JSON.stringify({
    issue: 790,
    fixture: fixture.fixtureId,
    panelChecks: requiredPanels.length,
  }),
);
