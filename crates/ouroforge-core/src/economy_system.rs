//! Economy and Currency v1 (#1658) — currency balances and transactions as a
//! data system on the existing runtime.
//!
//! This module owns the trusted, deterministic economy state for a profile: the
//! declared currencies, their balances, and the integrity-checked transactions
//! that change them. It is a data system on the existing runtime/save-state
//! surfaces, not a new runtime: it folds recorded earn/spend transactions into a
//! persistent, serializable ledger.
//!
//! Determinism: balances are held in an ordered `BTreeMap`, accrual is checked
//! integer addition, and spending is checked subtraction. Applying the same
//! transactions from the same initial state always reproduces the same balances,
//! and a state round-trips through JSON unchanged.
//!
//! Fail-closed: validation rejects malformed definitions, an undeclared
//! currency, a duplicate transaction id, an earn that overflows, and a spend
//! that would drive a balance negative (the non-negative invariant), each with a
//! structured reason rather than silently clamping. Balances are `u64`, so the
//! non-negative invariant is also type-enforced on restore.
//!
//! Boundary: Rust/local owns this trusted ledger and its transaction rule. The
//! in-game economy UI is read-only JavaScript runtime presentation of the
//! exposed state (`read_model`); the browser never writes back. Generated
//! economy definitions are proposals through the existing
//! review/apply/trust-gradient path, never a direct trusted write. See
//! `docs/long-form-systems-v1.md`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const ECONOMY_SYSTEM_SCHEMA_VERSION: &str = "economy-system-v1";

/// Canonical trust boundary recorded on every definition and state so the
/// read-only/proposal-only contract travels with the persisted data.
pub const ECONOMY_SYSTEM_BOUNDARY: &str =
    "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient";

/// The kind of an economy transaction. `Earn` adds to a balance; `Spend`
/// subtracts from it and fails closed if the balance is insufficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EconomyTransactionKind {
    Earn,
    Spend,
}

impl EconomyTransactionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Earn => "earn",
            Self::Spend => "spend",
        }
    }
}

/// One economy transaction. `tx_id` is the idempotency key — a transaction is
/// applied at most once.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EconomyTransaction {
    pub tx_id: String,
    pub kind: EconomyTransactionKind,
    pub currency: String,
    pub amount: u64,
}

/// The declarative economy definition for one profile: the bounded set of
/// declared currencies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EconomyDefinition {
    pub schema_version: String,
    pub profile_id: String,
    /// Declared, bounded currencies. A transaction may only target a declared
    /// currency.
    pub currencies: Vec<String>,
    pub boundary: String,
}

/// The persistent economy state for one profile: the trusted, serializable
/// ledger of currency balances and the ordered list of already-applied
/// transaction ids (for idempotent replay).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EconomyState {
    pub schema_version: String,
    pub profile_id: String,
    pub balances: BTreeMap<String, u64>,
    pub applied_tx: Vec<String>,
    pub boundary: String,
}

/// Read-only summary for browser/Studio presentation. Derived from the trusted
/// state; the browser renders this and never writes trusted state.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EconomyReadModel {
    pub schema_version: String,
    pub profile_id: String,
    pub balances: BTreeMap<String, u64>,
    pub applied_tx_count: usize,
    pub boundary: String,
}

