//! Designer intent capture and over-solution detector v1 (#1581).
//!
//! This is the trusted Rust/local owner of the over-solution detector for Era F
//! Milestone 28 (`docs/puzzle-solver-oversolution-v1.md`). It is the moat
//! capability: solvability is table stakes, but a level only has design
//! integrity if it has *exactly* its intended solution.
//!
//! It reuses the Deterministic Grid-Puzzle Solver (#1580) and the existing
//! `ouroforge.grid-puzzle.v1` state model — the same validator, stepper, and
//! reachable-state search. It is a detector over that search, not a new search
//! engine.
//!
//! Two pieces:
//!
//! 1. **Intent capture** — a validated artifact recording the designer's
//!    intended solution path and, optionally, the mechanic the level teaches.
//!    Both are validated against the real stepper and fail closed.
//! 2. **Over-solution detector** — an exhaustive bounded search for *any*
//!    winning solution strictly shorter than the captured intended solution.
//!    Each is returned as a replayable counterexample trace ("watch the
//!    bypass"), never a "trust me". A level with exactly its intended solution
//!    yields no counterexample (no false positive).

use std::collections::HashSet;

use serde_json::Value;

use crate::puzzle_solver::{self, GridState, SolveBudget, DIRECTIONS};

/// Map a human mechanic name to the deterministic step result it produces. The
/// grid-puzzle game class teaches one non-trivial mechanic in v1: pushing.
fn mechanic_result(name: &str) -> Option<&'static str> {
    match name {
        "push" => Some("pushed"),
        _ => None,
    }
}

/// A validated designer-intent artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedIntent {
    /// The intended solution path the designer asserts. Validated to reach the
    /// win state on the real stepper.
    pub intended_solution: Vec<String>,
    /// The mechanic the level teaches, if declared. Validated to be a known
    /// mechanic that the intended path actually exercises.
    pub taught_mechanic: Option<String>,
}

/// One unintended winning solution that bypasses the captured intent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CounterExample {
    /// How the solution bypasses intent. v1 detects `shorter-than-intended`.
    pub kind: String,
    /// A replayable action sequence that reaches the win state.
    pub trace: Vec<String>,
    /// Length of the trace in steps.
    pub length: usize,
    /// Whether this bypass exercises the captured taught mechanic, when one is
    /// declared (`None` when no mechanic was captured).
    pub exercises_mechanic: Option<bool>,
}

/// The deterministic result of detecting over-solutions for a level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverSolutionReport {
    /// Length of the captured intended solution.
    pub intended_length: usize,
    /// Each distinct winning solution shorter than the intended one.
    pub counterexamples: Vec<CounterExample>,
    /// True when the search bound was reached before the shorter-solution space
    /// was fully explored; reported explicitly, never as "no over-solutions".
    pub exhausted: bool,
}

impl OverSolutionReport {
    /// True when at least one over-solution was found.
    pub fn has_oversolution(&self) -> bool {
        !self.counterexamples.is_empty()
    }
}

/// Validate a designer-intent artifact against a grid-puzzle spec, failing
/// closed on any malformed or unsatisfiable intent.
pub fn capture_intent(spec: &Value, intent: &Value) -> Result<CapturedIntent, String> {
    let steps = intent
        .get("intendedSolution")
        .and_then(Value::as_array)
        .filter(|steps| !steps.is_empty())
        .ok_or("intent capture requires a non-empty intendedSolution")?;
    let mut intended_solution = Vec::with_capacity(steps.len());
    for step in steps {
        let action = step
            .as_str()
            .ok_or("intendedSolution steps must be strings")?;
        intended_solution.push(action.to_string());
    }

    // The intended path must actually reach the win state on the real stepper.
    let final_state = puzzle_solver::replay(spec, &intended_solution)?;
    if !final_state.is_won() {
        return Err("captured intended solution does not reach the win state".into());
    }

    let taught_mechanic = match intent.get("taughtMechanic") {
        None | Some(Value::Null) => None,
        Some(value) => {
            let name = value.as_str().ok_or("taughtMechanic must be a string")?;
            let result = mechanic_result(name)
                .ok_or_else(|| format!("unknown taught mechanic \"{name}\""))?;
            if !path_exercises(spec, &intended_solution, result)? {
                return Err(format!(
                    "intended solution does not exercise the taught mechanic \"{name}\""
                ));
            }
            Some(name.to_string())
        }
    };

    Ok(CapturedIntent {
        intended_solution,
        taught_mechanic,
    })
}

