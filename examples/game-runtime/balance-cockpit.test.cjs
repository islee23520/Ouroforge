'use strict';

// Runtime contract test for Balance Cockpit Read-Only Surface and Re-Run Diff v1
// (#1608). Mirror of the Rust contract test
// crates/ouroforge-core/tests/balance_cockpit_rerun_contract.rs.
//
// Validates that the cockpit surfaces a balance report read-only (HTML-escaped,
// no execution, per-flag counterexample/replay) and that a proposed nerf can be
// re-run on the identical seed distribution and its win-rate impact diffed
// (reusing the compare digest-equality signal) — never an auto-applied nerf and
// never a trusted write.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const deck = require('./deck-roguelike.js');
const synthetic = require('./synthetic-player.js');
const telemetry = require('./balance-telemetry.js');
const cockpit = require('./balance-cockpit.js');

function readJson(file) {
  return JSON.parse(fs.readFileSync(path.join(__dirname, file), 'utf8'));
}

const scene = readJson('deck-roguelike-balance-rerun-scene-v1.json');
const deckSpec = scene.deckRoguelike;
const rosterSpec = readJson('synthetic-player-personas-v1.json');
const change = readJson('balance-rerun-change-v1.json').change;

// Pinned re-run diff digest on the planted scene (seed 31). The Rust contract
// test pins the same string, so the JS runtime and the trusted Rust mirror agree
// byte-for-byte on the re-run diff.
const EXPECTED_DIFF_DIGEST =
  'rerun-diff|status=changed|change=smite[damage=6]|wr=5/5->2/5(-3)'
  + '|cards=brick:0->0,defend:1->3,smite:20->38,strike:5->14'
  + '|degenResolved=|degenIntroduced=|deadResolved=|deadIntroduced=';

// --- Seeded re-run + win-rate diff (reuse compare) ----------------------------
{
  const diff = cockpit.rerunWithChange(deck, synthetic, telemetry, deckSpec, rosterSpec, change);
  assert.equal(diff.status, 'changed', 'the nerf changes the seeded outcome distribution');
  assert.deepEqual(diff.winRate.before, { wins: 5, total: 5 }, 'baseline wins every run');
  assert.deepEqual(diff.winRate.after, { wins: 2, total: 5 }, 'the nerf drops the win-rate');
  assert.equal(diff.winRate.deltaWins, -3, 'the win-rate impact is diffed');
  assert.equal(diff.digest, EXPECTED_DIFF_DIGEST, 'the re-run diff matches the pinned cross-language digest');

  // Deterministic: re-running the identical seed distribution reproduces it.
  const again = cockpit.rerunWithChange(deck, synthetic, telemetry, deckSpec, rosterSpec, change);
  assert.equal(again.digest, diff.digest, 'the re-run diff is deterministic');

  // The cockpit is read-only: the change is a proposal, never auto-applied.
  assert.equal(diff.policy.browserWriteAccess, 'none', 'the cockpit grants no browser write access');
  assert.equal(diff.policy.autoApplied, false, 'the nerf is never auto-applied');
}

// --- The proposed change never mutates the trusted spec -----------------------
{
  const before = JSON.stringify(deckSpec);
  const candidate = cockpit.applyBalanceChange(deckSpec, change);
  assert.equal(candidate.cards.smite.damage, 6, 'the change is applied to the returned copy');
  assert.equal(deckSpec.cards.smite.damage, 18, 'the original spec is unchanged');
  assert.equal(JSON.stringify(deckSpec), before, 'applyBalanceChange does not mutate its input');
}

// --- Read-only surfacing: escaped, no execution, per-flag counterexample ------
{
  const runs = synthetic.playRoster(deck, deckSpec, rosterSpec);
  // A hostile scene id must be surfaced escaped, never executed.
  const report = telemetry.aggregate(runs, telemetry.vocabularyOf(deckSpec), '<script>alert(1)</script>');
  const surface = cockpit.surfaceBalanceReport(report);
  assert.equal(surface.scene, '&lt;script&gt;alert(1)&lt;/script&gt;', 'the report is HTML-escaped, never executed');
  assert.equal(surface.winRate, '5/5');
  // Each degenerate flag carries a counterexample/replay seed.
  assert.ok(surface.degenerateFlags.length >= 1, 'degenerate flags are surfaced');
  assert.match(surface.degenerateFlags[0].counterexample, /^seed 31 \/ persona /, 'a flag carries a replayable counterexample');
  assert.ok(surface.deadFlags.some((d) => d.card === 'brick'), 'the dead item is surfaced');
  assert.deepEqual(surface.readOnlyInspection.disallowedActions, [
    'trusted writes', 'auto-applied nerf or buff', 'command execution', 'live mutation',
  ]);
}

// --- compare reuse: read-only CLI command surfacing ---------------------------
{
  const command = cockpit.compareCommand('runs/before', 'runs/after');
  assert.equal(command, 'cargo run -p ouroforge-cli -- compare runs/before runs/after --output-dir runs/after/comparisons');
}

// --- Fail closed on a malformed balance change --------------------------------
{
  assert.throws(() => cockpit.applyBalanceChange(deckSpec, { card: 'phantom', damage: 1 }), /undeclared card "phantom"/, 'an unknown card fails closed');
  assert.throws(() => cockpit.applyBalanceChange(deckSpec, { card: 'smite' }), /must set at least one of/, 'a no-op change fails closed');
  assert.throws(() => cockpit.applyBalanceChange(deckSpec, { card: 'smite', damage: -1 }), /must be a non-negative integer/, 'a negative field fails closed');
}

console.log('balance-cockpit.test.cjs: all balance cockpit re-run and surfacing cases passed');
