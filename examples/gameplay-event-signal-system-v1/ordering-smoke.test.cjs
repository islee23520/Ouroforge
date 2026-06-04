const assert = require('assert');
const fs = require('fs');
const path = require('path');

const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, 'event-signal.valid.fixture.json'), 'utf8'));
const shuffled = [...fixture.events].reverse();
const orderedEventIds = shuffled
  .sort((left, right) => left.tick - right.tick || left.orderingIndex - right.orderingIndex || left.id.localeCompare(right.id))
  .map((event) => event.id);

const queueSummary = {
  schemaVersion: 'gameplay-event-signal-queue-summary.v1',
  eventLogId: fixture.eventLogId,
  eventCount: fixture.events.length,
  consumedCount: fixture.events.filter((event) => event.consumed).length,
  unconsumedCount: fixture.events.filter((event) => !event.consumed).length,
  earliestTick: Math.min(...fixture.events.map((event) => event.tick)),
  latestTick: Math.max(...fixture.events.map((event) => event.tick)),
  orderedEventIds,
  queueRules: [
    'Validate event artifacts before ordering.',
    'Order events deterministically by tick, then orderingIndex, then id.',
    'Keep consumed and unconsumed state visible; do not drop unconsumed events.',
    'Bound queues to at most 256 events for v1 fixtures.'
  ],
  boundary: 'Read-only deterministic Gameplay Event Signal queue summary; no runtime execution, no script execution, no eval, no dynamic import, no plugin loader, no command bridge, no browser trusted writes, no source apply, and no production-stable scripting API claim.'
};

assert.equal(queueSummary.schemaVersion, 'gameplay-event-signal-queue-summary.v1');
assert.deepEqual(queueSummary.orderedEventIds, [
  'player-spike-contact',
  'keycard-collected',
  'blue-door-flag-changed',
  'hazard-pulse-timer',
  'dash-input-action',
  'guard-state-changed'
]);
assert.equal(queueSummary.consumedCount, 5);
assert.equal(queueSummary.unconsumedCount, 1);
assert.match(queueSummary.queueRules.join(' '), /tick, then orderingIndex, then id/);
assert.match(queueSummary.boundary, /no runtime execution/);
assert.match(queueSummary.boundary, /no command bridge/);
assert.doesNotMatch(JSON.stringify(queueSummary), /execute_script|plugin_loader|dynamic_import|eval\(|trustedWrite/);

const doc = fs.readFileSync(path.join(__dirname, '../../docs/gameplay-event-signal-system-v1.md'), 'utf8');
assert.match(doc, /Deterministic ordering and bounded queue rules/);
assert.match(doc, /Sort by `tick`, then `orderingIndex`, then stable `id`/);
assert.match(doc, /gameplay-event-signal-queue-summary\.v1/);
assert.match(doc, /no runtime execution,\s+no script execution, no command bridge, no browser trusted writes, no source\s+apply/);

console.log('gameplay event signal ordering smoke passed');