/// True when replaying `actions` over `spec` produces `result` on any step.
fn path_exercises(spec: &Value, actions: &[String], result: &str) -> Result<bool, String> {
    let mut state = puzzle_solver::validate_spec(spec)?;
    let mut exercised = false;
    for action in actions {
        state.advance(action);
        if state.last_result() == result {
            exercised = true;
        }
    }
    Ok(exercised)
}

/// The dynamic part of a state: player position plus sorted pushable positions.
fn state_key(state: &GridState) -> (usize, Vec<(usize, usize)>) {
    (
        state.player().1 * state.width() + state.player().0,
        state.pushable_positions(),
    )
}

/// Detect over-solutions: validate intent, then exhaustively (within budget)
/// search for every distinct winning solution strictly shorter than the
/// intended one, returning each as a replayable counterexample trace.
///
/// Each counterexample is a distinct *trace* (action sequence), not merely a
/// distinct final board: two different routes that end on the same winning
/// board are both surfaced. Enumeration is over *simple* paths — a state is
/// never revisited within a single solution — so it terminates and excludes
/// padded/looping solutions while keeping every genuinely distinct route.
pub fn detect_oversolutions(
    spec: &Value,
    intent: &Value,
    budget: SolveBudget,
) -> Result<OverSolutionReport, String> {
    let captured = capture_intent(spec, intent)?;
    let intended_length = captured.intended_solution.len();
    let mechanic_result = captured
        .taught_mechanic
        .as_deref()
        .and_then(mechanic_result);

    let initial = puzzle_solver::validate_spec(spec)?;

    let mut enumeration = Enumeration {
        spec,
        intended_length,
        mechanic_result,
        budget: budget.max_states,
        explored: 0,
        exhausted: false,
        counterexamples: Vec::new(),
    };
    let mut on_path: HashSet<(usize, Vec<(usize, usize)>)> = HashSet::new();
    on_path.insert(state_key(&initial));
    let mut trace: Vec<String> = Vec::new();
    enumeration.visit(&initial, &mut trace, &mut on_path)?;

    let mut counterexamples = enumeration.counterexamples;
    // Deterministic order: shortest first, then lexicographically by trace.
    counterexamples.sort_by(|a, b| a.length.cmp(&b.length).then_with(|| a.trace.cmp(&b.trace)));

    Ok(OverSolutionReport {
        intended_length,
        counterexamples,
        exhausted: enumeration.exhausted,
    })
}

/// Recursive depth-first enumeration of simple winning paths strictly shorter
/// than the intended solution.
struct Enumeration<'a> {
    spec: &'a Value,
    intended_length: usize,
    mechanic_result: Option<&'static str>,
    budget: usize,
    explored: usize,
    exhausted: bool,
    counterexamples: Vec<CounterExample>,
}

impl Enumeration<'_> {
    fn visit(
        &mut self,
        state: &GridState,
        trace: &mut Vec<String>,
        on_path: &mut HashSet<(usize, Vec<(usize, usize)>)>,
    ) -> Result<(), String> {
        if self.exhausted {
            return Ok(());
        }
        self.explored += 1;
        if self.explored > self.budget {
            self.exhausted = true;
            return Ok(());
        }
        for direction in DIRECTIONS {
            let mut next = state.clone();
            next.advance(direction);
            if next.last_result() == "blocked" || next.last_result() == "none" {
                continue;
            }
            let child_len = trace.len() + 1;
            // Only solutions strictly shorter than the intended one are bypasses.
            if child_len >= self.intended_length {
                continue;
            }
            trace.push(direction.to_string());
            if next.is_won() {
                let exercises_mechanic = match self.mechanic_result {
                    Some(result) => Some(path_exercises(self.spec, trace, result)?),
                    None => None,
                };
                self.counterexamples.push(CounterExample {
                    kind: "shorter-than-intended".to_string(),
                    length: trace.len(),
                    trace: trace.clone(),
                    exercises_mechanic,
                });
            } else if child_len + 1 < self.intended_length {
                // A win below this child would still be strictly shorter, so it
                // is worth descending — but never back into a state already on
                // this path (keeps enumeration to simple paths).
                let key = state_key(&next);
                if on_path.insert(key.clone()) {
                    self.visit(&next, trace, on_path)?;
                    on_path.remove(&key);
                }
            }
            trace.pop();
            if self.exhausted {
                return Ok(());
            }
        }
        Ok(())
    }
}
