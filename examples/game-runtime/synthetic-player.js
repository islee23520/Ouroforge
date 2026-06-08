// Synthetic Player Persona Agents v1 (#1606, part of Synthetic Player Balance
// v1 #1605 under #1 Era F Milestone 32).
//
// Human-like persona agents that play seeded deck-roguelike runs deterministically.
// A persona is a (skill, style/aggression) pair: it observes the existing
// deck-roguelike probe read-model and chooses ONE existing action per step
// (play a card, or end the turn), driving `OuroforgeDeckRoguelike.advance`.
// These are persona agents driving the existing probe (#1601) over the seeded
// determinism layer (#1600) — NOT a new runtime, engine, solver, or
// win-maximizer. There is no lookahead, search, or per-game tuning: a bounded
// heuristic plus seeded "human" noise, so a low-skill persona misplays and can
// lose. All randomness is integer mulberry32 (identical algorithm to the deck
// shuffle stream) carried on a separate per-run persona stream, so it never
// perturbs the deck's own shuffle determinism: same (deck seed, persona) ⇒ same
// trajectory ⇒ same digest. No wall-clock, host entropy, or Math.random.
//
// The persona stream is decision-only and integer-exact (no floating point) so
// the trusted Rust contract test reproduces digest-identical runs. Persona specs
// fail closed with a clear diagnostic; the trusted validation that authors these
// specs is Rust/local. Generation/observation is proposal-only and the
// browser/Studio surface stays read-only: a persona only replays existing probe
// actions and never performs a trusted write.
(() => {
  const PERSONAS_SCHEMA = 'ouroforge.synthetic-player-personas.v1';
  const RUN_SCHEMA = 'ouroforge.synthetic-player-run.v1';
  // mulberry32 increment — identical to the deck shuffle / runtime seeded-rng
  // layer (#1600) so the persona decision stream shares the same replay-stable
  // discipline. The persona stream is independent of the deck stream.
  const RNG_INCREMENT = 0x6d2b79f5;
  // Narrow "fumble" band just past the skill threshold: a less-skilled persona
  // sometimes ends its turn with energy unspent (a human misplay), bounded so it
  // never loops forever and never exceeds the run budget.
  const FUMBLE_BAND = 10;
  // Default bounded run budget when a personas spec omits one.
  const DEFAULT_BUDGET = { maxTurns: 64, maxActions: 512 };
  const MAX_PERSONAS = 64;
  const PARAM_MAX = 100;

  function fail(message) {
    throw new Error(`synthetic player spec invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function isParamInt(value) {
    return Number.isInteger(value) && value >= 0 && value <= PARAM_MAX;
  }

  function isNonNegativeInt(value) {
    return Number.isInteger(value) && value >= 0;
  }

  function normalizeSeed(value) {
    const numeric = Number(value);
    if (!Number.isFinite(numeric)) return 0;
    return Math.floor(numeric) >>> 0;
  }

  // One mulberry32 draw. Mutates the rng record in place and returns the raw
  // 32-bit unsigned integer, mirroring the deck shuffle stream exactly.
  function nextRaw(rng) {
    rng.state = (rng.state + RNG_INCREMENT) >>> 0;
    let t = rng.state;
    t = Math.imul(t ^ (t >>> 15), 1 | t);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    const raw = (t ^ (t >>> 14)) >>> 0;
    rng.drawCount += 1;
    return raw;
  }

  // Deterministic bounded integer in [0, bound). Seeded; never host entropy.
  function nextBelow(rng, bound) {
    if (bound <= 1) return 0;
    return nextRaw(rng) % bound;
  }

  // Validate-then-build a single persona. Fails closed with a clear diagnostic.
  function normalizePersona(raw) {
    if (!isPlainObject(raw)) fail('each persona must be an object');
    if (typeof raw.id !== 'string' || raw.id.length === 0) {
      fail('persona id must be a non-empty string');
    }
    if (!isParamInt(raw.skill)) fail(`persona "${raw.id}" skill must be an integer in [0, ${PARAM_MAX}]`);
    if (!isParamInt(raw.aggression)) {
      fail(`persona "${raw.id}" aggression must be an integer in [0, ${PARAM_MAX}]`);
    }
    if (raw.seed !== undefined && !isNonNegativeInt(raw.seed)) {
      fail(`persona "${raw.id}" seed must be a non-negative integer`);
    }
    return {
      schemaVersion: 'ouroforge.synthetic-player-persona.v1',
      id: raw.id,
      skill: raw.skill,
      aggression: raw.aggression,
      seed: normalizeSeed(raw.seed === undefined ? 0 : raw.seed),
    };
  }

  // Validate-then-build a personas spec (the roster plus a bounded budget).
  function normalizePersonas(rawSpec) {
    if (!isPlainObject(rawSpec)) fail('personas spec must be an object');
    if (rawSpec.schemaVersion !== PERSONAS_SCHEMA) {
      fail(`schemaVersion must be ${PERSONAS_SCHEMA}`);
    }
    const list = rawSpec.personas;
    if (!Array.isArray(list) || list.length === 0) {
      fail('personas must be a non-empty array');
    }
    if (list.length > MAX_PERSONAS) fail(`personas must not exceed ${MAX_PERSONAS}`);
    const ids = new Set();
    const personas = [];
    for (const raw of list) {
      const persona = normalizePersona(raw);
      if (ids.has(persona.id)) fail(`duplicate persona id "${persona.id}"`);
      ids.add(persona.id);
      personas.push(persona);
    }
    const budget = normalizeBudget(rawSpec.budget);
    return { schemaVersion: PERSONAS_SCHEMA, personas, budget };
  }

  function normalizeBudget(raw) {
    if (raw === undefined || raw === null) return { ...DEFAULT_BUDGET };
    if (!isPlainObject(raw)) fail('budget must be an object');
    const maxTurns = raw.maxTurns === undefined ? DEFAULT_BUDGET.maxTurns : raw.maxTurns;
    const maxActions = raw.maxActions === undefined ? DEFAULT_BUDGET.maxActions : raw.maxActions;
    if (!Number.isInteger(maxTurns) || maxTurns <= 0) fail('budget.maxTurns must be a positive integer');
    if (!Number.isInteger(maxActions) || maxActions <= 0) {
      fail('budget.maxActions must be a positive integer');
    }
    return { maxTurns, maxActions };
  }

  // Integer card value under a persona's style. Aggression nudges attacks up and
  // block down; block gains value under threat. Integer-only so JS and the Rust
  // mirror score identically. Not an optimal evaluator — a coherent human style.
  function scoreCard(card, aggression, threat) {
    if (card.type === 'attack') {
      return card.damage * (100 + aggression);
    }
    return card.block * (100 + (PARAM_MAX - aggression)) + (threat > 0 ? threat * 5 : 0);
  }

  // Choose one existing probe action for the current world-state view. Mutates
  // the persona decision rng. Returns { action: 'play-card', handIndex } or
  // { action: 'end-turn' }. Human-like: a skilled persona plays its best
  // style-weighted affordable card; an unskilled persona sometimes fumbles
  // (ends early) or picks a random affordable card. Always a legal, in-budget
  // action — never a search or a win-maximizing solve.
  function chooseAction(persona, rng, view) {
    if (!view || view.status !== 'playing') return { action: 'end-turn', reason: 'frozen' };
    const energy = view.player.energy;
    const affordable = [];
    for (let i = 0; i < view.hand.length; i += 1) {
      if (view.cards[view.hand[i]].cost <= energy) affordable.push(i);
    }
    if (affordable.length === 0) return { action: 'end-turn', reason: 'no-affordable-card' };
    const intentValue = view.enemy.intent && Number.isInteger(view.enemy.intent.value)
      ? view.enemy.intent.value
      : 0;
    const threat = Math.max(0, intentValue - view.player.block);
    const roll = nextBelow(rng, 100);
    if (roll >= persona.skill) {
      const localOffset = roll - persona.skill;
      if (localOffset < FUMBLE_BAND) return { action: 'end-turn', reason: 'fumble' };
      const pick = nextBelow(rng, affordable.length);
      return { action: 'play-card', handIndex: affordable[pick], reason: 'misplay' };
    }
    let bestIndex = affordable[0];
    let bestScore = scoreCard(view.cards[view.hand[bestIndex]], persona.aggression, threat);
    for (let k = 1; k < affordable.length; k += 1) {
      const i = affordable[k];
      const candidate = scoreCard(view.cards[view.hand[i]], persona.aggression, threat);
      if (candidate > bestScore) {
        bestScore = candidate;
        bestIndex = i;
      }
    }
    return { action: 'play-card', handIndex: bestIndex, reason: 'skilled' };
  }

  // Canonical compact digest of a finished run's deck-state view. Identical
  // format to the deck contract digest so the Rust mirror pins the same string.
  function deckDigest(view) {
    return [
      `rng=${view.rng.seed}:${view.rng.state}:${view.rng.drawCount}`,
      `turn=${view.turn}`,
      `status=${view.status}`,
      `php=${view.player.hp}`,
      `pbl=${view.player.block}`,
      `ehp=${view.enemy.hp}`,
      `hand=${view.hand.join(',')}`,
      `draw=${view.drawPile.join(',')}`,
      `discard=${view.discardPile.join(',')}`,
    ].join('|');
  }

  // Run one persona against one deck spec under a bounded budget, driving the
  // existing deck-roguelike probe. Deterministic: the persona stream is seeded
  // from (deck seed XOR persona seed). Returns an observation-only run record.
  function playRun(deck, deckSpec, persona, budget) {
    if (!deck || typeof deck.createState !== 'function') {
      fail('playRun requires the OuroforgeDeckRoguelike module');
    }
    const normalizedPersona = persona.schemaVersion === 'ouroforge.synthetic-player-persona.v1'
      ? persona
      : normalizePersona(persona);
    const bounds = normalizeBudget(budget);
    let state = deck.createState(deckSpec);
    const rng = { state: (state.seed ^ normalizedPersona.seed) >>> 0, drawCount: 0, seed: normalizedPersona.seed };
    let actions = 0;
    let budgetExhausted = false;
    while (state.status === 'playing') {
      if (actions >= bounds.maxActions) {
        budgetExhausted = true;
        break;
      }
      const view = deck.worldStateView(state);
      const decision = chooseAction(normalizedPersona, rng, view);
      if (decision.action === 'play-card') {
        state = deck.advance(state, { action: 'play-card', handIndex: decision.handIndex });
      } else {
        // Ending a turn begins the next one; stop before starting a turn beyond
        // the budget so a run record never advances past maxTurns.
        if (state.turn >= bounds.maxTurns) {
          budgetExhausted = true;
          break;
        }
        state = deck.advance(state, { action: 'end-turn' });
      }
      actions += 1;
    }
    const finalView = deck.worldStateView(state);
    const digest = `persona=${normalizedPersona.id}|skill=${normalizedPersona.skill}`
      + `|aggro=${normalizedPersona.aggression}|outcome=${state.status}|turn=${state.turn}`
      + `|actions=${actions}|budget=${budgetExhausted ? 1 : 0}|${deckDigest(finalView)}`;
    return {
      schemaVersion: RUN_SCHEMA,
      personaId: normalizedPersona.id,
      skill: normalizedPersona.skill,
      aggression: normalizedPersona.aggression,
      seed: normalizedPersona.seed,
      deckSeed: state.seed,
      outcome: state.status,
      turns: state.turn,
      actions,
      budgetExhausted,
      digest,
      readOnlyInspection: {
        trustedEmitter: 'synthetic-player-persona-run',
        browserStudioMode: 'read-only seeded persona run observation',
        disallowedActions: ['trusted writes', 'auto-apply', 'win-maximizing solve', 'live mutation'],
      },
    };
  }

  // Run every persona in a normalized roster against one deck spec. Returns the
  // run records in roster order; pure and deterministic.
  function playRoster(deck, deckSpec, personasSpec) {
    const roster = normalizePersonas(personasSpec);
    return roster.personas.map((persona) => playRun(deck, deckSpec, persona, roster.budget));
  }

  const api = {
    PERSONAS_SCHEMA,
    RUN_SCHEMA,
    DEFAULT_BUDGET,
    FUMBLE_BAND,
    normalizePersona,
    normalizePersonas,
    scoreCard,
    chooseAction,
    deckDigest,
    playRun,
    playRoster,
  };

  if (typeof window !== 'undefined') {
    window.OuroforgeSyntheticPlayer = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
