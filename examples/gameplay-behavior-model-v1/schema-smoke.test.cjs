const assert = require('assert');
const fs = require('fs');
const path = require('path');

const root = __dirname;
const readJson = (rel) => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));

const valid = readJson('behavior-model.valid.fixture.json');
assert.strictEqual(valid.schemaVersion, 'gameplay-behavior-model.v1');
assert.strictEqual(valid.status, 'ready');
assert.strictEqual(valid.behaviors.length, 7);
assert.deepStrictEqual(
  valid.behaviors.map((behavior) => behavior.id),
  [
    'patrol-guard-route-a',
    'collect-keycard',
    'spike-damage-contact',
    'open-blue-door',
    'win-after-exit',
    'timed-hazard-pulse',
    'dash-ability-trigger'
  ]
);
for (const behavior of valid.behaviors) {
  assert.match(behavior.id, /^[a-z0-9][a-z0-9-]*$/);
  assert.ok(behavior.target && typeof behavior.target === 'object');
  assert.ok(Array.isArray(behavior.triggers));
  assert.ok(Array.isArray(behavior.conditions));
  assert.ok(Array.isArray(behavior.actions));
  assert.ok(Array.isArray(behavior.evidenceRefs));
  assert.ok(Array.isArray(behavior.blockedReasons));
  const serialized = JSON.stringify(behavior);
  assert.doesNotMatch(serialized, /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
}

const partial = readJson('behavior-model.partial.fixture.json');
assert.strictEqual(partial.status, 'partial');
assert.match(partial.behaviors[0].blockedReasons[0], /#614/);

const blocked = readJson('behavior-model.blocked.fixture.json');
assert.strictEqual(blocked.status, 'blocked');
assert.match(blocked.blockedReasons.join(' '), /script execution remains unauthorized/);

const invalid = readJson('invalid/behavior-model.invalid.fixture.json');
assert.strictEqual(invalid.status, 'invalid');
assert.match(JSON.stringify(invalid), /execute_script/);
assert.match(JSON.stringify(invalid), /plugin_loader/);
assert.match(invalid.blockedReasons.join(' '), /negative fixture/);

const doc = fs.readFileSync(path.join(root, '../../docs/gameplay-behavior-model-v1.md'), 'utf8');
assert.match(doc, /Issue: #612/);
assert.match(doc, /data-first contract/);
assert.match(doc, /patrol, collect item, damage on contact, door opens\s+on flag, win condition, timed hazard, and simple ability trigger/);
assert.match(doc, /must not contain executable script bodies, `eval`, dynamic\s+imports, plugin loader instructions, command strings/);
assert.match(doc, /GL10\.2\.2 should validate duplicate ids, unsafe targets, unsupported actions/);
assert.match(doc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(doc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(doc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);

console.log('gameplay behavior model schema smoke passed');
