#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');

const root = __dirname;
function readJson(rel) {
  return JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
}

const required = [
  'README.md',
  'ouroforge.project.json',
  'seeds/signal-gate-relay.yaml',
  'asset-manifest.json',
];
for (const rel of required) assert.ok(fs.existsSync(path.join(root, rel)), `missing ${rel}`);

const manifest = readJson('ouroforge.project.json');
assert.equal(manifest.schemaVersion, 'project-manifest-v1');
assert.equal(manifest.project.id, 'signal_gate_relay_dogfood');
assert.equal(manifest.project.gdd, 'docs/dogfood-mini-game-gdd-v1.md');
assert.deepEqual(manifest.governance.anchorsRemainOpen, [1, 23]);
for (const rootName of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.ok(manifest.generated.roots.includes(rootName), `generated root not declared: ${rootName}`);
  assert.equal(fs.existsSync(path.join(root, rootName)), false, `generated root must not be committed: ${rootName}`);
}
assert.ok(manifest.scenes[0].path.startsWith('scenes/'));
assert.ok(manifest.seeds[0].path.startsWith('seeds/'));
assert.ok(manifest.scenarioPacks[0].path.startsWith('scenarios/'));

const seed = fs.readFileSync(path.join(root, 'seeds/signal-gate-relay.yaml'), 'utf8');
for (const token of ['Signal Gate Relay', 'product-observed-pending-live-evidence', '#1 and #23 remain open', 'generated artifacts stay in ignored roots']) {
  assert.ok(seed.includes(token), `seed missing ${token}`);
}

const assets = readJson('asset-manifest.json');
assert.equal(assets.id, 'signal-gate-relay-assets');
assert.equal(assets.assets.length, 2);
assert.ok(assets.boundary.includes('no generated asset root'));

console.log('signal-gate scaffold audit passed');
