const assert = require('assert');
const fs = require('fs');
const path = require('path');

const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, 'event-signal.valid.fixture.json'), 'utf8'));
const ordered = [...fixture.events].sort((left, right) => left.tick - right.tick || left.orderingIndex - right.orderingIndex || left.id.localeCompare(right.id));
const eventTypeCounts = Object.fromEntries(
  [...new Set(fixture.events.map((event) => event.eventType))].sort().map((eventType) => [
    eventType,
    fixture.events.filter((event) => event.eventType === eventType).length
  ])
);
const signalNames = [...new Set(fixture.events.map((event) => event.signalName).filter(Boolean))].sort();
const sourceRefs = [...new Set(fixture.events.flatMap((event) => Object.values(event.source)))].sort();
const targetRefs = [...new Set(fixture.events.flatMap((event) => Object.values(event.target)))].sort();
const linkedEvidenceRefs = [...new Set([...fixture.evidenceRefs, ...fixture.events.flatMap((event) => event.evidenceRefs || [])])].sort();

const readModel = {
  schemaVersion: 'gameplay-event-signal-read-model.v1',
  eventLogId: fixture.eventLogId,
  status: fixture.status,
  eventCount: fixture.events.length,
  consumedCount: fixture.events.filter((event) => event.consumed).length,
  unconsumedCount: fixture.events.filter((event) => !event.consumed).length,
  eventTypeCounts,
  orderedEventIds: ordered.map((event) => event.id),
  signalNames,
  sourceRefs,
  targetRefs,
  linkedEvidenceRefs,
  queueSummary: { schemaVersion: 'gameplay-event-signal-queue-summary.v1' },
  boundary: 'Read-only Gameplay Event Signal evidence/read-model summary; no runtime execution, no script execution, no eval, no dynamic import, no plugin loader, no command bridge, no browser trusted writes, no source apply, and no production-stable scripting API claim.'
};

assert.equal(readModel.schemaVersion, 'gameplay-event-signal-read-model.v1');
assert.equal(readModel.eventCount, 6);
assert.equal(readModel.consumedCount, 5);
assert.equal(readModel.unconsumedCount, 1);
assert.equal(readModel.eventTypeCounts.collision_contact, 1);
assert.deepEqual(readModel.orderedEventIds, [
  'player-spike-contact',
  'keycard-collected',
  'blue-door-flag-changed',
  'hazard-pulse-timer',
  'dash-input-action',
  'guard-state-changed'
]);
assert.ok(readModel.signalNames.includes('flag_changed'));
assert.ok(readModel.sourceRefs.includes('flag-store'));
assert.ok(readModel.targetRefs.includes('player'));
assert.ok(readModel.linkedEvidenceRefs.includes('docs/gameplay-event-signal-system-v1.md'));
assert.match(readModel.boundary, /Read-only/);
assert.match(readModel.boundary, /no runtime execution/);
assert.match(readModel.boundary, /no script execution/);
assert.match(readModel.boundary, /no command bridge/);
assert.match(readModel.boundary, /no browser trusted writes/);
assert.match(readModel.boundary, /no source apply/);
assert.doesNotMatch(JSON.stringify(readModel), /execute_script|plugin_loader|dynamic_import|eval\(|trustedWrite/);

const doc = fs.readFileSync(path.join(__dirname, '../../docs/gameplay-event-signal-system-v1.md'), 'utf8');
assert.match(doc, /Evidence\/read-model export compatibility/);
assert.match(doc, /gameplay-event-signal-read-model\.v1/);
assert.match(doc, /does not emit runtime events, dispatch signals, apply\s+behavior, mutate source files, or create browser write authority/);
assert.match(doc, /read-only, no runtime execution, no\s+script execution, no command bridge, no browser trusted writes, no source\s+apply/);

console.log('gameplay event signal read-model smoke passed');
