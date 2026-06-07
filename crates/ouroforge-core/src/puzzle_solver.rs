//! Deterministic bounded grid-puzzle solver (#1580).
//!
//! This is the trusted Rust/local owner of the solver logic for the Grid-Puzzle
//! Game Class v1 (#1574) under #1 Era F Milestone 28
//! (`docs/puzzle-solver-oversolution-v1.md`). It operates over the existing
//! grid-puzzle state model — the same `ouroforge.grid-puzzle.v1` spec the
//! runtime (`examples/game-runtime/grid-puzzle.js`) and the game-class contract
//! test consume — and decides solvability with a replayable witness.
//!
//! It re-derives the same deterministic, fail-closed validator and fixed-step
//! stepper as the runtime module so the witness it returns replays identically
//! on the browser-local runtime. It adds no new runtime, engine, or game model;
//! it is a solver over existing state.
//!
//! The search is a bounded breadth-first exploration of the reachable state
//! space. It returns the shortest witness when one exists, an explicit
//! `Unsolvable` verdict only after the full reachable space is exhausted, and an
//! explicit `Exhausted` verdict when the search bound is reached first — never a
//! false negative.

use std::collections::{HashSet, VecDeque};

use serde_json::Value;

/// The fixed direction order the solver expands. Keeping this total and constant
/// makes both the verdict and the witness deterministic across runs.
pub const DIRECTIONS: [&str; 4] = ["up", "down", "left", "right"];

/// Default search bound for callers that do not declare their own. Large enough
/// to fully decide small authored levels, small enough to stay bounded.
pub const DEFAULT_MAX_STATES: usize = 200_000;

/// A bound on how many distinct states the solver may expand before it must
/// report exhaustion rather than continue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolveBudget {
    /// Maximum number of distinct reachable states the search may expand.
    pub max_states: usize,
}

impl Default for SolveBudget {
    fn default() -> Self {
        Self {
            max_states: DEFAULT_MAX_STATES,
        }
    }
}

/// The deterministic verdict of a bounded solve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveOutcome {
    /// A win is reachable; `witness` is the shortest replayable action sequence
    /// (deterministic under [`DIRECTIONS`]), `explored` counts expanded states.
    Solvable {
        witness: Vec<String>,
        explored: usize,
    },
    /// The full reachable space was explored within budget and no win exists.
    Unsolvable { explored: usize },
    /// The search bound was reached before a verdict; this is reported
    /// explicitly and must never be treated as `Unsolvable`.
    Exhausted { explored: usize, budget: usize },
}

impl SolveOutcome {
    /// True only for a decided, solvable level.
    pub fn is_solvable(&self) -> bool {
        matches!(self, SolveOutcome::Solvable { .. })
    }

    /// The witness solution when one was found.
    pub fn witness(&self) -> Option<&[String]> {
        match self {
            SolveOutcome::Solvable { witness, .. } => Some(witness),
            _ => None,
        }
    }
}

/// Deterministic grid-puzzle state, mirroring the runtime game-class module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridState {
    width: usize,
    height: usize,
    role_by_object: Vec<(String, String)>,
    cells: Vec<Vec<Vec<String>>>,
    player: (usize, usize),
    targets: Vec<(usize, usize)>,
    status: String,
    last_result: String,
}

impl GridState {
    /// Width of the grid in cells.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the grid in cells.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Current player coordinate.
    pub fn player(&self) -> (usize, usize) {
        self.player
    }

    /// `"playing"`, `"won"`, etc. — the current game status.
    pub fn status(&self) -> &str {
        &self.status
    }

    /// Result of the last [`advance`](GridState::advance) call.
    pub fn last_result(&self) -> &str {
        &self.last_result
    }

    /// True when every target cell is covered by a pushable.
    pub fn is_won(&self) -> bool {
        evaluate_win(self)
    }

