#!/usr/bin/env node
'use strict';

// Godot-Plus demo scaffold generated-state audit (#781).
//
// Asserts that the collect-and-exit scaffold ships the expected source-like
// files (project manifest, scene, seed, scenario pack, asset manifest, export
// profile placeholder, package metadata placeholder, and inert plugin
// descriptor) and that no generated-state roots (runs/, target/, dashboard-data/)
// are tracked inside the fixture. Pure read-only audit: no writes, no commands,
// no network.

const fs = require('fs');
const path = require('path');

const root = __dirname;
let failures = 0;

function check(label, fn) {
  try {
    fn();
    console.log(`ok - ${label}`);
  } catch (err) {
    failures += 1;
    console.error(`not ok - ${label}: ${err.message}`);
  }
}

function readJson(rel) {
  const full = path.join(root, rel);
  return JSON.parse(fs.readFileSync(full, 'utf8'));
}

function assert(cond, msg) {
  if (!cond) throw new Error(msg);
}

// 1. Required source-like scaffold files exist and parse.
const requiredJson = [
  'ouroforge.project.json',
  'scenes/collect-and-exit.scene.json',
  'scenarios/collect-and-exit.json',
  'asset-manifest.json',
  'export/export-profile.json',
  'export/package-metadata.json',
  'plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json',
];
for (const rel of requiredJson) {
  check(`scaffold file parses: ${rel}`, () => {
    readJson(rel);
  });
}
check('scaffold file exists: seeds/collect-and-exit.yaml', () => {
  assert(fs.existsSync(path.join(root, 'seeds/collect-and-exit.yaml')), 'missing Seed');
});

// 2. Project manifest declares generated roots and stays unchanged in shape.
check('project manifest declares generated roots', () => {
  const project = readJson('ouroforge.project.json');
  assert(project.project.id === 'collect_and_exit_demo', 'unexpected project id');
  const roots = (project.generated && project.generated.roots) || [];
  for (const r of ['runs', 'target', 'dashboard-data']) {
    assert(roots.includes(r), `generated root not declared: ${r}`);
  }
});

// 3. No generated-state roots are tracked inside the fixture.
check('no tracked generated-state roots in fixture', () => {
  for (const r of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert(!fs.existsSync(path.join(root, r)), `generated root must not be committed: ${r}`);
  }
});

// 4. Export profile placeholder stays local and fail-closed.
check('export profile is local web target, fail closed', () => {
  const profile = readJson('export/export-profile.json');
  assert(profile.schemaVersion === 'export-profile-v1', 'bad export schemaVersion');
  assert(profile.exportTarget === 'web-local', 'export target must be web-local');
  assert(profile.outputDir.startsWith('dist/'), 'output must land in ignored staging root');
  const b = profile.boundary.toLowerCase();
  for (const token of ['local', 'evidence', 'no publish', 'fail closed']) {
    assert(b.includes(token), `export boundary missing token: ${token}`);
  }
  for (const bad of ['production-ready', 'godot replacement', 'auto-merge', 'http://', 'https://']) {
    assert(!b.includes(bad), `export boundary contains forbidden text: ${bad}`);
  }
});

// 5. Plugin descriptor is an inert read-only panel placeholder.
check('plugin descriptor is inert read-only panel', () => {
  const plugin = readJson('plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json');
  assert(plugin.schemaVersion === 'ouroforge.plugin-manifest.v1', 'bad plugin schemaVersion');
  assert(
    (plugin.declaredCapabilities || []).includes('dashboardPanel'),
    'plugin must declare dashboardPanel'
  );
  const b = plugin.boundary.toLowerCase();
  assert(b.includes('no executable code'), 'plugin boundary must be inert');
});

if (failures > 0) {
  console.error(`\nscaffold audit FAILED with ${failures} failure(s)`);
  process.exit(1);
}
console.log('\nscaffold audit OK');
