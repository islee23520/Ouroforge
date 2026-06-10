#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = __dirname;
const gate = JSON.parse(fs.readFileSync(path.join(root, 'gate.fixture.json'), 'utf8'));
assert.equal(gate.schemaVersion, 'production-usability-gate-v1');
assert.equal(gate.scenarioCoverageSuite, 'scenario-coverage-v111');
assert.deepEqual(gate.phases.map((phase) => phase.issue), [2391, 2392, 2393, 2394]);
const byIssue = new Map(gate.phases.map((phase) => [phase.issue, phase]));
assert.ok(byIssue.get(2391).workflowTranscriptRefs.length > 0, '#2391 needs transcript refs');
assert.ok(byIssue.get(2391).screenshotRefs.length > 0, '#2391 needs screenshot refs');
assert.ok(byIssue.get(2391).manualGaps.length > 0, '#2391 manual steps must be ledgered');
assert.ok(
  byIssue.get(2391).screenshotRefs.some((ref) => ref.includes('m130-2391-signal-gate-win-2499/screenshots/final.png')),
  '#2391 needs the #2499 win-state browser screenshot ref'
);
assert.ok(
  byIssue.get(2391).manualGaps.every((gap) => gap.gapId !== 'm130-2391-win-state-browser-screenshot'),
  '#2499 resolves the stale #2391 win-state browser screenshot gap'
);
assert.match(byIssue.get(2392).comparisonVerdict, /^(improvement|regression)$/);
assert.ok(byIssue.get(2393).packageRefs.every((ref) => ref.startsWith('dist/local-web/')));
assert.ok(gate.guardrails.some((guardrail) => guardrail.includes('no new distribution scope')));
assert.ok(byIssue.get(2394).governanceRefs.some((ref) => ref.includes('#1')));
assert.deepEqual(gate.anchorsRemainOpen, [1, 23]);
for (const forbidden of ['native export', 'store upload', 'public release automation']) {
  const packageFixture = fs.readFileSync(path.join(root, 'local-package-provenance.fixture.json'), 'utf8');
  assert.ok(packageFixture.includes(forbidden), `package fixture must preserve ${forbidden} non-goal`);
}
console.log('production usability gate v111 smoke passed');