    /// Sorted positions of every pushable on the grid. Together with the player
    /// position this is the dynamic part of the state; static geometry never
    /// moves. Callers (e.g. the over-solution detector) use it to key states.
    pub fn pushable_positions(&self) -> Vec<(usize, usize)> {
        let mut pushables = Vec::new();
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell
                    .iter()
                    .any(|object| self.role_of(object) == Some("pushable"))
                {
                    pushables.push((x, y));
                }
            }
        }
        pushables.sort_unstable();
        pushables
    }

    fn role_of(&self, object: &str) -> Option<&str> {
        self.role_by_object
            .iter()
            .find(|(name, _)| name == object)
            .map(|(_, role)| role.as_str())
    }

    /// Apply one deterministic fixed-step transition in `direction`. Mirrors the
    /// runtime module: the player moves into free cells, pushes a single
    /// pushable into a free cell beyond, and is blocked by solids, pushables
    /// that cannot move, and the grid edge.
    pub fn advance(&mut self, direction: &str) {
        advance(self, direction);
    }
}

fn delta(direction: &str) -> Option<(i64, i64)> {
    match direction {
        "up" => Some((0, -1)),
        "down" => Some((0, 1)),
        "left" => Some((-1, 0)),
        "right" => Some((1, 0)),
        _ => None,
    }
}

/// Validate-then-build a grid-puzzle spec, mirroring the runtime module. Returns
/// the initial state or a clear diagnostic; this is the trusted fail-closed
/// validator shared by the solver and the game-class contract.
pub fn validate_spec(spec: &Value) -> Result<GridState, String> {
    let obj = spec.as_object().ok_or("spec must be an object")?;
    if spec["schemaVersion"] != "ouroforge.grid-puzzle.v1" {
        return Err("schemaVersion must be ouroforge.grid-puzzle.v1".into());
    }
    let width = spec["width"]
        .as_u64()
        .filter(|w| *w > 0)
        .ok_or("width must be a positive integer")? as usize;
    let height = spec["height"]
        .as_u64()
        .filter(|h| *h > 0)
        .ok_or("height must be a positive integer")? as usize;

    let objects = spec["objects"]
        .as_object()
        .filter(|o| !o.is_empty())
        .ok_or("objects vocabulary must be a non-empty object")?;
    let mut role_by_object: Vec<(String, String)> = Vec::new();
    let allowed_roles = [
        "background",
        "solid",
        "pushable",
        "player",
        "target",
        "hazard",
    ];
    for (name, definition) in objects {
        let role = definition["role"]
            .as_str()
            .ok_or_else(|| format!("object \"{name}\" must declare a string role"))?;
        if !allowed_roles.contains(&role) {
            return Err(format!("object \"{name}\" has unknown role \"{role}\""));
        }
        role_by_object.push((name.clone(), role.to_string()));
    }

    let legend = spec["legend"]
        .as_object()
        .filter(|l| !l.is_empty())
        .ok_or("legend must be a non-empty object")?;
    let mut legend_by_char: Vec<(char, Vec<String>)> = Vec::new();
    for (symbol, layers) in legend {
        let mut chars = symbol.chars();
        let symbol_char = chars
            .next()
            .ok_or("legend key must be a single character")?;
        if chars.next().is_some() {
            return Err(format!(
                "legend key \"{symbol}\" must be a single character"
            ));
        }
        let layers = layers
            .as_array()
            .filter(|a| !a.is_empty())
            .ok_or_else(|| format!("legend \"{symbol}\" must map to a non-empty layer array"))?;
        let mut layer_names = Vec::new();
        for layer in layers {
            let layer = layer.as_str().ok_or("legend layer must be a string")?;
            if !role_by_object.iter().any(|(name, _)| name == layer) {
                return Err(format!(
                    "legend \"{symbol}\" references undeclared object \"{layer}\""
                ));
            }
            layer_names.push(layer.to_string());
        }
        legend_by_char.push((symbol_char, layer_names));
    }
    let layers_for = |symbol: char| -> Option<&Vec<String>> {
        legend_by_char
            .iter()
            .find(|(c, _)| *c == symbol)
            .map(|(_, layers)| layers)
    };

    let rows = spec["rows"]
        .as_array()
        .filter(|r| r.len() == height)
        .ok_or_else(|| format!("rows must be an array of exactly {height} strings"))?;
    let mut cells: Vec<Vec<Vec<String>>> = Vec::new();
    let mut player_count = 0;
    let mut player = (0usize, 0usize);
    let mut targets: Vec<(usize, usize)> = Vec::new();
    let mut pushable_count = 0;
    let role_lookup = |object: &str| -> Option<&str> {
        role_by_object
            .iter()
            .find(|(name, _)| name == object)
            .map(|(_, role)| role.as_str())
    };
    for (y, row) in rows.iter().enumerate() {
        let row = row
            .as_str()
            .filter(|s| s.chars().count() == width)
            .ok_or_else(|| format!("row {y} must be a string of length {width}"))?;
        let mut cell_row = Vec::new();
        for (x, symbol) in row.chars().enumerate() {
            let layers = layers_for(symbol).ok_or_else(|| {
                format!("row {y} column {x} uses character \"{symbol}\" absent from the legend")
            })?;
            for layer in layers {
                match role_lookup(layer) {
                    Some("player") => {
                        player_count += 1;
                        player = (x, y);
                    }
                    Some("target") => targets.push((x, y)),
                    Some("pushable") => pushable_count += 1,
                    _ => {}
                }
            }
            cell_row.push(layers.clone());
        }
        cells.push(cell_row);
    }

    if player_count != 1 {
        return Err(format!(
            "grid must contain exactly one player cell, found {player_count}"
        ));
    }

    let win = spec["win"]
        .as_object()
        .ok_or("a win condition with a string type is required")?;
    let win_type = win
        .get("type")
        .and_then(Value::as_str)
        .ok_or("a win condition with a string type is required")?;
    if win_type != "all-targets-covered" {
        return Err(format!("unsupported win type \"{win_type}\""));
    }
    if targets.is_empty() {
        return Err("win type all-targets-covered requires at least one target".into());
    }
    if pushable_count < targets.len() {
        return Err(
            "win type all-targets-covered requires at least as many pushables as targets".into(),
        );
    }

    // Touch the spec object so an empty spec is rejected before this point.
    let _ = obj;

    Ok(GridState {
        width,
        height,
        role_by_object,
        cells,
        player,
        targets,
        status: "playing".into(),
        last_result: "none".into(),
    })
}

