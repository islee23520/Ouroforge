//! Narrative/Dialogue/Event System v1 (#1661) — an optional, data-driven
//! dialogue/event system as a data system on the existing runtime event/state
//! model.
//!
//! This module owns the trusted, deterministic narrative state for a story:
//! declared boolean flags, a declared dialogue-node graph (linear `next` or
//! branching `choices`), and declared events that fire when their flag
//! conditions are met. It is a data system, not a new runtime or scripting
//! engine: nodes and events only set declared flags and follow declared edges;
//! there is no executable content.
//!
//! Determinism: flags are an ordered `BTreeMap`, advancing processes the current
//! node (setting its flags), then re-evaluates events to a fixpoint in declared
//! order, then moves along the chosen/linear edge. The same definition and the
//! same choices always reproduce the same state, and a state round-trips through
//! JSON unchanged.
//!
//! Fail-closed: validation rejects malformed definitions, an undeclared flag, an
//! ambiguous node (both `next` and `choices`), a dangling node reference, a
//! non-deterministic choice (duplicate action), a missing/invalid choice at
//! runtime, advancing an ended dialogue, and a non-canonical boundary — each
//! with a structured reason.
//!
//! Boundary: Rust/local owns this trusted state. The in-game dialogue/event UI
//! is read-only JavaScript runtime presentation; the browser never writes
//! trusted state. Generated narrative content is a proposal through the existing
//! review/apply/trust-gradient path, never a direct trusted write, and narrative
//! tone/quality remains a human decision. See `docs/long-form-systems-v1.md`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const NARRATIVE_SYSTEM_SCHEMA_VERSION: &str = "narrative-system-v1";

/// Canonical trust boundary recorded on every definition and state.
pub const NARRATIVE_SYSTEM_BOUNDARY: &str =
    "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient";

/// A branching dialogue choice: a player action and the node it leads to.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DialogueChoice {
    pub action: String,
    pub to: String,
}

/// One dialogue node. Entering (processing) a node sets its `set_flags`. A node
/// is linear (`next`), branching (`choices`), or terminal (neither); declaring
/// both `next` and `choices` is rejected as ambiguous.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DialogueNode {
    pub id: String,
    #[serde(default)]
    pub set_flags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default)]
    pub choices: Vec<DialogueChoice>,
}

/// A condition on a declared flag.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlagCondition {
    pub flag: String,
    pub equals: bool,
}

/// A declared event: when all `when` conditions hold it fires once, setting
/// `set_flags`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeEvent {
    pub id: String,
    #[serde(default)]
    pub when: Vec<FlagCondition>,
    #[serde(default)]
    pub set_flags: Vec<String>,
}

/// The declarative narrative definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeDefinition {
    pub schema_version: String,
    pub story_id: String,
    pub flags: Vec<String>,
    pub nodes: Vec<DialogueNode>,
    #[serde(default)]
    pub events: Vec<NarrativeEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_node: Option<String>,
    pub boundary: String,
}

/// The persistent narrative state: flags, the current dialogue node (or `None`
/// when ended), fired events, and visited nodes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeState {
    pub schema_version: String,
    pub story_id: String,
    pub flags: BTreeMap<String, bool>,
    pub current_node: Option<String>,
    pub fired_events: BTreeSet<String>,
    pub visited_nodes: Vec<String>,
    pub boundary: String,
}

/// Read-only summary for browser/Studio presentation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeReadModel {
    pub schema_version: String,
    pub story_id: String,
    pub current_node: Option<String>,
    pub ended: bool,
    pub flags: BTreeMap<String, bool>,
    pub fired_event_count: usize,
    pub visited_node_count: usize,
    pub boundary: String,
}

