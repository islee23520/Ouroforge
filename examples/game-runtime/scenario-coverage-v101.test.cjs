const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const path = require('node:path');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const cases = [
  { id: 'v101-visual-rubric-contract', script: 'visual-rubric.test.cjs' },
  { id: 'v101-four-state-rubric-report', script: 'visual-rubric-report.test.cjs', json: true },
  { id: 'v101-regression-capture-report', script: 'visual-regression-capture.test.cjs', json: true },
  { id: 'v101-m118-runtime-shell-regression', script: 'scenario-coverage-v99.test.cjs', json: true },
];
const results = cases.map((testCase) => {
  const result = spawnSync(process.execPath, [path.join(runtimeDir, testCase.script)], { cwd: repoRoot, encoding: 'utf8' });
  const record = {
    id: testCase.id,
    script: `examples/game-runtime/${testCase.script}`,
    status: result.status === 0 ? 'passed' : 'failed',
    stderrTail: (result.stderr || '').trim().split(/\r?\n/).filter(Boolean).slice(-4),
  };
  if (testCase.json && result.status === 0) record.report = JSON.parse(result.stdout);
  else record.stdoutTail = (result.stdout || '').trim().split(/\r?\n/).slice(-4);
  return record;
});
const report = {
  schemaVersion: 'ouroforge.scenario-coverage.v101',
  issue: 2361,
  milestone: 'M120 visual quality and regression capture',
  cases: results,
  allPassed: results.every((result) => result.status === 'passed'),
  capturePolicy: 'Generated screenshots/reports remain under ignored runs/; committed baselines require documented rendering settings.',
};
assert.equal(report.allPassed, true, JSON.stringify(report, null, 2));
const capture = results.find((result) => result.id === 'v101-regression-capture-report').report;
assert.equal(capture.comparison.passed, true);
assert.ok(capture.captures.every((entry) => entry.generatedPath.startsWith('runs/session-f-2361/screenshots/')));
console.log(JSON.stringify(report, null, 2));
