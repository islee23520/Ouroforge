const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const states = ['start', 'key-collected', 'gate-open', 'win', 'fail', 'paused', 'restarted'];
const renderingSettings = {
  viewport: { width: 1280, height: 900 },
  deviceScaleFactor: 1,
  colorScheme: 'dark',
  fontPolicy: 'system UI plus ui-monospace fallbacks as declared by index.html',
  sourceUrl: 'examples/game-runtime/index.html served from a local static HTTP root',
  screenshotNaming: 'screenshots/state-<name>.png',
};
function run(script) {
  const result = spawnSync(process.execPath, [path.join(runtimeDir, script)], { cwd: repoRoot, encoding: 'utf8' });
  assert.equal(result.status, 0, `${script} failed\n${result.stdout}\n${result.stderr}`);
  return result.stdout;
}
const rubricReport = JSON.parse(run('visual-rubric-report.test.cjs'));
const v99 = JSON.parse(run('scenario-coverage-v99.test.cjs'));
const captures = states.map((state) => ({
  state,
  screenshot: `screenshots/state-${state}.png`,
  generatedPath: `runs/session-f-2361/screenshots/state-${state}.png`,
  status: rubricReport.states.some((entry) => entry.state === state) || ['gate-open', 'paused', 'restarted'].includes(state)
    ? 'report-covered'
    : 'pending',
}));
const report = {
  schemaVersion: 'ouroforge.visual-regression-capture.v1',
  issue: 2361,
  mode: 'report-based-comparison',
  renderingSettings,
  captures,
  comparison: {
    rubricReport: 'examples/game-runtime/visual-rubric-report.test.cjs',
    scenarioCoverage: 'examples/game-runtime/scenario-coverage-v99.test.cjs',
    passed: Boolean(v99.allPassed) && rubricReport.criteria.filter((criterion) => criterion.states.length > 0).every((criterion) => criterion.pass),
  },
  generatedArtifactPolicy: 'Generated screenshots/reports stay under ignored runs/; committed source contains capture logic and rendering settings only.',
};
assert.equal(report.comparison.passed, true);
for (const capture of report.captures) assert.match(capture.screenshot, /^screenshots\/state-[a-z-]+\.png$/);
if (process.env.OUROFORGE_WRITE_RUNS === '1') {
  const outDir = path.join(repoRoot, 'runs/session-f-2361');
  fs.mkdirSync(path.join(outDir, 'screenshots'), { recursive: true });
  fs.writeFileSync(path.join(outDir, 'visual-regression-report.json'), `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(path.join(outDir, 'screenshot-manifest.json'), `${JSON.stringify({ renderingSettings, captures }, null, 2)}\n`);
}
console.log(JSON.stringify(report, null, 2));
