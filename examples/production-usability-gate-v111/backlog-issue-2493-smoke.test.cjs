#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '..', '..');
const gateDir = path.join(root, 'examples', 'production-usability-gate-v111');
const gate = JSON.parse(fs.readFileSync(path.join(gateDir, 'gate.fixture.json'), 'utf8'));
const phase2391 = gate.phases.find((phase) => phase.issue === 2391);
assert.ok(phase2391, 'gate fixture must include phase 2391');

const evidence2499Md = 'docs/evidence/signal-gate-win-state-browser-screenshot-2499.md';
const evidence2499Json = 'docs/evidence/signal-gate-win-state-browser-screenshot-2499.json';
const evidence2493Md = 'docs/evidence/backlog-issue-2493-signal-gate-win-evidence.md';
const evidence2493Json = 'docs/evidence/backlog-issue-2493-signal-gate-win-evidence.json';

for (const ref of [evidence2499Md, evidence2499Json, evidence2493Md, evidence2493Json]) {
  assert.ok(fs.existsSync(path.join(root, ref)), `tracked evidence path must exist: ${ref}`);
}

const refs2391 = [...phase2391.evidenceRefs, ...phase2391.screenshotRefs];
assert.ok(
  refs2391.some((ref) => ref.includes('m130-2391-signal-gate-win-2499')),
  'phase 2391 must reference #2499 run root paths'
);
assert.ok(
  phase2391.evidenceRefs.includes(evidence2499Md),
  'phase 2391 evidenceRefs must include #2499 stable md'
);
assert.ok(
  phase2391.evidenceRefs.includes(evidence2493Md),
  'phase 2391 evidenceRefs must include #2493 backlog ledger md'
);

const replayLabels = ['start', 'relay-1', 'key-gate', 'win-exit'];
const md2493 = fs.readFileSync(path.join(root, evidence2493Md), 'utf8');
const json2493 = fs.readFileSync(path.join(root, evidence2493Json), 'utf8');
const runner = fs.readFileSync(path.join(root, 'tools/live-observability-runner/runner.mjs'), 'utf8');
for (const label of replayLabels) {
  assert.ok(md2493.includes(label), `#2493 md must document replay label ${label}`);
  assert.ok(json2493.includes(label), `#2493 json must list replay label ${label}`);
  assert.ok(runner.includes(label), `runner must define replay label ${label}`);
}

assert.match(md2493, /Closure classification:/, '#2493 md must include Closure classification line');
assert.deepEqual(gate.anchorsRemainOpen, [1, 23], 'gate fixture must keep governance anchors open');

console.log('backlog issue 2493 signal gate win evidence smoke passed');