fn roles_at(state: &GridState, x: i64, y: i64) -> Option<Vec<String>> {
    if x < 0 || y < 0 || x >= state.width as i64 || y >= state.height as i64 {
        return None;
    }
    let cell = &state.cells[y as usize][x as usize];
    Some(
        cell.iter()
            .filter_map(|object| state.role_of(object).map(str::to_string))
            .collect(),
    )
}

fn move_object_by_role(
    state: &mut GridState,
    from: (usize, usize),
    to: (usize, usize),
    role: &str,
) {
    let cell = &mut state.cells[from.1][from.0];
    let index = cell
        .iter()
        .position(|object| {
            state
                .role_by_object
                .iter()
                .find(|(name, _)| name == object)
                .map(|(_, r)| r.as_str())
                == Some(role)
        })
        .expect("object with role present");
    let object = cell.remove(index);
    state.cells[to.1][to.0].push(object);
}

fn evaluate_win(state: &GridState) -> bool {
    state.targets.iter().all(|(tx, ty)| {
        roles_at(state, *tx as i64, *ty as i64)
            .map(|roles| roles.iter().any(|r| r == "pushable"))
            .unwrap_or(false)
    })
}

/// One deterministic fixed-step transition (mirror of the runtime module).
fn advance(state: &mut GridState, direction: &str) {
    if state.status != "playing" {
        state.last_result = "frozen".into();
        return;
    }
    let (dx, dy) = match delta(direction) {
        Some(d) => d,
        None => {
            state.last_result = "none".into();
            return;
        }
    };
    let (px, py) = (state.player.0 as i64, state.player.1 as i64);
    let (tx, ty) = (px + dx, py + dy);
    let target_roles = roles_at(state, tx, ty);

    let result = match &target_roles {
        None => "blocked",
        Some(roles) if roles.iter().any(|r| r == "solid") => "blocked",
        Some(roles) if roles.iter().any(|r| r == "pushable") => {
            let (bx, by) = (tx + dx, ty + dy);
            let beyond = roles_at(state, bx, by);
            match beyond {
                Some(broles) if !broles.iter().any(|r| r == "solid" || r == "pushable") => {
                    move_object_by_role(
                        state,
                        (tx as usize, ty as usize),
                        (bx as usize, by as usize),
                        "pushable",
                    );
                    move_object_by_role(
                        state,
                        (px as usize, py as usize),
                        (tx as usize, ty as usize),
                        "player",
                    );
                    state.player = (tx as usize, ty as usize);
                    "pushed"
                }
                _ => "blocked",
            }
        }
        Some(_) => {
            move_object_by_role(
                state,
                (px as usize, py as usize),
                (tx as usize, ty as usize),
                "player",
            );
            state.player = (tx as usize, ty as usize);
            "moved"
        }
    };

    state.last_result = result.to_string();
    if result != "blocked" && evaluate_win(state) {
        state.status = "won".into();
    }
}

