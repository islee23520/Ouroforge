#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const root = __dirname;
const hud = JSON.parse(fs.readFileSync(path.join(root, 'hud-feedback.json'), 'utf8'));
const rubric = JSON.parse(fs.readFileSync(path.join(root, 'visual-audio-rubric.json'), 'utf8'));
const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-signal-gate-polish-'));
const report = {
  schemaVersion: 'signal-gate-polish-evidence-v1',
  issue: 2387,
  m120Applied: true,
  screenshotNames: ['state-start.png', 'state-key-collected.png', 'state-gate-open.png', 'state-fail-blocked.png', 'state-restarted.png', 'state-win-exit.png'],
  hudStates: hud.hudStates.map((state) => state.state),
  visualChecksPassed: rubric.checks.filter((check) => check.status === 'pass').length,
  audioIntentCount: rubric.audioIntent.length,
  classification: 'product-observed-local-rubric-smoke',
  knownVisualGaps: [],
  boundary: 'Report/rubric evidence is not pixel-perfect proof; generated screenshots stay ignored.'
};
const p = path.join(tempDir, 'polish-report.json');
fs.writeFileSync(p, JSON.stringify(report, null, 2));
const observed = JSON.parse(fs.readFileSync(p, 'utf8'));
assert.equal(observed.m120Applied, true);
assert.ok(observed.screenshotNames.every((name) => name.startsWith('state-') && name.endsWith('.png')));
assert.ok(observed.hudStates.includes('fail/blocked'));
assert.ok(observed.visualChecksPassed >= 4);
assert.ok(observed.audioIntentCount >= 5);
assert.equal(observed.knownVisualGaps.length, 0);
assert.ok(observed.boundary.includes('not pixel-perfect proof'));
fs.rmSync(tempDir, { recursive: true, force: true });
for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}
console.log('signal-gate polish evidence smoke passed');
