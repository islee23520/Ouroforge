'use strict';

// Runtime contract test for Synthetic Player Persona Agents v1 (#1606).
// Mirror of the Rust contract test
// crates/ouroforge-core/tests/synthetic_player_agents_contract.rs.
//
// Validates that human-like persona agents drive the existing deck-roguelike
// probe (#1601) deterministically: a persona reproduces an identical run on a
// fixed seed, skill/style parameters vary behavior in a bounded way (including a
// deterministic win/loss spread on a tuned encounter), the run budget bounds
// every run, and a malformed persona spec fails closed. Personas are human-like
// (skill governs misplay, aggression governs style) — never win-maximizing
// solvers: there is no lookahead or per-game tuning, so a reckless novice
// over-extends and dies while disciplined personas win the same seeded fight.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const deck = require('./deck-roguelike.js');
const synthetic = require('./synthetic-player.js');

function readJson(file) {
  return JSON.parse(fs.readFileSync(path.join(__dirname, file), 'utf8'));
}

// The tuned balance encounter: persona skill/style produce a deterministic
// win/loss spread here, unlike the trivially-winnable base deck scene.
const scene = readJson('deck-roguelike-balance-scene-v1.json');
const deckSpec = scene.deckRoguelike;
const rosterSpec = readJson('synthetic-player-personas-v1.json');
const roster = synthetic.normalizePersonas(rosterSpec);

// Pinned roster digests on the balance scene (seed 20). The Rust contract test
// pins the same strings, so the JS runtime and the trusted Rust mirror agree
// byte-for-byte on every persona's seeded trajectory.
const EXPECTED_DIGESTS = {
  'cautious-novice':
    'persona=cautious-novice|skill=20|aggro=25|outcome=won|turn=4|actions=11|budget=0|'
    + 'rng=20:440012099:19|turn=4|status=won|php=4|pbl=0|ehp=0|hand=strike,defend,strike,bash|draw=strike,defend,bash|discard=strike',
  'reckless-novice':
    'persona=reckless-novice|skill=20|aggro=90|outcome=lost|turn=3|actions=7|budget=0|'
    + 'rng=20:1703683439:15|turn=3|status=lost|php=0|pbl=0|ehp=24|hand=|draw=strike,defend,bash|discard=strike,strike,bash,defend,strike',
  'balanced-veteran':
    'persona=balanced-veteran|skill=70|aggro=50|outcome=won|turn=3|actions=7|budget=0|'
    + 'rng=20:1703683439:15|turn=3|status=won|php=5|pbl=0|ehp=0|hand=defend,strike,strike,bash|draw=strike,strike,defend|discard=bash',
  'aggressive-expert':
    'persona=aggressive-expert|skill=95|aggro=95|outcome=won|turn=3|actions=7|budget=0|'
    + 'rng=20:1703683439:15|turn=3|status=won|php=5|pbl=0|ehp=0|hand=defend,strike,strike,bash|draw=strike,strike,defend|discard=bash',
  'defensive-expert':
    'persona=defensive-expert|skill=95|aggro=5|outcome=won|turn=4|actions=12|budget=0|'
    + 'rng=20:440012099:19|turn=4|status=won|php=4|pbl=10|ehp=0|hand=strike,strike|draw=bash,bash,strike|discard=defend,defend,strike',
};

// --- Persona determinism on a fixed seed -------------------------------------
{
  const persona = roster.personas.find((p) => p.id === 'balanced-veteran');
  const first = synthetic.playRun(deck, deckSpec, persona, roster.budget);
  const second = synthetic.playRun(deck, deckSpec, persona, roster.budget);
  assert.equal(first.digest, second.digest, 'a persona reproduces an identical run on a fixed seed');
  assert.deepEqual(first, second, 'the full run record is reproducible');
  assert.equal(first.readOnlyInspection.trustedEmitter, 'synthetic-player-persona-run');
  assert.deepEqual(first.readOnlyInspection.disallowedActions, [
    'trusted writes', 'auto-apply', 'win-maximizing solve', 'live mutation',
  ]);
}

