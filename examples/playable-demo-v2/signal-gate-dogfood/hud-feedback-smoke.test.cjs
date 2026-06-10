#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = __dirname;
const feedback = JSON.parse(fs.readFileSync(path.join(root, 'hud-feedback.json'), 'utf8'));
assert.equal(feedback.schemaVersion, 'signal-gate-hud-feedback-v1');
assert.equal(feedback.issue, 2387);
assert.deepEqual(feedback.hudStates.map((state) => state.state), ['start', 'relay-1', 'gate-open', 'fail/blocked', 'win/exit']);
for (const state of feedback.hudStates) {
  assert.ok(state.objective && state.message, `${state.state} needs objective/message`);
  assert.ok(Object.hasOwn(state, 'relay') && Object.hasOwn(state, 'key') && Object.hasOwn(state, 'gate'));
}
assert.equal(feedback.feedbackEvents.length, 5);
assert.ok(feedback.feedbackEvents.every((event) => event.intent && event.observable));
assert.ok(feedback.boundary.includes('no automated fun verdict'));
for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}
console.log('signal-gate hud feedback smoke passed');
