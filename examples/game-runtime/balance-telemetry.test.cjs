'use strict';

// Runtime contract test for Balance Telemetry Aggregation v1 (#1607).
// Mirror of the Rust contract test
// crates/ouroforge-core/tests/balance_telemetry_contract.rs.
//
// Validates that aggregating many seeded synthetic-player runs (#1606) over a
// planted deck produces a deterministic, descriptive balance report that flags a
// planted degenerate combo (smite+hex) and a planted dead item (brick) with
// replayable seeds, while leaving a normal card (strike) unflagged. The
// aggregation is observation-only: it never mutates state or applies a balance
// change.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const deck = require('./deck-roguelike.js');
const synthetic = require('./synthetic-player.js');
const telemetry = require('./balance-telemetry.js');

function readJson(file) {
  return JSON.parse(fs.readFileSync(path.join(__dirname, file), 'utf8'));
}

const scene = readJson('deck-roguelike-balance-telemetry-scene-v1.json');
const deckSpec = scene.deckRoguelike;
const rosterSpec = readJson('synthetic-player-personas-v1.json');
const sampleReport = readJson('balance-telemetry-report-v1.json');

// Pinned report digest on the planted scene (seed 31). The Rust contract test
// pins the same string, so the JS runtime and the trusted Rust mirror agree
// byte-for-byte on the aggregated balance report.
const EXPECTED_DIGEST =
  'report|scene=deck-roguelike-balance-telemetry-trial-v1|runs=5|won=5|lost=0|playing=0'
  + '|cards=brick:0:0:0:0,hex:12:5:5:0,smite:13:5:5:0,strike:6:5:5:0'
  + '|degen={hex@5/5#31:cautious-novice},{hex+smite@5/5#31:cautious-novice},{smite@5/5#31:cautious-novice}'
  + '|dead={brick#31:cautious-novice}'
  + '|curve=aggressive-expert:won:2:7,balanced-veteran:won:4:9,cautious-novice:won:2:7,defensive-expert:won:2:7,reckless-novice:won:3:9';

function buildReport() {
  const runs = synthetic.playRoster(deck, deckSpec, rosterSpec);
  return telemetry.aggregate(runs, telemetry.vocabularyOf(deckSpec), deckSpec.id);
}

// --- Aggregation determinism --------------------------------------------------
{
  const a = buildReport();
  const b = buildReport();
  assert.equal(a.digest, b.digest, 'aggregation is deterministic');
  assert.deepEqual(a, b, 'the full balance report is reproducible');
  assert.equal(a.digest, EXPECTED_DIGEST, 'report matches the pinned cross-language digest');
}

// --- The produced report equals the committed sample report -------------------
{
  const report = buildReport();
  assert.deepEqual(report, sampleReport, 'the produced report matches the committed sample balance report');
  assert.equal(report.readOnlyInspection.trustedEmitter, 'balance-telemetry-aggregation');
  assert.deepEqual(report.readOnlyInspection.disallowedActions, [
    'trusted writes', 'auto-applied nerf or buff', 'balance guarantee', 'live mutation',
  ]);
}

// --- A planted degenerate combo is flagged with a replayable seed -------------
{
  const report = buildReport();
  const combos = report.degenerateCombos.map((g) => g.cards.join('+'));
  assert.ok(combos.includes('hex+smite'), 'the planted degenerate combo (hex+smite) is flagged');
  const combo = report.degenerateCombos.find((g) => g.cards.join('+') === 'hex+smite');
  assert.equal(combo.winInclusion.included, combo.winInclusion.totalWins, 'the combo appears in every winning run');
  // The flag carries a replayable seed (deck seed + persona) that reproduces it.
  assert.equal(combo.replay.deckSeed, 31);
  assert.equal(typeof combo.replay.persona, 'string');
  assert.ok(Number.isInteger(combo.replay.personaSeed));
  // Replay the cited seed and confirm both cards are actually played in that run.
  const persona = synthetic.normalizePersonas(rosterSpec).personas.find((p) => p.id === combo.replay.persona);
  const replayRun = synthetic.playRun(deck, deckSpec, persona, synthetic.normalizePersonas(rosterSpec).budget);
  assert.ok((replayRun.cardPlays.hex || 0) > 0 && (replayRun.cardPlays.smite || 0) > 0, 'the replay seed reproduces the combo');
}

// --- A planted dead item is flagged with a replayable seed --------------------
{
  const report = buildReport();
  const dead = report.deadItems.map((d) => d.card);
  assert.deepEqual(dead, ['brick'], 'only the planted dead item (brick) is flagged dead');
  const brick = report.deadItems[0];
  assert.equal(brick.plays, 0, 'the dead item is never played');
  assert.equal(brick.replay.deckSeed, 31);
  // strike is a normal, played card — it must NOT be flagged dead or degenerate.
  const strike = report.cards.find((c) => c.card === 'strike');
  assert.ok(strike.plays > 0, 'the normal card strike is played');
  assert.ok(!report.degenerateCombos.some((g) => g.cards.includes('strike')), 'the normal card is not degenerate');
  assert.ok(!dead.includes('strike'), 'the normal card is not flagged dead');
}

// --- Fail closed on malformed aggregation input -------------------------------
{
  assert.throws(() => telemetry.aggregate([], ['a'], 's'), /runs must be a non-empty array/, 'empty runs fail closed');
  assert.throws(() => telemetry.aggregate([{ outcome: 'won' }], [], 's'), /vocabulary must be a non-empty array/, 'empty vocabulary fails closed');
  assert.throws(() => telemetry.vocabularyOf({}), /must declare a cards vocabulary/, 'a deck without a vocabulary fails closed');
}

console.log('balance-telemetry.test.cjs: all balance telemetry aggregation cases passed');
