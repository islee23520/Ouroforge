//! Meta-Progression and Unlocks v1 (#1657) — persistent cross-run progression
//! and unlock state as a data system on the existing runtime.
//!
//! This module owns the trusted, deterministic state describing what a profile
//! has accrued across runs (progression counters) and what it has unlocked
//! (threshold-gated unlock rules). It is a data/scene system on the existing
//! runtime and save-state surfaces, not a new runtime: it consumes run
//! outcomes already produced by the deterministic runtime and folds them into a
//! persistent, serializable state.
//!
//! Determinism: counters and unlocks are held in ordered (`BTreeMap` /
//! `BTreeSet`) collections, accrual is checked integer addition, and unlocks
//! are a pure threshold predicate over the counters. Applying the same run
//! outcomes from the same initial state always reproduces the same state and
//! the same unlocks, and a state round-trips through JSON unchanged.
//!
//! Fail-closed: validation rejects malformed definitions, an unknown counter in
//! a run outcome, a duplicate run id, an integer overflow, and a save state
//! inconsistent with its definition, each with a structured reason rather than
//! silently guessing or clamping.
//!
//! Boundary: Rust/local owns this trusted state and its apply rule. The
//! in-game unlock UI is read-only JavaScript runtime presentation of the
//! exposed state (`read_model`); the browser never writes back. Generated
//! progression definitions are proposals through the existing
//! review/apply/trust-gradient path, never a direct trusted write. See
//! `docs/long-form-systems-v1.md`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const META_PROGRESSION_SCHEMA_VERSION: &str = "meta-progression-v1";

/// Canonical trust boundary recorded on every definition and state so the
/// read-only/proposal-only contract travels with the persisted data.
pub const META_PROGRESSION_BOUNDARY: &str =
    "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient";

/// A threshold-gated unlock: `unlock_id` becomes unlocked once `counter`
/// reaches `threshold` (inclusive). Unlocks are monotonic — once reached they
/// stay unlocked, because progression counters only accrue.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockRule {
    pub unlock_id: String,
    pub counter: String,
    pub threshold: u64,
}

/// The declarative meta-progression definition for one profile: the bounded set
/// of declared progression counters and the unlock rules over them.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetaProgressionDefinition {
    pub schema_version: String,
    pub profile_id: String,
    /// Declared, bounded progression counters. A run outcome may only
    /// contribute to a declared counter.
    pub counters: Vec<String>,
    pub unlocks: Vec<UnlockRule>,
    pub boundary: String,
}

/// One recorded run outcome: the deterministic per-counter accrual a single run
/// contributed. `run_id` is the idempotency key — a run is applied at most once.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunOutcome {
    pub run_id: String,
    #[serde(default)]
    pub deltas: BTreeMap<String, u64>,
}

/// The persistent meta-progression state for one profile. This is the trusted,
/// serializable save state: counters, the unlocked set, and the ordered list of
/// already-applied run ids (for idempotent replay).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetaProgressionState {
    pub schema_version: String,
    pub profile_id: String,
    pub counters: BTreeMap<String, u64>,
    pub unlocked: BTreeSet<String>,
    pub applied_runs: Vec<String>,
    pub boundary: String,
}

/// Read-only summary for browser/Studio presentation. Derived from the trusted
/// state; the browser renders this and never writes trusted state.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetaProgressionReadModel {
    pub schema_version: String,
    pub profile_id: String,
    pub counters: BTreeMap<String, u64>,
    pub unlocked: Vec<String>,
    pub unlock_count: usize,
    pub applied_run_count: usize,
    pub boundary: String,
}

