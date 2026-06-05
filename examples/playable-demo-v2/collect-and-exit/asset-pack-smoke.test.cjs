#!/usr/bin/env node
'use strict';

// Godot-Plus demo asset pack smoke (#786).
//
// Validates the Collect-and-Exit asset pack: every manifest asset file exists,
// the committed fnv1a64-file-v1 content hash matches the file bytes, there are no
// duplicate asset ids or paths, every asset records license/source/provenance,
// the provenance record covers every manifest asset with no copyright risk, and
// the atlas/tileset/tilemap cross-references resolve. Pure read-only audit: no
// writes, no commands, no network.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;

function readJson(rel) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, rel), 'utf8'));
}

// fnv1a64-file-v1: FNV-1a 64-bit over file bytes, 16-hex (matches Rust fnv1a64).
function fnv1a64(buf) {
  let hash = 0xcbf29ce484222325n;
  const prime = 0x100000001b3n;
  const mask = (1n << 64n) - 1n;
  for (const byte of buf) {
    hash ^= BigInt(byte);
    hash = (hash * prime) & mask;
  }
  return hash.toString(16).padStart(16, '0');
}

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

const manifest = readJson('asset-manifest.json');
const provenance = readJson('asset-provenance.json');

assert.equal(manifest.schemaVersion, 'asset-manifest-v1');
assert.equal(provenance.schemaVersion, 'demo-asset-provenance-v1');

const seenIds = new Set();
const seenPaths = new Set();

for (const asset of manifest.assets) {
  check(`asset file exists: ${asset.id}`, () => {
    assert.ok(fs.existsSync(path.join(fixtureDir, asset.path)), `missing file ${asset.path}`);
  });
  check(`content hash matches: ${asset.id}`, () => {
    assert.equal(asset.contentHash.algorithm, 'fnv1a64-file-v1', 'unexpected hash algorithm');
    const actual = fnv1a64(fs.readFileSync(path.join(fixtureDir, asset.path)));
    assert.equal(actual, asset.contentHash.value, `hash drift for ${asset.path}`);
  });
  check(`no duplicate id: ${asset.id}`, () => {
    assert.ok(!seenIds.has(asset.id), `duplicate asset id ${asset.id}`);
    seenIds.add(asset.id);
  });
  check(`no duplicate path: ${asset.path}`, () => {
    assert.ok(!seenPaths.has(asset.path), `duplicate asset path ${asset.path}`);
    seenPaths.add(asset.path);
  });
  check(`license/source/classification present: ${asset.id}`, () => {
    assert.ok(asset.license && asset.license.length, 'missing license');
    assert.ok(asset.source && asset.source.length, 'missing source');
    assert.ok(asset.classification === 'source_like', 'asset must be source_like');
  });
}

// Provenance covers every manifest asset, with no copyright risk.
check('provenance covers every manifest asset with no copyright risk', () => {
  assert.equal(provenance.copyrightRisk, 'none', 'pack copyright risk must be none');
  const provById = new Map(provenance.assets.map((entry) => [entry.id, entry]));
  for (const asset of manifest.assets) {
    const entry = provById.get(asset.id);
    assert.ok(entry, `provenance missing for ${asset.id}`);
    assert.equal(entry.copyrightRisk, 'none', `copyright risk for ${asset.id}`);
    assert.ok(entry.license && entry.license.length, `provenance license for ${asset.id}`);
    assert.ok(entry.origin === 'authored-in-repo', `provenance origin for ${asset.id}`);
  }
});

// Cross-references resolve (atlas image, tileset->tilemap).
check('asset cross-references resolve', () => {
  const byId = new Map(manifest.assets.map((asset) => [asset.id, asset]));
  const atlas = manifest.assets.find((a) => a.type === 'sprite_atlas');
  assert.ok(byId.has(atlas.atlas.imageAssetId), 'atlas imageAssetId resolves');
  const tilemap = manifest.assets.find((a) => a.type === 'tilemap');
  assert.ok(byId.has(tilemap.tilemap.tilesetAssetId), 'tilemap tilesetAssetId resolves');
});

if (failures > 0) {
  console.error(`\nasset pack smoke FAILED with ${failures} failure(s)`);
  process.exit(1);
}
console.log(`\nasset pack smoke OK; ${manifest.assets.length} assets verified`);
