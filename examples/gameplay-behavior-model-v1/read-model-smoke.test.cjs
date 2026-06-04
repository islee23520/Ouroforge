const assert = require('assert');
const fs = require('fs');
const path = require('path');

const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, 'behavior-model.valid.fixture.json'), 'utf8'));
const behaviorIds = fixture.behaviors.map((behavior) => behavior.id);
const triggerKinds = [...new Set(fixture.behaviors.flatMap((behavior) => behavior.triggers.map((trigger) => trigger.kind)))].sort();
const conditionKinds = [...new Set(fixture.behaviors.flatMap((behavior) => behavior.conditions.map((condition) => condition.kind)))].sort();
const actionKinds = [...new Set(fixture.behaviors.flatMap((behavior) => behavior.actions.map((action) => action.kind)))].sort();
const targetRefs = [...new Set(fixture.behaviors.flatMap((behavior) => Object.values(behavior.target)))].sort();

const readModel = {
  schemaVersion: 'gameplay-behavior-model-read-model.v1',
  behaviorPackId: fixture.behaviorPackId,
  status: fixture.status,
  behaviorCount: fixture.behaviors.length,
  readyCount: fixture.behaviors.filter((behavior) => behavior.status === 'ready').length,
  partialCount: fixture.behaviors.filter((behavior) => behavior.status === 'partial').length,
  blockedCount: fixture.behaviors.filter((behavior) => behavior.status === 'blocked').length,
  unsupportedCount: fixture.behaviors.filter((behavior) => behavior.status === 'unsupported').length,
  behaviorIds,
  targetRefs,
  triggerKinds,
  conditionKinds,
  actionKinds,
  boundary: 'Read-only Gameplay Behavior Model summary; no runtime execution, no script execution, no eval, no dynamic import, no plugin loader, no command bridge, no browser trusted writes, no source apply, and no production-stable scripting API claim.'
};

assert.equal(readModel.schemaVersion, 'gameplay-behavior-model-read-model.v1');
assert.equal(readModel.behaviorCount, 7);
assert.equal(readModel.readyCount, 7);
assert.deepEqual(readModel.behaviorIds, [
  'patrol-guard-route-a',
  'collect-keycard',
  'spike-damage-contact',
  'open-blue-door',
  'win-after-exit',
  'timed-hazard-pulse',
  'dash-ability-trigger'
]);
assert.deepEqual(triggerKinds, ['on_collect', 'on_contact', 'on_flag_changed', 'on_input', 'on_start', 'on_timer']);
assert.ok(conditionKinds.includes('flag_equals'));
assert.ok(conditionKinds.includes('cooldown_ready'));
assert.ok(actionKinds.includes('complete_objective'));
assert.ok(actionKinds.includes('emit_signal'));
assert.ok(targetRefs.includes('player'));
assert.match(readModel.boundary, /Read-only/);
assert.match(readModel.boundary, /no runtime execution/);
assert.match(readModel.boundary, /no script execution/);
assert.match(readModel.boundary, /no command bridge/);
assert.doesNotMatch(JSON.stringify(readModel), /execute_script|plugin_loader|dynamic_import|eval\(|trustedWrite/);

const doc = fs.readFileSync(path.join(__dirname, '../../docs/gameplay-behavior-model-v1.md'), 'utf8');
assert.match(doc, /read-model\/export compatibility/i);
assert.match(doc, /behavior ids, target\s+summaries, trigger\/condition\/action counts/);
assert.match(doc, /without interpreting arbitrary code or creating write\s+authority/);

console.log('gameplay behavior read-model smoke passed');
