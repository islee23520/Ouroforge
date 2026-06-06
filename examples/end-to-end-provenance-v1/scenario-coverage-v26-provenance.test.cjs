const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const fixtureRoot = path.join(root, 'scenario-coverage-v26');

const requiredLinkKinds = [
  'intent-design-brief',
  'generated-edited-artifact',
  'validation-result',
  'runtime-observation',
  'evaluator-verdict',
  'regression-comparison',
  'journal-review-decision',
  'promotion-rollback-record',
];

function readJson(relativePath) {
  const fullPath = path.join(fixtureRoot, relativePath);
  return JSON.parse(fs.readFileSync(fullPath, 'utf8'));
}

function assertLocalRef(reference, label) {
  assert.equal(typeof reference, 'string', `${label} ref must be a string`);
  assert.ok(reference.length > 0, `${label} ref must not be empty`);
  assert.equal(path.isAbsolute(reference), false, `${label} ref must be relative`);
  assert.equal(reference.includes('\\'), false, `${label} ref must use forward slashes`);
  assert.equal(reference.split('/').includes('..'), false, `${label} ref must stay fixture-local`);
}

function evaluateBundle(bundle) {
  assert.equal(bundle.schemaVersion, 'provenance-bundle-v1');
  assert.equal(typeof bundle.bundleId, 'string');
  assert.equal(typeof bundle.changeId, 'string');
  assert.ok(['complete', 'incomplete', 'dangling', 'stale'].includes(bundle.status));
  assert.ok(Array.isArray(bundle.chainLinks));
  assert.ok(bundle.chainLinks.length > 0);

  const states = new Map();
  const seen = new Set();
  const issues = [];

  for (const kind of requiredLinkKinds) {
    states.set(kind, 'missing');
    issues.push(`missing chain link: ${kind}`);
  }

  for (const link of bundle.chainLinks) {
    assert.ok(requiredLinkKinds.includes(link.kind), `unsupported chain link kind ${link.kind}`);
    assert.equal(seen.has(link.kind), false, `duplicate chain link kind ${link.kind}`);
    seen.add(link.kind);
    assertLocalRef(link.ref, `${bundle.bundleId}:${link.kind}`);
    assert.equal(typeof link.artifactId, 'string', `${link.kind} artifactId`);
    assert.ok(link.artifactId.length > 0, `${link.kind} artifactId must not be empty`);

    const missingIndex = issues.indexOf(`missing chain link: ${link.kind}`);
    if (missingIndex >= 0) {
      issues.splice(missingIndex, 1);
    }

    if (link.stale) {
      assert.equal(typeof link.staleReason, 'string', `${link.kind} staleReason`);
      states.set(link.kind, 'stale');
    } else if (fs.existsSync(path.join(fixtureRoot, link.ref))) {
      states.set(link.kind, 'present');
    } else {
      states.set(link.kind, 'dangling');
      issues.push(`dangling reference: ${link.ref}`);
    }
  }

  assert.equal(typeof bundle.generatedState, 'object');
  assert.equal(typeof bundle.generatedState.generated, 'boolean');
  assert.equal(typeof bundle.generatedState.tracked, 'boolean');
  assert.equal(typeof bundle.generatedState.fixtureScoped, 'boolean');
  assert.notEqual(
    bundle.generatedState.generated && bundle.generatedState.tracked && !bundle.generatedState.fixtureScoped,
    true,
    `${bundle.bundleId} tracks generated state without fixture scope`,
  );

  assert.ok(Array.isArray(bundle.incompleteReasons));
  assert.ok(Array.isArray(bundle.compatibilityNotes));
  assert.ok(Array.isArray(bundle.guardrails));
  assert.match(bundle.boundary, /read-only/i);
  assert.match(bundle.boundary, /audit/i);
  assert.match(bundle.boundary, /replay/i);

  for (const guardrail of bundle.guardrails) {
    assert.equal(typeof guardrail, 'string');
  }

  if (bundle.replayInputs) {
    assertLocalRef(bundle.replayInputs.runRef, `${bundle.bundleId}:runRef`);
    assertLocalRef(bundle.replayInputs.expectedVerdictRef, `${bundle.bundleId}:expectedVerdictRef`);
    assert.equal(typeof bundle.replayInputs.deterministicInputs, 'boolean');
    assert.ok(Array.isArray(bundle.replayInputs.deterministicMetadataRefs));
    for (const reference of bundle.replayInputs.deterministicMetadataRefs) {
      assertLocalRef(reference, `${bundle.bundleId}:deterministicMetadataRefs`);
    }
  }

  if (bundle.incompleteReasons.length > 0) {
    for (const reason of bundle.incompleteReasons) {
      assert.equal(typeof reason, 'string');
      assert.ok(reason.trim().length > 0);
    }
    issues.push(...bundle.incompleteReasons.map((reason) => `incomplete bundle state: ${reason}`));
  }

  const computedStatus = [...states.values()].includes('dangling')
    ? 'dangling'
    : [...states.values()].includes('stale')
      ? 'stale'
      : issues.length === 0
        ? 'complete'
        : 'incomplete';

  return {
    computedStatus,
    linkStates: Object.fromEntries(states.entries()),
    issues,
  };
}

