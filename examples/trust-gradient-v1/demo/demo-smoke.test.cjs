// Trust Gradient Demo v1 smoke test (#1481).
//
// Deterministic, fixture-scoped demonstration that a low-risk proposal
// auto-applies within budget (audited, reversible), a high-risk proposal falls
// back to manual review, budget exhaustion falls back to manual, and the kill
// switch halts autonomy. It reimplements the bounded auto-apply decision rules
// in Node and asserts every demo fixture produces its documented outcome,
// without network, a live browser, or any trusted write. No demo-only writer
// exists: the demonstration is evidence-only.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const demoDir = __dirname;
const HIGH_CONFIDENCE_THRESHOLD = 0.9;

function fixture(name) {
  return JSON.parse(fs.readFileSync(path.join(demoDir, name), 'utf8'));
}

function gatesAllPass(gates) {
  return gates
    && gates.mechanical === 'pass'
    && gates.runtime === 'pass'
    && gates.visual === 'pass'
    && gates.semantic === 'pass';
}

// Mirror of the Rust bounded auto-apply decision (fail-closed). Returns the
// outcome and, when applied, the single rollback command.
function decideAutoApply(request) {
  const reasons = [];
  let apply = true;
  if (request.autonomyEnabled !== true) { apply = false; reasons.push('autonomy disabled (kill switch / default no-auto-apply)'); }
  if (request.eligibility !== 'auto-apply-eligible') { apply = false; reasons.push('not auto-apply eligible'); }
  if (request.tier !== 'low') { apply = false; reasons.push('risk tier is not low'); }
  if (!(typeof request.confidence === 'number' && request.confidence >= HIGH_CONFIDENCE_THRESHOLD)) { apply = false; reasons.push('confidence missing or below threshold'); }
  if (!gatesAllPass(request.rerunGates)) { apply = false; reasons.push('not all four gates pass on rerun'); }
  const budget = request.budget || {};
  if (!(typeof budget.cost === 'number' && budget.cost > 0 && typeof budget.remaining === 'number' && budget.cost <= budget.remaining)) { apply = false; reasons.push('risk budget exhausted'); }
  const handle = request.rollbackHandle;
  if (!handle || !handle.applyTransactionId || !handle.reverseRef) { apply = false; reasons.push('no rollback handle'); }
  if (apply) {
    return {
      outcome: 'auto-applied',
      rollbackCommand: `ouroforge rollback --transaction ${handle.applyTransactionId} --reverse ${handle.reverseRef}`,
      budgetAfter: { remaining: budget.remaining - budget.cost, cost: budget.cost },
      reasons,
    };
  }
  return { outcome: 'manual-fallback', rollbackCommand: null, budgetAfter: budget, reasons };
}

function assertScenario(name, expectedOutcome) {
  const scenario = fixture(name);
  assert.equal(scenario.schemaVersion, 'trust-gradient-demo-v1', `${name} schemaVersion`);
  assert.match(scenario.boundary, /not auto-merge/);
  assert.match(scenario.boundary, /not self-approval/);
  assert.match(scenario.boundary, /reversible/);
  const decision = decideAutoApply(scenario.request);
  assert.equal(decision.outcome, expectedOutcome, `${name} outcome`);
  assert.equal(decision.outcome, scenario.expectedOutcome, `${name} matches documented expectedOutcome`);
  return { scenario, decision };
}

// 1) Low-risk proposal auto-applies within budget: audited and reversible.
const low = assertScenario('low-risk-auto-apply.json', 'auto-applied');
assert.ok(low.decision.rollbackCommand, 'auto-applied scenario yields a one-command rollback');
assert.match(low.decision.rollbackCommand, /^ouroforge rollback --transaction .+ --reverse .+$/);
assert.equal(low.decision.rollbackCommand, low.scenario.expectedRollbackCommand, 'rollback command matches documented command');
assert.ok(low.decision.budgetAfter.remaining < low.scenario.request.budget.remaining, 'budget is consumed on apply');
// The applied scenario carries a complete append-only audit entry.
const auditEntry = low.scenario.auditEntry;
assert.equal(auditEntry.sequence, 0);
assert.equal(auditEntry.applyResult, 'auto-applied');
assert.ok(auditEntry.rollbackHandle.applyTransactionId && auditEntry.rollbackHandle.reverseRef, 'audit entry has an intact rollback handle');

// 2) High-risk proposal falls back to manual review (nothing source-affecting auto-applies).
assertScenario('high-risk-manual-fallback.json', 'manual-fallback');

// 3) Budget exhaustion falls back to manual review.
assertScenario('budget-exhausted-fallback.json', 'manual-fallback');

// 4) Kill switch halts autonomy: eligible proposal still falls back to manual.
const halt = assertScenario('kill-switch-halt.json', 'manual-fallback');
assert.equal(halt.scenario.killSwitch.engaged, true, 'kill-switch scenario records the engaged switch');
assert.ok(halt.scenario.killSwitch.reason, 'engaged kill switch records a reason');

console.log('trust gradient demo smoke test passed');
