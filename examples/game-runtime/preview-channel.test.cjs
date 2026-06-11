'use strict';
// M131.2 PR-1 contract tests (Era X #2519): mechanical preview-delta
// application. Table-driven over the committed fixture file so the JS
// mapping stays 1:1 with the Rust allowlist in
// crates/ouroforge-core-types (SUPPORTED_SCENE_EDIT_PATHS) and the delta
// shape in docs/preview-session-v1.md.

const assert = require('node:assert');
const path = require('node:path');
const previewChannel = require('./preview-channel.js');

const fixtures = require(path.join(__dirname, 'preview-delta-fixtures-v1.json'));

const RUST_SUPPORTED_SCENE_EDIT_PATHS = [
  'sprite.color',
  'components.transform.x',
  'components.transform.y',
  'components.velocity.x',
  'components.velocity.y',
  'components.size.width',
  'components.size.height',
  'components.controllable',
  'components.status.hitPoints',
  'components.status.maxHitPoints',
  'components.input.moveSpeed',
  'components.input.jumpImpulse',
  'components.cameraTarget.weight',
  'components.uiText.text',
];

function baseWorld() {
  // Mirrors the normalized live-entity shape produced by runtime.js
  // normalizeEntity for the collect-and-exit player plus a minimal wall.
  return {
    entities: [
      {
        id: 'player',
        sprite: { color: '#8be9fd', layer: 'default', order: 0, visible: true },
        components: {
          transform: { x: 32, y: 120 },
          velocity: { x: 0, y: 0 },
          size: { width: 12, height: 14 },
          controllable: true,
          status: { hitPoints: 3, maxHitPoints: 3 },
          input: { moveSpeed: 60, jumpImpulse: 140 },
          cameraTarget: { weight: 100 },
        },
      },
      {
        id: 'wall',
        sprite: { color: '#44475a', layer: 'default', order: 0, visible: true },
        components: {
          transform: { x: 0, y: 0 },
          velocity: { x: 0, y: 0 },
          size: { width: 320, height: 8 },
          controllable: false,
        },
      },
    ],
  };
}

function lookup(world, entityId, editPath) {
  const entity = world.entities.find((candidate) => candidate.id === entityId);
  const segments = editPath.split('.');
  let cursor = entity;
  for (const segment of segments) cursor = cursor[segment];
  return cursor;
}

let failures = 0;
function run(name, fn) {
  try {
    fn();
    console.log(`ok - ${name}`);
  } catch (error) {
    failures += 1;
    console.error(`not ok - ${name}`);
    console.error(`  ${error && error.message ? error.message : error}`);
  }
}

run('supported path list matches the Rust allowlist exactly', () => {
  assert.deepStrictEqual(
    [...previewChannel.supportedDeltaPaths].sort(),
    [...RUST_SUPPORTED_SCENE_EDIT_PATHS].sort()
  );
  assert.strictEqual(previewChannel.supportedDeltaPaths.length, 14);
});

run('fixture table covers every supported path at least once', () => {
  const covered = new Set();
  for (const fixture of fixtures.cases) {
    for (const edit of fixture.delta.edits || []) covered.add(edit.path);
  }
  for (const supported of RUST_SUPPORTED_SCENE_EDIT_PATHS) {
    assert.ok(covered.has(supported), `fixture table missing path ${supported}`);
  }
});

for (const fixture of fixtures.cases) {
  run(`fixture: ${fixture.name}`, () => {
    const world = baseWorld();
    const result = previewChannel.applyPreviewDelta(world, fixture.delta);
    if (fixture.expect.ok) {
      assert.strictEqual(result.ok, true, JSON.stringify(result));
      if (fixture.expect.requiresSceneReload) {
        assert.strictEqual(result.requiresSceneReload, true);
      }
      if (fixture.expect.skipped) {
        assert.strictEqual(result.skipped, fixture.expect.skipped);
      }
      if (Number.isInteger(fixture.expect.appliedEdits)) {
        assert.strictEqual(result.appliedEdits, fixture.expect.appliedEdits);
      }
      for (const check of fixture.expect.worldChecks || []) {
        assert.deepStrictEqual(
          lookup(world, check.entityId, check.path),
          check.value,
          `world check failed for ${check.entityId}.${check.path}`
        );
      }
    } else {
      assert.strictEqual(result.ok, false, JSON.stringify(result));
      assert.strictEqual(result.diagnostic, fixture.expect.diagnostic);
      assert.ok(
        result.error.includes(fixture.expect.errorIncludes),
        `error "${result.error}" should include "${fixture.expect.errorIncludes}"`
      );
      for (const check of fixture.expect.worldChecks || []) {
        assert.deepStrictEqual(
          lookup(world, check.entityId, check.path),
          check.value,
          `failed delta must leave world untouched: ${check.entityId}.${check.path}`
        );
      }
    }
  });
}

run('multi-edit deltas are atomic: a bad second edit reverts nothing because nothing committed', () => {
  const world = baseWorld();
  const result = previewChannel.applyPreviewDelta(world, {
    schemaVersion: 'ouroforge.preview-delta.v1',
    deltaId: 'preview-delta-atomic',
    sessionId: 's',
    intentId: 'i',
    sequence: 1,
    kind: 'entityTransform',
    status: 'applied',
    edits: [
      { entityId: 'player', path: 'components.transform.x', value: 99 },
      { entityId: 'player', path: 'components.transform.y', value: 'broken' },
    ],
    beforeSceneHash: { algorithm: 'fnv1a64', value: '0' },
    afterSceneHash: { algorithm: 'fnv1a64', value: '1' },
  });
  assert.strictEqual(result.ok, false);
  assert.strictEqual(world.entities[0].components.transform.x, 32, 'x must not have committed');
  assert.strictEqual(world.entities[0].components.transform.y, 120);
});

run('application is deterministic across identical worlds', () => {
  const delta = fixtures.cases.find((c) => c.name === 'applies entity transform x and y atomically').delta;
  const worldA = baseWorld();
  const worldB = baseWorld();
  previewChannel.applyPreviewDelta(worldA, delta);
  previewChannel.applyPreviewDelta(worldB, delta);
  assert.deepStrictEqual(worldA, worldB);
});

run('module never adds missing optional components (no interpretation)', () => {
  const world = baseWorld();
  const result = previewChannel.applyPreviewDelta(world, {
    schemaVersion: 'ouroforge.preview-delta.v1',
    deltaId: 'preview-delta-wall-input',
    sessionId: 's',
    intentId: 'i',
    sequence: 1,
    kind: 'parameterSet',
    status: 'applied',
    edits: [{ entityId: 'wall', path: 'components.input.moveSpeed', value: 5 }],
    beforeSceneHash: { algorithm: 'fnv1a64', value: '0' },
    afterSceneHash: { algorithm: 'fnv1a64', value: '1' },
  });
  assert.strictEqual(result.ok, false);
  assert.ok(result.error.includes('requires input component'));
  assert.strictEqual(world.entities[1].components.input, undefined);
});

if (failures > 0) {
  console.error(`\n${failures} preview-channel test(s) failed`);
  process.exit(1);
}
console.log('\npreview-channel contract tests passed');
