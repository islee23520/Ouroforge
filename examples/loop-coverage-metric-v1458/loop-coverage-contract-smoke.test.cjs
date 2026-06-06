const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '../..');
const docPath = path.join(root, 'docs/loop-coverage-metric-v1.md');
const doc = fs.readFileSync(docPath, 'utf8');

for (const phrase of [
  'Issue: **#1458**',
  'Seed → Build → Observe → Verify → Journal → Evolve',
  '`loop-produced`',
  '`loop-verified`',
  '`manual`',
  'schemaVersion',
  'loop-coverage-metric-v1',
  '`computed`',
  '`insufficient-data`',
  '`regressed`',
  '`unsupported`',
  'baselineLoopCovered - currentLoopCovered > dropThreshold',
  'Milestone 25',
  '#1460',
  '#1461',
  '#1462',
  '#1463',
  '#1464',
  '#1465',
  '#1 and #23 remain open',
]) {
  assert.ok(doc.includes(phrase), `missing required phrase: ${phrase}`);
}

const requiredSections = [
  /## Provenance classes/,
  /## Loop-coverage evidence artifact schema/,
  /## Coverage verdict states/,
  /## Regression rule/,
  /## Rust\/local ownership and UI boundaries/,
  /## Milestone 20 and Milestone 25 boundary/,
  /## Backward compatibility and generated state/,
  /## Follow-up dependency order and closure gates/,
  /## Wording audit/,
  /## Governance/,
];
for (const section of requiredSections) assert.match(doc, section);

const boundaryPatterns = [
  /adds no executable behavior/i,
  /no auto-fix/i,
  /no auto-apply/i,
  /no auto-merge/i,
  /Browser, dashboard, and Studio surfaces.*read-only/i,
  /Rust\/local validation owns metric computation/i,
  /Generated runs, coverage artifacts, traces.*remain untracked/i,
  /descriptive.*not.*quality/i,
  /full intent-to-promotion provenance bundle.*out of scope/i,
];
for (const pattern of boundaryPatterns) assert.match(doc, pattern);

const forbiddenClaims = [
  /loop coverage proves game quality/i,
  /loop coverage proves fun/i,
  /loop coverage is a production-ready metric/i,
  /loop coverage is a release readiness score/i,
  /loop coverage is Godot replacement evidence/i,
  /grants automatic fix\/apply\/merge authority/i,
  /grants browser trusted write authority/i,
];
const proseWithoutDisallowedList = doc.split('## Wording audit')[0];
for (const pattern of forbiddenClaims) assert.doesNotMatch(proseWithoutDisallowedList, pattern);

console.log('loop coverage metric v1458 contract smoke passed');
