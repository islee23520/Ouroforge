// UI/UX Flow, Onboarding and Accessibility v1 (#1660, contract
// docs/long-form-systems-v1.md).
//
// A deterministic, probe-exposed in-game UI/UX flow for the existing
// game-runtime: declared screens (menu / onboarding / hud / settings),
// deterministic navigation transitions, and declared accessibility options
// (toggles and enums). It is in-game UI in the JS runtime, read-only with
// respect to trusted state — never a Studio trusted-write surface.
//
// The module is pure and deterministic: given the same flow spec and the same
// navigation/accessibility inputs it reproduces the same flow state and the same
// read-only view. Validation fails closed with a clear diagnostic: an undeclared
// screen, a non-deterministic transition (two transitions sharing from+action),
// an unreachable screen, or a missing/invalid accessibility option is rejected.
(() => {
  const SPEC_SCHEMA = 'uiux-flow-v1';
  const STATE_SCHEMA = 'ouroforge.uiux-flow-state.v1';
  const SCREEN_KINDS = ['menu', 'onboarding', 'hud', 'settings'];
  const OPTION_TYPES = ['toggle', 'enum'];
  // Canonical read-only/proposal-only boundary, mirrored from the Rust contract
  // (crates/ouroforge-core/src/uiux_flow.rs). The runtime rejects any other
  // value so a scene cannot load with a misleading UI/UX boundary.
  const BOUNDARY =
    'rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient';

  function fail(message) {
    throw new Error(`uiux flow spec invalid: ${message}`);
  }

  function isPlainObject(value) {
    return value !== null && typeof value === 'object' && !Array.isArray(value);
  }

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function normalizeSpec(spec) {
    if (!isPlainObject(spec)) fail('spec must be an object');
    if (spec.schemaVersion !== SPEC_SCHEMA) {
      fail(`schemaVersion must be ${SPEC_SCHEMA}`);
    }
    if (spec.boundary !== BOUNDARY) {
      fail('boundary must be the canonical read-only/proposal-only contract');
    }

    if (!Array.isArray(spec.screens) || spec.screens.length === 0) {
      fail('screens must be a non-empty array');
    }
    const screens = [];
    const screenIds = new Set();
    for (const screen of spec.screens) {
      if (!isPlainObject(screen)) fail('each screen must be an object');
      const id = typeof screen.id === 'string' ? screen.id.trim() : '';
      if (!id) fail('screen id must be a non-empty string');
      if (screenIds.has(id)) fail(`duplicate screen id ${id}`);
      if (!SCREEN_KINDS.includes(screen.kind)) {
        fail(`screen ${id} has invalid kind ${screen.kind}`);
      }
      screenIds.add(id);
      screens.push({ id, kind: screen.kind });
    }

    const initialScreen = typeof spec.initialScreen === 'string' ? spec.initialScreen : '';
    if (!screenIds.has(initialScreen)) fail('initialScreen must be a declared screen');

    if (!Array.isArray(spec.transitions)) fail('transitions must be an array');
    const transitions = [];
    const transitionMap = {};
    for (const transition of spec.transitions) {
      if (!isPlainObject(transition)) fail('each transition must be an object');
      const from = typeof transition.from === 'string' ? transition.from : '';
      const action = typeof transition.action === 'string' ? transition.action.trim() : '';
      const to = typeof transition.to === 'string' ? transition.to : '';
      if (!screenIds.has(from)) fail(`transition references undeclared from-screen ${from}`);
      if (!screenIds.has(to)) fail(`transition references undeclared to-screen ${to}`);
      if (!action) fail('transition action must be a non-empty string');
      if (!transitionMap[from]) transitionMap[from] = {};
      if (Object.prototype.hasOwnProperty.call(transitionMap[from], action)) {
        fail(`non-deterministic transition: ${from} + ${action}`);
      }
      transitionMap[from][action] = to;
      transitions.push({ from, action, to });
    }

    // Reachability: every declared screen must be reachable from the initial
    // screen, otherwise the flow has a dead UI state.
    const reachable = new Set([initialScreen]);
    const queue = [initialScreen];
    while (queue.length > 0) {
      const current = queue.shift();
      const outgoing = transitionMap[current] || {};
      for (const action of Object.keys(outgoing)) {
        const to = outgoing[action];
        if (!reachable.has(to)) {
          reachable.add(to);
          queue.push(to);
        }
      }
    }
    for (const screen of screens) {
      if (!reachable.has(screen.id)) fail(`screen ${screen.id} is unreachable from initialScreen`);
    }

    if (!Array.isArray(spec.accessibilityOptions) || spec.accessibilityOptions.length === 0) {
      fail('accessibilityOptions must declare at least one option');
    }
    const options = [];
    const optionIds = new Set();
    for (const option of spec.accessibilityOptions) {
      if (!isPlainObject(option)) fail('each accessibility option must be an object');
      const id = typeof option.id === 'string' ? option.id.trim() : '';
      if (!id) fail('accessibility option id must be a non-empty string');
      if (optionIds.has(id)) fail(`duplicate accessibility option ${id}`);
      if (!OPTION_TYPES.includes(option.type)) {
        fail(`accessibility option ${id} has invalid type ${option.type}`);
      }
      if (option.type === 'toggle') {
        if (typeof option.default !== 'boolean') {
          fail(`toggle option ${id} default must be a boolean`);
        }
        options.push({ id, type: 'toggle', default: option.default });
      } else {
        if (!Array.isArray(option.values) || option.values.length === 0) {
          fail(`enum option ${id} must declare a non-empty values array`);
        }
        const values = option.values.map(String);
        if (!values.includes(String(option.default))) {
          fail(`enum option ${id} default must be one of its values`);
        }
        options.push({ id, type: 'enum', default: String(option.default), values });
      }
      optionIds.add(id);
    }

    const accessibility = {};
    for (const option of options) accessibility[option.id] = option.default;

    return {
      schema: STATE_SCHEMA,
      screens,
      transitionMap,
      options,
      currentScreen: initialScreen,
      visited: [initialScreen],
      accessibility,
      lastNavigation: null,
      boundary: BOUNDARY,
    };
  }

  function screenKind(state, id) {
    const screen = state.screens.find((entry) => entry.id === id);
    return screen ? screen.kind : null;
  }

  // Apply one navigation action. A declared (currentScreen, action) transition
  // moves the flow; an undeclared action is a deterministic no-op recorded as a
  // rejected navigation (in-game UI ignores invalid navigation rather than
  // failing the runtime).
  function navigate(state, action) {
    const next = clone(state);
    const act = typeof action === 'string' ? action.trim() : '';
    const outgoing = state.transitionMap[state.currentScreen] || {};
    if (act && Object.prototype.hasOwnProperty.call(outgoing, act)) {
      next.currentScreen = outgoing[act];
      if (!next.visited.includes(next.currentScreen)) next.visited.push(next.currentScreen);
      next.lastNavigation = {
        action: act,
        from: state.currentScreen,
        to: next.currentScreen,
        accepted: true,
      };
    } else {
      next.lastNavigation = {
        action: act,
        from: state.currentScreen,
        to: state.currentScreen,
        accepted: false,
      };
    }
    return next;
  }

  // Set a declared accessibility option. An undeclared option or an
  // out-of-domain value fails closed.
  function setAccessibility(state, optionId, value) {
    const option = state.options.find((entry) => entry.id === optionId);
    if (!option) throw new Error(`uiux flow: unknown accessibility option ${optionId}`);
    const next = clone(state);
    if (option.type === 'toggle') {
      if (typeof value !== 'boolean') {
        throw new Error(`uiux flow: toggle option ${optionId} requires a boolean`);
      }
      next.accessibility[optionId] = value;
    } else {
      const candidate = String(value);
      if (!option.values.includes(candidate)) {
        throw new Error(`uiux flow: enum option ${optionId} value ${candidate} is not allowed`);
      }
      next.accessibility[optionId] = candidate;
    }
    return next;
  }

  function onboardingComplete(state) {
    const visitedOnboarding = state.visited.some((id) => screenKind(state, id) === 'onboarding');
    const currentlyOnboarding = screenKind(state, state.currentScreen) === 'onboarding';
    return visitedOnboarding && !currentlyOnboarding;
  }

  // Read-only world-state view exposed through the runtime probe.
  function worldStateView(state) {
    return {
      schema: STATE_SCHEMA,
      currentScreen: state.currentScreen,
      currentKind: screenKind(state, state.currentScreen),
      visitedScreens: clone(state.visited),
      screenCount: state.screens.length,
      accessibility: clone(state.accessibility),
      accessibilityOptionCount: state.options.length,
      onboardingComplete: onboardingComplete(state),
      lastNavigation: clone(state.lastNavigation),
      boundary: state.boundary,
    };
  }

  const api = {
    SPEC_SCHEMA,
    STATE_SCHEMA,
    normalizeSpec,
    createState: normalizeSpec,
    navigate,
    setAccessibility,
    worldStateView,
  };

  if (typeof window !== 'undefined') {
    window.OuroforgeUiuxFlow = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
