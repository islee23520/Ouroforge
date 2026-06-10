const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const path = require('node:path');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const cases = [
  { id: 'v99-shell-hud-checkpoints', script: 'hud-binding.test.cjs' },
  { id: 'v99-pause-restart-win-fail', script: 'pause-restart.test.cjs' },
  { id: 'v99-runtime-shell-dom', script: 'game-ui.test.cjs' },
  { id: 'v99-collect-and-exit-runtime', script: 'playable-demo-v2.test.cjs' },
];
const results = [];
for (const testCase of cases) {
  const result = spawnSync(process.execPath, [path.join(runtimeDir, testCase.script)], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  results.push({
    id: testCase.id,
    script: `examples/game-runtime/${testCase.script}`,
    status: result.status === 0 ? 'passed' : 'failed',
    stdoutTail: (result.stdout || '').trim().split(/\r?\n/).slice(-4),
    stderrTail: (result.stderr || '').trim().split(/\r?\n/).filter(Boolean).slice(-4),
  });
}
const report = {
  schemaVersion: 'ouroforge.scenario-coverage.v99',
  issue: 2355,
  milestone: 'M118 runtime shell UX',
  closureClassification: 'contract-complete-with-deterministic-runtime-evidence',
  cases: results,
  allPassed: results.every((result) => result.status === 'passed'),
  generatedArtifactsPolicy: 'No trusted source writes; live screenshots/world samples stay under ignored runs/ bundles.',
  apiGap: 'False initial gameplayRules flags may be absent from componentModel.goalFlags; #2357 records the M119.2 materialization decision.',
};
assert.equal(report.allPassed, true, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