/// The dynamic part of a state that determines reachability: the player
/// position plus the sorted set of pushable positions. Static geometry (solids,
/// targets, background) never changes, so it is excluded from the visited key.
fn search_key(state: &GridState) -> (usize, Vec<(usize, usize)>) {
    (
        state.player.1 * state.width + state.player.0,
        state.pushable_positions(),
    )
}

/// Deterministic bounded breadth-first solve over a validated initial state.
pub fn search(initial: GridState, budget: SolveBudget) -> SolveOutcome {
    if evaluate_win(&initial) {
        return SolveOutcome::Solvable {
            witness: Vec::new(),
            explored: 0,
        };
    }

    let mut visited: HashSet<(usize, Vec<(usize, usize)>)> = HashSet::new();
    visited.insert(search_key(&initial));
    let mut queue: VecDeque<(GridState, Vec<String>)> = VecDeque::new();
    queue.push_back((initial, Vec::new()));
    let mut explored = 0usize;

    while let Some((state, path)) = queue.pop_front() {
        explored += 1;
        if explored > budget.max_states {
            return SolveOutcome::Exhausted {
                explored: explored - 1,
                budget: budget.max_states,
            };
        }
        for direction in DIRECTIONS {
            let mut next = state.clone();
            advance(&mut next, direction);
            // A blocked or no-op move never changes the dynamic state.
            if next.last_result == "blocked" || next.last_result == "none" {
                continue;
            }
            let key = search_key(&next);
            if !visited.insert(key) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push(direction.to_string());
            if next.status == "won" {
                return SolveOutcome::Solvable {
                    witness: next_path,
                    explored,
                };
            }
            queue.push_back((next, next_path));
        }
    }

    SolveOutcome::Unsolvable { explored }
}

/// Validate `spec` and run a bounded deterministic solve, returning the verdict
/// or a fail-closed validation diagnostic.
pub fn solve(spec: &Value, budget: SolveBudget) -> Result<SolveOutcome, String> {
    let initial = validate_spec(spec)?;
    Ok(search(initial, budget))
}

/// Replay a witness over a validated spec and return the resulting state. Used
/// to confirm a returned witness actually reaches the win state.
pub fn replay(spec: &Value, actions: &[String]) -> Result<GridState, String> {
    let mut state = validate_spec(spec)?;
    for action in actions {
        state.advance(action);
    }
    Ok(state)
}