impl MetaProgressionDefinition {
    /// Parse and validate a definition from JSON, failing closed on malformed
    /// or out-of-contract input.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let definition: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid meta-progression definition json: {err}"))?;
        definition.validate()?;
        Ok(definition)
    }

    /// Validate the definition. Fails closed on: wrong schema version, empty
    /// profile/boundary, empty/duplicate/blank counters, and unlock rules with
    /// blank/duplicate ids or that reference an undeclared counter.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != META_PROGRESSION_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected meta-progression schema version: {} (expected {})",
                self.schema_version,
                META_PROGRESSION_SCHEMA_VERSION
            ));
        }
        if self.profile_id.trim().is_empty() {
            return Err(anyhow!("meta-progression profile_id must not be empty"));
        }
        if self.boundary.trim().is_empty() {
            return Err(anyhow!("meta-progression boundary must not be empty"));
        }
        if self.counters.is_empty() {
            return Err(anyhow!(
                "meta-progression definition must declare at least one counter"
            ));
        }
        let mut seen_counters = BTreeSet::new();
        for counter in &self.counters {
            if counter.trim().is_empty() {
                return Err(anyhow!("meta-progression counter name must not be empty"));
            }
            if !seen_counters.insert(counter.as_str()) {
                return Err(anyhow!("duplicate meta-progression counter: {counter}"));
            }
        }
        let mut seen_unlocks = BTreeSet::new();
        for rule in &self.unlocks {
            if rule.unlock_id.trim().is_empty() {
                return Err(anyhow!("meta-progression unlock_id must not be empty"));
            }
            if !seen_unlocks.insert(rule.unlock_id.as_str()) {
                return Err(anyhow!(
                    "duplicate meta-progression unlock id: {}",
                    rule.unlock_id
                ));
            }
            if !seen_counters.contains(rule.counter.as_str()) {
                return Err(anyhow!(
                    "unlock '{}' references undeclared counter '{}'",
                    rule.unlock_id,
                    rule.counter
                ));
            }
        }
        Ok(())
    }

    /// The initial persistent state: every declared counter at zero, unlocks
    /// recomputed (any zero-threshold unlock is already satisfied), no runs
    /// applied.
    pub fn initial_state(&self) -> MetaProgressionState {
        let counters: BTreeMap<String, u64> =
            self.counters.iter().map(|c| (c.clone(), 0)).collect();
        let unlocked = self.compute_unlocked(&counters, &BTreeSet::new());
        MetaProgressionState {
            schema_version: META_PROGRESSION_SCHEMA_VERSION.to_string(),
            profile_id: self.profile_id.clone(),
            counters,
            unlocked,
            applied_runs: Vec::new(),
            boundary: META_PROGRESSION_BOUNDARY.to_string(),
        }
    }

    /// Apply one run outcome to a state, returning the next state. Fails closed
    /// on a state that does not match this definition, a blank/duplicate run
    /// id, a delta to an undeclared counter, or an accrual overflow.
    pub fn apply_run_outcome(
        &self,
        state: &MetaProgressionState,
        outcome: &RunOutcome,
    ) -> Result<MetaProgressionState> {
        self.validate_state(state)?;
        if outcome.run_id.trim().is_empty() {
            return Err(anyhow!(
                "meta-progression run outcome run_id must not be empty"
            ));
        }
        if state.applied_runs.iter().any(|id| id == &outcome.run_id) {
            return Err(anyhow!(
                "meta-progression run '{}' already applied",
                outcome.run_id
            ));
        }
        let declared: BTreeSet<&str> = self.counters.iter().map(String::as_str).collect();
        let mut counters = state.counters.clone();
        for (counter, delta) in &outcome.deltas {
            if !declared.contains(counter.as_str()) {
                return Err(anyhow!(
                    "run '{}' contributes to undeclared counter '{}'",
                    outcome.run_id,
                    counter
                ));
            }
            let current = counters.entry(counter.clone()).or_insert(0);
            *current = current.checked_add(*delta).ok_or_else(|| {
                anyhow!(
                    "meta-progression counter '{}' overflowed applying run '{}'",
                    counter,
                    outcome.run_id
                )
            })?;
        }
        let unlocked = self.compute_unlocked(&counters, &state.unlocked);
        let mut applied_runs = state.applied_runs.clone();
        applied_runs.push(outcome.run_id.clone());
        Ok(MetaProgressionState {
            schema_version: META_PROGRESSION_SCHEMA_VERSION.to_string(),
            profile_id: self.profile_id.clone(),
            counters,
            unlocked,
            applied_runs,
            boundary: META_PROGRESSION_BOUNDARY.to_string(),
        })
    }

    /// Apply an ordered sequence of run outcomes, folding each into the next
    /// state. Deterministic: the same inputs always yield the same final state.
    pub fn apply_run_outcomes(
        &self,
        state: &MetaProgressionState,
        outcomes: &[RunOutcome],
    ) -> Result<MetaProgressionState> {
        let mut current = state.clone();
        for outcome in outcomes {
            current = self.apply_run_outcome(&current, outcome)?;
        }
        Ok(current)
    }

    /// Validate that a (possibly restored) state is consistent with this
    /// definition: matching schema, profile, and exactly the declared counters.
    pub fn validate_state(&self, state: &MetaProgressionState) -> Result<()> {
        if state.schema_version != META_PROGRESSION_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected meta-progression state schema version: {} (expected {})",
                state.schema_version,
                META_PROGRESSION_SCHEMA_VERSION
            ));
        }
        if state.profile_id != self.profile_id {
            return Err(anyhow!(
                "meta-progression state profile '{}' does not match definition profile '{}'",
                state.profile_id,
                self.profile_id
            ));
        }
        let declared: BTreeSet<&str> = self.counters.iter().map(String::as_str).collect();
        let present: BTreeSet<&str> = state.counters.keys().map(String::as_str).collect();
        if declared != present {
            return Err(anyhow!(
                "meta-progression state counters do not match declared counters"
            ));
        }
        // Unlocks are a pure threshold function of the counters, so a consistent
        // state's unlocked set must be exactly the set the counters justify. This
        // rejects a tampered or stale save that claims an unlock whose threshold
        // is not met (which the monotonic apply path would otherwise preserve), an
        // unknown unlock id, or a save missing an unlock its counters require.
        let expected = self.compute_unlocked(&state.counters, &BTreeSet::new());
        if state.unlocked != expected {
            if let Some(unjustified) = state.unlocked.difference(&expected).next() {
                return Err(anyhow!(
                    "meta-progression state unlock '{}' is not justified by its counter threshold",
                    unjustified
                ));
            }
            if let Some(missing) = expected.difference(&state.unlocked).next() {
                return Err(anyhow!(
                    "meta-progression state is missing unlock '{}' required by its counters",
                    missing
                ));
            }
        }
        Ok(())
    }

    /// Pure threshold predicate: the set of unlock ids satisfied by `counters`,
    /// unioned with the already-`unlocked` set so unlocks never regress.
    fn compute_unlocked(
        &self,
        counters: &BTreeMap<String, u64>,
        already: &BTreeSet<String>,
    ) -> BTreeSet<String> {
        let mut unlocked = already.clone();
        for rule in &self.unlocks {
            let value = counters.get(&rule.counter).copied().unwrap_or(0);
            if value >= rule.threshold {
                unlocked.insert(rule.unlock_id.clone());
            }
        }
        unlocked
    }
}

