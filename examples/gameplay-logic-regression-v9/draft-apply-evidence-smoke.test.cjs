const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
// Repo root is two levels up from examples/gameplay-logic-regression-v9, used to
// resolve repo-relative evidence refs (examples/.../evidence/...).
const repoRoot = path.resolve(fixtureDir, '..', '..');
function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}
function assertNoForbiddenAuthority(value) {
  const serialized = JSON.stringify(value).toLowerCase();
  for (const forbidden of ['executescript', 'dynamicimport', 'pluginloader', 'commandbridge', 'browsertrustedwrite']) {
    assert.equal(serialized.includes(`"${forbidden}"`), false, `${forbidden} is not a fixture field or action`);
  }
}

const draft = readJson('drafts/behavior-draft.gl10.14.2.fixture.json');
const apply = readJson('applies/behavior-apply.gl10.14.2.fixture.json');
const bundle = readJson('evidence/behavior-evidence-bundle.gl10.14.2.fixture.json');
const stale = readJson('evidence/behavior-evidence-bundle.stale.fixture.json');
const journal = fs.readFileSync(path.join(fixtureDir, 'journal/behavior-draft-apply-evidence.fragment.md'), 'utf8');

assert.equal(draft.schemaVersion, 'ouroforge.behavior-draft.v1');
assert.equal(draft.validationStatus, 'drafted');
assert.equal(draft.target.scenePath, 'scenes/gameplay-logic-regression-v9.scene.json');
assert.equal(draft.proposedBehavior.artifactId, 'gameplay-logic-regression-v9');
assert.ok(draft.untrustedBoundary.includes('does not apply trusted files'));
assert.equal(draft.linkedEvidence.length, 2);
// Each linked evidence ref must resolve to an actual fixture-scoped artifact so a
// draft/evidence reader can follow it (regression for the stale-path defect where
// linkedEvidence pointed at evidence/gameplay-logic-regression-v9/... instead of
// the real examples/gameplay-logic-regression-v9/evidence/... files).
for (const link of draft.linkedEvidence) {
  assert.ok(
    fs.existsSync(path.join(repoRoot, link.path)),
    `linked evidence ${link.id} path resolves to a fixture artifact: ${link.path}`,
  );
}
assertNoForbiddenAuthority(draft.proposedBehavior);

assert.equal(apply.schemaVersion, 'ouroforge.behavior-apply-transaction.v1');
assert.equal(apply.status, 'readyForTrustedApply');
assert.notEqual(apply.reviewDecision.reviewerId, apply.reviewDecision.draftAuthorId);
assert.equal(apply.targetHashes.expectedBeforeHash, apply.target.sceneHash);
assert.equal(apply.targetHashes.observedBeforeHash, apply.rollbackMetadata.beforeHash);
assert.match(apply.transactionOutputRef, /^runs\/behavior-applies\/gameplay-logic-regression-v9\//);
assert.ok(apply.trustedBoundary.includes('Accepted review'));
assert.ok(apply.trustedBoundary.includes('rollback'));
assert.deepEqual(apply.rerunCommand.argv.slice(0, 5), ['ouroforge', 'behavior', 'apply', 'transaction', 'validate']);
assertNoForbiddenAuthority(apply.proposedBehavior);

assert.equal(bundle.schemaVersion, 'behavior-evidence-bundle-v1');
assert.equal(bundle.status, 'complete');
assert.equal(bundle.draftRefs[0].path, 'examples/gameplay-logic-regression-v9/drafts/behavior-draft.gl10.14.2.fixture.json');
assert.equal(bundle.applyTransactionRefs[0].path, 'examples/gameplay-logic-regression-v9/applies/behavior-apply.gl10.14.2.fixture.json');
assert.equal(bundle.linkedEvidence.length, 8);
for (const required of ['behavior-definition', 'runtime-event-log', 'scenario-outcome', 'behavior-draft', 'behavior-review-decision', 'behavior-apply-transaction', 'behavior-rollback-metadata', 'behavior-rerun-comparison']) {
  assert.ok(bundle.linkedEvidence.some((entry) => entry.kind === required), `${required} evidence is linked`);
}
assert.ok(bundle.guardrails.some((guardrail) => /#1 and #23 remain open/.test(guardrail)));
assert.ok(bundle.guardrails.some((guardrail) => /no arbitrary script execution/.test(guardrail)));

assert.equal(stale.status, 'stale');
assert.ok(stale.blockedReasons.some((reason) => /stale/.test(reason)));
assert.equal(stale.applyTransactionRefs.length, 0, 'stale fixture visibly omits later apply refs');

assert.match(journal, /Draft: `draft-gameplay-logic-regression-v9-routing`/);
assert.match(journal, /Review\/apply:/);
assert.match(journal, /no arbitrary script execution/);

for (const name of ['runs', 'dashboard-data', 'target', 'tmp']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
}

console.log('scenario coverage v9 draft/apply/evidence smoke passed');
