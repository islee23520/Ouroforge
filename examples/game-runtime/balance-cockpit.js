// Balance Cockpit Read-Only Surface and Re-Run Diff v1 (#1608, part of Synthetic
// Player Balance v1 #1605 under #1 Era F Milestone 32).
//
// Two read-only capabilities over a balance report (#1607):
//   1. surfaceBalanceReport: a read-only, HTML-escaped rendering of the report
//      with per-flag counterexample/replay seeds. No execution, no mutation.
//   2. rerunWithChange: apply a PROPOSED balance change (e.g. a nerf) to a COPY
//      of the deck spec, re-run the identical seed distribution (the same
//      persona roster #1606), and diff the win-rate impact.
//
// The cockpit is read-only and human-in-the-loop: a balance change is a proposal
// only — never auto-applied, never a trusted write. `applyBalanceChange` returns
// a new deck spec and never mutates its input. The win-rate diff reuses the
// existing compare contract: it compares the two reports' digests (the same
// digest-equality signal the runtime `compareReplayDigest` uses) and mirrors the
// compare evidence shape (status + policy with `browserWriteAccess: 'none'`); the
// full evidence comparison remains the existing `compare` CLI, surfaced here as a
// read-only command. Aggregation is deterministic and integer-only, so the Rust
// mirror reproduces the same diff digest byte-for-byte.
(() => {
  const SURFACE_SCHEMA = 'ouroforge.balance-cockpit-surface.v1';
  const DIFF_SCHEMA = 'ouroforge.balance-rerun-diff.v1';
  const CHANGE_FIELDS = ['cost', 'damage', 'block'];

  function fail(message) {
    throw new Error(`balance cockpit invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function isNonNegativeInt(value) {
    return Number.isInteger(value) && value >= 0;
  }

  // HTML-escape for read-only surfacing (same discipline as the cockpit).
  function escapeText(value) {
    return String(value === undefined || value === null ? '' : value)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  // Apply a proposed balance change to a COPY of the deck spec. Pure: the input
  // spec is never mutated and nothing is written. Fails closed on an unknown
  // card or a malformed field — a change can never silently no-op.
  function applyBalanceChange(deckSpec, change) {
    if (!isPlainObject(deckSpec) || !isPlainObject(deckSpec.cards)) {
      fail('deck spec must declare a cards vocabulary');
    }
    if (!isPlainObject(change) || typeof change.card !== 'string') {
      fail('change must declare a card');
    }
    if (!Object.prototype.hasOwnProperty.call(deckSpec.cards, change.card)) {
      fail(`change references undeclared card "${change.card}"`);
    }
    const applied = CHANGE_FIELDS.filter((field) => change[field] !== undefined);
    if (applied.length === 0) {
      fail(`change for card "${change.card}" must set at least one of ${CHANGE_FIELDS.join(', ')}`);
    }
    const next = clone(deckSpec);
    for (const field of applied) {
      if (!isNonNegativeInt(change[field])) {
        fail(`change ${field} for card "${change.card}" must be a non-negative integer`);
      }
      next.cards[change.card][field] = change[field];
    }
    return next;
  }

  // Canonical serialization of a change for the diff digest.
  function changeKey(change) {
    const fields = CHANGE_FIELDS.filter((field) => change[field] !== undefined)
      .map((field) => `${field}=${change[field]}`)
      .join(',');
    return `${change.card}[${fields}]`;
  }

  function degenerateKeys(report) {
    return report.degenerateCombos.map((g) => g.cards.join('+')).slice().sort();
  }

  function deadKeys(report) {
    return report.deadItems.map((d) => d.card).slice().sort();
  }

  function setDiff(before, after) {
    const afterSet = new Set(after);
    const beforeSet = new Set(before);
    return {
      resolved: before.filter((x) => !afterSet.has(x)),
      introduced: after.filter((x) => !beforeSet.has(x)),
    };
  }

  // Diff two balance reports for a proposed change. Reuses the compare
  // digest-equality signal (status) and mirrors the compare evidence shape.
  function diffBalanceReports(before, after, change) {
    if (!isPlainObject(before) || !isPlainObject(after)) fail('diff requires two reports');
    const status = before.digest === after.digest ? 'unchanged' : 'changed';
    const cardsBefore = new Map(before.cards.map((c) => [c.card, c.plays]));
    const cardsAfter = new Map(after.cards.map((c) => [c.card, c.plays]));
    const vocab = Array.from(new Set([...cardsBefore.keys(), ...cardsAfter.keys()])).sort();
    const cardDeltas = vocab.map((card) => {
      const playsBefore = cardsBefore.get(card) || 0;
      const playsAfter = cardsAfter.get(card) || 0;
      return { card, playsBefore, playsAfter, delta: playsAfter - playsBefore };
    });
    const degen = setDiff(degenerateKeys(before), degenerateKeys(after));
    const dead = setDiff(deadKeys(before), deadKeys(after));
    const winRate = {
      before: { wins: before.winRate.wins, total: before.winRate.total },
      after: { wins: after.winRate.wins, total: after.winRate.total },
      deltaWins: after.winRate.wins - before.winRate.wins,
    };
    const diff = {
      schemaVersion: DIFF_SCHEMA,
      status,
      change: clone(change),
      baselineDigest: before.digest,
      candidateDigest: after.digest,
      winRate,
      cardDeltas,
      flagChanges: {
        degenerateResolved: degen.resolved,
        degenerateIntroduced: degen.introduced,
        deadResolved: dead.resolved,
        deadIntroduced: dead.introduced,
      },
      policy: {
        rootKind: 'generated_evidence',
        trustedWriter: 'rust-local-balance-telemetry-v1',
        browserWriteAccess: 'none',
        autoApplied: false,
        retention: 'generated balance re-run diff; untracked unless fixture-scoped',
      },
    };
    diff.digest = diffDigest(diff);
    return diff;
  }

  function diffDigest(diff) {
    const cards = diff.cardDeltas
      .map((d) => `${d.card}:${d.playsBefore}->${d.playsAfter}`)
      .join(',');
    return [
      `rerun-diff|status=${diff.status}`,
      `change=${changeKey(diff.change)}`,
      `wr=${diff.winRate.before.wins}/${diff.winRate.before.total}->${diff.winRate.after.wins}/${diff.winRate.after.total}(${diff.winRate.deltaWins})`,
      `cards=${cards}`,
      `degenResolved=${diff.flagChanges.degenerateResolved.join('+')}`,
      `degenIntroduced=${diff.flagChanges.degenerateIntroduced.join('+')}`,
      `deadResolved=${diff.flagChanges.deadResolved.join('+')}`,
      `deadIntroduced=${diff.flagChanges.deadIntroduced.join('+')}`,
    ].join('|');
  }

  // Apply a proposed change to a copy of the deck spec, re-run the identical
  // persona roster (the same seed distribution), and diff the win-rate impact.
  // `deck`, `synthetic`, `telemetry` are the existing modules (injected so this
  // stays pure and testable). Never mutates the input spec; never writes.
  function rerunWithChange(deck, synthetic, telemetry, deckSpec, rosterSpec, change) {
    const baselineRuns = synthetic.playRoster(deck, deckSpec, rosterSpec);
    const baseline = telemetry.aggregate(baselineRuns, telemetry.vocabularyOf(deckSpec), deckSpec.id);
    const candidateSpec = applyBalanceChange(deckSpec, change);
    const candidateRuns = synthetic.playRoster(deck, candidateSpec, rosterSpec);
    const candidate = telemetry.aggregate(candidateRuns, telemetry.vocabularyOf(candidateSpec), candidateSpec.id);
    return diffBalanceReports(baseline, candidate, change);
  }

  // Read-only, HTML-escaped surfacing of a balance report with per-flag
  // counterexample/replay seeds. No execution, no mutation, no trusted write.
  function surfaceBalanceReport(report) {
    if (!isPlainObject(report)) fail('surface requires a balance report');
    return {
      schemaVersion: SURFACE_SCHEMA,
      scene: escapeText(report.scene),
      winRate: `${report.wins}/${report.totalRuns}`,
      cards: report.cards.map((c) => ({
        card: escapeText(c.card),
        plays: c.plays,
        winsIncluded: c.winsIncluded,
      })),
      degenerateFlags: report.degenerateCombos.map((g) => ({
        cards: g.cards.map(escapeText),
        counterexample: `seed ${escapeText(g.replay.deckSeed)} / persona ${escapeText(g.replay.persona)}`,
      })),
      deadFlags: report.deadItems.map((d) => ({
        card: escapeText(d.card),
        counterexample: `seed ${escapeText(d.replay.deckSeed)} / persona ${escapeText(d.replay.persona)}`,
      })),
      readOnlyInspection: {
        trustedEmitter: 'balance-cockpit-surface',
        browserStudioMode: 'read-only escaped balance surfacing; no execution',
        disallowedActions: ['trusted writes', 'auto-applied nerf or buff', 'command execution', 'live mutation'],
      },
    };
  }

  // Surface the existing `compare` CLI command (read-only) for a full evidence
  // comparison of two run directories. Display only; never executed here.
  function compareCommand(beforeRun = 'runs/before', afterRun = 'runs/after', outputDir = `${afterRun}/comparisons`) {
    return `cargo run -p ouroforge-cli -- compare ${beforeRun} ${afterRun} --output-dir ${outputDir}`;
  }

  const api = {
    SURFACE_SCHEMA,
    DIFF_SCHEMA,
    escapeText,
    applyBalanceChange,
    diffBalanceReports,
    rerunWithChange,
    surfaceBalanceReport,
    compareCommand,
  };

  if (typeof window !== 'undefined') {
    window.OuroforgeBalanceCockpit = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