const matrix = readJson('matrix.fixture.json');
assert.equal(matrix.schemaVersion, 'scenario-coverage-v26-provenance-matrix-v1');
assert.equal(matrix.issue, 1505);
assert.match(matrix.generatedStatePolicy, /fixture-scoped/i);
assert.match(matrix.generatedStatePolicy, /ignored/i);

for (const bundleCase of matrix.bundleCases) {
  const bundle = readJson(bundleCase.fixture);
  const evaluation = evaluateBundle(bundle);
  assert.equal(evaluation.computedStatus, bundleCase.expectedStatus, bundleCase.id);
  assert.equal(bundle.status, bundleCase.expectedStatus, `${bundleCase.id} declared status`);
  if (bundleCase.requiresIncompleteReasons) {
    assert.ok(bundle.incompleteReasons.length > 0, `${bundleCase.id} needs visible reasons`);
  }
  for (const [kind, state] of Object.entries(bundleCase.requiredLinkStates)) {
    assert.equal(evaluation.linkStates[kind], state, `${bundleCase.id}:${kind}`);
  }
}

for (const replayCase of matrix.replayCases) {
  const bundle = readJson(replayCase.bundleFixture);
  const result = readJson(replayCase.resultFixture);
  assert.equal(result.schemaVersion, 'provenance-replay-result-v1', replayCase.id);
  assert.equal(result.bundleId, bundle.bundleId, `${replayCase.id} bundle link`);
  assert.equal(result.status, replayCase.expectedStatus, `${replayCase.id} replay status`);
  assert.ok(['reproduced', 'diverged', 'not-replayable'].includes(result.status));
  assert.ok(Array.isArray(result.diff), `${replayCase.id} diff shape`);
  assert.ok(Array.isArray(result.issues), `${replayCase.id} issue shape`);
  assert.match(result.boundary, /local/i);
  assert.match(result.boundary, /no command execution/i);
  if (replayCase.expectedStatus === 'reproduced') {
    assert.deepEqual(result.diff, [], `${replayCase.id} reproduced diff`);
    assert.deepEqual(result.issues, [], `${replayCase.id} reproduced issues`);
    assert.deepEqual(result.expectedVerdict, result.actualVerdict, `${replayCase.id} verdicts`);
  }
  if (replayCase.requiresDiff) {
    assert.ok(result.diff.some((entry) => entry.path === '$.status'), `${replayCase.id} diff path`);
  }
  if (replayCase.requiresIssues) {
    assert.ok(result.issues.length > 0, `${replayCase.id} issues visible`);
    assert.equal(result.actualVerdict, null, `${replayCase.id} actual verdict`);
  }
}

for (const compatibilityCase of matrix.compatibilityCases) {
  const golden = readJson(compatibilityCase.fixture);
  assert.equal(golden.status, compatibilityCase.expectedStatus, compatibilityCase.id);
  assert.equal(Object.hasOwn(golden, compatibilityCase.mustOmitField), false, compatibilityCase.id);
  assert.ok(
    golden.compatibilityNotes.some((note) => /additive/i.test(note)),
    `${compatibilityCase.id} additive note`,
  );
}

const requiredText = [
  fs.readFileSync(path.join(root, '..', '..', 'docs', 'scenario-coverage-v26.md'), 'utf8'),
  fs.readFileSync(path.join(fixtureRoot, 'matrix.fixture.json'), 'utf8'),
  ...matrix.bundleCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.fixture), 'utf8')),
  ...matrix.replayCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.resultFixture), 'utf8')),
  ...matrix.compatibilityCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.fixture), 'utf8')),
].join('\n');

const wordingText = [
  fs.readFileSync(path.join(root, '..', '..', 'docs', 'scenario-coverage-v26.md'), 'utf8'),
  ...matrix.bundleCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.fixture), 'utf8')),
  ...matrix.replayCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.resultFixture), 'utf8')),
  ...matrix.compatibilityCases.map((entry) => fs.readFileSync(path.join(fixtureRoot, entry.fixture), 'utf8')),
].join('\n');

for (const phrase of matrix.audits.requiredPhrases) {
  assert.match(requiredText, new RegExp(phrase.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i'), phrase);
}

const lower = wordingText.toLowerCase();
for (const phrase of matrix.audits.forbiddenPhrases) {
  assert.equal(lower.includes(phrase.toLowerCase()), false, `forbidden wording: ${phrase}`);
}

console.log('scenario coverage v26 provenance regression matrix passed');
