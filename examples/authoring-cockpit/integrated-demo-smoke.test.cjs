const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cockpit = require('./cockpit.js');

const fixturePath = path.join(__dirname, 'integrated-demo-v1.fixture.json');
const docPath = path.resolve(__dirname, '..', '..', 'docs', 'full-studio-integrated-demo-v1.md');

const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
const doc = fs.readFileSync(docPath, 'utf8');

assert.equal(fixture.schemaVersion, 'full-studio-integrated-demo-v1');
assert.equal(fixture.issue, 774);
assert.equal(fixture.generatedState.trackedFixtureOnly, true);
assert.match(doc, /Full Studio Editor Integrated Demo v1/);
assert.match(doc, /not full Godot editor parity/);
assert.match(doc, /#1 and #23 remain open/);

const markup = [
  `<section><h2>Scene tree</h2>${cockpit.renderTree(fixture.scene, fixture.selectedEntity)}</section>`,
  `<section><h2>Inspector</h2>${cockpit.renderInspector(fixture.scene, fixture.selectedEntity, fixture.scenePath)}</section>`,
  cockpit.renderIntegration(fixture.run),
].join('\n');
const requiredPanels = [
  /Project workspace/,
  /Scene tree/,
  /Inspector/,
  /Live preview controls|Preview/,
  /Asset inspector/,
  /Scenario and playtest panel/,
  /Export \/ package inspection/,
  /Plugin \/ extension panel/,
  /Command palette/,
  /Studio draft authoring/,
  /Visual diff preview/,
  /Source patch evidence bundle/,
  /Safe Source Apply handoff/,
];

for (const panel of requiredPanels) {
  assert.match(markup, panel);
}

assert.match(markup, /draft-scene-color/);
assert.match(markup, /source-apply-handoff-preview/);
assert.match(markup, /Publish\/release:|Export\/publish\/deploy\/sign\/upload:/);
assert.match(markup, /does not execute it, apply patches, merge branches, self-approve reviews, or write trusted files|does not execute commands or write trusted files/);
assert.doesNotMatch(markup, /trustedWriteCommand|executeCommand|browserCommandBridge|publishCommand|deployCommand|signCommand|uploadCommand|installPluginCommand|pluginExecuteCommand|marketplaceCommand|networkInstall/);
assert.doesNotMatch(markup, /Godot replacement|full Godot editor parity|production-ready collaborative editor|secure sandbox/);

console.log(JSON.stringify({
  issue: 774,
  panelCount: requiredPanels.length,
  fixture: fixture.fixtureId,
}));
