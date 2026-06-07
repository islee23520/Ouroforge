// Deck-Roguelike Game Class v1 (#1601, contract docs/deck-roguelike-game-class-v1.md).
//
// A deterministic, probe-exposed deck-roguelike game class for the existing
// game-runtime. It models the canonical deck-builder roguelike shape (a single
// combat encounter inside a run): a draw/hand/discard card economy, an energy
// budget per turn, attack/skill cards, run-start and turn-start relics, and a
// scripted enemy with a fixed intent cycle. One probe action maps to one
// deterministic transition (play a card, or end the turn).
//
// This is a game class added to the existing runtime, not a new engine. The
// module is pure and deterministic: given the same spec (including its seed) and
// the same action sequence it reproduces the same trajectory and the same
// digest. All shuffles draw from a mulberry32 stream carried on the deck state,
// reusing the seeded stochastic determinism layer (#1600, identical algorithm)
// so runs are seed-reproducible and replay-stable. No wall-clock, host entropy,
// or Math.random. Validation fails closed with a clear diagnostic; the trusted
// validation that produces these specs is Rust/local.
(() => {
  const SPEC_SCHEMA = 'ouroforge.deck-roguelike.v1';
  const STATE_SCHEMA = 'ouroforge.deck-roguelike-state.v1';
  const CARD_TYPES = ['attack', 'skill'];
  const RELIC_TRIGGERS = ['run-start', 'turn-start'];
  const MAX_DECK = 64;
  const MAX_LOG = 32;
  // mulberry32 increment — identical to the runtime seeded-rng layer (#1600) so
  // deck shuffles share the same replay-stable stream discipline.
  const RNG_INCREMENT = 0x6d2b79f5;

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function fail(message) {
    throw new Error(`deck roguelike spec invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function isPositiveInt(value) {
    return Number.isInteger(value) && value > 0;
  }

  function isNonNegativeInt(value) {
    return Number.isInteger(value) && value >= 0;
  }

  function normalizeSeed(value) {
    const numeric = Number(value);
    if (!Number.isFinite(numeric)) return 0;
    return Math.floor(numeric) >>> 0;
  }

  // One mulberry32 draw. Mutates the rng record (state/drawCount) in place and
  // returns the raw 32-bit unsigned integer, mirroring the runtime stream.
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

  // Seeded Fisher-Yates over a copy. Deterministic for a given rng position.
  function shuffle(cards, rng) {
    const result = cards.slice();
    for (let i = result.length - 1; i > 0; i -= 1) {
      const j = nextBelow(rng, i + 1);
      const temp = result[i];
      result[i] = result[j];
      result[j] = temp;
    }
    return result;
  }

  // Validate-then-build. Returns the immutable initial state, or throws a clear
  // diagnostic. This is runtime input hygiene that fails closed; the trusted
  // validation that authors these specs is Rust/local.
  function normalizeSpec(rawSpec) {
    if (!isPlainObject(rawSpec)) fail('spec must be an object');
    if (rawSpec.schemaVersion !== SPEC_SCHEMA) {
      fail(`schemaVersion must be ${SPEC_SCHEMA}`);
    }
    const seed = normalizeSeed(rawSpec.seed);

    const playerSpec = rawSpec.player;
    if (!isPlainObject(playerSpec)) fail('player must be an object');
    if (!isPositiveInt(playerSpec.maxHp)) fail('player.maxHp must be a positive integer');
    if (!isPositiveInt(playerSpec.energyPerTurn)) fail('player.energyPerTurn must be a positive integer');
    if (!isPositiveInt(playerSpec.handSize)) fail('player.handSize must be a positive integer');

    const cards = rawSpec.cards;
    if (!isPlainObject(cards) || Object.keys(cards).length === 0) {
      fail('cards vocabulary must be a non-empty object');
    }
    const cardById = {};
    for (const [id, definition] of Object.entries(cards)) {
      if (!isPlainObject(definition)) fail(`card "${id}" must be an object`);
      if (!CARD_TYPES.includes(definition.type)) {
        fail(`card "${id}" has unknown type "${definition.type}"`);
      }
      if (!isNonNegativeInt(definition.cost)) fail(`card "${id}" cost must be a non-negative integer`);
      if (definition.type === 'attack' && !isNonNegativeInt(definition.damage)) {
        fail(`attack card "${id}" must declare a non-negative integer damage`);
      }
      if (definition.type === 'skill' && !isNonNegativeInt(definition.block)) {
        fail(`skill card "${id}" must declare a non-negative integer block`);
      }
      cardById[id] = {
        type: definition.type,
        cost: definition.cost,
        damage: isNonNegativeInt(definition.damage) ? definition.damage : 0,
        block: isNonNegativeInt(definition.block) ? definition.block : 0,
      };
    }

    const deck = rawSpec.deck;
    if (!Array.isArray(deck) || deck.length === 0) {
      fail('deck must be a non-empty array of card ids');
    }
    if (deck.length > MAX_DECK) fail(`deck must not exceed ${MAX_DECK} cards`);
    for (const cardId of deck) {
      if (!Object.prototype.hasOwnProperty.call(cardById, cardId)) {
        fail(`deck references undeclared card "${cardId}"`);
      }
    }

    const relicVocabulary = {};
    const relicSpec = rawSpec.relicVocabulary;
    if (relicSpec !== undefined) {
      if (!isPlainObject(relicSpec)) fail('relicVocabulary must be an object');
      for (const [id, definition] of Object.entries(relicSpec)) {
        if (!isPlainObject(definition) || !RELIC_TRIGGERS.includes(definition.trigger)) {
          fail(`relic "${id}" must declare a trigger of ${RELIC_TRIGGERS.join(' or ')}`);
        }
        const effect = isPlainObject(definition.effect) ? definition.effect : {};
        const block = isNonNegativeInt(effect.block) ? effect.block : 0;
        const energy = isNonNegativeInt(effect.energy) ? effect.energy : 0;
        relicVocabulary[id] = { trigger: definition.trigger, effect: { block, energy } };
      }
    }
    let relics = [];
    if (rawSpec.relics !== undefined) {
      if (!Array.isArray(rawSpec.relics)) fail('relics must be an array of relic ids');
      for (const relicId of rawSpec.relics) {
        if (!Object.prototype.hasOwnProperty.call(relicVocabulary, relicId)) {
          fail(`relics references undeclared relic "${relicId}"`);
        }
      }
      relics = rawSpec.relics.slice();
    }

    const enemySpec = rawSpec.enemy;
    if (!isPlainObject(enemySpec)) fail('enemy must be an object');
    if (!isPositiveInt(enemySpec.maxHp)) fail('enemy.maxHp must be a positive integer');
    if (!Array.isArray(enemySpec.intents) || enemySpec.intents.length === 0) {
      fail('enemy.intents must be a non-empty array');
    }
    const intents = [];
    for (const intent of enemySpec.intents) {
      if (!isPlainObject(intent) || intent.type !== 'attack' || !isNonNegativeInt(intent.value)) {
        fail('each enemy intent must be { type: "attack", value: non-negative integer }');
      }
      intents.push({ type: 'attack', value: intent.value });
    }

    const rng = { schemaVersion: 'runtime-seeded-rng-v1', algorithm: 'mulberry32', seed, state: seed, drawCount: 0 };

    const state = {
      schemaVersion: STATE_SCHEMA,
      specSchemaVersion: SPEC_SCHEMA,
      id: typeof rawSpec.id === 'string' ? rawSpec.id : 'deck-roguelike',
      seed,
      rng,
      cards: cardById,
      player: {
        maxHp: playerSpec.maxHp,
        hp: playerSpec.maxHp,
        block: 0,
        energy: 0,
        energyPerTurn: playerSpec.energyPerTurn,
        bonusEnergy: 0,
        handSize: playerSpec.handSize,
      },
      relics,
      relicVocabulary,
      enemy: {
        id: typeof enemySpec.id === 'string' ? enemySpec.id : 'enemy',
        maxHp: enemySpec.maxHp,
        hp: enemySpec.maxHp,
        block: 0,
        intents,
        intentIndex: 0,
        intent: clone(intents[0]),
      },
      drawPile: shuffle(deck, rng),
      hand: [],
      discardPile: [],
      turn: 0,
      status: 'playing',
      lastAction: { type: 'none', accepted: false },
      log: [],
    };

    applyRunStartRelics(state);
    beginPlayerTurn(state);
    return state;
  }

  function logEvent(state, event) {
    state.log.push(event);
    if (state.log.length > MAX_LOG) state.log.shift();
  }

  function applyRunStartRelics(state) {
    for (const relicId of state.relics) {
      const relic = state.relicVocabulary[relicId];
      if (relic && relic.trigger === 'run-start') {
        state.player.block += relic.effect.block;
        state.player.bonusEnergy += relic.effect.energy;
      }
    }
  }

  // Draw from the front of the draw pile. When the draw pile is empty and the
  // discard pile has cards, the discard pile is reshuffled (seeded) into the
  // draw pile first. Returns the drawn card id, or null if no cards remain.
  function drawCard(state) {
    if (state.drawPile.length === 0) {
      if (state.discardPile.length === 0) return null;
      state.drawPile = shuffle(state.discardPile, state.rng);
      state.discardPile = [];
    }
    return state.drawPile.shift();
  }

  function beginPlayerTurn(state) {
    state.turn += 1;
    for (const relicId of state.relics) {
      const relic = state.relicVocabulary[relicId];
      if (relic && relic.trigger === 'turn-start') {
        state.player.block += relic.effect.block;
      }
    }
    state.player.energy = state.player.energyPerTurn + state.player.bonusEnergy;
    while (state.hand.length < state.player.handSize) {
      const card = drawCard(state);
      if (card === null) break;
      state.hand.push(card);
    }
  }

  function dealDamage(amount, target) {
    const absorbed = Math.min(target.block, amount);
    target.block -= absorbed;
    const remaining = amount - absorbed;
    if (remaining > 0) target.hp = Math.max(0, target.hp - remaining);
    return remaining;
  }

  function playCard(state, handIndex) {
    if (!Number.isInteger(handIndex) || handIndex < 0 || handIndex >= state.hand.length) {
      state.lastAction = { type: 'play-card', accepted: false, reason: 'invalid-hand-index', handIndex };
      return;
    }
    const cardId = state.hand[handIndex];
    const card = state.cards[cardId];
    if (card.cost > state.player.energy) {
      state.lastAction = { type: 'play-card', accepted: false, reason: 'insufficient-energy', card: cardId };
      return;
    }
    state.player.energy -= card.cost;
    let dealt = 0;
    if (card.type === 'attack') {
      dealt = dealDamage(card.damage, state.enemy);
    } else {
      state.player.block += card.block;
    }
    state.hand.splice(handIndex, 1);
    state.discardPile.push(cardId);
    state.lastAction = { type: 'play-card', accepted: true, card: cardId, dealt };
    logEvent(state, { type: 'play-card', card: cardId, turn: state.turn });
    if (state.enemy.hp <= 0) {
      state.status = 'won';
      logEvent(state, { type: 'run-won', turn: state.turn });
    }
  }

  function endTurn(state) {
    const intent = state.enemy.intent;
    if (intent && intent.type === 'attack') {
      dealDamage(intent.value, state.player);
    }
    state.enemy.intentIndex = (state.enemy.intentIndex + 1) % state.enemy.intents.length;
    state.enemy.intent = clone(state.enemy.intents[state.enemy.intentIndex]);
    while (state.hand.length > 0) state.discardPile.push(state.hand.shift());
    state.lastAction = { type: 'end-turn', accepted: true };
    logEvent(state, { type: 'end-turn', turn: state.turn });
    if (state.player.hp <= 0) {
      state.status = 'lost';
      logEvent(state, { type: 'run-lost', turn: state.turn });
      return;
    }
    beginPlayerTurn(state);
  }

  // One deterministic transition. Returns the next state; the input state is not
  // mutated. The action is { action: 'play-card', handIndex } or
  // { action: 'end-turn' }; any other input is an observed no-op.
  function advance(previousState, input) {
    const state = clone(previousState);
    if (state.status !== 'playing') {
      state.lastAction = { type: 'frozen', accepted: false };
      return state;
    }
    const action = isPlainObject(input) ? input.action : null;
    if (action === 'play-card') {
      playCard(state, input.handIndex);
    } else if (action === 'end-turn') {
      endTurn(state);
    } else {
      state.lastAction = { type: 'none', accepted: false };
    }
    return state;
  }

  // Probe-facing read model. The deck world-state is observation-only; the
  // browser never writes back through it.
  function worldStateView(state) {
    if (!state) return null;
    return {
      schemaVersion: STATE_SCHEMA,
      id: state.id,
      seed: state.seed,
      status: state.status,
      turn: state.turn,
      rng: clone(state.rng),
      player: clone(state.player),
      enemy: clone(state.enemy),
      relics: state.relics.slice(),
      hand: state.hand.slice(),
      drawPileCount: state.drawPile.length,
      discardPileCount: state.discardPile.length,
      drawPile: state.drawPile.slice(),
      discardPile: state.discardPile.slice(),
      cards: clone(state.cards),
      lastAction: clone(state.lastAction),
      log: clone(state.log),
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-deck-roguelike-world-state',
        browserStudioMode: 'read-only deck-roguelike world-state inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'live mutation'],
      },
    };
  }

  // Canonical subset hashed into the runtime replay digest. Deterministic and
  // sufficient to reconstruct the observable run trajectory; includes the rng
  // position so identical seeds reproduce digest-identical runs.
  function digestState(state) {
    if (!state) return null;
    return {
      schemaVersion: state.schemaVersion,
      seed: state.seed,
      rng: { seed: state.rng.seed, state: state.rng.state, drawCount: state.rng.drawCount },
      turn: state.turn,
      status: state.status,
      player: {
        hp: state.player.hp,
        block: state.player.block,
        energy: state.player.energy,
      },
      enemy: {
        hp: state.enemy.hp,
        block: state.enemy.block,
        intentIndex: state.enemy.intentIndex,
      },
      hand: state.hand,
      drawPile: state.drawPile,
      discardPile: state.discardPile,
      lastAction: state.lastAction,
    };
  }

  const api = {
    SPEC_SCHEMA,
    STATE_SCHEMA,
    normalizeSpec,
    createState: normalizeSpec,
    advance,
    worldStateView,
    digestState,
  };

  if (typeof window !== 'undefined') {
    window.OuroforgeDeckRoguelike = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
