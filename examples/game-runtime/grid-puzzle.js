// Grid-Puzzle Game Class v1 (#1574, contract docs/grid-puzzle-game-class-v1.md).
//
// A deterministic, probe-exposed grid-puzzle game class for the existing
// game-runtime. It models the canonical PuzzleScript flagship shape (a
// block-pushing / Sokoban-style puzzle): a bounded 2-D grid of cells, each cell
// a stack of declared object layers; deterministic role-driven movement rules
// (a player pushes pushable objects into free cells, solids block); a
// deterministic win/lose predicate; and a fixed-step update where one input
// maps to one grid transition.
//
// This is a game class added to the existing runtime, not a new engine. The
// module is pure and deterministic: given the same spec and the same input
// sequence it reproduces the same trajectory and the same digest. Validation
// fails closed with a clear diagnostic. The full PuzzleScript rule-rewrite DSL
// is out of scope here (it is the DSL ingest issue #1575); this game class
// derives movement from declared object roles.
(() => {
  const SPEC_SCHEMA = 'ouroforge.grid-puzzle.v1';
  const STATE_SCHEMA = 'ouroforge.grid-puzzle-state.v1';
  const MAX_DIMENSION = 64;
  // Deterministic single-direction resolution: at most one move per fixed step,
  // resolved in a fixed priority order so a multi-key frame is reproducible.
  const DIRECTION_PRIORITY = ['up', 'down', 'left', 'right'];
  const DIRECTION_DELTA = {
    up: { x: 0, y: -1 },
    down: { x: 0, y: 1 },
    left: { x: -1, y: 0 },
    right: { x: 1, y: 0 },
  };
  const ALLOWED_ROLES = ['background', 'solid', 'pushable', 'player', 'target', 'hazard'];

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function fail(message) {
    throw new Error(`grid puzzle spec invalid: ${message}`);
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function isPositiveInt(value) {
    return Number.isInteger(value) && value > 0;
  }

  // Validate-then-build. Returns immutable initial state, or throws a clear
  // diagnostic. This is runtime input hygiene that fails closed; the trusted
  // DSL validation that produces these specs is Rust/local (#1575).
  function normalizeSpec(rawSpec) {
    if (!isPlainObject(rawSpec)) fail('spec must be an object');
    if (rawSpec.schemaVersion !== SPEC_SCHEMA) {
      fail(`schemaVersion must be ${SPEC_SCHEMA}`);
    }
    const { width, height } = rawSpec;
    if (!isPositiveInt(width) || !isPositiveInt(height)) {
      fail('width and height must be positive integers');
    }
    if (width > MAX_DIMENSION || height > MAX_DIMENSION) {
      fail(`width and height must not exceed ${MAX_DIMENSION}`);
    }

    const objects = rawSpec.objects;
    if (!isPlainObject(objects) || Object.keys(objects).length === 0) {
      fail('objects vocabulary must be a non-empty object');
    }
    const roleByObject = {};
    for (const [name, definition] of Object.entries(objects)) {
      if (!isPlainObject(definition) || typeof definition.role !== 'string') {
        fail(`object "${name}" must declare a string role`);
      }
      if (!ALLOWED_ROLES.includes(definition.role)) {
        fail(`object "${name}" has unknown role "${definition.role}"`);
      }
      roleByObject[name] = definition.role;
    }

    const legend = rawSpec.legend;
    if (!isPlainObject(legend) || Object.keys(legend).length === 0) {
      fail('legend must be a non-empty object');
    }
    const legendByChar = {};
    for (const [symbol, layers] of Object.entries(legend)) {
      if (symbol.length !== 1) fail(`legend key "${symbol}" must be a single character`);
      if (!Array.isArray(layers) || layers.length === 0) {
        fail(`legend "${symbol}" must map to a non-empty layer array`);
      }
      for (const layer of layers) {
        if (!Object.prototype.hasOwnProperty.call(roleByObject, layer)) {
          fail(`legend "${symbol}" references undeclared object "${layer}"`);
        }
      }
      legendByChar[symbol] = layers.slice();
    }

    const rows = rawSpec.rows;
    if (!Array.isArray(rows) || rows.length !== height) {
      fail(`rows must be an array of exactly ${height} strings`);
    }
    const cells = [];
    let playerCount = 0;
    let player = null;
    const targets = [];
    let pushableCount = 0;
    for (let y = 0; y < height; y += 1) {
      const row = rows[y];
      if (typeof row !== 'string' || row.length !== width) {
        fail(`row ${y} must be a string of length ${width}`);
      }
      const cellRow = [];
      for (let x = 0; x < width; x += 1) {
        const symbol = row[x];
        const layers = legendByChar[symbol];
        if (!layers) fail(`row ${y} column ${x} uses character "${symbol}" absent from the legend`);
        const cellLayers = layers.slice();
        for (const layer of cellLayers) {
          const role = roleByObject[layer];
          if (role === 'player') {
            playerCount += 1;
            player = { x, y };
          } else if (role === 'target') {
            targets.push({ x, y });
          } else if (role === 'pushable') {
            pushableCount += 1;
          }
        }
        cellRow.push(cellLayers);
      }
      cells.push(cellRow);
    }

    if (playerCount !== 1) fail(`grid must contain exactly one player cell, found ${playerCount}`);

    const win = rawSpec.win;
    if (!isPlainObject(win) || typeof win.type !== 'string') {
      fail('a win condition with a string type is required');
    }
    if (win.type !== 'all-targets-covered') {
      fail(`unsupported win type "${win.type}"`);
    }
    if (targets.length === 0) fail('win type all-targets-covered requires at least one target');
    if (pushableCount < targets.length) {
      fail('win type all-targets-covered requires at least as many pushables as targets');
    }

    const lose = isPlainObject(rawSpec.lose) ? rawSpec.lose : { type: 'none' };
    if (typeof lose.type !== 'string') fail('lose.type must be a string');
    if (lose.type !== 'none' && lose.type !== 'player-on-hazard') {
      fail(`unsupported lose type "${lose.type}"`);
    }

    let intendedSolution = [];
    if (rawSpec.intendedSolution !== undefined) {
      if (!Array.isArray(rawSpec.intendedSolution)) fail('intendedSolution must be an array of directions');
      for (const move of rawSpec.intendedSolution) {
        if (!Object.prototype.hasOwnProperty.call(DIRECTION_DELTA, move)) {
          fail(`intendedSolution contains unknown direction "${move}"`);
        }
      }
      intendedSolution = rawSpec.intendedSolution.slice();
    }

    return {
      schemaVersion: STATE_SCHEMA,
      specSchemaVersion: SPEC_SCHEMA,
      id: typeof rawSpec.id === 'string' ? rawSpec.id : 'grid-puzzle',
      width,
      height,
      objects: clone(objects),
      roleByObject,
      legend: legendByChar,
      cells,
      player,
      targets,
      win: { type: win.type, satisfied: false },
      lose: { type: lose.type, satisfied: false },
      intendedSolution,
      status: 'playing',
      tick: 0,
      moveCount: 0,
      lastMove: { direction: null, result: 'none' },
    };
  }

  function rolesAt(state, x, y) {
    if (x < 0 || y < 0 || x >= state.width || y >= state.height) return null;
    return state.cells[y][x].map((object) => state.roleByObject[object]);
  }

  function hasRole(state, x, y, role) {
    const roles = rolesAt(state, x, y);
    return Array.isArray(roles) && roles.includes(role);
  }

  function moveObjectByRole(state, fromX, fromY, toX, toY, role) {
    const fromCell = state.cells[fromY][fromX];
    const index = fromCell.findIndex((object) => state.roleByObject[object] === role);
    const [object] = fromCell.splice(index, 1);
    state.cells[toY][toX].push(object);
  }

  function evaluateWin(state) {
    return state.targets.every((target) => hasRole(state, target.x, target.y, 'pushable'));
  }

  function evaluateLose(state) {
    if (state.lose.type !== 'player-on-hazard') return false;
    return hasRole(state, state.player.x, state.player.y, 'hazard');
  }

  function resolveDirection(input) {
    if (!isPlainObject(input)) return null;
    for (const direction of DIRECTION_PRIORITY) {
      if (input[direction] === true) return direction;
    }
    return null;
  }

  // One deterministic fixed-step transition. Returns the next state; the input
  // state is not mutated.
  function advance(previousState, input) {
    const state = clone(previousState);
    state.tick += 1;
    if (state.status !== 'playing') {
      state.lastMove = { direction: null, result: 'frozen' };
      return state;
    }
    const direction = resolveDirection(input);
    if (!direction) {
      state.lastMove = { direction: null, result: 'none' };
      return state;
    }
    const delta = DIRECTION_DELTA[direction];
    const targetX = state.player.x + delta.x;
    const targetY = state.player.y + delta.y;
    const targetRoles = rolesAt(state, targetX, targetY);

    let result = 'blocked';
    if (targetRoles === null || targetRoles.includes('solid')) {
      result = 'blocked';
    } else if (targetRoles.includes('pushable')) {
      const beyondX = targetX + delta.x;
      const beyondY = targetY + delta.y;
      const beyondRoles = rolesAt(state, beyondX, beyondY);
      if (beyondRoles !== null && !beyondRoles.includes('solid') && !beyondRoles.includes('pushable')) {
        moveObjectByRole(state, targetX, targetY, beyondX, beyondY, 'pushable');
        moveObjectByRole(state, state.player.x, state.player.y, targetX, targetY, 'player');
        state.player = { x: targetX, y: targetY };
        state.moveCount += 1;
        result = 'pushed';
      } else {
        result = 'blocked';
      }
    } else {
      moveObjectByRole(state, state.player.x, state.player.y, targetX, targetY, 'player');
      state.player = { x: targetX, y: targetY };
      state.moveCount += 1;
      result = 'moved';
    }

    state.lastMove = { direction, result };
    state.win.satisfied = evaluateWin(state);
    state.lose.satisfied = evaluateLose(state);
    if (state.lose.satisfied) {
      state.status = 'lost';
    } else if (state.win.satisfied) {
      state.status = 'won';
    }
    return state;
  }

  // Probe-facing read model. The grid world-state is observation-only; the
  // browser never writes back through it.
  function worldStateView(state) {
    if (!state) return null;
    return {
      schemaVersion: STATE_SCHEMA,
      id: state.id,
      width: state.width,
      height: state.height,
      status: state.status,
      tick: state.tick,
      moveCount: state.moveCount,
      lastMove: clone(state.lastMove),
      player: clone(state.player),
      targets: clone(state.targets),
      cells: clone(state.cells),
      objects: clone(state.objects),
      legend: clone(state.legend),
      win: clone(state.win),
      lose: clone(state.lose),
      intendedSolution: state.intendedSolution.slice(),
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-grid-puzzle-world-state',
        browserStudioMode: 'read-only grid-puzzle world-state inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'live mutation'],
      },
    };
  }

  // Canonical subset hashed into the runtime replay digest. Deterministic and
  // sufficient to reconstruct the observable grid trajectory.
  function digestState(state) {
    if (!state) return null;
    return {
      schemaVersion: state.schemaVersion,
      width: state.width,
      height: state.height,
      cells: state.cells,
      player: state.player,
      status: state.status,
      tick: state.tick,
      moveCount: state.moveCount,
      lastMove: state.lastMove,
      win: state.win,
      lose: state.lose,
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
    window.OuroforgeGridPuzzle = api;
  }
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = api;
  }
})();
