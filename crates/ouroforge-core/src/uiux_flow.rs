//! UI/UX Flow, Onboarding and Accessibility v1 (#1660) — the trusted contract
//! for the in-game UI/UX flow that the JS runtime implements and exposes through
//! the read-only probe.
//!
//! This module owns the trusted validation of a UI/UX flow contract: declared
//! screens (menu / onboarding / hud / settings), deterministic navigation
//! transitions, and declared accessibility options (toggles and enums). It adds
//! no runtime; the in-game UI itself is the deterministic JavaScript runtime
//! (`examples/game-runtime/uiux-flow.js`), read-only with respect to trusted
//! state. This Rust contract is the trusted, machine-checkable acceptance shape
//! the runtime flow must satisfy and the mirror of the runtime contract test.
//!
//! Fail-closed: validation rejects an undeclared screen, an undeclared
//! `initialScreen`, a non-deterministic transition (two transitions sharing
//! from+action), an unreachable screen, a missing accessibility set, an invalid
//! option type/default, and a non-canonical boundary — each with a structured
//! reason. It makes no UX-quality or taste judgement; it only checks that the
//! declared flow is deterministic, fully reachable, and carries the declared
//! accessibility affordances.
//!
//! Boundary: Rust/local owns this trusted contract. The in-game UI is read-only
//! JavaScript runtime presentation; the browser never writes trusted state.
//! See `docs/long-form-systems-v1.md`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const UIUX_FLOW_SCHEMA_VERSION: &str = "uiux-flow-v1";

/// Canonical trust boundary recorded on every flow contract.
pub const UIUX_FLOW_BOUNDARY: &str =
    "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient";

/// The declared kinds of UI/UX screen.
pub const UIUX_SCREEN_KINDS: &[&str] = &["menu", "onboarding", "hud", "settings"];

/// The declared kinds of accessibility option.
pub const UIUX_OPTION_TYPES: &[&str] = &["toggle", "enum"];

/// One declared screen.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UiuxScreen {
    pub id: String,
    pub kind: String,
}

/// One declared navigation transition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UiuxTransition {
    pub from: String,
    pub action: String,
    pub to: String,
}

/// One declared accessibility option. `default` is a boolean for a `toggle` and
/// a string for an `enum`; `values` is present only for an `enum`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UiuxAccessibilityOption {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub default: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The declarative UI/UX flow contract.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UiuxFlowContract {
    pub schema_version: String,
    pub boundary: String,
    pub screens: Vec<UiuxScreen>,
    pub initial_screen: String,
    pub transitions: Vec<UiuxTransition>,
    pub accessibility_options: Vec<UiuxAccessibilityOption>,
}

/// Read-only summary of a validated flow contract for browser/Studio surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UiuxFlowReadModel {
    pub schema_version: String,
    pub initial_screen: String,
    pub screen_count: usize,
    pub transition_count: usize,
    pub accessibility_option_count: usize,
    pub screen_kinds: Vec<String>,
    pub boundary: String,
}

