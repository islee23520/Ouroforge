// Balance Telemetry Aggregation v1 (#1607, part of Synthetic Player Balance v1
// #1605 under #1 Era F Milestone 32).
//
// Aggregates balance telemetry over many seeded synthetic-player runs (#1606)
// into a descriptive balance-report artifact: per-card pick-rate and win
// inclusion, a difficulty curve, plus degenerate-combo and dead-item flags that
// each carry a replayable seed. This aggregates over EXISTING persona-run
// evidence (the run records produced by `synthetic-player.js`) — it is not a new
// simulation, runtime, engine, or solver. The aggregation is pure and
// deterministic: the same run records and card vocabulary always produce the
// same report and the same report digest.
//
// The metrics are DESCRIPTIVE, not a balance/quality guarantee, and never an
// auto-applied nerf or buff: flags are read-only observations a human reviews.
// Generation/observation is proposal-only and the browser/Studio surface stays
// read-only. The trusted aggregation/detector logic is owned by Rust/local; this
// module reproduces the same observable report for the browser-local dashboard,
// and the integer-only thresholds keep the two byte-for-byte identical.
(() => {
  const REPORT_SCHEMA = 'ouroforge.balance-report.v1';
  // A card is flagged degenerate when it both drives wins (played in at least
  // one winning run) and dominates usage: its share of all card plays is at
  // least DEGEN_SHARE_PCT percent. Share, not mere inclusion, is the signal —
  // personas dump their affordable hand each turn, so over a multi-turn run
  // nearly every card is played at least once, which makes inclusion saturate.
  // Integer comparison: plays * 100 >= totalPlays * DEGEN_SHARE_PCT.
  const DEGEN_SHARE_PCT = 30;
  // A degenerate pair is flagged when both members are degenerate and co-played
  // in at least 90% of winning runs (included * DEN >= totalWins * NUM).
  const DEGEN_NUM = 9;
  const DEGEN_DEN = 10;

  function fail(message) {
    throw new Error(`balance telemetry invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  // The sorted card vocabulary declared by a deck spec. Dead-item detection needs
  // the full vocabulary, not only the cards that happened to be played.
  function vocabularyOf(deckSpec) {
    if (!isPlainObject(deckSpec) || !isPlainObject(deckSpec.cards)) {
      fail('deck spec must declare a cards vocabulary');
    }
    return Object.keys(deckSpec.cards).slice().sort();
  }

  function replaySeed(run) {
    return { deckSeed: run.deckSeed, persona: run.personaId, personaSeed: run.seed };
  }

  function playsOf(run, card) {
    return run.cardPlays && Number.isInteger(run.cardPlays[card]) ? run.cardPlays[card] : 0;
  }

  // Aggregate run records over a card vocabulary into a balance report. Pure and
  // deterministic; `runs` is the list of synthetic-player run records, `vocab` a
  // sorted card-id list, `sceneId` the scene identifier the runs share.
  function aggregate(runs, vocab, sceneId) {
    if (!Array.isArray(runs) || runs.length === 0) fail('runs must be a non-empty array');
    if (!Array.isArray(vocab) || vocab.length === 0) fail('vocabulary must be a non-empty array');
    const totalRuns = runs.length;
    const winningRuns = runs.filter((r) => r.outcome === 'won');
    const totalWins = winningRuns.length;
    const losses = runs.filter((r) => r.outcome === 'lost').length;
    const playing = runs.filter((r) => r.outcome === 'playing').length;

    let totalPlays = 0;
    for (const run of runs) {
      for (const card of vocab) totalPlays += playsOf(run, card);
    }

    const cards = vocab.map((card) => {
      let plays = 0;
      let runsIncluded = 0;
      let winsIncluded = 0;
      let lossesIncluded = 0;
      for (const run of runs) {
        const count = playsOf(run, card);
        plays += count;
        if (count > 0) {
          runsIncluded += 1;
          if (run.outcome === 'won') winsIncluded += 1;
          else if (run.outcome === 'lost') lossesIncluded += 1;
        }
      }
      return { card, plays, runsIncluded, winsIncluded, lossesIncluded };
    });

    // Dead items: declared in the vocabulary but never played in any run.
    const deadItems = cards
      .filter((c) => c.plays === 0)
      .map((c) => ({ card: c.card, plays: 0, replay: replaySeed(runs[0]) }));

    // Degenerate cards: dominate usage (>= DEGEN_SHARE_PCT of all plays) and
    // drive wins. Each carries a replayable seed (the first winning run that
    // played it).
    const degenerateCards = totalWins > 0
      ? cards.filter((c) => c.winsIncluded > 0 && c.plays * 100 >= totalPlays * DEGEN_SHARE_PCT)
      : [];
    const degenerateCombos = [];
    for (const c of degenerateCards) {
      const run = winningRuns.find((r) => playsOf(r, c.card) > 0);
      degenerateCombos.push({
        cards: [c.card],
        share: { plays: c.plays, totalPlays },
        winInclusion: { included: c.winsIncluded, totalWins },
        replay: replaySeed(run),
      });
    }
    // Degenerate combos: unordered pairs of degenerate cards co-played in >= 90%
    // of winning runs.
    for (let i = 0; i < degenerateCards.length; i += 1) {
      for (let j = i + 1; j < degenerateCards.length; j += 1) {
        const a = degenerateCards[i].card;
        const b = degenerateCards[j].card;
        const coRuns = winningRuns.filter((r) => playsOf(r, a) > 0 && playsOf(r, b) > 0);
        if (coRuns.length > 0 && coRuns.length * DEGEN_DEN >= totalWins * DEGEN_NUM) {
          degenerateCombos.push({
            cards: [a, b].slice().sort(),
            winInclusion: { included: coRuns.length, totalWins },
            replay: replaySeed(coRuns[0]),
          });
        }
      }
    }
    degenerateCombos.sort((x, y) => (x.cards.join('+') < y.cards.join('+') ? -1 : 1));

    const difficultyCurve = runs
      .map((r) => ({
        persona: r.personaId,
        skill: r.skill,
        aggression: r.aggression,
        outcome: r.outcome,
        turns: r.turns,
        actions: r.actions,
      }))
      .slice()
      .sort((x, y) => (x.persona < y.persona ? -1 : 1));

    const report = {
      schemaVersion: REPORT_SCHEMA,
      scene: typeof sceneId === 'string' ? sceneId : 'unknown-scene',
      totalRuns,
      winRate: { wins: totalWins, total: totalRuns },
      wins: totalWins,
      losses,
      playing,
      cards,
      degenerateCombos,
      deadItems,
      difficultyCurve,
      readOnlyInspection: {
        trustedEmitter: 'balance-telemetry-aggregation',
        browserStudioMode: 'read-only descriptive balance report',
        disallowedActions: ['trusted writes', 'auto-applied nerf or buff', 'balance guarantee', 'live mutation'],
      },
    };
    report.digest = reportDigest(report);
    return report;
  }

  // Canonical compact report digest. Integer-only and order-stable so the Rust
  // mirror pins the same string.
  function reportDigest(report) {
    const cards = report.cards
      .map((c) => `${c.card}:${c.plays}:${c.runsIncluded}:${c.winsIncluded}:${c.lossesIncluded}`)
      .join(',');
    const degen = report.degenerateCombos
      .map((g) => `{${g.cards.join('+')}@${g.winInclusion.included}/${g.winInclusion.totalWins}#${g.replay.deckSeed}:${g.replay.persona}}`)
      .join(',');
    const dead = report.deadItems
      .map((d) => `{${d.card}#${d.replay.deckSeed}:${d.replay.persona}}`)
      .join(',');
    const curve = report.difficultyCurve
      .map((p) => `${p.persona}:${p.outcome}:${p.turns}:${p.actions}`)
      .join(',');
    return [
      `report|scene=${report.scene}`,
      `runs=${report.totalRuns}`,
      `won=${report.wins}`,
      `lost=${report.losses}`,
      `playing=${report.playing}`,
      `cards=${cards}`,
      `degen=${degen}`,
      `dead=${dead}`,
      `curve=${curve}`,
    ].join('|');
  }

  const api = {
    REPORT_SCHEMA,
    DEGEN_NUM,
    DEGEN_DEN,
    vocabularyOf,
    aggregate,
    reportDigest,
  };

  if (typeof window !== 'undefined') {
    window.OuroforgeBalanceTelemetry = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
