'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '../..');
const runtimeDir = path.join(root, 'examples/game-runtime');
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 'scenario-coverage-v32/matrix.fixture.json'), 'utf8'));
const deck = require(path.join(runtimeDir, 'deck-roguelike.js'));
const synthetic = require(path.join(runtimeDir, 'synthetic-player.js'));

assert.equal(matrix.issue, 1610);

const scene = JSON.parse(fs.readFileSync(path.join(root, matrix.runtimeFixtures.deckBalanceScene), 'utf8'));
const rosterSpec = JSON.parse(fs.readFileSync(path.join(root, matrix.runtimeFixtures.personas), 'utf8'));
const roster = synthetic.normalizePersonas(rosterSpec);
const persona = roster.personas.find((p) => p.id === matrix.expectedPersonaDigest.personaId);
assert.ok(persona, 'expected persona fixture is present');

const first = synthetic.playRun(deck, scene.deckRoguelike, persona, roster.budget);
const second = synthetic.playRun(deck, scene.deckRoguelike, persona, roster.budget);
assert.deepEqual(second, first, 'persona run is deterministic for a fixed seed');
assert.equal(first.digest, matrix.expectedPersonaDigest.digest);
assert.equal(first.schemaVersion, 'ouroforge.synthetic-player-run.v1');
assert.equal(first.readOnlyInspection.trustedEmitter, 'synthetic-player-persona-run');
assert.deepEqual(first.readOnlyInspection.disallowedActions, [
  'trusted writes', 'auto-apply', 'win-maximizing solve', 'live mutation',
]);

const records = synthetic.playRoster(deck, scene.deckRoguelike, rosterSpec);
assert.equal(records.length, roster.personas.length);
assert.ok(records.some((record) => record.outcome === 'won'), 'roster includes winning evidence');
assert.ok(records.some((record) => record.outcome === 'lost'), 'roster includes losing evidence');
assert.ok(new Set(records.map((record) => record.digest)).size > 1, 'skill/style changes the run digest');
for (const record of records) {
  assert.equal(record.schemaVersion, 'ouroforge.synthetic-player-run.v1');
  assert.ok(record.turns <= roster.budget.maxTurns);
  assert.ok(record.actions <= roster.budget.maxActions);
  assert.equal(record.readOnlyInspection.browserStudioMode, 'read-only seeded persona run observation');
}

const capped = synthetic.playRun(deck, scene.deckRoguelike, persona, { maxTurns: 99, maxActions: 3 });
assert.equal(capped.budgetExhausted, true, 'nerf/re-run style capped comparison records budget evidence');
assert.equal(capped.actions, 3);
assert.notEqual(capped.digest, first.digest, 'changed run bounds produce diff evidence');

const telemetryDoc = fs.readFileSync(path.join(root, matrix.telemetryEvidence.doc), 'utf8');
const telemetryDocLower = telemetryDoc.toLowerCase();
for (const signal of matrix.telemetryEvidence.signals) assert.ok(telemetryDoc.includes(signal), `telemetry signal ${signal}`);
for (const phrase of matrix.telemetryEvidence.readOnlyBoundary) assert.ok(telemetryDocLower.includes(phrase.toLowerCase()), `boundary phrase ${phrase}`);

const compareGolden = JSON.parse(fs.readFileSync(path.join(root, matrix.compareGolden), 'utf8'));
assert.equal(compareGolden.schemaVersion, 'compare-backward-compat-v32');
assert.equal(compareGolden.fixtureScoped, true);
assert.equal(compareGolden.runKind, 'non-balance');
assert.equal(compareGolden.balanceTelemetryRequired, false);
assert.equal(compareGolden.status, 'unchanged');

console.log('scenario-coverage-v32 targeted assertions passed');