impl MetaProgressionState {
    /// Serialize the trusted state to canonical JSON for persistence.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|err| anyhow!("failed to serialize meta-progression state: {err}"))
    }

    /// Parse a persisted state from JSON, failing closed on malformed input or
    /// a wrong schema version. Use [`MetaProgressionDefinition::validate_state`]
    /// to additionally check the state against its definition on restore.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let state: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid meta-progression state json: {err}"))?;
        if state.schema_version != META_PROGRESSION_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected meta-progression state schema version: {} (expected {})",
                state.schema_version,
                META_PROGRESSION_SCHEMA_VERSION
            ));
        }
        Ok(state)
    }

    /// Whether a specific unlock id is currently unlocked.
    pub fn is_unlocked(&self, unlock_id: &str) -> bool {
        self.unlocked.iter().any(|id| id == unlock_id)
    }

    /// Derive the read-only presentation summary for browser/Studio surfaces.
    pub fn read_model(&self) -> MetaProgressionReadModel {
        MetaProgressionReadModel {
            schema_version: self.schema_version.clone(),
            profile_id: self.profile_id.clone(),
            counters: self.counters.clone(),
            unlocked: self.unlocked.iter().cloned().collect(),
            unlock_count: self.unlocked.len(),
            applied_run_count: self.applied_runs.len(),
            boundary: self.boundary.clone(),
        }
    }
}
