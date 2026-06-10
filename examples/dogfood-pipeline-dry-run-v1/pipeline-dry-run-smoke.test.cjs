const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const reportPath = path.join(repoRoot, '.omx', 'dogfood-validation', 'pipeline-dry-run.md');
const report = fs.readFileSync(reportPath, 'utf8');
const normalized = report.replace(/\s+/g, ' ').trim();

function mustContain(text, label) {
  assert.ok(normalized.includes(text.replace(/\s+/g, ' ')), label || `contains ${text}`);
}

for (const repoPath of [
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
]) {
  assert.ok(fs.existsSync(path.join(repoRoot, repoPath)), `referenced merged artifact exists: ${repoPath}`);
}

for (const section of [
  'Metadata',
  'Commands executed',
  'Evidence summary',
  'Classified failure details',
  'Non-goals and guardrails',
]) {
  assert.match(report, new RegExp(`## ${section}`), `report includes ${section}`);
}

for (const required of [
  'dogfood-pipeline-dry-run-v1',
  'dogfood-demo-spec-v1',
  'dogfood-claim-coverage-v1',
  'seed validate',
  'project validate',
  'scenario-pack collect-and-exit',
  'evaluate runs/run-',
  'journal show runs/run-',
  'mutation list runs/run-',
  'mutation review --defer',
  'compare runs/run-',
  'dashboard export --runs-root runs --output dashboard-data.json',
  'Seed valid: playable-demo.collect-and-exit',
  'Project manifest valid: collect_and_exit_demo',
  'Run project bound: collect_and_exit_demo',
  'failed-classified',
  '6 failure(s) found across 1 scenario result(s)',
  'review-decision-1',
  'No mutation apply was executed',
]) {
  mustContain(required, `report records ${required}`);
}

for (const rowLabel of [
  'Seed/spec validation',
  'Project validation/binding',
  'Run evidence and run ID',
  'Evaluator/verdict output',
  'Journal output',
  'Mutation/proposal output',
  'Replay/regression comparison',
  'Generated-state cleanup/retention',
]) {
  assert.match(report, new RegExp(`\\| ${rowLabel.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')} \\|`), `summary table has ${rowLabel}`);
}

for (const guardrail of [
  /#1 and #23 remain open/i,
  /No Era Q full-3D M102–M106 implementation or activation/i,
  /No hosted\/cloud\/multi-user scope/i,
  /No trusted browser\/source writes/i,
  /No auto-port, live bridge, foreign runtime embedding/i,
  /No release automation, signing, upload, publishing/i,
  /No mutation apply was executed/i,
]) {
  assert.match(report, guardrail, `guardrail present: ${guardrail}`);
}

for (const forbidden of [
  /M102(?:–|-| to )M106\s+(?:active|implemented|complete|ready)/i,
  /production-ready/i,
  /store-ready/i,
  /trusted browser writes are allowed/i,
  /(^|\n)mutation apply was executed/i,
]) {
  assert.doesNotMatch(report, forbidden, `forbidden affirmative absent: ${forbidden}`);
}

console.log('dogfood pipeline dry-run report smoke passed');
