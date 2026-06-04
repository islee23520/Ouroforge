const assert = require('assert');
const fs = require('fs');
const path = require('path');

const root = __dirname;
const readJson = (rel) => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
const valid = readJson('event-signal.valid.fixture.json');
assert.strictEqual(valid.schemaVersion, 'gameplay-event-signal-system.v1');
assert.strictEqual(valid.status, 'ready');
assert.deepStrictEqual(valid.events.map((event) => event.eventType), [
  'collision_contact',
  'item_collected',
  'flag_changed',
  'timer_elapsed',
  'input_action',
  'state_changed'
]);
for (const event of valid.events) {
  assert.match(event.id, /^[a-z0-9][a-z0-9-]*$/);
  assert.ok(event.source && Object.keys(event.source).length > 0);
  assert.ok(event.target && Object.keys(event.target).length > 0);
  assert.ok(Number.isInteger(event.tick));
  assert.ok(Number.isInteger(event.orderingIndex));
  assert.strictEqual(typeof event.consumed, 'boolean');
  const consumedBy = event.consumedBy || [];
  if (event.consumed) assert.ok(consumedBy.length > 0);
  if (!event.consumed) assert.equal(consumedBy.length, 0);
  assert.doesNotMatch(JSON.stringify(event), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
}

const partial = readJson('event-signal.partial.fixture.json');
assert.strictEqual(partial.status, 'partial');
assert.strictEqual(partial.events[0].consumed, false);

const blocked = readJson('event-signal.blocked.fixture.json');
assert.strictEqual(blocked.status, 'blocked');
assert.match(blocked.blockedReasons.join(' '), /target resolution evidence/);

const invalid = readJson('invalid/event-signal.invalid.fixture.json');
assert.match(JSON.stringify(invalid), /execute_script/);
assert.match(JSON.stringify(invalid), /plugin_loader/);
assert.equal(invalid.events[0].consumed, false);
assert.ok(invalid.events[0].consumedBy.length > 0);

const doc = fs.readFileSync(path.join(root, '../../docs/gameplay-event-signal-system-v1.md'), 'utf8');
assert.match(doc, /Issue: #613/);
assert.match(doc, /structured data contract/);
assert.match(doc, /collision_contact/);
assert.match(doc, /trigger_entered/);
assert.match(doc, /behavior_executed/);
assert.match(doc, /does not authorize arbitrary\s+JS\/Rust\/Python\/Lua\/WASM execution/);
assert.match(doc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(doc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(doc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);

console.log('gameplay event signal schema smoke passed');
