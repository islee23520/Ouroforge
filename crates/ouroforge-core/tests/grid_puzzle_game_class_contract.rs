//! Contract test for the Grid-Puzzle Game Class v1 (#1574).
//!
//! Rust mirror of the runtime test
//! `examples/game-runtime/grid-puzzle.test.cjs`. It re-derives the deterministic
//! grid-puzzle semantics in trusted Rust over the shared fixtures and
//! machine-checks the genre's acceptance properties rather than asserting them:
//! the declared intended solution reaches the win state, a blocked push does not
//! falsely win, and malformed / missing-win specs fail closed.
//!
//! Per the Era F language boundary (docs/grid-puzzle-game-class-v1.md), the
//! trusted solver/detector logic is owned by Rust/local; the JavaScript runtime
//! reproduces the same observable behavior for the browser-local probe.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_scene(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[derive(Clone, Debug)]
struct GridState {
    width: usize,
    height: usize,
    role_by_object: HashMap<String, String>,
    cells: Vec<Vec<Vec<String>>>,
    player: (usize, usize),
    targets: Vec<(usize, usize)>,
    status: String,
    last_result: String,
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

/// Validate-then-build, mirroring the runtime module. Returns the initial state
/// or a clear diagnostic; this is the trusted fail-closed validator.
fn validate_spec(spec: &Value) -> Result<GridState, String> {
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
    let mut role_by_object: HashMap<String, String> = HashMap::new();
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
        role_by_object.insert(name.clone(), role.to_string());
    }

    let legend = spec["legend"]
        .as_object()
        .filter(|l| !l.is_empty())
        .ok_or("legend must be a non-empty object")?;
    let mut legend_by_char: HashMap<char, Vec<String>> = HashMap::new();
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
            if !role_by_object.contains_key(layer) {
                return Err(format!(
                    "legend \"{symbol}\" references undeclared object \"{layer}\""
                ));
            }
            layer_names.push(layer.to_string());
        }
        legend_by_char.insert(symbol_char, layer_names);
    }

    let rows = spec["rows"]
        .as_array()
        .filter(|r| r.len() == height)
        .ok_or_else(|| format!("rows must be an array of exactly {height} strings"))?;
    let mut cells: Vec<Vec<Vec<String>>> = Vec::new();
    let mut player_count = 0;
    let mut player = (0usize, 0usize);
    let mut targets: Vec<(usize, usize)> = Vec::new();
    let mut pushable_count = 0;
    for (y, row) in rows.iter().enumerate() {
        let row = row
            .as_str()
            .filter(|s| s.chars().count() == width)
            .ok_or_else(|| format!("row {y} must be a string of length {width}"))?;
        let mut cell_row = Vec::new();
        for (x, symbol) in row.chars().enumerate() {
            let layers = legend_by_char.get(&symbol).ok_or_else(|| {
                format!("row {y} column {x} uses character \"{symbol}\" absent from the legend")
            })?;
            for layer in layers {
                match role_by_object.get(layer).map(String::as_str) {
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
            .filter_map(|object| state.role_by_object.get(object).cloned())
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
        .position(|object| state.role_by_object.get(object).map(String::as_str) == Some(role))
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

fn grid_puzzle_spec(scene_fixture: &str) -> Value {
    read_scene(scene_fixture)["gridPuzzle"].clone()
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|item| item.as_str().expect("string").to_string())
        .collect()
}

#[test]
fn valid_grid_puzzle_spec_satisfies_the_game_class_contract() {
    let spec = grid_puzzle_spec("examples/game-runtime/grid-puzzle-scene-v1.json");
    let state = validate_spec(&spec).expect("valid grid puzzle spec");
    assert_eq!(state.width, 6);
    assert_eq!(state.height, 5);
    assert_eq!(state.player, (3, 2));
    assert_eq!(state.targets, vec![(1, 1)]);
    assert_eq!(state.status, "playing");
    // The spec carries a declared, deterministic intended solution.
    let solution = string_array(&spec["intendedSolution"]);
    assert!(!solution.is_empty());
}

#[test]
fn declared_intended_solution_reaches_the_win_state() {
    let spec = grid_puzzle_spec("examples/game-runtime/grid-puzzle-scene-v1.json");
    let mut state = validate_spec(&spec).expect("valid grid puzzle spec");
    for direction in string_array(&spec["intendedSolution"]) {
        advance(&mut state, &direction);
    }
    assert_eq!(state.status, "won");
    assert_eq!(state.player, (1, 2));
    // The target cell is now covered by a pushable crate.
    let covered = roles_at(&state, 1, 1).expect("target cell in bounds");
    assert!(covered.iter().any(|r| r == "pushable"));
}

#[test]
fn blocked_push_into_a_wall_does_not_falsely_win() {
    let spec = grid_puzzle_spec("examples/game-runtime/grid-puzzle-scene-v1.json");
    let mut state = validate_spec(&spec).expect("valid grid puzzle spec");
    advance(&mut state, "left");
    advance(&mut state, "left");
    assert_eq!(state.status, "playing");
    assert_eq!(state.last_result, "blocked");
    assert_eq!(state.player, (2, 2));
}

#[test]
fn malformed_grid_fails_closed() {
    let spec = grid_puzzle_spec("examples/game-runtime/grid-puzzle-invalid-malformed-grid.json");
    let error = validate_spec(&spec).expect_err("malformed grid must be rejected");
    assert_eq!(error, "row 2 must be a string of length 6");
}

#[test]
fn missing_win_condition_fails_closed() {
    let spec = grid_puzzle_spec("examples/game-runtime/grid-puzzle-invalid-missing-win.json");
    let error = validate_spec(&spec).expect_err("missing win condition must be rejected");
    assert_eq!(error, "a win condition with a string type is required");
}
