#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = __dirname;
const rubric = JSON.parse(fs.readFileSync(path.join(root, 'visual-audio-rubric.json'), 'utf8'));
assert.equal(rubric.schemaVersion, 'signal-gate-visual-audio-rubric-v1');
assert.equal(rubric.issue, 2387);
assert.ok(rubric.m120Rubric.includes('visual-readability-rubric.md'));
assert.equal(rubric.viewport.width, 480);
assert.ok(Object.keys(rubric.tokens).length >= 8);
assert.ok(rubric.checks.length >= 4);
assert.ok(rubric.checks.every((check) => check.status === 'pass' && check.evidence));
assert.ok(rubric.audioIntent.includes('hazard_fail'));
assert.ok(rubric.boundary.includes('not pixel-perfect proof'));
for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}
console.log('signal-gate visual/audio rubric smoke passed');
