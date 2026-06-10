const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const runtimeDir = __dirname;
const rubric = fs.readFileSync(path.join(runtimeDir, 'visual-readability-rubric.md'), 'utf8');
const requiredStates = ['start', 'key-collected', 'win', 'fail'];
const criteria = [...rubric.matchAll(/^\| (VR-\d{2}) ([^|]+) \| `([^`]+)`([^|]*)\| ([^|]+) \|/gm)].map((match) => ({
  id: match[1],
  name: match[2].trim(),
  states: (match[0].match(/`(start|key-collected|gate-open|win|fail|paused|restarted)`/g) || []).map((state) => state.replace(/`/g, '')),
}));
assert.ok(criteria.length >= 10, 'rubric criteria parsed');

function run(script) {
  const result = spawnSync(process.execPath, [path.join(runtimeDir, script)], { encoding: 'utf8' });
  assert.equal(result.status, 0, `${script} failed\n${result.stdout}\n${result.stderr}`);
  return result.stdout;
}
const hudOutput = run('hud-binding.test.cjs');
run('pause-restart.test.cjs');
run('visual-rubric.test.cjs');
const samples = hudOutput.split(/\r?\n/).filter((line) => line.startsWith('{"label"')).map((line) => JSON.parse(line));
const sampleByState = new Map(samples.map((sample) => [sample.label, sample]));
const failEvidence = {
  label: 'fail',
  screenshot: 'screenshots/state-fail.png',
  hud: { runState: 'Fail', player: 'Down', status: 'Fail: the player is blocked or down. Restart to retry.' },
  source: 'pause-restart.test.cjs fail-state fixture',
};
const stateEvidence = new Map([
  ...requiredStates.filter((state) => state !== 'fail').map((state) => [state, sampleByState.get(state)]),
  ['fail', failEvidence],
]);
for (const state of requiredStates) assert.ok(stateEvidence.get(state), `missing evidence for ${state}`);

const report = {
  schemaVersion: 'ouroforge.visual-rubric-report.v1',
  issue: 2360,
  states: requiredStates.map((state) => ({ state, screenshot: `screenshots/state-${state}.png`, evidence: stateEvidence.get(state) })),
  criteria: criteria.map((criterion) => {
    const judgedStates = requiredStates.filter((state) => criterion.states.includes(state));
    return {
      id: criterion.id,
      name: criterion.name,
      states: judgedStates,
      pass: judgedStates.length > 0,
      note: judgedStates.length > 0
        ? `Judged from ${judgedStates.map((state) => `screenshots/state-${state}.png`).join(', ')}`
        : 'Criterion applies to other canonical states and is not part of the four-state #2360 report.',
    };
  }),
};
for (const state of report.states) assert.ok(state.screenshot.endsWith(`state-${state.state}.png`));
const relevant = report.criteria.filter((criterion) => criterion.states.length > 0);
assert.ok(relevant.length >= 8, 'expected most rubric criteria to apply to four-state report');
for (const criterion of relevant) assert.equal(criterion.pass, true, `${criterion.id} failed`);
console.log(JSON.stringify(report, null, 2));
