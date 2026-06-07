'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '../..');
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 'scenario-coverage-v29/matrix.fixture.json'), 'utf8'));
assert.equal(matrix.schemaVersion, 'scenario-coverage-v29-design-regression-matrix-v1');
assert.equal(matrix.issue, 1590);
for (const ref of [matrix.backwardCompat.spec, matrix.wordingAudit.doc]) {
  assert.ok(fs.existsSync(path.join(root, ref)), `repo ref ${ref}`);
}
const doc = fs.readFileSync(path.join(root, matrix.wordingAudit.doc), 'utf8');
for (const anchor of matrix.wordingAudit.anchors) {
  assert.ok(doc.includes(anchor), `doc includes ${anchor}`);
}
console.log('scenario-coverage-v29 fixture smoke passed');
