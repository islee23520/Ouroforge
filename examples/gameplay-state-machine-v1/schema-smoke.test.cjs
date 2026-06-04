const assert = require('assert');
const fs = require('fs');
const path = require('path');
const root = __dirname;
const readJson = (rel) => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
const valid = readJson('state-machine.valid.fixture.json');
assert.equal(valid.schemaVersion, 'gameplay-state-machine.v1');
assert.equal(valid.status, 'ready');
assert.deepEqual(valid.stateMachines.map((machine) => machine.id), ['player-dash-state', 'guard-alert-state', 'door-lock-state']);
for (const machine of valid.stateMachines) {
  assert.ok(machine.target && Object.keys(machine.target).length > 0);
  assert.ok(machine.states.some((state) => state.id === machine.initialStateId));
  for (const transition of machine.transitions) {
    assert.ok(machine.states.some((state) => state.id === transition.from));
    assert.ok(machine.states.some((state) => state.id === transition.to));
  }
  assert.doesNotMatch(JSON.stringify(machine), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
}
const partial = readJson('state-machine.partial.fixture.json');
assert.equal(partial.status, 'partial');
assert.match(partial.stateMachines[0].blockedReasons[0], /GL10\.4\.2/);
const blocked = readJson('state-machine.blocked.fixture.json');
assert.equal(blocked.status, 'blocked');
assert.match(blocked.blockedReasons.join(' '), /GL10\.4\.3/);
const invalid = readJson('invalid/state-machine.invalid.fixture.json');
assert.match(JSON.stringify(invalid), /execute_script/);
assert.match(JSON.stringify(invalid), /plugin_loader/);
const doc = fs.readFileSync(path.join(root, '../../docs/gameplay-state-machine-v1.md'), 'utf8');
assert.match(doc, /Issue: #614/);
assert.match(doc, /structured data contract/);
assert.match(doc, /player dash readiness, enemy patrol\/alert, locked\s+and opened door state/);
assert.match(doc, /does not authorize arbitrary JS\/Rust\/Python\/Lua\/WASM\s+execution/);
assert.match(doc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(doc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(doc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
console.log('gameplay state machine schema smoke passed');
