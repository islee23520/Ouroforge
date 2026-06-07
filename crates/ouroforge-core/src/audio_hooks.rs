//! Adaptive-Audio Runtime Hooks v1 (#1644).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! Adaptive audio reacts to the game's world state — combat music when enemies
//! appear, a low-health sting, a boss theme — by emitting **audio intents**. This
//! module is the trusted, deterministic evaluator for those hooks: given a set of
//! declarative hooks and a bounded snapshot of world-state signals, it produces
//! the ordered list of audio intents to emit.
//!
//! Boundary: this reuses the existing runtime audio-intent surface — it emits
//! [`crate::behavior_runtime::BehaviorIntent`] values that slot into
//! `world_state.audio_intents` — and adds adaptive-audio hooks, **not a new audio
//! engine**. It emits intent metadata only; it does not synthesize, mix, decode,
//! or play audio. Evaluation is a pure function of (hooks, signals): it reads no
//! clock, filesystem, or network and performs no trusted write, so identical
//! signals always yield identical intents and a restored world-state snapshot
//! reproduces exactly the same audio intents.

use crate::behavior_runtime::BehaviorIntent;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const AUDIO_HOOKS_SCHEMA_VERSION: &str = "audio-hooks-v1";

/// A bounded snapshot of world-state signals an adaptive-audio hook may read.
/// Sorted maps/sets keep evaluation deterministic. The deterministic runtime
/// derives these from world state; the evaluator never reads world state directly.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioHookSignals {
    #[serde(default)]
    pub flags: BTreeMap<String, String>,
    #[serde(default)]
    pub numbers: BTreeMap<String, f64>,
    #[serde(default)]
    pub events: BTreeSet<String>,
}

/// The condition under which a hook fires. Tagged by `kind`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AudioHookCondition {
    /// Always fires.
    Always,
    /// Fires when `flags[flag] == value`.
    FlagEquals { flag: String, value: String },
    /// Fires when `numbers[signal] >= threshold`.
    NumberAtLeast { signal: String, threshold: f64 },
    /// Fires when `numbers[signal] < threshold`.
    NumberBelow { signal: String, threshold: f64 },
    /// Fires when `event` is present in the signal events.
    EventPresent { event: String },
}

impl AudioHookCondition {
    fn matches(&self, signals: &AudioHookSignals) -> bool {
        match self {
            AudioHookCondition::Always => true,
            AudioHookCondition::FlagEquals { flag, value } => {
                signals.flags.get(flag).map(String::as_str) == Some(value.as_str())
            }
            AudioHookCondition::NumberAtLeast { signal, threshold } => signals
                .numbers
                .get(signal)
                .is_some_and(|value| *value >= *threshold),
            AudioHookCondition::NumberBelow { signal, threshold } => signals
                .numbers
                .get(signal)
                .is_some_and(|value| *value < *threshold),
            AudioHookCondition::EventPresent { event } => signals.events.contains(event),
        }
    }

    fn validate(&self, hook_id: &str) -> Result<()> {
        match self {
            AudioHookCondition::Always => {}
            AudioHookCondition::FlagEquals { flag, value } => {
                require_text(&format!("audio hook {hook_id} condition flag"), flag)?;
                require_text(&format!("audio hook {hook_id} condition value"), value)?;
            }
            AudioHookCondition::NumberAtLeast { signal, threshold }
            | AudioHookCondition::NumberBelow { signal, threshold } => {
                require_text(&format!("audio hook {hook_id} condition signal"), signal)?;
                if !threshold.is_finite() {
                    return Err(anyhow!(
                        "audio hook {hook_id} condition threshold must be finite"
                    ));
                }
            }
            AudioHookCondition::EventPresent { event } => {
                require_text(&format!("audio hook {hook_id} condition event"), event)?;
            }
        }
        Ok(())
    }
}

/// The audio intent a hook emits when its condition fires. Mirrors the existing
/// [`BehaviorIntent`] audio-intent shape so it slots into the runtime surface.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AudioHookIntent {
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "targetEntityId")]
    pub target_entity_id: String,
    pub intent: String,
}

impl AudioHookIntent {
    fn validate(&self, hook_id: &str) -> Result<()> {
        require_text(
            &format!("audio hook {hook_id} emit behaviorId"),
            &self.behavior_id,
        )?;
        require_text(
            &format!("audio hook {hook_id} emit actionId"),
            &self.action_id,
        )?;
        require_text(
            &format!("audio hook {hook_id} emit targetEntityId"),
            &self.target_entity_id,
        )?;
        require_text(&format!("audio hook {hook_id} emit intent"), &self.intent)?;
        Ok(())
    }

    fn to_behavior_intent(&self) -> BehaviorIntent {
        BehaviorIntent {
            behavior_id: self.behavior_id.clone(),
            action_id: self.action_id.clone(),
            target_entity_id: self.target_entity_id.clone(),
            intent: self.intent.clone(),
        }
    }
}

/// A single adaptive-audio hook: a condition over world-state signals and the
/// audio intent to emit when it fires. `priority` orders emitted intents
/// deterministically (higher first; ties broken by `hookId`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioHook {
    #[serde(rename = "hookId")]
    pub hook_id: String,
    #[serde(default)]
    pub priority: i64,
    pub when: AudioHookCondition,
    pub emit: AudioHookIntent,
}

/// A validated set of adaptive-audio hooks.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioHookSet {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub hooks: Vec<AudioHook>,
}

impl AudioHookSet {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let set: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("audio hook set is not valid JSON: {err}"))?;
        set.validate()?;
        Ok(set)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != AUDIO_HOOKS_SCHEMA_VERSION {
            return Err(anyhow!(
                "audio hook set schemaVersion must be \"{AUDIO_HOOKS_SCHEMA_VERSION}\""
            ));
        }
        if self.hooks.is_empty() {
            return Err(anyhow!("audio hook set must declare at least one hook"));
        }
        if self.hooks.len() > 64 {
            return Err(anyhow!("audio hook set is overbroad for v1 (max 64 hooks)"));
        }
        let mut seen = BTreeSet::new();
        for hook in &self.hooks {
            require_text("audio hook hookId", &hook.hook_id)?;
            if !seen.insert(hook.hook_id.as_str()) {
                return Err(anyhow!(
                    "audio hook hookId \"{}\" is declared more than once",
                    hook.hook_id
                ));
            }
            hook.when.validate(&hook.hook_id)?;
            hook.emit.validate(&hook.hook_id)?;
        }
        Ok(())
    }

    /// Evaluate the hooks against a signal snapshot and return the audio intents
    /// to emit, deterministically ordered (priority descending, then hookId
    /// ascending). Pure: identical signals always yield identical intents, so a
    /// restored world-state snapshot reproduces exactly the same audio intents.
    pub fn evaluate(&self, signals: &AudioHookSignals) -> Vec<BehaviorIntent> {
        let mut matched: Vec<&AudioHook> = self
            .hooks
            .iter()
            .filter(|hook| hook.when.matches(signals))
            .collect();
        matched.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.hook_id.cmp(&b.hook_id))
        });
        matched
            .into_iter()
            .map(|hook| hook.emit.to_behavior_intent())
            .collect()
    }
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}
