'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '../..');
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 'scenario-coverage-v30/matrix.fixture.json'), 'utf8'));
assert.equal(matrix.issue, 1597);
for (const obj of [matrix.intake, matrix.promotion, matrix.accessibility]) {
  for (const ref of Object.values(obj)) assert.ok(fs.existsSync(path.join(root, ref)));
}
const promotionPass = JSON.parse(fs.readFileSync(path.join(root, matrix.promotion.pass), 'utf8'));
const promotionGateFail = JSON.parse(fs.readFileSync(path.join(root, matrix.promotion.gateFail), 'utf8'));
const promotionOversolution = JSON.parse(fs.readFileSync(path.join(root, matrix.promotion.oversolution), 'utf8'));
assert.equal(promotionPass.briefId, 'brief-promotion-pass-v1');
assert.ok(promotionPass.intendedSolution.length > 1, 'promotion pass fixture declares an authored playable route');
assert.equal(promotionGateFail.briefId, 'brief-promotion-gate-fail-v1');
assert.deepEqual(promotionGateFail.intendedSolution, ['right']);
assert.ok(promotionOversolution.intendedSolution.length > 1, 'over-solution fixture declares a longer authored route');

const accessibilityVerified = JSON.parse(fs.readFileSync(path.join(root, matrix.accessibility.verified), 'utf8'));
const accessibilityUnverified = JSON.parse(fs.readFileSync(path.join(root, matrix.accessibility.unverified), 'utf8'));
assert.equal(accessibilityVerified.briefId, 'brief-accessibility-verified-v1');
assert.equal(accessibilityUnverified.briefId, 'brief-accessibility-unverified-v1');
assert.ok(accessibilityVerified.intendedSolution.length > 0);
assert.ok(accessibilityUnverified.intendedSolution.length > 0);
assert.notDeepEqual(accessibilityUnverified.intendedSolution, accessibilityVerified.intendedSolution);

const trustDoc = fs.readFileSync(path.join(root, matrix.backwardCompat.trustGradientDoc), 'utf8');
assert.match(trustDoc, /no auto-apply/);
assert.match(trustDoc, /review-gated/);
assert.match(trustDoc, /backward-compatible/);
const audit = JSON.parse(fs.readFileSync(path.join(root, matrix.backwardCompat.trustGradientAuditFixture), 'utf8'));
assert.equal(audit.schemaVersion, 'trust-gradient-audit-v1');
assert.equal(audit.entries.length, 2);
assert.equal(audit.killSwitch.engaged, false);
const doc = fs.readFileSync(path.join(root, matrix.wordingAudit.doc), 'utf8');
for (const a of matrix.wordingAudit.anchors) assert.ok(doc.includes(a));
console.log('scenario-coverage-v30 smoke passed');