impl NarrativeDefinition {
    /// Parse and validate a definition from JSON, failing closed on malformed or
    /// out-of-contract input.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let definition: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid narrative definition json: {err}"))?;
        definition.validate()?;
        Ok(definition)
    }

    /// Validate the definition, failing closed on any contract violation.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected narrative schema version: {} (expected {})",
                self.schema_version,
                NARRATIVE_SYSTEM_SCHEMA_VERSION
            ));
        }
        if self.story_id.trim().is_empty() {
            return Err(anyhow!("narrative story_id must not be empty"));
        }
        if self.boundary != NARRATIVE_SYSTEM_BOUNDARY {
            return Err(anyhow!(
                "narrative definition boundary must be the canonical read-only/proposal-only contract"
            ));
        }

        let mut flag_ids: BTreeSet<&str> = BTreeSet::new();
        for flag in &self.flags {
            if flag.trim().is_empty() {
                return Err(anyhow!("narrative flag name must not be empty"));
            }
            if !flag_ids.insert(flag.as_str()) {
                return Err(anyhow!("duplicate narrative flag '{}'", flag));
            }
        }

        if self.nodes.is_empty() {
            return Err(anyhow!("narrative must declare at least one dialogue node"));
        }
        let mut node_ids: BTreeSet<&str> = BTreeSet::new();
        for node in &self.nodes {
            if node.id.trim().is_empty() {
                return Err(anyhow!("narrative node id must not be empty"));
            }
            if !node_ids.insert(node.id.as_str()) {
                return Err(anyhow!("duplicate narrative node id '{}'", node.id));
            }
        }
        for node in &self.nodes {
            for flag in &node.set_flags {
                if !flag_ids.contains(flag.as_str()) {
                    return Err(anyhow!(
                        "node '{}' sets undeclared flag '{}'",
                        node.id,
                        flag
                    ));
                }
            }
            if node.next.is_some() && !node.choices.is_empty() {
                return Err(anyhow!(
                    "node '{}' is ambiguous: declare either next or choices, not both",
                    node.id
                ));
            }
            if let Some(next) = &node.next {
                if !node_ids.contains(next.as_str()) {
                    return Err(anyhow!(
                        "node '{}' has dangling next reference '{}'",
                        node.id,
                        next
                    ));
                }
            }
            let mut choice_actions: BTreeSet<&str> = BTreeSet::new();
            for choice in &node.choices {
                if choice.action.trim().is_empty() {
                    return Err(anyhow!("node '{}' has an empty choice action", node.id));
                }
                if !choice_actions.insert(choice.action.as_str()) {
                    return Err(anyhow!(
                        "node '{}' has non-deterministic choice action '{}'",
                        node.id,
                        choice.action
                    ));
                }
                if !node_ids.contains(choice.to.as_str()) {
                    return Err(anyhow!(
                        "node '{}' choice '{}' has dangling target '{}'",
                        node.id,
                        choice.action,
                        choice.to
                    ));
                }
            }
        }

        let mut event_ids: BTreeSet<&str> = BTreeSet::new();
        for event in &self.events {
            if event.id.trim().is_empty() {
                return Err(anyhow!("narrative event id must not be empty"));
            }
            if !event_ids.insert(event.id.as_str()) {
                return Err(anyhow!("duplicate narrative event id '{}'", event.id));
            }
            for condition in &event.when {
                if !flag_ids.contains(condition.flag.as_str()) {
                    return Err(anyhow!(
                        "event '{}' references undeclared flag '{}'",
                        event.id,
                        condition.flag
                    ));
                }
            }
            for flag in &event.set_flags {
                if !flag_ids.contains(flag.as_str()) {
                    return Err(anyhow!(
                        "event '{}' sets undeclared flag '{}'",
                        event.id,
                        flag
                    ));
                }
            }
        }

        if let Some(initial) = &self.initial_node {
            if !node_ids.contains(initial.as_str()) {
                return Err(anyhow!(
                    "narrative initialNode '{}' is not a declared node",
                    initial
                ));
            }
        }

        Ok(())
    }

    fn node(&self, id: &str) -> Option<&DialogueNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// The initial narrative state: every declared flag `false`, the current
    /// node set to `initialNode`, and events evaluated once to a fixpoint.
    pub fn initial_state(&self) -> NarrativeState {
        let flags: BTreeMap<String, bool> = self.flags.iter().map(|f| (f.clone(), false)).collect();
        let mut state = NarrativeState {
            schema_version: NARRATIVE_SYSTEM_SCHEMA_VERSION.to_string(),
            story_id: self.story_id.clone(),
            flags,
            current_node: self.initial_node.clone(),
            fired_events: BTreeSet::new(),
            visited_nodes: Vec::new(),
            boundary: NARRATIVE_SYSTEM_BOUNDARY.to_string(),
        };
        self.evaluate_events(&mut state);
        state
    }

    /// Fire every eligible, not-yet-fired event to a fixpoint in declared order.
    fn evaluate_events(&self, state: &mut NarrativeState) {
        loop {
            let mut fired_any = false;
            for event in &self.events {
                if state.fired_events.contains(event.id.as_str()) {
                    continue;
                }
                let satisfied = event
                    .when
                    .iter()
                    .all(|c| state.flags.get(&c.flag).copied().unwrap_or(false) == c.equals);
                if satisfied {
                    for flag in &event.set_flags {
                        state.flags.insert(flag.clone(), true);
                    }
                    state.fired_events.insert(event.id.clone());
                    fired_any = true;
                }
            }
            if !fired_any {
                break;
            }
        }
    }

    /// Advance the dialogue: process the current node (set its flags), evaluate
    /// events to a fixpoint, then follow the chosen branch or the linear `next`.
    /// `choice` is required for a branching node and ignored for a linear one.
    /// Fails closed on an ended dialogue, an inconsistent state, or a
    /// missing/invalid choice.
    pub fn advance(&self, state: &NarrativeState, choice: Option<&str>) -> Result<NarrativeState> {
        self.validate_state(state)?;
        let current_id = state
            .current_node
            .as_ref()
            .ok_or_else(|| anyhow!("narrative dialogue has already ended"))?;
        let node = self
            .node(current_id)
            .ok_or_else(|| anyhow!("narrative current node '{}' is not declared", current_id))?;

        let mut next = state.clone();
        for flag in &node.set_flags {
            next.flags.insert(flag.clone(), true);
        }
        next.visited_nodes.push(node.id.clone());

        let destination = if node.choices.is_empty() {
            node.next.clone()
        } else {
            let action = choice
                .ok_or_else(|| anyhow!("narrative node '{}' requires a choice action", node.id))?;
            let chosen = node
                .choices
                .iter()
                .find(|c| c.action == action)
                .ok_or_else(|| {
                    anyhow!("narrative node '{}' has no choice '{}'", node.id, action)
                })?;
            Some(chosen.to.clone())
        };
        next.current_node = destination;
        self.evaluate_events(&mut next);
        Ok(next)
    }

    /// Validate that a (possibly restored) state is consistent with this
    /// definition.
    pub fn validate_state(&self, state: &NarrativeState) -> Result<()> {
        if state.schema_version != NARRATIVE_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected narrative state schema version: {} (expected {})",
                state.schema_version,
                NARRATIVE_SYSTEM_SCHEMA_VERSION
            ));
        }
        if state.story_id != self.story_id {
            return Err(anyhow!(
                "narrative state story '{}' does not match definition story '{}'",
                state.story_id,
                self.story_id
            ));
        }
        if state.boundary != NARRATIVE_SYSTEM_BOUNDARY {
            return Err(anyhow!(
                "narrative state boundary must be the canonical read-only/proposal-only contract"
            ));
        }
        let declared_flags: BTreeSet<&str> = self.flags.iter().map(String::as_str).collect();
        let present_flags: BTreeSet<&str> = state.flags.keys().map(String::as_str).collect();
        if declared_flags != present_flags {
            return Err(anyhow!("narrative state flags do not match declared flags"));
        }
        let node_ids: BTreeSet<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
        if let Some(current) = &state.current_node {
            if !node_ids.contains(current.as_str()) {
                return Err(anyhow!(
                    "narrative state current node '{}' is not declared",
                    current
                ));
            }
        }
        for fired in &state.fired_events {
            // A fired event must be declared, and (because flags only ever go
            // true) all of its effects must be present in the restored state.
            // Otherwise `evaluate_events` would skip the already-fired event and
            // its required flags would never be set after restore.
            let event =
                self.events.iter().find(|e| &e.id == fired).ok_or_else(|| {
                    anyhow!("narrative state has unknown fired event '{}'", fired)
                })?;
            for flag in &event.set_flags {
                if state.flags.get(flag).copied() != Some(true) {
                    return Err(anyhow!(
                        "narrative state marks event '{}' fired but its effect flag '{}' is not set",
                        fired,
                        flag
                    ));
                }
            }
        }
        Ok(())
    }

    /// Derive the read-only presentation summary for browser/Studio surfaces.
    pub fn read_model(&self, state: &NarrativeState) -> NarrativeReadModel {
        NarrativeReadModel {
            schema_version: state.schema_version.clone(),
            story_id: state.story_id.clone(),
            current_node: state.current_node.clone(),
            ended: state.current_node.is_none(),
            flags: state.flags.clone(),
            fired_event_count: state.fired_events.len(),
            visited_node_count: state.visited_nodes.len(),
            boundary: state.boundary.clone(),
        }
    }
}

impl NarrativeState {
    /// Serialize the trusted state to canonical JSON for persistence.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|err| anyhow!("failed to serialize narrative state: {err}"))
    }

    /// Parse a persisted state from JSON, failing closed on malformed input or a
    /// wrong schema version.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let state: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid narrative state json: {err}"))?;
        if state.schema_version != NARRATIVE_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected narrative state schema version: {} (expected {})",
                state.schema_version,
                NARRATIVE_SYSTEM_SCHEMA_VERSION
            ));
        }
        Ok(state)
    }

    /// Whether the dialogue has ended (no current node).
    pub fn is_ended(&self) -> bool {
        self.current_node.is_none()
    }
}