impl UiuxFlowContract {
    /// Parse and validate a flow contract from JSON, failing closed on malformed
    /// or out-of-contract input.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid uiux flow contract json: {err}"))?;
        contract.validate()?;
        Ok(contract)
    }

    /// Validate the flow contract, failing closed on any contract violation.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != UIUX_FLOW_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected uiux flow schema version: {} (expected {})",
                self.schema_version,
                UIUX_FLOW_SCHEMA_VERSION
            ));
        }
        if self.boundary != UIUX_FLOW_BOUNDARY {
            return Err(anyhow!(
                "uiux flow boundary must be the canonical read-only/proposal-only contract"
            ));
        }

        if self.screens.is_empty() {
            return Err(anyhow!("uiux flow must declare at least one screen"));
        }
        let mut screen_ids: BTreeSet<&str> = BTreeSet::new();
        for screen in &self.screens {
            if screen.id.trim().is_empty() {
                return Err(anyhow!("uiux screen id must not be empty"));
            }
            if !UIUX_SCREEN_KINDS.contains(&screen.kind.as_str()) {
                return Err(anyhow!(
                    "uiux screen '{}' has invalid kind '{}'",
                    screen.id,
                    screen.kind
                ));
            }
            if !screen_ids.insert(screen.id.as_str()) {
                return Err(anyhow!("duplicate uiux screen id '{}'", screen.id));
            }
        }

        if !screen_ids.contains(self.initial_screen.as_str()) {
            return Err(anyhow!(
                "uiux initialScreen '{}' is not a declared screen",
                self.initial_screen
            ));
        }

        // Deterministic transitions: at most one transition per (from, action).
        let mut transition_map: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
        for transition in &self.transitions {
            if !screen_ids.contains(transition.from.as_str()) {
                return Err(anyhow!(
                    "uiux transition references undeclared from-screen '{}'",
                    transition.from
                ));
            }
            if !screen_ids.contains(transition.to.as_str()) {
                return Err(anyhow!(
                    "uiux transition references undeclared to-screen '{}'",
                    transition.to
                ));
            }
            if transition.action.trim().is_empty() {
                return Err(anyhow!("uiux transition action must not be empty"));
            }
            let outgoing = transition_map.entry(transition.from.as_str()).or_default();
            if outgoing
                .insert(transition.action.as_str(), transition.to.as_str())
                .is_some()
            {
                return Err(anyhow!(
                    "non-deterministic uiux transition: '{}' + '{}'",
                    transition.from,
                    transition.action
                ));
            }
        }

        // Reachability: every screen reachable from the initial screen.
        let mut reachable: BTreeSet<&str> = BTreeSet::new();
        reachable.insert(self.initial_screen.as_str());
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(self.initial_screen.as_str());
        while let Some(current) = queue.pop_front() {
            if let Some(outgoing) = transition_map.get(current) {
                for to in outgoing.values() {
                    if reachable.insert(to) {
                        queue.push_back(to);
                    }
                }
            }
        }
        for screen in &self.screens {
            if !reachable.contains(screen.id.as_str()) {
                return Err(anyhow!(
                    "uiux screen '{}' is unreachable from initialScreen",
                    screen.id
                ));
            }
        }

        // Accessibility affordances must be present and well-formed.
        if self.accessibility_options.is_empty() {
            return Err(anyhow!(
                "uiux flow must declare at least one accessibility option"
            ));
        }
        let mut option_ids: BTreeSet<&str> = BTreeSet::new();
        for option in &self.accessibility_options {
            if option.id.trim().is_empty() {
                return Err(anyhow!("uiux accessibility option id must not be empty"));
            }
            if !option_ids.insert(option.id.as_str()) {
                return Err(anyhow!(
                    "duplicate uiux accessibility option '{}'",
                    option.id
                ));
            }
            if !UIUX_OPTION_TYPES.contains(&option.kind.as_str()) {
                return Err(anyhow!(
                    "uiux accessibility option '{}' has invalid type '{}'",
                    option.id,
                    option.kind
                ));
            }
            match option.kind.as_str() {
                "toggle" => {
                    if !option.default.is_boolean() {
                        return Err(anyhow!(
                            "uiux toggle option '{}' default must be a boolean",
                            option.id
                        ));
                    }
                    if option.values.is_some() {
                        return Err(anyhow!(
                            "uiux toggle option '{}' must not declare enum values",
                            option.id
                        ));
                    }
                }
                "enum" => {
                    let values = option.values.as_ref().ok_or_else(|| {
                        anyhow!("uiux enum option '{}' must declare values", option.id)
                    })?;
                    if values.is_empty() {
                        return Err(anyhow!(
                            "uiux enum option '{}' must declare a non-empty values array",
                            option.id
                        ));
                    }
                    let default = option.default.as_str().ok_or_else(|| {
                        anyhow!("uiux enum option '{}' default must be a string", option.id)
                    })?;
                    if !values.iter().any(|v| v == default) {
                        return Err(anyhow!(
                            "uiux enum option '{}' default must be one of its values",
                            option.id
                        ));
                    }
                }
                _ => unreachable!("option kind already validated"),
            }
        }

        Ok(())
    }

    /// Derive the read-only presentation summary for browser/Studio surfaces.
    pub fn read_model(&self) -> UiuxFlowReadModel {
        UiuxFlowReadModel {
            schema_version: self.schema_version.clone(),
            initial_screen: self.initial_screen.clone(),
            screen_count: self.screens.len(),
            transition_count: self.transitions.len(),
            accessibility_option_count: self.accessibility_options.len(),
            screen_kinds: self.screens.iter().map(|s| s.kind.clone()).collect(),
            boundary: self.boundary.clone(),
        }
    }
}
