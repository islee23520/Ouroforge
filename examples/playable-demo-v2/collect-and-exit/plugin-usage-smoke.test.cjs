#!/usr/bin/env node
'use strict';

// Godot-Plus demo plugin usage smoke (#792).
// Validates declarative plugin descriptor usage only: dashboard panel,
// scenario template, asset metadata, registry/evidence linkage, and no
// executable/install/network/source-write authority.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}

const evidence = readJson('plugin-usage-evidence.json');
assert.equal(evidence.schemaVersion, 'demo-plugin-usage-evidence-v1');
assert.equal(evidence.issue, 792);
assert.deepEqual(evidence.governance.protectedIssuesMustRemainOpen, [1, 23]);

const expected = new Map(evidence.plugins.map((plugin) => [plugin.pluginId, plugin]));
assert.deepEqual([...expected.keys()].sort(), [
  'collect-and-exit-asset-metadata',
  'collect-and-exit-dashboard-panel',
  'collect-and-exit-scenario-template',
]);

const forbiddenText = /executeScript|run_command|install_dependency|network_access|write_source|publish_export|access_credentials|mutate_ci|native_extension|marketplace|autoApply|autoMerge/i;
for (const plugin of evidence.plugins) {
  const manifest = readJson(plugin.manifestPath);
  assert.equal(manifest.schemaVersion, 'ouroforge.plugin-manifest.v1');
  assert.equal(manifest.pluginId, plugin.pluginId);
  assert.ok(manifest.declaredCapabilities.includes(plugin.capability), `${plugin.pluginId} capability`);
  assert.ok(manifest.extensionPoints.includes(plugin.extensionPoint), `${plugin.pluginId} extension point`);
  assert.match(manifest.boundary, /Declarative read-only/i);
  assert.match(manifest.boundary, /no executable code/i);
  assert.match(manifest.boundary, /no command execution/i);
  assert.match(manifest.boundary, /no network install/i);
  assert.doesNotMatch(JSON.stringify(manifest.permissions || []), forbiddenText, `${plugin.pluginId} permissions stay safe`);
}

const scenarioManifest = readJson('plugins/collect-and-exit-scenario-template/ouroforge.plugin.json');
assert.equal(scenarioManifest.descriptorRefs.length, 1);
assert.equal(scenarioManifest.descriptorRefs[0].kind, 'scenarioTemplate');
const scenarioTemplate = readJson(path.join('plugins/collect-and-exit-scenario-template', scenarioManifest.descriptorRefs[0].path));
assert.equal(scenarioTemplate.templateId, 'collect-and-exit-win-path-template');
assert.equal(scenarioTemplate.expectedEvidenceType, 'scenarioPack');
assert.match(scenarioTemplate.boundary, /no executable scripts/i);
assert.match(scenarioTemplate.boundary, /no command hooks/i);
assert.match(scenarioTemplate.boundary, /no network references/i);
assert.match(scenarioTemplate.boundary, /no trusted writes/i);

const assetManifest = readJson('plugins/collect-and-exit-asset-metadata/ouroforge.plugin.json');
assert.equal(assetManifest.assetMetadata.length, 1);
assert.equal(assetManifest.assetMetadata[0].descriptorId, 'collect-and-exit-demo-asset-metadata');
assert.ok(assetManifest.assetMetadata[0].fields.some((field) => field.name === 'fixtureRole'));
assert.ok(assetManifest.assetMetadata[0].manifestIntegrationKeys.includes('assetManifestId'));
assert.match(assetManifest.assetMetadata[0].boundary, /no asset generation/i);
assert.match(assetManifest.assetMetadata[0].boundary, /no command execution/i);
assert.match(assetManifest.assetMetadata[0].boundary, /no network access/i);

for (const [guardrail, value] of Object.entries(evidence.guardrails)) {
  if (typeof value === 'boolean') assert.equal(value, true, `${guardrail} guardrail`);
}
assert.match(evidence.guardrails.wordingBoundary, /no Godot replacement/i);
assert.equal(evidence.studioDashboardIntegration.trustedWrites, 0);
assert.equal(evidence.studioDashboardIntegration.browserCommandBridge, false);

for (const generatedName of ['runs', 'target', 'dashboard-data', 'dist', 'screenshots', 'qa-reports']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, generatedName)), false, `${generatedName} remains generated/untracked`);
}

console.log('collect-and-exit plugin usage smoke passed; 3 declarative descriptors verified');
