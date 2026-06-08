// Deckbuilder UI v1 (#1826, #1827).
//
// A deterministic, probe-observable in-game UI read model for the existing JS
// runtime. It renders the current deck-roguelike hand plus draft-only pipeline
// slots, shop offers, and run-map navigation without adding a UI framework or a
// trusted-write surface. Interactions update local UI selection/queue/path state
// only; any trusted action remains a proposal for the existing review/apply/
// trust-gradient path.
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

  function normalizeUiId(value, fallback, label) {
    const id = text(value, fallback);
    if (!/^[A-Za-z0-9_-]{1,64}$/.test(id)) fail(`${label} id "${id}" must be alphanumeric/dash/underscore`);
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
    const shop = normalizeShop(rawSpec.shop);
    const runMap = normalizeRunMap(rawSpec.runMap);
    const scoreDisplay = normalizeScoreDisplay(rawSpec.scoreDisplay);
    return {
      schemaVersion: SPEC_SCHEMA,
      id: text(rawSpec.id, 'deckbuilder-ui'),
      title: text(rawSpec.title, 'Deckbuilder UI'),
      pipelineSlots,
      shop,
      runMap,
      scoreDisplay,
      boundary: 'Browser/runtime UI is read-only and draft-only; trusted writes route through the existing review/apply/trust-gradient path.',
    };
  }

  function formatDisplayNumber(value) {
    const finiteValue = Number.isFinite(value) ? Math.trunc(value) : 0;
    const sign = finiteValue < 0 ? '-' : '';
    const digits = String(Math.abs(finiteValue));
    return `${sign}${digits.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}`;
  }

  function normalizeShop(rawShop = {}) {
    if (rawShop === undefined || rawShop === null) rawShop = {};
    if (!isPlainObject(rawShop)) fail('shop must be an object');
    const offers = Array.isArray(rawShop.offers) ? rawShop.offers : [];
    const seen = new Set();
    return {
      id: normalizeUiId(rawShop.id, 'shop-v1', 'shop'),
      title: text(rawShop.title, 'Shop'),
      currency: text(rawShop.currency, 'gold'),
      balance: Number.isFinite(rawShop.balance) ? rawShop.balance : 0,
      refreshSeed: text(rawShop.refreshSeed, 'declared-seed'),
      offers: offers.map((rawOffer, index) => {
        if (!isPlainObject(rawOffer)) fail(`shop offer ${index} must be an object`);
        const id = normalizeUiId(rawOffer.id, `offer-${index + 1}`, 'shop offer');
        if (seen.has(id)) fail(`shop offer id "${id}" must be unique`);
        seen.add(id);
        const price = Number.isFinite(rawOffer.price) ? rawOffer.price : 0;
        const available = rawOffer.available === undefined ? price <= (Number.isFinite(rawShop.balance) ? rawShop.balance : 0) : Boolean(rawOffer.available);
        return {
          id,
          label: text(rawOffer.label, id),
          kind: text(rawOffer.kind || rawOffer.type, 'card'),
          price,
          requirement: text(rawOffer.requirement, price > 0 ? `${price} ${text(rawShop.currency, 'gold')}` : 'free'),
          available,
          unavailableReason: available ? null : text(rawOffer.unavailableReason, 'unavailable by declared shop state'),
          draftOnly: true,
        };
      }),
    };
  }

  function normalizeRunMap(rawMap = {}) {
    if (rawMap === undefined || rawMap === null) rawMap = {};
    if (!isPlainObject(rawMap)) fail('runMap must be an object');
    const nodesInput = Array.isArray(rawMap.nodes) ? rawMap.nodes : [];
    const seen = new Set();
    const nodes = nodesInput.map((rawNode, index) => {
      if (!isPlainObject(rawNode)) fail(`run-map node ${index} must be an object`);
      const id = normalizeUiId(rawNode.id, `node-${index + 1}`, 'run-map node');
      if (seen.has(id)) fail(`run-map node id "${id}" must be unique`);
      seen.add(id);
      return {
        id,
        label: text(rawNode.label, id),
        kind: text(rawNode.kind || rawNode.type, 'encounter'),
        status: text(rawNode.status, 'available'),
        known: rawNode.known === undefined ? true : Boolean(rawNode.known),
        blockedReason: text(rawNode.blockedReason, ''),
      };
    });
    const currentNodeId = text(rawMap.currentNodeId, nodes[0] ? nodes[0].id : null);
    if (currentNodeId && !seen.has(currentNodeId)) fail(`runMap currentNodeId "${currentNodeId}" must reference a declared node`);
    const edges = (Array.isArray(rawMap.edges) ? rawMap.edges : []).map((rawEdge, index) => {
      if (!isPlainObject(rawEdge)) fail(`run-map edge ${index} must be an object`);
      const from = normalizeUiId(rawEdge.from, '', 'run-map edge from');
      const to = normalizeUiId(rawEdge.to, '', 'run-map edge to');
      if (!seen.has(from) || !seen.has(to)) fail(`run-map edge ${index} must reference declared nodes`);
      return { from, to, blocked: Boolean(rawEdge.blocked), reason: text(rawEdge.reason, '') };
    });
    return {
      id: normalizeUiId(rawMap.id, 'run-map-v1', 'run-map'),
      title: text(rawMap.title, 'Run Map'),
      currentNodeId,
      nodes,
      edges,
      knownUpcomingNodeIds: (Array.isArray(rawMap.knownUpcomingNodeIds) ? rawMap.knownUpcomingNodeIds : [])
        .map((nodeId, index) => normalizeUiId(nodeId, `known-${index + 1}`, 'known upcoming node'))
        .filter((nodeId) => seen.has(nodeId)),
    };
  }

  function normalizeScoreDisplay(rawScoreDisplay = {}) {
    if (rawScoreDisplay === undefined || rawScoreDisplay === null) rawScoreDisplay = {};
    if (!isPlainObject(rawScoreDisplay)) fail('scoreDisplay must be an object');
    const events = (Array.isArray(rawScoreDisplay.events) ? rawScoreDisplay.events : []).map((rawEvent, index) => {
      if (!isPlainObject(rawEvent)) fail(`score display event ${index} must be an object`);
      const stepIndex = Number.isInteger(rawEvent.stepIndex) ? rawEvent.stepIndex : index;
      if (stepIndex !== index) fail('score display events must be declared in cascade step order');
      const phase = text(rawEvent.phase, 'unknown');
      const before = Number.isFinite(rawEvent.before) ? rawEvent.before : 0;
      const addScore = Number.isFinite(rawEvent.addScore) ? rawEvent.addScore : 0;
      const multiplyScore = Number.isFinite(rawEvent.multiplyScore) ? rawEvent.multiplyScore : 1;
      const after = Number.isFinite(rawEvent.after) ? rawEvent.after : 0;
      const cumulativeTotal = Number.isFinite(rawEvent.cumulativeTotal) ? rawEvent.cumulativeTotal : after;
      const modifierId = typeof rawEvent.modifierId === 'string' && rawEvent.modifierId ? rawEvent.modifierId : null;
      const cardId = typeof rawEvent.cardId === 'string' && rawEvent.cardId ? rawEvent.cardId : null;
      return {
        eventId: normalizeUiId(rawEvent.eventId, `score-event-${index + 1}`, 'score display event'),
        stepIndex,
        phase,
        cardId,
        modifierId,
        operation: text(rawEvent.operation, phase),
        before,
        addScore,
        multiplyScore,
        after,
        cumulativeTotal,
        displayValue: formatDisplayNumber(after),
        cumulativeDisplayValue: formatDisplayNumber(cumulativeTotal),
        tooltip: text(rawEvent.tooltip, tooltipForScoreEvent({ phase, cardId, modifierId, before, addScore, multiplyScore, after, cumulativeTotal })),
        readOnlyEvidence: rawEvent.readOnlyEvidence !== false,
      };
    });
    const declaredFinal = Number.isFinite(rawScoreDisplay.finalScore)
      ? rawScoreDisplay.finalScore
      : (events.length ? events[events.length - 1].after : 0);
    const authoritativeScore = Number.isFinite(rawScoreDisplay.authoritativeScore)
      ? rawScoreDisplay.authoritativeScore
      : declaredFinal;
    if (declaredFinal !== authoritativeScore) fail('scoreDisplay finalScore must match authoritativeScore');
    return {
      id: normalizeUiId(rawScoreDisplay.id, 'score-display-v1', 'score display'),
      title: text(rawScoreDisplay.title, 'Score Cascade'),
      source: text(rawScoreDisplay.source, 'rust-score-cascade-feedback'),
      sourceSchemaVersion: text(rawScoreDisplay.sourceSchemaVersion, 'ouroforge.score-cascade-feedback.v1'),
      finalScore: declaredFinal,
      authoritativeScore,
      formattedFinalScore: formatDisplayNumber(declaredFinal),
      events,
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-deckbuilder-ui-probe',
        browserStudioMode: 'read-only score cascade display inspection',
        disallowedActions: ['trusted writes', 'score recomputation authority', 'command bridge', 'live mutation', 'automated fun verdict'],
      },
      boundary: 'Score display presents Rust/local scoring resolution and cascade feedback only; it is not browser score authority, not a trusted write, and not a fun/quality verdict.',
    };
  }

  function tooltipForScoreEvent(event) {
    if (event.phase === 'base') {
      return `${event.cardId || 'card'} base score ${formatSigned(event.addScore)} => ${formatDisplayNumber(event.after)}`;
    }
    if (event.phase === 'modifier') {
      return `${event.modifierId || 'modifier'}: (${formatDisplayNumber(event.before)} + ${formatDisplayNumber(event.addScore)}) × ${formatDisplayNumber(event.multiplyScore)} = ${formatDisplayNumber(event.after)}`;
    }
    if (event.phase === 'card-total') {
      return `${event.cardId || 'card'} contributes ${formatDisplayNumber(event.addScore)}; cumulative total ${formatDisplayNumber(event.cumulativeTotal)}`;
    }
    if (event.phase === 'cascade-complete') {
      return `Authoritative Rust/local score ${formatDisplayNumber(event.after)} matched by cascade feedback`;
    }
    return `${event.phase}: ${formatDisplayNumber(event.after)}`;
  }

  function formatSigned(value) {
    const normalized = Number.isFinite(value) ? Math.trunc(value) : 0;
    return normalized >= 0 ? `+${formatDisplayNumber(normalized)}` : formatDisplayNumber(normalized);
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
      shop: {
        id: spec.shop.id,
        title: spec.shop.title,
        currency: spec.shop.currency,
        balance: spec.shop.balance,
        refreshSeed: spec.shop.refreshSeed,
        offers: spec.shop.offers.map((offer) => ({
          kind: 'shop-offer',
          offerId: offer.id,
          label: offer.label,
          offerKind: offer.kind,
          price: offer.price,
          requirement: offer.requirement,
          available: offer.available,
          unavailableReason: offer.unavailableReason,
          selected: interaction.selectedShopOfferId === offer.id,
          draftOnly: true,
        })),
      },
      runMap: {
        id: spec.runMap.id,
        title: spec.runMap.title,
        currentNodeId: spec.runMap.currentNodeId,
        nodes: spec.runMap.nodes.map((node) => ({
          kind: 'run-map-node',
          nodeId: node.id,
          label: node.label,
          nodeKind: node.kind,
          status: node.status,
          known: node.known,
          current: node.id === spec.runMap.currentNodeId,
          planned: interaction.plannedRunMapNodeId === node.id,
          blockedReason: node.blockedReason,
        })),
        edges: clone(spec.runMap.edges),
        knownUpcomingNodeIds: spec.runMap.knownUpcomingNodeIds.slice(),
      },
      scoreDisplay: {
        id: spec.scoreDisplay.id,
        title: spec.scoreDisplay.title,
        source: spec.scoreDisplay.source,
        sourceSchemaVersion: spec.scoreDisplay.sourceSchemaVersion,
        finalScore: spec.scoreDisplay.finalScore,
        formattedFinalScore: spec.scoreDisplay.formattedFinalScore,
        authoritativeScore: spec.scoreDisplay.authoritativeScore,
        cascade: spec.scoreDisplay.events.map((event) => ({
          kind: 'score-cascade-event',
          eventId: event.eventId,
          stepIndex: event.stepIndex,
          phase: event.phase,
          cardId: event.cardId,
          modifierId: event.modifierId,
          operation: event.operation,
          displayValue: event.displayValue,
          cumulativeDisplayValue: event.cumulativeDisplayValue,
          tooltip: event.tooltip,
          readOnlyEvidence: event.readOnlyEvidence,
        })),
        readOnlyInspection: clone(spec.scoreDisplay.readOnlyInspection),
        boundary: spec.scoreDisplay.boundary,
      },
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
      selectedShopOfferId: null,
      plannedRunMapNodeId: null,
      lastAction: { type: 'none', accepted: false },
      proposal: null,
      navigation: {
        currentNodeId: spec.runMap.currentNodeId,
        plannedNodeId: null,
        draftOnly: true,
      },
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

  function selectShopOffer(previousState, offerId) {
    const state = clone(previousState);
    const id = text(offerId, '');
    const offer = state.spec.shop.offers.find((candidate) => candidate.id === id);
    if (!offer) {
      state.interaction.lastAction = { type: 'select-shop-offer', accepted: false, reason: 'invalid-offer', offerId };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    if (!offer.available) {
      state.interaction.selectedShopOfferId = null;
      state.interaction.lastAction = { type: 'select-shop-offer', accepted: false, reason: offer.unavailableReason || 'unavailable-offer', offerId: offer.id };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    state.interaction.selectedShopOfferId = offer.id;
    state.interaction.proposal = {
      action: 'shop-offer-selection',
      offerId: offer.id,
      trustedWrite: false,
      draftOnly: true,
      route: 'existing review/apply/trust-gradient path',
    };
    state.interaction.lastAction = { type: 'select-shop-offer', accepted: true, offerId: offer.id, draftOnly: true };
    state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
    return state;
  }

  function planRunMapNode(previousState, nodeId) {
    const state = clone(previousState);
    const id = text(nodeId, '');
    const node = state.spec.runMap.nodes.find((candidate) => candidate.id === id);
    if (!node) {
      state.interaction.lastAction = { type: 'plan-run-map-node', accepted: false, reason: 'invalid-node', nodeId };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    const current = state.spec.runMap.currentNodeId;
    const edge = state.spec.runMap.edges.find((candidate) => candidate.from === current && candidate.to === id);
    if (!edge || edge.blocked || node.status === 'blocked' || node.known === false) {
      state.interaction.lastAction = {
        type: 'plan-run-map-node',
        accepted: false,
        reason: edge && edge.reason ? edge.reason : (node.blockedReason || 'unavailable-path'),
        nodeId: id,
      };
      state.renderModel = renderModelFor(state.spec, state.handCards, state.pipelineSlots, state.interaction);
      return state;
    }
    state.interaction.plannedRunMapNodeId = id;
    state.interaction.navigation = {
      currentNodeId: current,
      plannedNodeId: id,
      draftOnly: true,
    };
    state.interaction.proposal = {
      action: 'run-map-path-plan',
      from: current,
      to: id,
      trustedWrite: false,
      draftOnly: true,
      route: 'existing review/apply/trust-gradient path',
    };
    state.interaction.lastAction = { type: 'plan-run-map-node', accepted: true, nodeId: id, draftOnly: true };
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
      shop: clone(state.spec.shop),
      runMap: clone(state.spec.runMap),
      scoreDisplay: clone(state.spec.scoreDisplay),
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
    formatDisplayNumber,
    createState,
    syncWithDeck,
    selectCard,
    queueSelected,
    selectShopOffer,
    planRunMapNode,
    worldStateView,
  };

  if (typeof window !== 'undefined') window.OuroforgeDeckbuilderUi = api;
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
})();
