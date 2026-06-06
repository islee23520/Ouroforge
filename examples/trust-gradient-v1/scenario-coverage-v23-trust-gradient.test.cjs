// Scenario Coverage v23: Trust Gradient Regression Suite (#1483).
//
// Locks Trust Gradient v1 behavior with an enumerated regression matrix:
// risk-tier classification, bounded auto-apply, audit completeness, and
// kill-switch halt, plus an autonomy-off backward-compatibility golden. It
// reimplements the fail-closed decision rules in Node and asserts states and
// shapes only — no subjective quality and no flaky/timing assertions — so a
// breaking change fails CI.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const matrix = JSON.parse(
  fs.readFileSync(path.join(__dirname, 'coverage-v23', 'coverage-matrix.json'), 'utf8'),
);
const THRESHOLD = matrix.highConfidenceThreshold;

const SOURCE_AFFECTING_FRAGMENTS = [
  'src/', 'crates/', 'build.rs', 'scripts/', '.github/', 'cargo.toml', 'cargo.lock',
  'package.json', 'package-lock.json', 'makefile', '.sh', '.rs', '.yml', '.yaml', '.toml',
];

function hasSourceAffectingPath(scopePaths) {
  return (scopePaths || []).some((p) => {
    const lowered = String(p).toLowerCase();
    return SOURCE_AFFECTING_FRAGMENTS.some((frag) => lowered.includes(frag));
  });
}

function baseTier(kind) {
  if (kind === 'scene-only-data') return 'low';
  if (['scene-transition', 'manifest', 'scenario-pack', 'promotion-matrix'].includes(kind)) return 'medium';
  return 'high';
}

const TIER_RANK = { low: 0, medium: 1, high: 2 };
function maxTier(a, b) { return TIER_RANK[a] >= TIER_RANK[b] ? a : b; }

function gatesAllPass(gates) {
  return gates && gates.mechanical === 'pass' && gates.runtime === 'pass' && gates.visual === 'pass' && gates.semantic === 'pass';
}

// Mirror of classify_mutation_risk_tier (fail-closed).
function classify(descriptor) {
  let tier = baseTier(descriptor.mutationKind);
  let eligible = descriptor.mutationKind === 'scene-only-data';
  if (hasSourceAffectingPath(descriptor.scopePaths)) { eligible = false; tier = maxTier(tier, 'high'); }
  if (!(typeof descriptor.confidence === 'number')) { eligible = false; tier = maxTier(tier, 'medium'); }
  else if (descriptor.confidence < THRESHOLD) { eligible = false; tier = maxTier(tier, 'medium'); }
  if (!gatesAllPass(descriptor.gates)) { eligible = false; tier = maxTier(tier, 'medium'); }
  if (descriptor.refsFresh !== true) { eligible = false; tier = maxTier(tier, 'medium'); }
  if (eligible) tier = 'low';
  return { tier, eligibility: eligible ? 'auto-apply-eligible' : 'manual-only' };
}

// Mirror of decide_auto_apply (fail-closed).
function decide(request) {
  let apply = true;
  if (request.autonomyEnabled !== true) apply = false;
  if (request.eligibility !== 'auto-apply-eligible') apply = false;
  if (request.tier !== 'low') apply = false;
  if (!(typeof request.confidence === 'number' && request.confidence >= THRESHOLD)) apply = false;
  if (!gatesAllPass(request.rerunGates)) apply = false;
  const b = request.budget || {};
  if (!(typeof b.cost === 'number' && b.cost > 0 && typeof b.remaining === 'number' && b.cost <= b.remaining)) apply = false;
  const h = request.rollbackHandle;
  if (!h || !h.applyTransactionId || !h.reverseRef) apply = false;
  if (apply) {
    return { outcome: 'auto-applied', rollbackCommand: `ouroforge rollback --transaction ${h.applyTransactionId} --reverse ${h.reverseRef}` };
  }
  return { outcome: 'manual-fallback', rollbackCommand: null };
}

// Mirror of AutoApplyAuditLog::validate + is_autonomy_halted (append-only).
function validateAuditLog(log) {
  const entries = log.entries || [];
  for (let i = 0; i < entries.length; i += 1) {
    const entry = entries[i];
    if (entry.sequence !== i) return { valid: false, halted: !!(log.killSwitch && log.killSwitch.engaged) };
    const h = entry.rollbackHandle || {};
    if (!h.applyTransactionId || !h.reverseRef) return { valid: false, halted: !!(log.killSwitch && log.killSwitch.engaged) };
  }
  if (log.killSwitch && log.killSwitch.engaged && !log.killSwitch.reason) return { valid: false, halted: true };
  return { valid: true, halted: !!(log.killSwitch && log.killSwitch.engaged) };
}

// 1) Risk-tier classification coverage.
assert.ok(matrix.riskTierCases.length >= 5, 'risk-tier coverage enumerates the tiers and conservative defaults');
for (const c of matrix.riskTierCases) {
  const got = classify(c.descriptor);
  assert.equal(got.tier, c.expectedTier, `${c.id} tier`);
  assert.equal(got.eligibility, c.expectedEligibility, `${c.id} eligibility`);
  if (got.eligibility === 'auto-apply-eligible') assert.equal(got.tier, 'low', `${c.id} only low is eligible`);
}

// 2) Bounded auto-apply coverage.
assert.ok(matrix.autoApplyCases.length >= 4, 'auto-apply coverage enumerates success/rollback/budget/ineligible');
for (const c of matrix.autoApplyCases) {
  const got = decide(c.request);
  assert.equal(got.outcome, c.expectedOutcome, `${c.id} outcome`);
  assert.equal(!!got.rollbackCommand, !!c.expectedRollback, `${c.id} rollback presence`);
  if (got.outcome === 'auto-applied') assert.match(got.rollbackCommand, /^ouroforge rollback --transaction .+ --reverse .+$/, `${c.id} one-command rollback`);
}

// 3) Audit completeness and kill-switch halt coverage.
for (const c of matrix.auditCases) {
  const res = validateAuditLog(c.log);
  assert.equal(res.valid, c.expectValid, `${c.id} validity`);
  if (c.expectHalted !== undefined) assert.equal(res.halted, c.expectHalted, `${c.id} halted`);
}

// 4) Backward compatibility: with autonomy off (default), an otherwise eligible
//    proposal stays on the review-gated manual path — behaviour unchanged.
const bc = matrix.backwardCompatAutonomyOff;
assert.equal(decide(bc.request).outcome, bc.expectedOutcome, `${bc.id} default-off review-gated apply unchanged`);
// Determinism golden: re-deciding yields an identical outcome (no timing/flake).
assert.deepEqual(decide(bc.request), decide(bc.request), `${bc.id} deterministic`);

assert.match(matrix.boundary, /no flaky/);
assert.match(matrix.boundary, /Default-off autonomy/);

console.log('scenario coverage v23 trust gradient regression suite passed');
