const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const doc = fs.readFileSync(path.join(__dirname, 'visual-readability-rubric.md'), 'utf8');
const states = ['start', 'key-collected', 'gate-open', 'win', 'fail', 'paused', 'restarted'];
for (const state of states) {
  assert.ok(doc.includes(`\`${state}\``), `missing state ${state}`);
  assert.ok(doc.includes(`screenshots/state-${state}.png`), `missing screenshot path for ${state}`);
}
const criteria = [...doc.matchAll(/\| VR-\d{2} /g)].length;
assert.ok(criteria >= 10, `expected at least 10 criteria, saw ${criteria}`);
for (const line of doc.split('\n').filter((line) => /^\| VR-\d{2}/.test(line))) {
  assert.match(line, /`(start|key-collected|gate-open|win|fail|paused|restarted)`/, `criterion lacks named state: ${line}`);
  assert.doesNotMatch(line, /commercial|AAA|production-ready/i, `criterion overclaims: ${line}`);
}
console.log('visual readability rubric contract test passed');
