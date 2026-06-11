#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const fixturePath = path.join(root, 'playtest-pacing-triage.fixture.json');
const evidenceDoc = path.join(root, '..', '..', '..', 'docs', 'evidence', 'playtest-pacing-triage-signal-gate-2496.md');

const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
assert.equal(fixture.schemaVersion, 'signal-gate-playtest-pacing-template-v1');
assert.equal(fixture.issue, 2496);
assert.equal(fixture.classification, 'contract-complete');
assert.equal(fixture.status, 'template-prepared-awaiting-human-playtest');
assert.match(fixture.liveEntryPointUrl, /127\.0\.0\.1:8896/);
assert.match(fixture.localServerCommand, /http\.server 8896/);

const template = fixture.observationTemplate;
assert.equal(template.schemaVersion, 'ouroforge.playtest-capture-template.v1');
assert.equal(template.playtester.humanConfirmed, false);
assert.equal(template.requiredFieldsBeforeClosure.humanFunFeelNotes, 'TODO-human-owned-evidence-not-automated-score');
assert.equal(template.requiredFieldsBeforeClosure.mechanicalVerdict, 'TODO-separate-from-fun-feel');
assert.equal(template.trustedWriteRequested, false);
assert.equal(template.releaseAuthority, false);

const backlog = fixture.playtestGapBacklog;
const pacing = backlog.findings.find((finding) => finding.findingId === 'pacing-blocker');
assert.ok(pacing, 'pacing-blocker remains visible');
assert.equal(pacing.status, 'awaiting-human-playtest');
assert.equal(pacing.blocksProductObserved, true);
assert.equal(pacing.ownerIssue, '#2496');
assert.equal(backlog.readModel.closureAllowed, false);
assert.equal(backlog.readModel.blockingDeferredCount, 1);

assert.equal(fixture.verdictSeparation.funFeel.status, 'awaiting-human-evidence');
assert.equal(fixture.verdictSeparation.funFeel.releaseGoNoGo, 'not-recorded');
assert.ok(fixture.guardrails.includes('do not close #2496 until human playtest evidence exists'));

const doc = fs.readFileSync(evidenceDoc, 'utf8');
assert.ok(doc.includes('Closure classification: contract-complete'));
assert.ok(doc.includes('Handoff request for the user'));
assert.ok(doc.includes('I will skip closing #2496'));

for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked`);
}

console.log('signal-gate playtest pacing template smoke passed');
