#!/usr/bin/env node
const fs = require('node:fs');

const taxonomy = JSON.parse(fs.readFileSync('docs/product-gap-taxonomy.json', 'utf8'));
const ledger = JSON.parse(fs.readFileSync('docs/product-observed/collect-and-exit-live-audit-gap-ledger.json', 'utf8'));
const coverage = fs.readFileSync('docs/scenario-coverage-v98-current-live-audit.md', 'utf8');
const report = fs.readFileSync('docs/product-observed/collect-and-exit-live-audit-2351.md', 'utf8');

const categories = new Set(taxonomy.categoryEnum.map((entry) => entry.id));
const severities = new Set(taxonomy.severityEnum.map((entry) => entry.id));
const requiredFields = taxonomy.findingRequiredFields;
const allowedMilestones = new Set(Array.from({ length: 13 }, (_, index) => `M${118 + index}`));
const requiredRows = [
  'v98-live-bundle-layout',
  'v98-rendered-product-fail',
  'v98-screenshot-evidence',
  'v98-replay-state-samples',
  'v98-gap-taxonomy-enforced',
  'v98-gap-owner-milestones',
  'v98-no-soft-pass',
  'v98-generated-state-clean',
];

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

assert(ledger.schemaVersion === 'ouroforge.product-observed-gap-ledger.v1', 'unexpected ledger schema');
assert(ledger.issue === 2351, 'ledger must belong to #2351');
assert(ledger.taxonomySource === 'docs/product-gap-taxonomy.json', 'ledger must use M117.1 taxonomy source');
assert(ledger.classification === 'contract-pass / product-observed FAIL', 'ledger must preserve product-observed failure');
assert(ledger.generatedBundleRoot === 'runs/live-observability/collect-and-exit-2351-live-audit/', 'ledger must preserve M116.1 bundle path');
assert(Array.isArray(ledger.findings) && ledger.findings.length >= 5, 'ledger must include current audit findings');
assert(ledger.boundaryReview?.noSoftPass === true, 'ledger must forbid soft pass');

for (const finding of ledger.findings) {
  for (const field of requiredFields) {
    assert(Object.hasOwn(finding, field), `${finding.id ?? '<unknown>'} missing required field ${field}`);
  }
  assert(categories.has(finding.category), `${finding.id} uses unknown category ${finding.category}`);
  assert(severities.has(finding.severity), `${finding.id} uses unknown severity ${finding.severity}`);
  assert(allowedMilestones.has(finding.owningMilestone), `${finding.id} maps outside M118-M130`);
  assert(/^#23\d{2}-#23\d{2}$/.test(finding.owningIssueRange), `${finding.id} missing owning issue range`);
  assert(Array.isArray(finding.evidenceRefs) && finding.evidenceRefs.length > 0, `${finding.id} needs evidence refs`);
}

for (const row of requiredRows) {
  assert(coverage.includes(row), `missing v98 row ${row}`);
}

assert(report.includes('product-observed FAIL'), 'report must state product-observed FAIL');
assert(report.includes('missing_asset'), 'report must preserve missing_asset diagnostic');
assert(report.includes('#1 and #23 remain open'), 'report must preserve governance anchors');
assert(!fs.existsSync('docs/roadmap.md.__v98_touch_marker'), 'v98 must not mutate roadmap via marker');

console.log(`Scenario Coverage v98 audit ledger ok: ${ledger.findings.length} findings`);
