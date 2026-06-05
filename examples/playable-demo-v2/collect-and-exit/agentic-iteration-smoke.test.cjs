#!/usr/bin/env node
'use strict';

// Godot-Plus demo agentic iteration smoke (#789).
//
// Demonstrates evidence-driven agentic iteration end to end on an in-memory scene
// copy (committed source is never mutated):
//   1. inject a controlled failure (key trigger omits door_open) and reproduce it;
//   2. confirm the agent draft proposal is review-gated (not auto-applied);
//   3. confirm an independent review decision (no self-approval) before apply;
//   4. apply the proposed fix through the (simulated) Safe Source Apply handoff;
//   5. rerun and compare before/after evidence (failure -> pass);
//   6. validate the journal links hypothesis -> evidence -> mutation -> review ->
//      result; and prove no committed source file is mutated.
// Pure read-only harness: temp dir only, removed before exit.

const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const iterDir = path.join(fixtureDir, 'agentic-iteration');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(p) {
  return JSON.parse(fs.readFileSync(p, 'utf8'));
}

function fileHash(rel) {
  return crypto.createHash('sha256').update(fs.readFileSync(path.join(fixtureDir, rel))).digest('hex');
}

function createRuntime(scene) {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '' },
    document: { getElementById: () => null },
    fetch: () => Promise.resolve({ json: () => Promise.resolve(scene) }),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function keyTrigger(scene) {
  return scene.entities.find((e) => e.id === 'key').components.trigger;
}

// Apply an in-memory scene operation (inject failure or apply fix).
function applyOperation(scene, op) {
  const trigger = keyTrigger(scene);
  if (op.kind === 'removeTriggerAction') {
    trigger.onEnter = trigger.onEnter.filter(
      (action) => !(action.kind === op.action.kind && action.flag === op.action.flag)
    );
  } else if (op.kind === 'addTriggerAction') {
    const exists = trigger.onEnter.some(
      (action) => action.kind === op.action.kind && action.flag === op.action.flag
    );
    if (!exists) trigger.onEnter.push(op.action);
  } else {
    throw new Error(`unsupported operation ${op.kind}`);
  }
}

async function playToOutcome(scene) {
  const api = createRuntime(scene);
  await api.whenReady();
  api.setInput({ right: true });
  const world = api.step(150);
  api.setInput({ right: false });
  return world.componentModel.goalFlags;
}

(async () => {
  const baseScene = readJson(path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json'));
  const seed = readJson(path.join(iterDir, 'failure-seed.json'));
  const proposal = readJson(path.join(iterDir, 'draft-proposal.json'));
  const review = readJson(path.join(iterDir, 'review-decision.json'));
  const journal = readJson(path.join(iterDir, 'journal.json'));

  assert.equal(seed.schemaVersion, 'demo-agentic-failure-seed-v1');
  assert.equal(proposal.schemaVersion, 'demo-agentic-draft-proposal-v1');
  assert.equal(review.schemaVersion, 'demo-agentic-review-decision-v1');
  assert.equal(journal.schemaVersion, 'demo-agentic-journal-v1');

  // Capture committed-source hashes to prove nothing is mutated on disk.
  const watched = ['scenes/collect-and-exit.scene.json', 'agentic-iteration/draft-proposal.json'];
  const before = Object.fromEntries(watched.map((rel) => [rel, fileHash(rel)]));

  // 1. Reproduce the controlled failure on an in-memory clone.
  const failScene = JSON.parse(JSON.stringify(baseScene));
  applyOperation(failScene, seed.inject);
  const failFlags = await playToOutcome(failScene);
  assert.equal(failFlags.key_collected, seed.expectedFailure.key_collected, 'failure: key collected');
  assert.notEqual(failFlags.door_open, true, 'failure: gate stays closed');
  assert.notEqual(failFlags.exit_reached, true, 'failure: exit unreachable');

  // 2. The draft proposal is review-gated, not auto-applied.
  assert.equal(proposal.reviewRequired, true, 'proposal requires review');
  assert.equal(proposal.applied, false, 'proposal is not pre-applied');
  assert.equal(proposal.autoApply, false, 'proposal disables auto-apply');
  assert.equal(proposal.failureSeedId, seed.id, 'proposal targets the failure seed');

  // 3. Independent review decision gates the apply (no self-approval).
  assert.equal(review.proposalId, proposal.id, 'review targets the proposal');
  assert.equal(review.decision, 'accepted', 'review accepted');
  assert.equal(review.selfApproval, false, 'no self-approval flag');
  assert.notEqual(review.reviewerId, proposal.author, 'reviewer is not the proposal author');
  assert.equal(review.reviewRequired, true, 'apply remains review-required');

  // 4. Apply only because review is accepted and not a self-approval.
  const reviewGateOpen = review.decision === 'accepted' && review.selfApproval === false
    && review.reviewerId !== proposal.author;
  assert.ok(reviewGateOpen, 'apply gate requires an accepted independent review');
  const fixedScene = JSON.parse(JSON.stringify(failScene));
  applyOperation(fixedScene, proposal.operation);

  // 5. Rerun + compare before/after.
  const fixedFlags = await playToOutcome(fixedScene);
  assert.equal(fixedFlags.door_open, true, 'after apply: gate opens');
  assert.equal(fixedFlags.exit_reached, true, 'after apply: exit reached');
  const improvement = failFlags.exit_reached !== true && fixedFlags.exit_reached === true;
  assert.ok(improvement, 'before/after comparison shows improvement');

  // 6. Journal links every stage.
  const stages = journal.entries.map((entry) => entry.stage);
  for (const stage of ['hypothesis', 'failure_evidence', 'draft_proposal', 'review_decision', 'apply', 'rerun_comparison', 'result']) {
    assert.ok(stages.includes(stage), `journal links stage: ${stage}`);
  }
  assert.equal(journal.entries.find((e) => e.stage === 'result').outcome, 'improvement', 'journal result');

  // No committed source mutated.
  for (const rel of watched) {
    assert.equal(fileHash(rel), before[rel], `must not mutate committed source: ${rel}`);
  }

  // Evidence report -> temp dir, removed before exit.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-agentic-'));
  const tempReport = path.join(tempDir, 'iteration.json');
  fs.writeFileSync(tempReport, JSON.stringify({ before: failFlags, after: fixedFlags, improvement }, null, 2));
  assert.equal(readJson(tempReport).after.exit_reached, true);
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log('collect-and-exit agentic iteration smoke passed; controlled failure -> review-gated apply -> pass');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
