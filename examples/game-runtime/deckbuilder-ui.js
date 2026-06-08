// Deckbuilder UI v1 (#1826).
//
// A deterministic, probe-observable in-game UI read model for the existing JS
// runtime. It renders the current deck-roguelike hand plus draft-only pipeline
// slots without adding a UI framework or a trusted-write surface. Interactions
// update local UI selection/queue state only; any trusted action remains a
// proposal for the existing review/apply/trust-gradient path.
(() => {
  const SPEC_SCHEMA = 'ouroforge.deckbuilder-ui.v1';
  const STATE_SCHEMA = 'ouroforge.deckbuilder-ui-state.v1';
  const DEFAULT_SLOT_IDS = ['intent', 'modifier', 'commit'];
  const DISALLOWED_ACTIONS = ['trusted writes', 'command bridge', 'live mutation', 'auto-apply', 'auto-merge'];

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function fail(message) {
    throw new Error(`deckbuilder ui spec invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function text(value, fallback) {
    return typeof value === 'string' && value.trim() ? value.trim() : fallback;
  }

  function normalizeSlotId(value, index) {
    const id = text(value, `slot-${index + 1}`);
    if (!/^[A-Za-z0-9_-]{1,48}$/.test(id)) fail(`pipeline slot id "${id}" must be alphanumeric/dash/underscore`);
    return id;
  }

  function normalizeSpec(rawSpec = {}) {
    if (!isPlainObject(rawSpec)) fail('spec must be an object');
    if (rawSpec.schemaVersion !== SPEC_SCHEMA) fail(`schemaVersion must be ${SPEC_SCHEMA}`);
    const slotInput = rawSpec.pipelineSlots === undefined ? DEFAULT_SLOT_IDS : rawSpec.pipelineSlots;
    if (!Array.isArray(slotInput) || slotInput.length === 0) fail('pipelineSlots must be a non-empty array');
    const seen = new Set();
    const pipelineSlots = slotInput.map((slot, index) => {
      const source = isPlainObject(slot) ? slot : { id: slot };
      const id = normalizeSlotId(source.id, index);
      if (seen.has(id)) fail(`pipeline slot id "${id}" must be unique`);
      seen.add(id);
      return {
        id,
        label: text(source.label, id),
        purpose: text(source.purpose, 'draft-only planning slot'),
      };
    });
    return {
      schemaVersion: SPEC_SCHEMA,
      id: text(rawSpec.id, 'deckbuilder-ui'),
      title: text(rawSpec.title, 'Deckbuilder UI'),
      pipelineSlots,
      boundary: 'Browser/runtime UI is read-only and draft-only; trusted writes route through the existing review/apply/trust-gradient path.',
    };
  }

  function cardEffectText(card) {
    if (!card) return 'unknown card';
    if (card.type === 'attack') return `${card.damage || 0} damage`;
    if (card.type === 'skill') return `${card.block || 0} block`;
    return 'observed card';
  }

  function handCardsFromDeck(deckView) {
    if (!deckView || !Array.isArray(deckView.hand) || !isPlainObject(deckView.cards)) return [];
    return deckView.hand.map((cardId, handIndex) => {
      const card = deckView.cards[cardId] || {};
      const playable = deckView.status === 'playing' && Number(card.cost || 0) <= Number(deckView.player && deckView.player.energy || 0);
      return {
        id: cardId,
        handIndex,
        label: cardId,
        type: card.type || 'unknown',
        cost: Number.isFinite(card.cost) ? card.cost : 0,
        effectText: cardEffectText(card),
        playable,
      };
    });
  }

  function renderModelFor(spec, handCards, pipelineSlots, interaction) {
    return {
      schemaVersion: 'ouroforge.deckbuilder-ui-render.v1',
      title: spec.title,
      hand: handCards.map((card) => ({
        kind: 'card',
        key: `hand-${card.handIndex}-${card.id}`,
        cardId: card.id,
        handIndex: card.handIndex,
        label: card.label,
        badges: [`cost:${card.cost}`, card.type, card.playable ? 'playable' : 'blocked'],
        selected: interaction.selectedHandIndex === card.handIndex,
        text: `${card.label} · ${card.effectText}`,
      })),
      pipeline: pipelineSlots.map((slot) => ({
        kind: 'pipeline-slot',
        slotId: slot.id,
        label: slot.label,
        purpose: slot.purpose,
        queuedCard: slot.queuedCard ? clone(slot.queuedCard) : null,
      })),
      interaction: clone(interaction),
    };
  }

  function createState(rawSpec, deckView) {
    const spec = normalizeSpec(rawSpec);
    const handCards = handCardsFromDeck(deckView);
    const pipelineSlots = spec.pipelineSlots.map((slot) => ({ ...slot, queuedCard: null }));
    const interaction = {
      selectedHandIndex: null,
      selectedCardId: null,
      lastAction: { type: 'none', accepted: false },
      proposal: null,
    };
    return {
      schemaVersion: STATE_SCHEMA,
      spec,
      deckRef: deckView ? { status: deckView.status, turn: deckView.turn, seed: deckView.seed } : null,
      handCards,
      pipelineSlots,
      interaction,
      renderModel: renderModelFor(spec, handCards, pipelineSlots, interaction),
    };
  }

  function syncWithDeck(previousState, deckView) {
    if (!previousState) return null;
    const state = clone(previousState);
    state.deckRef = deckView ? { status: deckView.status, turn: deckView.turn, seed: deckView.seed } : null;
    state.handCards = handCardsFromDeck(deckView);
    const playedSelectedCard = deckView
      && deckView.lastAction
      && deckView.lastAction.type === 'play-card'
      && deckView.lastAction.accepted === true
      && deckView.lastAction.card === state.interaction.selectedCardId;
    const selectedStillPresent = state.handCards.some((card) => card.handIndex === state.interaction.selectedHandIndex && card.id === state.interaction.selectedCardId);
    if (playedSelectedCard || !selectedStillPresent) {
      state.interaction.selectedHandIndex = null;
      state.interaction.selectedCardId = null;
    }
    state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
    return state;
  }

  function selectCard(previousState, handIndex) {
    const state = clone(previousState);
    const index = Number.isInteger(handIndex) ? handIndex : -1;
    const card = state.handCards.find((candidate) => candidate.handIndex === index);
    if (!card) {
      state.interaction.lastAction = { type: 'select-card', accepted: false, reason: 'invalid-hand-index', handIndex };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    state.interaction.selectedHandIndex = card.handIndex;
    state.interaction.selectedCardId = card.id;
    state.interaction.lastAction = { type: 'select-card', accepted: true, handIndex: card.handIndex, cardId: card.id };
    state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
    return state;
  }

  function queueSelected(previousState, slotId) {
    const state = clone(previousState);
    const slot = state.pipelineSlots.find((candidate) => candidate.id === slotId);
    const card = state.handCards.find((candidate) => candidate.handIndex === state.interaction.selectedHandIndex && candidate.id === state.interaction.selectedCardId);
    if (!slot) {
      state.interaction.lastAction = { type: 'queue-selected', accepted: false, reason: 'invalid-slot', slotId };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    if (!card) {
      state.interaction.lastAction = { type: 'queue-selected', accepted: false, reason: 'no-selected-card', slotId };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    slot.queuedCard = {
      cardId: card.id,
      handIndex: card.handIndex,
      playable: card.playable,
      draftOnly: true,
      proposal: {
        action: 'play-card',
        handIndex: card.handIndex,
        cardId: card.id,
        trustedWrite: false,
        route: 'existing review/apply/trust-gradient path',
      },
    };
    state.interaction.proposal = clone(slot.queuedCard.proposal);
    state.interaction.lastAction = { type: 'queue-selected', accepted: true, slotId, cardId: card.id, draftOnly: true };
    state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
    return state;
  }

  function worldStateView(state) {
    if (!state) return null;
    return {
      schemaVersion: STATE_SCHEMA,
      id: state.spec.id,
      deckRef: clone(state.deckRef),
      handCards: clone(state.handCards),
      pipelineSlots: clone(state.pipelineSlots),
      renderModel: clone(state.renderModel),
      interaction: clone(state.interaction),
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-deckbuilder-ui-probe',
        browserStudioMode: 'read-only deckbuilder UI inspection',
        disallowedActions: DISALLOWED_ACTIONS.slice(),
      },
      trustedWriteBoundary: state.spec.boundary,
      generatedStatePolicy: 'Generated runs/artifacts remain untracked unless explicitly fixture-scoped.',
    };
  }

  const api = {
    SPEC_SCHEMA,
    STATE_SCHEMA,
    normalizeSpec,
    createState,
    syncWithDeck,
    selectCard,
    queueSelected,
    worldStateView,
  };

  if (typeof window !== 'undefined') window.OuroforgeDeckbuilderUi = api;
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
})();
