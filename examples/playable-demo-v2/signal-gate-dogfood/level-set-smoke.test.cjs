#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const levelSet = JSON.parse(fs.readFileSync(path.join(root, 'levels/signal-gate-relay-level-set.json'), 'utf8'));
assert.equal(levelSet.schemaVersion, 'signal-gate-level-set-v1');
assert.equal(levelSet.issue, 2386);
assert.equal(levelSet.studioEditSmoke.blockedBy2368, 'closed');
assert.ok(levelSet.studioEditSmoke.path.includes('M122'));
assert.ok(levelSet.levels.length >= 3, 'at least three encounters are required');
assert.deepEqual(levelSet.levels.map((level) => level.id), ['relay-yard', 'key-switch-hall', 'hazard-timing-room']);
assert.equal(levelSet.progressionSummary.encounterCount, 3);
assert.deepEqual(levelSet.progression.map((edge) => edge.to), ['key-switch-hall', 'hazard-timing-room', 'final-signal-gate']);
for (const level of levelSet.levels) {
  assert.ok(level.targetDurationSeconds >= 60, `${level.id} target duration too short`);
  assert.ok(level.sceneRef.startsWith('scenes/'));
  assert.ok(level.objectives.length >= 2);
  assert.ok(level.evidenceStates.every((name) => name.startsWith('state-') && name.endsWith('.png')));
}
assert.deepEqual(levelSet.progression[0].requires, ['relay_1_active']);
for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}
console.log('signal-gate level set smoke passed');