impl EconomyDefinition {
    /// Parse and validate a definition from JSON, failing closed on malformed or
    /// out-of-contract input.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let definition: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid economy definition json: {err}"))?;
        definition.validate()?;
        Ok(definition)
    }

    /// Validate the definition. Fails closed on wrong schema version, empty
    /// profile/boundary, and empty/duplicate/blank currencies.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ECONOMY_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected economy schema version: {} (expected {})",
                self.schema_version,
                ECONOMY_SYSTEM_SCHEMA_VERSION
            ));
        }
        if self.profile_id.trim().is_empty() {
            return Err(anyhow!("economy profile_id must not be empty"));
        }
        if self.boundary != ECONOMY_SYSTEM_BOUNDARY {
            return Err(anyhow!(
                "economy definition boundary must be the canonical read-only/proposal-only contract"
            ));
        }
        if self.currencies.is_empty() {
            return Err(anyhow!(
                "economy definition must declare at least one currency"
            ));
        }
        let mut seen = BTreeSet::new();
        for currency in &self.currencies {
            if currency.trim().is_empty() {
                return Err(anyhow!("economy currency name must not be empty"));
            }
            if !seen.insert(currency.as_str()) {
                return Err(anyhow!("duplicate economy currency: {currency}"));
            }
        }
        Ok(())
    }

    /// The initial persistent state: every declared currency at zero balance,
    /// no transactions applied.
    pub fn initial_state(&self) -> EconomyState {
        let balances: BTreeMap<String, u64> =
            self.currencies.iter().map(|c| (c.clone(), 0)).collect();
        EconomyState {
            schema_version: ECONOMY_SYSTEM_SCHEMA_VERSION.to_string(),
            profile_id: self.profile_id.clone(),
            balances,
            applied_tx: Vec::new(),
            boundary: ECONOMY_SYSTEM_BOUNDARY.to_string(),
        }
    }

    /// Apply one transaction to a state, returning the next state. Fails closed
    /// on a state that does not match this definition, a blank/duplicate
    /// transaction id, an undeclared currency, an earn overflow, or a spend that
    /// would drive the balance negative.
    pub fn apply_transaction(
        &self,
        state: &EconomyState,
        tx: &EconomyTransaction,
    ) -> Result<EconomyState> {
        self.validate_state(state)?;
        if tx.tx_id.trim().is_empty() {
            return Err(anyhow!("economy transaction tx_id must not be empty"));
        }
        if state.applied_tx.iter().any(|id| id == &tx.tx_id) {
            return Err(anyhow!(
                "economy transaction '{}' already applied",
                tx.tx_id
            ));
        }
        let declared: BTreeSet<&str> = self.currencies.iter().map(String::as_str).collect();
        if !declared.contains(tx.currency.as_str()) {
            return Err(anyhow!(
                "transaction '{}' targets undeclared currency '{}'",
                tx.tx_id,
                tx.currency
            ));
        }
        let mut balances = state.balances.clone();
        let balance = balances.entry(tx.currency.clone()).or_insert(0);
        match tx.kind {
            EconomyTransactionKind::Earn => {
                *balance = balance.checked_add(tx.amount).ok_or_else(|| {
                    anyhow!(
                        "currency '{}' overflowed earning in transaction '{}'",
                        tx.currency,
                        tx.tx_id
                    )
                })?;
            }
            EconomyTransactionKind::Spend => {
                *balance = balance.checked_sub(tx.amount).ok_or_else(|| {
                    anyhow!(
                        "insufficient '{}' to spend {} in transaction '{}' (balance {})",
                        tx.currency,
                        tx.amount,
                        tx.tx_id,
                        balance
                    )
                })?;
            }
        }
        let mut applied_tx = state.applied_tx.clone();
        applied_tx.push(tx.tx_id.clone());
        Ok(EconomyState {
            schema_version: ECONOMY_SYSTEM_SCHEMA_VERSION.to_string(),
            profile_id: self.profile_id.clone(),
            balances,
            applied_tx,
            boundary: ECONOMY_SYSTEM_BOUNDARY.to_string(),
        })
    }

    /// Apply an ordered sequence of transactions, folding each into the next
    /// state. Deterministic: the same inputs always yield the same final state.
    pub fn apply_transactions(
        &self,
        state: &EconomyState,
        txs: &[EconomyTransaction],
    ) -> Result<EconomyState> {
        let mut current = state.clone();
        for tx in txs {
            current = self.apply_transaction(&current, tx)?;
        }
        Ok(current)
    }

    /// Validate that a (possibly restored) state is consistent with this
    /// definition: matching schema, profile, and exactly the declared
    /// currencies. Balances are `u64`, so the non-negative invariant holds by
    /// construction on restore.
    pub fn validate_state(&self, state: &EconomyState) -> Result<()> {
        if state.schema_version != ECONOMY_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected economy state schema version: {} (expected {})",
                state.schema_version,
                ECONOMY_SYSTEM_SCHEMA_VERSION
            ));
        }
        if state.profile_id != self.profile_id {
            return Err(anyhow!(
                "economy state profile '{}' does not match definition profile '{}'",
                state.profile_id,
                self.profile_id
            ));
        }
        if state.boundary != ECONOMY_SYSTEM_BOUNDARY {
            return Err(anyhow!(
                "economy state boundary must be the canonical read-only/proposal-only contract"
            ));
        }
        let declared: BTreeSet<&str> = self.currencies.iter().map(String::as_str).collect();
        let present: BTreeSet<&str> = state.balances.keys().map(String::as_str).collect();
        if declared != present {
            return Err(anyhow!(
                "economy state balances do not match declared currencies"
            ));
        }
        Ok(())
    }
}

impl EconomyState {
    /// Serialize the trusted state to canonical JSON for persistence.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|err| anyhow!("failed to serialize economy state: {err}"))
    }

    /// Parse a persisted state from JSON, failing closed on malformed input or a
    /// wrong schema version. Use [`EconomyDefinition::validate_state`] to
    /// additionally check the state against its definition on restore.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let state: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid economy state json: {err}"))?;
        if state.schema_version != ECONOMY_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected economy state schema version: {} (expected {})",
                state.schema_version,
                ECONOMY_SYSTEM_SCHEMA_VERSION
            ));
        }
        Ok(state)
    }

    /// The current balance of a currency (zero if not present).
    pub fn balance(&self, currency: &str) -> u64 {
        self.balances.get(currency).copied().unwrap_or(0)
    }

    /// Derive the read-only presentation summary for browser/Studio surfaces.
    pub fn read_model(&self) -> EconomyReadModel {
        EconomyReadModel {
            schema_version: self.schema_version.clone(),
            profile_id: self.profile_id.clone(),
            balances: self.balances.clone(),
            applied_tx_count: self.applied_tx.len(),
            boundary: self.boundary.clone(),
        }
    }
}
