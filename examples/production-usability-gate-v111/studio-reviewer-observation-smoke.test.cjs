#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');

const repoRoot = path.resolve(__dirname, '..', '..');
const gateDir = __dirname;
const fixturePath = path.join(gateDir, 'studio-reviewer-observation.fixture.json');
const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));

assert.equal(fixture.schemaVersion, 'm130-studio-reviewer-observation-v1');
assert.equal(fixture.projectRef, 'signal-gate-dogfood');
assert.equal(fixture.ownerIssue, 2494);
assert.equal(fixture.trustedWritePolicy, 'rust-cli-only');
assert.equal(fixture.manualGapId, 'm130-2391-manual-studio-launch');

const manifestPath = path.join(repoRoot, fixture.projectManifestPath);
assert.ok(fs.existsSync(manifestPath), `project manifest must exist: ${fixture.projectManifestPath}`);

for (const surface of fixture.readOnlySurfaces) {
  assert.ok(fs.existsSync(path.join(repoRoot, surface)), `read-only surface must exist: ${surface}`);
}

const cockpit = require(path.join(repoRoot, 'examples/authoring-cockpit/cockpit.js'));
for (const fnName of fixture.cockpitRenderFunctionsReferenced) {
  assert.equal(typeof cockpit[fnName], 'function', `cockpit.js must export ${fnName}`);
}

for (const commandRef of fixture.copyableCommands) {
  const commandPath = path.join(repoRoot, commandRef);
  assert.ok(fs.existsSync(commandPath), `copyable command file must exist: ${commandRef}`);
  const commandFixture = JSON.parse(fs.readFileSync(commandPath, 'utf8'));
  assert.ok(typeof commandFixture.command === 'string' && commandFixture.command.length > 0);
}

const fixtureRaw = fs.readFileSync(fixturePath, 'utf8');
const forbidden = ['localStorage', 'indexedDB', 'trustedWriteCommand', 'browserCommandBridge'];
for (const word of forbidden) {
  assert.ok(!fixtureRaw.includes(word), `fixture must not reference forbidden write/bridge keyword: ${word}`);
}

const evidencePath = path.join(repoRoot, fixture.evidenceDocRef);
assert.ok(fs.existsSync(evidencePath), `evidence doc must exist: ${fixture.evidenceDocRef}`);

execFileSync('node', ['--check', path.join(repoRoot, 'examples/authoring-cockpit/cockpit.js')], {
  cwd: repoRoot,
  stdio: 'pipe',
});

console.log('studio reviewer observation smoke passed');