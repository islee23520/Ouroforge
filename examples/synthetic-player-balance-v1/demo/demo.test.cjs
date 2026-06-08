'use strict';

// Synthetic Player Balance Demo v1 smoke test (#1609).
//
// Deterministic, fixture-scoped, no network and no live browser. From a fresh
// clone this reproduces the demo end-to-end over the existing modules:
//   synthetic personas (#1606) play the deck-roguelike (#1601),
//   balance telemetry (#1607) flags a degenerate combo with a replayable seed,
//   and the read-only cockpit (#1608) re-runs a proposed nerf on the identical
//   seed distribution and diffs the win-rate impact.
// It asserts the flag, the replayable seed, and the re-run diff, and checks the
// committed demo evidence reproduces byte-for-byte. The trusted Rust mirror is
// crates/ouroforge-core/tests/synthetic_player_balance_demo_contract.rs.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const deck = require('../../game-runtime/deck-roguelike.js');
const synthetic = require('../../game-runtime/synthetic-player.js');
const telemetry = require('../../game-runtime/balance-telemetry.js');
const cockpit = require('../../game-runtime/balance-cockpit.js');

function readJson(file) {
  return JSON.parse(fs.readFileSync(path.join(__dirname, file), 'utf8'));
}

const scene = readJson('deck-roguelike-demo-scene.json');
const deckSpec = scene.deckRoguelike;
const rosterSpec = readJson('personas.json');
const change = readJson('balance-change.json').change;
const evidence = readJson('demo-evidence.json');

// --- Personas play the deck-roguelike; telemetry flags a degenerate combo -----
const runs = synthetic.playRoster(deck, deckSpec, rosterSpec);
const report = telemetry.aggregate(runs, telemetry.vocabularyOf(deckSpec), deckSpec.id);

// A degenerate combo (hex+smite) is flagged.
const combo = report.degenerateCombos.find((g) => g.cards.join('+') === 'hex+smite');
assert.ok(combo, 'the demo flags the hex+smite degenerate combo');
assert.equal(combo.winInclusion.included, combo.winInclusion.totalWins, 'the combo appears in every winning run');

// The flag carries a replayable seed; replaying it reproduces both cards.
assert.equal(combo.replay.deckSeed, 31, 'the flag carries the replayable deck seed');
const persona = synthetic.normalizePersonas(rosterSpec).personas.find((p) => p.id === combo.replay.persona);
const replayRun = synthetic.playRun(deck, deckSpec, persona, synthetic.normalizePersonas(rosterSpec).budget);
assert.ok((replayRun.cardPlays.hex || 0) > 0 && (replayRun.cardPlays.smite || 0) > 0, 'the replay seed reproduces the combo');

// The dead item is flagged.
assert.ok(report.deadItems.some((d) => d.card === 'brick'), 'the demo flags the dead item brick');

// --- The read-only cockpit re-runs a proposed nerf and diffs the win-rate -----
const diff = cockpit.rerunWithChange(deck, synthetic, telemetry, deckSpec, rosterSpec, change);
assert.equal(diff.status, 'changed', 'the nerf changes the seeded outcome distribution');
assert.deepEqual(diff.winRate.before, { wins: 5, total: 5 }, 'baseline wins every run');
assert.deepEqual(diff.winRate.after, { wins: 3, total: 5 }, 'the nerf drops the win-rate');
assert.equal(diff.policy.autoApplied, false, 'the nerf is never auto-applied');
assert.equal(deckSpec.cards.smite.damage, 18, 'the trusted demo spec is never mutated');

// --- The committed demo evidence reproduces byte-for-byte ---------------------
assert.equal(report.digest, evidence.reportDigest, 'the committed report digest reproduces');
assert.equal(diff.digest, evidence.rerun.diffDigest, 'the committed re-run diff digest reproduces');
assert.deepEqual(combo.cards, evidence.degenerateCombo.cards, 'the committed combo reproduces');
assert.deepEqual(combo.replay, evidence.degenerateCombo.replay, 'the committed replay seed reproduces');
assert.deepEqual(diff.winRate, evidence.rerun.winRate, 'the committed win-rate diff reproduces');

console.log('demo.test.cjs: synthetic player balance demo reproduced deterministically');
