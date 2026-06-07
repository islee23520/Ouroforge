//! Difficulty metric artifact v1 (#1582).
//!
//! Computes an auditable difficulty-metric artifact for a grid-puzzle level —
//! solution length, branching factor, dead-end density, and
//! mechanic-introduction order — for Era F Milestone 28
//! (`docs/puzzle-solver-oversolution-v1.md`).
//!
//! The metrics are *computed* by trusted Rust/local from solver/detector
//! evidence (#1580/#1581), not asserted by prose. It is a measurement over the
//! existing solver's reachable-state search and the captured intent, reusing the
//! existing `ouroforge.grid-puzzle.v1` validator and stepper; it is not a new
//! analysis engine.
//!
//! Every metric is descriptive measurement only. None of them is a difficulty,
//! quality, or fun guarantee. Stale or missing solver evidence fails closed.

use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::Value;

use crate::puzzle_solver::{self, GridState, SolveBudget, DIRECTIONS};

/// Schema of the solver/detector evidence the difficulty metric is computed
/// from. It records the solver witness (#1580) and the captured intended
/// solution (#1581) so the metric can be re-derived and audited.
pub const EVIDENCE_SCHEMA: &str = "ouroforge.puzzle-solver-evidence.v1";

/// A descriptive, auditable difficulty measurement for a grid-puzzle level.
#[derive(Clone, Debug, PartialEq)]
pub struct DifficultyMetric {
    /// Steps in the solver witness (the recorded solution length).
    pub solution_length: usize,
    /// Average number of legal (non-blocked) moves available per reachable
    /// state.
    pub branching_factor: f64,
    /// Fraction of reachable states from which the win condition is no longer
    /// reachable (dead ends — no progress toward the win condition).
    pub dead_end_density: f64,
    /// The mechanics the level teaches, in the order they are first exercised
    /// along the intended solution.
    pub mechanic_introduction_order: Vec<String>,
    /// Number of distinct reachable states explored while measuring.
    pub reachable_states: usize,
}

/// Map a deterministic step result to the mechanic name it exercises.
fn mechanic_for_result(result: &str) -> Option<&'static str> {
    match result {
        "moved" => Some("move"),
        "pushed" => Some("push"),
        _ => None,
    }
}

fn string_array(value: Option<&Value>) -> Option<Vec<String>> {
    let array = value?.as_array()?;
    let mut out = Vec::with_capacity(array.len());
    for item in array {
        out.push(item.as_str()?.to_string());
    }
    Some(out)
}

/// The dynamic part of a state: player position plus sorted pushable positions.
fn state_key(state: &GridState) -> (usize, Vec<(usize, usize)>) {
    (
        state.player().1 * state.width() + state.player().0,
        state.pushable_positions(),
    )
}

/// Compute the difficulty-metric artifact for `spec` from solver/detector
/// `evidence`, failing closed on stale or missing evidence.
pub fn compute_difficulty(
    spec: &Value,
    evidence: &Value,
    budget: SolveBudget,
) -> Result<DifficultyMetric, String> {
    if evidence.get("schemaVersion").and_then(Value::as_str) != Some(EVIDENCE_SCHEMA) {
        return Err(format!(
            "difficulty metric requires {EVIDENCE_SCHEMA} solver evidence"
        ));
    }
    let witness = string_array(evidence.get("witness"))
        .filter(|w| !w.is_empty())
        .ok_or("difficulty metric requires a non-empty solver witness")?;
    let intended = string_array(evidence.get("intendedSolution"))
        .filter(|s| !s.is_empty())
        .ok_or("difficulty metric requires a non-empty intendedSolution")?;

    // Evidence must not be stale: both the witness and the intended solution
    // must still solve the current level on the trusted stepper.
    if !puzzle_solver::replay(spec, &witness)?.is_won() {
        return Err("solver witness evidence is stale: it does not solve the current level".into());
    }
    if !puzzle_solver::replay(spec, &intended)?.is_won() {
        return Err(
            "intended solution evidence is stale: it does not solve the current level".into(),
        );
    }

    let initial = puzzle_solver::validate_spec(spec)?;

    // Traverse the reachable state space (reusing the solver's stepper). Record
    // the forward edges so dead ends can be found by reverse reachability, and
    // count legal moves for the branching factor.
    let mut index_of: HashMap<(usize, Vec<(usize, usize)>), usize> = HashMap::new();
    let mut edges: Vec<Vec<usize>> = Vec::new();
    let mut is_win: Vec<bool> = Vec::new();
    let mut legal_moves_total = 0usize;

    index_of.insert(state_key(&initial), 0);
    edges.push(Vec::new());
    is_win.push(initial.is_won());
    let mut queue: VecDeque<(usize, GridState)> = VecDeque::new();
    queue.push_back((0, initial));
    let mut explored = 0usize;

    while let Some((index, state)) = queue.pop_front() {
        explored += 1;
        if explored > budget.max_states {
            return Err("level too large to measure difficulty within budget".into());
        }
        for direction in DIRECTIONS {
            let mut next = state.clone();
            next.advance(direction);
            // Only a real move (not blocked, no-op, or post-win freeze) counts.
            if next.last_result() != "moved" && next.last_result() != "pushed" {
                continue;
            }
            legal_moves_total += 1;
            let key = state_key(&next);
            let next_index = match index_of.get(&key) {
                Some(&existing) => existing,
                None => {
                    let new_index = edges.len();
                    index_of.insert(key, new_index);
                    edges.push(Vec::new());
                    is_win.push(next.is_won());
                    queue.push_back((new_index, next.clone()));
                    new_index
                }
            };
            edges[index].push(next_index);
        }
    }

    let reachable_states = edges.len();
    let branching_factor = legal_moves_total as f64 / reachable_states as f64;

    // Dead ends: states from which no winning state is reachable. Found by
    // reverse reachability from the winning states over the forward edges.
    let mut reverse: Vec<Vec<usize>> = vec![Vec::new(); reachable_states];
    for (from, outs) in edges.iter().enumerate() {
        for &to in outs {
            reverse[to].push(from);
        }
    }
    let mut can_reach_win = vec![false; reachable_states];
    let mut win_queue: VecDeque<usize> = VecDeque::new();
    for (index, &won) in is_win.iter().enumerate() {
        if won {
            can_reach_win[index] = true;
            win_queue.push_back(index);
        }
    }
    while let Some(node) = win_queue.pop_front() {
        for &predecessor in &reverse[node] {
            if !can_reach_win[predecessor] {
                can_reach_win[predecessor] = true;
                win_queue.push_back(predecessor);
            }
        }
    }
    let dead_ends = can_reach_win
        .iter()
        .filter(|reachable| !**reachable)
        .count();
    let dead_end_density = dead_ends as f64 / reachable_states as f64;

    // Mechanic-introduction order: the mechanics exercised along the intended
    // path, in first-occurrence order.
    let mut mechanic_introduction_order = Vec::new();
    let mut seen = HashSet::new();
    let mut state = puzzle_solver::validate_spec(spec)?;
    for action in &intended {
        state.advance(action);
        if let Some(mechanic) = mechanic_for_result(state.last_result()) {
            if seen.insert(mechanic) {
                mechanic_introduction_order.push(mechanic.to_string());
            }
        }
    }

    Ok(DifficultyMetric {
        solution_length: witness.len(),
        branching_factor,
        dead_end_density,
        mechanic_introduction_order,
        reachable_states,
    })
}