// --- Skill/style variation, including a deterministic win/loss spread ---------
{
  const records = synthetic.playRoster(deck, deckSpec, rosterSpec);
  assert.equal(records.length, roster.personas.length);

  // Every persona reproduces its pinned digest; each is individually deterministic.
  for (const record of records) {
    assert.equal(record.digest, EXPECTED_DIGESTS[record.personaId], `persona "${record.personaId}" matches its pinned digest`);
    const replay = synthetic.playRun(deck, deckSpec, roster.personas.find((p) => p.id === record.personaId), roster.budget);
    assert.equal(replay.digest, record.digest, `persona "${record.personaId}" replays identically`);
  }

  // Skill/style parameters vary the trajectory and the observable behavior.
  const distinctDigests = new Set(records.map((r) => r.digest));
  assert.ok(distinctDigests.size > 1, 'skill/style parameters vary the trajectory');
  const behaviors = new Set(records.map((r) => `${r.outcome}:${r.turns}:${r.actions}`));
  assert.ok(behaviors.size > 1, 'skill/style parameters vary observable behavior');

  // The variation is bounded: every run stays within the run budget and ends in
  // a legal status.
  for (const record of records) {
    assert.ok(record.turns <= roster.budget.maxTurns, `${record.personaId} stays within the turn budget`);
    assert.ok(record.actions <= roster.budget.maxActions, `${record.personaId} stays within the action budget`);
    assert.ok(['playing', 'won', 'lost'].includes(record.outcome), `${record.personaId} ends in a legal status`);
  }

  // Human-like, not win-maximizers: the roster produces a real win/loss spread.
  // The reckless novice over-extends and dies; the disciplined personas win.
  const wins = records.filter((r) => r.outcome === 'won');
  const losses = records.filter((r) => r.outcome === 'lost');
  assert.ok(wins.length >= 1, 'at least one persona wins the seeded encounter');
  assert.ok(losses.length >= 1, 'at least one persona loses — agents are human-like, not win-maximizers');
  assert.equal(records.find((r) => r.personaId === 'reckless-novice').outcome, 'lost', 'the reckless novice over-extends and dies');
  assert.equal(records.find((r) => r.personaId === 'aggressive-expert').outcome, 'won', 'the disciplined expert wins the same seeded fight');

  for (const record of records) {
    console.log(`  roster ${record.personaId}: ${record.outcome} (turn ${record.turns}, ${record.actions} actions)`);
  }
}

// --- Bounded run budget ------------------------------------------------------
{
  const persona = roster.personas.find((p) => p.id === 'aggressive-expert');

  // A tight action budget halts the run mid-encounter, fail-safe, not forever.
  const capped = synthetic.playRun(deck, deckSpec, persona, { maxTurns: 99, maxActions: 3 });
  assert.equal(capped.actions, 3, 'the action budget caps the number of actions');
  assert.equal(capped.budgetExhausted, true, 'hitting the action budget flags budget exhaustion');
  assert.equal(capped.outcome, 'playing', 'the capped run is unfinished, not a forced result');

  // A tight turn budget likewise halts the run.
  const turnCapped = synthetic.playRun(deck, deckSpec, persona, { maxTurns: 1, maxActions: 999 });
  assert.equal(turnCapped.budgetExhausted, true, 'hitting the turn budget flags budget exhaustion');
  assert.ok(turnCapped.turns <= 2, 'the turn budget bounds the number of turns');

  // A generous budget lets the same persona finish, proving the cap — not the
  // policy — stopped the bounded runs.
  const generous = synthetic.playRun(deck, deckSpec, persona, { maxTurns: 64, maxActions: 512 });
  assert.equal(generous.outcome, 'won', 'a generous budget lets the persona finish the run');
  assert.equal(generous.budgetExhausted, false, 'a finished run does not flag budget exhaustion');
}

// --- Malformed persona fails closed ------------------------------------------
{
  const malformed = readJson('synthetic-player-invalid-persona.json');
  assert.throws(
    () => synthetic.normalizePersonas(malformed),
    /skill must be an integer in \[0, 100\]/,
    'an out-of-range skill must fail closed with a clear diagnostic',
  );
  assert.throws(
    () => synthetic.playRun({}, deckSpec, roster.personas[0], roster.budget),
    /requires the OuroforgeDeckRoguelike module/,
    'driving without the deck module must fail closed',
  );
}

console.log('synthetic-player.test.cjs: all synthetic player persona agent cases passed');
