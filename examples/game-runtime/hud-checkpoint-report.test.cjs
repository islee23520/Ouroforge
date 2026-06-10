const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');

const result = spawnSync(process.execPath, [path.join(runtimeDir, 'hud-binding.test.cjs')], {
  cwd: repoRoot,
  encoding: 'utf8',
});
if (result.status !== 0) {
  process.stdout.write(result.stdout || '');
  process.stderr.write(result.stderr || '');
  process.exit(result.status || 1);
}
const lines = result.stdout.split(/\r?\n/).filter((line) => line.startsWith('{"label"'));
assert.equal(lines.length, 4, 'expected start/key-collected/gate-open/win samples');
const samples = lines.map((line) => JSON.parse(line));
const expectedScreenshots = new Map([
  ['start', 'screenshots/state-start.png'],
  ['key-collected', 'screenshots/state-key-collected.png'],
  ['gate-open', 'screenshots/state-gate-open.png'],
  ['win', 'screenshots/state-win.png'],
]);
const report = {
  schemaVersion: 'ouroforge.runtime-shell-hud-checkpoint-report.v1',
  issue: 2354,
  source: 'examples/game-runtime/hud-binding.test.cjs',
  apiBoundary: ['getWorldState', 'getEvents', 'setInput', 'step', 'loadScene', 'whenReady'],
  checkpoints: samples.map((sample) => ({
    label: sample.label,
    screenshot: expectedScreenshots.get(sample.label),
    tick: sample.tick,
    flags: sample.flags,
    hud: sample.hud,
    pass: Boolean(expectedScreenshots.get(sample.label))
      && sample.hud.tick === String(sample.tick)
      && sample.hud.player === 'Alive'
      && (sample.label !== 'win' || sample.hud.runState === 'Win'),
  })),
};
for (const checkpoint of report.checkpoints) {
  assert.ok(checkpoint.screenshot, `missing screenshot path for ${checkpoint.label}`);
  assert.equal(checkpoint.pass, true, `checkpoint ${checkpoint.label} failed HUD/world sample cross-check`);
}

if (process.env.OUROFORGE_WRITE_RUNS === '1') {
  const outDir = path.join(repoRoot, 'runs/session-f-2354');
  fs.mkdirSync(outDir, { recursive: true });
  fs.writeFileSync(path.join(outDir, 'world-samples.jsonl'), samples.map((sample) => JSON.stringify(sample)).join('\n') + '\n');
  fs.writeFileSync(path.join(outDir, 'hud-checkpoint-report.json'), `${JSON.stringify(report, null, 2)}\n`);
}
console.log(JSON.stringify(report, null, 2));
