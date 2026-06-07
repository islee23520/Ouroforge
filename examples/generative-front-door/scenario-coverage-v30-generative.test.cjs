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
const doc = fs.readFileSync(path.join(root, matrix.wordingAudit.doc), 'utf8');
for (const a of matrix.wordingAudit.anchors) assert.ok(doc.includes(a));
console.log('scenario-coverage-v30 smoke passed');
