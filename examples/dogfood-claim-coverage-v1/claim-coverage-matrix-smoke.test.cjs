const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const matrixPath = path.join(repoRoot, '.omx', 'dogfood-validation', 'claim-coverage-matrix.md');
const matrix = fs.readFileSync(matrixPath, 'utf8');

const requiredColumns = [
  'Claim ID',
  'Claim text',
  '#1 link',
  'Owner lane',
  'Evidence path',
  'Verdict',
  'Gap classification',
];

function normalize(value) {
  return value.replace(/\s+/g, ' ').trim();
}

function parseTable(markdown) {
  const lines = markdown.split(/\r?\n/).filter((line) => line.trim().startsWith('|'));
  const headerIndex = lines.findIndex((line) => requiredColumns.every((column) => line.includes(column)));
  assert.notEqual(headerIndex, -1, 'claim coverage matrix has the required table header');

  const headers = lines[headerIndex].split('|').slice(1, -1).map(normalize);
  const rows = [];
  for (const line of lines.slice(headerIndex + 2)) {
    const cells = line.split('|').slice(1, -1).map(normalize);
    if (cells.length !== headers.length || !cells[0]) continue;
    rows.push(Object.fromEntries(headers.map((header, index) => [header, cells[index] || ''])));
  }
  return rows;
}

const rows = parseTable(matrix);
assert.ok(rows.length >= 10, 'matrix records the main #1 and dogfood demo claims');

const allowed = new Set(['verified', 'unverified', 'deferred', 'non-goal']);
for (const row of rows) {
  for (const column of requiredColumns) {
    assert.ok(row[column], `${row['Claim ID'] || 'row'} includes ${column}`);
  }
  assert.match(row['Claim ID'], /^(OF|PROTECT)-\d{3}$/, `${row['Claim ID']} uses a stable claim id`);
  assert.match(row['#1 link'], /^https:\/\/github\.com\/shaun0927\/Ouroforge\/issues\/(1|23)$/, `${row['Claim ID']} links to #1 or protected #23`);
  assert.ok(allowed.has(row.Verdict), `${row['Claim ID']} uses an allowed verdict`);
  assert.ok(allowed.has(row['Gap classification']), `${row['Claim ID']} uses an allowed gap classification`);
  assert.ok(row['Owner lane'].length >= 3, `${row['Claim ID']} has an owner lane`);
  assert.ok(row['Evidence path'].length >= 3, `${row['Claim ID']} has an evidence path or explicit gap source`);
}

for (const row of rows.filter((entry) => entry.Verdict === 'verified')) {
  const firstEvidence = row['Evidence path'].split(';')[0].trim();
  if (!firstEvidence.startsWith('https://')) {
    assert.ok(fs.existsSync(path.join(repoRoot, firstEvidence)), `${row['Claim ID']} verified evidence path exists: ${firstEvidence}`);
  }
}

const m102Row = rows.find((row) => /M102.?M106|M102–M106/.test(row['Claim text']));
assert.ok(m102Row, 'matrix has a guard row for Era Q M102-M106');
assert.equal(m102Row.Verdict, 'deferred', 'Era Q M102-M106 are deferred');
assert.equal(m102Row['Gap classification'], 'deferred', 'Era Q M102-M106 gap classification is deferred');
assert.match(matrix, /#1 remains open/i, 'matrix explicitly protects #1 as open');
assert.match(matrix, /#23 remains open/i, 'matrix explicitly protects #23 as open');
assert.doesNotMatch(matrix, /(?:^|\n)\s*(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?)\s+#(?:1|23)\b/i, 'matrix does not use PR-closing keyword lines for #1 or #23');
assert.match(matrix, /Do not implement Era Q full-3D M102–M106/i, 'matrix records full-3D non-goal');

console.log('dogfood claim coverage matrix smoke passed');
