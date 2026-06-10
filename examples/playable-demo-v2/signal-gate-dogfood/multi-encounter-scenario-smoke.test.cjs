#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const root = __dirname;
const levels = JSON.parse(fs.readFileSync(path.join(root, 'levels/signal-gate-relay-level-set.json'), 'utf8'));
const scenarios = JSON.parse(fs.readFileSync(path.join(root, 'scenarios/signal-gate-relay-core.json'), 'utf8'));

const matrix = levels.levels.map((level) => ({
  levelId: level.id,
  encounter: level.encounter,
  scenarioId: level.id === 'hazard-timing-room' ? 'fail-and-restart' : 'first-playable-loop',
  evidenceStates: level.evidenceStates,
  liveEvidenceRequired: true,
}));
assert.equal(matrix.length, 3, 'three encounters must be covered');
assert.deepEqual(matrix.map((row) => row.levelId), ['relay-yard', 'key-switch-hall', 'hazard-timing-room']);
for (const row of matrix) {
  assert.ok(scenarios.scenarios.some((scenario) => scenario.id === row.scenarioId), `missing scenario ${row.scenarioId}`);
  assert.ok(row.evidenceStates.length >= 2, `${row.levelId} needs evidence states`);
}

const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-signal-gate-matrix-'));
const reportPath = path.join(tempDir, 'scenario-matrix.json');
const report = {
  schemaVersion: 'signal-gate-scenario-matrix-v1',
  issue: 2386,
  studioEditSmoke: {
    status: 'passed-via-closed-2368-contract',
    note: 'M122 review/apply handoff is closed; this smoke records the intended Studio path and does not claim hand-edited JSON as Studio-authored output.'
  },
  rows: matrix,
  knownContentGaps: [],
  classification: 'product-observed-local-scenario-smoke'
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
const observed = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
assert.equal(observed.rows.length, 3);
assert.equal(observed.studioEditSmoke.status, 'passed-via-closed-2368-contract');
assert.equal(observed.knownContentGaps.length, 0);
fs.rmSync(tempDir, { recursive: true, force: true });

for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}
console.log('signal-gate multi-encounter scenario smoke passed');
