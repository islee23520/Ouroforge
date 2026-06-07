// Runtime test for Adaptive-Audio Runtime Hooks v1 (#1644).
//
// Exercises the deterministic adaptive-audio hook evaluator against the shared
// fixture: deterministic audio-intent emission and snapshot/restore parity. The
// fixture is the same one the Rust contract test consumes, so the two
// implementations stay in lockstep (cross-language determinism).
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { evaluateAudioHooks, validateAudioHookSet } = require('./audio-hooks.js');

const fixture = JSON.parse(
  fs.readFileSync(
    path.join(__dirname, '../audio-hooks-v1/audio-hooks.fixture.json'),
    'utf8',
  ),
);
const { hookSet, signalSequence } = fixture;

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

// The hook set validates.
assert.equal(validateAudioHookSet(hookSet), true);

// Deterministic emission: each step emits exactly the expected ordered intents.
for (const step of signalSequence) {
  const signals = { flags: step.flags, numbers: step.numbers, events: step.events };
  const intents = evaluateAudioHooks(hookSet, signals);
  assert.deepEqual(
    intents.map((i) => i.intent),
    step.expectedIntents,
    `step ${step.label} emits expected intents`,
  );
  // Emitted intents carry the audio-intent shape.
  for (const intent of intents) {
    assert.ok(intent.behaviorId && intent.actionId && intent.targetEntityId && intent.intent);
  }
  // Re-evaluating identical signals yields an identical result.
  const again = evaluateAudioHooks(hookSet, signals);
  assert.deepEqual(again, intents, `step ${step.label} is deterministic`);
}

// Snapshot/restore parity: capture a signal snapshot, mutate the live signals,
// restore the snapshot, and confirm the restored signals reproduce exactly the
// same audio intents (the evaluator is a pure function of the signals).
{
  const boss = signalSequence.find((s) => s.label === 'boss-low-health');
  const live = { flags: clone(boss.flags), numbers: clone(boss.numbers), events: clone(boss.events) };
  const snapshot = clone(live);
  const before = evaluateAudioHooks(hookSet, live);

  // Mutate the live signals into a different (calm) state.
  live.flags = {};
  live.numbers = { enemyCount: 0, playerHealth: 100 };
  live.events = [];
  const mutated = evaluateAudioHooks(hookSet, live);
  assert.notDeepEqual(mutated.map((i) => i.intent), before.map((i) => i.intent));

  // Restore and re-evaluate: identical to the pre-mutation emission.
  const restored = clone(snapshot);
  const after = evaluateAudioHooks(hookSet, restored);
  assert.deepEqual(after, before, 'restored snapshot reproduces identical audio intents');
}

// Validation fails closed on a duplicate hook id.
{
  const bad = clone(hookSet);
  bad.hooks.push(clone(bad.hooks[0]));
  assert.throws(() => evaluateAudioHooks(bad, { flags: {}, numbers: {}, events: [] }), /declared more than once/);
}

// Validation fails closed on an unknown condition kind.
{
  const bad = clone(hookSet);
  bad.hooks = [{ hookId: 'bad', priority: 1, when: { kind: 'sometimes' }, emit: { behaviorId: 'a', actionId: 'b', targetEntityId: 'c', intent: 'd' } }];
  assert.throws(() => evaluateAudioHooks(bad, { flags: {}, numbers: {}, events: [] }), /unknown audio hook condition kind/);
}

// Validation fails closed on a condition missing a required field (parity with
// the Rust AudioHookCondition::validate).
{
  const bad = clone(hookSet);
  bad.hooks = [{ hookId: 'bad', priority: 1, when: { kind: 'flagEquals', flag: 'x' }, emit: { behaviorId: 'a', actionId: 'b', targetEntityId: 'c', intent: 'd' } }];
  assert.throws(() => validateAudioHookSet(bad), /condition value is required/);
}

// Validation fails closed on a non-finite numeric threshold.
{
  const bad = clone(hookSet);
  bad.hooks = [{ hookId: 'bad', priority: 1, when: { kind: 'numberAtLeast', signal: 'enemyCount' }, emit: { behaviorId: 'a', actionId: 'b', targetEntityId: 'c', intent: 'd' } }];
  assert.throws(() => validateAudioHookSet(bad), /condition threshold must be finite/);
}

// Validation fails closed on a non-integer priority (parity with the Rust i64
// contract).
{
  const bad = clone(hookSet);
  bad.hooks[0].priority = 1.5;
  assert.throws(() => validateAudioHookSet(bad), /priority must be an integer/);
}

console.log('adaptive-audio runtime hooks deterministic emission + snapshot/restore parity test passed');
