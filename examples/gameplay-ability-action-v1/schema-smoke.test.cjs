const assert = require('assert');
const fs = require('fs');
const path = require('path');
const root = __dirname;
const readJson = (rel) => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
const valid = readJson('ability-action.valid.fixture.json');
assert.equal(valid.schemaVersion, 'gameplay-ability-action.v1');
assert.equal(valid.status, 'ready');
assert.deepEqual(valid.abilities.map((ability) => ability.actionId), [
  'action-dash',
  'action-alert-strike',
  'action-open-blue-door',
  'action-hazard-pulse',
  'action-complete-win-state',
]);
for (const ability of valid.abilities) {
  assert.ok(ability.target && Object.keys(ability.target).length > 0);
  assert.ok(ability.trigger.kind);
  assert.ok(ability.effect.kind);
  assert.doesNotMatch(JSON.stringify(ability), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
}
const partial = readJson('ability-action.partial.fixture.json');
assert.equal(partial.status, 'partial');
assert.match(partial.abilities[0].blockedReasons[0], /GL10\.4\.3/);
const blocked = readJson('ability-action.blocked.fixture.json');
assert.equal(blocked.status, 'blocked');
assert.match(blocked.blockedReasons.join(' '), /GL10\.4\.3/);
const invalid = readJson('invalid/ability-action.invalid.fixture.json');
assert.match(JSON.stringify(invalid), /execute_script/);
assert.match(JSON.stringify(invalid), /plugin_loader/);
const doc = fs.readFileSync(path.join(root, '../../docs/gameplay-ability-action-v1.md'), 'utf8');
assert.match(doc, /Issue: #614/);
assert.match(doc, /structured data contract/);
assert.match(doc, /player dash, enemy alert attack, locked\/opened door/);
assert.match(doc, /does not authorize arbitrary JS\/Rust\/Python\/Lua\/WASM\s+execution/);
assert.match(doc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(doc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(doc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
console.log('gameplay ability action schema smoke passed');
