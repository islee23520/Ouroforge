'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '../..');
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 'scenario-coverage-v27/matrix.fixture.json'), 'utf8'));
assert.equal(matrix.issue, 1577);
for (const key of ['valid','malformed','unsupported']) {
  assert.ok(fs.existsSync(path.join(root, matrix.dslFixtures[key])));
}
const doc = fs.readFileSync(path.join(root, matrix.wordingAudit.doc), 'utf8');
for (const a of matrix.wordingAudit.anchors) assert.ok(doc.includes(a));
console.log('scenario-coverage-v27 fixture smoke passed');
