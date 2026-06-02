use anyhow::{anyhow, Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tungstenite::client::IntoClientRequest;

static LEDGER_APPEND_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static EVIDENCE_INDEX_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub id: String,
    pub title: String,
    pub goal: String,
    pub constraints: Constraints,
    pub acceptance: Vec<String>,
    pub scenarios: Vec<Scenario>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluator: Option<EvaluatorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Constraints {
    pub target: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvaluatorConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub console: Option<ConsoleEvaluatorConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<PerformanceEvaluatorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConsoleEvaluatorConfig {
    #[serde(rename = "failOnLevels")]
    pub fail_on_levels: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceEvaluatorConfig {
    #[serde(rename = "maxMetrics")]
    pub max_metrics: std::collections::BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub steps: Vec<ScenarioStep>,
    #[serde(default)]
    pub assertions: Vec<ScenarioAssertion>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ScenarioStep {
    Wait {
        wait: WaitStep,
    },
    Input {
        input: InputStep,
    },
    Replay {
        replay: InputReplay,
    },
    ReplayRef {
        #[serde(rename = "replayRef")]
        replay_ref: ReplayReference,
    },
    Snapshot {
        snapshot: SnapshotStep,
    },
    Restore {
        restore: RestoreStep,
    },
    VisualCheckpoint {
        #[serde(rename = "visualCheckpoint")]
        visual_checkpoint: VisualCheckpointStep,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WaitStep {
    pub frames: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct InputStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SnapshotStep {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RestoreStep {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualCheckpointStep {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<VisualBaselineMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<VisualThreshold>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualBaselineMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualThreshold {
    #[serde(rename = "maxDimensionDelta")]
    pub max_dimension_delta: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReplayReference {
    pub id: String,
    pub path: String,
}

const INPUT_REPLAY_SCHEMA_VERSION: &str = "1";
const MAX_INPUT_REPLAY_EVENTS: usize = 10_000;
const MAX_INPUT_REPLAY_FRAME: u32 = 100_000;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputReplay {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub id: String,
    pub events: Vec<InputReplayEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputReplayEvent {
    pub frame: u32,
    pub key: ReplayKey,
    pub pressed: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ReplayKey {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ScenarioAssertion {
    WorldState {
        world_state: JsonPathAssertion,
    },
    FrameStats {
        frame_stats: JsonPathAssertion,
    },
    RuntimeEvents {
        runtime_events: JsonPathAssertion,
    },
    PerformanceMetrics {
        performance_metrics: JsonPathAssertion,
    },
    ConsoleErrors {
        console_errors: JsonPathAssertion,
    },
    CollisionEvidence {
        collision_evidence: JsonPathAssertion,
    },
    AudioEvidence {
        audio_evidence: JsonPathAssertion,
    },
    AnimationEvidence {
        animation_evidence: JsonPathAssertion,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct JsonPathAssertion {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<serde_json::Value>,
    #[serde(default, rename = "notEquals", skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<serde_json::Value>,
    #[serde(
        default,
        rename = "greaterThan",
        skip_serializing_if = "Option::is_none"
    )]
    pub greater_than: Option<serde_json::Value>,
    #[serde(default, rename = "lessThan", skip_serializing_if = "Option::is_none")]
    pub less_than: Option<serde_json::Value>,
    #[serde(
        default,
        rename = "countEquals",
        skip_serializing_if = "Option::is_none"
    )]
    pub count_equals: Option<u64>,
    #[serde(
        default,
        rename = "countGreaterThan",
        skip_serializing_if = "Option::is_none"
    )]
    pub count_greater_than: Option<u64>,
    #[serde(
        default,
        rename = "countLessThan",
        skip_serializing_if = "Option::is_none"
    )]
    pub count_less_than: Option<u64>,
}

impl Seed {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        let seed: Seed = serde_yaml::from_str(input).context("failed to parse Seed YAML")?;
        seed.validate()?;
        Ok(seed)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path)
            .with_context(|| format!("failed to read Seed file {}", path.display()))?;
        let seed = Self::from_yaml_str(&input)?;
        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
        seed.validate_replay_references(base_dir)?;
        Ok(seed)
    }

    pub fn validate(&self) -> Result<()> {
        require_text("id", &self.id)?;
        require_text("title", &self.title)?;
        require_text("goal", &self.goal)?;
        require_text("constraints.target", &self.constraints.target)?;

        if self.acceptance.is_empty() {
            return Err(anyhow!("acceptance must contain at least one item"));
        }
        for (index, item) in self.acceptance.iter().enumerate() {
            require_text(&format!("acceptance[{index}]"), item)?;
        }

        if let Some(evaluator) = &self.evaluator {
            evaluator.validate()?;
        }

        if self.scenarios.is_empty() {
            return Err(anyhow!("scenarios must contain at least one item"));
        }
        for (index, scenario) in self.scenarios.iter().enumerate() {
            scenario.validate(index)?;
        }

        Ok(())
    }

    fn validate_replay_references(&self, base_dir: &Path) -> Result<()> {
        for (scenario_index, scenario) in self.scenarios.iter().enumerate() {
            for (step_index, step) in scenario.steps.iter().enumerate() {
                if let ScenarioStep::ReplayRef { replay_ref } = step {
                    replay_ref
                        .load_from_base(base_dir)
                        .with_context(|| {
                            format!(
                                "scenarios[{scenario_index}].steps[{step_index}].replayRef could not be loaded"
                            )
                        })?;
                }
            }
        }
        Ok(())
    }

    fn replay_references(&self) -> Vec<&ReplayReference> {
        self.scenarios
            .iter()
            .flat_map(|scenario| scenario.steps.iter())
            .filter_map(|step| match step {
                ScenarioStep::ReplayRef { replay_ref } => Some(replay_ref),
                _ => None,
            })
            .collect()
    }
}

impl EvaluatorConfig {
    fn validate(&self) -> Result<()> {
        if let Some(console) = &self.console {
            console.validate()?;
        }
        if let Some(performance) = &self.performance {
            performance.validate()?;
        }
        Ok(())
    }
}

impl ConsoleEvaluatorConfig {
    fn validate(&self) -> Result<()> {
        if self.fail_on_levels.is_empty() {
            return Err(anyhow!("evaluator.console.failOnLevels must not be empty"));
        }
        for level in &self.fail_on_levels {
            if !matches!(level.as_str(), "debug" | "info" | "log" | "warn" | "error") {
                return Err(anyhow!(
                    "evaluator.console.failOnLevels entries must be debug, info, log, warn, or error"
                ));
            }
        }
        Ok(())
    }
}

impl PerformanceEvaluatorConfig {
    fn validate(&self) -> Result<()> {
        if self.max_metrics.is_empty() {
            return Err(anyhow!(
                "evaluator.performance.maxMetrics must not be empty"
            ));
        }
        for (metric, threshold) in &self.max_metrics {
            require_text("evaluator.performance metric name", metric)?;
            if *threshold == 0 {
                return Err(anyhow!(
                    "evaluator.performance.maxMetrics thresholds must be greater than 0"
                ));
            }
        }
        Ok(())
    }
}

impl Scenario {
    fn validate(&self, index: usize) -> Result<()> {
        require_text(&format!("scenarios[{index}].id"), &self.id)?;
        require_text(
            &format!("scenarios[{index}].description"),
            &self.description,
        )?;
        for (step_index, step) in self.steps.iter().enumerate() {
            step.validate(index, step_index)?;
        }
        for (assertion_index, assertion) in self.assertions.iter().enumerate() {
            assertion.validate(index, assertion_index)?;
        }
        Ok(())
    }
}

impl ScenarioStep {
    fn validate(&self, scenario_index: usize, step_index: usize) -> Result<()> {
        match self {
            ScenarioStep::Wait { wait } => {
                if wait.frames == 0 {
                    return Err(anyhow!(
                        "scenarios[{scenario_index}].steps[{step_index}].wait.frames must be greater than 0"
                    ));
                }
            }
            ScenarioStep::Input { input } => {
                if input.left.is_none()
                    && input.right.is_none()
                    && input.up.is_none()
                    && input.down.is_none()
                {
                    return Err(anyhow!(
                        "scenarios[{scenario_index}].steps[{step_index}].input must set at least one direction"
                    ));
                }
            }
            ScenarioStep::Replay { replay } => replay.validate().with_context(|| {
                format!("scenarios[{scenario_index}].steps[{step_index}].replay is invalid")
            })?,
            ScenarioStep::ReplayRef { replay_ref } => replay_ref.validate().with_context(|| {
                format!("scenarios[{scenario_index}].steps[{step_index}].replayRef is invalid")
            })?,
            ScenarioStep::Snapshot { snapshot } => {
                validate_path_component("snapshot step id", &snapshot.id).with_context(|| {
                    format!(
                        "scenarios[{scenario_index}].steps[{step_index}].snapshot.id is invalid"
                    )
                })?;
            }
            ScenarioStep::Restore { restore } => {
                validate_path_component("restore step id", &restore.id).with_context(|| {
                    format!("scenarios[{scenario_index}].steps[{step_index}].restore.id is invalid")
                })?;
            }
            ScenarioStep::VisualCheckpoint { visual_checkpoint } => {
                visual_checkpoint.validate().with_context(|| {
                    format!(
                        "scenarios[{scenario_index}].steps[{step_index}].visualCheckpoint is invalid"
                    )
                })?;
            }
        }
        Ok(())
    }
}

impl VisualCheckpointStep {
    fn validate(&self) -> Result<()> {
        validate_path_component("visual checkpoint id", &self.id)?;
        if let Some(baseline) = &self.baseline {
            baseline.validate()?;
        }
        if self.threshold.is_some() {
            let baseline = self
                .baseline
                .as_ref()
                .ok_or_else(|| anyhow!("visual checkpoint threshold requires baseline metadata"))?;
            if baseline.width.is_none() || baseline.height.is_none() {
                return Err(anyhow!(
                    "visual checkpoint threshold requires baseline width and height"
                ));
            }
        }
        Ok(())
    }
}

impl VisualBaselineMetadata {
    fn validate(&self) -> Result<()> {
        if let Some(id) = &self.id {
            validate_path_component("visual baseline id", id)?;
        }
        if let Some(path) = &self.path {
            validate_replay_reference_path("visual baseline path", path)?;
        }
        Ok(())
    }
}

impl ReplayReference {
    fn validate(&self) -> Result<()> {
        validate_path_component("replay reference id", &self.id)?;
        validate_replay_reference_path("replay reference path", &self.path)
    }

    fn load_from_base(&self, base_dir: &Path) -> Result<InputReplay> {
        self.validate()?;
        let path = base_dir.join(&self.path);
        let input = fs::read_to_string(&path)
            .with_context(|| format!("failed to read replay reference {}", path.display()))?;
        let replay = if self.path.ends_with(".json") {
            InputReplay::from_json_str(&input)
        } else {
            InputReplay::from_yaml_str(&input)
        }
        .with_context(|| format!("failed to parse replay reference {}", path.display()))?;
        if replay.id != self.id {
            return Err(anyhow!(
                "replay reference id {} does not match replay id {}",
                self.id,
                replay.id
            ));
        }
        Ok(replay)
    }
}

impl InputReplay {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        let replay: InputReplay =
            serde_yaml::from_str(input).context("failed to parse Input Replay YAML")?;
        replay.validate()?;
        Ok(replay)
    }

    pub fn from_json_str(input: &str) -> Result<Self> {
        let replay: InputReplay =
            serde_json::from_str(input).context("failed to parse Input Replay JSON")?;
        replay.validate()?;
        Ok(replay)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != INPUT_REPLAY_SCHEMA_VERSION {
            return Err(anyhow!(
                "input replay schemaVersion must be {INPUT_REPLAY_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("input replay id", &self.id)?;
        if self.events.is_empty() {
            return Err(anyhow!(
                "input replay events must contain at least one event"
            ));
        }
        if self.events.len() > MAX_INPUT_REPLAY_EVENTS {
            return Err(anyhow!(
                "input replay events must contain at most {MAX_INPUT_REPLAY_EVENTS} events"
            ));
        }

        let mut last_frame = 0;
        for (index, event) in self.events.iter().enumerate() {
            if event.frame > MAX_INPUT_REPLAY_FRAME {
                return Err(anyhow!(
                    "input replay events[{index}].frame must be <= {MAX_INPUT_REPLAY_FRAME}"
                ));
            }
            if index > 0 && event.frame < last_frame {
                return Err(anyhow!(
                    "input replay events must be ordered by nondecreasing frame"
                ));
            }
            last_frame = event.frame;
        }
        Ok(())
    }
}

fn validate_replay_reference_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if !value.starts_with("replays/") {
        return Err(anyhow!("{field} must start with replays/"));
    }
    if !(value.ends_with(".yaml") || value.ends_with(".yml") || value.ends_with(".json")) {
        return Err(anyhow!("{field} must point to a YAML or JSON replay file"));
    }
    if !value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '-' | '_'))
    {
        return Err(anyhow!(
            "{field} may only contain ASCII letters, numbers, '/', '.', '-' or '_'"
        ));
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => return Err(anyhow!("{field} must stay inside the replay fixture tree")),
        }
    }
    Ok(())
}

const ASSET_MANIFEST_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetManifest {
    #[serde(rename = "schemaVersion")]
    #[serde(default = "asset_manifest_schema_v1")]
    pub schema_version: String,
    pub id: String,
    pub assets: Vec<AssetManifestEntry>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetManifestEntry {
    pub id: String,
    pub kind: AssetManifestKind,
    pub path: String,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AssetManifestKind {
    Image,
    Sprite,
    Audio,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResolvedAssetManifestEntry {
    pub id: String,
    pub kind: AssetManifestKind,
    pub path: String,
}

fn asset_manifest_schema_v1() -> String {
    ASSET_MANIFEST_SCHEMA_VERSION.to_string()
}

impl AssetManifest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let manifest: AssetManifest =
            serde_json::from_str(input).context("failed to parse Asset Manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path)
            .with_context(|| format!("failed to read asset manifest {}", path.display()))?;
        let manifest = Self::from_json_str(&input)
            .with_context(|| format!("failed to parse asset manifest {}", path.display()))?;
        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
        manifest
            .validate_files(base_dir)
            .with_context(|| format!("asset manifest {} references invalid files", manifest.id))?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ASSET_MANIFEST_SCHEMA_VERSION {
            return Err(anyhow!(
                "asset manifest schemaVersion must be {ASSET_MANIFEST_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("asset manifest id", &self.id)?;
        validate_scene_metadata("asset manifest metadata", &self.metadata)?;
        if self.assets.is_empty() {
            return Err(anyhow!("asset manifest assets must not be empty"));
        }

        let mut ids = BTreeSet::new();
        let mut paths = BTreeSet::new();
        for asset in &self.assets {
            asset.validate()?;
            if !ids.insert(asset.id.clone()) {
                return Err(anyhow!("duplicate asset manifest asset id: {}", asset.id));
            }
            if !paths.insert(asset.path.clone()) {
                return Err(anyhow!(
                    "duplicate asset manifest asset path: {}",
                    asset.path
                ));
            }
        }
        Ok(())
    }

    pub fn validate_files(&self, base_dir: &Path) -> Result<()> {
        self.validate()?;
        let base = base_dir.canonicalize().with_context(|| {
            format!(
                "failed to resolve asset manifest base {}",
                base_dir.display()
            )
        })?;
        for asset in &self.assets {
            let path = base_dir.join(&asset.path);
            if !path.is_file() {
                return Err(anyhow!(
                    "asset manifest asset {} missing file: {}",
                    asset.id,
                    asset.path
                ));
            }
            let resolved = path.canonicalize().with_context(|| {
                format!(
                    "failed to resolve asset manifest asset {} path {}",
                    asset.id, asset.path
                )
            })?;
            if !resolved.starts_with(&base) {
                return Err(anyhow!(
                    "asset manifest asset {} must stay inside the manifest asset tree",
                    asset.id
                ));
            }
        }
        Ok(())
    }

    pub fn resolved_assets(&self) -> Vec<ResolvedAssetManifestEntry> {
        let mut assets = self
            .assets
            .iter()
            .map(|asset| ResolvedAssetManifestEntry {
                id: asset.id.clone(),
                kind: asset.kind,
                path: asset.path.clone(),
            })
            .collect::<Vec<_>>();
        assets.sort_by(|left, right| {
            (left.id.as_str(), left.kind, left.path.as_str()).cmp(&(
                right.id.as_str(),
                right.kind,
                right.path.as_str(),
            ))
        });
        assets
    }
}

impl AssetManifestEntry {
    fn validate(&self) -> Result<()> {
        validate_path_component("asset manifest asset id", &self.id)?;
        validate_asset_manifest_path("asset manifest asset path", &self.path, self.kind)?;
        validate_scene_metadata(
            &format!("asset manifest asset {} metadata", self.id),
            &self.metadata,
        )
    }
}

fn validate_asset_manifest_path(field: &str, value: &str, kind: AssetManifestKind) -> Result<()> {
    validate_scene_local_asset_path(field, value)?;
    let extension = Path::new(value)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let supported = match kind {
        AssetManifestKind::Image | AssetManifestKind::Sprite => {
            matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "svg" | "webp")
        }
        AssetManifestKind::Audio => matches!(extension.as_str(), "ogg" | "mp3" | "wav"),
    };
    if supported {
        Ok(())
    } else {
        Err(anyhow!(
            "{field} has unsupported extension for asset kind {:?}",
            kind
        ))
    }
}

impl ScenarioAssertion {
    fn validate(&self, scenario_index: usize, assertion_index: usize) -> Result<()> {
        let assertion = match self {
            ScenarioAssertion::WorldState { world_state } => world_state,
            ScenarioAssertion::FrameStats { frame_stats } => frame_stats,
            ScenarioAssertion::RuntimeEvents { runtime_events } => runtime_events,
            ScenarioAssertion::PerformanceMetrics {
                performance_metrics,
            } => performance_metrics,
            ScenarioAssertion::ConsoleErrors { console_errors } => console_errors,
            ScenarioAssertion::CollisionEvidence { collision_evidence } => collision_evidence,
            ScenarioAssertion::AudioEvidence { audio_evidence } => audio_evidence,
            ScenarioAssertion::AnimationEvidence { animation_evidence } => animation_evidence,
        };
        assertion.validate(scenario_index, assertion_index)
    }
}

impl JsonPathAssertion {
    fn validate(&self, scenario_index: usize, assertion_index: usize) -> Result<()> {
        require_text(
            &format!("scenarios[{scenario_index}].assertions[{assertion_index}].path"),
            &self.path,
        )?;
        validate_scenario_path(&self.path).with_context(|| {
            format!("scenarios[{scenario_index}].assertions[{assertion_index}].path is invalid")
        })?;
        let operator_count = [
            self.equals.is_some(),
            self.not_equals.is_some(),
            self.exists.is_some(),
            self.contains.is_some(),
            self.greater_than.is_some(),
            self.less_than.is_some(),
            self.count_equals.is_some(),
            self.count_greater_than.is_some(),
            self.count_less_than.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count();
        if operator_count != 1 {
            return Err(anyhow!(
                "scenarios[{scenario_index}].assertions[{assertion_index}] must define exactly one bounded assertion operator"
            ));
        }
        for (operator, value) in [
            ("equals", &self.equals),
            ("notEquals", &self.not_equals),
            ("contains", &self.contains),
        ] {
            if value.as_ref().is_some_and(serde_json::Value::is_null) {
                return Err(anyhow!(
                    "scenarios[{scenario_index}].assertions[{assertion_index}].{operator} must not be null"
                ));
            }
        }
        for (operator, value) in [
            ("greaterThan", &self.greater_than),
            ("lessThan", &self.less_than),
        ] {
            if value.as_ref().is_some_and(|value| !value.is_number()) {
                return Err(anyhow!(
                    "scenarios[{scenario_index}].assertions[{assertion_index}].{operator} must be numeric"
                ));
            }
        }
        Ok(())
    }
}

fn validate_scenario_path(path: &str) -> Result<()> {
    for segment in path.split('.') {
        require_text("scenario assertion path segment", segment)?;
        if !segment
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        {
            return Err(anyhow!(
                "scenario assertion paths may only contain ASCII letters, numbers, '_', '-' and '.'"
            ));
        }
    }
    Ok(())
}

fn validate_path_component(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if !value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err(anyhow!(
            "{field} may only contain ASCII letters, numbers, '-' or '_'"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

/// Fail-closed validation for URLs that reach a browser navigation sink.
///
/// The local smoke/scenario paths only ever drive `http`/`https` targets, so we
/// reject other schemes (for example `file:`, `chrome:`, `data:`) before the URL
/// is handed to CDP `Page.navigate`, preventing capture of unintended local pages.
fn require_http_url(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lowered = value.trim().to_ascii_lowercase();
    if lowered.starts_with("http://") || lowered.starts_with("https://") {
        Ok(())
    } else {
        Err(anyhow!("{field} must use http:// or https://"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub added_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceIndex {
    pub artifacts: Vec<EvidenceArtifact>,
}

pub fn append_ledger_event(
    run_dir: impl AsRef<Path>,
    kind: &str,
    actor: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value> {
    require_text("ledger event kind", kind)?;
    require_text("ledger event actor", actor)?;

    let event = json!({
        "event": kind,
        "actor": actor,
        "payload": payload,
        "created_at_unix_ms": unix_millis()?,
    });
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let line = serde_json::to_string(&event).context("failed to serialize ledger event")?;
    let _guard = LEDGER_APPEND_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| anyhow!("ledger append lock poisoned"))?;
    let mut file = OpenOptions::new()
        .create(false)
        .append(true)
        .open(&ledger_path)
        .with_context(|| format!("failed to open ledger for append {}", ledger_path.display()))?;
    writeln!(file, "{line}").context("failed to append ledger event")?;
    Ok(event)
}

pub fn read_ledger_events(run_dir: impl AsRef<Path>) -> Result<Vec<serde_json::Value>> {
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let file = File::open(&ledger_path)
        .with_context(|| format!("failed to read ledger {}", ledger_path.display()))?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!(
                "failed to read ledger line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let event: serde_json::Value = serde_json::from_str(&line).with_context(|| {
            format!(
                "failed to parse ledger JSON on line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        events.push(event);
    }

    Ok(events)
}

pub fn add_evidence_artifact(
    run_dir: impl AsRef<Path>,
    id: &str,
    kind: &str,
    path: &str,
    metadata: serde_json::Value,
) -> Result<EvidenceArtifact> {
    require_text("evidence artifact id", id)?;
    require_text("evidence artifact kind", kind)?;
    require_text("evidence artifact path", path)?;
    validate_evidence_artifact_path(path)?;

    let _guard = EVIDENCE_INDEX_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| anyhow!("evidence index lock poisoned"))?;
    let mut index = read_evidence_index(&run_dir)?;
    if index.artifacts.iter().any(|artifact| artifact.id == id) {
        return Err(anyhow!("evidence artifact id already exists: {id}"));
    }

    let artifact = EvidenceArtifact {
        id: id.to_string(),
        kind: kind.to_string(),
        path: path.to_string(),
        metadata,
        added_at_unix_ms: unix_millis()?,
    };
    index.artifacts.push(artifact.clone());
    write_evidence_index(run_dir, &index)?;
    Ok(artifact)
}

pub fn list_evidence_artifacts(run_dir: impl AsRef<Path>) -> Result<Vec<EvidenceArtifact>> {
    Ok(read_evidence_index(run_dir)?.artifacts)
}

fn validate_evidence_artifact_path(path: &str) -> Result<()> {
    let evidence_path = Path::new(path);
    if evidence_path.is_absolute() {
        return Err(anyhow!("evidence artifact path must be relative"));
    }
    if !path.starts_with("evidence/") {
        return Err(anyhow!("evidence artifact path must start with evidence/"));
    }
    for component in evidence_path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "evidence artifact path must stay inside the run evidence tree"
                ));
            }
        }
    }
    Ok(())
}

fn read_evidence_index(run_dir: impl AsRef<Path>) -> Result<EvidenceIndex> {
    let index_path = run_dir.as_ref().join("evidence/index.json");
    let input = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read evidence index {}", index_path.display()))?;
    let index: EvidenceIndex = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse evidence index {}", index_path.display()))?;
    Ok(index)
}

fn write_evidence_index(run_dir: impl AsRef<Path>, index: &EvidenceIndex) -> Result<()> {
    write_json_atomic(&run_dir.as_ref().join("evidence/index.json"), &json!(index))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdpConnectionConfig {
    pub target_ws_url: String,
    pub io_timeout: Duration,
}

impl CdpConnectionConfig {
    pub fn new(target_ws_url: impl Into<String>) -> Result<Self> {
        let target_ws_url = target_ws_url.into();
        require_text("CDP target WebSocket URL", &target_ws_url)?;
        Ok(Self {
            target_ws_url,
            io_timeout: default_cdp_io_timeout(),
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CdpTargetSelection {
    pub target_id: Option<String>,
}

impl CdpTargetSelection {
    pub fn first_page() -> Self {
        Self::default()
    }

    pub fn target_id(target_id: impl Into<String>) -> Result<Self> {
        let target_id = target_id.into();
        require_text("CDP target id", &target_id)?;
        Ok(Self {
            target_id: Some(target_id),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdpNavigateResult {
    pub frame_id: Option<String>,
    pub loader_id: Option<String>,
}

pub trait CdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>;
}

pub struct CdpClient<T> {
    transport: T,
}

impl<T: CdpTransport> CdpClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn into_transport(self) -> T {
        self.transport
    }

    pub fn navigate(&mut self, url: &str) -> Result<CdpNavigateResult> {
        require_text("navigation URL", url)?;
        let result = self
            .transport
            .send_command("Page.navigate", json!({ "url": url }))?;
        if let Some(error_text) = result.get("errorText").and_then(|value| value.as_str()) {
            return Err(anyhow!("CDP navigation failed: {error_text}"));
        }
        Ok(CdpNavigateResult {
            frame_id: result
                .get("frameId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            loader_id: result
                .get("loaderId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
        })
    }

    pub fn enable_page(&mut self) -> Result<()> {
        self.transport.send_command("Page.enable", json!({}))?;
        Ok(())
    }

    pub fn add_script_to_evaluate_on_new_document(&mut self, source: &str) -> Result<()> {
        require_text("CDP preload script", source)?;
        self.transport.send_command(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": source }),
        )?;
        Ok(())
    }

    pub fn bring_page_to_front(&mut self) -> Result<()> {
        self.transport
            .send_command("Page.bringToFront", json!({}))?;
        Ok(())
    }

    pub fn capture_screenshot_png(&mut self) -> Result<Vec<u8>> {
        let result = self
            .transport
            .send_command("Page.captureScreenshot", json!({ "format": "png" }))?;
        let data = result
            .get("data")
            .and_then(|value| value.as_str())
            .ok_or_else(|| anyhow!("CDP screenshot response missing data"))?;
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .context("failed to decode CDP screenshot data")
    }

    pub fn enable_performance(&mut self) -> Result<()> {
        self.transport
            .send_command("Performance.enable", json!({}))?;
        Ok(())
    }

    pub fn performance_metrics(&mut self) -> Result<serde_json::Value> {
        self.transport
            .send_command("Performance.getMetrics", json!({}))
    }

    pub fn evaluate_json(&mut self, expression: &str) -> Result<serde_json::Value> {
        require_text("CDP Runtime.evaluate expression", expression)?;
        let result = self.transport.send_command(
            "Runtime.evaluate",
            json!({
                "expression": expression,
                "returnByValue": true,
                "awaitPromise": false
            }),
        )?;
        if let Some(exception) = result.get("exceptionDetails") {
            return Err(anyhow!("CDP runtime evaluation failed: {exception}"));
        }
        Ok(result
            .get("result")
            .and_then(|remote_object| remote_object.get("value"))
            .cloned()
            .unwrap_or(serde_json::Value::Null))
    }
}

pub struct WebSocketCdpTransport {
    socket: tungstenite::WebSocket<std::net::TcpStream>,
    next_id: u64,
}

impl WebSocketCdpTransport {
    pub fn connect(config: &CdpConnectionConfig) -> Result<Self> {
        let request = config
            .target_ws_url
            .as_str()
            .into_client_request()
            .context("failed to build CDP WebSocket request")?;
        let endpoint = CdpWebSocketEndpoint::parse(&config.target_ws_url)?;
        let stream = endpoint.connect(config.io_timeout)?;
        let (mut socket, _) = tungstenite::client(request, stream)
            .with_context(|| format!("failed to connect to CDP target {}", config.target_ws_url))?;
        set_tcp_stream_timeouts(socket.get_mut(), config.io_timeout)?;
        Ok(Self { socket, next_id: 1 })
    }
}

impl CdpTransport for WebSocketCdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        require_text("CDP method", method)?;
        let id = self.next_id;
        self.next_id += 1;
        let request = json!({
            "id": id,
            "method": method,
            "params": params,
        });
        let request_body =
            serde_json::to_string(&request).context("failed to serialize CDP request")?;
        self.socket
            .send(tungstenite::Message::Text(request_body))
            .context("failed to send CDP request")?;

        loop {
            let message = self.socket.read().context("failed to read CDP response")?;
            let tungstenite::Message::Text(body) = message else {
                continue;
            };
            let response: serde_json::Value =
                serde_json::from_str(&body).context("failed to parse CDP response")?;
            if response.get("id").and_then(|value| value.as_u64()) != Some(id) {
                continue;
            }
            if let Some(error) = response.get("error") {
                return Err(anyhow!("CDP command {method} failed: {error}"));
            }
            return Ok(response.get("result").cloned().unwrap_or_else(|| json!({})));
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CdpTargetInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}

pub fn read_cdp_targets(debugging_http_url: &str) -> Result<Vec<CdpTargetInfo>> {
    require_text("CDP debugging HTTP URL", debugging_http_url)?;
    let endpoint = CdpHttpEndpoint::parse(debugging_http_url)?;
    let body = endpoint.get("/json/list")?;
    parse_cdp_targets(&body)
}

pub fn create_cdp_page_target(
    debugging_http_url: &str,
    initial_url: &str,
) -> Result<CdpConnectionConfig> {
    require_text("CDP debugging HTTP URL", debugging_http_url)?;
    require_text("CDP target initial URL", initial_url)?;
    let endpoint = CdpHttpEndpoint::parse(debugging_http_url)?;
    let body = endpoint.put(&format!("/json/new?{initial_url}"))?;
    let target: CdpTargetInfo =
        serde_json::from_str(&body).context("failed to parse created CDP target")?;
    CdpConnectionConfig::new(target.web_socket_debugger_url)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CdpHttpEndpoint {
    host: IpAddr,
    port: u16,
    timeout: Duration,
}

impl CdpHttpEndpoint {
    fn parse(input: &str) -> Result<Self> {
        let without_scheme = input
            .trim()
            .strip_prefix("http://")
            .ok_or_else(|| anyhow!("CDP debugging URL must start with http://"))?
            .trim_end_matches('/');
        let (host, port) = parse_host_port("CDP debugging URL", without_scheme)?;
        Ok(Self {
            host,
            port,
            timeout: default_cdp_io_timeout(),
        })
    }

    fn get(&self, path: &str) -> Result<String> {
        self.request("GET", path)
    }

    fn put(&self, path: &str) -> Result<String> {
        self.request("PUT", path)
    }

    fn request(&self, method: &str, path: &str) -> Result<String> {
        let mut stream =
            connect_with_timeout(self.host, self.port, self.timeout).with_context(|| {
                format!(
                    "failed to connect to CDP HTTP endpoint {}:{}",
                    self.host, self.port
                )
            })?;
        set_tcp_stream_timeouts(&stream, self.timeout)?;
        write!(
            stream,
            "{method} {path} HTTP/1.1\r\nHost: {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            format_host_authority(self.host, self.port)
        )
        .context("failed to write CDP HTTP request")?;

        let mut response_bytes = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    response_bytes.extend_from_slice(&buffer[..read]);
                    if http_response_has_complete_body(&response_bytes) {
                        break;
                    }
                }
                Err(error)
                    if error.kind() == ErrorKind::WouldBlock && !response_bytes.is_empty() =>
                {
                    break;
                }
                Err(error) if error.kind() == ErrorKind::TimedOut && !response_bytes.is_empty() => {
                    break;
                }
                Err(error) => return Err(error).context("failed to read CDP HTTP response"),
            }
        }
        let response =
            String::from_utf8(response_bytes).context("CDP HTTP response was not UTF-8")?;
        let (headers, body) = response
            .split_once("\r\n\r\n")
            .ok_or_else(|| anyhow!("invalid CDP HTTP response"))?;
        if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
            return Err(anyhow!("CDP HTTP request failed: {headers}"));
        }
        Ok(body.to_string())
    }
}

fn http_response_has_complete_body(response_bytes: &[u8]) -> bool {
    let Some(header_end) = response_bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
    else {
        return false;
    };
    let Ok(headers) = std::str::from_utf8(&response_bytes[..header_end]) else {
        return false;
    };
    let Some(content_length) = headers.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.eq_ignore_ascii_case("content-length") {
            value.trim().parse::<usize>().ok()
        } else {
            None
        }
    }) else {
        return false;
    };
    response_bytes.len().saturating_sub(header_end + 4) >= content_length
}

pub fn select_page_target(
    targets: &[CdpTargetInfo],
    selection: &CdpTargetSelection,
) -> Result<CdpConnectionConfig> {
    let target = targets
        .iter()
        .find(|target| {
            let id_matches = selection
                .target_id
                .as_ref()
                .is_none_or(|target_id| target.id == *target_id);
            id_matches && target.target_type == "page" && !target.web_socket_debugger_url.is_empty()
        })
        .ok_or_else(|| {
            anyhow!("no matching page CDP target with a WebSocket debugger URL found")
        })?;
    CdpConnectionConfig::new(target.web_socket_debugger_url.clone())
}

pub fn first_page_target(targets: &[CdpTargetInfo]) -> Result<CdpConnectionConfig> {
    select_page_target(targets, &CdpTargetSelection::first_page())
}

fn default_cdp_io_timeout() -> Duration {
    Duration::from_secs(10)
}

fn set_tcp_stream_timeouts(stream: &std::net::TcpStream, timeout: Duration) -> Result<()> {
    stream
        .set_read_timeout(Some(timeout))
        .context("failed to set CDP read timeout")?;
    stream
        .set_write_timeout(Some(timeout))
        .context("failed to set CDP write timeout")
}

fn format_host_authority(host: IpAddr, port: u16) -> String {
    match host {
        IpAddr::V4(addr) => format!("{addr}:{port}"),
        IpAddr::V6(addr) => format!("[{addr}]:{port}"),
    }
}

fn connect_with_timeout(host: IpAddr, port: u16, timeout: Duration) -> Result<std::net::TcpStream> {
    let addr = SocketAddr::new(host, port);
    std::net::TcpStream::connect_timeout(&addr, timeout)
        .with_context(|| format!("failed to connect to {addr} within {timeout:?}"))
}

fn parse_host_port(label: &str, authority: &str) -> Result<(IpAddr, u16)> {
    if let Some(rest) = authority.strip_prefix('[') {
        let (host, port_part) = rest
            .split_once(']')
            .ok_or_else(|| anyhow!("{label} has an unterminated IPv6 host"))?;
        let port = port_part
            .strip_prefix(':')
            .ok_or_else(|| anyhow!("{label} must include host:port"))?;
        return Ok((
            parse_loopback_ip(label, host)?,
            port.parse::<u16>()
                .with_context(|| format!("invalid {label} port: {port}"))?,
        ));
    }

    let (host, port) = authority
        .rsplit_once(':')
        .ok_or_else(|| anyhow!("{label} must include host:port"))?;
    if host.contains(':') {
        return Err(anyhow!("{label} IPv6 hosts must be bracketed"));
    }
    Ok((
        parse_loopback_ip(label, host)?,
        port.parse::<u16>()
            .with_context(|| format!("invalid {label} port: {port}"))?,
    ))
}

fn parse_loopback_ip(field: &str, value: &str) -> Result<IpAddr> {
    require_text(field, value)?;
    let ip = value
        .parse::<IpAddr>()
        .with_context(|| format!("{field} must be a numeric loopback IP address"))?;
    if !ip.is_loopback() {
        return Err(anyhow!("{field} must be a loopback IP address"));
    }
    Ok(ip)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CdpWebSocketEndpoint {
    host: IpAddr,
    port: u16,
    _path: String,
}

impl CdpWebSocketEndpoint {
    fn parse(input: &str) -> Result<Self> {
        let without_scheme = input
            .trim()
            .strip_prefix("ws://")
            .ok_or_else(|| anyhow!("CDP WebSocket URL must start with ws://"))?;
        let (authority, path) = without_scheme
            .split_once('/')
            .unwrap_or((without_scheme, ""));
        let (host, port) = parse_host_port("CDP WebSocket URL", authority)?;
        Ok(Self {
            host,
            port,
            _path: format!("/{path}"),
        })
    }

    fn connect(&self, timeout: Duration) -> Result<std::net::TcpStream> {
        let stream = connect_with_timeout(self.host, self.port, timeout)?;
        set_tcp_stream_timeouts(&stream, timeout)?;
        Ok(stream)
    }
}

fn parse_cdp_targets(input: &str) -> Result<Vec<CdpTargetInfo>> {
    serde_json::from_str(input).context("failed to parse CDP target list")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerId(String);

impl WorkerId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_path_component("worker id", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn evidence_dir(&self) -> String {
        format!("evidence/workers/{}", self.0)
    }

    pub fn screenshot_path(&self, suffix: u128) -> String {
        format!("{}/browser-smoke-{suffix}.png", self.evidence_dir())
    }

    pub fn performance_metrics_path(&self, suffix: u128) -> String {
        format!(
            "{}/browser-smoke-metrics-{suffix}.json",
            self.evidence_dir()
        )
    }

    pub fn console_log_path(&self, suffix: u128) -> String {
        format!("{}/browser-console-{suffix}.json", self.evidence_dir())
    }

    pub fn cdp_trace_summary_path(&self, suffix: u128) -> String {
        format!(
            "{}/browser-cdp-trace-summary-{suffix}.json",
            self.evidence_dir()
        )
    }

    pub fn probe_json_path(&self, probe_name: &str, suffix: u128) -> String {
        format!(
            "{}/browser-probe-{probe_name}-{suffix}.json",
            self.evidence_dir()
        )
    }
}

impl Default for WorkerId {
    fn default() -> Self {
        Self("worker-1".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserSmokeConfig {
    pub run_dir: PathBuf,
    pub url: String,
    pub debugging_http_url: String,
    pub target_selection: CdpTargetSelection,
    pub target_ws_url: Option<String>,
    pub worker_id: WorkerId,
}

impl BrowserSmokeConfig {
    pub fn new(run_dir: impl Into<PathBuf>, url: impl Into<String>) -> Result<Self> {
        let url = url.into();
        require_http_url("browser smoke URL", &url)?;
        Ok(Self {
            run_dir: run_dir.into(),
            url,
            debugging_http_url: "http://127.0.0.1:9222".to_string(),
            target_selection: CdpTargetSelection::first_page(),
            target_ws_url: None,
            worker_id: WorkerId::default(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserSmokeResult {
    pub screenshot_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserSmokePoolConfig {
    pub base: BrowserSmokeConfig,
    pub workers: usize,
}

impl BrowserSmokePoolConfig {
    pub fn new(base: BrowserSmokeConfig, workers: usize) -> Result<Self> {
        if workers == 0 {
            return Err(anyhow!("browser smoke workers must be at least 1"));
        }
        Ok(Self { base, workers })
    }

    pub fn worker_config(&self, index: usize) -> Result<BrowserSmokeConfig> {
        if index >= self.workers {
            return Err(anyhow!("worker index {index} is out of range"));
        }
        let mut config = self.base.clone();
        if self.workers > 1 {
            config.worker_id = WorkerId::new(format!("worker-{}", index + 1))?;
        }
        Ok(config)
    }

    pub fn worker_configs(&self) -> Result<Vec<BrowserSmokeConfig>> {
        (0..self.workers)
            .map(|index| self.worker_config(index))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BrowserSmokeWorkerOutcome {
    pub worker_id: String,
    pub ok: bool,
    pub screenshot_path: Option<PathBuf>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BrowserSmokePoolResult {
    pub workers: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub outcomes: Vec<BrowserSmokeWorkerOutcome>,
}

impl BrowserSmokePoolResult {
    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }
}

pub fn run_browser_smoke_pool(config: &BrowserSmokePoolConfig) -> BrowserSmokePoolResult {
    let worker_configs = match config.worker_configs() {
        Ok(worker_configs) => worker_configs,
        Err(error) => {
            return BrowserSmokePoolResult {
                workers: config.workers,
                succeeded: 0,
                failed: 1,
                outcomes: vec![BrowserSmokeWorkerOutcome {
                    worker_id: "pool".to_string(),
                    ok: false,
                    screenshot_path: None,
                    error: Some(error.to_string()),
                }],
            };
        }
    };

    let mut setup_failures = Vec::new();
    let worker_configs: Vec<_> = worker_configs
        .into_iter()
        .filter_map(|mut worker_config| {
            if config.workers > 1 {
                match create_cdp_page_target(&worker_config.debugging_http_url, "about:blank") {
                    Ok(connection) => {
                        worker_config.target_ws_url = Some(connection.target_ws_url);
                    }
                    Err(error) => {
                        let error_message = error.to_string();
                        let _ = append_ledger_event(
                            &worker_config.run_dir,
                            "browser.worker.failed",
                            "browser-smoke",
                            json!({
                                "worker_id": worker_config.worker_id.as_str(),
                                "error": error_message,
                                "phase": "target_setup"
                            }),
                        );
                        setup_failures.push(BrowserSmokeWorkerOutcome {
                            worker_id: worker_config.worker_id.as_str().to_string(),
                            ok: false,
                            screenshot_path: None,
                            error: Some(error_message),
                        });
                        return None;
                    }
                }
            }
            Some(worker_config)
        })
        .collect();

    let handles: Vec<_> = worker_configs
        .into_iter()
        .map(|worker_config| {
            thread::spawn(move || {
                let worker_id = worker_config.worker_id.as_str().to_string();
                match run_browser_smoke(&worker_config) {
                    Ok(result) => BrowserSmokeWorkerOutcome {
                        worker_id,
                        ok: true,
                        screenshot_path: Some(result.screenshot_path),
                        error: None,
                    },
                    Err(error) => BrowserSmokeWorkerOutcome {
                        worker_id,
                        ok: false,
                        screenshot_path: None,
                        error: Some(error.to_string()),
                    },
                }
            })
        })
        .collect();

    let mut outcomes = setup_failures;
    outcomes.reserve(handles.len());
    for handle in handles {
        match handle.join() {
            Ok(outcome) => outcomes.push(outcome),
            Err(_) => outcomes.push(BrowserSmokeWorkerOutcome {
                worker_id: "unknown".to_string(),
                ok: false,
                screenshot_path: None,
                error: Some("browser smoke worker panicked".to_string()),
            }),
        }
    }
    outcomes.sort_by(|left, right| left.worker_id.cmp(&right.worker_id));
    let succeeded = outcomes.iter().filter(|outcome| outcome.ok).count();
    let failed = outcomes.len().saturating_sub(succeeded);
    BrowserSmokePoolResult {
        workers: config.workers,
        succeeded,
        failed,
        outcomes,
    }
}

pub fn run_browser_smoke(config: &BrowserSmokeConfig) -> Result<BrowserSmokeResult> {
    append_ledger_event(
        &config.run_dir,
        "browser.worker.started",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "debugging_http_url": config.debugging_http_url
        }),
    )?;

    let result = run_browser_smoke_inner(config);
    match &result {
        Ok(smoke) => {
            append_ledger_event(
                &config.run_dir,
                "browser.worker.completed",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "screenshot_path": smoke.screenshot_path.to_string_lossy()
                }),
            )?;
        }
        Err(error) => {
            let _ = append_ledger_event(
                &config.run_dir,
                "browser.worker.failed",
                "browser-smoke",
                json!({ "worker_id": config.worker_id.as_str(), "error": error.to_string() }),
            );
        }
    }
    result
}

fn capture_runtime_probe<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
) -> Result<()> {
    let available = client.evaluate_json(
        "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.getWorldState === 'function' && typeof window.__OUROFORGE__.getFrameStats === 'function')",
    )?;
    if available != json!(true) {
        append_ledger_event(
            &config.run_dir,
            "browser.probe.skipped",
            "browser-smoke",
            json!({
                "worker_id": config.worker_id.as_str(),
                "url": config.url,
                "reason": "window.__OUROFORGE__ probe API not found",
                "optional": true
            }),
        )?;
        return Ok(());
    }

    capture_runtime_probe_value(
        config,
        client,
        "world-state",
        "getWorldState",
        "window.__OUROFORGE__.getWorldState()",
    )?;
    capture_runtime_probe_value(
        config,
        client,
        "frame-stats",
        "getFrameStats",
        "window.__OUROFORGE__.getFrameStats()",
    )?;
    Ok(())
}

fn capture_runtime_probe_value<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
    artifact_name: &str,
    call_name: &str,
    expression: &str,
) -> Result<()> {
    let value = client.evaluate_json(expression)?;
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.probe_json_path(artifact_name, suffix);
    fs::create_dir_all(config.run_dir.join(config.worker_id.evidence_dir())).with_context(
        || {
            format!(
                "failed to create worker evidence directory {}",
                config
                    .run_dir
                    .join(config.worker_id.evidence_dir())
                    .display()
            )
        },
    )?;
    write_json(&config.run_dir.join(&rel_path), &value)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-probe-{artifact_name}-{}-{suffix}",
            config.worker_id.as_str()
        ),
        "application/json",
        &rel_path,
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "probe_call": call_name
        }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.probe.captured",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "probe_call": call_name,
            "path": rel_path
        }),
    )?;
    Ok(())
}

const CONSOLE_CAPTURE_SCRIPT: &str = r#"
(() => {
  if (window.__OUROFORGE_CONSOLE_INSTALLED__) return;
  window.__OUROFORGE_CONSOLE_INSTALLED__ = true;
  window.__OUROFORGE_CONSOLE__ = [];
  const levels = ['debug', 'info', 'log', 'warn', 'error'];
  for (const level of levels) {
    const original = console[level] && console[level].bind(console);
    console[level] = (...args) => {
      try {
        window.__OUROFORGE_CONSOLE__.push({
          level,
          text: args.map((arg) => {
            if (typeof arg === 'string') return arg;
            try { return JSON.stringify(arg); } catch (_) { return String(arg); }
          }).join(' '),
          argCount: args.length,
          timestampMs: Math.round(performance.now())
        });
        if (window.__OUROFORGE_CONSOLE__.length > 100) window.__OUROFORGE_CONSOLE__.shift();
      } catch (_) {}
      if (original) original(...args);
    };
  }
})();
"#;

fn install_console_capture<T: CdpTransport>(client: &mut CdpClient<T>) -> Result<()> {
    client.add_script_to_evaluate_on_new_document(CONSOLE_CAPTURE_SCRIPT)
}

fn capture_console_log<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
) -> Result<Option<String>> {
    let logs = client.evaluate_json("window.__OUROFORGE_CONSOLE__ || []")?;
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.console_log_path(suffix);
    fs::create_dir_all(config.run_dir.join(config.worker_id.evidence_dir())).with_context(
        || {
            format!(
                "failed to create worker evidence directory {}",
                config
                    .run_dir
                    .join(config.worker_id.evidence_dir())
                    .display()
            )
        },
    )?;
    write_json(&config.run_dir.join(&rel_path), &logs)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("browser-console-{}-{suffix}", config.worker_id.as_str()),
        "application/json",
        &rel_path,
        json!({
            "artifact": "console_log",
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "bounded": true,
            "limit": 100
        }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.console",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "path": rel_path,
            "bounded": true,
            "limit": 100
        }),
    )?;
    Ok(Some(rel_path))
}

fn count_cdp_metrics(metrics: &serde_json::Value) -> usize {
    metrics
        .get("metrics")
        .or_else(|| metrics.get("Metrics"))
        .and_then(|value| value.as_array())
        .map_or(0, Vec::len)
}

fn write_worker_cdp_trace_summary(
    config: &BrowserSmokeConfig,
    navigation: &CdpNavigateResult,
    performance_metric_count: Option<usize>,
) -> Result<String> {
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.cdp_trace_summary_path(suffix);
    let mut events = vec![json!({
        "name": "Page.navigate",
        "frameIdPresent": navigation.frame_id.is_some(),
        "loaderIdPresent": navigation.loader_id.is_some()
    })];
    if let Some(metric_count) = performance_metric_count {
        events.push(json!({
            "name": "Performance.getMetrics",
            "metricCount": metric_count
        }));
    }
    write_json(
        &config.run_dir.join(&rel_path),
        &json!({
            "bounded": true,
            "limit": 32,
            "source": "cdp-summary",
            "workerId": config.worker_id.as_str(),
            "events": events
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-cdp-trace-summary-{}-{suffix}",
            config.worker_id.as_str()
        ),
        "application/json",
        &rel_path,
        json!({
            "artifact": "cdp_trace_summary",
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "bounded": true,
            "limit": 32
        }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.cdp_trace_summary",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "path": rel_path,
            "bounded": true,
            "limit": 32
        }),
    )?;
    Ok(rel_path)
}

fn run_browser_smoke_inner(config: &BrowserSmokeConfig) -> Result<BrowserSmokeResult> {
    let connection = if let Some(target_ws_url) = &config.target_ws_url {
        CdpConnectionConfig::new(target_ws_url.clone())?
    } else {
        let targets = read_cdp_targets(&config.debugging_http_url)?;
        select_page_target(&targets, &config.target_selection)?
    };
    let transport = WebSocketCdpTransport::connect(&connection)?;
    let mut client = CdpClient::new(transport);

    client.enable_page()?;
    install_console_capture(&mut client)?;
    let _ = client.bring_page_to_front();
    let navigation = client.navigate(&config.url)?;
    append_ledger_event(
        &config.run_dir,
        "browser.navigation.completed",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "frame_id": navigation.frame_id,
            "loader_id": navigation.loader_id
        }),
    )?;

    std::thread::sleep(Duration::from_millis(300));
    capture_console_log(config, &mut client)?;
    capture_runtime_probe(config, &mut client)?;
    let _ = client.bring_page_to_front();
    let screenshot = client.capture_screenshot_png()?;
    let artifact_id_suffix = unix_millis()?;
    let worker_evidence_dir = config.worker_id.evidence_dir();
    fs::create_dir_all(config.run_dir.join(&worker_evidence_dir)).with_context(|| {
        format!(
            "failed to create worker evidence directory {}",
            config.run_dir.join(&worker_evidence_dir).display()
        )
    })?;
    let screenshot_rel_path =
        format!("{worker_evidence_dir}/browser-smoke-{artifact_id_suffix}.png");
    let screenshot_path = config.run_dir.join(&screenshot_rel_path);
    fs::write(&screenshot_path, screenshot)
        .with_context(|| format!("failed to write screenshot {}", screenshot_path.display()))?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-smoke-screenshot-{}-{artifact_id_suffix}",
            config.worker_id.as_str()
        ),
        "image/png",
        &screenshot_rel_path,
        json!({ "worker_id": config.worker_id.as_str(), "url": config.url }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.screenshot",
        "browser-smoke",
        json!({ "worker_id": config.worker_id.as_str(), "path": screenshot_rel_path }),
    )?;

    let mut performance_metric_count = None;
    match client
        .enable_performance()
        .and_then(|_| client.performance_metrics())
    {
        Ok(metrics) => {
            performance_metric_count = Some(count_cdp_metrics(&metrics));
            let metrics_rel_path = config.worker_id.performance_metrics_path(unix_millis()?);
            let metrics_path = config.run_dir.join(&metrics_rel_path);
            write_json(&metrics_path, &metrics)?;
            let _ = add_evidence_artifact(
                &config.run_dir,
                &format!(
                    "browser-smoke-performance-{}-{}",
                    config.worker_id.as_str(),
                    unix_millis()?
                ),
                "application/json",
                &metrics_rel_path,
                json!({
                    "artifact": "performance_metrics",
                    "worker_id": config.worker_id.as_str(),
                    "url": config.url,
                    "optional": true,
                    "bounded": true
                }),
            );
            append_ledger_event(
                &config.run_dir,
                "browser.capture.performance",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "path": metrics_rel_path,
                    "optional": true
                }),
            )?;
        }
        Err(error) => {
            append_ledger_event(
                &config.run_dir,
                "browser.capture.performance.skipped",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "error": error.to_string(),
                    "optional": true
                }),
            )?;
        }
    }

    write_worker_cdp_trace_summary(config, &navigation, performance_metric_count)?;

    Ok(BrowserSmokeResult { screenshot_path })
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvolveSummary {
    pub status: String,
    pub proposals_created: usize,
    pub proposal_ids: Vec<String>,
    pub classification_ids: Vec<String>,
    pub patch_draft_ids: Vec<String>,
    pub reason: String,
}

pub fn evolve_run(run_dir: impl AsRef<Path>) -> Result<EvolveSummary> {
    let run_dir = run_dir.as_ref();
    append_ledger_event(run_dir, "evolve.started", "evolve-cli", json!({}))?;

    let verdict_input = fs::read_to_string(run_dir.join("verdict.json"))
        .context("failed to read verdict for evolve")?;
    let verdict: serde_json::Value =
        serde_json::from_str(&verdict_input).context("failed to parse verdict for evolve")?;
    let verdict_status = verdict["status"].as_str().unwrap_or("unknown");

    if verdict_status != "failed" {
        let summary = EvolveSummary {
            status: "noop".to_string(),
            proposals_created: 0,
            proposal_ids: Vec::new(),
            classification_ids: Vec::new(),
            patch_draft_ids: Vec::new(),
            reason: format!("verdict status is {verdict_status}; evolve v0 only proposes mutations for failed runs"),
        };
        append_ledger_event(
            run_dir,
            "evolve.completed",
            "evolve-cli",
            json!({ "status": summary.status, "proposals_created": 0 }),
        )?;
        update_journal(run_dir)?;
        return Ok(summary);
    }

    let evidence = read_evidence_index(run_dir)?;
    let failures = verdict["failures"].as_array().cloned().unwrap_or_default();
    let mut proposal_ids = Vec::new();
    let failure = failures
        .first()
        .cloned()
        .unwrap_or_else(|| json!({ "kind": "failed_verdict" }));
    let evidence_id = select_evidence_id_for_failure(&evidence, &failure, &verdict)
        .ok_or_else(|| anyhow!("failed verdict has no evidence artifact to link"))?;
    let proposal = create_mutation_proposal(
        run_dir,
        MutationProposalInput {
            reason: format!(
                "Deterministic evolve v0 placeholder for verdict failure `{}`",
                failure["kind"].as_str().unwrap_or("failed_verdict")
            ),
            evidence_id,
            target: "seeds/platformer.yaml".to_string(),
            path: "scenarios.bootstrap-smoke.assertions".to_string(),
            from: "current evidence-linked failing criteria".to_string(),
            to: "review evidence and adjust the next explicit implementation issue".to_string(),
        },
    )?;
    proposal_ids.push(proposal.id);
    update_journal(run_dir)?;
    let classification_artifact = classify_mutation_failures(run_dir, &proposal_ids)?;
    let classification_ids = classification_artifact
        .classifications
        .iter()
        .map(|classification| classification.id.clone())
        .collect::<Vec<_>>();
    let patch_draft_artifact = generate_patch_drafts(run_dir)?;
    let patch_draft_ids = patch_draft_artifact
        .drafts
        .iter()
        .map(|draft| draft.id.clone())
        .collect::<Vec<_>>();

    let summary = EvolveSummary {
        status: "proposed".to_string(),
        proposals_created: proposal_ids.len(),
        proposal_ids,
        classification_ids,
        patch_draft_ids,
        reason: "failed verdict produced deterministic placeholder mutation proposal".to_string(),
    };
    append_ledger_event(
        run_dir,
        "evolve.completed",
        "evolve-cli",
        json!({
            "status": summary.status,
            "proposals_created": summary.proposals_created,
            "proposal_ids": summary.proposal_ids,
            "classification_ids": summary.classification_ids,
            "patch_draft_ids": summary.patch_draft_ids
        }),
    )?;
    Ok(summary)
}

fn select_evidence_id_for_failure(
    evidence: &EvidenceIndex,
    failure: &serde_json::Value,
    verdict: &serde_json::Value,
) -> Option<String> {
    for key in ["path", "evidence_path"] {
        if let Some(path) = failure.get(key).and_then(|value| value.as_str()) {
            if let Some(artifact) = evidence
                .artifacts
                .iter()
                .find(|artifact| artifact.path == path)
            {
                return Some(artifact.id.clone());
            }
        }
    }
    verdict
        .get("evidence_refs")
        .and_then(|value| value.as_array())
        .and_then(|refs| {
            refs.iter()
                .filter_map(|value| value.as_str())
                .find_map(|path| {
                    evidence
                        .artifacts
                        .iter()
                        .find(|artifact| artifact.path == path)
                        .map(|artifact| artifact.id.clone())
                })
        })
        .or_else(|| {
            evidence
                .artifacts
                .first()
                .map(|artifact| artifact.id.clone())
        })
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MutationProposal {
    pub id: String,
    pub reason: String,
    pub evidence_id: String,
    pub target: String,
    pub path: String,
    pub from: String,
    pub to: String,
    pub confidence: String,
    pub status: String,
    pub verdict_status: String,
    pub created_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct MutationProposalIndex {
    pub proposals: Vec<MutationProposal>,
}

pub struct MutationProposalInput {
    pub reason: String,
    pub evidence_id: String,
    pub target: String,
    pub path: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MutationClassificationCategory {
    ScenarioAssertionFailure,
    RuntimeProbeFailure,
    ConsoleError,
    PerformanceRegression,
    VisualMismatch,
    MissingEvidence,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MutationClassificationState {
    Classified,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MutationClassification {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    pub category: MutationClassificationCategory,
    pub lifecycle_state: MutationClassificationState,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub verdict_ref: String,
    pub journal_ref: String,
    #[serde(default)]
    pub scenario_result_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MutationClassificationArtifact {
    pub schema_version: String,
    pub run_id: String,
    pub classifications: Vec<MutationClassification>,
}

impl MutationClassificationArtifact {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != "1" {
            return Err(anyhow!(
                "mutation classification schema_version must be \"1\""
            ));
        }
        require_text("mutation classification run_id", &self.run_id)?;
        if self.classifications.is_empty() {
            return Err(anyhow!(
                "mutation classification artifact must include at least one classification"
            ));
        }
        let mut ids = std::collections::HashSet::new();
        for classification in &self.classifications {
            classification.validate()?;
            if !ids.insert(classification.id.as_str()) {
                return Err(anyhow!(
                    "duplicate mutation classification id: {}",
                    classification.id
                ));
            }
        }
        Ok(())
    }
}

impl MutationClassification {
    pub fn validate(&self) -> Result<()> {
        require_text("mutation classification id", &self.id)?;
        if let Some(proposal_id) = &self.proposal_id {
            require_text("mutation classification proposal_id", proposal_id)?;
        }
        require_text("mutation classification reason", &self.reason)?;
        require_text("mutation classification verdict_ref", &self.verdict_ref)?;
        require_text("mutation classification journal_ref", &self.journal_ref)?;
        for evidence_ref in &self.evidence_refs {
            require_text("mutation classification evidence_ref", evidence_ref)?;
        }
        for scenario_result_ref in &self.scenario_result_refs {
            require_text(
                "mutation classification scenario_result_ref",
                scenario_result_ref,
            )?;
        }
        if self.category != MutationClassificationCategory::Unknown && self.evidence_refs.is_empty()
        {
            return Err(anyhow!(
                "non-unknown mutation classifications require at least one evidence ref"
            ));
        }
        Ok(())
    }
}

pub fn read_mutation_classification_artifact(
    run_dir: impl AsRef<Path>,
) -> Result<MutationClassificationArtifact> {
    let path = run_dir.as_ref().join("mutation/classifications.json");
    let input = fs::read_to_string(&path)
        .with_context(|| format!("failed to read mutation classifications {}", path.display()))?;
    let artifact: MutationClassificationArtifact =
        serde_json::from_str(&input).with_context(|| {
            format!(
                "failed to parse mutation classifications {}",
                path.display()
            )
        })?;
    artifact.validate()?;
    Ok(artifact)
}

pub fn write_mutation_classification_artifact(
    run_dir: impl AsRef<Path>,
    artifact: &MutationClassificationArtifact,
) -> Result<PathBuf> {
    artifact.validate()?;
    let dir = run_dir.as_ref().join("mutation");
    fs::create_dir_all(&dir).context("failed to create mutation directory")?;
    let path = dir.join("classifications.json");
    write_json_atomic(&path, &json!(artifact))?;
    Ok(path)
}

pub fn classify_mutation_failures(
    run_dir: impl AsRef<Path>,
    proposal_ids: &[String],
) -> Result<MutationClassificationArtifact> {
    let run_dir = run_dir.as_ref();
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let journal = fs::read_to_string(run_dir.join("journal.md"))
        .context("failed to read journal for mutation classification")?;
    let evidence = read_evidence_index(run_dir)?;
    let failures = verdict
        .get("failures")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let failures = if failures.is_empty() {
        vec![json!({
            "kind": "unknown",
            "summary": "failed verdict did not include structured failures"
        })]
    } else {
        failures
    };

    let mut classifications = Vec::new();
    for (index, failure) in failures.iter().enumerate() {
        let evidence_refs = collect_classification_evidence_refs(failure, &verdict);
        let scenario_result_refs = evidence_refs
            .iter()
            .filter(|path| {
                path.contains("scenario-result")
                    || evidence.artifacts.iter().any(|artifact| {
                        artifact.path == **path
                            && artifact
                                .metadata
                                .get("artifact")
                                .and_then(|value| value.as_str())
                                == Some("scenario_result")
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        let (category, reason) =
            classify_failure_category(failure, &verdict, &journal, &evidence_refs);
        let proposal_id = proposal_ids.get(index).cloned().or_else(|| {
            if proposal_ids.len() == 1 {
                proposal_ids.first().cloned()
            } else {
                None
            }
        });
        classifications.push(MutationClassification {
            id: format!("classification-{}", index + 1),
            proposal_id,
            category,
            lifecycle_state: MutationClassificationState::Classified,
            reason,
            evidence_refs,
            verdict_ref: "verdict.json".to_string(),
            journal_ref: "journal.md".to_string(),
            scenario_result_refs,
        });
    }

    let artifact = MutationClassificationArtifact {
        schema_version: "1".to_string(),
        run_id,
        classifications,
    };
    let path = write_mutation_classification_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.classified",
        "evolve-cli",
        json!({
            "path": path
                .strip_prefix(run_dir)
                .ok()
                .and_then(|path| path.to_str())
                .unwrap_or("mutation/classifications.json"),
            "classification_ids": artifact
                .classifications
                .iter()
                .map(|classification| classification.id.clone())
                .collect::<Vec<_>>()
        }),
    )?;
    Ok(artifact)
}

fn collect_classification_evidence_refs(
    failure: &serde_json::Value,
    verdict: &serde_json::Value,
) -> Vec<String> {
    let mut refs = Vec::new();
    for key in ["path", "evidence_path"] {
        if let Some(path) = failure.get(key).and_then(|value| value.as_str()) {
            push_unique_ref(&mut refs, path);
        }
    }
    if let Some(paths) = failure
        .get("evidence_refs")
        .and_then(|value| value.as_array())
    {
        for path in paths.iter().filter_map(|value| value.as_str()) {
            push_unique_ref(&mut refs, path);
        }
    }
    if refs.is_empty() {
        if let Some(paths) = verdict
            .get("evidence_refs")
            .and_then(|value| value.as_array())
        {
            for path in paths.iter().filter_map(|value| value.as_str()) {
                push_unique_ref(&mut refs, path);
            }
        }
    }
    refs
}

fn push_unique_ref(refs: &mut Vec<String>, value: &str) {
    if !value.trim().is_empty() && !refs.iter().any(|existing| existing == value) {
        refs.push(value.to_string());
    }
}

fn classify_failure_category(
    failure: &serde_json::Value,
    verdict: &serde_json::Value,
    journal: &str,
    evidence_refs: &[String],
) -> (MutationClassificationCategory, String) {
    if evidence_refs.is_empty() {
        return (
            MutationClassificationCategory::Unknown,
            "failed verdict did not provide evidence refs for deterministic classification"
                .to_string(),
        );
    }
    let haystack = [
        failure.to_string(),
        verdict
            .get("summary")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        journal.to_string(),
        evidence_refs.join(" "),
    ]
    .join(" ")
    .to_ascii_lowercase();

    if haystack.contains("visual") || haystack.contains("screenshot") {
        (
            MutationClassificationCategory::VisualMismatch,
            "failure references visual or screenshot evidence".to_string(),
        )
    } else if haystack.contains("performance") || haystack.contains("metric") {
        (
            MutationClassificationCategory::PerformanceRegression,
            "failure references performance evidence".to_string(),
        )
    } else if haystack.contains("console") {
        (
            MutationClassificationCategory::ConsoleError,
            "failure references console evidence".to_string(),
        )
    } else if haystack.contains("runtime") || haystack.contains("probe") {
        (
            MutationClassificationCategory::RuntimeProbeFailure,
            "failure references runtime probe evidence".to_string(),
        )
    } else if haystack.contains("missing") && haystack.contains("evidence") {
        (
            MutationClassificationCategory::MissingEvidence,
            "failure reports missing evidence".to_string(),
        )
    } else if haystack.contains("assertion") || haystack.contains("scenario") {
        (
            MutationClassificationCategory::ScenarioAssertionFailure,
            "failure references scenario assertion evidence".to_string(),
        )
    } else {
        (
            MutationClassificationCategory::Unknown,
            "failure evidence did not match a bounded classification category".to_string(),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatchDraftState {
    Drafted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchDraft {
    pub id: String,
    pub proposal_id: String,
    pub classification_id: String,
    pub lifecycle_state: PatchDraftState,
    pub target_path: String,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub draft_text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchDraftArtifact {
    pub schema_version: String,
    pub run_id: String,
    pub drafts: Vec<PatchDraft>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MutationReviewState {
    PendingReview,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MutationReviewDecision {
    pub id: String,
    pub patch_draft_id: String,
    pub state: MutationReviewState,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub reviewer: String,
    pub decided_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MutationReviewArtifact {
    pub schema_version: String,
    pub run_id: String,
    pub decisions: Vec<MutationReviewDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationReviewDecisionInput {
    pub patch_draft_id: String,
    pub state: MutationReviewState,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub reviewer: String,
}

impl PatchDraftArtifact {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != "1" {
            return Err(anyhow!("patch draft schema_version must be \"1\""));
        }
        require_text("patch draft run_id", &self.run_id)?;
        if self.drafts.is_empty() {
            return Err(anyhow!(
                "patch draft artifact must include at least one draft"
            ));
        }
        let mut ids = std::collections::HashSet::new();
        for draft in &self.drafts {
            draft.validate()?;
            if !ids.insert(draft.id.as_str()) {
                return Err(anyhow!("duplicate patch draft id: {}", draft.id));
            }
        }
        Ok(())
    }
}

impl PatchDraft {
    pub fn validate(&self) -> Result<()> {
        require_text("patch draft id", &self.id)?;
        require_text("patch draft proposal_id", &self.proposal_id)?;
        require_text("patch draft classification_id", &self.classification_id)?;
        validate_patch_draft_target_path(&self.target_path)?;
        require_text("patch draft rationale", &self.rationale)?;
        if self.evidence_refs.is_empty() {
            return Err(anyhow!("patch draft requires at least one evidence ref"));
        }
        for evidence_ref in &self.evidence_refs {
            require_text("patch draft evidence_ref", evidence_ref)?;
        }
        require_text("patch draft draft_text", &self.draft_text)?;
        Ok(())
    }
}

impl MutationReviewArtifact {
    pub fn validate(&self, run_dir: impl AsRef<Path>) -> Result<()> {
        if self.schema_version != "1" {
            return Err(anyhow!("mutation review schema_version must be \"1\""));
        }
        require_text("mutation review run_id", &self.run_id)?;
        let drafts = read_patch_draft_artifact(run_dir)?;
        let draft_ids = drafts
            .drafts
            .iter()
            .map(|draft| draft.id.as_str())
            .collect::<std::collections::HashSet<_>>();
        let mut decision_ids = std::collections::HashSet::new();
        for decision in &self.decisions {
            decision.validate(&draft_ids)?;
            if !decision_ids.insert(decision.id.as_str()) {
                return Err(anyhow!(
                    "duplicate mutation review decision id: {}",
                    decision.id
                ));
            }
        }
        Ok(())
    }
}

impl MutationReviewDecision {
    fn validate(&self, draft_ids: &std::collections::HashSet<&str>) -> Result<()> {
        require_text("mutation review decision id", &self.id)?;
        validate_path_component("mutation review decision id", &self.id)?;
        require_text("mutation review patch_draft_id", &self.patch_draft_id)?;
        if !draft_ids.contains(self.patch_draft_id.as_str()) {
            return Err(anyhow!(
                "mutation review patch draft id not found: {}",
                self.patch_draft_id
            ));
        }
        match self.state {
            MutationReviewState::Accepted | MutationReviewState::Rejected => {
                require_text("mutation review reason", &self.reason)?;
                if self.evidence_refs.is_empty() {
                    return Err(anyhow!(
                        "mutation review accept/reject requires evidence or comparison ref"
                    ));
                }
            }
            MutationReviewState::PendingReview => {}
        }
        for evidence_ref in &self.evidence_refs {
            validate_mutation_review_ref(evidence_ref)?;
        }
        require_text("mutation review reviewer", &self.reviewer)?;
        Ok(())
    }
}

pub fn read_mutation_review_artifact(run_dir: impl AsRef<Path>) -> Result<MutationReviewArtifact> {
    let run_dir = run_dir.as_ref();
    let path = run_dir.join("mutation/review-decisions.json");
    if !path.is_file() {
        let run = read_json_value(run_dir.join("run.json"))?;
        let run_id = json_string(&run, "id").unwrap_or_else(|| {
            run_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown-run")
                .to_string()
        });
        return Ok(MutationReviewArtifact {
            schema_version: "1".to_string(),
            run_id,
            decisions: Vec::new(),
        });
    }
    let input = fs::read_to_string(&path)
        .with_context(|| format!("failed to read mutation review {}", path.display()))?;
    let artifact: MutationReviewArtifact = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse mutation review {}", path.display()))?;
    artifact.validate(run_dir)?;
    Ok(artifact)
}

pub fn write_mutation_review_artifact(
    run_dir: impl AsRef<Path>,
    artifact: &MutationReviewArtifact,
) -> Result<PathBuf> {
    let run_dir = run_dir.as_ref();
    artifact.validate(run_dir)?;
    let dir = run_dir.join("mutation");
    fs::create_dir_all(&dir).context("failed to create mutation directory")?;
    let path = dir.join("review-decisions.json");
    write_json_atomic(&path, &json!(artifact))?;
    Ok(path)
}

pub fn append_mutation_review_decision(
    run_dir: impl AsRef<Path>,
    input: MutationReviewDecisionInput,
) -> Result<MutationReviewDecision> {
    let run_dir = run_dir.as_ref();
    let mut artifact = read_mutation_review_artifact(run_dir)?;
    let next_index = artifact.decisions.len() + 1;
    let decision = MutationReviewDecision {
        id: format!("review-decision-{next_index}"),
        patch_draft_id: input.patch_draft_id,
        state: input.state,
        reason: input.reason,
        evidence_refs: input.evidence_refs,
        reviewer: input.reviewer,
        decided_at_unix_ms: unix_millis()?,
    };
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft_ids = drafts
        .drafts
        .iter()
        .map(|draft| draft.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    decision.validate(&draft_ids)?;
    artifact.decisions.push(decision.clone());
    write_mutation_review_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.review_decision",
        "mutation-review",
        json!({
            "decision_id": decision.id,
            "patch_draft_id": decision.patch_draft_id,
            "state": decision.state,
            "evidence_refs": decision.evidence_refs,
            "reviewer": decision.reviewer,
        }),
    )?;
    Ok(decision)
}

pub fn append_mutation_review_decision_from_path(
    run_or_draft_path: impl AsRef<Path>,
    state: MutationReviewState,
    reason: String,
    evidence_refs: Vec<String>,
    reviewer: String,
) -> Result<MutationReviewDecision> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("mutation review requires at least one patch draft"))?
        .id
        .clone();
    let evidence_refs = if evidence_refs.is_empty() {
        default_mutation_review_evidence_refs(&run_dir, &patch_draft_id)?
    } else {
        evidence_refs
    };
    append_mutation_review_decision(
        run_dir,
        MutationReviewDecisionInput {
            patch_draft_id,
            state,
            reason,
            evidence_refs,
            reviewer,
        },
    )
}

fn default_mutation_review_evidence_refs(
    run_dir: &Path,
    patch_draft_id: &str,
) -> Result<Vec<String>> {
    let comparison = run_dir.join("mutation/rerun-orchestration.json");
    if comparison.is_file() {
        return Ok(vec!["mutation/rerun-orchestration.json".to_string()]);
    }
    let sandbox_result = format!("sandbox/{patch_draft_id}/evidence/result.json");
    if run_dir.join(&sandbox_result).is_file() {
        return Ok(vec![sandbox_result]);
    }
    Err(anyhow!(
        "mutation review requires evidence or comparison ref; run evolve compare first or pass --evidence"
    ))
}

fn validate_mutation_review_ref(reference: &str) -> Result<()> {
    require_text("mutation review evidence ref", reference)?;
    let path = Path::new(reference);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "mutation review evidence ref must be run-relative and must not escape the run"
        ));
    }
    if !(reference.starts_with("evidence/")
        || reference.starts_with("mutation/")
        || reference.starts_with("sandbox/"))
    {
        return Err(anyhow!(
            "mutation review evidence ref must point to evidence/, mutation/, or sandbox/"
        ));
    }
    Ok(())
}

fn validate_patch_draft_target_path(target_path: &str) -> Result<()> {
    require_text("patch draft target_path", target_path)?;
    let path = Path::new(target_path);
    if path.is_absolute() {
        return Err(anyhow!("patch draft target_path must be relative"));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(anyhow!(
            "patch draft target_path must not escape the repository"
        ));
    }
    Ok(())
}

pub fn read_patch_draft_artifact(run_dir: impl AsRef<Path>) -> Result<PatchDraftArtifact> {
    let path = run_dir.as_ref().join("mutation/patch-drafts.json");
    let input = fs::read_to_string(&path)
        .with_context(|| format!("failed to read patch drafts {}", path.display()))?;
    let artifact: PatchDraftArtifact = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse patch drafts {}", path.display()))?;
    artifact.validate()?;
    Ok(artifact)
}

pub fn write_patch_draft_artifact(
    run_dir: impl AsRef<Path>,
    artifact: &PatchDraftArtifact,
) -> Result<PathBuf> {
    artifact.validate()?;
    let dir = run_dir.as_ref().join("mutation");
    fs::create_dir_all(&dir).context("failed to create mutation directory")?;
    let path = dir.join("patch-drafts.json");
    write_json_atomic(&path, &json!(artifact))?;
    Ok(path)
}

pub fn generate_patch_drafts(run_dir: impl AsRef<Path>) -> Result<PatchDraftArtifact> {
    let run_dir = run_dir.as_ref();
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let proposals = read_mutation_proposals(run_dir)?.proposals;
    if proposals.is_empty() {
        return Err(anyhow!(
            "patch draft generation requires a mutation proposal"
        ));
    }
    let classifications = read_mutation_classification_artifact(run_dir)?;
    let mut drafts = Vec::new();
    for (index, proposal) in proposals.iter().enumerate() {
        validate_patch_draft_target_path(&proposal.target)
            .with_context(|| format!("unsupported mutation proposal target for {}", proposal.id))?;
        let classification = classifications
            .classifications
            .iter()
            .find(|classification| {
                classification.proposal_id.as_deref() == Some(proposal.id.as_str())
            })
            .or_else(|| classifications.classifications.get(index))
            .ok_or_else(|| {
                anyhow!(
                    "patch draft generation requires classification for proposal {}",
                    proposal.id
                )
            })?;
        let mut evidence_refs = classification.evidence_refs.clone();
        push_unique_ref(&mut evidence_refs, &proposal.evidence_id);
        let draft = PatchDraft {
            id: format!("patch-draft-{}", index + 1),
            proposal_id: proposal.id.clone(),
            classification_id: classification.id.clone(),
            lifecycle_state: PatchDraftState::Drafted,
            target_path: proposal.target.clone(),
            rationale: format!(
                "Draft derived from proposal {} and classification {} ({:?}).",
                proposal.id, classification.id, classification.category
            ),
            evidence_refs,
            draft_text: render_patch_draft_text(proposal, classification),
        };
        draft.validate()?;
        drafts.push(draft);
    }

    let artifact = PatchDraftArtifact {
        schema_version: "1".to_string(),
        run_id,
        drafts,
    };
    let path = write_patch_draft_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.drafted",
        "evolve-cli",
        json!({
            "path": path
                .strip_prefix(run_dir)
                .ok()
                .and_then(|path| path.to_str())
                .unwrap_or("mutation/patch-drafts.json"),
            "patch_draft_ids": artifact
                .drafts
                .iter()
                .map(|draft| draft.id.clone())
                .collect::<Vec<_>>()
        }),
    )?;
    Ok(artifact)
}

fn render_patch_draft_text(
    proposal: &MutationProposal,
    classification: &MutationClassification,
) -> String {
    let evidence_refs = classification.evidence_refs.join(", ");
    format!(
        "\
# Ouroforge patch draft v1
# This is an inspectable draft artifact only. It has not been applied.
proposal_id: {proposal_id}
classification_id: {classification_id}
category: {category:?}
target: {target}
path: {path}
evidence_refs: {evidence_refs}
reason: {reason}

suggested_change:
  from: {from}
  to: {to}
",
        proposal_id = proposal.id,
        classification_id = classification.id,
        category = classification.category,
        target = proposal.target,
        path = proposal.path,
        evidence_refs = evidence_refs,
        reason = proposal.reason,
        from = proposal.from,
        to = proposal.to
    )
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatchSandboxState {
    Planned,
    Applied,
    Verified,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchSandboxLayout {
    pub sandbox_id: String,
    pub patch_draft_id: String,
    pub lifecycle_state: PatchSandboxState,
    pub sandbox_root: String,
    pub worktree_path: String,
    pub evidence_path: String,
    pub plan_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchSandboxPlan {
    pub schema_version: String,
    pub run_id: String,
    pub layout: PatchSandboxLayout,
    pub verification_commands: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchSandboxVerificationOutput {
    pub command: String,
    pub status: i32,
    pub stdout_path: String,
    pub stderr_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchSandboxApplicationResult {
    pub schema_version: String,
    pub run_id: String,
    pub sandbox_id: String,
    pub patch_draft_id: String,
    pub lifecycle_state: PatchSandboxState,
    pub sandbox_root: String,
    pub worktree_path: String,
    pub evidence_path: String,
    pub applied_target_path: String,
    pub applied_draft_path: String,
    pub result_path: String,
    pub verification: Vec<PatchSandboxVerificationOutput>,
    pub primary_git_status_before: String,
    pub primary_git_status_after: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveRunReference {
    pub run_id: String,
    pub run_path: String,
    pub verdict_path: String,
    pub evidence_index_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveSandboxRunReference {
    pub run_id: String,
    pub sandbox_root: String,
    pub sandbox_result_path: String,
    pub applied_target_path: String,
    pub verification_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveRerunOrchestration {
    pub schema_version: String,
    pub source_run_id: String,
    pub patch_draft_id: String,
    pub mutation_proposal_id: String,
    pub mutation_classification_id: String,
    pub before: EvolveRunReference,
    pub after: EvolveSandboxRunReference,
    pub comparison_artifact_path: Option<String>,
    pub final_classification: Option<String>,
    pub evolve_evidence_path: String,
    pub primary_git_status_before: String,
    pub primary_git_status_after: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveDemoLifecycleSummary {
    pub schema_version: String,
    pub run_id: String,
    pub status: String,
    pub classification_artifact_path: String,
    pub classification_ids: Vec<String>,
    pub patch_draft_artifact_path: String,
    pub patch_draft_ids: Vec<String>,
    pub sandbox_result_path: String,
    pub rerun_evidence_path: String,
    pub comparison_artifact_path: String,
    pub manual_review_state: MutationReviewState,
    pub review_decision_artifact_path: Option<String>,
    pub lifecycle_summary_path: String,
    pub primary_git_status_before: String,
    pub primary_git_status_after: String,
    pub omitted_features: Vec<String>,
}

impl PatchSandboxPlan {
    pub fn validate(&self, run_dir: impl AsRef<Path>) -> Result<()> {
        if self.schema_version != "1" {
            return Err(anyhow!("patch sandbox schema_version must be \"1\""));
        }
        require_text("patch sandbox run_id", &self.run_id)?;
        self.layout.validate(run_dir)?;
        if self.verification_commands.is_empty() {
            return Err(anyhow!(
                "patch sandbox plan requires at least one verification command"
            ));
        }
        for command in &self.verification_commands {
            require_text("patch sandbox verification command", command)?;
        }
        Ok(())
    }
}

impl PatchSandboxLayout {
    pub fn validate(&self, run_dir: impl AsRef<Path>) -> Result<()> {
        require_text("patch sandbox sandbox_id", &self.sandbox_id)?;
        validate_sandbox_id(&self.sandbox_id)?;
        require_text("patch sandbox patch_draft_id", &self.patch_draft_id)?;
        let expected_root = format!("sandbox/{}", self.sandbox_id);
        let expected_worktree = format!("{expected_root}/worktree");
        let expected_evidence = format!("{expected_root}/evidence");
        let expected_plan = format!("{expected_root}/plan.json");
        if self.sandbox_root != expected_root {
            return Err(anyhow!("patch sandbox root does not match sandbox id"));
        }
        if self.worktree_path != expected_worktree {
            return Err(anyhow!("patch sandbox worktree path is invalid"));
        }
        if self.evidence_path != expected_evidence {
            return Err(anyhow!("patch sandbox evidence path is invalid"));
        }
        if self.plan_path != expected_plan {
            return Err(anyhow!("patch sandbox plan path is invalid"));
        }
        for relative in [
            &self.sandbox_root,
            &self.worktree_path,
            &self.evidence_path,
            &self.plan_path,
        ] {
            validate_run_relative_sandbox_path(relative)?;
            ensure_path_inside(run_dir.as_ref(), &run_dir.as_ref().join(relative))?;
        }
        Ok(())
    }
}

fn validate_sandbox_id(sandbox_id: &str) -> Result<()> {
    require_text("patch sandbox sandbox_id", sandbox_id)?;
    if !sandbox_id
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
    {
        return Err(anyhow!(
            "patch sandbox sandbox_id may contain only ASCII letters, digits, and '-'"
        ));
    }
    Ok(())
}

fn validate_run_relative_sandbox_path(relative_path: &str) -> Result<()> {
    require_text("patch sandbox path", relative_path)?;
    let path = Path::new(relative_path);
    if path.is_absolute() {
        return Err(anyhow!("patch sandbox path must be relative"));
    }
    let mut components = path.components();
    if components.next() != Some(Component::Normal(std::ffi::OsStr::new("sandbox"))) {
        return Err(anyhow!("patch sandbox path must stay under sandbox/"));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(anyhow!("patch sandbox path must not escape the run"));
    }
    Ok(())
}

fn ensure_path_inside(root: &Path, candidate: &Path) -> Result<()> {
    let root = root
        .canonicalize()
        .with_context(|| format!("failed to canonicalize root {}", root.display()))?;
    let parent = candidate.parent().unwrap_or(candidate);
    let existing_parent = first_existing_ancestor(parent)
        .ok_or_else(|| anyhow!("no existing parent for {}", candidate.display()))?;
    let canonical_parent = existing_parent
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", existing_parent.display()))?;
    if !canonical_parent.starts_with(&root) {
        return Err(anyhow!("path is outside sandbox root"));
    }
    Ok(())
}

fn first_existing_ancestor(path: &Path) -> Option<&Path> {
    let mut current = Some(path);
    while let Some(candidate) = current {
        if candidate.exists() {
            return Some(candidate);
        }
        current = candidate.parent();
    }
    None
}

pub fn plan_patch_sandbox(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
) -> Result<PatchSandboxPlan> {
    let run_dir = run_dir.as_ref();
    require_text("patch sandbox patch_draft_id", patch_draft_id)?;
    validate_sandbox_id(patch_draft_id)?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let drafts = read_patch_draft_artifact(run_dir)?;
    if !drafts.drafts.iter().any(|draft| draft.id == patch_draft_id) {
        return Err(anyhow!(
            "patch draft id not found for sandbox: {}",
            patch_draft_id
        ));
    }
    let layout = PatchSandboxLayout {
        sandbox_id: patch_draft_id.to_string(),
        patch_draft_id: patch_draft_id.to_string(),
        lifecycle_state: PatchSandboxState::Planned,
        sandbox_root: format!("sandbox/{patch_draft_id}"),
        worktree_path: format!("sandbox/{patch_draft_id}/worktree"),
        evidence_path: format!("sandbox/{patch_draft_id}/evidence"),
        plan_path: format!("sandbox/{patch_draft_id}/plan.json"),
    };
    let plan = PatchSandboxPlan {
        schema_version: "1".to_string(),
        run_id,
        layout,
        verification_commands: vec!["cargo fmt --check".to_string(), "cargo test".to_string()],
    };
    plan.validate(run_dir)?;
    Ok(plan)
}

pub fn create_patch_sandbox_layout(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
) -> Result<PatchSandboxPlan> {
    let run_dir = run_dir.as_ref();
    let plan = plan_patch_sandbox(run_dir, patch_draft_id)?;
    let root = run_dir.join(&plan.layout.sandbox_root);
    let worktree = run_dir.join(&plan.layout.worktree_path);
    let evidence = run_dir.join(&plan.layout.evidence_path);
    ensure_path_inside(run_dir, &root)?;
    fs::create_dir_all(&worktree).context("failed to create patch sandbox worktree")?;
    fs::create_dir_all(&evidence).context("failed to create patch sandbox evidence")?;
    let plan_path = run_dir.join(&plan.layout.plan_path);
    write_json_atomic(&plan_path, &json!(plan))?;
    Ok(plan)
}

pub fn apply_patch_sandbox_from_path(
    run_or_draft_path: impl AsRef<Path>,
) -> Result<PatchSandboxApplicationResult> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("patch sandbox requires at least one patch draft"))?
        .id
        .clone();
    let repo_root = git_repo_root()?;
    apply_patch_sandbox(&run_dir, &patch_draft_id, &repo_root, true)
}

pub fn orchestrate_evolve_rerun_from_path(
    run_or_draft_path: impl AsRef<Path>,
) -> Result<EvolveRerunOrchestration> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("evolve rerun comparison requires at least one patch draft"))?
        .id
        .clone();
    let repo_root = git_repo_root()?;
    orchestrate_evolve_rerun(&run_dir, &patch_draft_id, &repo_root, true)
}

pub fn run_evolve_demo_lifecycle_from_path(
    run_or_demo_path: impl AsRef<Path>,
) -> Result<EvolveDemoLifecycleSummary> {
    let run_dir = run_or_demo_path.as_ref();
    if !run_dir.is_dir() {
        return Err(anyhow!(
            "evolve demo expects a run directory produced by the demo seed"
        ));
    }
    let repo_root = git_repo_root()?;
    run_evolve_demo_lifecycle(run_dir, &repo_root, true)
}

pub fn run_evolve_demo_lifecycle(
    run_dir: impl AsRef<Path>,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<EvolveDemoLifecycleSummary> {
    let run_dir = run_dir.as_ref();
    if !run_dir.join("mutation/patch-drafts.json").is_file() {
        evolve_run(run_dir)?;
    }
    let classifications = read_mutation_classification_artifact(run_dir)?;
    let drafts = read_patch_draft_artifact(run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("evolve demo requires at least one patch draft"))?
        .id
        .clone();
    let rerun = orchestrate_evolve_rerun(run_dir, &patch_draft_id, repo_root, run_verification)?;
    let reviews = read_mutation_review_artifact(run_dir)?;
    let manual_review_state = reviews
        .decisions
        .last()
        .map(|decision| decision.state.clone())
        .unwrap_or(MutationReviewState::PendingReview);
    let lifecycle_summary_path = "mutation/evolve-v1-demo-summary.json".to_string();
    let review_decision_artifact_path = if reviews.decisions.is_empty() {
        None
    } else {
        Some("mutation/review-decisions.json".to_string())
    };
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let summary = EvolveDemoLifecycleSummary {
        schema_version: "1".to_string(),
        run_id,
        status: "lifecycle_evidence_ready".to_string(),
        classification_artifact_path: "mutation/classifications.json".to_string(),
        classification_ids: classifications
            .classifications
            .iter()
            .map(|classification| classification.id.clone())
            .collect(),
        patch_draft_artifact_path: "mutation/patch-drafts.json".to_string(),
        patch_draft_ids: drafts.drafts.iter().map(|draft| draft.id.clone()).collect(),
        sandbox_result_path: rerun.after.sandbox_result_path,
        rerun_evidence_path: rerun.evolve_evidence_path,
        comparison_artifact_path: rerun
            .comparison_artifact_path
            .ok_or_else(|| anyhow!("evolve demo requires comparison artifact"))?,
        manual_review_state,
        review_decision_artifact_path,
        lifecycle_summary_path,
        primary_git_status_before: rerun.primary_git_status_before,
        primary_git_status_after: rerun.primary_git_status_after,
        omitted_features: vec![
            "Manual review remains a separate explicit mutation review command.".to_string(),
            "No patch is applied or merged into the primary working tree.".to_string(),
        ],
    };
    write_json_atomic(
        &run_dir.join(&summary.lifecycle_summary_path),
        &json!(summary),
    )?;
    append_ledger_event(
        run_dir,
        "evolve.demo_lifecycle",
        "evolve-cli",
        json!({
            "lifecycle_summary_path": summary.lifecycle_summary_path,
            "classification_artifact_path": summary.classification_artifact_path,
            "patch_draft_artifact_path": summary.patch_draft_artifact_path,
            "sandbox_result_path": summary.sandbox_result_path,
            "comparison_artifact_path": summary.comparison_artifact_path,
            "manual_review_state": summary.manual_review_state,
        }),
    )?;
    Ok(summary)
}

pub fn orchestrate_evolve_rerun(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<EvolveRerunOrchestration> {
    let run_dir = run_dir.as_ref();
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft = drafts
        .drafts
        .iter()
        .find(|draft| draft.id == patch_draft_id)
        .ok_or_else(|| {
            anyhow!("patch draft id not found for rerun comparison: {patch_draft_id}")
        })?;
    let sandbox = apply_patch_sandbox(run_dir, patch_draft_id, repo_root, run_verification)?;
    let before = evolve_run_reference(run_dir)?;
    let after_run_id = format!("{}--sandbox-{}", before.run_id, patch_draft_id);
    let verification_evidence_refs = sandbox
        .verification
        .iter()
        .flat_map(|output| [output.stdout_path.clone(), output.stderr_path.clone()])
        .collect::<Vec<_>>();
    let after_run_dir = write_sandbox_after_run_reference(run_dir, &sandbox, &after_run_id)?;
    let comparison_path =
        write_run_comparison_artifact(run_dir, &after_run_dir, run_dir.join("mutation"))?;
    let comparison = compare_runs(run_dir, &after_run_dir)?;
    let comparison_artifact_path = run_relative_path(run_dir, &comparison_path)?;
    let final_classification =
        normalize_evolve_comparison_classification(&comparison.classification).to_string();
    let evolve_evidence_path = "mutation/rerun-orchestration.json".to_string();
    let result = EvolveRerunOrchestration {
        schema_version: "1".to_string(),
        source_run_id: before.run_id.clone(),
        patch_draft_id: draft.id.clone(),
        mutation_proposal_id: draft.proposal_id.clone(),
        mutation_classification_id: draft.classification_id.clone(),
        before,
        after: EvolveSandboxRunReference {
            run_id: after_run_id,
            sandbox_root: sandbox.sandbox_root,
            sandbox_result_path: sandbox.result_path,
            applied_target_path: sandbox.applied_target_path,
            verification_evidence_refs,
        },
        comparison_artifact_path: Some(comparison_artifact_path),
        final_classification: Some(final_classification),
        evolve_evidence_path,
        primary_git_status_before: sandbox.primary_git_status_before,
        primary_git_status_after: sandbox.primary_git_status_after,
    };
    let evidence_path = run_dir.join(&result.evolve_evidence_path);
    write_json_atomic(&evidence_path, &json!(result))?;
    append_ledger_event(
        run_dir,
        "mutation.rerun_orchestrated",
        "evolve-cli",
        json!({
            "patch_draft_id": result.patch_draft_id,
            "before_run_id": result.before.run_id,
            "after_run_id": result.after.run_id,
            "evolve_evidence_path": result.evolve_evidence_path,
            "comparison_artifact_path": result.comparison_artifact_path,
            "final_classification": result.final_classification,
        }),
    )?;
    Ok(result)
}

fn write_sandbox_after_run_reference(
    source_run_dir: &Path,
    sandbox: &PatchSandboxApplicationResult,
    after_run_id: &str,
) -> Result<PathBuf> {
    let after_run_dir = source_run_dir.join(&sandbox.sandbox_root).join("after-run");
    let evidence_dir = after_run_dir.join("evidence");
    fs::create_dir_all(&evidence_dir)
        .with_context(|| format!("failed to create {}", evidence_dir.display()))?;
    write_json(
        &after_run_dir.join("run.json"),
        &json!({
            "id": after_run_id,
            "source_run_id": sandbox.run_id,
            "sandbox_result_path": sandbox.result_path,
            "status": "sandbox_verified",
        }),
    )?;
    let verdict_status = if sandbox.lifecycle_state == PatchSandboxState::Verified {
        "passed"
    } else {
        "failed"
    };
    write_json(
        &after_run_dir.join("verdict.json"),
        &json!({
            "status": verdict_status,
            "summary": "Sandbox verification result used as evolve after-run reference.",
            "failures": [],
            "evidence_refs": [
                sandbox.result_path,
                sandbox.applied_draft_path,
            ],
            "metadata": {
                "evaluator": "ouroforge-evolve-rerun-v1",
                "sandbox_id": sandbox.sandbox_id,
                "patch_draft_id": sandbox.patch_draft_id,
            }
        }),
    )?;
    let mut artifacts = vec![
        EvidenceArtifact {
            id: "sandbox-result".to_string(),
            kind: "application/json".to_string(),
            path: "evidence/sandbox-result.json".to_string(),
            metadata: json!({ "artifact": "sandbox_result" }),
            added_at_unix_ms: unix_millis()?,
        },
        EvidenceArtifact {
            id: "applied-draft".to_string(),
            kind: "text/plain".to_string(),
            path: "evidence/applied-draft.txt".to_string(),
            metadata: json!({ "artifact": "applied_patch_draft" }),
            added_at_unix_ms: unix_millis()?,
        },
    ];
    for (index, output) in sandbox.verification.iter().enumerate() {
        artifacts.push(EvidenceArtifact {
            id: format!("sandbox-verification-{}", index + 1),
            kind: "text/plain".to_string(),
            path: format!("evidence/sandbox-verification-{}.txt", index + 1),
            metadata: json!({
                "artifact": "sandbox_verification",
                "command": output.command,
                "status": output.status,
                "stdout_path": output.stdout_path,
                "stderr_path": output.stderr_path,
            }),
            added_at_unix_ms: unix_millis()?,
        });
    }
    write_evidence_index(&after_run_dir, &EvidenceIndex { artifacts })?;
    Ok(after_run_dir)
}

fn normalize_evolve_comparison_classification(classification: &str) -> &'static str {
    match classification {
        "improved" => "improved",
        "regressed" => "regressed",
        "no_change" => "no_change",
        _ => "no_change",
    }
}

pub fn apply_patch_sandbox(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<PatchSandboxApplicationResult> {
    let run_dir = run_dir.as_ref();
    let repo_root = repo_root.as_ref();
    let primary_git_status_before = git_status_short(repo_root)?;
    let plan = create_patch_sandbox_layout(run_dir, patch_draft_id)?;
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft = drafts
        .drafts
        .iter()
        .find(|draft| draft.id == patch_draft_id)
        .ok_or_else(|| anyhow!("patch draft id not found for sandbox: {patch_draft_id}"))?;

    let worktree = run_dir.join(&plan.layout.worktree_path);
    let evidence = run_dir.join(&plan.layout.evidence_path);
    copy_repo_tracked_files(repo_root, &worktree)?;

    let applied_target = worktree.join(&draft.target_path);
    ensure_path_inside(&worktree, &applied_target)?;
    if !applied_target.exists() {
        return Err(anyhow!(
            "patch sandbox target does not exist in sandbox worktree: {}",
            draft.target_path
        ));
    }
    fs::write(&applied_target, &draft.draft_text).with_context(|| {
        format!(
            "failed to apply patch draft {} to sandbox target {}",
            draft.id,
            applied_target.display()
        )
    })?;

    let applied_draft_path = evidence.join("applied-draft.txt");
    fs::write(&applied_draft_path, &draft.draft_text)
        .with_context(|| format!("failed to write {}", applied_draft_path.display()))?;

    let mut verification = Vec::new();
    if run_verification {
        for (index, command) in plan.verification_commands.iter().enumerate() {
            verification.push(run_sandbox_verification_command(
                &worktree,
                &evidence,
                index + 1,
                command,
            )?);
        }
    }

    let primary_git_status_after = git_status_short(repo_root)?;
    let lifecycle_state = if verification.iter().all(|output| output.status == 0) {
        PatchSandboxState::Verified
    } else {
        PatchSandboxState::Failed
    };
    let result = PatchSandboxApplicationResult {
        schema_version: "1".to_string(),
        run_id: plan.run_id,
        sandbox_id: plan.layout.sandbox_id,
        patch_draft_id: draft.id.clone(),
        lifecycle_state,
        sandbox_root: plan.layout.sandbox_root,
        worktree_path: plan.layout.worktree_path,
        evidence_path: plan.layout.evidence_path,
        applied_target_path: run_relative_path(run_dir, &applied_target)?,
        applied_draft_path: run_relative_path(run_dir, &applied_draft_path)?,
        result_path: format!("sandbox/{patch_draft_id}/evidence/result.json"),
        verification,
        primary_git_status_before,
        primary_git_status_after,
    };
    let result_path = run_dir.join(&result.result_path);
    write_json_atomic(&result_path, &json!(result))?;
    if result.primary_git_status_before != result.primary_git_status_after {
        return Err(anyhow!(
            "primary repo git status changed during sandbox application"
        ));
    }
    if result.lifecycle_state == PatchSandboxState::Failed {
        return Err(anyhow!(
            "patch sandbox verification failed; result written to {}",
            result_path.display()
        ));
    }
    append_ledger_event(
        run_dir,
        "mutation.sandboxed",
        "evolve-cli",
        json!({
            "patch_draft_id": result.patch_draft_id,
            "sandbox_root": result.sandbox_root,
            "result_path": result.result_path,
        }),
    )?;
    Ok(result)
}

fn evolve_run_reference(run_dir: &Path) -> Result<EvolveRunReference> {
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = run
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("evolve rerun baseline run.json missing id"))?
        .to_string();
    for relative in ["run.json", "verdict.json", "evidence/index.json"] {
        let path = run_dir.join(relative);
        if !path.is_file() {
            return Err(anyhow!(
                "evolve rerun baseline is missing required artifact {}",
                path.display()
            ));
        }
    }
    Ok(EvolveRunReference {
        run_id,
        run_path: "run.json".to_string(),
        verdict_path: "verdict.json".to_string(),
        evidence_index_path: "evidence/index.json".to_string(),
    })
}

fn resolve_patch_sandbox_run_dir(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        return Ok(path.to_path_buf());
    }
    if path.file_name().and_then(|name| name.to_str()) == Some("patch-drafts.json") {
        return path
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
            .ok_or_else(|| anyhow!("patch draft path must be under <run>/mutation/"));
    }
    Err(anyhow!(
        "evolve sandbox expects a run directory or <run>/mutation/patch-drafts.json"
    ))
}

fn git_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to locate git repository root")?;
    if !output.status.success() {
        return Err(anyhow!(
            "failed to locate git repository root: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(PathBuf::from(String::from_utf8(output.stdout)?.trim()))
}

fn git_status_short(repo_root: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["status", "--short", "--untracked-files=no"])
        .current_dir(repo_root)
        .output()
        .context("failed to inspect primary git status")?;
    if !output.status.success() {
        return Err(anyhow!(
            "failed to inspect primary git status: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn copy_repo_tracked_files(repo_root: &Path, worktree: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(repo_root)
        .output()
        .context("failed to list tracked repository files")?;
    if !output.status.success() {
        return Err(anyhow!(
            "failed to list tracked repository files: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    for raw in output.stdout.split(|byte| *byte == 0) {
        if raw.is_empty() {
            continue;
        }
        let relative = std::str::from_utf8(raw).context("tracked path is not valid UTF-8")?;
        let source = repo_root.join(relative);
        let target = worktree.join(relative);
        ensure_path_inside(worktree, &target)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::copy(&source, &target).with_context(|| {
            format!(
                "failed to copy tracked file {} to sandbox {}",
                source.display(),
                target.display()
            )
        })?;
    }
    Ok(())
}

fn run_sandbox_verification_command(
    worktree: &Path,
    evidence: &Path,
    index: usize,
    command: &str,
) -> Result<PatchSandboxVerificationOutput> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(worktree)
        .output()
        .with_context(|| format!("failed to run sandbox verification command `{command}`"))?;
    let stdout_path = evidence.join(format!("verification-{index}-stdout.txt"));
    let stderr_path = evidence.join(format!("verification-{index}-stderr.txt"));
    fs::write(&stdout_path, &output.stdout)
        .with_context(|| format!("failed to write {}", stdout_path.display()))?;
    fs::write(&stderr_path, &output.stderr)
        .with_context(|| format!("failed to write {}", stderr_path.display()))?;
    Ok(PatchSandboxVerificationOutput {
        command: command.to_string(),
        status: output.status.code().unwrap_or(-1),
        stdout_path: run_relative_path_from_sandbox_worktree(worktree, &stdout_path)?,
        stderr_path: run_relative_path_from_sandbox_worktree(worktree, &stderr_path)?,
    })
}

fn run_relative_path(run_dir: &Path, path: &Path) -> Result<String> {
    path.strip_prefix(run_dir)
        .with_context(|| format!("{} is outside run directory", path.display()))?
        .to_str()
        .map(str::to_string)
        .ok_or_else(|| anyhow!("run-relative path is not valid UTF-8"))
}

fn run_relative_path_from_sandbox_worktree(worktree: &Path, path: &Path) -> Result<String> {
    let run_dir = worktree
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .ok_or_else(|| anyhow!("sandbox worktree is not under a run directory"))?;
    run_relative_path(run_dir, path)
}

pub fn create_mutation_proposal(
    run_dir: impl AsRef<Path>,
    input: MutationProposalInput,
) -> Result<MutationProposal> {
    let run_dir = run_dir.as_ref();
    require_text("mutation reason", &input.reason)?;
    require_text("mutation evidence", &input.evidence_id)?;
    require_text("mutation target", &input.target)?;
    require_text("mutation path", &input.path)?;
    require_text("mutation from", &input.from)?;
    require_text("mutation to", &input.to)?;
    let evidence = read_evidence_index(run_dir)?;
    if !evidence
        .artifacts
        .iter()
        .any(|artifact| artifact.id == input.evidence_id)
    {
        return Err(anyhow!(
            "mutation evidence id not found: {}",
            input.evidence_id
        ));
    }
    let verdict_status = fs::read_to_string(run_dir.join("verdict.json"))
        .ok()
        .and_then(|input| serde_json::from_str::<serde_json::Value>(&input).ok())
        .and_then(|value| {
            value
                .get("status")
                .and_then(|status| status.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "unknown".to_string());
    let mut index = read_mutation_proposals(run_dir)?;
    let created_at_unix_ms = unix_millis()?;
    let proposal = MutationProposal {
        id: format!(
            "mutation-{created_at_unix_ms}-{}",
            index.proposals.len() + 1
        ),
        reason: input.reason,
        evidence_id: input.evidence_id,
        target: input.target,
        path: input.path,
        from: input.from,
        to: input.to,
        confidence: "medium".to_string(),
        status: "proposed".to_string(),
        verdict_status,
        created_at_unix_ms,
    };
    index.proposals.push(proposal.clone());
    write_mutation_proposals(run_dir, &index)?;
    append_ledger_event(
        run_dir,
        "mutation.proposed",
        "mutation-cli",
        json!({
            "proposal_id": proposal.id,
            "evidence_id": proposal.evidence_id,
            "target": proposal.target,
            "path": proposal.path,
            "status": proposal.status
        }),
    )?;
    Ok(proposal)
}

pub fn list_mutation_proposals(run_dir: impl AsRef<Path>) -> Result<Vec<MutationProposal>> {
    Ok(read_mutation_proposals(run_dir)?.proposals)
}

fn read_mutation_proposals(run_dir: impl AsRef<Path>) -> Result<MutationProposalIndex> {
    let path = run_dir.as_ref().join("mutation/proposals.json");
    if !path.exists() {
        return Ok(MutationProposalIndex::default());
    }
    let input = fs::read_to_string(&path)
        .with_context(|| format!("failed to read mutation proposals {}", path.display()))?;
    serde_json::from_str(&input)
        .with_context(|| format!("failed to parse mutation proposals {}", path.display()))
}

fn write_mutation_proposals(
    run_dir: impl AsRef<Path>,
    index: &MutationProposalIndex,
) -> Result<()> {
    let dir = run_dir.as_ref().join("mutation");
    fs::create_dir_all(&dir).context("failed to create mutation directory")?;
    write_json_atomic(&dir.join("proposals.json"), &json!(index))
}

pub fn update_journal(run_dir: impl AsRef<Path>) -> Result<String> {
    let run_dir = run_dir.as_ref();
    let seed = Seed::from_path(run_dir.join("seed.snapshot.yaml"))?;
    let evidence = read_evidence_index(run_dir)?;
    let ledger = read_ledger_events(run_dir)?;
    let verdict_input = fs::read_to_string(run_dir.join("verdict.json"))
        .context("failed to read verdict for journal")?;
    let verdict: serde_json::Value =
        serde_json::from_str(&verdict_input).context("failed to parse verdict for journal")?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let proposals = read_mutation_proposals(run_dir)?.proposals;
    let journal = render_journal(&seed, &evidence, &ledger, &verdict, &proposals, &run);
    fs::write(run_dir.join("journal.md"), &journal).context("failed to write journal")?;
    Ok(journal)
}

pub fn show_journal(run_dir: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(run_dir.as_ref().join("journal.md")).context("failed to read journal")
}

fn render_journal(
    seed: &Seed,
    evidence: &EvidenceIndex,
    ledger: &[serde_json::Value],
    verdict: &serde_json::Value,
    proposals: &[MutationProposal],
    run: &serde_json::Value,
) -> String {
    let mut out = String::new();
    out.push_str("# Ouroforge Run Journal\n\n");
    out.push_str("## Seed Summary\n\n");
    out.push_str(&format!("- Seed: `{}` — {}\n", seed.id, seed.title));
    out.push_str(&format!("- Goal: {}\n", seed.goal));
    out.push_str(&format!("- Target: `{}`\n\n", seed.constraints.target));

    out.push_str("## Expected Criteria\n\n");
    for item in &seed.acceptance {
        out.push_str(&format!("- {}\n", item));
    }
    out.push('\n');

    if let Some(provenance) = run.get("transaction_provenance") {
        out.push_str("## Scene Edit Transaction\n\n");
        if let Some(id) = provenance
            .get("transactionId")
            .and_then(|value| value.as_str())
        {
            out.push_str(&format!("- Transaction: `{}`\n", id));
        }
        if let Some(path) = provenance
            .get("transactionArtifactPath")
            .and_then(|value| value.as_str())
        {
            out.push_str(&format!("- Artifact: `{}`\n", path));
        }
        if let Some(scene_path) = provenance.get("scenePath").and_then(|value| value.as_str()) {
            out.push_str(&format!("- Scene: `{}`\n", scene_path));
        }
        out.push('\n');
    }

    out.push_str("## Executed Scenarios\n\n");
    for scenario in &seed.scenarios {
        let started = ledger.iter().any(|event| {
            event["event"] == "scenario.started"
                && event["payload"]["scenario_id"] == scenario.id.as_str()
        });
        let completed = ledger.iter().any(|event| {
            event["event"] == "scenario.completed"
                && event["payload"]["scenario_id"] == scenario.id.as_str()
        });
        out.push_str(&format!(
            "- `{}`: {} (started: {}, completed: {})\n",
            scenario.id, scenario.description, started, completed
        ));
    }
    out.push('\n');

    out.push_str("## Observations\n\n");
    out.push_str(&format!("- Ledger events recorded: {}\n", ledger.len()));
    out.push_str(&format!(
        "- Evidence artifacts indexed: {}\n\n",
        evidence.artifacts.len()
    ));

    out.push_str("## Evidence\n\n");
    if evidence.artifacts.is_empty() {
        out.push_str("- No evidence artifacts indexed.\n");
    } else {
        for artifact in &evidence.artifacts {
            out.push_str(&format!(
                "- `{}` ({}) → `{}`\n",
                artifact.id, artifact.kind, artifact.path
            ));
        }
    }
    out.push('\n');

    out.push_str("## Verdict Summary\n\n");
    out.push_str(&format!(
        "- Status: `{}`\n",
        verdict["status"].as_str().unwrap_or("unknown")
    ));
    out.push_str(&format!(
        "- Summary: {}\n\n",
        verdict["summary"]
            .as_str()
            .unwrap_or("No summary available.")
    ));

    out.push_str("## Failed Criteria\n\n");
    let failures = verdict["failures"].as_array().cloned().unwrap_or_default();
    if failures.is_empty() {
        out.push_str("- None recorded.\n");
    } else {
        for failure in failures {
            out.push_str(&format!(
                "- `{}`: {}\n",
                failure["kind"].as_str().unwrap_or("failure"),
                failure
            ));
        }
    }
    out.push('\n');

    out.push_str("## Open Questions\n\n");
    out.push_str("- None recorded by deterministic artifacts.\n\n");
    out.push_str("## Next Mutation\n\n");
    if proposals.is_empty() {
        out.push_str("- No mutation proposals recorded.\n");
    } else {
        for proposal in proposals {
            out.push_str(&format!(
                "- `{}`: {} (target `{}` path `{}` evidence `{}` status `{}`)\n",
                proposal.id,
                proposal.reason,
                proposal.target,
                proposal.path,
                proposal.evidence_id,
                proposal.status
            ));
        }
    }
    out
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvaluationVerdict {
    pub status: String,
    pub summary: String,
    pub failures: Vec<serde_json::Value>,
    pub evidence_refs: Vec<String>,
    pub metadata: serde_json::Value,
}

pub fn evaluate_run(run_dir: impl AsRef<Path>) -> Result<EvaluationVerdict> {
    let run_dir = run_dir.as_ref();
    let evidence = read_evidence_index(run_dir)?;
    let evaluator_config = Seed::from_path(run_dir.join("seed.snapshot.yaml"))
        .ok()
        .and_then(|seed| seed.evaluator);
    let mut failures = Vec::new();
    let mut evidence_refs = Vec::new();
    let mut scenario_results = Vec::new();
    let mut suite_summaries = 0usize;

    for artifact in &evidence.artifacts {
        let artifact_path = run_dir.join(&artifact.path);
        if !artifact_path.is_file() {
            failures.push(json!({
                "kind": "missing_evidence",
                "artifact_id": artifact.id,
                "path": artifact.path
            }));
            continue;
        }
        evidence_refs.push(artifact.path.clone());
        if artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str())
            == Some("scenario_result")
        {
            let input = fs::read_to_string(&artifact_path).with_context(|| {
                format!("failed to read scenario result {}", artifact_path.display())
            })?;
            let result: serde_json::Value = serde_json::from_str(&input).with_context(|| {
                format!(
                    "failed to parse scenario result {}",
                    artifact_path.display()
                )
            })?;
            scenario_results.push((artifact.path.clone(), result));
        }
        if artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str())
            == Some("suite_summary")
        {
            suite_summaries += 1;
        }
    }

    if scenario_results.is_empty() {
        let status = if failures.is_empty() {
            "pending"
        } else {
            "failed"
        };
        let summary = if failures.is_empty() {
            "No scenario result artifacts are available yet.".to_string()
        } else {
            format!(
                "{} evidence consistency failure(s) found before scenario results were available.",
                failures.len()
            )
        };
        let verdict = EvaluationVerdict {
            status: status.to_string(),
            summary,
            failures,
            evidence_refs,
            metadata: json!({
                "evaluator": "ouroforge-evaluator-v0",
                "scenario_results": 0,
                "suite_summaries": suite_summaries
            }),
        };
        write_json(&run_dir.join("verdict.json"), &json!(verdict))?;
        return Ok(verdict);
    }

    for (path, result) in &scenario_results {
        if result.get("status").and_then(|value| value.as_str()) != Some("passed") {
            failures.push(json!({
                "kind": "scenario_failed",
                "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                "path": path,
                "assertions": result.get("assertions").cloned().unwrap_or_else(|| json!([]))
            }));
        }
        if let Some(assertions) = result.get("assertions").and_then(|value| value.as_array()) {
            for assertion in assertions.iter().filter(|assertion| {
                assertion.get("passed").and_then(|value| value.as_bool()) == Some(false)
            }) {
                failures.push(json!({
                    "kind": "assertion_failed",
                    "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                    "path": path,
                    "target": assertion.get("target").cloned().unwrap_or(serde_json::Value::Null),
                    "assertion_path": assertion.get("path").cloned().unwrap_or(serde_json::Value::Null),
                    "operator": assertion.get("operator").cloned().unwrap_or(serde_json::Value::Null),
                    "evidence_ref": assertion.get("evidence_ref").cloned().unwrap_or(serde_json::Value::Null)
                }));
            }
        }
        if let Some(visual_checks) = result
            .get("visual_checkpoints")
            .and_then(|value| value.as_array())
        {
            for visual_check in visual_checks.iter().filter(|visual_check| {
                visual_check.get("passed").and_then(|value| value.as_bool()) == Some(false)
            }) {
                failures.push(json!({
                    "kind": "visual_checkpoint_failed",
                    "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                    "path": path,
                    "checkpoint_id": visual_check.get("checkpoint_id").cloned().unwrap_or(serde_json::Value::Null),
                    "evidence_ref": visual_check.get("evidence_ref").cloned().unwrap_or(serde_json::Value::Null),
                    "comparison": visual_check.get("comparison").cloned().unwrap_or(serde_json::Value::Null)
                }));
            }
        }
        for evidence_path in ["world_state", "frame_stats"] {
            if let Some(path) = result
                .get("evidence")
                .and_then(|evidence| evidence.get(evidence_path))
                .and_then(|value| value.as_str())
            {
                if !run_dir.join(path).is_file() {
                    failures.push(json!({
                        "kind": "missing_scenario_evidence",
                        "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                        "path": path
                    }));
                }
            }
        }
        if let Some(paths) = result
            .get("evidence")
            .and_then(|evidence| evidence.get("input_replays"))
            .and_then(|value| value.as_array())
        {
            for path in paths.iter().filter_map(|value| value.as_str()) {
                if !run_dir.join(path).is_file() {
                    failures.push(json!({
                        "kind": "missing_scenario_evidence",
                        "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                        "path": path
                    }));
                }
            }
        }
        for evidence_list in [
            "snapshots",
            "visual_checkpoints",
            "visual_checkpoint_screenshots",
            "console_logs",
            "performance_metrics",
            "cdp_trace_summaries",
        ] {
            if let Some(paths) = result
                .get("evidence")
                .and_then(|evidence| evidence.get(evidence_list))
                .and_then(|value| value.as_array())
            {
                for path in paths.iter().filter_map(|value| value.as_str()) {
                    if !run_dir.join(path).is_file() {
                        failures.push(json!({
                            "kind": "missing_scenario_evidence",
                            "scenario_id": result.get("scenario_id").cloned().unwrap_or(serde_json::Value::Null),
                            "path": path,
                            "evidence_list": evidence_list
                        }));
                    }
                }
            }
        }
    }

    if let Some(config) = &evaluator_config {
        apply_explicit_evaluator_checks(run_dir, &evidence, config, &mut failures)?;
    }

    let status = if failures.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let summary = if failures.is_empty() {
        format!(
            "{} scenario result(s) passed with consistent evidence.",
            scenario_results.len()
        )
    } else {
        format!(
            "{} failure(s) found across {} scenario result(s).",
            failures.len(),
            scenario_results.len()
        )
    };
    let verdict = EvaluationVerdict {
        status: status.to_string(),
        summary,
        failures,
        evidence_refs,
        metadata: json!({
            "evaluator": "ouroforge-evaluator-v0",
            "scenario_results": scenario_results.len(),
            "suite_summaries": suite_summaries
        }),
    };
    write_json(&run_dir.join("verdict.json"), &json!(verdict))?;
    Ok(verdict)
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunComparison {
    pub before_run_id: String,
    pub after_run_id: String,
    pub classification: String,
    pub before: RunComparisonSnapshot,
    pub after: RunComparisonSnapshot,
    pub deltas: serde_json::Value,
    pub semantic: RunSemanticDiff,
    pub evidence_refs: Vec<String>,
    pub unsupported: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticDiff {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub reasons: Vec<RunSemanticReason>,
    pub scenarios: Vec<RunSemanticScenarioDiff>,
    #[serde(rename = "worldState")]
    pub world_state: RunSemanticWorldStateDiff,
    pub events: RunSemanticEventDiff,
    pub performance: RunSemanticPerformanceDiff,
    pub evidence: RunSemanticEvidenceDiff,
    #[serde(rename = "transactionProvenance")]
    pub transaction_provenance: RunSemanticTransactionDiff,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticReason {
    pub kind: String,
    pub severity: String,
    pub summary: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticScenarioDiff {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub before: String,
    pub after: String,
    pub classification: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticWorldStateDiff {
    pub changed: Vec<RunSemanticValueDiff>,
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticValueDiff {
    pub path: String,
    pub before: serde_json::Value,
    pub after: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticEventDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticPerformanceDiff {
    pub changed: Vec<RunSemanticValueDiff>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticEvidenceDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunSemanticTransactionDiff {
    pub before: Option<RunTransactionProvenance>,
    pub after: Option<RunTransactionProvenance>,
    pub changed: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunComparisonSnapshot {
    pub run_id: String,
    pub verdict_status: String,
    pub scenario_results: usize,
    pub failed_scenarios: usize,
    pub assertion_failures: usize,
    pub performance_artifacts: usize,
    pub evidence_artifacts: usize,
    pub mutation_proposals: usize,
}

#[derive(Debug, Clone)]
struct RunComparisonDetails {
    snapshot: RunComparisonSnapshot,
    scenario_statuses: BTreeMap<String, String>,
    world_state: BTreeMap<String, serde_json::Value>,
    events: BTreeSet<String>,
    performance: BTreeMap<String, serde_json::Value>,
    evidence_keys: BTreeSet<String>,
    transaction_provenance: Option<RunTransactionProvenance>,
    warnings: Vec<String>,
}

pub fn compare_runs(
    before_run_dir: impl AsRef<Path>,
    after_run_dir: impl AsRef<Path>,
) -> Result<RunComparison> {
    let before_run_dir = before_run_dir.as_ref();
    let after_run_dir = after_run_dir.as_ref();
    let before_details = load_run_comparison_details(before_run_dir, "before")?;
    let after_details = load_run_comparison_details(after_run_dir, "after")?;
    let before = before_details.snapshot.clone();
    let after = after_details.snapshot.clone();
    let classification = classify_run_comparison(&before, &after).to_string();
    let semantic = build_run_semantic_diff(&before_details, &after_details, &classification);
    let evidence_refs = vec![
        before_run_dir.join("run.json").display().to_string(),
        before_run_dir.join("verdict.json").display().to_string(),
        before_run_dir
            .join("evidence/index.json")
            .display()
            .to_string(),
        after_run_dir.join("run.json").display().to_string(),
        after_run_dir.join("verdict.json").display().to_string(),
        after_run_dir
            .join("evidence/index.json")
            .display()
            .to_string(),
    ];
    Ok(RunComparison {
        before_run_id: before.run_id.clone(),
        after_run_id: after.run_id.clone(),
        classification,
        deltas: json!({
            "scenario_results": after.scenario_results as i64 - before.scenario_results as i64,
            "failed_scenarios": after.failed_scenarios as i64 - before.failed_scenarios as i64,
            "assertion_failures": after.assertion_failures as i64 - before.assertion_failures as i64,
            "performance_artifacts": after.performance_artifacts as i64 - before.performance_artifacts as i64,
            "evidence_artifacts": after.evidence_artifacts as i64 - before.evidence_artifacts as i64,
            "mutation_proposals": after.mutation_proposals as i64 - before.mutation_proposals as i64
        }),
        semantic,
        before,
        after,
        evidence_refs,
        unsupported: vec![
            "semantic gameplay quality is not inferred beyond verdict/scenario/evidence deltas"
                .to_string(),
        ],
    })
}

fn build_run_semantic_diff(
    before: &RunComparisonDetails,
    after: &RunComparisonDetails,
    classification: &str,
) -> RunSemanticDiff {
    let scenarios = semantic_scenario_diffs(before, after);
    let world_state = semantic_map_diff(&before.world_state, &after.world_state, 24);
    let events = semantic_set_diff(&before.events, &after.events);
    let mut performance_warnings = Vec::new();
    let performance = RunSemanticPerformanceDiff {
        changed: semantic_value_diffs(&before.performance, &after.performance, 24),
        warnings: {
            if before.performance.is_empty() || after.performance.is_empty() {
                performance_warnings.push(
                    "performance diff is partial because one side has no performance artifacts"
                        .to_string(),
                );
            }
            performance_warnings
        },
    };
    let evidence = semantic_evidence_set_diff(&before.evidence_keys, &after.evidence_keys);
    let transaction_provenance = RunSemanticTransactionDiff {
        before: before.transaction_provenance.clone(),
        after: after.transaction_provenance.clone(),
        changed: before.transaction_provenance != after.transaction_provenance,
    };
    let mut warnings = before
        .warnings
        .iter()
        .chain(after.warnings.iter())
        .cloned()
        .collect::<Vec<_>>();
    warnings.sort();
    warnings.dedup();
    let reasons = semantic_reasons(RunSemanticReasonInputs {
        classification,
        scenarios: &scenarios,
        world_state: &world_state,
        events: &events,
        performance: &performance,
        evidence: &evidence,
        transaction: &transaction_provenance,
        warnings: &warnings,
    });
    RunSemanticDiff {
        schema_version: "run-semantic-diff-v1".to_string(),
        reasons,
        scenarios,
        world_state,
        events,
        performance,
        evidence,
        transaction_provenance,
        warnings,
    }
}

pub fn write_run_comparison_artifact(
    before_run_dir: impl AsRef<Path>,
    after_run_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<PathBuf> {
    let comparison = compare_runs(before_run_dir, after_run_dir)?;
    validate_path_component("before run id", &comparison.before_run_id)?;
    validate_path_component("after run id", &comparison.after_run_id)?;
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "failed to create comparison output dir {}",
            output_dir.display()
        )
    })?;
    let path = output_dir.join(format!(
        "run-comparison-{}--{}.json",
        comparison.before_run_id, comparison.after_run_id
    ));
    write_json(&path, &json!(comparison))?;
    Ok(path)
}

fn load_run_comparison_details(run_dir: &Path, label: &str) -> Result<RunComparisonDetails> {
    let run_path = run_dir.join("run.json");
    let verdict_path = run_dir.join("verdict.json");
    let evidence_path = run_dir.join("evidence/index.json");
    for path in [&run_path, &verdict_path, &evidence_path] {
        if !path.is_file() {
            return Err(anyhow!(
                "{label} run is missing required artifact {}",
                path.display()
            ));
        }
    }
    let run = read_json_value(&run_path)?;
    let verdict = read_json_value(&verdict_path)?;
    let evidence = read_evidence_index(run_dir)?;
    let transaction_provenance = run
        .get("transaction_provenance")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok());
    let mut failed_scenarios = 0usize;
    let mut assertion_failures = 0usize;
    let mut scenario_results = 0usize;
    let mut performance_artifacts = 0usize;
    let mut mutation_proposals = 0usize;
    let mut scenario_statuses = BTreeMap::new();
    let mut world_state = BTreeMap::new();
    let mut events = BTreeSet::new();
    let mut performance = BTreeMap::new();
    let mut evidence_keys = BTreeSet::new();
    let mut warnings = Vec::new();
    for artifact in &evidence.artifacts {
        evidence_keys.insert(format!(
            "{}|{}|{}",
            artifact.id, artifact.kind, artifact.path
        ));
        match artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str())
        {
            Some("scenario_result") => {
                scenario_results += 1;
                let Ok(result) = read_json_value(run_dir.join(&artifact.path)) else {
                    warnings.push(format!(
                        "{label} scenario_result artifact could not be read: {}",
                        artifact.path
                    ));
                    continue;
                };
                let scenario_id = result
                    .get("scenario_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or(&artifact.id)
                    .to_string();
                let status = result
                    .get("status")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                scenario_statuses.insert(scenario_id, status.clone());
                if result.get("status").and_then(|value| value.as_str()) != Some("passed") {
                    failed_scenarios += 1;
                }
                assertion_failures += result
                    .get("assertions")
                    .and_then(|value| value.as_array())
                    .map(|assertions| {
                        assertions
                            .iter()
                            .filter(|assertion| {
                                assertion.get("passed").and_then(|value| value.as_bool())
                                    == Some(false)
                            })
                            .count()
                    })
                    .unwrap_or(0);
            }
            Some("world_state") => match read_json_value(run_dir.join(&artifact.path)) {
                Ok(value) => collect_semantic_scalars("world", &value, &mut world_state, 64),
                Err(_) => warnings.push(format!(
                    "{label} world_state artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("console_log") => match read_json_value(run_dir.join(&artifact.path)) {
                Ok(value) => collect_semantic_events("console", &value, &mut events, 64),
                Err(_) => warnings.push(format!(
                    "{label} console_log artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("performance_metrics") => {
                performance_artifacts += 1;
                match read_json_value(run_dir.join(&artifact.path)) {
                    Ok(value) => collect_semantic_scalars(
                        &format!("performance/{}", artifact.id),
                        &value,
                        &mut performance,
                        32,
                    ),
                    Err(_) => warnings.push(format!(
                        "{label} performance_metrics artifact could not be read: {}",
                        artifact.path
                    )),
                }
            }
            Some("frame_stats") => match read_json_value(run_dir.join(&artifact.path)) {
                Ok(value) => collect_semantic_scalars(
                    &format!("frame_stats/{}", artifact.id),
                    &value,
                    &mut performance,
                    32,
                ),
                Err(_) => warnings.push(format!(
                    "{label} frame_stats artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("mutation_proposal") => mutation_proposals += 1,
            _ => {}
        }
    }
    let snapshot = RunComparisonSnapshot {
        run_id: run
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or_else(|| anyhow!("{label} run.json missing id"))?
            .to_string(),
        verdict_status: verdict
            .get("status")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string(),
        scenario_results,
        failed_scenarios,
        assertion_failures,
        performance_artifacts,
        evidence_artifacts: evidence.artifacts.len(),
        mutation_proposals,
    };
    Ok(RunComparisonDetails {
        snapshot,
        scenario_statuses,
        world_state,
        events,
        performance,
        evidence_keys,
        transaction_provenance,
        warnings,
    })
}

fn collect_semantic_scalars(
    prefix: &str,
    value: &serde_json::Value,
    out: &mut BTreeMap<String, serde_json::Value>,
    limit: usize,
) {
    if out.len() >= limit {
        return;
    }
    match value {
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {
            out.insert(prefix.to_string(), value.clone());
        }
        serde_json::Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                if out.len() >= limit {
                    break;
                }
                collect_semantic_scalars(&format!("{prefix}/{index}"), item, out, limit);
            }
        }
        serde_json::Value::Object(map) => {
            for (key, item) in map {
                if out.len() >= limit {
                    break;
                }
                collect_semantic_scalars(&format!("{prefix}/{key}"), item, out, limit);
            }
        }
    }
}

fn collect_semantic_events(
    prefix: &str,
    value: &serde_json::Value,
    out: &mut BTreeSet<String>,
    limit: usize,
) {
    if out.len() >= limit {
        return;
    }
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                collect_semantic_events(prefix, item, out, limit);
                if out.len() >= limit {
                    break;
                }
            }
        }
        serde_json::Value::Object(map) => {
            if let Some(text) = map
                .get("text")
                .or_else(|| map.get("name"))
                .or_else(|| map.get("type"))
                .and_then(|value| value.as_str())
            {
                let level = map
                    .get("level")
                    .and_then(|value| value.as_str())
                    .unwrap_or("event");
                out.insert(format!("{prefix}:{level}:{text}"));
            }
            for item in map.values() {
                if out.len() >= limit {
                    break;
                }
                collect_semantic_events(prefix, item, out, limit);
            }
        }
        _ => {}
    }
}

fn semantic_scenario_diffs(
    before: &RunComparisonDetails,
    after: &RunComparisonDetails,
) -> Vec<RunSemanticScenarioDiff> {
    before
        .scenario_statuses
        .keys()
        .chain(after.scenario_statuses.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|scenario_id| {
            let before_status = before
                .scenario_statuses
                .get(&scenario_id)
                .cloned()
                .unwrap_or_else(|| "missing".to_string());
            let after_status = after
                .scenario_statuses
                .get(&scenario_id)
                .cloned()
                .unwrap_or_else(|| "missing".to_string());
            if before_status == after_status {
                return None;
            }
            Some(RunSemanticScenarioDiff {
                scenario_id,
                classification: scenario_status_diff_classification(&before_status, &after_status)
                    .to_string(),
                before: before_status,
                after: after_status,
                evidence_refs: Vec::new(),
            })
        })
        .collect()
}

fn scenario_status_diff_classification(before: &str, after: &str) -> &'static str {
    match (before, after) {
        (before, "passed") if before != "passed" => "improved",
        ("passed", after) if after != "passed" => "regressed",
        _ => "changed",
    }
}

fn semantic_map_diff(
    before: &BTreeMap<String, serde_json::Value>,
    after: &BTreeMap<String, serde_json::Value>,
    limit: usize,
) -> RunSemanticWorldStateDiff {
    let before_keys = before.keys().cloned().collect::<BTreeSet<_>>();
    let after_keys = after.keys().cloned().collect::<BTreeSet<_>>();
    RunSemanticWorldStateDiff {
        changed: semantic_value_diffs(before, after, limit),
        added: after_keys
            .difference(&before_keys)
            .take(limit)
            .cloned()
            .collect(),
        removed: before_keys
            .difference(&after_keys)
            .take(limit)
            .cloned()
            .collect(),
    }
}

fn semantic_value_diffs(
    before: &BTreeMap<String, serde_json::Value>,
    after: &BTreeMap<String, serde_json::Value>,
    limit: usize,
) -> Vec<RunSemanticValueDiff> {
    before
        .keys()
        .filter(|key| after.contains_key(*key))
        .filter_map(|key| {
            let before_value = before.get(key)?;
            let after_value = after.get(key)?;
            (before_value != after_value).then(|| RunSemanticValueDiff {
                path: key.clone(),
                before: before_value.clone(),
                after: after_value.clone(),
            })
        })
        .take(limit)
        .collect()
}

fn semantic_set_diff(before: &BTreeSet<String>, after: &BTreeSet<String>) -> RunSemanticEventDiff {
    RunSemanticEventDiff {
        added: after.difference(before).take(64).cloned().collect(),
        removed: before.difference(after).take(64).cloned().collect(),
    }
}

fn semantic_evidence_set_diff(
    before: &BTreeSet<String>,
    after: &BTreeSet<String>,
) -> RunSemanticEvidenceDiff {
    RunSemanticEvidenceDiff {
        added: after.difference(before).take(64).cloned().collect(),
        removed: before.difference(after).take(64).cloned().collect(),
    }
}

struct RunSemanticReasonInputs<'a> {
    classification: &'a str,
    scenarios: &'a [RunSemanticScenarioDiff],
    world_state: &'a RunSemanticWorldStateDiff,
    events: &'a RunSemanticEventDiff,
    performance: &'a RunSemanticPerformanceDiff,
    evidence: &'a RunSemanticEvidenceDiff,
    transaction: &'a RunSemanticTransactionDiff,
    warnings: &'a [String],
}

fn semantic_reasons(inputs: RunSemanticReasonInputs<'_>) -> Vec<RunSemanticReason> {
    let mut reasons = Vec::new();
    for scenario in inputs.scenarios {
        reasons.push(RunSemanticReason {
            kind: "scenario_verdict".to_string(),
            severity: scenario.classification.clone(),
            summary: format!(
                "scenario {} changed from {} to {}",
                scenario.scenario_id, scenario.before, scenario.after
            ),
            evidence_refs: scenario.evidence_refs.clone(),
        });
    }
    if !inputs.world_state.changed.is_empty()
        || !inputs.world_state.added.is_empty()
        || !inputs.world_state.removed.is_empty()
    {
        reasons.push(RunSemanticReason {
            kind: "world_state".to_string(),
            severity: "changed".to_string(),
            summary: format!(
                "{} world-state values changed, {} added, {} removed",
                inputs.world_state.changed.len(),
                inputs.world_state.added.len(),
                inputs.world_state.removed.len()
            ),
            evidence_refs: Vec::new(),
        });
    }
    if !inputs.events.added.is_empty() || !inputs.events.removed.is_empty() {
        reasons.push(RunSemanticReason {
            kind: "events".to_string(),
            severity: "changed".to_string(),
            summary: format!(
                "{} events added, {} removed",
                inputs.events.added.len(),
                inputs.events.removed.len()
            ),
            evidence_refs: Vec::new(),
        });
    }
    if !inputs.performance.changed.is_empty() {
        reasons.push(RunSemanticReason {
            kind: "performance".to_string(),
            severity: "changed".to_string(),
            summary: format!(
                "{} performance values changed",
                inputs.performance.changed.len()
            ),
            evidence_refs: Vec::new(),
        });
    }
    if !inputs.evidence.added.is_empty() || !inputs.evidence.removed.is_empty() {
        reasons.push(RunSemanticReason {
            kind: "evidence_artifacts".to_string(),
            severity: "changed".to_string(),
            summary: format!(
                "{} evidence artifacts added, {} removed",
                inputs.evidence.added.len(),
                inputs.evidence.removed.len()
            ),
            evidence_refs: Vec::new(),
        });
    }
    if inputs.transaction.changed {
        reasons.push(RunSemanticReason {
            kind: "transaction_provenance".to_string(),
            severity: "changed".to_string(),
            summary: "scene edit transaction provenance changed".to_string(),
            evidence_refs: Vec::new(),
        });
    }
    if !inputs.warnings.is_empty() {
        reasons.push(RunSemanticReason {
            kind: "warnings".to_string(),
            severity: "warning".to_string(),
            summary: format!("{} semantic diff warning(s)", inputs.warnings.len()),
            evidence_refs: Vec::new(),
        });
    }
    if reasons.is_empty() {
        reasons.push(RunSemanticReason {
            kind: "classification".to_string(),
            severity: inputs.classification.to_string(),
            summary: format!("comparison classification is {}", inputs.classification),
            evidence_refs: Vec::new(),
        });
    }
    reasons
}

fn classify_run_comparison(
    before: &RunComparisonSnapshot,
    after: &RunComparisonSnapshot,
) -> &'static str {
    match (
        before.verdict_status.as_str(),
        after.verdict_status.as_str(),
    ) {
        (before_status, after_status) if before_status != "passed" && after_status == "passed" => {
            "improved"
        }
        ("passed", after_status) if after_status != "passed" => "regressed",
        _ if after.failed_scenarios < before.failed_scenarios
            || after.assertion_failures < before.assertion_failures =>
        {
            "improved"
        }
        _ if after.failed_scenarios > before.failed_scenarios
            || after.assertion_failures > before.assertion_failures =>
        {
            "regressed"
        }
        _ if run_comparison_snapshots_match(before, after) => "no_change",
        _ => "changed",
    }
}

fn run_comparison_snapshots_match(
    before: &RunComparisonSnapshot,
    after: &RunComparisonSnapshot,
) -> bool {
    before.verdict_status == after.verdict_status
        && before.scenario_results == after.scenario_results
        && before.failed_scenarios == after.failed_scenarios
        && before.assertion_failures == after.assertion_failures
        && before.performance_artifacts == after.performance_artifacts
        && before.evidence_artifacts == after.evidence_artifacts
        && before.mutation_proposals == after.mutation_proposals
}

fn apply_explicit_evaluator_checks(
    run_dir: &Path,
    evidence: &EvidenceIndex,
    config: &EvaluatorConfig,
    failures: &mut Vec<serde_json::Value>,
) -> Result<()> {
    if let Some(console) = &config.console {
        for artifact in evidence.artifacts.iter().filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("console_log")
        }) {
            let value = read_json_value(run_dir.join(&artifact.path))?;
            for entry in console_entries(&value) {
                let level = entry
                    .get("level")
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if console
                    .fail_on_levels
                    .iter()
                    .any(|expected| expected == level)
                {
                    failures.push(json!({
                        "kind": "console_level_matched",
                        "level": level,
                        "path": artifact.path,
                        "text": entry.get("text").cloned().unwrap_or(serde_json::Value::Null)
                    }));
                }
            }
        }
    }

    if let Some(performance) = &config.performance {
        for artifact in evidence.artifacts.iter().filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("performance_metrics")
        }) {
            let value = read_json_value(run_dir.join(&artifact.path))?;
            for (metric, threshold) in &performance.max_metrics {
                if let Some(actual) = performance_metric_value(&value, metric) {
                    if actual > *threshold as f64 {
                        failures.push(json!({
                            "kind": "performance_threshold_exceeded",
                            "metric": metric,
                            "threshold": threshold,
                            "actual": actual,
                            "path": artifact.path
                        }));
                    }
                }
            }
        }
    }
    Ok(())
}

fn console_entries(value: &serde_json::Value) -> Vec<&serde_json::Value> {
    value
        .as_array()
        .or_else(|| value.get("logs").and_then(|logs| logs.as_array()))
        .map(|entries| entries.iter().collect())
        .unwrap_or_default()
}

fn performance_metric_value(value: &serde_json::Value, metric_name: &str) -> Option<f64> {
    let metrics = value
        .get("metrics")
        .or_else(|| value.get("Metrics"))
        .unwrap_or(value);
    let metrics = metrics
        .get("metrics")
        .or_else(|| metrics.get("Metrics"))
        .unwrap_or(metrics);
    metrics.as_array()?.iter().find_map(|metric| {
        let name = metric
            .get("name")
            .or_else(|| metric.get("Name"))
            .and_then(|value| value.as_str())?;
        if name != metric_name {
            return None;
        }
        metric
            .get("value")
            .or_else(|| metric.get("Value"))
            .and_then(|value| value.as_f64())
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioRunConfig {
    pub run_dir: PathBuf,
    pub url: String,
    pub debugging_http_url: String,
}

impl ScenarioRunConfig {
    pub fn new(run_dir: impl Into<PathBuf>, url: impl Into<String>) -> Result<Self> {
        let url = url.into();
        require_http_url("scenario run URL", &url)?;
        Ok(Self {
            run_dir: run_dir.into(),
            url,
            debugging_http_url: "http://127.0.0.1:9222".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ScenarioRunSummary {
    pub scenarios: usize,
    pub completed: usize,
    pub passed: usize,
    pub failed: usize,
    pub scenario_order: Vec<String>,
    pub suite_summary_path: String,
    pub evidence_paths: Vec<String>,
    pub result_paths: Vec<String>,
}

impl ScenarioRunSummary {
    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }
}

pub fn run_scenarios(config: &ScenarioRunConfig) -> Result<ScenarioRunSummary> {
    let seed = Seed::from_path(config.run_dir.join("seed.snapshot.yaml"))?;
    let connection = create_cdp_page_target(&config.debugging_http_url, "about:blank")?;
    let transport = WebSocketCdpTransport::connect(&connection)?;
    let mut client = CdpClient::new(transport);

    client.enable_page()?;
    install_console_capture(&mut client)?;
    let _ = client.bring_page_to_front();
    client.navigate(&config.url)?;
    std::thread::sleep(Duration::from_millis(300));

    run_scenarios_with_client(config, &seed, &mut client)
}

fn run_scenarios_with_client<T: CdpTransport>(
    config: &ScenarioRunConfig,
    seed: &Seed,
    client: &mut CdpClient<T>,
) -> Result<ScenarioRunSummary> {
    let mut evidence_paths = Vec::new();
    let mut result_paths = Vec::new();
    let mut scenario_order = Vec::new();
    let mut scenario_summaries = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    for scenario in &seed.scenarios {
        scenario_order.push(scenario.id.clone());
        let result = run_scenario(config, client, scenario)?;
        evidence_paths.extend(result.evidence_paths);
        result_paths.push(result.result_path.clone());
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
        }
        scenario_summaries.push(json!({
            "scenario_id": scenario.id,
            "status": if result.passed { "passed" } else { "failed" },
            "result_path": result.result_path
        }));
    }
    let suite_summary_path = format!("evidence/suite-summary-{}.json", unix_millis()?);
    write_json(
        &config.run_dir.join(&suite_summary_path),
        &json!({
            "artifact": "suite_summary",
            "status": if failed == 0 { "passed" } else { "failed" },
            "scenarios": seed.scenarios.len(),
            "completed": result_paths.len(),
            "passed": passed,
            "failed": failed,
            "scenario_order": scenario_order.clone(),
            "scenario_results": scenario_summaries,
            "result_paths": result_paths.clone(),
            "evidence_paths": evidence_paths.clone()
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("suite-summary-{}", unix_millis()?),
        "application/json",
        &suite_summary_path,
        json!({
            "artifact": "suite_summary",
            "scenarios": seed.scenarios.len(),
            "passed": passed,
            "failed": failed
        }),
    )?;
    evidence_paths.push(suite_summary_path.clone());

    Ok(ScenarioRunSummary {
        scenarios: seed.scenarios.len(),
        completed: result_paths.len(),
        passed,
        failed,
        scenario_order,
        suite_summary_path,
        evidence_paths,
        result_paths,
    })
}

struct ScenarioExecutionResult {
    passed: bool,
    evidence_paths: Vec<String>,
    result_path: String,
}

fn run_scenario<T: CdpTransport>(
    config: &ScenarioRunConfig,
    client: &mut CdpClient<T>,
    scenario: &Scenario,
) -> Result<ScenarioExecutionResult> {
    validate_path_component("scenario id", &scenario.id)?;
    append_ledger_event(
        &config.run_dir,
        "scenario.started",
        "scenario-runner",
        json!({ "scenario_id": scenario.id, "url": config.url }),
    )?;

    let suffix = unix_millis()?;
    let scenario_dir = format!("evidence/scenarios/{}", scenario.id);
    fs::create_dir_all(config.run_dir.join(&scenario_dir)).with_context(|| {
        format!(
            "failed to create scenario evidence directory {}",
            config.run_dir.join(&scenario_dir).display()
        )
    })?;

    let mut replay_paths = Vec::new();
    let mut snapshot_paths = Vec::new();
    let mut visual_checkpoint_paths = Vec::new();
    let mut visual_checkpoint_screenshot_paths = Vec::new();
    let mut visual_checkpoint_summaries = Vec::new();
    let mut snapshot_ids = std::collections::BTreeMap::new();
    for step in &scenario.steps {
        match step {
            ScenarioStep::Replay { replay } => {
                let replay_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    "input-replay",
                    "input_replay",
                    unix_millis()?,
                    &json!(replay),
                )?;
                replay_paths.push(replay_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.input_replay",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "replay_id": replay.id,
                        "events": replay.events.len(),
                        "path": replay_path,
                        "source": "inline"
                    }),
                )?;
                execute_scenario_step(client, step)?;
            }
            ScenarioStep::ReplayRef { replay_ref } => {
                let replay = replay_ref.load_from_base(&config.run_dir)?;
                let replay_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    "input-replay",
                    "input_replay",
                    unix_millis()?,
                    &json!({
                        "reference": replay_ref,
                        "replay": replay
                    }),
                )?;
                replay_paths.push(replay_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.input_replay",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "replay_id": replay.id,
                        "events": replay.events.len(),
                        "path": replay_path,
                        "source": "replayRef",
                        "reference_path": replay_ref.path
                    }),
                )?;
                execute_input_replay(client, &replay)?;
            }
            ScenarioStep::Snapshot { snapshot } => {
                let snapshot_result = client.evaluate_json("window.__OUROFORGE__.snapshot()")?;
                let snapshot_id = snapshot_result
                    .get("snapshotId")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| anyhow!("snapshot probe did not return snapshotId"))?
                    .to_string();
                snapshot_ids.insert(snapshot.id.clone(), snapshot_id.clone());
                let world_state = client.evaluate_json("window.__OUROFORGE__.getWorldState()")?;
                let snapshot_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    &format!("snapshot-{}", snapshot.id),
                    "snapshot",
                    unix_millis()?,
                    &json!({
                        "step_id": snapshot.id,
                        "snapshot_id": snapshot_id,
                        "snapshot": snapshot_result,
                        "world_state": world_state
                    }),
                )?;
                snapshot_paths.push(snapshot_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.snapshot",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "step_id": snapshot.id,
                        "snapshot_id": snapshot_id,
                        "path": snapshot_path
                    }),
                )?;
            }
            ScenarioStep::Restore { restore } => {
                let snapshot_id = snapshot_ids
                    .get(&restore.id)
                    .ok_or_else(|| anyhow!("snapshot id not found for restore: {}", restore.id))?;
                let snapshot_json = serde_json::to_string(snapshot_id)
                    .context("failed to serialize snapshot id")?;
                let restored_world_state = client
                    .evaluate_json(&format!("window.__OUROFORGE__.restore({snapshot_json})"))?;
                let restore_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    &format!("restore-{}", restore.id),
                    "snapshot_restore",
                    unix_millis()?,
                    &json!({
                        "step_id": restore.id,
                        "snapshot_id": snapshot_id,
                        "world_state": restored_world_state
                    }),
                )?;
                snapshot_paths.push(restore_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.restore",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "step_id": restore.id,
                        "snapshot_id": snapshot_id,
                        "path": restore_path
                    }),
                )?;
            }
            ScenarioStep::VisualCheckpoint { visual_checkpoint } => {
                let capture = capture_visual_checkpoint(
                    config,
                    scenario,
                    &scenario_dir,
                    visual_checkpoint,
                    client,
                )?;
                visual_checkpoint_paths.push(capture.metadata_path.clone());
                visual_checkpoint_screenshot_paths.push(capture.screenshot_path.clone());
                visual_checkpoint_summaries.push(capture.summary.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.visual_checkpoint",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "checkpoint_id": visual_checkpoint.id,
                        "screenshot_path": capture.screenshot_path,
                        "metadata_path": capture.metadata_path,
                        "advisory": true
                    }),
                )?;
            }
            _ => execute_scenario_step(client, step)?,
        }
    }

    let world_state = client.evaluate_json("window.__OUROFORGE__.getWorldState()")?;
    let world_state_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "world-state",
        "world_state",
        suffix,
        &world_state,
    )?;
    let frame_stats = client.evaluate_json("window.__OUROFORGE__.getFrameStats()")?;
    let frame_stats_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "frame-stats",
        "frame_stats",
        unix_millis()?,
        &frame_stats,
    )?;
    let mut console_paths = Vec::new();
    let mut performance_paths = Vec::new();
    let mut trace_paths = Vec::new();
    let mut console_source = json!({ "logs": [], "count": 0 });
    if let Ok(console_logs) = client.evaluate_json("window.__OUROFORGE_CONSOLE__ || []") {
        console_source = json!({
            "logs": console_entries(&console_logs)
                .into_iter()
                .filter(|entry| entry.get("level").and_then(|value| value.as_str()) == Some("error"))
                .cloned()
                .collect::<Vec<_>>(),
            "count": console_entries(&console_logs)
                .into_iter()
                .filter(|entry| entry.get("level").and_then(|value| value.as_str()) == Some("error"))
                .count()
        });
        let console_path = write_scenario_json_artifact(
            config,
            scenario,
            &scenario_dir,
            "console-log",
            "console_log",
            unix_millis()?,
            &json!({
                "bounded": true,
                "limit": 100,
                "logs": console_logs
            }),
        )?;
        console_paths.push(console_path);
    }
    let mut scenario_performance_metric_count = None;
    let mut performance_source = json!({});
    if let Ok(performance_metrics) = client
        .enable_performance()
        .and_then(|_| client.performance_metrics())
    {
        scenario_performance_metric_count = Some(count_cdp_metrics(&performance_metrics));
        performance_source = performance_metrics_by_name(&performance_metrics);
        let performance_path = write_scenario_json_artifact(
            config,
            scenario,
            &scenario_dir,
            "performance-metrics",
            "performance_metrics",
            unix_millis()?,
            &json!({
                "bounded": true,
                "metrics": performance_metrics
            }),
        )?;
        performance_paths.push(performance_path);
    }
    let trace_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "cdp-trace-summary",
        "cdp_trace_summary",
        unix_millis()?,
        &json!({
            "bounded": true,
            "limit": 32,
            "source": "scenario-cdp-summary",
            "scenarioId": scenario.id,
            "events": [
                { "name": "Scenario.steps", "count": scenario.steps.len() },
                { "name": "Scenario.assertions", "count": scenario.assertions.len() },
                { "name": "Runtime.getWorldState", "captured": true },
                { "name": "Runtime.getFrameStats", "captured": true },
                { "name": "Performance.getMetrics", "metricCount": scenario_performance_metric_count.unwrap_or(0) }
            ]
        }),
    )?;
    trace_paths.push(trace_path.clone());

    let runtime_events = json!({
        "steps": &scenario.steps,
        "stepCount": scenario.steps.len(),
        "inputReplays": &replay_paths,
        "snapshots": &snapshot_paths
    });
    let collision_source = world_state
        .get("collisions")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let audio_source = world_state
        .get("audio")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let animation_source = world_state
        .get("animation")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let assertion_sources = ScenarioAssertionSources {
        world_state: AssertionSource {
            value: &world_state,
            evidence_ref: &world_state_path,
        },
        frame_stats: AssertionSource {
            value: &frame_stats,
            evidence_ref: &frame_stats_path,
        },
        runtime_events: AssertionSource {
            value: &runtime_events,
            evidence_ref: &trace_path,
        },
        performance_metrics: AssertionSource {
            value: &performance_source,
            evidence_ref: performance_paths
                .first()
                .map_or(frame_stats_path.as_str(), String::as_str),
        },
        console_errors: AssertionSource {
            value: &console_source,
            evidence_ref: console_paths
                .first()
                .map_or(trace_path.as_str(), String::as_str),
        },
        collision_evidence: AssertionSource {
            value: &collision_source,
            evidence_ref: &world_state_path,
        },
        audio_evidence: AssertionSource {
            value: &audio_source,
            evidence_ref: &world_state_path,
        },
        animation_evidence: AssertionSource {
            value: &animation_source,
            evidence_ref: &world_state_path,
        },
    };
    let assertions = evaluate_scenario_assertions(scenario, &assertion_sources);
    for assertion in &assertions {
        append_ledger_event(
            &config.run_dir,
            "scenario.assertion",
            "scenario-runner",
            json!({
                "scenario_id": scenario.id,
                "target": assertion["target"],
                "path": assertion["path"],
                "passed": assertion["passed"],
                "evidence_path": assertion["evidence_ref"]
            }),
        )?;
    }
    let passed = assertions
        .iter()
        .all(|assertion| assertion["passed"].as_bool() == Some(true))
        && visual_checkpoint_summaries
            .iter()
            .all(|summary| summary["passed"].as_bool() != Some(false));
    let status = if passed { "passed" } else { "failed" };
    let result_path = format!("{scenario_dir}/scenario-result-{}.json", unix_millis()?);
    write_json(
        &config.run_dir.join(&result_path),
        &json!({
            "scenario_id": scenario.id,
            "status": status,
            "evidence": {
                "world_state": world_state_path,
                "frame_stats": frame_stats_path,
                "input_replays": replay_paths.clone(),
                "snapshots": snapshot_paths.clone(),
                "visual_checkpoints": visual_checkpoint_paths.clone(),
                "visual_checkpoint_screenshots": visual_checkpoint_screenshot_paths.clone(),
                "console_logs": console_paths.clone(),
                "performance_metrics": performance_paths.clone(),
                "cdp_trace_summaries": trace_paths.clone()
            },
            "assertions": assertions,
            "visual_checkpoints": visual_checkpoint_summaries
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("scenario-result-{}-{}", scenario.id, unix_millis()?),
        "application/json",
        &result_path,
        json!({ "scenario_id": scenario.id, "url": config.url, "artifact": "scenario_result", "status": status }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "scenario.completed",
        "scenario-runner",
        json!({
            "scenario_id": scenario.id,
            "status": status,
            "world_state_path": world_state_path,
            "frame_stats_path": frame_stats_path,
            "input_replay_paths": replay_paths.clone(),
            "snapshot_paths": snapshot_paths.clone(),
            "visual_checkpoint_paths": visual_checkpoint_paths.clone(),
            "visual_checkpoint_screenshot_paths": visual_checkpoint_screenshot_paths.clone(),
            "console_log_paths": console_paths.clone(),
            "performance_metric_paths": performance_paths.clone(),
            "cdp_trace_summary_paths": trace_paths.clone(),
            "result_path": result_path
        }),
    )?;
    let mut evidence_paths = replay_paths;
    evidence_paths.extend(snapshot_paths);
    evidence_paths.extend(visual_checkpoint_paths);
    evidence_paths.extend(visual_checkpoint_screenshot_paths);
    evidence_paths.push(world_state_path);
    evidence_paths.push(frame_stats_path);
    evidence_paths.extend(console_paths);
    evidence_paths.extend(performance_paths);
    evidence_paths.extend(trace_paths);
    Ok(ScenarioExecutionResult {
        passed,
        evidence_paths,
        result_path,
    })
}

fn write_scenario_json_artifact(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    file_prefix: &str,
    artifact_name: &str,
    suffix: u128,
    value: &serde_json::Value,
) -> Result<String> {
    let rel_path = format!("{scenario_dir}/{file_prefix}-{suffix}.json");
    write_json(&config.run_dir.join(&rel_path), value)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("scenario-{artifact_name}-{}-{suffix}", scenario.id),
        "application/json",
        &rel_path,
        json!({ "scenario_id": scenario.id, "url": config.url, "artifact": artifact_name }),
    )?;
    Ok(rel_path)
}

#[derive(Clone, Copy)]
struct AssertionSource<'a> {
    value: &'a serde_json::Value,
    evidence_ref: &'a str,
}

struct ScenarioAssertionSources<'a> {
    world_state: AssertionSource<'a>,
    frame_stats: AssertionSource<'a>,
    runtime_events: AssertionSource<'a>,
    performance_metrics: AssertionSource<'a>,
    console_errors: AssertionSource<'a>,
    collision_evidence: AssertionSource<'a>,
    audio_evidence: AssertionSource<'a>,
    animation_evidence: AssertionSource<'a>,
}

struct VisualCheckpointCapture {
    screenshot_path: String,
    metadata_path: String,
    summary: serde_json::Value,
}

fn capture_visual_checkpoint<T: CdpTransport>(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    checkpoint: &VisualCheckpointStep,
    client: &mut CdpClient<T>,
) -> Result<VisualCheckpointCapture> {
    let suffix = unix_millis()?;
    let screenshot = client.capture_screenshot_png()?;
    let dimensions = png_dimensions(&screenshot);
    let comparison = visual_comparison_summary(checkpoint, dimensions);
    let passed = comparison
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    let screenshot_path = format!(
        "{scenario_dir}/visual-checkpoint-{}-{suffix}.png",
        checkpoint.id
    );
    let full_screenshot_path = config.run_dir.join(&screenshot_path);
    fs::write(&full_screenshot_path, screenshot).with_context(|| {
        format!(
            "failed to write visual checkpoint screenshot {}",
            full_screenshot_path.display()
        )
    })?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "scenario-visual-checkpoint-screenshot-{}-{}-{suffix}",
            scenario.id, checkpoint.id
        ),
        "image/png",
        &screenshot_path,
        json!({
            "scenario_id": scenario.id,
            "checkpoint_id": checkpoint.id,
            "url": config.url,
            "artifact": "visual_checkpoint_screenshot",
            "advisory": checkpoint.threshold.is_none()
        }),
    )?;
    let summary = json!({
        "checkpoint_id": checkpoint.id,
        "screenshot_path": screenshot_path,
        "metadata_path": format!("{scenario_dir}/visual-checkpoint-{}-{suffix}.json", checkpoint.id),
        "advisory": checkpoint.threshold.is_none(),
        "passed": passed,
        "evidence_ref": screenshot_path,
        "comparison": comparison
    });

    let metadata_path = write_scenario_json_artifact(
        config,
        scenario,
        scenario_dir,
        &format!("visual-checkpoint-{}", checkpoint.id),
        "visual_checkpoint",
        suffix,
        &json!({
            "checkpoint_id": checkpoint.id,
            "screenshot_path": screenshot_path,
            "advisory": checkpoint.threshold.is_none(),
            "passed": passed,
            "dimensions": dimensions.map(|(width, height)| json!({ "width": width, "height": height })),
            "comparison": comparison,
            "baseline": checkpoint.baseline
        }),
    )?;
    Ok(VisualCheckpointCapture {
        screenshot_path,
        metadata_path,
        summary,
    })
}

fn png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 24 || &bytes[0..8] != PNG_SIGNATURE || &bytes[12..16] != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some((width, height))
}

fn visual_comparison_summary(
    checkpoint: &VisualCheckpointStep,
    dimensions: Option<(u32, u32)>,
) -> serde_json::Value {
    let baseline = checkpoint.baseline.as_ref();
    let threshold = checkpoint.threshold.as_ref();
    let width_delta = baseline
        .and_then(|baseline| baseline.width)
        .zip(dimensions.map(|(width, _)| width))
        .map(|(baseline, actual)| baseline.abs_diff(actual));
    let height_delta = baseline
        .and_then(|baseline| baseline.height)
        .zip(dimensions.map(|(_, height)| height))
        .map(|(baseline, actual)| baseline.abs_diff(actual));
    let passed = threshold.map(|threshold| {
        width_delta.is_some_and(|delta| delta <= threshold.max_dimension_delta)
            && height_delta.is_some_and(|delta| delta <= threshold.max_dimension_delta)
    });
    json!({
        "bounded": true,
        "kind": "dimension_summary",
        "pixel_diff": null,
        "dimensions": dimensions.map(|(width, height)| json!({ "width": width, "height": height })),
        "baseline": baseline,
        "threshold": threshold,
        "width_delta": width_delta,
        "height_delta": height_delta,
        "advisory": threshold.is_none(),
        "passed": passed
    })
}

fn evaluate_scenario_assertions(
    scenario: &Scenario,
    sources: &ScenarioAssertionSources<'_>,
) -> Vec<serde_json::Value> {
    scenario
        .assertions
        .iter()
        .map(|assertion| {
            let (target, assertion, source) = match assertion {
                ScenarioAssertion::WorldState { world_state } => {
                    ("world_state", world_state, sources.world_state)
                }
                ScenarioAssertion::FrameStats { frame_stats } => {
                    ("frame_stats", frame_stats, sources.frame_stats)
                }
                ScenarioAssertion::RuntimeEvents { runtime_events } => {
                    ("runtime_events", runtime_events, sources.runtime_events)
                }
                ScenarioAssertion::PerformanceMetrics {
                    performance_metrics,
                } => (
                    "performance_metrics",
                    performance_metrics,
                    sources.performance_metrics,
                ),
                ScenarioAssertion::ConsoleErrors { console_errors } => {
                    ("console_errors", console_errors, sources.console_errors)
                }
                ScenarioAssertion::CollisionEvidence { collision_evidence } => (
                    "collision_evidence",
                    collision_evidence,
                    sources.collision_evidence,
                ),
                ScenarioAssertion::AudioEvidence { audio_evidence } => {
                    ("audio_evidence", audio_evidence, sources.audio_evidence)
                }
                ScenarioAssertion::AnimationEvidence { animation_evidence } => (
                    "animation_evidence",
                    animation_evidence,
                    sources.animation_evidence,
                ),
            };
            let actual = read_json_path(source.value, &assertion.path)
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            let outcome = evaluate_json_path_assertion(assertion, &actual);
            json!({
                "target": target,
                "path": assertion.path,
                "operator": outcome.operator,
                "expected": outcome.expected,
                "actual": actual,
                "passed": outcome.passed,
                "evidence_ref": source.evidence_ref
            })
        })
        .collect()
}

struct AssertionEvaluation {
    operator: &'static str,
    expected: serde_json::Value,
    passed: bool,
}

fn evaluate_json_path_assertion(
    assertion: &JsonPathAssertion,
    actual: &serde_json::Value,
) -> AssertionEvaluation {
    if let Some(expected) = &assertion.equals {
        return AssertionEvaluation {
            operator: "equals",
            expected: expected.clone(),
            passed: actual == expected,
        };
    }
    if let Some(expected) = &assertion.not_equals {
        return AssertionEvaluation {
            operator: "notEquals",
            expected: expected.clone(),
            passed: actual != expected,
        };
    }
    if let Some(expected) = assertion.exists {
        return AssertionEvaluation {
            operator: "exists",
            expected: json!(expected),
            passed: (actual != &serde_json::Value::Null) == expected,
        };
    }
    if let Some(expected) = &assertion.contains {
        return AssertionEvaluation {
            operator: "contains",
            expected: expected.clone(),
            passed: json_contains(actual, expected),
        };
    }
    if let Some(expected) = &assertion.greater_than {
        return AssertionEvaluation {
            operator: "greaterThan",
            expected: expected.clone(),
            passed: compare_json_numbers(actual, expected, std::cmp::Ordering::Greater),
        };
    }
    if let Some(expected) = &assertion.less_than {
        return AssertionEvaluation {
            operator: "lessThan",
            expected: expected.clone(),
            passed: compare_json_numbers(actual, expected, std::cmp::Ordering::Less),
        };
    }
    if let Some(expected) = assertion.count_equals {
        return AssertionEvaluation {
            operator: "countEquals",
            expected: json!(expected),
            passed: json_count(actual) == Some(expected),
        };
    }
    if let Some(expected) = assertion.count_greater_than {
        return AssertionEvaluation {
            operator: "countGreaterThan",
            expected: json!(expected),
            passed: json_count(actual).is_some_and(|actual| actual > expected),
        };
    }
    if let Some(expected) = assertion.count_less_than {
        return AssertionEvaluation {
            operator: "countLessThan",
            expected: json!(expected),
            passed: json_count(actual).is_some_and(|actual| actual < expected),
        };
    }
    AssertionEvaluation {
        operator: "invalid",
        expected: serde_json::Value::Null,
        passed: false,
    }
}

fn json_contains(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    match actual {
        serde_json::Value::Array(items) => items.iter().any(|item| item == expected),
        serde_json::Value::Object(object) => expected
            .as_str()
            .is_some_and(|key| object.contains_key(key)),
        serde_json::Value::String(text) => expected
            .as_str()
            .is_some_and(|needle| text.contains(needle)),
        _ => false,
    }
}

fn compare_json_numbers(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
    ordering: std::cmp::Ordering,
) -> bool {
    match (actual.as_f64(), expected.as_f64()) {
        (Some(actual), Some(expected)) => actual.partial_cmp(&expected) == Some(ordering),
        _ => false,
    }
}

fn json_count(value: &serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Array(items) => Some(items.len() as u64),
        serde_json::Value::Object(object) => Some(object.len() as u64),
        serde_json::Value::String(text) => Some(text.chars().count() as u64),
        _ => None,
    }
}

fn performance_metrics_by_name(metrics: &serde_json::Value) -> serde_json::Value {
    let mut mapped = serde_json::Map::new();
    let values = metrics
        .get("metrics")
        .or_else(|| metrics.get("Metrics"))
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten();
    for metric in values {
        if let (Some(name), Some(value)) = (
            metric
                .get("name")
                .or_else(|| metric.get("Name"))
                .and_then(|value| value.as_str()),
            metric.get("value").or_else(|| metric.get("Value")).cloned(),
        ) {
            mapped.insert(name.to_string(), value);
        }
    }
    serde_json::Value::Object(mapped)
}

fn read_json_path<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for segment in path.split('.') {
        current = match current {
            serde_json::Value::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => current.get(segment)?,
        };
    }
    Some(current)
}

fn wait_for_runtime_step_api<T: CdpTransport>(client: &mut CdpClient<T>) -> Result<()> {
    let expression = "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.step === 'function' && typeof window.__OUROFORGE__.setInput === 'function')";
    let mut last_value = serde_json::Value::Null;
    for _ in 0..30 {
        last_value = client.evaluate_json(expression)?;
        if last_value == json!(true) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(anyhow!(
        "window.__OUROFORGE__ step/input API not ready for scenario step; last readiness value: {}",
        last_value
    ))
}

fn execute_scenario_step<T: CdpTransport>(
    client: &mut CdpClient<T>,
    step: &ScenarioStep,
) -> Result<()> {
    wait_for_runtime_step_api(client)?;
    match step {
        ScenarioStep::Wait { wait } => {
            client.evaluate_json(&format!("window.__OUROFORGE__.step({})", wait.frames))?;
        }
        ScenarioStep::Input { input } => {
            let input_json =
                serde_json::to_string(input).context("failed to serialize input step")?;
            client.evaluate_json(&format!("window.__OUROFORGE__.setInput({input_json})"))?;
        }
        ScenarioStep::Replay { replay } => {
            execute_input_replay(client, replay)?;
        }
        ScenarioStep::ReplayRef { .. } => {
            return Err(anyhow!(
                "replayRef execution is implemented by scenario evidence context"
            ));
        }
        ScenarioStep::Snapshot { .. }
        | ScenarioStep::Restore { .. }
        | ScenarioStep::VisualCheckpoint { .. } => {
            return Err(anyhow!(
                "snapshot/restore/visual checkpoint steps require scenario evidence context"
            ));
        }
    }
    Ok(())
}

fn execute_input_replay<T: CdpTransport>(
    client: &mut CdpClient<T>,
    replay: &InputReplay,
) -> Result<()> {
    replay.validate()?;
    let mut current_frame = 0;
    let mut index = 0;
    while index < replay.events.len() {
        let frame = replay.events[index].frame;
        if frame > current_frame {
            wait_for_runtime_step_api(client)?;
            client.evaluate_json(&format!(
                "window.__OUROFORGE__.step({})",
                frame - current_frame
            ))?;
            current_frame = frame;
        }

        let mut patch = InputStep::default();
        while index < replay.events.len() && replay.events[index].frame == frame {
            let event = &replay.events[index];
            match event.key {
                ReplayKey::Left => patch.left = Some(event.pressed),
                ReplayKey::Right => patch.right = Some(event.pressed),
                ReplayKey::Up => patch.up = Some(event.pressed),
                ReplayKey::Down => patch.down = Some(event.pressed),
            }
            index += 1;
        }
        let input_json =
            serde_json::to_string(&patch).context("failed to serialize replay input patch")?;
        wait_for_runtime_step_api(client)?;
        client.evaluate_json(&format!("window.__OUROFORGE__.setInput({input_json})"))?;
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneDocument {
    #[serde(rename = "schemaVersion")]
    #[serde(default = "scene_schema_v1")]
    pub schema_version: String,
    pub id: String,
    pub bounds: SceneBounds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<SceneRenderer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tilemaps: Vec<SceneTilemap>,
    #[serde(
        default,
        rename = "assetManifest",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_manifest: Option<AssetManifest>,
    #[serde(
        default,
        rename = "componentDefaults",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_defaults: Option<SceneComponentDefaults>,
    pub entities: Vec<SceneEntity>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneReloadValidationReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    #[serde(rename = "entityCount")]
    pub entity_count: usize,
    #[serde(rename = "assetManifest")]
    pub asset_manifest: Option<SceneReloadAssetManifestReport>,
    #[serde(rename = "resetState")]
    pub reset_state: Vec<String>,
    #[serde(rename = "preservedState")]
    pub preserved_state: Vec<String>,
    #[serde(rename = "unsupported")]
    pub unsupported: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneReloadAssetManifestReport {
    pub id: String,
    #[serde(rename = "assetCount")]
    pub asset_count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneBounds {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneRenderer {
    #[serde(default = "scene_renderer_v1")]
    pub version: String,
    #[serde(default)]
    pub camera: ScenePoint,
    pub viewport: SceneSize,
    #[serde(default = "default_renderer_background")]
    pub background: String,
    pub layers: Vec<SceneRenderLayer>,
    #[serde(default)]
    pub debug: SceneRendererDebug,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneRenderLayer {
    pub id: String,
    #[serde(default)]
    pub order: i64,
    #[serde(default = "default_layer_visible")]
    pub visible: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneRendererDebug {
    #[serde(default, rename = "showBounds")]
    pub show_bounds: bool,
    #[serde(default, rename = "showCamera")]
    pub show_camera: bool,
    #[serde(default, rename = "showEntityIds")]
    pub show_entity_ids: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneRenderOrderEntry {
    #[serde(rename = "entityId")]
    pub entity_id: String,
    pub layer: String,
    #[serde(rename = "layerOrder")]
    pub layer_order: i64,
    #[serde(rename = "spriteOrder")]
    pub sprite_order: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneTilemap {
    pub id: String,
    #[serde(rename = "tileSize")]
    pub tile_size: SceneSize,
    pub grid: SceneTilemapGrid,
    pub tiles: Vec<SceneTileDefinition>,
    pub layers: Vec<SceneTilemapLayer>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneTilemapGrid {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneTileDefinition {
    pub id: String,
    pub color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(default)]
    pub solid: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneTilemapLayer {
    pub id: String,
    #[serde(default)]
    pub order: i64,
    #[serde(default = "default_layer_visible")]
    pub visible: bool,
    pub data: Vec<Option<String>>,
    #[serde(
        default,
        rename = "collisionLayer",
        skip_serializing_if = "Option::is_none"
    )]
    pub collision_layer: Option<String>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneTilemapLayerOrderEntry {
    #[serde(rename = "tilemapId")]
    pub tilemap_id: String,
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub order: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEntity {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    pub sprite: SceneSprite,
    pub components: SceneComponents,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneSprite {
    pub color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default = "default_layer_visible")]
    pub visible: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneComponentDefaults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<ScenePoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub velocity: Option<ScenePoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<SceneSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controllable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneComponents {
    pub transform: ScenePoint,
    pub velocity: ScenePoint,
    pub size: SceneSize,
    pub controllable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collider: Option<SceneCollider>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<SceneAnimation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<SceneAudio>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ScenePoint {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneSize {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneCollider {
    #[serde(default = "aabb_collider_shape")]
    pub shape: String,
    #[serde(default = "static_collider_body")]
    pub body: String,
    #[serde(default)]
    pub offset: ScenePoint,
    pub size: SceneSize,
    #[serde(default)]
    pub sensor: bool,
    #[serde(default)]
    pub trigger: bool,
    #[serde(
        default,
        rename = "collisionGroup",
        skip_serializing_if = "Option::is_none"
    )]
    pub collision_group: Option<String>,
    #[serde(
        default,
        rename = "collisionMask",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collision_mask: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAnimation {
    pub mode: String,
    #[serde(rename = "frameDuration")]
    pub frame_duration: u32,
    #[serde(default = "default_animation_loop")]
    pub r#loop: bool,
    #[serde(default)]
    pub frames: Vec<SceneAnimationFrame>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clips: Vec<SceneAnimationClip>,
    #[serde(
        default,
        rename = "currentClip",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_clip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SceneAnimationState>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAnimationClip {
    pub id: String,
    #[serde(rename = "frameDuration")]
    pub frame_duration: u32,
    #[serde(default = "default_animation_loop")]
    pub r#loop: bool,
    pub frames: Vec<SceneAnimationFrame>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAnimationFrame {
    pub color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAnimationState {
    #[serde(
        default,
        rename = "currentClip",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_clip: Option<String>,
    #[serde(default, rename = "elapsedFrames")]
    pub elapsed_frames: u32,
    #[serde(default, rename = "frameIndex")]
    pub frame_index: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAudio {
    pub events: Vec<SceneAudioEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAudioEvent {
    pub name: String,
    pub trigger: String,
    #[serde(default = "default_audio_action")]
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

fn default_audio_action() -> String {
    "play".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEdit {
    pub entity_id: String,
    pub path: String,
    pub value: serde_json::Value,
}

pub const SUPPORTED_SCENE_EDIT_PATHS: &[&str] = &[
    "sprite.color",
    "components.transform.x",
    "components.transform.y",
    "components.velocity.x",
    "components.velocity.y",
    "components.size.width",
    "components.size.height",
    "components.controllable",
];

pub fn supported_scene_edit_paths() -> &'static [&'static str] {
    SUPPORTED_SCENE_EDIT_PATHS
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneHash {
    pub algorithm: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEditRollbackMetadata {
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    #[serde(rename = "restoreHash")]
    pub restore_hash: SceneHash,
    pub strategy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEditValidationResult {
    pub status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEditTransaction {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub id: String,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    pub edit: SceneEdit,
    #[serde(rename = "beforeSceneHash")]
    pub before_scene_hash: SceneHash,
    #[serde(rename = "afterSceneHash", skip_serializing_if = "Option::is_none")]
    pub after_scene_hash: Option<SceneHash>,
    #[serde(rename = "validationResult")]
    pub validation_result: SceneEditValidationResult,
    pub rollback: SceneEditRollbackMetadata,
}

pub fn hash_scene_document(scene: &SceneDocument) -> Result<SceneHash> {
    let value = canonical_json_value(json!(scene));
    let bytes = serde_json::to_vec(&value).context("failed to serialize canonical scene JSON")?;
    Ok(SceneHash {
        algorithm: "fnv1a64-canonical-json-v1".to_string(),
        value: format!("{:016x}", fnv1a64(&bytes)),
    })
}

pub fn write_scene_edit_transaction_artifact(
    path: impl AsRef<Path>,
    transaction: &SceneEditTransaction,
) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create transaction artifact directory {}",
                    parent.display()
                )
            })?;
        }
    }
    write_json(path, &json!(transaction))
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunTransactionProvenance {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "transactionArtifactPath")]
    pub transaction_artifact_path: String,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    #[serde(rename = "beforeSceneHash")]
    pub before_scene_hash: SceneHash,
    #[serde(rename = "afterSceneHash")]
    pub after_scene_hash: SceneHash,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneOnlyMutationOperation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "targetScenePath")]
    pub target_scene_path: String,
    pub edit: SceneEdit,
    #[serde(rename = "expectedBeforeSceneHash")]
    pub expected_before_scene_hash: SceneHash,
    #[serde(rename = "validationRequired")]
    pub validation_required: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SceneOnlyMutationValidation {
    pub status: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "targetScenePath")]
    pub target_scene_path: String,
    #[serde(rename = "beforeSceneHash")]
    pub before_scene_hash: SceneHash,
    #[serde(rename = "allowedPath")]
    pub allowed_path: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneOnlyMutationApplicationRecord {
    pub id: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "targetScenePath")]
    pub target_scene_path: String,
    #[serde(rename = "transactionArtifactPath")]
    pub transaction_artifact_path: String,
    #[serde(rename = "beforeSceneHash")]
    pub before_scene_hash: SceneHash,
    #[serde(rename = "afterSceneHash")]
    pub after_scene_hash: SceneHash,
    pub status: String,
    #[serde(rename = "createdAtUnixMs")]
    pub created_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct SceneOnlyMutationApplicationIndex {
    pub applications: Vec<SceneOnlyMutationApplicationRecord>,
}

pub fn validate_scene_only_mutation_operation(
    run_dir: impl AsRef<Path>,
    operation: &SceneOnlyMutationOperation,
) -> Result<SceneOnlyMutationValidation> {
    let run_dir = run_dir.as_ref();
    if operation.schema_version != "scene-only-mutation-v1" {
        return Err(anyhow!(
            "scene-only mutation schemaVersion must be scene-only-mutation-v1"
        ));
    }
    if !operation.validation_required {
        return Err(anyhow!(
            "scene-only mutation requires validationRequired=true"
        ));
    }
    require_text("scene-only mutation proposalId", &operation.proposal_id)?;
    require_text(
        "scene-only mutation targetScenePath",
        &operation.target_scene_path,
    )?;
    if !supported_scene_edit_paths().contains(&operation.edit.path.as_str()) {
        return Err(anyhow!(
            "scene-only mutation edit path is not allowed: {}",
            operation.edit.path
        ));
    }
    let proposals = read_mutation_proposals(run_dir)?;
    let proposal = proposals
        .proposals
        .iter()
        .find(|proposal| proposal.id == operation.proposal_id)
        .ok_or_else(|| {
            anyhow!(
                "scene-only mutation proposal id not found: {}",
                operation.proposal_id
            )
        })?;
    if proposal.status != "proposed" {
        return Err(anyhow!(
            "scene-only mutation requires a proposed mutation; found status {}",
            proposal.status
        ));
    }
    let scene = read_scene(&operation.target_scene_path)?;
    let before_scene_hash = hash_scene_document(&scene)?;
    if before_scene_hash != operation.expected_before_scene_hash {
        return Err(anyhow!(
            "scene-only mutation before hash mismatch; expected {}, found {}",
            operation.expected_before_scene_hash.value,
            before_scene_hash.value
        ));
    }
    let mut candidate_scene = scene.clone();
    apply_scene_edit(&mut candidate_scene, operation.edit.clone())?;
    validate_scene(&candidate_scene).context("scene-only mutation candidate validation failed")?;
    Ok(SceneOnlyMutationValidation {
        status: "passed".to_string(),
        proposal_id: operation.proposal_id.clone(),
        target_scene_path: operation.target_scene_path.clone(),
        before_scene_hash,
        allowed_path: true,
    })
}

pub fn apply_scene_only_mutation_operation(
    run_dir: impl AsRef<Path>,
    operation: &SceneOnlyMutationOperation,
    transaction_output: impl AsRef<Path>,
) -> Result<SceneEditTransaction> {
    let run_dir = run_dir.as_ref();
    let transaction_output = transaction_output.as_ref();
    if operation.schema_version != "scene-only-mutation-v1" {
        return Err(anyhow!(
            "scene-only mutation schemaVersion must be scene-only-mutation-v1"
        ));
    }
    if !operation.validation_required {
        return Err(anyhow!(
            "scene-only mutation requires validationRequired=true"
        ));
    }
    if !supported_scene_edit_paths().contains(&operation.edit.path.as_str()) {
        return Err(anyhow!(
            "scene-only mutation edit path is not allowed: {}",
            operation.edit.path
        ));
    }
    let proposals = read_mutation_proposals(run_dir)?;
    let proposal = proposals
        .proposals
        .iter()
        .find(|proposal| proposal.id == operation.proposal_id)
        .ok_or_else(|| {
            anyhow!(
                "scene-only mutation proposal id not found: {}",
                operation.proposal_id
            )
        })?;
    if proposal.status != "proposed" {
        return Err(anyhow!(
            "scene-only mutation requires a proposed mutation; found status {}",
            proposal.status
        ));
    }
    let scene = read_scene(&operation.target_scene_path)?;
    let before_scene_hash = hash_scene_document(&scene)?;
    if before_scene_hash != operation.expected_before_scene_hash {
        return Err(anyhow!(
            "scene-only mutation before hash mismatch; expected {}, found {}",
            operation.expected_before_scene_hash.value,
            before_scene_hash.value
        ));
    }
    let transaction =
        preview_scene_edit_transaction(&operation.target_scene_path, operation.edit.clone())?;
    write_scene_edit_transaction_artifact(transaction_output, &transaction)?;
    if transaction.validation_result.status != "passed" {
        return Err(anyhow!(
            "scene-only mutation transaction failed validation; artifact written to {}",
            transaction_output.display()
        ));
    }
    edit_scene(&operation.target_scene_path, operation.edit.clone())?;
    let after_scene_hash = transaction
        .after_scene_hash
        .clone()
        .ok_or_else(|| anyhow!("passed scene-only transaction missing afterSceneHash"))?;
    let application = append_scene_only_mutation_application(
        run_dir,
        operation,
        &transaction,
        transaction_output,
        &after_scene_hash,
    )?;
    append_ledger_event(
        run_dir,
        "mutation.scene_applied",
        "mutation-cli",
        json!({
            "proposal_id": operation.proposal_id,
            "application_id": application.id,
            "transaction_id": transaction.id,
            "transaction_artifact_path": transaction_output.to_string_lossy(),
            "target_scene_path": operation.target_scene_path,
            "before_scene_hash": transaction.before_scene_hash,
            "after_scene_hash": after_scene_hash
        }),
    )?;
    Ok(transaction)
}

fn append_scene_only_mutation_application(
    run_dir: &Path,
    operation: &SceneOnlyMutationOperation,
    transaction: &SceneEditTransaction,
    transaction_output: &Path,
    after_scene_hash: &SceneHash,
) -> Result<SceneOnlyMutationApplicationRecord> {
    let mut index = read_scene_only_mutation_applications(run_dir)?;
    let record = SceneOnlyMutationApplicationRecord {
        id: format!(
            "scene-application-{}-{}",
            unix_millis()?,
            index.applications.len() + 1
        ),
        proposal_id: operation.proposal_id.clone(),
        transaction_id: transaction.id.clone(),
        target_scene_path: operation.target_scene_path.clone(),
        transaction_artifact_path: transaction_output.to_string_lossy().to_string(),
        before_scene_hash: transaction.before_scene_hash.clone(),
        after_scene_hash: after_scene_hash.clone(),
        status: "applied".to_string(),
        created_at_unix_ms: unix_millis()?,
    };
    index.applications.push(record.clone());
    write_scene_only_mutation_applications(run_dir, &index)?;
    Ok(record)
}

fn read_scene_only_mutation_applications(
    run_dir: impl AsRef<Path>,
) -> Result<SceneOnlyMutationApplicationIndex> {
    let path = run_dir.as_ref().join("mutation/scene-applications.json");
    if !path.is_file() {
        return Ok(SceneOnlyMutationApplicationIndex::default());
    }
    let input = fs::read_to_string(&path)
        .with_context(|| format!("failed to read scene applications {}", path.display()))?;
    serde_json::from_str(&input)
        .with_context(|| format!("failed to parse scene applications {}", path.display()))
}

fn write_scene_only_mutation_applications(
    run_dir: impl AsRef<Path>,
    index: &SceneOnlyMutationApplicationIndex,
) -> Result<()> {
    let path = run_dir.as_ref().join("mutation/scene-applications.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create scene application directory {}",
                parent.display()
            )
        })?;
    }
    write_json(&path, &json!(index))
}

pub fn read_scene_edit_transaction_artifact(
    path: impl AsRef<Path>,
) -> Result<SceneEditTransaction> {
    let path = path.as_ref();
    let value = read_json_value(path)?;
    serde_json::from_value(value)
        .with_context(|| format!("failed to parse scene edit transaction {}", path.display()))
}

pub fn run_transaction_provenance_from_artifact(
    path: impl AsRef<Path>,
) -> Result<RunTransactionProvenance> {
    let path = path.as_ref();
    let transaction = read_scene_edit_transaction_artifact(path)?;
    if transaction.validation_result.status != "passed" {
        return Err(anyhow!(
            "run transaction provenance requires a passed transaction: {}",
            transaction.id
        ));
    }
    let after_scene_hash = transaction.after_scene_hash.clone().ok_or_else(|| {
        anyhow!(
            "run transaction provenance requires afterSceneHash for transaction: {}",
            transaction.id
        )
    })?;
    Ok(RunTransactionProvenance {
        transaction_id: transaction.id,
        transaction_artifact_path: path.to_string_lossy().to_string(),
        scene_path: transaction.scene_path,
        before_scene_hash: transaction.before_scene_hash,
        after_scene_hash,
    })
}

pub fn bind_run_transaction_provenance(
    run_dir: impl AsRef<Path>,
    transaction_path: impl AsRef<Path>,
) -> Result<RunTransactionProvenance> {
    let run_dir = run_dir.as_ref();
    let provenance = run_transaction_provenance_from_artifact(&transaction_path)?;
    let scene = read_scene(&provenance.scene_path).with_context(|| {
        format!(
            "failed to read transaction source scene {}",
            provenance.scene_path
        )
    })?;
    let current_hash = hash_scene_document(&scene)?;
    if current_hash != provenance.after_scene_hash {
        return Err(anyhow!(
            "transaction source scene hash mismatch for {}; expected afterSceneHash {}, found {}",
            provenance.transaction_id,
            provenance.after_scene_hash.value,
            current_hash.value
        ));
    }
    let run_path = run_dir.join("run.json");
    let mut run = read_json_value(&run_path)?;
    let run_object = run
        .as_object_mut()
        .ok_or_else(|| anyhow!("run.json must be a JSON object"))?;
    run_object.insert(
        "transaction_provenance".to_string(),
        json!(provenance.clone()),
    );
    write_json_atomic(&run_path, &run)?;
    append_ledger_event(
        run_dir,
        "run.transaction_bound",
        "run-cli",
        json!({
            "transaction_id": provenance.transaction_id,
            "transaction_artifact_path": provenance.transaction_artifact_path,
            "scene_path": provenance.scene_path,
            "before_scene_hash": provenance.before_scene_hash,
            "after_scene_hash": provenance.after_scene_hash
        }),
    )?;
    Ok(provenance)
}

pub fn preview_scene_edit_transaction(
    scene_path: impl AsRef<Path>,
    edit: SceneEdit,
) -> Result<SceneEditTransaction> {
    let scene_path = scene_path.as_ref();
    validate_path_component("scene edit entity", &edit.entity_id)?;
    require_text("scene edit path", &edit.path)?;
    let before = read_scene(scene_path)?;
    let before_hash = hash_scene_document(&before)?;
    let mut after = before.clone();
    let validation_error = apply_scene_edit(&mut after, edit.clone())
        .and_then(|_| validate_scene(&after))
        .err()
        .map(|error| error.to_string());
    let after_hash = if validation_error.is_none() {
        Some(hash_scene_document(&after)?)
    } else {
        None
    };
    let transaction_seed = json!({
        "scenePath": scene_path.to_string_lossy(),
        "edit": edit,
        "beforeSceneHash": before_hash,
        "afterSceneHash": after_hash
    });
    let transaction_id = format!(
        "scene-edit-{}",
        fnv1a64(&serde_json::to_vec(&canonical_json_value(
            transaction_seed
        ))?)
    );
    Ok(SceneEditTransaction {
        schema_version: "ouroforge.scene-edit-transaction.v1".to_string(),
        id: transaction_id,
        scene_path: scene_path.to_string_lossy().to_string(),
        edit,
        before_scene_hash: before_hash.clone(),
        after_scene_hash: after_hash,
        validation_result: match validation_error {
            Some(error) => SceneEditValidationResult {
                status: "failed".to_string(),
                errors: vec![error],
            },
            None => SceneEditValidationResult {
                status: "passed".to_string(),
                errors: Vec::new(),
            },
        },
        rollback: SceneEditRollbackMetadata {
            scene_path: scene_path.to_string_lossy().to_string(),
            restore_hash: before_hash,
            strategy: "restore scene file content matching beforeSceneHash".to_string(),
        },
    })
}

fn canonical_json_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.into_iter().map(canonical_json_value).collect())
        }
        serde_json::Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut entries = map.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            for (key, value) in entries {
                sorted.insert(key, canonical_json_value(value));
            }
            serde_json::Value::Object(sorted)
        }
        other => other,
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn scene_schema_v1() -> String {
    "1".to_string()
}

fn scene_renderer_v1() -> String {
    "1".to_string()
}

fn default_renderer_background() -> String {
    "#172532".to_string()
}

fn default_layer_visible() -> bool {
    true
}

fn aabb_collider_shape() -> String {
    "aabb".to_string()
}

fn static_collider_body() -> String {
    "static".to_string()
}

fn default_animation_loop() -> bool {
    true
}

fn empty_json_object() -> serde_json::Value {
    json!({})
}

pub fn read_scene(scene_path: impl AsRef<Path>) -> Result<SceneDocument> {
    let scene_path = scene_path.as_ref();
    let input = fs::read_to_string(scene_path)
        .with_context(|| format!("failed to read scene {}", scene_path.display()))?;
    let scene: SceneDocument = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse scene {}", scene_path.display()))?;
    validate_scene(&scene)?;
    Ok(scene)
}

pub fn validate_scene_reload(scene_path: impl AsRef<Path>) -> Result<SceneReloadValidationReport> {
    let scene = read_scene(scene_path)?;
    Ok(SceneReloadValidationReport {
        schema_version: "ouroforge.scene-reload.v0".to_string(),
        scene_id: scene.id.clone(),
        entity_count: scene.entities.len(),
        asset_manifest: scene.asset_manifest.as_ref().map(|manifest| {
            SceneReloadAssetManifestReport {
                id: manifest.id.clone(),
                asset_count: manifest.assets.len(),
            }
        }),
        reset_state: vec![
            "tick".to_string(),
            "entities".to_string(),
            "collisions".to_string(),
            "collisionEvents".to_string(),
            "audioEvents".to_string(),
            "snapshots".to_string(),
            "renderer".to_string(),
            "tilemaps".to_string(),
            "assetManifest".to_string(),
        ],
        preserved_state: vec![
            "runtime API".to_string(),
            "fixedDeltaMs".to_string(),
            "bounded event history".to_string(),
        ],
        unsupported: vec![
            "source code reload".to_string(),
            "live code HMR".to_string(),
            "filesystem watchers".to_string(),
            "direct browser file writes".to_string(),
            "editor persistence".to_string(),
            "native shell".to_string(),
        ],
    })
}

pub fn edit_scene(scene_path: impl AsRef<Path>, edit: SceneEdit) -> Result<SceneDocument> {
    let scene_path = scene_path.as_ref();
    validate_path_component("scene edit entity", &edit.entity_id)?;
    require_text("scene edit path", &edit.path)?;
    let mut scene = read_scene(scene_path)?;
    apply_scene_edit(&mut scene, edit)?;
    validate_scene(&scene)?;
    write_json(scene_path, &json!(scene))?;
    Ok(scene)
}

fn validate_scene(scene: &SceneDocument) -> Result<()> {
    if scene.schema_version != "1" {
        return Err(anyhow!("scene schemaVersion must be 1 for scene schema v1"));
    }
    validate_path_component("scene id", &scene.id)?;
    if scene.bounds.width <= 0 || scene.bounds.height <= 0 {
        return Err(anyhow!("scene bounds must be positive"));
    }
    if let Some(renderer) = &scene.renderer {
        validate_scene_renderer(scene, renderer)?;
    }
    if let Some(manifest) = &scene.asset_manifest {
        manifest
            .validate()
            .context("scene assetManifest is invalid")?;
    }
    validate_scene_tilemaps(&scene.tilemaps, scene.asset_manifest.as_ref())?;
    validate_scene_metadata("scene metadata", &scene.metadata)?;
    if scene.entities.is_empty() {
        return Err(anyhow!("scene entities must not be empty"));
    }
    if let Some(defaults) = &scene.component_defaults {
        validate_scene_component_defaults(defaults)?;
    }
    validate_scene_composition(scene)?;
    let mut ids = std::collections::BTreeSet::new();
    for entity in &scene.entities {
        validate_path_component("scene entity id", &entity.id)?;
        if let Some(parent) = &entity.parent {
            validate_path_component(&format!("scene entity {} parent", entity.id), parent)?;
        }
        if !ids.insert(entity.id.clone()) {
            return Err(anyhow!("duplicate scene entity id: {}", entity.id));
        }
        validate_scene_color(&entity.sprite.color)?;
        if let Some(asset) = &entity.sprite.asset {
            validate_scene_asset_ref("scene sprite asset", asset, scene.asset_manifest.as_ref())?;
        }
        if let Some(layer) = &entity.sprite.layer {
            validate_path_component(&format!("scene entity {} sprite layer", entity.id), layer)?;
            if let Some(renderer) = &scene.renderer {
                if !renderer
                    .layers
                    .iter()
                    .any(|candidate| candidate.id == *layer)
                {
                    return Err(anyhow!(
                        "scene entity {} sprite layer references unknown renderer layer: {}",
                        entity.id,
                        layer
                    ));
                }
            }
        }
        if entity.components.size.width <= 0 || entity.components.size.height <= 0 {
            return Err(anyhow!("scene entity {} size must be positive", entity.id));
        }
        if let Some(collider) = &entity.components.collider {
            validate_scene_collider(&entity.id, collider)?;
        }
        if let Some(animation) = &entity.components.animation {
            validate_scene_animation(&entity.id, animation, scene.asset_manifest.as_ref())?;
        }
        if let Some(audio) = &entity.components.audio {
            validate_scene_audio(&entity.id, audio, scene.asset_manifest.as_ref())?;
        }
        validate_scene_tags(&entity.id, &entity.tags)?;
        validate_scene_metadata(
            &format!("scene entity {} metadata", entity.id),
            &entity.metadata,
        )?;
    }
    Ok(())
}

fn validate_scene_component_defaults(defaults: &SceneComponentDefaults) -> Result<()> {
    if let Some(size) = &defaults.size {
        if size.width <= 0 || size.height <= 0 {
            return Err(anyhow!("scene componentDefaults size must be positive"));
        }
    }
    Ok(())
}

fn validate_scene_composition(scene: &SceneDocument) -> Result<()> {
    let ids = scene
        .entities
        .iter()
        .map(|entity| entity.id.as_str())
        .collect::<BTreeSet<_>>();
    for entity in &scene.entities {
        let Some(parent) = &entity.parent else {
            continue;
        };
        if parent == &entity.id {
            return Err(anyhow!(
                "scene entity {} parent must not reference itself",
                entity.id
            ));
        }
        if !ids.contains(parent.as_str()) {
            return Err(anyhow!(
                "scene entity {} references missing parent: {}",
                entity.id,
                parent
            ));
        }
    }
    for entity in &scene.entities {
        let mut seen = BTreeSet::new();
        let mut current = entity.parent.as_deref();
        while let Some(parent_id) = current {
            if !seen.insert(parent_id) {
                return Err(anyhow!(
                    "scene composition cycle detected at entity {}",
                    entity.id
                ));
            }
            current = scene
                .entities
                .iter()
                .find(|candidate| candidate.id == parent_id)
                .and_then(|parent| parent.parent.as_deref());
        }
    }
    Ok(())
}

fn validate_scene_tilemaps(
    tilemaps: &[SceneTilemap],
    manifest: Option<&AssetManifest>,
) -> Result<()> {
    let mut tilemap_ids = std::collections::BTreeSet::new();
    for tilemap in tilemaps {
        validate_path_component("scene tilemap id", &tilemap.id)?;
        if !tilemap_ids.insert(tilemap.id.clone()) {
            return Err(anyhow!("duplicate scene tilemap id: {}", tilemap.id));
        }
        if tilemap.tile_size.width <= 0 || tilemap.tile_size.height <= 0 {
            return Err(anyhow!(
                "scene tilemap {} tileSize must be positive",
                tilemap.id
            ));
        }
        if tilemap.grid.width == 0 || tilemap.grid.height == 0 {
            return Err(anyhow!(
                "scene tilemap {} grid dimensions must be positive",
                tilemap.id
            ));
        }
        let expected_cells = tilemap
            .grid
            .width
            .checked_mul(tilemap.grid.height)
            .ok_or_else(|| anyhow!("scene tilemap {} grid dimensions overflow", tilemap.id))?;
        let mut tile_ids = std::collections::BTreeSet::new();
        for tile in &tilemap.tiles {
            validate_path_component(&format!("scene tilemap {} tile id", tilemap.id), &tile.id)?;
            if !tile_ids.insert(tile.id.clone()) {
                return Err(anyhow!(
                    "duplicate scene tilemap {} tile id: {}",
                    tilemap.id,
                    tile.id
                ));
            }
            validate_scene_color(&tile.color).with_context(|| {
                format!(
                    "scene tilemap {} tile {} color is invalid",
                    tilemap.id, tile.id
                )
            })?;
            if let Some(asset) = &tile.asset {
                validate_scene_asset_ref("scene tilemap tile asset", asset, manifest)?;
            }
        }
        if tile_ids.is_empty() {
            return Err(anyhow!(
                "scene tilemap {} tiles must not be empty",
                tilemap.id
            ));
        }
        let mut layer_ids = std::collections::BTreeSet::new();
        for layer in &tilemap.layers {
            validate_path_component(&format!("scene tilemap {} layer id", tilemap.id), &layer.id)?;
            if !layer_ids.insert(layer.id.clone()) {
                return Err(anyhow!(
                    "duplicate scene tilemap {} layer id: {}",
                    tilemap.id,
                    layer.id
                ));
            }
            if layer.data.len() != expected_cells {
                return Err(anyhow!(
                    "scene tilemap {} layer {} data length must equal grid cell count {}",
                    tilemap.id,
                    layer.id,
                    expected_cells
                ));
            }
            for tile_id in layer.data.iter().flatten() {
                if !tile_ids.contains(tile_id) {
                    return Err(anyhow!(
                        "scene tilemap {} layer {} references unknown tile id: {}",
                        tilemap.id,
                        layer.id,
                        tile_id
                    ));
                }
            }
            validate_scene_metadata(
                &format!("scene tilemap {} layer {} metadata", tilemap.id, layer.id),
                &layer.metadata,
            )?;
        }
        if tilemap.layers.is_empty() {
            return Err(anyhow!(
                "scene tilemap {} layers must not be empty",
                tilemap.id
            ));
        }
        for layer in &tilemap.layers {
            if let Some(collision_layer) = &layer.collision_layer {
                validate_path_component(
                    &format!("scene tilemap {} collision layer ref", tilemap.id),
                    collision_layer,
                )?;
                if !layer_ids.contains(collision_layer) {
                    return Err(anyhow!(
                        "scene tilemap {} layer {} references unknown collisionLayer: {}",
                        tilemap.id,
                        layer.id,
                        collision_layer
                    ));
                }
            }
        }
        validate_scene_metadata(
            &format!("scene tilemap {} metadata", tilemap.id),
            &tilemap.metadata,
        )?;
    }
    Ok(())
}

pub fn scene_tilemap_layer_order(scene: &SceneDocument) -> Vec<SceneTilemapLayerOrderEntry> {
    let mut entries = scene
        .tilemaps
        .iter()
        .flat_map(|tilemap| {
            tilemap
                .layers
                .iter()
                .filter(|layer| layer.visible)
                .map(|layer| SceneTilemapLayerOrderEntry {
                    tilemap_id: tilemap.id.clone(),
                    layer_id: layer.id.clone(),
                    order: layer.order,
                })
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        (left.order, left.tilemap_id.as_str(), left.layer_id.as_str()).cmp(&(
            right.order,
            right.tilemap_id.as_str(),
            right.layer_id.as_str(),
        ))
    });
    entries
}

fn validate_scene_renderer(scene: &SceneDocument, renderer: &SceneRenderer) -> Result<()> {
    if renderer.version != "1" {
        return Err(anyhow!("scene renderer version must be 1"));
    }
    if renderer.viewport.width <= 0 || renderer.viewport.height <= 0 {
        return Err(anyhow!("scene renderer viewport must be positive"));
    }
    if renderer.viewport.width > scene.bounds.width
        || renderer.viewport.height > scene.bounds.height
    {
        return Err(anyhow!(
            "scene renderer viewport must fit inside scene bounds"
        ));
    }
    validate_scene_color(&renderer.background)
        .context("scene renderer background color is invalid")?;
    if renderer.layers.is_empty() {
        return Err(anyhow!("scene renderer layers must not be empty"));
    }
    let mut layer_ids = std::collections::BTreeSet::new();
    for layer in &renderer.layers {
        validate_path_component("scene renderer layer id", &layer.id)?;
        if !layer_ids.insert(layer.id.clone()) {
            return Err(anyhow!("duplicate scene renderer layer id: {}", layer.id));
        }
    }
    Ok(())
}

pub fn scene_render_order(scene: &SceneDocument) -> Vec<SceneRenderOrderEntry> {
    let layer_order = scene
        .renderer
        .as_ref()
        .map(|renderer| {
            renderer
                .layers
                .iter()
                .map(|layer| (layer.id.as_str(), layer.order))
                .collect::<std::collections::BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    let mut entries = scene
        .entities
        .iter()
        .filter(|entity| entity.sprite.visible)
        .map(|entity| {
            let layer = entity
                .sprite
                .layer
                .clone()
                .unwrap_or_else(|| "default".to_string());
            let layer_order = layer_order.get(layer.as_str()).copied().unwrap_or(0);
            SceneRenderOrderEntry {
                entity_id: entity.id.clone(),
                layer,
                layer_order,
                sprite_order: entity.sprite.order.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        (left.layer_order, left.sprite_order, left.entity_id.as_str()).cmp(&(
            right.layer_order,
            right.sprite_order,
            right.entity_id.as_str(),
        ))
    });
    entries
}

fn validate_scene_asset_ref(
    field: &str,
    value: &str,
    manifest: Option<&AssetManifest>,
) -> Result<()> {
    if let Some(manifest) = manifest {
        validate_path_component(field, value)?;
        if manifest.assets.iter().any(|asset| asset.id == value) {
            Ok(())
        } else {
            Err(anyhow!(
                "{field} references unknown asset manifest id: {value}"
            ))
        }
    } else {
        validate_scene_local_asset_path(field, value)
    }
}

fn validate_scene_collider(entity_id: &str, collider: &SceneCollider) -> Result<()> {
    if collider.shape != "aabb" {
        return Err(anyhow!(
            "scene entity {entity_id} collider shape must be aabb"
        ));
    }
    if !matches!(collider.body.as_str(), "static" | "dynamic" | "kinematic") {
        return Err(anyhow!(
            "scene entity {entity_id} collider body must be static, dynamic, or kinematic"
        ));
    }
    if collider.size.width <= 0 || collider.size.height <= 0 {
        return Err(anyhow!(
            "scene entity {entity_id} collider size must be positive"
        ));
    }
    if let Some(group) = &collider.collision_group {
        validate_path_component(
            &format!("scene entity {entity_id} collider collisionGroup"),
            group,
        )?;
    }
    let mut masks = std::collections::BTreeSet::new();
    for mask in &collider.collision_mask {
        validate_path_component(
            &format!("scene entity {entity_id} collider collisionMask"),
            mask,
        )?;
        if !masks.insert(mask) {
            return Err(anyhow!(
                "duplicate scene entity {entity_id} collider collisionMask: {mask}"
            ));
        }
    }
    Ok(())
}

fn validate_scene_animation(
    entity_id: &str,
    animation: &SceneAnimation,
    manifest: Option<&AssetManifest>,
) -> Result<()> {
    if animation.mode != "sprite_frame" {
        return Err(anyhow!(
            "scene entity {entity_id} animation mode must be sprite_frame"
        ));
    }
    if animation.frame_duration == 0 {
        return Err(anyhow!(
            "scene entity {entity_id} animation frameDuration must be greater than 0"
        ));
    }
    if animation.frames.is_empty() && animation.clips.is_empty() {
        return Err(anyhow!(
            "scene entity {entity_id} animation frames or clips must not be empty"
        ));
    }
    for frame in &animation.frames {
        validate_scene_animation_frame(entity_id, "default", frame, manifest)?;
    }

    let mut clip_ids = std::collections::BTreeSet::new();
    for clip in &animation.clips {
        validate_path_component(
            &format!("scene entity {entity_id} animation clip id"),
            &clip.id,
        )?;
        if !clip_ids.insert(clip.id.clone()) {
            return Err(anyhow!(
                "duplicate scene entity {entity_id} animation clip id: {}",
                clip.id
            ));
        }
        if clip.frame_duration == 0 {
            return Err(anyhow!(
                "scene entity {entity_id} animation clip {} frameDuration must be greater than 0",
                clip.id
            ));
        }
        if clip.frames.is_empty() {
            return Err(anyhow!(
                "scene entity {entity_id} animation clip {} frames must not be empty",
                clip.id
            ));
        }
        for frame in &clip.frames {
            validate_scene_animation_frame(entity_id, &clip.id, frame, manifest)?;
        }
    }
    if let Some(current_clip) = &animation.current_clip {
        validate_path_component(
            &format!("scene entity {entity_id} animation currentClip"),
            current_clip,
        )?;
        if !animation.clips.is_empty() && !clip_ids.contains(current_clip) {
            return Err(anyhow!(
                "scene entity {entity_id} animation currentClip references unknown clip: {current_clip}"
            ));
        }
    }
    if let Some(state) = &animation.state {
        if let Some(current_clip) = &state.current_clip {
            validate_path_component(
                &format!("scene entity {entity_id} animation state currentClip"),
                current_clip,
            )?;
            if !animation.clips.is_empty() && !clip_ids.contains(current_clip) {
                return Err(anyhow!(
                    "scene entity {entity_id} animation state currentClip references unknown clip: {current_clip}"
                ));
            }
        }
        let frame_count = animation
            .current_clip
            .as_ref()
            .and_then(|clip_id| animation.clips.iter().find(|clip| clip.id == *clip_id))
            .or_else(|| animation.clips.first())
            .map(|clip| clip.frames.len())
            .unwrap_or(animation.frames.len());
        if frame_count > 0 && state.frame_index >= frame_count {
            return Err(anyhow!(
                "scene entity {entity_id} animation state frameIndex must be within current frame list"
            ));
        }
    }
    Ok(())
}

fn validate_scene_animation_frame(
    entity_id: &str,
    clip_id: &str,
    frame: &SceneAnimationFrame,
    manifest: Option<&AssetManifest>,
) -> Result<()> {
    validate_scene_color(&frame.color).with_context(|| {
        format!("scene entity {entity_id} animation clip {clip_id} frame color is invalid")
    })?;
    if let Some(asset) = &frame.asset {
        validate_scene_asset_ref("scene animation frame asset", asset, manifest)?;
    }
    Ok(())
}

fn validate_scene_audio(
    entity_id: &str,
    audio: &SceneAudio,
    manifest: Option<&AssetManifest>,
) -> Result<()> {
    if audio.events.is_empty() {
        return Err(anyhow!(
            "scene entity {entity_id} audio events must not be empty"
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    for event in &audio.events {
        validate_path_component(
            &format!("scene entity {entity_id} audio event name"),
            &event.name,
        )?;
        if !names.insert(event.name.clone()) {
            return Err(anyhow!(
                "duplicate scene entity {entity_id} audio event: {}",
                event.name
            ));
        }
        if event.trigger != "scene_loaded" {
            return Err(anyhow!(
                "scene entity {entity_id} audio trigger must be scene_loaded"
            ));
        }
        if !matches!(event.action.as_str(), "play" | "stop") {
            return Err(anyhow!(
                "scene entity {entity_id} audio event {} action must be play or stop",
                event.name
            ));
        }
        if event.action == "play" && event.asset.is_none() {
            return Err(anyhow!(
                "scene entity {entity_id} audio event {} play action requires an asset ref",
                event.name
            ));
        }
        if let Some(asset) = &event.asset {
            validate_scene_asset_ref("scene audio asset", asset, manifest)?;
        }
    }
    Ok(())
}

fn validate_scene_tags(entity_id: &str, tags: &[String]) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for tag in tags {
        validate_path_component(&format!("scene entity {entity_id} tag"), tag)?;
        if !seen.insert(tag) {
            return Err(anyhow!("duplicate scene entity {entity_id} tag: {tag}"));
        }
    }
    Ok(())
}

fn validate_scene_metadata(field: &str, metadata: &serde_json::Value) -> Result<()> {
    if metadata.is_object() {
        Ok(())
    } else {
        Err(anyhow!("{field} must be a JSON object"))
    }
}

fn validate_scene_local_asset_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lowered = value.to_ascii_lowercase();
    if lowered.starts_with("http://") || lowered.starts_with("https://") {
        return Err(anyhow!("{field} must be a local static asset path"));
    }
    if !value.starts_with("assets/") {
        return Err(anyhow!("{field} must start with assets/"));
    }
    if !value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '-' | '_'))
    {
        return Err(anyhow!(
            "{field} may only contain ASCII letters, numbers, '/', '.', '-' or '_'"
        ));
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "{field} must stay inside the local scene asset tree"
                ))
            }
        }
    }
    Ok(())
}

fn apply_scene_edit(scene: &mut SceneDocument, edit: SceneEdit) -> Result<()> {
    let entity = scene
        .entities
        .iter_mut()
        .find(|entity| entity.id == edit.entity_id)
        .ok_or_else(|| anyhow!("scene entity not found: {}", edit.entity_id))?;
    match edit.path.as_str() {
        "sprite.color" => entity.sprite.color = scene_edit_string(&edit.value, &edit.path)?,
        "components.transform.x" => {
            entity.components.transform.x = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.transform.y" => {
            entity.components.transform.y = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.velocity.x" => {
            entity.components.velocity.x = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.velocity.y" => {
            entity.components.velocity.y = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.size.width" => {
            entity.components.size.width = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.size.height" => {
            entity.components.size.height = scene_edit_i64(&edit.value, &edit.path)?
        }
        "components.controllable" => {
            entity.components.controllable = scene_edit_bool(&edit.value, &edit.path)?
        }
        _ => {
            return Err(anyhow!(
                "unsupported scene edit path `{}`; supported paths are {}",
                edit.path,
                supported_scene_edit_paths().join(", ")
            ));
        }
    }
    Ok(())
}

fn scene_edit_i64(value: &serde_json::Value, path: &str) -> Result<i64> {
    value
        .as_i64()
        .ok_or_else(|| anyhow!("scene edit path `{path}` requires an integer value"))
}

fn scene_edit_bool(value: &serde_json::Value, path: &str) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| anyhow!("scene edit path `{path}` requires a boolean value"))
}

fn scene_edit_string(value: &serde_json::Value, path: &str) -> Result<String> {
    let value = value
        .as_str()
        .ok_or_else(|| anyhow!("scene edit path `{path}` requires a string value"))?
        .to_string();
    validate_scene_color(&value)?;
    Ok(value)
}

fn validate_scene_color(color: &str) -> Result<()> {
    let Some(hex) = color.strip_prefix('#') else {
        return Err(anyhow!("scene sprite color must be a #RRGGBB hex value"));
    };
    if hex.len() != 6 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("scene sprite color must be a #RRGGBB hex value"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardSummary {
    pub id: String,
    pub run_dir: PathBuf,
    pub seed_id: String,
    pub seed_title: String,
    pub run_status: String,
    pub verdict_status: String,
    pub scenario_status: String,
    pub created_at_unix_ms: u128,
    pub evidence_count: usize,
    pub mutation_count: usize,
    pub worker_count: usize,
    pub evidence_categories: Vec<RunDashboardCategorySummary>,
    pub journal_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardCategorySummary {
    pub id: String,
    pub label: String,
    pub count: usize,
    pub missing_count: usize,
    pub malformed_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardArtifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub metadata: serde_json::Value,
    pub value: Option<serde_json::Value>,
    pub exists: bool,
    pub read_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReadModel {
    pub summary: RunDashboardSummary,
    pub run: serde_json::Value,
    pub verdict: serde_json::Value,
    pub journal: String,
    pub journal_view: RunDashboardJournal,
    pub evidence: Vec<EvidenceArtifact>,
    pub screenshots: Vec<RunDashboardArtifact>,
    pub console_logs: Vec<RunDashboardArtifact>,
    pub cdp_trace_summaries: Vec<RunDashboardArtifact>,
    pub world_states: Vec<RunDashboardArtifact>,
    pub frame_metrics: Vec<RunDashboardArtifact>,
    pub performance_metrics: Vec<RunDashboardArtifact>,
    pub scenario_results: Vec<RunDashboardArtifact>,
    pub mutation_artifacts: Vec<RunDashboardArtifact>,
    pub mutation_lifecycle: RunDashboardMutationLifecycle,
    pub replay: RunDashboardReplay,
    pub comparison: RunDashboardComparison,
    pub transaction_provenance: Option<RunTransactionProvenance>,
    pub engine_summaries: RunDashboardEngineSummaries,
    pub evidence_categories: Vec<RunDashboardCategorySummary>,
    pub mutations: Vec<MutationProposal>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardJournal {
    pub path: String,
    pub exists: bool,
    pub read_error: Option<String>,
    pub summary: String,
    pub entries: Vec<RunDashboardJournalEntry>,
    pub evidence_refs: Vec<String>,
    pub verdict_refs: Vec<String>,
    pub mutation_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardJournalEntry {
    pub id: String,
    pub heading: String,
    pub level: usize,
    pub category: String,
    pub body: String,
    pub evidence_refs: Vec<String>,
    pub verdict_refs: Vec<String>,
    pub mutation_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardMutationLifecycle {
    pub terminal_state: String,
    pub stages: Vec<RunDashboardMutationLifecycleStage>,
    pub command_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardMutationLifecycleStage {
    pub id: String,
    pub label: String,
    pub state: String,
    pub artifact_path: Option<String>,
    pub record_count: usize,
    pub evidence_refs: Vec<String>,
    pub records: Vec<serde_json::Value>,
    pub read_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReplay {
    pub present: bool,
    pub empty_state: String,
    pub sequences: Vec<RunDashboardReplaySequence>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReplaySequence {
    pub id: String,
    pub source: String,
    pub scenario_id: Option<String>,
    pub replay_path: String,
    pub event_count: usize,
    pub frames: Vec<u64>,
    pub first_frame: Option<u64>,
    pub last_frame: Option<u64>,
    pub evidence_refs: Vec<String>,
    pub checkpoints: Vec<RunDashboardReplayCheckpoint>,
    pub read_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReplayCheckpoint {
    pub id: String,
    pub label: String,
    pub frame: Option<u64>,
    pub tick: Option<u64>,
    pub world_state_path: Option<String>,
    pub frame_stats_path: Option<String>,
    pub world_state: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardComparison {
    pub present: bool,
    pub empty_state: String,
    pub artifacts: Vec<RunDashboardComparisonArtifact>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardComparisonArtifact {
    pub id: String,
    pub path: String,
    pub exists: bool,
    pub read_error: Option<String>,
    pub before_run_id: Option<String>,
    pub after_run_id: Option<String>,
    pub classification: Option<String>,
    pub deltas: serde_json::Value,
    pub semantic: serde_json::Value,
    pub evidence_refs: Vec<String>,
    pub unsupported: Vec<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardEngineSummaries {
    pub present: bool,
    pub empty_state: String,
    pub source_world_state: Option<String>,
    pub scene: serde_json::Value,
    pub renderer: serde_json::Value,
    pub tilemaps: serde_json::Value,
    pub assets: serde_json::Value,
    pub animation: serde_json::Value,
    pub audio: serde_json::Value,
    pub physics: serde_json::Value,
    pub reload: serde_json::Value,
    pub composition: serde_json::Value,
}

pub fn list_dashboard_runs(runs_root: impl AsRef<Path>) -> Result<Vec<RunDashboardSummary>> {
    let runs_root = runs_root.as_ref();
    if !runs_root.exists() {
        return Ok(Vec::new());
    }
    let mut runs = Vec::new();
    for entry in fs::read_dir(runs_root)
        .with_context(|| format!("failed to read runs root {}", runs_root.display()))?
    {
        let entry = entry.context("failed to read runs root entry")?;
        let path = entry.path();
        if !path.is_dir() || !path.join("run.json").is_file() {
            continue;
        }
        runs.push(read_dashboard_run_summary(&path)?);
    }
    runs.sort_by(|left, right| {
        right
            .created_at_unix_ms
            .cmp(&left.created_at_unix_ms)
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(runs)
}

pub fn read_dashboard_run(run_dir: impl AsRef<Path>) -> Result<RunDashboardReadModel> {
    let run_dir = run_dir.as_ref();
    let summary = read_dashboard_run_summary(run_dir)?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let evidence = read_evidence_index(run_dir)?.artifacts;
    let mutations = list_mutation_proposals(run_dir)?;
    let journal_view = read_dashboard_journal(run_dir, &evidence, &mutations);
    let journal = if journal_view.exists {
        fs::read_to_string(run_dir.join("journal.md")).unwrap_or_default()
    } else {
        String::new()
    };
    let screenshots =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_screenshot)?;
    let console_logs =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_console_log)?;
    let cdp_trace_summaries =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_cdp_trace_summary)?;
    let world_states =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_world_state)?;
    let frame_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_frame_metric)?;
    let performance_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_performance_metric)?;
    let scenario_results =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_scenario_result)?;
    let mutation_artifacts = select_dashboard_mutation_artifacts(run_dir)?;
    let mutation_lifecycle = read_dashboard_mutation_lifecycle(run_dir, &mutations);
    let replay = read_dashboard_replay(run_dir, &evidence)?;
    let comparison = read_dashboard_comparison(run_dir);
    let transaction_provenance = read_dashboard_transaction_provenance(&run);
    let engine_summaries = read_dashboard_engine_summaries(&world_states);
    let evidence_categories = dashboard_category_summaries(DashboardCategoryArtifacts {
        screenshots: &screenshots,
        world_states: &world_states,
        frame_metrics: &frame_metrics,
        performance_metrics: &performance_metrics,
        console_logs: &console_logs,
        cdp_trace_summaries: &cdp_trace_summaries,
        scenario_results: &scenario_results,
        mutation_artifacts: &mutation_artifacts,
    });
    Ok(RunDashboardReadModel {
        summary,
        run,
        verdict,
        journal,
        journal_view,
        evidence,
        screenshots,
        console_logs,
        cdp_trace_summaries,
        world_states,
        frame_metrics,
        performance_metrics,
        scenario_results,
        mutation_artifacts,
        mutation_lifecycle,
        replay,
        comparison,
        transaction_provenance,
        engine_summaries,
        evidence_categories,
        mutations,
    })
}

fn read_dashboard_run_summary(run_dir: &Path) -> Result<RunDashboardSummary> {
    let run = read_json_value(run_dir.join("run.json"))?;
    let evidence = read_evidence_index(run_dir)?.artifacts;
    let evidence_count = evidence.len();
    let mutations = list_mutation_proposals(run_dir)?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let mutation_artifacts = select_dashboard_mutation_artifacts(run_dir)?;
    let screenshots =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_screenshot)?;
    let console_logs =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_console_log)?;
    let cdp_trace_summaries =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_cdp_trace_summary)?;
    let world_states =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_world_state)?;
    let frame_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_frame_metric)?;
    let performance_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_performance_metric)?;
    let scenario_results =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_scenario_result)?;
    let evidence_categories = dashboard_category_summaries(DashboardCategoryArtifacts {
        screenshots: &screenshots,
        world_states: &world_states,
        frame_metrics: &frame_metrics,
        performance_metrics: &performance_metrics,
        console_logs: &console_logs,
        cdp_trace_summaries: &cdp_trace_summaries,
        scenario_results: &scenario_results,
        mutation_artifacts: &mutation_artifacts,
    });
    let id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    Ok(RunDashboardSummary {
        id,
        run_dir: run_dir.to_path_buf(),
        seed_id: json_string(&run, "seed_id").unwrap_or_else(|| "unknown-seed".to_string()),
        seed_title: json_string(&run, "seed_title").unwrap_or_else(|| "Untitled Seed".to_string()),
        run_status: json_string(&run, "status").unwrap_or_else(|| "unknown".to_string()),
        verdict_status: json_string(&verdict, "status").unwrap_or_else(|| "unknown".to_string()),
        scenario_status: dashboard_scenario_status(&verdict),
        created_at_unix_ms: run
            .get("created_at_unix_ms")
            .and_then(|value| value.as_u64())
            .map(u128::from)
            .unwrap_or(0),
        evidence_count,
        mutation_count: mutations.len(),
        worker_count: dashboard_worker_count(&evidence),
        evidence_categories,
        journal_path: run_dir.join("journal.md"),
    })
}

fn select_dashboard_artifacts(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
    predicate: fn(&EvidenceArtifact) -> bool,
) -> Result<Vec<RunDashboardArtifact>> {
    evidence
        .iter()
        .filter(|artifact| predicate(artifact))
        .map(|artifact| dashboard_artifact(run_dir, artifact))
        .collect()
}

fn dashboard_artifact(run_dir: &Path, artifact: &EvidenceArtifact) -> Result<RunDashboardArtifact> {
    dashboard_artifact_from_parts(
        run_dir,
        artifact.id.clone(),
        artifact.kind.clone(),
        artifact.path.clone(),
        artifact.metadata.clone(),
    )
}

fn dashboard_artifact_from_parts(
    run_dir: &Path,
    id: String,
    kind: String,
    path: String,
    metadata: serde_json::Value,
) -> Result<RunDashboardArtifact> {
    let absolute_path = run_dir.join(&path);
    let exists = absolute_path.is_file();
    let mut read_error = None;
    let value = if exists && (kind == "application/json" || path.ends_with(".json")) {
        match read_json_value(&absolute_path) {
            Ok(value) => Some(value),
            Err(error) => {
                read_error = Some(error.to_string());
                None
            }
        }
    } else if exists {
        None
    } else {
        read_error = Some(format!("missing artifact file: {path}"));
        None
    };
    Ok(RunDashboardArtifact {
        id,
        kind,
        path,
        metadata,
        value,
        exists,
        read_error,
    })
}

fn dashboard_artifact_is_screenshot(artifact: &EvidenceArtifact) -> bool {
    artifact.kind == "image/png" || artifact.path.ends_with(".png")
}

fn dashboard_artifact_is_console_log(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("console_log")
        || artifact.id.contains("console")
        || artifact.path.contains("console")
}

fn dashboard_artifact_is_cdp_trace_summary(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("cdp_trace_summary")
        || artifact.id.contains("cdp-trace")
        || artifact.path.contains("cdp-trace")
}

fn dashboard_artifact_is_world_state(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("world_state")
        || artifact
            .metadata
            .get("probe_call")
            .and_then(|value| value.as_str())
            == Some("getWorldState")
        || artifact.id.contains("world-state")
        || artifact.path.contains("world-state")
}

fn dashboard_artifact_is_frame_metric(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("probe_call")
        .and_then(|value| value.as_str())
        == Some("getFrameStats")
        || artifact.id.contains("frame-stats")
        || artifact.path.contains("frame-stats")
}

fn dashboard_artifact_is_performance_metric(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("performance_metrics")
        || artifact.id.contains("performance")
        || artifact.path.contains("metrics")
}

fn dashboard_artifact_is_scenario_result(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("scenario_result")
        || artifact.id.contains("scenario-result")
        || artifact.path.contains("scenario-result")
}

fn dashboard_artifact_is_input_replay(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("input_replay")
        || artifact.id.contains("input-replay")
        || artifact.path.contains("input-replay")
}

fn read_dashboard_replay(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardReplay> {
    let replay_artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_input_replay)?;
    if replay_artifacts.is_empty() {
        return Ok(RunDashboardReplay {
            present: false,
            empty_state: "No replay evidence artifacts were found for this run.".to_string(),
            sequences: Vec::new(),
        });
    }
    let scenario_results =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_scenario_result)?;
    let mut sequences = Vec::new();
    for artifact in replay_artifacts {
        let scenario_id = dashboard_artifact_scenario_id(&artifact);
        let replay_value = artifact
            .value
            .as_ref()
            .and_then(dashboard_replay_payload_value);
        let frames = replay_value.map_or_else(Vec::new, dashboard_replay_frames);
        let replay_id = replay_value
            .and_then(|value| json_string(value, "id"))
            .unwrap_or_else(|| artifact.id.clone());
        let event_count = replay_value
            .and_then(|value| value.get("events"))
            .and_then(|value| value.as_array())
            .map_or(0, Vec::len);
        let source = dashboard_replay_source(&artifact);
        let evidence_refs = std::iter::once(artifact.path.clone())
            .chain(
                scenario_results
                    .iter()
                    .filter(|result| {
                        dashboard_artifact_matches_scenario(result, scenario_id.as_deref())
                    })
                    .map(|result| result.path.clone()),
            )
            .collect::<Vec<_>>();
        sequences.push(RunDashboardReplaySequence {
            id: replay_id,
            source,
            scenario_id: scenario_id.clone(),
            replay_path: artifact.path.clone(),
            event_count,
            first_frame: frames.first().copied(),
            last_frame: frames.last().copied(),
            frames,
            evidence_refs,
            checkpoints: dashboard_replay_checkpoints(
                run_dir,
                scenario_id.as_deref(),
                &scenario_results,
            ),
            read_error: artifact.read_error.clone(),
        });
    }
    Ok(RunDashboardReplay {
        present: true,
        empty_state: String::new(),
        sequences,
    })
}

fn dashboard_replay_payload_value(value: &serde_json::Value) -> Option<&serde_json::Value> {
    value.get("replay").unwrap_or(value).as_object()?;
    Some(value.get("replay").unwrap_or(value))
}

fn dashboard_replay_frames(replay: &serde_json::Value) -> Vec<u64> {
    let mut frames = replay
        .get("events")
        .and_then(|value| value.as_array())
        .map(|events| {
            events
                .iter()
                .filter_map(|event| event.get("frame").and_then(|value| value.as_u64()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    frames.sort_unstable();
    frames.dedup();
    frames
}

fn dashboard_replay_source(artifact: &RunDashboardArtifact) -> String {
    artifact
        .metadata
        .get("source")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| {
            artifact
                .value
                .as_ref()
                .and_then(|value| value.get("reference"))
                .map(|_| "replayRef".to_string())
        })
        .unwrap_or_else(|| "inline".to_string())
}

fn dashboard_artifact_scenario_id(artifact: &RunDashboardArtifact) -> Option<String> {
    artifact
        .metadata
        .get("scenario_id")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| dashboard_scenario_id_from_path(&artifact.path))
}

fn dashboard_scenario_id_from_path(path: &str) -> Option<String> {
    let mut parts = path.split('/');
    while let Some(part) = parts.next() {
        if part == "scenarios" {
            return parts.next().map(str::to_string);
        }
    }
    None
}

fn dashboard_artifact_matches_scenario(
    artifact: &RunDashboardArtifact,
    scenario_id: Option<&str>,
) -> bool {
    match scenario_id {
        Some(expected) => dashboard_artifact_scenario_id(artifact).as_deref() == Some(expected),
        None => true,
    }
}

fn dashboard_replay_checkpoints(
    run_dir: &Path,
    scenario_id: Option<&str>,
    scenario_results: &[RunDashboardArtifact],
) -> Vec<RunDashboardReplayCheckpoint> {
    scenario_results
        .iter()
        .filter(|result| dashboard_artifact_matches_scenario(result, scenario_id))
        .filter_map(|result| {
            let value = result.value.as_ref()?;
            let evidence = value.get("evidence")?;
            let world_state_path = evidence
                .get("world_state")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            let frame_stats_path = evidence
                .get("frame_stats")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            let world_state = world_state_path
                .as_ref()
                .and_then(|path| read_json_value(run_dir.join(path)).ok());
            let frame_stats = frame_stats_path
                .as_ref()
                .and_then(|path| read_json_value(run_dir.join(path)).ok());
            let tick = world_state
                .as_ref()
                .and_then(|value| value.get("tick"))
                .and_then(|value| value.as_u64())
                .or_else(|| {
                    frame_stats
                        .as_ref()
                        .and_then(|value| value.get("tick"))
                        .and_then(|value| value.as_u64())
                });
            let frame = frame_stats
                .as_ref()
                .and_then(|value| value.get("frame"))
                .and_then(|value| value.as_u64())
                .or(tick);
            let checkpoint_id = format!(
                "{}-checkpoint",
                json_string(value, "scenario_id")
                    .or_else(|| dashboard_artifact_scenario_id(result))
                    .unwrap_or_else(|| result.id.clone())
            );
            Some(RunDashboardReplayCheckpoint {
                id: checkpoint_id,
                label: "Post-replay world state".to_string(),
                frame,
                tick,
                world_state_path,
                frame_stats_path,
                world_state,
            })
        })
        .collect()
}

fn dashboard_array_len(value: Option<&serde_json::Value>) -> usize {
    value
        .and_then(|candidate| candidate.as_array())
        .map_or(0, Vec::len)
}

fn dashboard_entities_with_component(world_state: &serde_json::Value, component: &str) -> usize {
    world_state
        .get("entities")
        .and_then(|entities| entities.as_array())
        .map(|entities| {
            entities
                .iter()
                .filter(|entity| {
                    entity
                        .get("components")
                        .and_then(|components| components.get(component))
                        .is_some()
                })
                .count()
        })
        .unwrap_or(0)
}

fn read_dashboard_transaction_provenance(
    run: &serde_json::Value,
) -> Option<RunTransactionProvenance> {
    run.get("transaction_provenance")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok())
}

fn read_dashboard_engine_summaries(
    world_states: &[RunDashboardArtifact],
) -> RunDashboardEngineSummaries {
    let source = world_states.iter().find(|artifact| {
        artifact.exists && artifact.read_error.is_none() && artifact.value.is_some()
    });
    let Some(artifact) = source else {
        return RunDashboardEngineSummaries {
            present: false,
            empty_state:
                "No readable world-state artifacts are available for Engine Expansion summaries."
                    .to_string(),
            source_world_state: None,
            scene: json!({}),
            renderer: json!({}),
            tilemaps: json!({}),
            assets: json!({}),
            animation: json!({}),
            audio: json!({}),
            physics: json!({}),
            reload: json!({}),
            composition: json!({}),
        };
    };
    let world_state = artifact.value.as_ref().expect("checked world-state value");
    let entities = dashboard_array_len(world_state.get("entities"));
    RunDashboardEngineSummaries {
        present: true,
        empty_state: String::new(),
        source_world_state: Some(artifact.path.clone()),
        scene: json!({
            "sceneId": world_state.get("sceneId").cloned().unwrap_or(json!(null)),
            "schemaVersion": world_state.get("schemaVersion").cloned().unwrap_or(json!(null)),
            "entityCount": entities,
            "tick": world_state.get("tick").cloned().unwrap_or(json!(null))
        }),
        renderer: json!({
            "present": world_state.get("renderer").is_some(),
            "version": world_state.pointer("/renderer/version").cloned().unwrap_or(json!(null)),
            "renderedEntities": dashboard_array_len(world_state.pointer("/renderer/renderedEntities")),
            "camera": world_state.pointer("/renderer/camera").cloned().unwrap_or(json!(null))
        }),
        tilemaps: json!({
            "present": world_state.get("tilemaps").is_some(),
            "tilemapCount": dashboard_array_len(world_state.pointer("/tilemaps/tilemaps")),
            "layerCount": dashboard_array_len(world_state.pointer("/tilemaps/layerOrder"))
        }),
        assets: json!({
            "manifestId": world_state.pointer("/assetManifest/id").cloned().unwrap_or(json!(null)),
            "assetCount": dashboard_array_len(world_state.get("assets")),
            "manifestAssetCount": world_state.pointer("/assetManifest/assetCount").cloned().unwrap_or(json!(null))
        }),
        animation: json!({
            "animatedEntityCount": dashboard_entities_with_component(world_state, "animation")
        }),
        audio: json!({
            "audioEventCount": dashboard_array_len(world_state.get("audioEvents")),
            "audioEntityCount": dashboard_entities_with_component(world_state, "audio")
        }),
        physics: json!({
            "collisionCount": dashboard_array_len(world_state.get("collisions")),
            "collisionEventCount": dashboard_array_len(world_state.get("collisionEvents")),
            "colliderEntityCount": dashboard_entities_with_component(world_state, "collider")
        }),
        reload: json!({
            "reloadCount": dashboard_array_len(world_state.get("reloads")),
            "lastStatus": world_state.get("reloads")
                .and_then(|reloads| reloads.as_array())
                .and_then(|reloads| reloads.last())
                .and_then(|reload| reload.get("status"))
                .cloned()
                .unwrap_or(json!(null))
        }),
        composition: json!({
            "present": world_state.get("composition").is_some(),
            "entityCount": dashboard_array_len(world_state.pointer("/composition/entities")),
            "parentedEntityCount": world_state
                .pointer("/composition/entities")
                .and_then(|entities| entities.as_array())
                .map(|entities| entities.iter().filter(|entity| entity.get("parent").is_some_and(|parent| !parent.is_null())).count())
                .unwrap_or(0)
        }),
    }
}

fn read_dashboard_comparison(run_dir: &Path) -> RunDashboardComparison {
    let paths = dashboard_run_comparison_artifact_paths(run_dir);
    if paths.is_empty() {
        return RunDashboardComparison {
            present: false,
            empty_state: "No run comparison artifacts were found for this run.".to_string(),
            artifacts: Vec::new(),
        };
    }
    let artifacts = paths
        .into_iter()
        .map(|path| dashboard_comparison_artifact(run_dir, path))
        .collect::<Vec<_>>();
    RunDashboardComparison {
        present: true,
        empty_state: String::new(),
        artifacts,
    }
}

fn dashboard_run_comparison_artifact_paths(run_dir: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    for dir in ["mutation", "comparisons"] {
        if let Ok(entries) = fs::read_dir(run_dir.join(dir)) {
            for entry in entries.flatten() {
                let path = entry.path();
                let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                    continue;
                };
                if name.starts_with("run-comparison-") && name.ends_with(".json") {
                    paths.push(format!("{dir}/{name}"));
                }
            }
        }
    }
    paths.sort();
    paths
}

fn dashboard_comparison_artifact(run_dir: &Path, path: String) -> RunDashboardComparisonArtifact {
    let id = Path::new(&path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("run-comparison")
        .to_string();
    let absolute_path = run_dir.join(&path);
    if !absolute_path.is_file() {
        return RunDashboardComparisonArtifact {
            id,
            path: path.clone(),
            exists: false,
            read_error: Some(format!("missing comparison artifact file: {path}")),
            before_run_id: None,
            after_run_id: None,
            classification: None,
            deltas: serde_json::Value::Null,
            semantic: serde_json::Value::Null,
            evidence_refs: Vec::new(),
            unsupported: Vec::new(),
            value: None,
        };
    }
    match read_json_value(&absolute_path) {
        Ok(value) => RunDashboardComparisonArtifact {
            id,
            path,
            exists: true,
            read_error: None,
            before_run_id: json_string(&value, "before_run_id"),
            after_run_id: json_string(&value, "after_run_id"),
            classification: json_string(&value, "classification"),
            deltas: value
                .get("deltas")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            semantic: value
                .get("semantic")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            evidence_refs: value
                .get("evidence_refs")
                .and_then(|refs| refs.as_array())
                .map(|refs| {
                    refs.iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            unsupported: value
                .get("unsupported")
                .and_then(|items| items.as_array())
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            value: Some(value),
        },
        Err(error) => RunDashboardComparisonArtifact {
            id,
            path,
            exists: true,
            read_error: Some(error.to_string()),
            before_run_id: None,
            after_run_id: None,
            classification: None,
            deltas: serde_json::Value::Null,
            semantic: serde_json::Value::Null,
            evidence_refs: Vec::new(),
            unsupported: Vec::new(),
            value: None,
        },
    }
}

fn select_dashboard_mutation_artifacts(run_dir: &Path) -> Result<Vec<RunDashboardArtifact>> {
    [
        ("mutation-proposals", "mutation/proposals.json"),
        ("mutation-classifications", "mutation/classifications.json"),
        ("mutation-patch-drafts", "mutation/patch-drafts.json"),
        (
            "mutation-rerun-orchestration",
            "mutation/rerun-orchestration.json",
        ),
        (
            "mutation-scene-applications",
            "mutation/scene-applications.json",
        ),
        (
            "mutation-review-decisions",
            "mutation/review-decisions.json",
        ),
        (
            "mutation-evolve-v1-demo-summary",
            "mutation/evolve-v1-demo-summary.json",
        ),
    ]
    .into_iter()
    .filter(|(_, path)| run_dir.join(path).is_file())
    .map(|(id, path)| {
        dashboard_artifact_from_parts(
            run_dir,
            id.to_string(),
            "application/json".to_string(),
            path.to_string(),
            json!({ "artifact": "mutation_artifact", "read_only": true }),
        )
    })
    .collect()
}

struct DashboardCategoryArtifacts<'a> {
    screenshots: &'a [RunDashboardArtifact],
    world_states: &'a [RunDashboardArtifact],
    frame_metrics: &'a [RunDashboardArtifact],
    performance_metrics: &'a [RunDashboardArtifact],
    console_logs: &'a [RunDashboardArtifact],
    cdp_trace_summaries: &'a [RunDashboardArtifact],
    scenario_results: &'a [RunDashboardArtifact],
    mutation_artifacts: &'a [RunDashboardArtifact],
}

fn dashboard_category_summaries(
    artifacts: DashboardCategoryArtifacts<'_>,
) -> Vec<RunDashboardCategorySummary> {
    vec![
        dashboard_category_summary("screenshots", "Screenshots", artifacts.screenshots),
        dashboard_category_summary(
            "world_states",
            "World-state snapshots",
            artifacts.world_states,
        ),
        dashboard_category_summary(
            "frame_performance_metrics",
            "Frame/performance metrics",
            &artifacts
                .frame_metrics
                .iter()
                .chain(artifacts.performance_metrics.iter())
                .cloned()
                .collect::<Vec<_>>(),
        ),
        dashboard_category_summary(
            "console_cdp_summaries",
            "Console/CDP summaries",
            &artifacts
                .console_logs
                .iter()
                .chain(artifacts.cdp_trace_summaries.iter())
                .cloned()
                .collect::<Vec<_>>(),
        ),
        dashboard_category_summary(
            "scenario_results",
            "Scenario results",
            artifacts.scenario_results,
        ),
        dashboard_category_summary(
            "mutation_artifacts",
            "Mutation artifacts",
            artifacts.mutation_artifacts,
        ),
    ]
}

fn dashboard_category_summary(
    id: &str,
    label: &str,
    artifacts: &[RunDashboardArtifact],
) -> RunDashboardCategorySummary {
    RunDashboardCategorySummary {
        id: id.to_string(),
        label: label.to_string(),
        count: artifacts.len(),
        missing_count: artifacts.iter().filter(|artifact| !artifact.exists).count(),
        malformed_count: artifacts
            .iter()
            .filter(|artifact| artifact.exists && artifact.read_error.is_some())
            .count(),
    }
}

fn dashboard_worker_count(evidence: &[EvidenceArtifact]) -> usize {
    let mut worker_ids = BTreeSet::new();
    for artifact in evidence {
        if let Some(worker_id) = artifact
            .metadata
            .get("worker_id")
            .and_then(|value| value.as_str())
        {
            worker_ids.insert(worker_id.to_string());
        }
    }
    worker_ids.len()
}

fn dashboard_scenario_status(verdict: &serde_json::Value) -> String {
    let scenario_results = verdict
        .get("metadata")
        .and_then(|metadata| metadata.get("scenario_results"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    if scenario_results == 0 {
        "pending".to_string()
    } else {
        json_string(verdict, "status").unwrap_or_else(|| "unknown".to_string())
    }
}

fn read_dashboard_journal(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> RunDashboardJournal {
    let path = "journal.md".to_string();
    let absolute_path = run_dir.join(&path);
    match fs::read_to_string(&absolute_path) {
        Ok(body) => parse_dashboard_journal(&path, true, None, &body, evidence, mutations),
        Err(error) if error.kind() == ErrorKind::NotFound => parse_dashboard_journal(
            &path,
            false,
            Some("missing journal artifact".to_string()),
            "",
            evidence,
            mutations,
        ),
        Err(error) => parse_dashboard_journal(
            &path,
            false,
            Some(format!("failed to read journal artifact: {error}")),
            "",
            evidence,
            mutations,
        ),
    }
}

fn parse_dashboard_journal(
    path: &str,
    exists: bool,
    read_error: Option<String>,
    journal: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> RunDashboardJournal {
    let entries = dashboard_journal_entries(journal, evidence, mutations);
    let evidence_refs = collect_entry_refs(&entries, |entry| &entry.evidence_refs);
    let verdict_refs = collect_entry_refs(&entries, |entry| &entry.verdict_refs);
    let mutation_refs = collect_entry_refs(&entries, |entry| &entry.mutation_refs);
    RunDashboardJournal {
        path: path.to_string(),
        exists,
        read_error,
        summary: dashboard_journal_summary(journal, &entries),
        entries,
        evidence_refs,
        verdict_refs,
        mutation_refs,
    }
}

fn dashboard_journal_entries(
    journal: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> Vec<RunDashboardJournalEntry> {
    let mut entries = Vec::new();
    let mut current_heading = String::new();
    let mut current_level = 0usize;
    let mut current_body = String::new();

    for line in journal.lines() {
        if let Some((level, heading)) = markdown_heading(line) {
            push_dashboard_journal_entry(
                &mut entries,
                &current_heading,
                current_level,
                &current_body,
                evidence,
                mutations,
            );
            current_heading = heading;
            current_level = level;
            current_body.clear();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    push_dashboard_journal_entry(
        &mut entries,
        &current_heading,
        current_level,
        &current_body,
        evidence,
        mutations,
    );
    entries
}

fn push_dashboard_journal_entry(
    entries: &mut Vec<RunDashboardJournalEntry>,
    heading: &str,
    level: usize,
    body: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) {
    let body = body.trim();
    if heading.trim().is_empty() && body.is_empty() {
        return;
    }
    let fallback_heading = if heading.trim().is_empty() {
        "Journal"
    } else {
        heading.trim()
    };
    let category = dashboard_journal_category(fallback_heading);
    let entry_text = format!("{fallback_heading}\n{body}");
    let index = entries.len() + 1;
    entries.push(RunDashboardJournalEntry {
        id: format!("journal-entry-{index}-{}", slug_for_id(fallback_heading)),
        heading: fallback_heading.to_string(),
        level,
        category,
        body: body.to_string(),
        evidence_refs: extract_journal_evidence_refs(&entry_text, evidence),
        verdict_refs: extract_journal_verdict_refs(&entry_text),
        mutation_refs: extract_journal_mutation_refs(&entry_text, mutations),
    });
}

fn markdown_heading(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim_start();
    let level = trimmed.chars().take_while(|ch| *ch == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let rest = trimmed.get(level..)?.trim();
    if rest.is_empty() {
        return None;
    }
    Some((level, rest.to_string()))
}

fn dashboard_journal_category(heading: &str) -> String {
    let lowered = heading.to_ascii_lowercase();
    if lowered.contains("hypothesis") {
        "hypothesis"
    } else if lowered.contains("observation") || lowered.contains("evidence") {
        "observation"
    } else if lowered.contains("failed") || lowered.contains("failure") {
        "failure"
    } else if lowered.contains("mutation") {
        "next_mutation"
    } else if lowered.contains("verdict") {
        "verdict"
    } else {
        "summary"
    }
    .to_string()
}

fn dashboard_journal_summary(journal: &str, entries: &[RunDashboardJournalEntry]) -> String {
    for line in journal.lines() {
        let trimmed = line.trim();
        if let Some(summary) = trimmed.strip_prefix("- Summary:") {
            return summary.trim().to_string();
        }
    }
    entries
        .iter()
        .find_map(|entry| {
            entry
                .body
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "No journal content available.".to_string())
}

fn extract_journal_evidence_refs(text: &str, evidence: &[EvidenceArtifact]) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for artifact in evidence {
        if text.contains(&artifact.path) || text.contains(&artifact.id) {
            refs.insert(artifact.path.clone());
        }
    }
    refs.into_iter().collect()
}

fn extract_journal_verdict_refs(text: &str) -> Vec<String> {
    let lowered = text.to_ascii_lowercase();
    if lowered.contains("verdict") || lowered.contains("failed criteria") {
        vec!["verdict.json".to_string()]
    } else {
        Vec::new()
    }
}

fn extract_journal_mutation_refs(text: &str, mutations: &[MutationProposal]) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for mutation in mutations {
        if text.contains(&mutation.id) {
            refs.insert(mutation.id.clone());
        }
    }
    refs.into_iter().collect()
}

fn collect_entry_refs(
    entries: &[RunDashboardJournalEntry],
    selector: fn(&RunDashboardJournalEntry) -> &Vec<String>,
) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for entry in entries {
        for reference in selector(entry) {
            refs.insert(reference.clone());
        }
    }
    refs.into_iter().collect()
}

fn slug_for_id(value: &str) -> String {
    let mut slug = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_string()
}

fn read_dashboard_mutation_lifecycle(
    run_dir: &Path,
    proposals: &[MutationProposal],
) -> RunDashboardMutationLifecycle {
    let stages = vec![
        dashboard_lifecycle_stage_from_records(
            "proposed",
            "Proposed",
            if proposals.is_empty() {
                "missing"
            } else {
                "proposed"
            },
            Some("mutation/proposals.json"),
            proposals.iter().map(|proposal| json!(proposal)).collect(),
            proposals
                .iter()
                .map(|proposal| proposal.evidence_id.clone())
                .collect(),
            None,
        ),
        dashboard_lifecycle_stage_from_json_file(
            run_dir,
            "classified",
            "Classified",
            "classified",
            "mutation/classifications.json",
            "classifications",
        ),
        dashboard_lifecycle_stage_from_json_file(
            run_dir,
            "drafted",
            "Drafted",
            "drafted",
            "mutation/patch-drafts.json",
            "drafts",
        ),
        dashboard_sandbox_stage(run_dir),
        dashboard_comparison_stage(run_dir),
        dashboard_lifecycle_stage_from_json_file(
            run_dir,
            "scene_applied",
            "Applied scene mutation",
            "applied",
            "mutation/scene-applications.json",
            "applications",
        ),
        dashboard_lifecycle_stage_from_json_file(
            run_dir,
            "reviewed",
            "Manual review",
            "pending_review",
            "mutation/review-decisions.json",
            "decisions",
        ),
    ];
    let terminal_state = dashboard_lifecycle_terminal_state(&stages);
    let command_hints = dashboard_mutation_command_hints(run_dir, &stages);
    RunDashboardMutationLifecycle {
        terminal_state,
        stages,
        command_hints,
    }
}

fn dashboard_lifecycle_stage_from_json_file(
    run_dir: &Path,
    id: &str,
    label: &str,
    present_state: &str,
    artifact_path: &str,
    array_field: &str,
) -> RunDashboardMutationLifecycleStage {
    let path = run_dir.join(artifact_path);
    if !path.is_file() {
        return dashboard_lifecycle_stage_from_records(
            id,
            label,
            "missing",
            Some(artifact_path),
            Vec::new(),
            Vec::new(),
            None,
        );
    }
    match read_json_value(&path) {
        Ok(value) => {
            let records = value
                .get(array_field)
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_else(|| vec![value]);
            let state = if id == "reviewed" {
                dashboard_review_state(&records)
            } else {
                present_state.to_string()
            };
            let evidence_refs = collect_json_evidence_refs(&records);
            dashboard_lifecycle_stage_from_records(
                id,
                label,
                &state,
                Some(artifact_path),
                records,
                evidence_refs,
                None,
            )
        }
        Err(error) => dashboard_lifecycle_stage_from_records(
            id,
            label,
            "malformed",
            Some(artifact_path),
            Vec::new(),
            Vec::new(),
            Some(error.to_string()),
        ),
    }
}

fn dashboard_lifecycle_stage_from_records(
    id: &str,
    label: &str,
    state: &str,
    artifact_path: Option<&str>,
    records: Vec<serde_json::Value>,
    evidence_refs: Vec<String>,
    read_error: Option<String>,
) -> RunDashboardMutationLifecycleStage {
    RunDashboardMutationLifecycleStage {
        id: id.to_string(),
        label: label.to_string(),
        state: state.to_string(),
        artifact_path: artifact_path.map(str::to_string),
        record_count: records.len(),
        evidence_refs,
        records,
        read_error,
    }
}

fn dashboard_sandbox_stage(run_dir: &Path) -> RunDashboardMutationLifecycleStage {
    let mut records = Vec::new();
    let sandbox_root = run_dir.join("sandbox");
    if let Ok(drafts) = fs::read_dir(&sandbox_root) {
        for draft in drafts.flatten() {
            let result_path = draft.path().join("evidence/result.json");
            if result_path.is_file() {
                if let Ok(value) = read_json_value(&result_path) {
                    records.push(value);
                }
            }
        }
    }
    let evidence_refs = collect_json_evidence_refs(&records);
    dashboard_lifecycle_stage_from_records(
        "sandboxed",
        "Sandboxed",
        if records.is_empty() {
            "missing"
        } else {
            "sandboxed"
        },
        Some("sandbox/*/evidence/result.json"),
        records,
        evidence_refs,
        None,
    )
}

fn dashboard_comparison_stage(run_dir: &Path) -> RunDashboardMutationLifecycleStage {
    let mut records = Vec::new();
    for path in dashboard_comparison_artifact_paths(run_dir) {
        if let Ok(value) = read_json_value(run_dir.join(&path)) {
            records.push(value);
        }
    }
    let evidence_refs = collect_json_evidence_refs(&records);
    dashboard_lifecycle_stage_from_records(
        "compared",
        "Compared",
        if records.is_empty() {
            "missing"
        } else {
            "compared"
        },
        Some("mutation/rerun-orchestration.json"),
        records,
        evidence_refs,
        None,
    )
}

fn dashboard_comparison_artifact_paths(run_dir: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    if run_dir.join("mutation/rerun-orchestration.json").is_file() {
        paths.push("mutation/rerun-orchestration.json".to_string());
    }
    if let Ok(entries) = fs::read_dir(run_dir.join("mutation")) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            if name.starts_with("run-comparison-") && name.ends_with(".json") {
                paths.push(format!("mutation/{name}"));
            }
        }
    }
    paths.sort();
    paths
}

fn dashboard_review_state(records: &[serde_json::Value]) -> String {
    let mut has_pending = false;
    for record in records {
        match record.get("state").and_then(|value| value.as_str()) {
            Some("accepted") => return "accepted".to_string(),
            Some("rejected") => return "rejected".to_string(),
            Some("pending_review") => has_pending = true,
            _ => {}
        }
    }
    if has_pending || !records.is_empty() {
        "pending_review".to_string()
    } else {
        "missing".to_string()
    }
}

fn dashboard_lifecycle_terminal_state(stages: &[RunDashboardMutationLifecycleStage]) -> String {
    stages
        .iter()
        .rev()
        .find(|stage| stage.state != "missing")
        .map(|stage| stage.state.clone())
        .unwrap_or_else(|| "missing".to_string())
}

fn dashboard_mutation_command_hints(
    run_dir: &Path,
    stages: &[RunDashboardMutationLifecycleStage],
) -> Vec<String> {
    let has_draft = stages
        .iter()
        .any(|stage| stage.id == "drafted" && stage.state != "missing");
    if !has_draft {
        return Vec::new();
    }
    let run_path = run_dir.to_string_lossy();
    vec![
        format!(
            "cargo run -p ouroforge-cli -- mutation review {run_path} --accept --reason \"manual evidence review accepted\""
        ),
        format!(
            "cargo run -p ouroforge-cli -- mutation review {run_path} --reject --reason \"manual evidence review rejected\""
        ),
    ]
}

fn collect_json_evidence_refs(values: &[serde_json::Value]) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for value in values {
        collect_json_evidence_refs_inner(value, &mut refs);
    }
    refs.into_iter().collect()
}

fn collect_json_evidence_refs_inner(value: &serde_json::Value, refs: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::String(text)
            if text.starts_with("evidence/")
                || text.starts_with("mutation/")
                || text.starts_with("sandbox/") =>
        {
            refs.insert(text.clone());
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_json_evidence_refs_inner(item, refs);
            }
        }
        serde_json::Value::Object(map) => {
            for value in map.values() {
                collect_json_evidence_refs_inner(value, refs);
            }
        }
        _ => {}
    }
}

fn read_json_value(path: impl AsRef<Path>) -> Result<serde_json::Value> {
    let path = path.as_ref();
    let input = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&input)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}

fn json_string(value: &serde_json::Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunArtifacts {
    pub run_dir: PathBuf,
}

pub fn create_run(
    seed_path: impl AsRef<Path>,
    runs_root: impl AsRef<Path>,
) -> Result<RunArtifacts> {
    let seed_path = seed_path.as_ref();
    let runs_root = runs_root.as_ref();
    let seed_yaml = fs::read_to_string(seed_path)
        .with_context(|| format!("failed to read Seed file {}", seed_path.display()))?;
    let seed = Seed::from_path(seed_path)?;
    let seed_base_dir = seed_path.parent().unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(runs_root)
        .with_context(|| format!("failed to create runs root {}", runs_root.display()))?;

    let created_at_unix_ms = unix_millis()?;
    let run_id = format!("run-{created_at_unix_ms}-{}", std::process::id());
    let run_dir = runs_root.join(&run_id);
    fs::create_dir(&run_dir)
        .with_context(|| format!("failed to create run directory {}", run_dir.display()))?;
    fs::create_dir(run_dir.join("evidence")).context("failed to create evidence directory")?;

    write_json(
        &run_dir.join("run.json"),
        &json!({
            "id": run_id,
            "seed_id": seed.id,
            "seed_title": seed.title,
            "status": "created",
            "created_at_unix_ms": created_at_unix_ms,
        }),
    )?;
    fs::write(run_dir.join("seed.snapshot.yaml"), seed_yaml)
        .context("failed to write seed snapshot")?;
    copy_replay_references_to_run(&seed, seed_base_dir, &run_dir)?;
    write_ledger_created(&run_dir.join("ledger.jsonl"), created_at_unix_ms)?;
    fs::write(run_dir.join("journal.md"), initial_journal()).context("failed to write journal")?;
    write_json(
        &run_dir.join("verdict.json"),
        &json!({ "status": "pending" }),
    )?;
    write_evidence_index(
        &run_dir,
        &EvidenceIndex {
            artifacts: Vec::new(),
        },
    )?;

    Ok(RunArtifacts { run_dir })
}

fn copy_replay_references_to_run(seed: &Seed, seed_base_dir: &Path, run_dir: &Path) -> Result<()> {
    let mut copied = std::collections::BTreeSet::new();
    for replay_ref in seed.replay_references() {
        if !copied.insert(replay_ref.path.clone()) {
            continue;
        }
        let source = seed_base_dir.join(&replay_ref.path);
        let target = run_dir.join(&replay_ref.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create replay directory {}", parent.display())
            })?;
        }
        fs::copy(&source, &target).with_context(|| {
            format!(
                "failed to copy replay reference {} to {}",
                source.display(),
                target.display()
            )
        })?;
    }
    Ok(())
}

fn unix_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX_EPOCH")?
        .as_millis())
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    fs::write(path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}

fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    let temp_path = path.with_extension(format!(
        "json.tmp-{}-{}",
        std::process::id(),
        unix_millis()?
    ));
    fs::write(&temp_path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", temp_path.display()))?;
    fs::rename(&temp_path, path).with_context(|| {
        format!(
            "failed to replace {} with {}",
            path.display(),
            temp_path.display()
        )
    })
}

fn write_ledger_created(path: &Path, created_at_unix_ms: u128) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("failed to write {}", path.display()))?;
    let line = serde_json::to_string(&json!({
        "event": "run.created",
        "created_at_unix_ms": created_at_unix_ms,
    }))
    .context("failed to serialize ledger event")?;
    writeln!(file, "{line}").context("failed to write ledger event")
}

fn initial_journal() -> &'static str {
    "# Ouroforge Run Journal\n\n## Seed\n\n## Hypothesis\n\n## Observations\n\n## Evidence\n\n## Verdict\n\n## Next Mutation\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_SEED: &str = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

    #[test]
    fn parses_valid_seed() {
        let seed = Seed::from_yaml_str(VALID_SEED).expect("valid seed parses");
        assert_eq!(seed.id, "platformer.v0");
        assert_eq!(seed.constraints.target, "file-harness");
    }

    #[test]
    fn validates_renderer_tilemap_feature_regression_seed() {
        let seed = Seed::from_yaml_str(include_str!(
            "../../../seeds/engine-feature-renderer-tilemap.yaml"
        ))
        .expect("renderer/tilemap regression seed validates");
        assert_eq!(seed.id, "engine-feature.renderer-tilemap");
        assert_eq!(seed.scenarios.len(), 2);
        assert_eq!(seed.scenarios[0].id, "renderer-layer-camera-regression");
        assert_eq!(seed.scenarios[1].id, "tilemap-grid-layer-regression");
    }

    #[test]
    fn validates_asset_animation_audio_feature_regression_seed() {
        let seed = Seed::from_yaml_str(include_str!(
            "../../../seeds/engine-feature-asset-animation-audio.yaml"
        ))
        .expect("asset/animation/audio regression seed validates");
        assert_eq!(seed.id, "engine-feature.asset-animation-audio");
        assert_eq!(seed.scenarios.len(), 3);
        assert_eq!(seed.scenarios[0].id, "asset-manifest-regression");
        assert_eq!(seed.scenarios[1].id, "animation-frame-regression");
        assert_eq!(seed.scenarios[2].id, "audio-intent-regression");
    }

    #[test]
    fn validates_physics_reload_composition_feature_regression_seed() {
        let seed = Seed::from_yaml_str(include_str!(
            "../../../seeds/engine-feature-physics-reload-composition.yaml"
        ))
        .expect("physics/reload/composition regression seed validates");
        assert_eq!(seed.id, "engine-feature.physics-reload-composition");
        assert_eq!(seed.scenarios.len(), 3);
        assert_eq!(seed.scenarios[0].id, "physics-contact-trigger-regression");
        assert_eq!(seed.scenarios[1].id, "reload-boundary-regression");
        assert_eq!(seed.scenarios[2].id, "scene-composition-regression");
    }

    #[test]
    fn parses_valid_scenario_dsl() {
        let valid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: probe-smoke
    description: Exercise the minimal probe DSL.
    steps:
      - wait:
          frames: 2
      - input:
          right: true
    assertions:
      - world_state:
          path: tick
          equals: 2
      - frame_stats:
          path: fixedDeltaMs
          equals: 16
"#;

        let seed = Seed::from_yaml_str(valid).expect("scenario dsl parses");

        assert_eq!(seed.scenarios[0].steps.len(), 2);
        assert_eq!(seed.scenarios[0].assertions.len(), 2);
    }

    #[test]
    fn scenario_replay_reference_validates_referenced_file() {
        let root = unique_temp_dir("scenario-replay-ref-valid");
        fs::create_dir_all(root.join("replays")).expect("replay fixture dir exists");
        fs::write(
            root.join("replays/move-right.yaml"),
            r#"
schemaVersion: "1"
id: move-right
events:
  - frame: 0
    key: right
    pressed: true
  - frame: 4
    key: right
    pressed: false
"#,
        )
        .expect("replay fixture written");
        let seed_path = root.join("seed.yaml");
        fs::write(
            &seed_path,
            r#"
id: replay-ref.seed
title: Replay Ref Seed
goal: Validate replay references.
constraints:
  target: game-runtime
acceptance:
  - Replay reference validates.
scenarios:
  - id: replay-ref-smoke
    description: Bind replay from a fixture file.
    steps:
      - replayRef:
          id: move-right
          path: replays/move-right.yaml
    assertions:
      - world_state:
          path: object.x
          equals: 40
"#,
        )
        .expect("seed written");

        let seed = Seed::from_path(&seed_path).expect("replay reference validates");
        assert!(matches!(
            seed.scenarios[0].steps[0],
            ScenarioStep::ReplayRef { .. }
        ));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn scenario_replay_reference_rejects_missing_and_malformed_files() {
        let root = unique_temp_dir("scenario-replay-ref-invalid");
        fs::create_dir_all(root.join("replays")).expect("replay fixture dir exists");
        let seed_path = root.join("missing-seed.yaml");
        fs::write(
            &seed_path,
            r#"
id: replay-ref.seed
title: Replay Ref Seed
goal: Validate replay references.
constraints:
  target: game-runtime
acceptance:
  - Replay reference validates.
scenarios:
  - id: replay-ref-smoke
    description: Bind replay from a fixture file.
    steps:
      - replayRef:
          id: move-right
          path: replays/missing.yaml
"#,
        )
        .expect("missing seed written");
        let rejected = Seed::from_path(&seed_path).expect_err("missing replay reference rejected");
        assert!(rejected
            .to_string()
            .contains("replayRef could not be loaded"));

        fs::write(root.join("replays/bad.yaml"), "id: bad\n").expect("bad replay written");
        let malformed_seed_path = root.join("malformed-seed.yaml");
        fs::write(
            &malformed_seed_path,
            r#"
id: replay-ref.seed
title: Replay Ref Seed
goal: Validate replay references.
constraints:
  target: game-runtime
acceptance:
  - Replay reference validates.
scenarios:
  - id: replay-ref-smoke
    description: Bind replay from a fixture file.
    steps:
      - replayRef:
          id: bad
          path: replays/bad.yaml
"#,
        )
        .expect("malformed seed written");
        let rejected =
            Seed::from_path(&malformed_seed_path).expect_err("malformed replay rejected");
        assert!(rejected
            .to_string()
            .contains("replayRef could not be loaded"));

        let escaping = Seed::from_yaml_str(
            r#"
id: replay-ref.seed
title: Replay Ref Seed
goal: Validate replay references.
constraints:
  target: game-runtime
acceptance:
  - Replay reference validates.
scenarios:
  - id: replay-ref-smoke
    description: Bind replay from a fixture file.
    steps:
      - replayRef:
          id: move-right
          path: replays/../move-right.yaml
"#,
        )
        .expect_err("escaping replay reference rejected");
        assert!(escaping.to_string().contains("replayRef is invalid"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn parses_valid_input_replay_v1() {
        let replay = InputReplay::from_yaml_str(
            r#"
schemaVersion: "1"
id: move-right
events:
  - frame: 0
    key: right
    pressed: true
  - frame: 4
    key: right
    pressed: false
"#,
        )
        .expect("valid replay parses");

        assert_eq!(replay.id, "move-right");
        assert_eq!(replay.events.len(), 2);
        assert_eq!(replay.events[0].key, ReplayKey::Right);
        assert!(replay.events[0].pressed);
    }

    #[test]
    fn rejects_invalid_input_replay_v1() {
        let unordered = InputReplay::from_yaml_str(
            r#"
schemaVersion: "1"
id: move-right
events:
  - frame: 4
    key: right
    pressed: true
  - frame: 3
    key: right
    pressed: false
"#,
        )
        .expect_err("unordered replay rejected");
        assert!(unordered
            .to_string()
            .contains("ordered by nondecreasing frame"));

        let unknown_key = InputReplay::from_yaml_str(
            r#"
schemaVersion: "1"
id: pointer-replay
events:
  - frame: 0
    key: pointer
    pressed: true
"#,
        )
        .expect_err("unsupported key rejected");
        assert!(unknown_key.to_string().contains("failed to parse"));

        let too_large = InputReplay::from_yaml_str(
            r#"
schemaVersion: "1"
id: huge-frame
events:
  - frame: 100001
    key: left
    pressed: true
"#,
        )
        .expect_err("oversized frame rejected");
        assert!(too_large.to_string().contains("frame must be <= 100000"));

        let malformed = InputReplay::from_yaml_str(
            r#"
schemaVersion: "1"
id: malformed
events:
  - frame: 0
    key: left
    pressed: true
    pointer: 10
"#,
        )
        .expect_err("malformed replay rejected");
        assert!(malformed.to_string().contains("failed to parse"));
    }

    #[test]
    fn rejects_input_replay_missing_schema_version() {
        let missing = InputReplay::from_yaml_str(
            r#"
id: move-right
events:
  - frame: 0
    key: right
    pressed: true
"#,
        )
        .expect_err("replay missing schemaVersion rejected");
        assert!(missing.to_string().contains("failed to parse"));
    }

    #[test]
    fn rejects_scenario_missing_id() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: ""
    description: Missing scenario id.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("missing scenario id fails");
        assert!(error.to_string().contains("scenarios[0].id is required"));
    }

    #[test]
    fn rejects_invalid_scenario_step() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: probe-smoke
    description: Invalid wait step.
    steps:
      - wait:
          frames: 0
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("invalid step fails");
        assert!(error
            .to_string()
            .contains("wait.frames must be greater than 0"));
    }

    #[test]
    fn rejects_invalid_scenario_assertion() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: probe-smoke
    description: Invalid assertion.
    assertions:
      - world_state:
          path: tick > 0
          equals: true
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("invalid assertion fails");
        assert!(error.to_string().contains("path is invalid"));
    }

    #[test]
    fn parses_bounded_richer_assertion_schema() {
        let valid = r#"
id: assertion-model.v1
title: Assertion Model Fixture
goal: Validate bounded assertion schema.
constraints:
  target: file-harness
acceptance:
  - Validate richer assertions.
scenarios:
  - id: assertion-smoke
    description: Valid richer assertions.
    assertions:
      - world_state:
          path: player.health
          greaterThan: 0
      - runtime_events:
          path: events
          countGreaterThan: 0
      - frame_stats:
          path: fixedDeltaMs
          lessThan: 33
      - performance_metrics:
          path: ScriptDuration
          exists: true
      - console_errors:
          path: logs
          countEquals: 0
      - collision_evidence:
          path: pairs
          contains: goal:player
      - audio_evidence:
          path: events.0.kind
          notEquals: missing
      - animation_evidence:
          path: tracks.idle
          equals: true
"#;

        let seed = Seed::from_yaml_str(valid).expect("richer assertion schema parses");

        assert_eq!(seed.scenarios[0].assertions.len(), 8);
    }

    #[test]
    fn rejects_unbounded_or_ambiguous_assertion_schema() {
        let arbitrary_expression = r#"
id: assertion-model.v1
title: Assertion Model Fixture
goal: Reject arbitrary assertion schema.
constraints:
  target: file-harness
acceptance:
  - Reject arbitrary assertions.
scenarios:
  - id: assertion-smoke
    description: Invalid expression assertion.
    assertions:
      - world_state:
          path: player.health
          expression: player.health > 0
"#;
        let error = Seed::from_yaml_str(arbitrary_expression).expect_err("expression field fails");
        assert!(error.to_string().contains("failed to parse Seed YAML"));

        let multiple_operators = r#"
id: assertion-model.v1
title: Assertion Model Fixture
goal: Reject ambiguous assertion schema.
constraints:
  target: file-harness
acceptance:
  - Reject ambiguous assertions.
scenarios:
  - id: assertion-smoke
    description: Invalid multi-operator assertion.
    assertions:
      - world_state:
          path: player.health
          equals: 1
          greaterThan: 0
"#;
        let error = Seed::from_yaml_str(multiple_operators).expect_err("multiple operators fail");
        assert!(error
            .to_string()
            .contains("must define exactly one bounded assertion operator"));
    }

    #[test]
    fn parses_visual_checkpoint_step_schema() {
        let valid = r#"
id: visual-hooks.v1
title: Visual Hooks Fixture
goal: Validate visual checkpoint step schema.
constraints:
  target: file-harness
acceptance:
  - Capture named visual checkpoints.
scenarios:
  - id: visual-smoke
    description: Visual checkpoint fixture.
    steps:
      - visualCheckpoint:
          id: after-load
          baseline:
            id: baseline-a
            width: 320
            height: 180
          threshold:
            maxDimensionDelta: 0
"#;

        let seed = Seed::from_yaml_str(valid).expect("visual checkpoint seed parses");

        assert!(matches!(
            &seed.scenarios[0].steps[0],
            ScenarioStep::VisualCheckpoint {
                visual_checkpoint
            } if visual_checkpoint.id == "after-load" && visual_checkpoint.threshold.is_some()
        ));
    }

    #[test]
    fn rejects_seed_missing_required_target() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints: {}
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("missing target fails");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn rejects_seed_with_unknown_fields() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
future_scope: should-not-be-accepted
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("unknown fields fail");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn run_transaction_provenance_reads_passed_transaction_artifact() {
        let temp = unique_temp_dir("ouroforge-run-provenance-model-test");
        fs::create_dir_all(&temp).expect("temp dir exists");
        let scene_path = temp.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        let transaction = preview_scene_edit_transaction(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
        )
        .expect("transaction previews");
        let transaction_path = temp.join("transaction.json");
        write_scene_edit_transaction_artifact(&transaction_path, &transaction)
            .expect("transaction written");
        let provenance =
            run_transaction_provenance_from_artifact(&transaction_path).expect("provenance reads");
        assert_eq!(provenance.transaction_id, transaction.id);
        assert_eq!(provenance.scene_path, scene_path.to_string_lossy());
        assert_eq!(provenance.before_scene_hash, transaction.before_scene_hash);
        assert_eq!(
            provenance.after_scene_hash,
            transaction.after_scene_hash.expect("after hash exists")
        );
        fs::remove_dir_all(temp).ok();
    }

    #[test]
    fn run_transaction_provenance_rejects_failed_transaction_artifact() {
        let temp = unique_temp_dir("ouroforge-run-provenance-failed-test");
        fs::create_dir_all(&temp).expect("temp dir exists");
        let scene_path = temp.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        let transaction = preview_scene_edit_transaction(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.size.width".to_string(),
                value: json!(0),
            },
        )
        .expect("transaction previews");
        let transaction_path = temp.join("failed-transaction.json");
        write_scene_edit_transaction_artifact(&transaction_path, &transaction)
            .expect("transaction written");
        let error = run_transaction_provenance_from_artifact(&transaction_path)
            .expect_err("failed transaction rejected");
        assert!(error.to_string().contains("requires a passed transaction"));
        fs::remove_dir_all(temp).ok();
    }

    #[test]
    fn creates_required_run_artifacts() {
        let root = unique_temp_dir("ouroforge-core-test");
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");

        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");

        assert!(artifacts.run_dir.join("run.json").is_file());
        assert!(artifacts.run_dir.join("seed.snapshot.yaml").is_file());
        assert!(artifacts.run_dir.join("ledger.jsonl").is_file());
        assert!(artifacts.run_dir.join("journal.md").is_file());
        assert!(artifacts.run_dir.join("verdict.json").is_file());
        assert!(artifacts.run_dir.join("evidence/index.json").is_file());

        let ledger = fs::read_to_string(artifacts.run_dir.join("ledger.jsonl")).unwrap();
        let first_event: serde_json::Value =
            serde_json::from_str(ledger.lines().next().unwrap()).unwrap();
        assert_eq!(first_event["event"], "run.created");

        let evidence = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let evidence_index: serde_json::Value = serde_json::from_str(&evidence).unwrap();
        assert_eq!(evidence_index["artifacts"].as_array().unwrap().len(), 0);

        let journal = fs::read_to_string(artifacts.run_dir.join("journal.md")).unwrap();
        for heading in [
            "## Seed",
            "## Hypothesis",
            "## Observations",
            "## Evidence",
            "## Verdict",
            "## Next Mutation",
        ] {
            assert!(journal.contains(heading), "journal missing {heading}");
        }

        let verdict = fs::read_to_string(artifacts.run_dir.join("verdict.json")).unwrap();
        let verdict_json: serde_json::Value = serde_json::from_str(&verdict).unwrap();
        assert_eq!(verdict_json["status"], "pending");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn appends_and_reads_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-ledger-test");

        append_ledger_event(
            &artifacts.run_dir,
            "test.event",
            "test",
            json!({ "ok": true }),
        )
        .expect("first event appended");
        append_ledger_event(&artifacts.run_dir, "test.second", "test", json!({}))
            .expect("second event appended");

        let events = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        assert_eq!(events.len(), 3);
        assert_eq!(events[0]["event"], "run.created");
        assert_eq!(events[1]["event"], "test.event");
        assert_eq!(events[1]["actor"], "test");
        assert_eq!(events[1]["payload"], json!({ "ok": true }));
        assert_eq!(events[2]["event"], "test.second");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-bad-ledger-test");
        fs::write(artifacts.run_dir.join("ledger.jsonl"), "not-json\n")
            .expect("bad ledger written");

        let error = read_ledger_events(&artifacts.run_dir).expect_err("bad ledger fails");
        assert!(error.to_string().contains("failed to parse ledger JSON"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn adds_and_lists_evidence_artifacts() {
        let (root, artifacts) = create_test_run("ouroforge-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({ "source": "unit-test" }),
        )
        .expect("first evidence added");
        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-2",
            "application/json",
            "evidence/artifact-2.json",
            json!({}),
        )
        .expect("second evidence added");

        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        assert_eq!(artifacts_list.len(), 2);
        assert_eq!(artifacts_list[0].id, "artifact-1");
        assert_eq!(artifacts_list[0].metadata, json!({ "source": "unit-test" }));
        assert_eq!(artifacts_list[1].kind, "application/json");

        let index = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&index).unwrap();
        assert_eq!(parsed["artifacts"].as_array().unwrap().len(), 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_evidence_artifact_paths_outside_evidence_tree() {
        let (root, artifacts) = create_test_run("ouroforge-evidence-path-test");

        for path in ["../escape.txt", "/tmp/escape.txt", "artifact.txt"] {
            let error = add_evidence_artifact(
                &artifacts.run_dir,
                &format!("artifact-{path}"),
                "text/plain",
                path,
                json!({}),
            )
            .expect_err("invalid evidence path fails");
            assert!(error.to_string().contains("evidence artifact path"));
        }

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_evidence_index_and_duplicate_ids() {
        let (root, artifacts) = create_test_run("ouroforge-bad-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({}),
        )
        .expect("evidence added");
        let duplicate = add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/duplicate.txt",
            json!({}),
        )
        .expect_err("duplicate id fails");
        assert!(duplicate.to_string().contains("already exists"));

        fs::write(
            artifacts.run_dir.join("evidence/index.json"),
            r#"{"artifacts":"not-an-array"}"#,
        )
        .expect("bad evidence index written");
        let error = list_evidence_artifacts(&artifacts.run_dir).expect_err("bad index fails");
        assert!(error.to_string().contains("failed to parse evidence index"));

        fs::remove_dir_all(root).ok();
    }

    #[derive(Default)]
    struct MockCdpTransport {
        calls: Vec<(String, serde_json::Value)>,
    }

    impl CdpTransport for MockCdpTransport {
        fn send_command(
            &mut self,
            method: &str,
            params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            self.calls.push((method.to_string(), params));
            Ok(json!({ "frameId": "frame-1", "loaderId": "loader-1" }))
        }
    }

    struct FailingNavigateTransport;

    impl CdpTransport for FailingNavigateTransport {
        fn send_command(
            &mut self,
            _method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            Ok(json!({ "frameId": "frame-1", "errorText": "net::ERR_CONNECTION_REFUSED" }))
        }
    }

    #[test]
    fn cdp_client_navigates_through_transport_boundary() {
        let transport = MockCdpTransport::default();
        let mut client = CdpClient::new(transport);

        let result = client
            .navigate("http://localhost:8000")
            .expect("navigation command succeeds");

        assert_eq!(result.frame_id.as_deref(), Some("frame-1"));
        assert_eq!(result.loader_id.as_deref(), Some("loader-1"));
        assert_eq!(client.transport.calls.len(), 1);
        assert_eq!(client.transport.calls[0].0, "Page.navigate");
        assert_eq!(
            client.transport.calls[0].1,
            json!({ "url": "http://localhost:8000" })
        );
    }

    #[test]
    fn cdp_client_reports_navigation_error_text() {
        let mut client = CdpClient::new(FailingNavigateTransport);

        let error = client
            .navigate("http://localhost:9")
            .expect_err("navigation errorText fails");

        assert!(error.to_string().contains("net::ERR_CONNECTION_REFUSED"));
    }

    #[test]
    fn worker_id_defines_isolated_evidence_directory() {
        let worker = WorkerId::new("worker-4").expect("worker id parses");
        assert_eq!(worker.as_str(), "worker-4");
        assert_eq!(worker.evidence_dir(), "evidence/workers/worker-4");
        assert_eq!(
            worker.screenshot_path(42),
            "evidence/workers/worker-4/browser-smoke-42.png"
        );
        assert_eq!(
            worker.performance_metrics_path(42),
            "evidence/workers/worker-4/browser-smoke-metrics-42.json"
        );
    }

    #[test]
    fn worker_artifact_paths_do_not_conflict() {
        let worker_1 = WorkerId::new("worker-1").expect("worker 1 parses");
        let worker_2 = WorkerId::new("worker-2").expect("worker 2 parses");

        assert_ne!(worker_1.screenshot_path(7), worker_2.screenshot_path(7));
        assert_ne!(
            worker_1.performance_metrics_path(7),
            worker_2.performance_metrics_path(7)
        );
    }

    #[test]
    fn rejects_worker_ids_that_escape_paths() {
        let error = WorkerId::new("../worker").expect_err("path-like worker id fails");
        assert!(error.to_string().contains("worker id may only contain"));
    }

    #[test]
    fn browser_smoke_config_defaults_to_worker_one() {
        let config = BrowserSmokeConfig::new("runs/run-test", "http://localhost:8765")
            .expect("config builds");
        assert_eq!(config.worker_id.as_str(), "worker-1");
        assert_eq!(config.worker_id.evidence_dir(), "evidence/workers/worker-1");
    }

    #[test]
    fn browser_smoke_config_rejects_non_http_url() {
        for url in [
            "file:///etc/passwd",
            "chrome://settings",
            "data:text/html,<h1>x",
        ] {
            let error = BrowserSmokeConfig::new("runs/run-test", url)
                .expect_err("non-http smoke URL is rejected");
            assert!(
                error.to_string().contains("must use http:// or https://"),
                "unexpected error for {url}: {error}"
            );
        }
        assert!(BrowserSmokeConfig::new("runs/run-test", "https://example.test").is_ok());
    }

    #[test]
    fn scenario_run_config_rejects_non_http_url() {
        let error = ScenarioRunConfig::new("runs/run-test", "file:///tmp/x")
            .expect_err("non-http scenario URL is rejected");
        assert!(error.to_string().contains("must use http:// or https://"));
    }

    #[test]
    fn browser_smoke_pool_assigns_stable_worker_ids() {
        let mut base = BrowserSmokeConfig::new("runs/run-test", "http://localhost:8765")
            .expect("config builds");
        base.worker_id = WorkerId::new("custom-worker").expect("worker id parses");

        let single = BrowserSmokePoolConfig::new(base.clone(), 1).expect("single worker pool");
        assert_eq!(
            single.worker_config(0).unwrap().worker_id.as_str(),
            "custom-worker"
        );

        let pool = BrowserSmokePoolConfig::new(base, 3).expect("pool config builds");
        let worker_ids: Vec<_> = pool
            .worker_configs()
            .expect("worker configs build")
            .into_iter()
            .map(|config| config.worker_id.as_str().to_string())
            .collect();
        assert_eq!(worker_ids, vec!["worker-1", "worker-2", "worker-3"]);
    }

    #[test]
    fn browser_smoke_pool_rejects_zero_workers() {
        let base = BrowserSmokeConfig::new("runs/run-test", "http://localhost:8765")
            .expect("config builds");
        let error = BrowserSmokePoolConfig::new(base, 0).expect_err("zero workers fail");
        assert!(error.to_string().contains("workers must be at least 1"));
    }

    struct RuntimeProbeTransport {
        responses: std::collections::VecDeque<serde_json::Value>,
    }

    impl CdpTransport for RuntimeProbeTransport {
        fn send_command(
            &mut self,
            method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            assert_eq!(method, "Runtime.evaluate");
            self.responses
                .pop_front()
                .ok_or_else(|| anyhow!("missing runtime response"))
        }
    }

    #[test]
    fn captures_runtime_probe_json_as_evidence() {
        let (root, artifacts) = create_test_run("ouroforge-runtime-probe-capture-test");
        let config = BrowserSmokeConfig::new(&artifacts.run_dir, "http://127.0.0.1:8767")
            .expect("config builds");
        let mut client = CdpClient::new(RuntimeProbeTransport {
            responses: std::collections::VecDeque::from(vec![
                json!({ "result": { "value": true } }),
                json!({ "result": { "value": { "tick": 7, "object": { "id": "probe-square" } } } }),
                json!({ "result": { "value": { "tick": 7, "fixedDeltaMs": 16 } } }),
            ]),
        });

        capture_runtime_probe(&config, &mut client).expect("probe captured");

        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        assert_eq!(artifacts_list.len(), 2);
        assert!(artifacts_list
            .iter()
            .any(|artifact| artifact.path.contains("browser-probe-world-state")));
        assert!(artifacts_list
            .iter()
            .any(|artifact| artifact.path.contains("browser-probe-frame-stats")));
        assert!(artifacts_list.iter().all(|artifact| {
            artifact.path.starts_with("evidence/workers/worker-1/")
                && artifact.metadata["worker_id"] == "worker-1"
        }));

        let events = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        let probe_events: Vec<_> = events
            .iter()
            .filter(|event| event["event"] == "browser.probe.captured")
            .collect();
        assert_eq!(probe_events.len(), 2);
        assert!(probe_events.iter().any(|event| {
            event["payload"]["probe_call"] == "getWorldState"
                && event["payload"]["worker_id"] == "worker-1"
        }));
        assert!(probe_events.iter().any(|event| {
            event["payload"]["probe_call"] == "getFrameStats"
                && event["payload"]["worker_id"] == "worker-1"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn captures_console_log_json_as_bounded_evidence() {
        let (root, artifacts) = create_test_run("ouroforge-console-capture-test");
        let config = BrowserSmokeConfig::new(&artifacts.run_dir, "http://127.0.0.1:8767")
            .expect("config builds");
        let mut client = CdpClient::new(RuntimeProbeTransport {
            responses: std::collections::VecDeque::from(vec![json!({
                "result": {
                    "value": [
                        { "level": "log", "text": "ready", "argCount": 1, "timestampMs": 1 }
                    ]
                }
            })]),
        });

        let path = capture_console_log(&config, &mut client)
            .expect("console captured")
            .expect("console path returned");
        let value = read_json_value(artifacts.run_dir.join(&path)).expect("console JSON reads");
        assert_eq!(value[0]["text"], "ready");

        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        let console_artifact = artifacts_list
            .iter()
            .find(|artifact| artifact.metadata["artifact"] == "console_log")
            .expect("console artifact indexed");
        assert_eq!(console_artifact.metadata["bounded"], true);
        assert_eq!(console_artifact.metadata["limit"], 100);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn cdp_client_reports_runtime_evaluation_exception() {
        let mut client = CdpClient::new(RuntimeProbeTransport {
            responses: std::collections::VecDeque::from(vec![json!({
                "exceptionDetails": { "text": "boom" }
            })]),
        });

        let error = client
            .evaluate_json("window.__OUROFORGE__.getWorldState()")
            .expect_err("exception fails");
        assert!(error.to_string().contains("runtime evaluation failed"));
    }

    struct RecordingRuntimeTransport {
        calls: Vec<String>,
    }

    impl CdpTransport for RecordingRuntimeTransport {
        fn send_command(
            &mut self,
            method: &str,
            params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            assert_eq!(method, "Runtime.evaluate");
            let expression = params["expression"]
                .as_str()
                .expect("expression is present")
                .to_string();
            self.calls.push(expression.clone());
            let value = if expression.contains("typeof window.__OUROFORGE__.step") {
                json!(true)
            } else {
                json!({})
            };
            Ok(json!({ "result": { "value": value } }))
        }
    }

    #[test]
    fn evolve_failed_run_creates_proposal_and_updates_journal() {
        let (root, artifacts) = create_test_run("ouroforge-evolve-failed-test");
        fs::write(artifacts.run_dir.join("evidence/failure.json"), "{}\n")
            .expect("evidence written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "failure-evidence",
            "application/json",
            "evidence/failure.json",
            json!({}),
        )
        .expect("evidence indexed");
        write_json(
            &artifacts.run_dir.join("verdict.json"),
            &json!({
                "status": "failed",
                "summary": "failed",
                "failures": [{ "kind": "scenario_failed", "path": "evidence/failure.json" }],
                "evidence_refs": ["evidence/failure.json"],
                "metadata": {}
            }),
        )
        .expect("verdict written");

        let summary = evolve_run(&artifacts.run_dir).expect("evolve succeeds");

        assert_eq!(summary.status, "proposed");
        assert_eq!(summary.proposals_created, 1);
        assert_eq!(summary.classification_ids, vec!["classification-1"]);
        let proposals = list_mutation_proposals(&artifacts.run_dir).expect("proposals list");
        assert_eq!(proposals[0].evidence_id, "failure-evidence");
        let classifications = read_mutation_classification_artifact(&artifacts.run_dir)
            .expect("classification artifact reads");
        assert_eq!(
            classifications.classifications[0].category,
            MutationClassificationCategory::ScenarioAssertionFailure
        );
        assert_eq!(
            classifications.classifications[0].proposal_id.as_deref(),
            Some(proposals[0].id.as_str())
        );
        assert_eq!(
            classifications.classifications[0].evidence_refs,
            vec!["evidence/failure.json"]
        );
        let journal = show_journal(&artifacts.run_dir).expect("journal reads");
        assert!(journal.contains(&proposals[0].id));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evolve_passed_run_is_noop() {
        let (root, artifacts) = create_test_run("ouroforge-evolve-passed-test");
        write_json(
            &artifacts.run_dir.join("verdict.json"),
            &json!({ "status": "passed", "summary": "passed", "failures": [], "evidence_refs": [], "metadata": {} }),
        )
        .expect("verdict written");

        let summary = evolve_run(&artifacts.run_dir).expect("evolve succeeds");

        assert_eq!(summary.status, "noop");
        assert!(summary.classification_ids.is_empty());
        assert!(list_mutation_proposals(&artifacts.run_dir)
            .unwrap()
            .is_empty());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evolve_missing_verdict_fails_clearly() {
        let (root, artifacts) = create_test_run("ouroforge-evolve-missing-verdict-test");
        fs::remove_file(artifacts.run_dir.join("verdict.json")).expect("verdict removed");

        let error = evolve_run(&artifacts.run_dir).expect_err("missing verdict fails");

        assert!(error
            .to_string()
            .contains("failed to read verdict for evolve"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn creates_and_lists_mutation_proposals_without_applying() {
        let (root, artifacts) = create_test_run("ouroforge-mutation-test");
        fs::write(artifacts.run_dir.join("evidence/source.json"), "{}\n")
            .expect("evidence written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "evidence-1",
            "application/json",
            "evidence/source.json",
            json!({}),
        )
        .expect("evidence indexed");

        let proposal = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "test".to_string(),
                evidence_id: "evidence-1".to_string(),
                target: "scenes/platformer.yaml".to_string(),
                path: "entities.player.jump_impulse".to_string(),
                from: "7.5".to_string(),
                to: "9.0".to_string(),
            },
        )
        .expect("proposal created");

        assert_eq!(proposal.status, "proposed");
        assert!(!Path::new("scenes/platformer.yaml").exists());
        let proposals = list_mutation_proposals(&artifacts.run_dir).expect("proposals list");
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].evidence_id, "evidence-1");
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_mutation_proposal_with_unknown_evidence() {
        let (root, artifacts) = create_test_run("ouroforge-mutation-bad-evidence-test");

        let error = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "test".to_string(),
                evidence_id: "missing".to_string(),
                target: "scenes/platformer.yaml".to_string(),
                path: "entities.player.jump_impulse".to_string(),
                from: "7.5".to_string(),
                to: "9.0".to_string(),
            },
        )
        .expect_err("missing evidence fails");

        assert!(error.to_string().contains("evidence id not found"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn writes_and_reads_valid_mutation_classification_artifact() {
        let (root, artifacts) = create_test_run("ouroforge-classification-valid-test");
        let artifact = MutationClassificationArtifact {
            schema_version: "1".to_string(),
            run_id: "run-classification-valid".to_string(),
            classifications: vec![MutationClassification {
                id: "classification-1".to_string(),
                proposal_id: Some("mutation-1".to_string()),
                category: MutationClassificationCategory::ScenarioAssertionFailure,
                lifecycle_state: MutationClassificationState::Classified,
                reason: "scenario assertion failed with linked evidence".to_string(),
                evidence_refs: vec!["evidence/scenarios/demo/scenario-result-1.json".to_string()],
                verdict_ref: "verdict.json".to_string(),
                journal_ref: "journal.md".to_string(),
                scenario_result_refs: vec![
                    "evidence/scenarios/demo/scenario-result-1.json".to_string()
                ],
            }],
        };

        let path = write_mutation_classification_artifact(&artifacts.run_dir, &artifact)
            .expect("classification writes");
        assert_eq!(
            path,
            artifacts.run_dir.join("mutation/classifications.json")
        );
        let round_trip = read_mutation_classification_artifact(&artifacts.run_dir)
            .expect("classification reads");

        assert_eq!(round_trip, artifact);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn unknown_mutation_classification_allows_reason_without_evidence_refs() {
        let artifact = MutationClassificationArtifact {
            schema_version: "1".to_string(),
            run_id: "run-classification-unknown".to_string(),
            classifications: vec![MutationClassification {
                id: "classification-unknown".to_string(),
                proposal_id: None,
                category: MutationClassificationCategory::Unknown,
                lifecycle_state: MutationClassificationState::Classified,
                reason: "verdict did not include enough structured evidence".to_string(),
                evidence_refs: Vec::new(),
                verdict_ref: "verdict.json".to_string(),
                journal_ref: "journal.md".to_string(),
                scenario_result_refs: Vec::new(),
            }],
        };

        artifact
            .validate()
            .expect("unknown classification is valid with reason");
    }

    #[test]
    fn rejects_non_unknown_mutation_classification_without_evidence_refs() {
        let artifact = MutationClassificationArtifact {
            schema_version: "1".to_string(),
            run_id: "run-classification-invalid".to_string(),
            classifications: vec![MutationClassification {
                id: "classification-invalid".to_string(),
                proposal_id: None,
                category: MutationClassificationCategory::ConsoleError,
                lifecycle_state: MutationClassificationState::Classified,
                reason: "console error must cite evidence".to_string(),
                evidence_refs: Vec::new(),
                verdict_ref: "verdict.json".to_string(),
                journal_ref: "journal.md".to_string(),
                scenario_result_refs: Vec::new(),
            }],
        };

        let error = artifact
            .validate()
            .expect_err("non-unknown classification without evidence fails");
        assert!(error
            .to_string()
            .contains("non-unknown mutation classifications require"));
    }

    #[test]
    fn rejects_duplicate_mutation_classification_ids() {
        let classification = MutationClassification {
            id: "classification-duplicate".to_string(),
            proposal_id: None,
            category: MutationClassificationCategory::MissingEvidence,
            lifecycle_state: MutationClassificationState::Classified,
            reason: "missing evidence was detected".to_string(),
            evidence_refs: vec!["verdict.json".to_string()],
            verdict_ref: "verdict.json".to_string(),
            journal_ref: "journal.md".to_string(),
            scenario_result_refs: Vec::new(),
        };
        let artifact = MutationClassificationArtifact {
            schema_version: "1".to_string(),
            run_id: "run-classification-duplicate".to_string(),
            classifications: vec![classification.clone(), classification],
        };

        let error = artifact
            .validate()
            .expect_err("duplicate classification ids fail");
        assert!(error
            .to_string()
            .contains("duplicate mutation classification id"));
    }

    #[test]
    fn rejects_unsupported_mutation_classification_category() {
        let input = r#"
{
  "schema_version": "1",
  "run_id": "run-invalid-category",
  "classifications": [
    {
      "id": "classification-invalid-category",
      "category": "semantic_root_cause",
      "lifecycle_state": "classified",
      "reason": "unsupported broad taxonomy should fail",
      "evidence_refs": ["verdict.json"],
      "verdict_ref": "verdict.json",
      "journal_ref": "journal.md"
    }
  ]
}
"#;

        let error = serde_json::from_str::<MutationClassificationArtifact>(input)
            .expect_err("unsupported category fails during parse");

        assert!(error.to_string().contains("unknown variant"));
    }

    #[test]
    fn classifies_unknown_failed_verdict_without_evidence_refs() {
        let (root, artifacts) = create_test_run("ouroforge-classification-unknown-run-test");
        write_json(
            &artifacts.run_dir.join("verdict.json"),
            &json!({
                "status": "failed",
                "summary": "failed without structured refs",
                "failures": [{ "kind": "unstructured_failure" }],
                "evidence_refs": [],
                "metadata": {}
            }),
        )
        .expect("verdict written");

        let artifact =
            classify_mutation_failures(&artifacts.run_dir, &[]).expect("classification writes");

        assert_eq!(
            artifact.classifications[0].category,
            MutationClassificationCategory::Unknown
        );
        assert!(artifact.classifications[0].evidence_refs.is_empty());
        assert!(artifact.classifications[0]
            .reason
            .contains("did not provide evidence refs"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn classification_generation_is_deterministic_for_same_inputs() {
        let (root, artifacts) = create_test_run("ouroforge-classification-deterministic-test");
        fs::create_dir_all(artifacts.run_dir.join("evidence/scenarios/demo"))
            .expect("scenario evidence dir exists");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/demo/scenario-result-1.json"),
            "{}\n",
        )
        .expect("scenario result written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "scenario-result-1",
            "application/json",
            "evidence/scenarios/demo/scenario-result-1.json",
            json!({ "artifact": "scenario_result" }),
        )
        .expect("evidence indexed");
        write_json(
            &artifacts.run_dir.join("verdict.json"),
            &json!({
                "status": "failed",
                "summary": "scenario assertion failed",
                "failures": [{
                    "kind": "scenario_assertion_failure",
                    "path": "evidence/scenarios/demo/scenario-result-1.json"
                }],
                "evidence_refs": ["evidence/scenarios/demo/scenario-result-1.json"],
                "metadata": {}
            }),
        )
        .expect("verdict written");
        let proposal_ids = vec!["mutation-1".to_string()];

        let first = classify_mutation_failures(&artifacts.run_dir, &proposal_ids)
            .expect("first classification writes");
        let second = classify_mutation_failures(&artifacts.run_dir, &proposal_ids)
            .expect("second classification writes");

        assert_eq!(first, second);
        assert_eq!(
            first.classifications[0].scenario_result_refs,
            vec!["evidence/scenarios/demo/scenario-result-1.json"]
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn maps_supported_mutation_classification_categories() {
        let cases = [
            (
                json!({ "kind": "scenario_assertion_failure", "path": "evidence/scenario-result.json" }),
                MutationClassificationCategory::ScenarioAssertionFailure,
            ),
            (
                json!({ "kind": "runtime_probe_failure", "path": "evidence/world-state.json" }),
                MutationClassificationCategory::RuntimeProbeFailure,
            ),
            (
                json!({ "kind": "console_error", "path": "evidence/console-log.json" }),
                MutationClassificationCategory::ConsoleError,
            ),
            (
                json!({ "kind": "performance_regression", "path": "evidence/performance-metrics.json" }),
                MutationClassificationCategory::PerformanceRegression,
            ),
            (
                json!({ "kind": "visual_mismatch", "path": "evidence/visual-checkpoint.png" }),
                MutationClassificationCategory::VisualMismatch,
            ),
            (
                json!({ "kind": "missing_evidence", "path": "verdict.json" }),
                MutationClassificationCategory::MissingEvidence,
            ),
        ];
        let verdict = json!({ "summary": "" });

        for (failure, expected) in cases {
            let evidence_refs = collect_classification_evidence_refs(&failure, &verdict);
            let (actual, reason) =
                classify_failure_category(&failure, &verdict, "", &evidence_refs);
            assert_eq!(actual, expected, "reason: {reason}");
            assert!(!reason.is_empty());
        }
    }

    fn valid_patch_draft_artifact() -> PatchDraftArtifact {
        PatchDraftArtifact {
            schema_version: "1".to_string(),
            run_id: "run-patch-draft".to_string(),
            drafts: vec![PatchDraft {
                id: "patch-draft-1".to_string(),
                proposal_id: "mutation-1".to_string(),
                classification_id: "classification-1".to_string(),
                lifecycle_state: PatchDraftState::Drafted,
                target_path: "seeds/platformer.yaml".to_string(),
                rationale: "draft follows evidence-linked mutation proposal".to_string(),
                evidence_refs: vec!["evidence/scenarios/demo/scenario-result-1.json".to_string()],
                draft_text: "--- a/seeds/platformer.yaml\n+++ b/seeds/platformer.yaml\n# inspectable draft only\n"
                    .to_string(),
            }],
        }
    }

    #[test]
    fn writes_and_reads_valid_patch_draft_artifact() {
        let (root, artifacts) = create_test_run("ouroforge-patch-draft-valid-test");
        let artifact = valid_patch_draft_artifact();

        let path = write_patch_draft_artifact(&artifacts.run_dir, &artifact).expect("draft writes");
        assert_eq!(path, artifacts.run_dir.join("mutation/patch-drafts.json"));
        let round_trip = read_patch_draft_artifact(&artifacts.run_dir).expect("draft reads");

        assert_eq!(round_trip, artifact);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_patch_draft_unsafe_target_paths() {
        let mut artifact = valid_patch_draft_artifact();
        artifact.drafts[0].target_path = "../secrets.yaml".to_string();

        let error = artifact.validate().expect_err("escaping target path fails");

        assert!(error.to_string().contains("must not escape the repository"));
    }

    #[test]
    fn rejects_patch_draft_without_evidence_refs() {
        let mut artifact = valid_patch_draft_artifact();
        artifact.drafts[0].evidence_refs.clear();

        let error = artifact.validate().expect_err("missing evidence refs fail");

        assert!(error
            .to_string()
            .contains("requires at least one evidence ref"));
    }

    #[test]
    fn rejects_duplicate_patch_draft_ids() {
        let mut artifact = valid_patch_draft_artifact();
        artifact.drafts.push(artifact.drafts[0].clone());

        let error = artifact.validate().expect_err("duplicate drafts fail");

        assert!(error.to_string().contains("duplicate patch draft id"));
    }

    #[test]
    fn writing_patch_draft_artifact_does_not_modify_target_file() {
        let (root, artifacts) = create_test_run("ouroforge-patch-draft-no-mutate-test");
        let target = root.join("seeds/platformer.yaml");
        fs::create_dir_all(target.parent().expect("target parent")).expect("parent exists");
        fs::write(&target, "original seed content\n").expect("target written");
        let artifact = valid_patch_draft_artifact();

        write_patch_draft_artifact(&artifacts.run_dir, &artifact).expect("draft writes");

        let target_after = fs::read_to_string(&target).expect("target reads");
        assert_eq!(target_after, "original seed content\n");
        fs::remove_dir_all(root).ok();
    }

    fn create_draft_generation_fixture(prefix: &str) -> (PathBuf, RunArtifacts, MutationProposal) {
        let (root, artifacts) = create_test_run(prefix);
        fs::write(artifacts.run_dir.join("evidence/source.json"), "{}\n")
            .expect("evidence written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "evidence-1",
            "application/json",
            "evidence/source.json",
            json!({ "artifact": "scenario_result" }),
        )
        .expect("evidence indexed");
        let proposal = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "draft from evidence-linked proposal".to_string(),
                evidence_id: "evidence-1".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.0.assertions".to_string(),
                from: "old assertion".to_string(),
                to: "new assertion".to_string(),
            },
        )
        .expect("proposal writes");
        write_mutation_classification_artifact(
            &artifacts.run_dir,
            &MutationClassificationArtifact {
                schema_version: "1".to_string(),
                run_id: "run-draft-generation".to_string(),
                classifications: vec![MutationClassification {
                    id: "classification-1".to_string(),
                    proposal_id: Some(proposal.id.clone()),
                    category: MutationClassificationCategory::ScenarioAssertionFailure,
                    lifecycle_state: MutationClassificationState::Classified,
                    reason: "scenario assertion failure".to_string(),
                    evidence_refs: vec!["evidence/source.json".to_string()],
                    verdict_ref: "verdict.json".to_string(),
                    journal_ref: "journal.md".to_string(),
                    scenario_result_refs: vec!["evidence/source.json".to_string()],
                }],
            },
        )
        .expect("classification writes");
        (root, artifacts, proposal)
    }

    #[test]
    fn generates_patch_draft_from_proposal_and_classification() {
        let (root, artifacts, proposal) =
            create_draft_generation_fixture("ouroforge-patch-draft-generate-test");

        let artifact = generate_patch_drafts(&artifacts.run_dir).expect("draft generates");

        assert_eq!(artifact.drafts.len(), 1);
        let draft = &artifact.drafts[0];
        assert_eq!(draft.id, "patch-draft-1");
        assert_eq!(draft.proposal_id, proposal.id);
        assert_eq!(draft.classification_id, "classification-1");
        assert_eq!(draft.target_path, "seeds/platformer.yaml");
        assert!(draft
            .evidence_refs
            .contains(&"evidence/source.json".to_string()));
        assert!(draft.evidence_refs.contains(&"evidence-1".to_string()));
        assert!(draft
            .draft_text
            .contains("This is an inspectable draft artifact only"));
        assert!(artifacts
            .run_dir
            .join("mutation/patch-drafts.json")
            .is_file());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_draft_generation_is_deterministic_for_same_inputs() {
        let (root, artifacts, _) =
            create_draft_generation_fixture("ouroforge-patch-draft-deterministic-test");

        let first = generate_patch_drafts(&artifacts.run_dir).expect("first draft generates");
        let second = generate_patch_drafts(&artifacts.run_dir).expect("second draft generates");

        assert_eq!(first, second);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_draft_generation_requires_classification_artifact() {
        let (root, artifacts) = create_test_run("ouroforge-patch-draft-missing-class-test");
        fs::write(artifacts.run_dir.join("evidence/source.json"), "{}\n")
            .expect("evidence written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "evidence-1",
            "application/json",
            "evidence/source.json",
            json!({}),
        )
        .expect("evidence indexed");
        create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "missing classification should fail".to_string(),
                evidence_id: "evidence-1".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.0.assertions".to_string(),
                from: "old".to_string(),
                to: "new".to_string(),
            },
        )
        .expect("proposal writes");

        let error =
            generate_patch_drafts(&artifacts.run_dir).expect_err("missing classification fails");

        assert!(error
            .to_string()
            .contains("failed to read mutation classifications"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_draft_generation_rejects_unsafe_proposal_target() {
        let (root, artifacts, proposal) =
            create_draft_generation_fixture("ouroforge-patch-draft-unsafe-target-test");
        let mut proposals = read_mutation_proposals(&artifacts.run_dir).expect("proposals read");
        proposals.proposals[0].target = "../secrets.yaml".to_string();
        write_mutation_proposals(&artifacts.run_dir, &proposals).expect("proposal rewrite");

        let error =
            generate_patch_drafts(&artifacts.run_dir).expect_err("unsafe proposal target fails");

        assert!(error.to_string().contains(&format!(
            "unsupported mutation proposal target for {}",
            proposal.id
        )));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_draft_generation_does_not_modify_target_file() {
        let (root, artifacts, _) =
            create_draft_generation_fixture("ouroforge-patch-draft-generate-no-mutate-test");
        let target = root.join("seeds/platformer.yaml");
        fs::create_dir_all(target.parent().expect("target parent")).expect("parent exists");
        fs::write(&target, "original seed content\n").expect("target written");

        generate_patch_drafts(&artifacts.run_dir).expect("draft generates");

        let target_after = fs::read_to_string(&target).expect("target reads");
        assert_eq!(target_after, "original seed content\n");
        fs::remove_dir_all(root).ok();
    }

    fn create_sandbox_fixture(prefix: &str) -> (PathBuf, RunArtifacts, PatchDraftArtifact) {
        let (root, artifacts, _) = create_draft_generation_fixture(prefix);
        let drafts = generate_patch_drafts(&artifacts.run_dir).expect("draft generates");
        (root, artifacts, drafts)
    }

    #[test]
    fn appends_mutation_review_decisions_without_overwriting_history() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-review-append-test");

        let accepted = append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Accepted,
                reason: "comparison evidence improved the failed scenario".to_string(),
                evidence_refs: vec!["mutation/rerun-orchestration.json".to_string()],
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect("accepted decision appends");
        let rejected = append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Rejected,
                reason: "manual review found the draft too broad".to_string(),
                evidence_refs: vec!["sandbox/patch-draft-1/evidence/result.json".to_string()],
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect("rejected decision appends");

        let artifact =
            read_mutation_review_artifact(&artifacts.run_dir).expect("review artifact reads");
        assert_eq!(accepted.id, "review-decision-1");
        assert_eq!(rejected.id, "review-decision-2");
        assert_eq!(artifact.decisions.len(), 2);
        assert_eq!(artifact.decisions[0].state, MutationReviewState::Accepted);
        assert_eq!(artifact.decisions[1].state, MutationReviewState::Rejected);
        assert!(artifacts
            .run_dir
            .join("mutation/review-decisions.json")
            .is_file());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn mutation_review_accept_reject_require_reason_and_evidence() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-review-required-test");

        let missing_reason = append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Accepted,
                reason: " ".to_string(),
                evidence_refs: vec!["mutation/rerun-orchestration.json".to_string()],
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect_err("missing reason fails");
        let missing_evidence = append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Rejected,
                reason: "not enough evidence".to_string(),
                evidence_refs: Vec::new(),
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect_err("missing evidence fails");

        assert!(missing_reason
            .to_string()
            .contains("mutation review reason"));
        assert!(missing_evidence
            .to_string()
            .contains("requires evidence or comparison ref"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn mutation_review_decision_does_not_apply_patch() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-review-no-apply-test");
        let target = root.join("seeds/platformer.yaml");
        fs::create_dir_all(target.parent().expect("target parent")).expect("parent exists");
        fs::write(&target, "primary source remains unchanged\n").expect("target written");

        append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Accepted,
                reason: "accepted for audit only".to_string(),
                evidence_refs: vec!["mutation/rerun-orchestration.json".to_string()],
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect("decision appends");

        assert_eq!(
            fs::read_to_string(&target).expect("target reads"),
            "primary source remains unchanged\n"
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn mutation_review_from_path_uses_comparison_evidence_by_default() {
        let (root, artifacts, _drafts) =
            create_sandbox_fixture("ouroforge-review-default-evidence-test");
        fs::write(
            artifacts.run_dir.join("mutation/rerun-orchestration.json"),
            "{}\n",
        )
        .expect("comparison evidence written");

        let decision = append_mutation_review_decision_from_path(
            &artifacts.run_dir,
            MutationReviewState::Accepted,
            "manual reviewer accepts based on rerun comparison".to_string(),
            Vec::new(),
            "test-reviewer".to_string(),
        )
        .expect("decision appends with default evidence");

        assert_eq!(
            decision.evidence_refs,
            vec!["mutation/rerun-orchestration.json".to_string()]
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn plans_patch_sandbox_layout_under_run_sandbox_root() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-sandbox-plan-test");

        let plan =
            plan_patch_sandbox(&artifacts.run_dir, &drafts.drafts[0].id).expect("plan builds");

        assert_eq!(plan.layout.sandbox_root, "sandbox/patch-draft-1");
        assert_eq!(plan.layout.worktree_path, "sandbox/patch-draft-1/worktree");
        assert_eq!(plan.layout.evidence_path, "sandbox/patch-draft-1/evidence");
        assert_eq!(plan.layout.plan_path, "sandbox/patch-draft-1/plan.json");
        plan.validate(&artifacts.run_dir).expect("plan validates");
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn creates_patch_sandbox_layout_without_applying_patch() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-sandbox-create-test");

        let plan = create_patch_sandbox_layout(&artifacts.run_dir, &drafts.drafts[0].id)
            .expect("sandbox layout creates");

        assert!(artifacts.run_dir.join(&plan.layout.worktree_path).is_dir());
        assert!(artifacts.run_dir.join(&plan.layout.evidence_path).is_dir());
        assert!(artifacts.run_dir.join(&plan.layout.plan_path).is_file());
        assert!(!artifacts
            .run_dir
            .join(&plan.layout.worktree_path)
            .join("seeds/platformer.yaml")
            .exists());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_patch_sandbox_ids_that_escape_paths() {
        let (root, artifacts, _) = create_sandbox_fixture("ouroforge-sandbox-bad-id-test");

        let error = plan_patch_sandbox(&artifacts.run_dir, "../patch-draft-1")
            .expect_err("escaping sandbox id fails");

        assert!(error.to_string().contains("may contain only ASCII"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_patch_sandbox_layout_outside_sandbox_root() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-sandbox-bad-layout-test");
        let mut plan =
            plan_patch_sandbox(&artifacts.run_dir, &drafts.drafts[0].id).expect("plan builds");
        plan.layout.worktree_path = "../worktree".to_string();

        let error = plan
            .validate(&artifacts.run_dir)
            .expect_err("bad worktree path fails");

        assert!(error.to_string().contains("worktree path is invalid"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_sandbox_layout_does_not_modify_primary_target_file() {
        let (root, artifacts, drafts) =
            create_sandbox_fixture("ouroforge-sandbox-primary-unchanged-test");
        let target = root.join("seeds/platformer.yaml");
        fs::create_dir_all(target.parent().expect("target parent")).expect("parent exists");
        fs::write(&target, "primary working tree content\n").expect("target written");

        create_patch_sandbox_layout(&artifacts.run_dir, &drafts.drafts[0].id)
            .expect("sandbox layout creates");

        let target_after = fs::read_to_string(&target).expect("target reads");
        assert_eq!(target_after, "primary working tree content\n");
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn applies_patch_draft_in_sandbox_and_captures_result() {
        let (root, artifacts, drafts) = create_sandbox_fixture("ouroforge-sandbox-apply-test");
        let repo_root = git_repo_root().expect("repo root resolves");
        let primary_target = repo_root.join(&drafts.drafts[0].target_path);
        let primary_before = fs::read_to_string(&primary_target).expect("primary target reads");
        let status_before = git_status_short(&repo_root).expect("status before reads");

        let result =
            apply_patch_sandbox(&artifacts.run_dir, &drafts.drafts[0].id, &repo_root, false)
                .expect("sandbox apply succeeds");

        let sandbox_target = artifacts.run_dir.join(&result.applied_target_path);
        let sandbox_after = fs::read_to_string(&sandbox_target).expect("sandbox target reads");
        let primary_after = fs::read_to_string(&primary_target).expect("primary target reads");
        let status_after = git_status_short(&repo_root).expect("status after reads");

        assert_eq!(result.lifecycle_state, PatchSandboxState::Verified);
        assert_eq!(result.verification.len(), 0);
        assert!(artifacts.run_dir.join(&result.result_path).is_file());
        assert!(sandbox_after.contains("Ouroforge patch draft v1"));
        assert_eq!(primary_before, primary_after);
        assert_eq!(status_before, status_after);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn patch_sandbox_reports_missing_target_clearly() {
        let (root, artifacts, mut drafts) =
            create_sandbox_fixture("ouroforge-sandbox-missing-target-test");
        drafts.drafts[0].target_path = "missing/sandbox-target.yaml".to_string();
        write_patch_draft_artifact(&artifacts.run_dir, &drafts).expect("draft rewrites");
        let repo_root = git_repo_root().expect("repo root resolves");

        let error =
            apply_patch_sandbox(&artifacts.run_dir, &drafts.drafts[0].id, &repo_root, false)
                .expect_err("missing target fails");

        assert!(error
            .to_string()
            .contains("patch sandbox target does not exist"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evolves_rerun_orchestration_records_before_after_refs() {
        let (root, artifacts, drafts) =
            create_sandbox_fixture("ouroforge-rerun-orchestration-test");
        let repo_root = git_repo_root().expect("repo root resolves");
        let status_before = git_status_short(&repo_root).expect("status before reads");

        let result =
            orchestrate_evolve_rerun(&artifacts.run_dir, &drafts.drafts[0].id, &repo_root, false)
                .expect("rerun orchestration succeeds");

        let status_after = git_status_short(&repo_root).expect("status after reads");
        assert_eq!(result.patch_draft_id, "patch-draft-1");
        assert_eq!(result.mutation_proposal_id, drafts.drafts[0].proposal_id);
        assert_eq!(
            result.mutation_classification_id,
            drafts.drafts[0].classification_id
        );
        assert_eq!(result.before.run_path, "run.json");
        assert!(result.after.run_id.ends_with("--sandbox-patch-draft-1"));
        assert!(result
            .comparison_artifact_path
            .as_deref()
            .unwrap_or_default()
            .starts_with("mutation/run-comparison-"));
        assert_eq!(result.final_classification.as_deref(), Some("improved"));
        assert!(artifacts
            .run_dir
            .join(&result.evolve_evidence_path)
            .is_file());
        assert!(artifacts
            .run_dir
            .join(
                result
                    .comparison_artifact_path
                    .as_ref()
                    .expect("comparison path")
            )
            .is_file());
        assert_eq!(status_before, status_after);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evolve_rerun_orchestration_fails_for_missing_baseline_artifacts() {
        let (root, artifacts, drafts) =
            create_sandbox_fixture("ouroforge-rerun-missing-baseline-test");
        fs::remove_file(artifacts.run_dir.join("verdict.json")).expect("verdict removed");
        let repo_root = git_repo_root().expect("repo root resolves");

        let error =
            orchestrate_evolve_rerun(&artifacts.run_dir, &drafts.drafts[0].id, &repo_root, false)
                .expect_err("missing baseline fails");

        assert!(error
            .to_string()
            .contains("evolve rerun baseline is missing required artifact"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evolve_demo_lifecycle_links_completed_evidence() {
        let (root, artifacts, _drafts) = create_sandbox_fixture("ouroforge-demo-lifecycle-test");
        let repo_root = git_repo_root().expect("repo root resolves");

        let summary =
            run_evolve_demo_lifecycle(&artifacts.run_dir, &repo_root, false).expect("demo runs");

        assert_eq!(summary.status, "lifecycle_evidence_ready");
        assert_eq!(
            summary.manual_review_state,
            MutationReviewState::PendingReview
        );
        assert_eq!(
            summary.classification_artifact_path,
            "mutation/classifications.json"
        );
        assert_eq!(
            summary.patch_draft_artifact_path,
            "mutation/patch-drafts.json"
        );
        assert!(summary.sandbox_result_path.starts_with("sandbox/"));
        assert!(summary
            .comparison_artifact_path
            .starts_with("mutation/run-comparison-"));
        assert!(artifacts
            .run_dir
            .join(&summary.lifecycle_summary_path)
            .is_file());
        assert_eq!(
            summary.primary_git_status_before,
            summary.primary_git_status_after
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn mutation_proposals_remain_compatible_without_classifications() {
        let (root, artifacts) = create_test_run("ouroforge-classification-compat-test");
        fs::write(artifacts.run_dir.join("evidence/source.json"), "{}\n")
            .expect("evidence written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "evidence-1",
            "application/json",
            "evidence/source.json",
            json!({}),
        )
        .expect("evidence indexed");

        create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "classification schema must not be required".to_string(),
                evidence_id: "evidence-1".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.0.assertions".to_string(),
                from: "old".to_string(),
                to: "new".to_string(),
            },
        )
        .expect("proposal still writes without classifications");

        let proposals = list_mutation_proposals(&artifacts.run_dir).expect("proposals list");
        assert_eq!(proposals.len(), 1);
        assert!(!artifacts
            .run_dir
            .join("mutation/classifications.json")
            .exists());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn journal_renders_pass_fail_and_pending_verdicts() {
        let seed = Seed::from_yaml_str(VALID_SEED).expect("seed parses");
        let evidence = EvidenceIndex {
            artifacts: vec![EvidenceArtifact {
                id: "artifact-1".to_string(),
                kind: "application/json".to_string(),
                path: "evidence/artifact-1.json".to_string(),
                metadata: json!({}),
                added_at_unix_ms: 1,
            }],
        };
        let ledger = vec![json!({
            "event": "scenario.completed",
            "payload": { "scenario_id": "smoke" }
        })];

        for status in ["passed", "failed", "pending"] {
            let journal = render_journal(
                &seed,
                &evidence,
                &ledger,
                &json!({
                    "status": status,
                    "summary": format!("{status} summary"),
                    "failures": if status == "failed" { vec![json!({"kind": "scenario_failed"})] } else { Vec::new() }
                }),
                &[],
                &json!({}),
            );
            assert!(journal.contains(&format!("- Status: `{status}`")));
            assert!(journal.contains("`artifact-1`"));
            assert!(journal.contains("## Next Mutation"));
        }
    }

    #[test]
    fn evaluator_marks_run_pending_without_scenario_results() {
        let (root, artifacts) = create_test_run("ouroforge-eval-pending-test");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "pending");
        assert!(artifacts.run_dir.join("verdict.json").is_file());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluator_marks_passing_scenario_results_passed() {
        let (root, artifacts) = create_test_run("ouroforge-eval-pass-test");
        write_scenario_result_fixture(&artifacts.run_dir, "passed");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "passed");
        assert!(verdict.failures.is_empty());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluator_marks_failed_scenario_results_failed() {
        let (root, artifacts) = create_test_run("ouroforge-eval-fail-test");
        write_scenario_result_fixture(&artifacts.run_dir, "failed");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "failed");
        assert!(verdict
            .failures
            .iter()
            .any(|failure| failure["kind"] == "scenario_failed"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluator_marks_missing_evidence_failed() {
        let (root, artifacts) = create_test_run("ouroforge-eval-missing-evidence-test");
        add_evidence_artifact(
            &artifacts.run_dir,
            "missing-artifact",
            "application/json",
            "evidence/missing.json",
            json!({}),
        )
        .expect("missing artifact indexed");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "failed");
        assert!(verdict
            .failures
            .iter()
            .any(|failure| failure["kind"] == "missing_evidence"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluator_reports_assertion_failures_with_evidence_refs() {
        let (root, artifacts) = create_test_run("ouroforge-eval-assertion-failure-test");
        write_scenario_result_fixture(&artifacts.run_dir, "failed");
        let result_path = artifacts
            .run_dir
            .join("evidence/scenarios/bootstrap-smoke/scenario-result.json");
        fs::write(
            &result_path,
            r#"{
  "scenario_id": "bootstrap-smoke",
  "status": "failed",
  "evidence": {
    "world_state": "evidence/scenarios/bootstrap-smoke/world-state.json",
    "frame_stats": "evidence/scenarios/bootstrap-smoke/frame-stats.json"
  },
  "assertions": [
    {
      "target": "world_state",
      "path": "tick",
      "operator": "greaterThan",
      "expected": 10,
      "actual": 2,
      "passed": false,
      "evidence_ref": "evidence/scenarios/bootstrap-smoke/world-state.json"
    }
  ]
}"#,
        )
        .expect("failed scenario result written");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "failed");
        assert!(verdict.failures.iter().any(|failure| {
            failure["kind"] == "assertion_failed"
                && failure["evidence_ref"] == "evidence/scenarios/bootstrap-smoke/world-state.json"
        }));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn compares_runs_without_mutating_sources() {
        let (before_root, before) = create_test_run("ouroforge-compare-before-test");
        write_scenario_result_fixture(&before.run_dir, "passed");
        evaluate_run(&before.run_dir).expect("before verdict");
        let before_run_json =
            fs::read_to_string(before.run_dir.join("run.json")).expect("before run reads");
        let output_dir = before_root.join("comparisons");

        let artifact_path =
            write_run_comparison_artifact(&before.run_dir, &before.run_dir, &output_dir)
                .expect("comparison written");

        assert!(artifact_path.is_file());
        let comparison = read_json_value(&artifact_path).expect("comparison reads");
        assert_eq!(comparison["classification"], "no_change");
        assert_eq!(
            comparison["semantic"]["schemaVersion"],
            "run-semantic-diff-v1"
        );
        assert_eq!(
            comparison["semantic"]["reasons"][0]["summary"],
            "comparison classification is no_change"
        );
        assert_eq!(comparison["before"]["verdict_status"], "passed");
        assert_eq!(comparison["after"]["verdict_status"], "passed");
        assert_eq!(
            fs::read_to_string(before.run_dir.join("run.json")).expect("before run rereads"),
            before_run_json
        );
        fs::remove_dir_all(before_root).ok();
    }

    #[test]
    fn compares_run_regressions_from_supported_evidence() {
        let (before_root, before) = create_test_run("ouroforge-compare-regression-before-test");
        let (after_root, after) = create_test_run("ouroforge-compare-regression-after-test");
        write_scenario_result_fixture(&before.run_dir, "passed");
        write_scenario_result_fixture(&after.run_dir, "failed");
        evaluate_run(&before.run_dir).expect("before verdict");
        evaluate_run(&after.run_dir).expect("after verdict");

        let comparison = compare_runs(&before.run_dir, &after.run_dir).expect("comparison");

        assert_eq!(comparison.classification, "regressed");
        assert_eq!(comparison.before.verdict_status, "passed");
        assert_eq!(comparison.after.verdict_status, "failed");
        assert_eq!(comparison.after.failed_scenarios, 1);
        assert!(comparison
            .semantic
            .scenarios
            .iter()
            .any(
                |diff| diff.classification == "regressed" && diff.scenario_id == "bootstrap-smoke"
            ));
        assert!(comparison
            .semantic
            .reasons
            .iter()
            .any(|reason| reason.kind == "scenario_verdict" && reason.severity == "regressed"));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn compares_run_improvements_with_semantic_reasons() {
        let (before_root, before) = create_test_run("ouroforge-compare-improvement-before-test");
        let (after_root, after) = create_test_run("ouroforge-compare-improvement-after-test");
        write_scenario_result_fixture(&before.run_dir, "failed");
        write_scenario_result_fixture(&after.run_dir, "passed");
        evaluate_run(&before.run_dir).expect("before verdict");
        evaluate_run(&after.run_dir).expect("after verdict");

        let comparison = compare_runs(&before.run_dir, &after.run_dir).expect("comparison");

        assert_eq!(comparison.classification, "improved");
        assert!(comparison
            .semantic
            .scenarios
            .iter()
            .any(|diff| diff.classification == "improved"));
        assert!(comparison
            .semantic
            .reasons
            .iter()
            .any(|reason| reason.summary.contains("changed from failed to passed")));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn compares_mixed_world_state_event_and_performance_semantics() {
        let (before_root, before) = create_test_run("ouroforge-compare-mixed-before-test");
        let (after_root, after) = create_test_run("ouroforge-compare-mixed-after-test");
        write_scenario_result_fixture(&before.run_dir, "passed");
        write_scenario_result_fixture(&after.run_dir, "passed");
        fs::write(
            before
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/world-state.json"),
            r#"{"player":{"x":1},"flag":"old"}"#,
        )
        .expect("before world state written");
        fs::write(
            after
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/world-state.json"),
            r#"{"player":{"x":4},"flag":"new"}"#,
        )
        .expect("after world state written");
        fs::write(
            before
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/frame-stats.json"),
            r#"{"fps":60,"frame":1}"#,
        )
        .expect("before frame stats written");
        fs::write(
            after
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/frame-stats.json"),
            r#"{"fps":55,"frame":1}"#,
        )
        .expect("after frame stats written");
        fs::write(
            before
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/console-log.json"),
            r#"[{"level":"info","text":"ready"}]"#,
        )
        .expect("before console written");
        fs::write(
            after
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/console-log.json"),
            r#"[{"level":"info","text":"ready"},{"level":"warn","text":"changed"}]"#,
        )
        .expect("after console written");
        add_evidence_artifact(
            &before.run_dir,
            "fixture-console-log",
            "application/json",
            "evidence/scenarios/bootstrap-smoke/console-log.json",
            json!({ "artifact": "console_log", "scenario_id": "bootstrap-smoke" }),
        )
        .expect("before console indexed");
        add_evidence_artifact(
            &after.run_dir,
            "fixture-console-log",
            "application/json",
            "evidence/scenarios/bootstrap-smoke/console-log.json",
            json!({ "artifact": "console_log", "scenario_id": "bootstrap-smoke" }),
        )
        .expect("after console indexed");
        evaluate_run(&before.run_dir).expect("before verdict");
        evaluate_run(&after.run_dir).expect("after verdict");

        let comparison = compare_runs(&before.run_dir, &after.run_dir).expect("comparison");

        assert_eq!(comparison.classification, "no_change");
        assert!(comparison
            .semantic
            .world_state
            .changed
            .iter()
            .any(|diff| diff.path == "world/player/x"));
        assert!(comparison
            .semantic
            .events
            .added
            .iter()
            .any(|event| event.contains("warn:changed")));
        assert!(comparison
            .semantic
            .performance
            .changed
            .iter()
            .any(|diff| diff.path.contains("frame_stats")));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn compare_runs_warns_for_missing_optional_semantic_artifacts() {
        let (before_root, before) = create_test_run("ouroforge-compare-warning-before-test");
        let (after_root, after) = create_test_run("ouroforge-compare-warning-after-test");
        write_scenario_result_fixture(&before.run_dir, "passed");
        write_scenario_result_fixture(&after.run_dir, "passed");
        fs::remove_file(
            after
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/world-state.json"),
        )
        .expect("after world-state fixture removed");
        evaluate_run(&before.run_dir).expect("before verdict");
        evaluate_run(&after.run_dir).expect("after verdict");

        let comparison = compare_runs(&before.run_dir, &after.run_dir).expect("comparison");

        assert!(comparison
            .semantic
            .warnings
            .iter()
            .any(|warning| warning.contains("after world_state artifact could not be read")));
        assert!(comparison
            .semantic
            .reasons
            .iter()
            .any(|reason| reason.kind == "warnings"));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn compare_runs_links_transaction_provenance_semantics() {
        let (before_root, before) = create_test_run("ouroforge-compare-tx-before-test");
        let (after_root, after) = create_test_run("ouroforge-compare-tx-after-test");
        write_scenario_result_fixture(&before.run_dir, "passed");
        write_scenario_result_fixture(&after.run_dir, "passed");
        for (artifacts, transaction_id, hash) in [
            (&before, "scene-edit-before", "before-hash"),
            (&after, "scene-edit-after", "after-hash"),
        ] {
            let run_path = artifacts.run_dir.join("run.json");
            let mut run_json: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&run_path).expect("run json reads"))
                    .expect("run json parses");
            run_json["transaction_provenance"] = json!({
                "transactionId": transaction_id,
                "transactionArtifactPath": format!("transactions/{transaction_id}.json"),
                "scenePath": "examples/game-runtime/scene.json",
                "beforeSceneHash": {
                    "algorithm": "fnv1a64-canonical-json-v1",
                    "value": hash
                },
                "afterSceneHash": {
                    "algorithm": "fnv1a64-canonical-json-v1",
                    "value": hash
                }
            });
            fs::write(
                run_path,
                serde_json::to_string_pretty(&run_json).expect("run serializes"),
            )
            .expect("run updated");
        }
        evaluate_run(&before.run_dir).expect("before verdict");
        evaluate_run(&after.run_dir).expect("after verdict");

        let comparison = compare_runs(&before.run_dir, &after.run_dir).expect("comparison");

        assert!(comparison.semantic.transaction_provenance.changed);
        assert_eq!(
            comparison
                .semantic
                .transaction_provenance
                .after
                .as_ref()
                .expect("after provenance")
                .transaction_id,
            "scene-edit-after"
        );
        assert!(comparison
            .semantic
            .reasons
            .iter()
            .any(|reason| reason.kind == "transaction_provenance"));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn compare_runs_fails_clearly_for_missing_artifacts() {
        let (before_root, before) = create_test_run("ouroforge-compare-missing-before-test");
        let after_root = unique_temp_dir("ouroforge-compare-missing-after-test");
        fs::create_dir_all(&after_root).expect("after dir");

        let error = compare_runs(&before.run_dir, &after_root).expect_err("missing after fails");

        assert!(error
            .to_string()
            .contains("after run is missing required artifact"));
        fs::remove_dir_all(before_root).ok();
        fs::remove_dir_all(after_root).ok();
    }

    #[test]
    fn evaluator_fails_on_configured_console_level() {
        let (root, artifacts) = create_test_run("ouroforge-eval-console-threshold-test");
        write_scenario_result_fixture(&artifacts.run_dir, "passed");
        append_evaluator_config(
            &artifacts.run_dir,
            r#"
  console:
    failOnLevels:
      - error
"#,
        );
        let console_path = "evidence/scenarios/bootstrap-smoke/console-log.json";
        fs::write(
            artifacts.run_dir.join(console_path),
            r#"{"logs":[{"level":"error","text":"boom"}]}"#,
        )
        .expect("console log written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-console-log",
            "application/json",
            console_path,
            json!({ "artifact": "console_log", "scenario_id": "bootstrap-smoke" }),
        )
        .expect("console artifact indexed");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "failed");
        assert!(verdict
            .failures
            .iter()
            .any(|failure| failure["kind"] == "console_level_matched"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluator_fails_on_configured_performance_threshold() {
        let (root, artifacts) = create_test_run("ouroforge-eval-performance-threshold-test");
        write_scenario_result_fixture(&artifacts.run_dir, "passed");
        append_evaluator_config(
            &artifacts.run_dir,
            r#"
  performance:
    maxMetrics:
      ScriptDuration: 1
"#,
        );
        let performance_path = "evidence/scenarios/bootstrap-smoke/performance-metrics.json";
        fs::write(
            artifacts.run_dir.join(performance_path),
            r#"{"metrics":{"metrics":[{"name":"ScriptDuration","value":2.5}]}}"#,
        )
        .expect("performance metrics written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-performance-metrics",
            "application/json",
            performance_path,
            json!({ "artifact": "performance_metrics", "scenario_id": "bootstrap-smoke" }),
        )
        .expect("performance artifact indexed");

        let verdict = evaluate_run(&artifacts.run_dir).expect("evaluation succeeds");

        assert_eq!(verdict.status, "failed");
        assert!(verdict
            .failures
            .iter()
            .any(|failure| failure["kind"] == "performance_threshold_exceeded"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn evaluates_scenario_assertions_against_captured_state() {
        let scenario = Scenario {
            id: "probe-smoke".to_string(),
            description: "probe".to_string(),
            steps: Vec::new(),
            assertions: vec![
                ScenarioAssertion::WorldState {
                    world_state: json_path_equals("tick", json!(2)),
                },
                ScenarioAssertion::FrameStats {
                    frame_stats: json_path_equals("fixedDeltaMs", json!(16)),
                },
                ScenarioAssertion::WorldState {
                    world_state: json_path_equals("object.id", json!("missing")),
                },
                ScenarioAssertion::WorldState {
                    world_state: json_path_equals("collisions.0.pairId", json!("goal:player")),
                },
                ScenarioAssertion::WorldState {
                    world_state: json_path_assertion("collisions", |assertion| {
                        assertion.count_equals = Some(1);
                    }),
                },
                ScenarioAssertion::ConsoleErrors {
                    console_errors: json_path_assertion("logs", |assertion| {
                        assertion.count_equals = Some(0);
                    }),
                },
                ScenarioAssertion::PerformanceMetrics {
                    performance_metrics: json_path_assertion("ScriptDuration", |assertion| {
                        assertion.less_than = Some(json!(10));
                    }),
                },
            ],
        };
        let world_state = json!({
            "tick": 2,
            "object": { "id": "probe-square" },
            "collisions": [{ "pairId": "goal:player" }]
        });
        let frame_stats = json!({ "fixedDeltaMs": 16 });
        let runtime_events = json!({ "stepCount": 0 });
        let performance_metrics = json!({ "ScriptDuration": 2.5 });
        let console_errors = json!({ "logs": [], "count": 0 });
        let none = serde_json::Value::Null;
        let sources = assertion_sources_for_test(
            &world_state,
            &frame_stats,
            &runtime_events,
            &performance_metrics,
            &console_errors,
            &world_state["collisions"],
            &none,
            &none,
        );

        let assertions = evaluate_scenario_assertions(&scenario, &sources);

        assert_eq!(assertions.len(), 7);
        assert_eq!(assertions[0]["passed"], true);
        assert_eq!(assertions[1]["passed"], true);
        assert_eq!(assertions[2]["passed"], false);
        assert_eq!(assertions[2]["actual"], "probe-square");
        assert_eq!(assertions[3]["passed"], true);
        assert_eq!(assertions[4]["operator"], "countEquals");
        assert_eq!(assertions[4]["evidence_ref"], "evidence/world-state.json");
        assert_eq!(assertions[5]["passed"], true);
        assert_eq!(assertions[6]["passed"], true);
    }

    #[test]
    fn scenario_steps_call_runtime_probe_api() {
        let mut client = CdpClient::new(RecordingRuntimeTransport { calls: Vec::new() });

        execute_scenario_step(
            &mut client,
            &ScenarioStep::Wait {
                wait: WaitStep { frames: 3 },
            },
        )
        .expect("wait executes");
        execute_scenario_step(
            &mut client,
            &ScenarioStep::Input {
                input: InputStep {
                    right: Some(true),
                    ..InputStep::default()
                },
            },
        )
        .expect("input executes");
        execute_scenario_step(
            &mut client,
            &ScenarioStep::Replay {
                replay: InputReplay {
                    schema_version: "1".to_string(),
                    id: "move-right".to_string(),
                    events: vec![
                        InputReplayEvent {
                            frame: 0,
                            key: ReplayKey::Right,
                            pressed: true,
                        },
                        InputReplayEvent {
                            frame: 4,
                            key: ReplayKey::Right,
                            pressed: false,
                        },
                    ],
                },
            },
        )
        .expect("replay executes");

        let transport = client.into_transport();
        let readiness = "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.step === 'function' && typeof window.__OUROFORGE__.setInput === 'function')";
        assert_eq!(transport.calls[0], readiness);
        assert_eq!(transport.calls[1], "window.__OUROFORGE__.step(3)");
        assert_eq!(transport.calls[2], readiness);
        assert_eq!(
            transport.calls[3],
            "window.__OUROFORGE__.setInput({\"right\":true})"
        );
        assert_eq!(transport.calls[4], readiness);
        assert_eq!(transport.calls[5], readiness);
        assert_eq!(
            transport.calls[6],
            "window.__OUROFORGE__.setInput({\"right\":true})"
        );
        assert_eq!(transport.calls[7], readiness);
        assert_eq!(transport.calls[8], "window.__OUROFORGE__.step(4)");
        assert_eq!(transport.calls[9], readiness);
        assert_eq!(
            transport.calls[10],
            "window.__OUROFORGE__.setInput({\"right\":false})"
        );
    }

    #[test]
    fn browser_smoke_pool_reports_each_worker_failure() {
        let (root, artifacts) = create_test_run("ouroforge-browser-pool-failure-test");
        let mut base = BrowserSmokeConfig::new(&artifacts.run_dir, "http://127.0.0.1:8765")
            .expect("config builds");
        base.debugging_http_url = "http://127.0.0.1:9".to_string();
        let pool = BrowserSmokePoolConfig::new(base, 3).expect("pool config builds");

        let result = run_browser_smoke_pool(&pool);

        assert_eq!(result.workers, 3);
        assert_eq!(result.succeeded, 0);
        assert_eq!(result.failed, 3);
        assert_eq!(
            result
                .outcomes
                .iter()
                .map(|outcome| outcome.worker_id.as_str())
                .collect::<Vec<_>>(),
            vec!["worker-1", "worker-2", "worker-3"]
        );
        assert!(result.outcomes.iter().all(|outcome| !outcome.ok));

        let events = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        let failed_workers: Vec<_> = events
            .iter()
            .filter(|event| event["event"] == "browser.worker.failed")
            .filter_map(|event| event["payload"]["worker_id"].as_str())
            .collect();
        assert_eq!(failed_workers.len(), 3);
        assert!(failed_workers.contains(&"worker-1"));
        assert!(failed_workers.contains(&"worker-2"));
        assert!(failed_workers.contains(&"worker-3"));

        fs::remove_dir_all(root).ok();
    }

    struct ScreenshotTransport;

    impl CdpTransport for ScreenshotTransport {
        fn send_command(
            &mut self,
            method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            match method {
                "Page.captureScreenshot" => Ok(json!({
                    "data": base64::engine::general_purpose::STANDARD.encode(test_png_bytes(320, 180))
                })),
                _ => Ok(json!({})),
            }
        }
    }

    #[test]
    fn cdp_client_decodes_screenshot_data() {
        let mut client = CdpClient::new(ScreenshotTransport);

        let bytes = client.capture_screenshot_png().expect("screenshot decodes");

        assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(png_dimensions(&bytes), Some((320, 180)));
    }

    #[test]
    fn captures_visual_checkpoint_screenshot_as_evidence() {
        let (root, artifacts) = create_test_run("ouroforge-visual-checkpoint-test");
        let config = ScenarioRunConfig::new(&artifacts.run_dir, "http://127.0.0.1:8080")
            .expect("scenario config");
        let scenario = Scenario {
            id: "visual-smoke".to_string(),
            description: "visual".to_string(),
            steps: Vec::new(),
            assertions: Vec::new(),
        };
        let scenario_dir = "evidence/scenarios/visual-smoke";
        fs::create_dir_all(artifacts.run_dir.join(scenario_dir)).expect("scenario dir");
        let mut client = CdpClient::new(ScreenshotTransport);

        let capture = capture_visual_checkpoint(
            &config,
            &scenario,
            scenario_dir,
            &VisualCheckpointStep {
                id: "after-load".to_string(),
                baseline: None,
                threshold: None,
            },
            &mut client,
        )
        .expect("visual checkpoint captured");

        assert!(artifacts.run_dir.join(&capture.screenshot_path).is_file());
        assert!(artifacts.run_dir.join(&capture.metadata_path).is_file());
        let metadata =
            read_json_value(artifacts.run_dir.join(&capture.metadata_path)).expect("metadata");
        assert_eq!(metadata["checkpoint_id"], "after-load");
        assert_eq!(metadata["screenshot_path"], capture.screenshot_path);
        assert_eq!(metadata["advisory"], true);
        assert_eq!(
            metadata["dimensions"],
            json!({ "width": 320, "height": 180 })
        );
        assert_eq!(metadata["comparison"]["advisory"], true);
        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        assert!(artifacts_list.iter().any(|artifact| {
            artifact.metadata["artifact"] == "visual_checkpoint_screenshot"
                && artifact.kind == "image/png"
        }));
        assert!(artifacts_list
            .iter()
            .any(|artifact| artifact.metadata["artifact"] == "visual_checkpoint"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn visual_checkpoint_threshold_can_fail_deterministically() {
        let checkpoint = VisualCheckpointStep {
            id: "after-load".to_string(),
            baseline: Some(VisualBaselineMetadata {
                id: Some("baseline-a".to_string()),
                path: None,
                width: Some(640),
                height: Some(480),
            }),
            threshold: Some(VisualThreshold {
                max_dimension_delta: 0,
            }),
        };

        let comparison = visual_comparison_summary(&checkpoint, Some((320, 180)));

        assert_eq!(comparison["advisory"], false);
        assert_eq!(comparison["passed"], false);
        assert_eq!(comparison["width_delta"], 320);
        assert_eq!(comparison["height_delta"], 300);
    }

    struct SuiteScenarioTransport {
        calls: Vec<String>,
    }

    impl CdpTransport for SuiteScenarioTransport {
        fn send_command(
            &mut self,
            method: &str,
            params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            match method {
                "Runtime.evaluate" => {
                    let expression = params["expression"]
                        .as_str()
                        .expect("expression present")
                        .to_string();
                    self.calls.push(expression.clone());
                    let value = if expression.contains("getWorldState") {
                        json!({ "object": { "x": 32 } })
                    } else if expression.contains("getFrameStats") {
                        json!({ "fixedDeltaMs": 16 })
                    } else if expression.contains("__OUROFORGE_CONSOLE__") {
                        json!([])
                    } else {
                        json!({})
                    };
                    Ok(json!({ "result": { "value": value } }))
                }
                "Performance.enable" => Ok(json!({})),
                "Performance.getMetrics" => Ok(json!({ "metrics": [] })),
                _ => Ok(json!({})),
            }
        }
    }

    #[test]
    fn suite_execution_preserves_document_order_and_failure_isolation() {
        let (root, artifacts) = create_test_run("ouroforge-suite-order-test");
        let seed = Seed::from_yaml_str(
            r#"
id: suite.v1
title: Suite Fixture
goal: Run multiple scenarios deterministically.
constraints:
  target: file-harness
acceptance:
  - Preserve scenario evidence.
scenarios:
  - id: first-pass
    description: First scenario passes.
    assertions:
      - world_state:
          path: object.x
          equals: 32
  - id: second-fail
    description: Second scenario fails without removing first evidence.
    assertions:
      - world_state:
          path: object.x
          equals: 999
"#,
        )
        .expect("suite seed parses");
        let config = ScenarioRunConfig::new(&artifacts.run_dir, "http://127.0.0.1:8080")
            .expect("scenario config");
        let mut client = CdpClient::new(SuiteScenarioTransport { calls: Vec::new() });

        let summary = run_scenarios_with_client(&config, &seed, &mut client).expect("suite runs");

        assert_eq!(summary.scenario_order, vec!["first-pass", "second-fail"]);
        assert_eq!(summary.completed, 2);
        assert_eq!(summary.passed, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.result_paths.len(), 2);
        assert!(artifacts
            .run_dir
            .join(&summary.suite_summary_path)
            .is_file());
        let suite_summary = read_json_value(artifacts.run_dir.join(&summary.suite_summary_path))
            .expect("suite summary reads");
        assert_eq!(suite_summary["status"], "failed");
        assert_eq!(
            suite_summary["scenario_order"],
            json!(["first-pass", "second-fail"])
        );
        assert_eq!(
            suite_summary["scenario_results"].as_array().unwrap().len(),
            2
        );
        assert!(summary
            .result_paths
            .iter()
            .any(|path| path.contains("first-pass")));
        assert!(summary
            .result_paths
            .iter()
            .any(|path| path.contains("second-fail")));
        assert!(artifacts
            .run_dir
            .join("evidence/scenarios/first-pass")
            .is_dir());
        assert!(artifacts
            .run_dir
            .join("evidence/scenarios/second-fail")
            .is_dir());
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn parses_cdp_websocket_endpoint() {
        let endpoint = CdpWebSocketEndpoint::parse("ws://127.0.0.1:9222/devtools/page/page-1")
            .expect("websocket endpoint parses");
        assert_eq!(endpoint.host, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(endpoint.port, 9222);
    }

    #[test]
    fn formats_ipv6_host_authority_with_brackets() {
        assert_eq!(
            format_host_authority("::1".parse::<IpAddr>().unwrap(), 9222),
            "[::1]:9222"
        );
        assert_eq!(
            format_host_authority("127.0.0.1".parse::<IpAddr>().unwrap(), 9222),
            "127.0.0.1:9222"
        );
    }

    #[test]
    fn parses_ipv6_cdp_endpoints() {
        let http = CdpHttpEndpoint::parse("http://[::1]:9222/").expect("ipv6 http parses");
        assert_eq!(http.host, "::1".parse::<IpAddr>().unwrap());
        assert_eq!(http.port, 9222);

        let websocket = CdpWebSocketEndpoint::parse("ws://[::1]:9222/devtools/page/page-1")
            .expect("ipv6 websocket parses");
        assert_eq!(websocket.host, "::1".parse::<IpAddr>().unwrap());
        assert_eq!(websocket.port, 9222);
    }

    #[test]
    fn parses_cdp_http_endpoint() {
        let endpoint = CdpHttpEndpoint::parse("http://127.0.0.1:9222/").expect("endpoint parses");
        assert_eq!(endpoint.host, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(endpoint.port, 9222);
    }

    #[test]
    fn rejects_hostname_cdp_endpoint() {
        let error =
            CdpHttpEndpoint::parse("http://localhost:9222").expect_err("hostname endpoint fails");
        assert!(error
            .to_string()
            .contains("must be a numeric loopback IP address"));
    }

    #[test]
    fn rejects_non_http_cdp_endpoint() {
        let error =
            CdpHttpEndpoint::parse("https://127.0.0.1:9222").expect_err("https endpoint fails");
        assert!(error.to_string().contains("must start with http://"));
    }

    #[test]
    fn parses_first_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"browser",
                "type":"browser",
                "url":"",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/browser/abc"
              },
              {
                "id":"page-1",
                "type":"page",
                "url":"about:blank",
                "title":"New Tab",
                "description":"",
                "devtoolsFrontendUrl":"/devtools/inspector.html?ws=127.0.0.1:9222/devtools/page/page-1",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-1"
              }
            ]"#,
        )
        .expect("targets parse");

        let config = first_page_target(&targets).expect("page target selected");
        assert_eq!(
            config.target_ws_url,
            "ws://127.0.0.1:9222/devtools/page/page-1"
        );
    }

    #[test]
    fn scene_hash_is_deterministic_for_canonical_scene_json() {
        let temp = unique_temp_dir("ouroforge-scene-hash-test");
        fs::create_dir_all(&temp).expect("temp dir exists");
        let scene_path = temp.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        let scene = read_scene(&scene_path).expect("scene parses");
        let first = hash_scene_document(&scene).expect("hash created");
        let second = hash_scene_document(&scene).expect("hash repeats");
        assert_eq!(first, second);
        assert_eq!(first.algorithm, "fnv1a64-canonical-json-v1");
        assert_eq!(first.value.len(), 16);
        fs::remove_dir_all(temp).ok();
    }

    #[test]
    fn scene_edit_transaction_records_success_and_rollback_metadata() {
        let temp = unique_temp_dir("ouroforge-scene-transaction-success-test");
        fs::create_dir_all(&temp).expect("temp dir exists");
        let scene_path = temp.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        let transaction = preview_scene_edit_transaction(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
        )
        .expect("transaction previews");
        assert_eq!(
            transaction.schema_version,
            "ouroforge.scene-edit-transaction.v1"
        );
        assert_eq!(transaction.validation_result.status, "passed");
        assert!(transaction.validation_result.errors.is_empty());
        assert!(transaction.after_scene_hash.is_some());
        assert_ne!(
            transaction.before_scene_hash,
            transaction
                .after_scene_hash
                .clone()
                .expect("after hash exists")
        );
        assert_eq!(
            transaction.rollback.restore_hash,
            transaction.before_scene_hash
        );
        assert!(transaction.rollback.strategy.contains("beforeSceneHash"));
        let persisted = read_scene(&scene_path).expect("scene still parses");
        assert_eq!(
            persisted.entities[0].components.transform.x, 32,
            "preview must not write"
        );
        fs::remove_dir_all(temp).ok();
    }

    #[test]
    fn scene_edit_transaction_records_failed_validation_without_after_hash() {
        let temp = unique_temp_dir("ouroforge-scene-transaction-failure-test");
        fs::create_dir_all(&temp).expect("temp dir exists");
        let scene_path = temp.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        let transaction = preview_scene_edit_transaction(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.size.width".to_string(),
                value: json!(0),
            },
        )
        .expect("failed validation still records transaction");
        assert_eq!(transaction.validation_result.status, "failed");
        assert!(transaction.after_scene_hash.is_none());
        assert!(transaction.validation_result.errors[0].contains("size must be positive"));
        let persisted = read_scene(&scene_path).expect("scene still parses");
        assert_eq!(
            persisted.entities[0].components.size.width, 16,
            "failed preview must not write"
        );
        fs::remove_dir_all(temp).ok();
    }

    #[test]
    fn scene_only_mutation_operation_validates_allowed_edit_without_writing() {
        let (root, artifacts, proposal, scene_path, before_hash) =
            create_scene_only_mutation_fixture("scene-only-mutation-valid");
        let operation = SceneOnlyMutationOperation {
            schema_version: "scene-only-mutation-v1".to_string(),
            proposal_id: proposal.id,
            target_scene_path: scene_path.to_string_lossy().to_string(),
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
            expected_before_scene_hash: before_hash.clone(),
            validation_required: true,
        };

        let validation = validate_scene_only_mutation_operation(&artifacts.run_dir, &operation)
            .expect("scene-only mutation validates");

        assert_eq!(validation.status, "passed");
        assert_eq!(validation.before_scene_hash, before_hash);
        assert!(validation.allowed_path);
        let persisted = read_scene(&scene_path).expect("scene still parses");
        assert_eq!(
            persisted.entities[0].components.transform.x, 32,
            "schema validation must not write trusted scene state"
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn scene_only_mutation_operation_rejects_forbidden_path_and_stale_hash() {
        let (root, artifacts, proposal, scene_path, before_hash) =
            create_scene_only_mutation_fixture("scene-only-mutation-rejects");
        let forbidden = SceneOnlyMutationOperation {
            schema_version: "scene-only-mutation-v1".to_string(),
            proposal_id: proposal.id.clone(),
            target_scene_path: scene_path.to_string_lossy().to_string(),
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "metadata.debug.mode".to_string(),
                value: json!("unsafe"),
            },
            expected_before_scene_hash: before_hash,
            validation_required: true,
        };
        let forbidden_error =
            validate_scene_only_mutation_operation(&artifacts.run_dir, &forbidden)
                .expect_err("forbidden path rejected");
        assert!(forbidden_error.to_string().contains("not allowed"));

        let stale = SceneOnlyMutationOperation {
            expected_before_scene_hash: SceneHash {
                algorithm: "fnv1a64-canonical-json-v1".to_string(),
                value: "0000000000000000".to_string(),
            },
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
            ..forbidden
        };
        let stale_error = validate_scene_only_mutation_operation(&artifacts.run_dir, &stale)
            .expect_err("stale hash rejected");
        assert!(stale_error.to_string().contains("before hash mismatch"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn scene_only_mutation_operation_rejects_malformed_or_validation_failure() {
        let (root, artifacts, proposal, scene_path, before_hash) =
            create_scene_only_mutation_fixture("scene-only-mutation-malformed");
        let malformed = serde_json::from_value::<SceneOnlyMutationOperation>(json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal.id.clone(),
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entity_id": "player", "path": "components.transform.x", "value": 48 },
            "expectedBeforeSceneHash": before_hash.clone(),
            "validationRequired": true,
            "extra": "rejected"
        }))
        .expect_err("unknown fields rejected");
        assert!(malformed.to_string().contains("unknown field"));

        let operation = SceneOnlyMutationOperation {
            schema_version: "scene-only-mutation-v1".to_string(),
            proposal_id: proposal.id,
            target_scene_path: scene_path.to_string_lossy().to_string(),
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "components.size.width".to_string(),
                value: json!(0),
            },
            expected_before_scene_hash: before_hash,
            validation_required: true,
        };
        let before_contents = fs::read_to_string(&scene_path).expect("scene before reads");
        let error = validate_scene_only_mutation_operation(&artifacts.run_dir, &operation)
            .expect_err("candidate validation failure rejected");
        assert!(error.to_string().contains("candidate validation failed"));
        assert_eq!(
            fs::read_to_string(&scene_path).expect("scene after reads"),
            before_contents,
            "failed validation must leave trusted scene unchanged"
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn scene_only_mutation_application_writes_transaction_and_fails_safely() {
        let (root, artifacts, proposal, scene_path, before_hash) =
            create_scene_only_mutation_fixture("scene-only-mutation-apply");
        let transaction_path = root.join("transactions/apply.json");
        let operation = SceneOnlyMutationOperation {
            schema_version: "scene-only-mutation-v1".to_string(),
            proposal_id: proposal.id.clone(),
            target_scene_path: scene_path.to_string_lossy().to_string(),
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
            expected_before_scene_hash: before_hash.clone(),
            validation_required: true,
        };
        let transaction =
            apply_scene_only_mutation_operation(&artifacts.run_dir, &operation, &transaction_path)
                .expect("scene-only mutation applies");
        assert_eq!(transaction.validation_result.status, "passed");
        assert!(transaction_path.is_file());
        assert_eq!(
            read_scene(&scene_path).expect("scene reads").entities[0]
                .components
                .transform
                .x,
            48
        );
        let ledger = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        assert!(ledger
            .iter()
            .any(|event| event["event"] == "mutation.scene_applied"));
        let applications =
            read_json_value(artifacts.run_dir.join("mutation/scene-applications.json"))
                .expect("scene applications artifact reads");
        assert_eq!(applications["applications"][0]["proposalId"], proposal.id);
        assert_eq!(
            applications["applications"][0]["transactionId"],
            transaction.id
        );
        assert_eq!(applications["applications"][0]["status"], "applied");
        let lifecycle = read_dashboard_run(&artifacts.run_dir)
            .expect("dashboard reads")
            .mutation_lifecycle;
        let scene_applied = lifecycle
            .stages
            .iter()
            .find(|stage| stage.id == "scene_applied")
            .expect("scene applied lifecycle stage exists");
        assert_eq!(scene_applied.state, "applied");
        assert_eq!(scene_applied.record_count, 1);
        assert!(read_dashboard_run(&artifacts.run_dir)
            .expect("dashboard reads")
            .mutation_artifacts
            .iter()
            .any(|artifact| artifact.id == "mutation-scene-applications"));

        let failed_path = root.join("transactions/failure.json");
        let failing_operation = SceneOnlyMutationOperation {
            edit: SceneEdit {
                entity_id: "player".to_string(),
                path: "components.size.width".to_string(),
                value: json!(0),
            },
            expected_before_scene_hash: hash_scene_document(
                &read_scene(&scene_path).expect("updated scene reads"),
            )
            .expect("updated hash"),
            ..operation
        };
        let error = apply_scene_only_mutation_operation(
            &artifacts.run_dir,
            &failing_operation,
            &failed_path,
        )
        .expect_err("validation failure rejected");
        assert!(error.to_string().contains("transaction failed validation"));
        assert!(failed_path.is_file());
        assert_eq!(
            read_scene(&scene_path).expect("scene reads").entities[0]
                .components
                .size
                .width,
            16
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn scene_edit_model_validates_and_preserves_scene_shape() {
        let root = unique_temp_dir("scene-edit-model");
        fs::create_dir_all(&root).expect("temp root exists");
        let scene_path = root.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene fixture written");

        let scene = read_scene(&scene_path).expect("scene reads");
        assert_eq!(scene.entities[0].id, "player");
        assert_eq!(
            supported_scene_edit_paths(),
            &[
                "sprite.color",
                "components.transform.x",
                "components.transform.y",
                "components.velocity.x",
                "components.velocity.y",
                "components.size.width",
                "components.size.height",
                "components.controllable"
            ]
        );

        let edited = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(48),
            },
        )
        .expect("transform edit applies");
        assert_eq!(edited.entities[0].components.transform.x, 48);

        let edited = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "sprite.color".to_string(),
                value: json!("#ffffff"),
            },
        )
        .expect("color edit applies");
        assert_eq!(edited.entities[0].sprite.color, "#ffffff");

        let supported_edits = [
            ("components.transform.y", json!(80)),
            ("components.velocity.x", json!(3)),
            ("components.velocity.y", json!(-2)),
            ("components.size.width", json!(20)),
            ("components.size.height", json!(18)),
            ("components.controllable", json!(false)),
        ];
        for (path, value) in supported_edits {
            edit_scene(
                &scene_path,
                SceneEdit {
                    entity_id: "player".to_string(),
                    path: path.to_string(),
                    value,
                },
            )
            .unwrap_or_else(|error| panic!("supported edit {path} should apply: {error}"));
        }
        let edited = read_scene(&scene_path).expect("edited scene reads");
        let player = &edited.entities[0];
        assert_eq!(player.components.transform.y, 80);
        assert_eq!(player.components.velocity.x, 3);
        assert_eq!(player.components.velocity.y, -2);
        assert_eq!(player.components.size.width, 20);
        assert_eq!(player.components.size.height, 18);
        assert!(!player.components.controllable);

        let before_invalid = fs::read_to_string(&scene_path).expect("scene before invalid read");
        let rejected = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.size.width".to_string(),
                value: json!(0),
            },
        )
        .expect_err("invalid size rejected");
        assert!(rejected.to_string().contains("size must be positive"));
        assert_eq!(
            fs::read_to_string(&scene_path).expect("scene after invalid read"),
            before_invalid,
            "invalid Rust-validated edit must not rewrite the scene file"
        );

        let rejected = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.transform.x".to_string(),
                value: json!("48"),
            },
        )
        .expect_err("invalid transform type rejected");
        assert!(rejected.to_string().contains("requires an integer value"));

        let rejected = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "sprite.color".to_string(),
                value: json!("white"),
            },
        )
        .expect_err("invalid color rejected");
        assert!(rejected.to_string().contains("#RRGGBB"));

        let rejected = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "missing".to_string(),
                path: "sprite.color".to_string(),
                value: json!("#000000"),
            },
        )
        .expect_err("missing entity rejected");
        assert!(rejected.to_string().contains("scene entity not found"));

        let rejected = edit_scene(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: "components.script".to_string(),
                value: json!("future"),
            },
        )
        .expect_err("unsupported edit rejected");
        assert!(rejected.to_string().contains("unsupported scene edit path"));
        assert!(rejected.to_string().contains("components.controllable"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn scene_schema_v1_accepts_entities_components_and_metadata() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "metadata": { "title": "fixture" },
            "entities": [
                {
                    "id": "player",
                    "sprite": {
                        "color": "#5eead4",
                        "asset": "assets/sprites/player.png"
                    },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "collider": {
                            "shape": "aabb",
                            "body": "dynamic",
                            "offset": { "x": 0, "y": 0 },
                            "size": { "width": 16, "height": 16 },
                            "sensor": false
                        },
                        "animation": {
                            "mode": "sprite_frame",
                            "frameDuration": 2,
                            "loop": true,
                            "frames": [
                                { "color": "#5eead4" },
                                { "color": "#2dd4bf" }
                            ]
                        },
                        "audio": {
                            "events": [
                                {
                                    "name": "player_spawn",
                                    "trigger": "scene_loaded",
                                    "asset": "assets/audio/player_spawn.ogg"
                                }
                            ]
                        }
                    },
                    "tags": ["player", "spawn"],
                    "metadata": { "role": "hero" }
                },
                {
                    "id": "goal",
                    "sprite": { "color": "#facc15" },
                    "components": {
                        "transform": { "x": 272, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": false
                    },
                    "tags": ["goal"],
                    "metadata": {}
                }
            ]
        }))
        .expect("scene fixture parses");

        validate_scene(&scene).expect("scene schema v1 validates");
        assert_eq!(scene.schema_version, "1");
        assert_eq!(scene.entities.len(), 2);
        assert_eq!(
            scene.entities[0]
                .components
                .collider
                .as_ref()
                .unwrap()
                .shape,
            "aabb"
        );
        let animation = scene.entities[0].components.animation.as_ref().unwrap();
        assert_eq!(animation.mode, "sprite_frame");
        assert_eq!(animation.frame_duration, 2);
        assert_eq!(animation.frames.len(), 2);
        let audio = scene.entities[0].components.audio.as_ref().unwrap();
        assert_eq!(audio.events[0].name, "player_spawn");
        assert_eq!(audio.events[0].trigger, "scene_loaded");
    }

    #[test]
    fn scene_tilemap_v1_accepts_bounded_grid_and_deterministic_layer_order() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 3, "height": 2 },
                    "tiles": [
                        { "id": "empty", "color": "#172532" },
                        { "id": "solid", "color": "#334155", "asset": "assets/tiles/solid.svg", "solid": true },
                        { "id": "hazard", "color": "#ef4444" }
                    ],
                    "layers": [
                        {
                            "id": "foreground",
                            "order": 10,
                            "data": [null, null, "hazard", null, "solid", null],
                            "collisionLayer": "collision"
                        },
                        {
                            "id": "collision",
                            "order": 0,
                            "visible": false,
                            "data": [null, null, null, null, "solid", null]
                        },
                        {
                            "id": "background",
                            "order": -10,
                            "data": ["empty", "empty", "empty", "empty", "empty", "empty"],
                            "metadata": { "role": "backdrop" }
                        }
                    ],
                    "metadata": { "purpose": "tilemap fixture" }
                }
            ],
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("tilemap scene parses");

        validate_scene(&scene).expect("tilemap v1 validates");
        assert_eq!(scene.tilemaps[0].tile_size.width, 16);
        assert_eq!(scene.tilemaps[0].grid.width, 3);
        assert!(scene.tilemaps[0].tiles.iter().any(|tile| tile.solid));
        assert_eq!(
            scene_tilemap_layer_order(&scene)
                .iter()
                .map(|entry| entry.layer_id.as_str())
                .collect::<Vec<_>>(),
            vec!["background", "foreground"]
        );
        assert_eq!(
            serde_json::to_string(&scene_tilemap_layer_order(&scene))
                .expect("tilemap layer order serializes"),
            r#"[{"tilemapId":"level","layerId":"background","order":-10},{"tilemapId":"level","layerId":"foreground","order":10}]"#
        );
    }

    #[test]
    fn asset_manifest_v1_accepts_local_assets_and_deterministic_resolution() {
        let root = unique_temp_dir("asset-manifest-v1-valid");
        fs::create_dir_all(root.join("assets/sprites")).expect("sprites dir");
        fs::create_dir_all(root.join("assets/images")).expect("images dir");
        fs::create_dir_all(root.join("assets/audio")).expect("audio dir");
        fs::write(root.join("assets/sprites/player.svg"), b"<svg></svg>").expect("sprite");
        fs::write(root.join("assets/images/goal.png"), test_png_bytes(1, 1)).expect("image");
        fs::write(root.join("assets/audio/spawn.ogg"), b"ogg fixture").expect("audio");

        let manifest_path = root.join("assets.manifest.json");
        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "1",
                "id": "runtime-v1-assets",
                "assets": [
                    {
                        "id": "spawn-sound",
                        "kind": "audio",
                        "path": "assets/audio/spawn.ogg",
                        "metadata": { "purpose": "audio fixture" }
                    },
                    {
                        "id": "player-sprite",
                        "kind": "sprite",
                        "path": "assets/sprites/player.svg"
                    },
                    {
                        "id": "goal-image",
                        "kind": "image",
                        "path": "assets/images/goal.png"
                    }
                ],
                "metadata": { "scene": "runtime" }
            }))
            .expect("manifest serializes"),
        )
        .expect("manifest written");

        let manifest = AssetManifest::from_path(&manifest_path).expect("manifest validates");
        assert_eq!(manifest.schema_version, "1");
        assert_eq!(manifest.assets.len(), 3);
        let resolved = manifest.resolved_assets();
        assert_eq!(
            resolved
                .iter()
                .map(|asset| asset.id.as_str())
                .collect::<Vec<_>>(),
            vec!["goal-image", "player-sprite", "spawn-sound"]
        );
        assert_eq!(
            serde_json::to_string(&resolved).expect("resolved assets serialize"),
            r#"[{"id":"goal-image","kind":"image","path":"assets/images/goal.png"},{"id":"player-sprite","kind":"sprite","path":"assets/sprites/player.svg"},{"id":"spawn-sound","kind":"audio","path":"assets/audio/spawn.ogg"}]"#
        );

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn asset_manifest_v1_rejects_invalid_entries_and_paths() {
        let root = unique_temp_dir("asset-manifest-v1-invalid");
        fs::create_dir_all(root.join("assets/sprites")).expect("sprites dir");
        fs::write(root.join("assets/sprites/player.svg"), b"<svg></svg>").expect("sprite");
        let valid_manifest = || -> AssetManifest {
            serde_json::from_value(json!({
                "schemaVersion": "1",
                "id": "runtime-v1-assets",
                "assets": [
                    { "id": "player-sprite", "kind": "sprite", "path": "assets/sprites/player.svg" }
                ]
            }))
            .expect("manifest parses")
        };

        let mut duplicate_id = valid_manifest();
        duplicate_id.assets.push(AssetManifestEntry {
            id: "player-sprite".to_string(),
            kind: AssetManifestKind::Image,
            path: "assets/sprites/player-copy.svg".to_string(),
            metadata: json!({}),
        });
        let rejected = duplicate_id.validate().expect_err("duplicate id rejected");
        assert!(rejected
            .to_string()
            .contains("duplicate asset manifest asset id"));

        let mut duplicate_path = valid_manifest();
        duplicate_path.assets.push(AssetManifestEntry {
            id: "player-sprite-copy".to_string(),
            kind: AssetManifestKind::Sprite,
            path: "assets/sprites/player.svg".to_string(),
            metadata: json!({}),
        });
        let rejected = duplicate_path
            .validate()
            .expect_err("duplicate path rejected");
        assert!(rejected
            .to_string()
            .contains("duplicate asset manifest asset path"));

        let mut unsafe_path = valid_manifest();
        unsafe_path.assets[0].path = "assets/../outside.svg".to_string();
        let rejected = unsafe_path.validate().expect_err("unsafe path rejected");
        assert!(rejected
            .to_string()
            .contains("must stay inside the local scene asset tree"));

        let mut unsupported_type = valid_manifest();
        unsupported_type.assets[0].path = "assets/sprites/player.txt".to_string();
        let rejected = unsupported_type
            .validate()
            .expect_err("unsupported type rejected");
        assert!(rejected.to_string().contains("unsupported extension"));

        let mut missing_file = valid_manifest();
        missing_file.assets[0].path = "assets/sprites/missing.svg".to_string();
        let rejected = missing_file
            .validate_files(&root)
            .expect_err("missing file rejected");
        assert!(rejected.to_string().contains("missing file"));

        let mut remote_url = valid_manifest();
        remote_url.assets[0].path = "https://example.com/player.svg".to_string();
        let rejected = remote_url.validate().expect_err("remote URL rejected");
        assert!(rejected
            .to_string()
            .contains("must be a local static asset path"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn scene_composition_v2_validates_parent_refs_and_defaults() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "composition-scene",
            "bounds": { "width": 320, "height": 180 },
            "componentDefaults": {
                "velocity": { "x": 0, "y": 0 },
                "size": { "width": 16, "height": 16 },
                "controllable": false
            },
            "entities": [
                {
                    "id": "ship",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 100, "y": 50 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 32, "height": 16 },
                        "controllable": true
                    }
                },
                {
                    "id": "turret",
                    "parent": "ship",
                    "sprite": { "color": "#facc15" },
                    "components": {
                        "transform": { "x": 8, "y": -4 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 8, "height": 8 },
                        "controllable": false
                    }
                }
            ]
        }))
        .expect("composition scene parses");
        validate_scene(&scene).expect("composition validates");
        assert_eq!(scene.entities[1].parent.as_deref(), Some("ship"));
        assert_eq!(
            scene.component_defaults.as_ref().unwrap().controllable,
            Some(false)
        );

        let mut missing_parent = scene.clone();
        missing_parent.entities[1].parent = Some("missing".to_string());
        let rejected = validate_scene(&missing_parent).expect_err("missing parent rejected");
        assert!(rejected.to_string().contains("references missing parent"));

        let mut self_parent = scene.clone();
        self_parent.entities[0].parent = Some("ship".to_string());
        let rejected = validate_scene(&self_parent).expect_err("self parent rejected");
        assert!(rejected
            .to_string()
            .contains("parent must not reference itself"));

        let mut cycle = scene.clone();
        cycle.entities[0].parent = Some("turret".to_string());
        let rejected = validate_scene(&cycle).expect_err("cycle rejected");
        assert!(rejected.to_string().contains("composition cycle detected"));

        let mut invalid_defaults = scene.clone();
        invalid_defaults.component_defaults.as_mut().unwrap().size = Some(SceneSize {
            width: 0,
            height: 16,
        });
        let rejected = validate_scene(&invalid_defaults).expect_err("invalid defaults rejected");
        assert!(rejected
            .to_string()
            .contains("componentDefaults size must be positive"));
    }

    #[test]
    fn scene_reload_validation_reports_contract_boundary() {
        let root = unique_temp_dir("scene-reload-valid");
        fs::create_dir_all(root.join("assets/sprites")).expect("asset dirs created");
        fs::write(root.join("assets/sprites/player.svg"), "<svg></svg>").expect("asset written");
        let scene_path = root.join("scene.json");
        fs::write(
            &scene_path,
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "1",
                "id": "reload-scene",
                "bounds": { "width": 320, "height": 180 },
                "assetManifest": {
                    "schemaVersion": "1",
                    "id": "reload-assets",
                    "assets": [{ "id": "player-sprite", "kind": "sprite", "path": "assets/sprites/player.svg" }]
                },
                "entities": [{
                    "id": "player",
                    "sprite": { "color": "#5eead4", "asset": "player-sprite" },
                    "components": {
                        "transform": { "x": 0, "y": 0 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }]
            }))
            .expect("scene serializes"),
        )
        .expect("scene written");

        let report = validate_scene_reload(&scene_path).expect("reload payload validates");
        assert_eq!(report.schema_version, "ouroforge.scene-reload.v0");
        assert_eq!(report.scene_id, "reload-scene");
        assert_eq!(report.entity_count, 1);
        assert_eq!(
            report.asset_manifest,
            Some(SceneReloadAssetManifestReport {
                id: "reload-assets".to_string(),
                asset_count: 1
            })
        );
        assert!(report.reset_state.contains(&"entities".to_string()));
        assert!(report.preserved_state.contains(&"fixedDeltaMs".to_string()));
        assert!(report.unsupported.contains(&"live code HMR".to_string()));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn scene_reload_validation_rejects_invalid_scene_without_report() {
        let root = unique_temp_dir("scene-reload-invalid");
        fs::create_dir_all(&root).expect("reload invalid fixture dir created");
        let scene_path = root.join("scene.json");
        fs::write(
            &scene_path,
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "1",
                "id": "reload-invalid",
                "bounds": { "width": 320, "height": 180 },
                "entities": [{
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 0, "y": 0 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": -1, "height": 16 },
                        "controllable": true
                    }
                }]
            }))
            .expect("scene serializes"),
        )
        .expect("scene written");

        let rejected = validate_scene_reload(&scene_path).expect_err("invalid reload rejected");
        assert!(rejected.to_string().contains("size must be positive"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn scene_physics_v2_accepts_kinematic_triggers_and_collision_masks() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "physics-v2-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 0, "y": 0 },
                        "velocity": { "x": 1, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "collider": {
                            "shape": "aabb",
                            "body": "kinematic",
                            "offset": { "x": 0, "y": 0 },
                            "size": { "width": 16, "height": 16 },
                            "trigger": true,
                            "collisionGroup": "actors",
                            "collisionMask": ["world", "triggers"]
                        }
                    }
                },
                {
                    "id": "goal",
                    "sprite": { "color": "#facc15" },
                    "components": {
                        "transform": { "x": 32, "y": 0 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": false,
                        "collider": {
                            "shape": "aabb",
                            "body": "static",
                            "size": { "width": 16, "height": 16 },
                            "sensor": true,
                            "collisionGroup": "triggers"
                        }
                    }
                }
            ]
        }))
        .expect("physics scene parses");
        validate_scene(&scene).expect("physics v2 collider schema validates");
        let collider = scene.entities[0]
            .components
            .collider
            .as_ref()
            .expect("collider present");
        assert_eq!(collider.body, "kinematic");
        assert!(collider.trigger);
        assert_eq!(collider.collision_group.as_deref(), Some("actors"));
        assert_eq!(
            collider.collision_mask,
            vec!["world".to_string(), "triggers".to_string()]
        );

        let mut invalid_body = scene.clone();
        invalid_body.entities[0]
            .components
            .collider
            .as_mut()
            .expect("collider")
            .body = "rigid".to_string();
        let rejected = validate_scene(&invalid_body).expect_err("invalid body rejected");
        assert!(rejected
            .to_string()
            .contains("collider body must be static, dynamic, or kinematic"));

        let mut duplicate_mask = scene.clone();
        duplicate_mask.entities[0]
            .components
            .collider
            .as_mut()
            .expect("collider")
            .collision_mask = vec!["world".to_string(), "world".to_string()];
        let rejected = validate_scene(&duplicate_mask).expect_err("duplicate mask rejected");
        assert!(rejected
            .to_string()
            .contains("duplicate scene entity player collider collisionMask"));

        let mut unsafe_group = scene.clone();
        unsafe_group.entities[0]
            .components
            .collider
            .as_mut()
            .expect("collider")
            .collision_group = Some("../world".to_string());
        let rejected = validate_scene(&unsafe_group).expect_err("unsafe group rejected");
        assert!(rejected.to_string().contains("may only contain ASCII"));
    }

    #[test]
    fn scene_audio_v1_validates_intent_events_and_manifest_refs() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "audio-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "assetManifest": {
                "schemaVersion": "1",
                "id": "runtime-v1-assets",
                "assets": [
                    { "id": "player-sprite", "kind": "sprite", "path": "assets/sprites/player.svg" },
                    { "id": "spawn-audio", "kind": "audio", "path": "assets/audio/player-spawn.ogg" }
                ]
            },
            "entities": [{
                "id": "player",
                "sprite": { "color": "#5eead4", "asset": "player-sprite" },
                "components": {
                    "transform": { "x": 0, "y": 0 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": true,
                    "audio": { "events": [{ "name": "player_spawn", "trigger": "scene_loaded", "action": "play", "asset": "spawn-audio" }] }
                }
            }]
        }))
        .expect("audio scene parses");
        validate_scene(&scene).expect("audio intent validates");

        let mut missing_asset = scene.clone();
        missing_asset.entities[0]
            .components
            .audio
            .as_mut()
            .expect("audio")
            .events[0]
            .asset = None;
        let rejected = validate_scene(&missing_asset).expect_err("play asset required");
        assert!(rejected
            .to_string()
            .contains("play action requires an asset ref"));

        let mut invalid_action = scene.clone();
        invalid_action.entities[0]
            .components
            .audio
            .as_mut()
            .expect("audio")
            .events[0]
            .action = "stream".to_string();
        let rejected = validate_scene(&invalid_action).expect_err("invalid audio action rejected");
        assert!(rejected.to_string().contains("action must be play or stop"));

        let mut unknown_ref = scene.clone();
        unknown_ref.entities[0]
            .components
            .audio
            .as_mut()
            .expect("audio")
            .events[0]
            .asset = Some("missing-audio".to_string());
        let rejected = validate_scene(&unknown_ref).expect_err("unknown audio ref rejected");
        assert!(rejected
            .to_string()
            .contains("references unknown asset manifest id"));
    }

    #[test]
    fn scene_animation_v1_accepts_named_clips_and_manifest_frame_refs() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "animation-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "assetManifest": {
                "schemaVersion": "1",
                "id": "runtime-v1-assets",
                "assets": [
                    { "id": "player-idle-1", "kind": "sprite", "path": "assets/sprites/player.svg" },
                    { "id": "player-idle-2", "kind": "sprite", "path": "assets/sprites/goal.svg" }
                ]
            },
            "entities": [{
                "id": "player",
                "sprite": { "color": "#5eead4", "asset": "player-idle-1" },
                "components": {
                    "transform": { "x": 0, "y": 0 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": true,
                    "animation": {
                        "mode": "sprite_frame",
                        "frameDuration": 2,
                        "currentClip": "idle",
                        "clips": [{
                            "id": "idle",
                            "frameDuration": 2,
                            "loop": true,
                            "frames": [
                                { "color": "#5eead4", "asset": "player-idle-1" },
                                { "color": "#2dd4bf", "asset": "player-idle-2" }
                            ]
                        }],
                        "state": { "currentClip": "idle", "elapsedFrames": 0, "frameIndex": 0 }
                    }
                }
            }]
        }))
        .expect("animation scene parses");
        validate_scene(&scene).expect("named animation clips validate");
        let animation = scene.entities[0]
            .components
            .animation
            .as_ref()
            .expect("animation present");
        assert_eq!(animation.current_clip.as_deref(), Some("idle"));
        assert_eq!(
            animation.clips[0].frames[1].asset.as_deref(),
            Some("player-idle-2")
        );

        let mut unknown_clip = scene.clone();
        unknown_clip.entities[0]
            .components
            .animation
            .as_mut()
            .expect("animation")
            .current_clip = Some("run".to_string());
        let rejected = validate_scene(&unknown_clip).expect_err("unknown clip rejected");
        assert!(rejected
            .to_string()
            .contains("currentClip references unknown clip"));

        let mut unknown_frame_ref = scene.clone();
        unknown_frame_ref.entities[0]
            .components
            .animation
            .as_mut()
            .expect("animation")
            .clips[0]
            .frames[0]
            .asset = Some("missing-frame".to_string());
        let rejected =
            validate_scene(&unknown_frame_ref).expect_err("unknown frame asset rejected");
        assert!(rejected
            .to_string()
            .contains("references unknown asset manifest id"));

        let mut bad_duration = scene.clone();
        bad_duration.entities[0]
            .components
            .animation
            .as_mut()
            .expect("animation")
            .clips[0]
            .frame_duration = 0;
        let rejected = validate_scene(&bad_duration).expect_err("zero clip duration rejected");
        assert!(rejected
            .to_string()
            .contains("frameDuration must be greater than 0"));
    }

    #[test]
    fn scene_asset_manifest_refs_gate_scene_asset_fields() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "manifest-backed-scene",
            "bounds": { "width": 320, "height": 180 },
            "assetManifest": {
                "schemaVersion": "1",
                "id": "runtime-v1-assets",
                "assets": [
                    { "id": "player-sprite", "kind": "sprite", "path": "assets/sprites/player.svg" },
                    { "id": "goal-tile", "kind": "image", "path": "assets/sprites/goal.svg" },
                    { "id": "spawn-sound", "kind": "audio", "path": "assets/audio/spawn.ogg" }
                ]
            },
            "tilemaps": [{
                "id": "asset-map",
                "tileSize": { "width": 16, "height": 16 },
                "grid": { "width": 1, "height": 1 },
                "tiles": [{ "id": "goal", "color": "#facc15", "asset": "goal-tile" }],
                "layers": [{ "id": "ground", "data": ["goal"] }]
            }],
            "entities": [{
                "id": "player",
                "sprite": { "color": "#5eead4", "asset": "player-sprite" },
                "components": {
                    "transform": { "x": 0, "y": 0 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": true,
                    "audio": { "events": [{ "name": "spawn", "trigger": "scene_loaded", "asset": "spawn-sound" }] }
                }
            }]
        }))
        .expect("manifest scene parses");
        validate_scene(&scene).expect("manifest IDs are accepted");

        let mut unknown_sprite = scene.clone();
        unknown_sprite.entities[0].sprite.asset = Some("assets/sprites/player.svg".to_string());
        let rejected =
            validate_scene(&unknown_sprite).expect_err("direct path rejected in manifest mode");
        assert!(
            rejected
                .to_string()
                .contains("references unknown asset manifest id")
                || rejected.to_string().contains("may only contain ASCII"),
            "unexpected rejection: {rejected}"
        );

        let mut unknown_tile = scene.clone();
        unknown_tile.tilemaps[0].tiles[0].asset = Some("missing-tile".to_string());
        let rejected = validate_scene(&unknown_tile).expect_err("unknown tile asset rejected");
        assert!(rejected
            .to_string()
            .contains("references unknown asset manifest id"));
    }

    #[test]
    fn scene_renderer_v1_accepts_camera_layers_and_deterministic_order() {
        let scene: SceneDocument = serde_json::from_value(json!({
            "schemaVersion": "1",
            "id": "renderer-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "renderer": {
                "version": "1",
                "camera": { "x": 8, "y": 12 },
                "viewport": { "width": 160, "height": 90 },
                "background": "#172532",
                "layers": [
                    { "id": "background", "order": -10 },
                    { "id": "actors", "order": 0 },
                    { "id": "hud", "order": 10, "visible": true }
                ],
                "debug": {
                    "showBounds": true,
                    "showCamera": true,
                    "showEntityIds": false
                }
            },
            "entities": [
                {
                    "id": "zebra",
                    "sprite": {
                        "color": "#facc15",
                        "layer": "actors",
                        "order": 5
                    },
                    "components": {
                        "transform": { "x": 72, "y": 32 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": false
                    }
                },
                {
                    "id": "player",
                    "sprite": {
                        "color": "#5eead4",
                        "asset": "assets/sprites/player.svg",
                        "layer": "actors",
                        "order": 5
                    },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                },
                {
                    "id": "sky",
                    "sprite": {
                        "color": "#0f172a",
                        "layer": "background",
                        "order": 0
                    },
                    "components": {
                        "transform": { "x": 0, "y": 0 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 320, "height": 180 },
                        "controllable": false
                    }
                },
                {
                    "id": "hidden-debug",
                    "sprite": {
                        "color": "#ffffff",
                        "layer": "hud",
                        "order": 0,
                        "visible": false
                    },
                    "components": {
                        "transform": { "x": 0, "y": 0 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 8, "height": 8 },
                        "controllable": false
                    }
                }
            ]
        }))
        .expect("renderer scene parses");

        validate_scene(&scene).expect("renderer v1 validates");
        let renderer = scene.renderer.as_ref().expect("renderer present");
        assert_eq!(renderer.version, "1");
        assert_eq!(renderer.camera, ScenePoint { x: 8, y: 12 });
        assert_eq!(renderer.viewport.width, 160);
        assert!(renderer.debug.show_bounds);

        let order = scene_render_order(&scene);
        assert_eq!(
            order
                .iter()
                .map(|entry| entry.entity_id.as_str())
                .collect::<Vec<_>>(),
            vec!["sky", "player", "zebra"]
        );
        assert_eq!(
            serde_json::to_string(&order).expect("render order serializes"),
            r#"[{"entityId":"sky","layer":"background","layerOrder":-10,"spriteOrder":0},{"entityId":"player","layer":"actors","layerOrder":0,"spriteOrder":5},{"entityId":"zebra","layer":"actors","layerOrder":0,"spriteOrder":5}]"#
        );
    }

    #[test]
    fn scene_schema_v1_rejects_invalid_entities_and_paths() {
        let invalid_tilemap_grid = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 0, "height": 2 },
                    "tiles": [{ "id": "solid", "color": "#334155" }],
                    "layers": [{ "id": "ground", "data": [] }]
                }
            ],
            "entities": []
        }))
        .expect("invalid tilemap grid parses");
        let rejected = validate_scene(&invalid_tilemap_grid).expect_err("grid rejected");
        assert!(rejected
            .to_string()
            .contains("grid dimensions must be positive"));

        let invalid_tilemap_data_len = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 2, "height": 2 },
                    "tiles": [{ "id": "solid", "color": "#334155" }],
                    "layers": [{ "id": "ground", "data": ["solid"] }]
                }
            ],
            "entities": []
        }))
        .expect("invalid tilemap data length parses");
        let rejected = validate_scene(&invalid_tilemap_data_len).expect_err("data length rejected");
        assert!(rejected
            .to_string()
            .contains("data length must equal grid cell count 4"));

        let invalid_tile_id = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 1, "height": 1 },
                    "tiles": [{ "id": "solid", "color": "#334155" }],
                    "layers": [{ "id": "ground", "data": ["missing"] }]
                }
            ],
            "entities": []
        }))
        .expect("invalid tile id parses");
        let rejected = validate_scene(&invalid_tile_id).expect_err("unknown tile rejected");
        assert!(rejected.to_string().contains("references unknown tile id"));

        let unsafe_tile_asset = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 1, "height": 1 },
                    "tiles": [{ "id": "solid", "color": "#334155", "asset": "https://example.com/tile.svg" }],
                    "layers": [{ "id": "ground", "data": ["solid"] }]
                }
            ],
            "entities": []
        }))
        .expect("unsafe tile asset parses");
        let rejected = validate_scene(&unsafe_tile_asset).expect_err("remote tile asset rejected");
        assert!(rejected
            .to_string()
            .contains("must be a local static asset path"));

        let unknown_collision_layer = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "tilemap-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "tilemaps": [
                {
                    "id": "level",
                    "tileSize": { "width": 16, "height": 16 },
                    "grid": { "width": 1, "height": 1 },
                    "tiles": [{ "id": "solid", "color": "#334155" }],
                    "layers": [{ "id": "ground", "data": ["solid"], "collisionLayer": "missing" }]
                }
            ],
            "entities": []
        }))
        .expect("unknown collision layer parses");
        let rejected =
            validate_scene(&unknown_collision_layer).expect_err("collision layer rejected");
        assert!(rejected
            .to_string()
            .contains("references unknown collisionLayer"));

        let duplicate_layer = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "renderer-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "renderer": {
                "version": "1",
                "camera": { "x": 0, "y": 0 },
                "viewport": { "width": 160, "height": 90 },
                "background": "#172532",
                "layers": [
                    { "id": "actors", "order": 0 },
                    { "id": "actors", "order": 1 }
                ],
                "debug": {}
            },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4", "layer": "actors" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("duplicate layer fixture parses");
        let rejected = validate_scene(&duplicate_layer).expect_err("duplicate layers rejected");
        assert!(rejected
            .to_string()
            .contains("duplicate scene renderer layer id"));

        let unknown_layer = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "renderer-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "renderer": {
                "version": "1",
                "camera": { "x": 0, "y": 0 },
                "viewport": { "width": 160, "height": 90 },
                "background": "#172532",
                "layers": [
                    { "id": "actors", "order": 0 }
                ],
                "debug": {}
            },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4", "layer": "missing" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("unknown layer fixture parses");
        let rejected = validate_scene(&unknown_layer).expect_err("unknown layer rejected");
        assert!(rejected
            .to_string()
            .contains("sprite layer references unknown renderer layer"));

        let oversized_viewport = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "renderer-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "renderer": {
                "version": "1",
                "camera": { "x": 0, "y": 0 },
                "viewport": { "width": 640, "height": 90 },
                "background": "#172532",
                "layers": [
                    { "id": "actors", "order": 0 }
                ],
                "debug": {}
            },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4", "layer": "actors" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("oversized viewport fixture parses");
        let rejected =
            validate_scene(&oversized_viewport).expect_err("oversized viewport rejected");
        assert!(rejected
            .to_string()
            .contains("viewport must fit inside scene bounds"));

        let duplicate = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                },
                {
                    "id": "player",
                    "sprite": { "color": "#facc15" },
                    "components": {
                        "transform": { "x": 272, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": false
                    }
                }
            ]
        }))
        .expect("duplicate fixture parses");
        let rejected = validate_scene(&duplicate).expect_err("duplicate ids rejected");
        assert!(rejected.to_string().contains("duplicate scene entity id"));

        let path_escape = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": {
                        "color": "#5eead4",
                        "asset": "assets/../outside.png"
                    },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("path fixture parses");
        let rejected = validate_scene(&path_escape).expect_err("asset path escape rejected");
        assert!(rejected.to_string().contains("local scene asset tree"));

        let absolute_path = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": {
                        "color": "#5eead4",
                        "asset": "/tmp/player.png"
                    },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("absolute path fixture parses");
        let rejected = validate_scene(&absolute_path).expect_err("absolute asset path rejected");
        assert!(rejected.to_string().contains("must start with assets/"));

        let remote_url = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": {
                        "color": "#5eead4",
                        "asset": "https://example.com/player.png"
                    },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true
                    }
                }
            ]
        }))
        .expect("remote URL fixture parses");
        let rejected = validate_scene(&remote_url).expect_err("remote asset URL rejected");
        assert!(rejected
            .to_string()
            .contains("must be a local static asset path"));

        let future_collider = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "collider": {
                            "shape": "circle",
                            "size": { "width": 16, "height": 16 }
                        }
                    }
                }
            ]
        }))
        .expect("collider fixture parses");
        let rejected = validate_scene(&future_collider).expect_err("future collider rejected");
        assert!(rejected.to_string().contains("collider shape must be aabb"));

        let kinematic_body = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "collider": {
                            "shape": "aabb",
                            "body": "kinematic",
                            "size": { "width": 16, "height": 16 }
                        }
                    }
                }
            ]
        }))
        .expect("collider body fixture parses");
        validate_scene(&kinematic_body).expect("kinematic body accepted for physics v2");

        let future_animation_mode = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "animation": {
                            "mode": "timeline",
                            "frameDuration": 2,
                            "frames": [{ "color": "#5eead4" }]
                        }
                    }
                }
            ]
        }))
        .expect("animation mode fixture parses");
        let rejected =
            validate_scene(&future_animation_mode).expect_err("future animation mode rejected");
        assert!(rejected
            .to_string()
            .contains("animation mode must be sprite_frame"));

        let empty_animation_frames = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "animation": {
                            "mode": "sprite_frame",
                            "frameDuration": 0,
                            "frames": []
                        }
                    }
                }
            ]
        }))
        .expect("empty animation fixture parses");
        let rejected =
            validate_scene(&empty_animation_frames).expect_err("invalid animation rejected");
        assert!(rejected
            .to_string()
            .contains("animation frameDuration must be greater than 0"));

        let future_audio_trigger = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "audio": {
                            "events": [
                                { "name": "jump", "trigger": "collision_enter" }
                            ]
                        }
                    }
                }
            ]
        }))
        .expect("audio trigger fixture parses");
        let rejected =
            validate_scene(&future_audio_trigger).expect_err("future audio trigger rejected");
        assert!(rejected
            .to_string()
            .contains("audio trigger must be scene_loaded"));

        let remote_audio_asset = serde_json::from_value::<SceneDocument>(json!({
            "schemaVersion": "1",
            "id": "runtime-v1-scene",
            "bounds": { "width": 320, "height": 180 },
            "entities": [
                {
                    "id": "player",
                    "sprite": { "color": "#5eead4" },
                    "components": {
                        "transform": { "x": 32, "y": 72 },
                        "velocity": { "x": 0, "y": 0 },
                        "size": { "width": 16, "height": 16 },
                        "controllable": true,
                        "audio": {
                            "events": [
                                {
                                    "name": "jump",
                                    "trigger": "scene_loaded",
                                    "asset": "https://example.com/jump.ogg"
                                }
                            ]
                        }
                    }
                }
            ]
        }))
        .expect("remote audio asset fixture parses");
        let rejected = validate_scene(&remote_audio_asset).expect_err("remote audio rejected");
        assert!(rejected
            .to_string()
            .contains("must be a local static asset path"));
    }

    #[test]
    fn dashboard_read_model_lists_runs_and_categorizes_artifacts() {
        let (root, artifacts) = create_test_run("dashboard-read-model");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"failed\",\"summary\":\"fixture failure\"}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/workers/worker-1"))
            .expect("worker evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/browser-smoke.png"),
            b"png fixture",
        )
        .expect("screenshot written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/world-state.json"),
            "{\"object\":{\"x\":40}}\n",
        )
        .expect("world state written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/console-log.json"),
            "[{\"level\":\"info\",\"text\":\"ready\"}]\n",
        )
        .expect("console log written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/frame-stats.json"),
            "{\"frame\":3,\"fps\":60}\n",
        )
        .expect("frame stats written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/performance-metrics.json"),
            "{\"metrics\":[{\"name\":\"ScriptDuration\",\"value\":1}]}\n",
        )
        .expect("performance metrics written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/cdp-trace-summary.json"),
            "{\"bounded\":true,\"events\":[]}\n",
        )
        .expect("cdp trace summary written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/scenarios/bootstrap-smoke"))
            .expect("scenario evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/bootstrap-smoke/scenario-result.json"),
            "{\"scenario_id\":\"bootstrap-smoke\",\"status\":\"passed\"}\n",
        )
        .expect("scenario result written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-screenshot",
            "image/png",
            "evidence/workers/worker-1/browser-smoke.png",
            json!({ "worker_id": "worker-1" }),
        )
        .expect("screenshot indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-world-state",
            "application/json",
            "evidence/workers/worker-1/world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-1" }),
        )
        .expect("world state indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-console-log",
            "application/json",
            "evidence/workers/worker-1/console-log.json",
            json!({ "artifact": "console_log", "worker_id": "worker-1" }),
        )
        .expect("console log indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-frame-stats",
            "application/json",
            "evidence/workers/worker-1/frame-stats.json",
            json!({ "probe_call": "getFrameStats", "worker_id": "worker-1" }),
        )
        .expect("frame stats indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-performance-metrics",
            "application/json",
            "evidence/workers/worker-1/performance-metrics.json",
            json!({ "artifact": "performance_metrics", "worker_id": "worker-1" }),
        )
        .expect("performance metrics indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-cdp-trace-summary",
            "application/json",
            "evidence/workers/worker-1/cdp-trace-summary.json",
            json!({ "artifact": "cdp_trace_summary", "worker_id": "worker-1" }),
        )
        .expect("cdp trace summary indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-scenario-result",
            "application/json",
            "evidence/scenarios/bootstrap-smoke/scenario-result.json",
            json!({ "artifact": "scenario_result" }),
        )
        .expect("scenario result indexed");
        create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "fixture failure".to_string(),
                evidence_id: "fixture-world-state".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.bootstrap-smoke.assertions".to_string(),
                from: "x == -1".to_string(),
                to: "x == 40".to_string(),
            },
        )
        .expect("mutation proposal created");

        let runs = list_dashboard_runs(root.join("runs")).expect("dashboard runs listed");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].verdict_status, "failed");
        assert_eq!(runs[0].scenario_status, "pending");
        assert_eq!(runs[0].evidence_count, 7);
        assert_eq!(runs[0].mutation_count, 1);
        assert_eq!(runs[0].worker_count, 1);

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run read");
        assert_eq!(model.summary.id, runs[0].id);
        assert_eq!(model.evidence.len(), 7);
        assert_eq!(model.screenshots.len(), 1);
        assert_eq!(model.world_states.len(), 1);
        assert_eq!(model.console_logs.len(), 1);
        assert_eq!(model.frame_metrics.len(), 1);
        assert_eq!(model.performance_metrics.len(), 1);
        assert_eq!(model.cdp_trace_summaries.len(), 1);
        assert_eq!(model.scenario_results.len(), 1);
        assert_eq!(model.mutation_artifacts.len(), 1);
        assert_eq!(model.mutations.len(), 1);
        assert!(model.transaction_provenance.is_none());
        let world_state = model.world_states[0]
            .value
            .as_ref()
            .expect("world state JSON loaded");
        let console_log = model.console_logs[0]
            .value
            .as_ref()
            .expect("console log JSON loaded");
        assert_eq!(world_state["object"]["x"], json!(40));
        assert_eq!(console_log[0]["text"], json!("ready"));
        assert!(model
            .evidence_categories
            .iter()
            .any(|category| category.id == "screenshots" && category.count == 1));
        assert!(model
            .evidence_categories
            .iter()
            .any(|category| { category.id == "frame_performance_metrics" && category.count == 2 }));
        assert!(model
            .evidence_categories
            .iter()
            .any(|category| { category.id == "console_cdp_summaries" && category.count == 2 }));
        assert!(model
            .evidence_categories
            .iter()
            .any(|category| category.id == "scenario_results" && category.count == 1));
        assert!(model
            .evidence_categories
            .iter()
            .any(|category| category.id == "mutation_artifacts" && category.count == 1));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_read_model_exposes_transaction_provenance_when_present() {
        let (root, artifacts) = create_test_run("dashboard-transaction-provenance");
        let mut run_json: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(artifacts.run_dir.join("run.json")).expect("run json reads"),
        )
        .expect("run json parses");
        run_json["transaction_provenance"] = json!({
            "transactionId": "scene-edit-fixture",
            "transactionArtifactPath": "transactions/scene-edit-fixture.json",
            "scenePath": "examples/game-runtime/scene.json",
            "beforeSceneHash": {
                "algorithm": "fnv1a64-canonical-json-v1",
                "value": "before-fixture"
            },
            "afterSceneHash": {
                "algorithm": "fnv1a64-canonical-json-v1",
                "value": "after-fixture"
            }
        });
        fs::write(
            artifacts.run_dir.join("run.json"),
            serde_json::to_string_pretty(&run_json).expect("run json serializes"),
        )
        .expect("run json updated");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        let provenance = model
            .transaction_provenance
            .expect("transaction provenance exposed");
        assert_eq!(provenance.transaction_id, "scene-edit-fixture");
        assert_eq!(
            provenance.transaction_artifact_path,
            "transactions/scene-edit-fixture.json"
        );
        assert_eq!(provenance.before_scene_hash.value, "before-fixture");
        assert_eq!(provenance.after_scene_hash.value, "after-fixture");

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_engine_summaries_extract_engine_expansion_state() {
        let (root, artifacts) = create_test_run("dashboard-engine-summaries");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"passed\"}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/workers/worker-1"))
            .expect("worker evidence dir");
        fs::write(
            artifacts.run_dir.join("evidence/workers/worker-1/world-state.json"),
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "1",
                "sceneId": "foundation-scene",
                "tick": 4,
                "entities": [{
                    "id": "player",
                    "components": {
                        "transform": { "x": 40, "y": 72 },
                        "collider": { "body": "dynamic" },
                        "animation": { "currentClip": "idle" },
                        "audio": { "events": [] }
                    }
                }],
                "renderer": { "version": "1", "camera": { "x": 0, "y": 0 }, "renderedEntities": [{ "entityId": "player" }] },
                "tilemaps": { "tilemaps": [{ "id": "platformer-ground" }], "layerOrder": [{ "layerId": "background" }] },
                "assetManifest": { "id": "runtime-v1-assets", "assetCount": 3 },
                "assets": [{ "id": "player-sprite", "status": "loaded" }],
                "audioEvents": [{ "name": "player_spawn" }],
                "collisions": [{ "pairId": "goal:player" }],
                "collisionEvents": [{ "type": "runtime.collision.trigger" }],
                "reloads": [{ "status": "succeeded" }],
                "composition": { "entities": [{ "entityId": "player", "parent": null }] }
            }))
            .expect("world-state serializes"),
        )
        .expect("world-state written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-world-state",
            "application/json",
            "evidence/workers/worker-1/world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-1" }),
        )
        .expect("world-state indexed");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(model.engine_summaries.present);
        assert_eq!(
            model.engine_summaries.source_world_state.as_deref(),
            Some("evidence/workers/worker-1/world-state.json")
        );
        assert_eq!(
            model.engine_summaries.scene["sceneId"],
            json!("foundation-scene")
        );
        assert_eq!(
            model.engine_summaries.renderer["renderedEntities"],
            json!(1)
        );
        assert_eq!(model.engine_summaries.tilemaps["tilemapCount"], json!(1));
        assert_eq!(
            model.engine_summaries.assets["manifestId"],
            json!("runtime-v1-assets")
        );
        assert_eq!(
            model.engine_summaries.animation["animatedEntityCount"],
            json!(1)
        );
        assert_eq!(model.engine_summaries.audio["audioEventCount"], json!(1));
        assert_eq!(
            model.engine_summaries.physics["collisionEventCount"],
            json!(1)
        );
        assert_eq!(
            model.engine_summaries.reload["lastStatus"],
            json!("succeeded")
        );
        assert_eq!(model.engine_summaries.composition["entityCount"], json!(1));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_engine_summaries_handle_missing_world_state() {
        let (root, artifacts) = create_test_run("dashboard-engine-summaries-empty");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"pending\"}\n",
        )
        .expect("verdict written");
        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(!model.engine_summaries.present);
        assert!(model
            .engine_summaries
            .empty_state
            .contains("No readable world-state artifacts"));
        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_replay_read_model_tracks_present_replay_evidence() {
        let (root, artifacts) = create_test_run("dashboard-replay-read-model");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"passed\",\"metadata\":{\"scenario_results\":1}}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/scenarios/replay-smoke"))
            .expect("scenario evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/replay-smoke/input-replay.json"),
            serde_json::to_string_pretty(&json!({
                "reference": {
                    "id": "move-right",
                    "path": "replays/move-right.yaml"
                },
                "replay": {
                    "schemaVersion": "input-replay-v1",
                    "id": "move-right",
                    "events": [
                        { "frame": 0, "key": "right", "pressed": true },
                        { "frame": 4, "key": "right", "pressed": false }
                    ]
                }
            }))
            .expect("replay JSON serializes"),
        )
        .expect("input replay evidence written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/replay-smoke/world-state.json"),
            "{\"tick\":4,\"entities\":[{\"id\":\"player\",\"components\":{\"transform\":{\"x\":40,\"y\":72}}}]}\n",
        )
        .expect("world state evidence written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/replay-smoke/frame-stats.json"),
            "{\"tick\":4,\"fixedDeltaMs\":16}\n",
        )
        .expect("frame stats evidence written");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/scenarios/replay-smoke/scenario-result.json"),
            serde_json::to_string_pretty(&json!({
                "scenario_id": "replay-smoke",
                "status": "passed",
                "evidence": {
                    "input_replays": ["evidence/scenarios/replay-smoke/input-replay.json"],
                    "world_state": "evidence/scenarios/replay-smoke/world-state.json",
                    "frame_stats": "evidence/scenarios/replay-smoke/frame-stats.json"
                }
            }))
            .expect("scenario result JSON serializes"),
        )
        .expect("scenario result written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-input-replay",
            "application/json",
            "evidence/scenarios/replay-smoke/input-replay.json",
            json!({ "artifact": "input_replay", "scenario_id": "replay-smoke", "source": "replayRef" }),
        )
        .expect("input replay indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-scenario-result",
            "application/json",
            "evidence/scenarios/replay-smoke/scenario-result.json",
            json!({ "artifact": "scenario_result", "scenario_id": "replay-smoke" }),
        )
        .expect("scenario result indexed");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(model.replay.present);
        assert_eq!(model.replay.empty_state, "");
        assert_eq!(model.replay.sequences.len(), 1);
        let sequence = &model.replay.sequences[0];
        assert_eq!(sequence.id, "move-right");
        assert_eq!(sequence.source, "replayRef");
        assert_eq!(sequence.scenario_id.as_deref(), Some("replay-smoke"));
        assert_eq!(sequence.event_count, 2);
        assert_eq!(sequence.frames, vec![0, 4]);
        assert_eq!(sequence.first_frame, Some(0));
        assert_eq!(sequence.last_frame, Some(4));
        assert_eq!(
            sequence.evidence_refs,
            vec![
                "evidence/scenarios/replay-smoke/input-replay.json".to_string(),
                "evidence/scenarios/replay-smoke/scenario-result.json".to_string()
            ]
        );
        assert_eq!(sequence.checkpoints.len(), 1);
        let checkpoint = &sequence.checkpoints[0];
        assert_eq!(checkpoint.frame, Some(4));
        assert_eq!(checkpoint.tick, Some(4));
        assert_eq!(
            checkpoint.world_state_path.as_deref(),
            Some("evidence/scenarios/replay-smoke/world-state.json")
        );
        assert_eq!(
            checkpoint
                .world_state
                .as_ref()
                .expect("world state is loaded")["entities"][0]["components"]["transform"]["x"],
            json!(40)
        );

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_replay_read_model_handles_missing_replay_evidence() {
        let (root, artifacts) = create_test_run("dashboard-replay-empty-read-model");
        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(!model.replay.present);
        assert!(model.replay.sequences.is_empty());
        assert!(model
            .replay
            .empty_state
            .contains("No replay evidence artifacts"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_comparison_read_model_tracks_present_and_malformed_artifacts() {
        let (root, artifacts) = create_test_run("dashboard-comparison-read-model");
        fs::create_dir_all(artifacts.run_dir.join("mutation")).expect("mutation dir exists");
        fs::write(
            artifacts
                .run_dir
                .join("mutation/run-comparison-before--after.json"),
            serde_json::to_string_pretty(&json!({
                "before_run_id": "before",
                "after_run_id": "after",
                "classification": "improved",
                "before": { "run_id": "before", "verdict_status": "failed" },
                "after": { "run_id": "after", "verdict_status": "passed" },
                "deltas": {
                    "scenario_results": 0,
                    "failed_scenarios": -1,
                    "assertion_failures": -2,
                    "performance_artifacts": 1,
                    "evidence_artifacts": 3
                },
                "semantic": {
                    "schemaVersion": "run-semantic-diff-v1",
                    "reasons": [{
                        "kind": "scenario_verdict",
                        "severity": "improved",
                        "summary": "scenario smoke changed from failed to passed",
                        "evidenceRefs": []
                    }],
                    "warnings": []
                },
                "evidence_refs": [
                    "runs/before/verdict.json",
                    "runs/after/verdict.json"
                ],
                "unsupported": [
                    "semantic gameplay quality is not inferred"
                ]
            }))
            .expect("comparison JSON serializes"),
        )
        .expect("comparison written");
        fs::write(
            artifacts
                .run_dir
                .join("mutation/run-comparison-bad--after.json"),
            "{bad-json",
        )
        .expect("malformed comparison written");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(model.comparison.present);
        assert_eq!(model.comparison.artifacts.len(), 2);
        let comparison = model
            .comparison
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "mutation/run-comparison-before--after.json")
            .expect("valid comparison present");
        assert_eq!(comparison.before_run_id.as_deref(), Some("before"));
        assert_eq!(comparison.after_run_id.as_deref(), Some("after"));
        assert_eq!(comparison.classification.as_deref(), Some("improved"));
        assert_eq!(comparison.deltas["failed_scenarios"], json!(-1));
        assert_eq!(
            comparison.semantic["schemaVersion"],
            json!("run-semantic-diff-v1")
        );
        assert_eq!(
            comparison.semantic["reasons"][0]["summary"],
            json!("scenario smoke changed from failed to passed")
        );
        assert_eq!(
            comparison.evidence_refs,
            vec![
                "runs/before/verdict.json".to_string(),
                "runs/after/verdict.json".to_string()
            ]
        );
        assert_eq!(
            comparison.unsupported,
            vec!["semantic gameplay quality is not inferred".to_string()]
        );
        let malformed = model
            .comparison
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "mutation/run-comparison-bad--after.json")
            .expect("malformed comparison present");
        assert!(malformed.read_error.is_some());
        assert!(malformed.value.is_none());

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_comparison_read_model_handles_missing_artifacts() {
        let (root, artifacts) = create_test_run("dashboard-comparison-empty-read-model");
        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(!model.comparison.present);
        assert!(model.comparison.artifacts.is_empty());
        assert!(model
            .comparison
            .empty_state
            .contains("No run comparison artifacts"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_read_model_surfaces_missing_and_malformed_evidence_without_mutating() {
        let (root, artifacts) = create_test_run("dashboard-partial-read-model");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"pending\",\"metadata\":{\"scenario_results\":0}}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/workers/worker-2"))
            .expect("worker evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-2/malformed-world-state.json"),
            "{not-json",
        )
        .expect("malformed world state written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "missing-world-state",
            "application/json",
            "evidence/workers/worker-2/missing-world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-2" }),
        )
        .expect("missing artifact indexed");
        add_evidence_artifact(
            &artifacts.run_dir,
            "malformed-world-state",
            "application/json",
            "evidence/workers/worker-2/malformed-world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-2" }),
        )
        .expect("malformed artifact indexed");

        let model = read_dashboard_run(&artifacts.run_dir)
            .expect("partial dashboard run remains inspectable");
        assert_eq!(model.world_states.len(), 2);
        assert_eq!(
            model
                .world_states
                .iter()
                .filter(|artifact| !artifact.exists)
                .count(),
            1
        );
        assert_eq!(
            model
                .world_states
                .iter()
                .filter(|artifact| artifact.exists && artifact.read_error.is_some())
                .count(),
            1
        );
        let world_state_category = model
            .evidence_categories
            .iter()
            .find(|category| category.id == "world_states")
            .expect("world-state category summarized");
        assert_eq!(world_state_category.count, 2);
        assert_eq!(world_state_category.missing_count, 1);
        assert_eq!(world_state_category.malformed_count, 1);
        assert_eq!(model.summary.worker_count, 1);

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_journal_read_model_links_evidence_verdict_and_mutations() {
        let (root, artifacts) = create_test_run("dashboard-journal-read-model");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"failed\",\"summary\":\"fixture journal failure\",\"failures\":[{\"kind\":\"assertion_failed\"}]}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/workers/worker-1"))
            .expect("worker evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/world-state.json"),
            "{\"object\":{\"x\":40}}\n",
        )
        .expect("world state written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-world-state",
            "application/json",
            "evidence/workers/worker-1/world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-1" }),
        )
        .expect("world state indexed");
        let proposal = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "fixture failure".to_string(),
                evidence_id: "fixture-world-state".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.bootstrap-smoke.assertions".to_string(),
                from: "x == -1".to_string(),
                to: "x == 40".to_string(),
            },
        )
        .expect("mutation proposal created");
        fs::write(
            artifacts.run_dir.join("journal.md"),
            format!(
                "# Ouroforge Run Journal\n\n## Hypothesis\n\n- Validate the failure links.\n\n## Observations\n\n- Evidence `fixture-world-state` at `evidence/workers/worker-1/world-state.json`.\n\n## Verdict Summary\n\n- Status: `failed`\n- Summary: fixture journal failure\n\n## Failed Criteria\n\n- `assertion_failed`: fixture\n\n## Next Mutation\n\n- `{}`: inspect proposal.\n",
                proposal.id
            ),
        )
        .expect("journal written");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert!(model.journal_view.exists);
        assert_eq!(model.journal_view.path, "journal.md");
        assert_eq!(model.journal_view.summary, "fixture journal failure");
        assert!(model
            .journal_view
            .entries
            .iter()
            .any(|entry| entry.category == "hypothesis"));
        assert!(model
            .journal_view
            .entries
            .iter()
            .any(|entry| entry.category == "observation"
                && entry
                    .evidence_refs
                    .contains(&"evidence/workers/worker-1/world-state.json".to_string())));
        assert!(model
            .journal_view
            .entries
            .iter()
            .any(|entry| entry.category == "verdict"
                && entry.verdict_refs.contains(&"verdict.json".to_string())));
        assert!(model
            .journal_view
            .entries
            .iter()
            .any(|entry| entry.category == "next_mutation"
                && entry.mutation_refs.contains(&proposal.id)));
        assert!(model
            .journal_view
            .evidence_refs
            .contains(&"evidence/workers/worker-1/world-state.json".to_string()));
        assert!(model
            .journal_view
            .verdict_refs
            .contains(&"verdict.json".to_string()));
        assert!(model.journal_view.mutation_refs.contains(&proposal.id));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_journal_read_model_handles_missing_journal() {
        let (root, artifacts) = create_test_run("dashboard-missing-journal-read-model");
        fs::remove_file(artifacts.run_dir.join("journal.md")).expect("journal removed");
        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run still reads");
        assert!(!model.journal_view.exists);
        assert_eq!(model.journal_view.summary, "No journal content available.");
        assert!(model.journal_view.read_error.is_some());
        assert!(model.journal_view.entries.is_empty());
        assert!(model.journal.is_empty());

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_mutation_lifecycle_read_model_tracks_partial_and_reviewed_states() {
        let (root, artifacts) = create_test_run("dashboard-mutation-lifecycle-read-model");
        fs::write(
            artifacts.run_dir.join("verdict.json"),
            "{\"status\":\"failed\",\"summary\":\"fixture failure\",\"failures\":[{\"kind\":\"assertion_failed\"}]}\n",
        )
        .expect("verdict written");
        fs::create_dir_all(artifacts.run_dir.join("evidence/workers/worker-1"))
            .expect("worker evidence dir");
        fs::write(
            artifacts
                .run_dir
                .join("evidence/workers/worker-1/world-state.json"),
            "{\"object\":{\"x\":40}}\n",
        )
        .expect("world state written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "fixture-world-state",
            "application/json",
            "evidence/workers/worker-1/world-state.json",
            json!({ "probe_call": "getWorldState", "worker_id": "worker-1" }),
        )
        .expect("world state indexed");
        let proposal = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "fixture failure".to_string(),
                evidence_id: "fixture-world-state".to_string(),
                target: "seeds/platformer.yaml".to_string(),
                path: "scenarios.bootstrap-smoke.assertions".to_string(),
                from: "x == -1".to_string(),
                to: "x == 40".to_string(),
            },
        )
        .expect("mutation proposal created");
        let classification = MutationClassification {
            id: "classification-1".to_string(),
            proposal_id: Some(proposal.id.clone()),
            category: MutationClassificationCategory::ScenarioAssertionFailure,
            lifecycle_state: MutationClassificationState::Classified,
            reason: "linked scenario failure".to_string(),
            evidence_refs: vec!["evidence/workers/worker-1/world-state.json".to_string()],
            verdict_ref: "verdict.json".to_string(),
            journal_ref: "journal.md".to_string(),
            scenario_result_refs: Vec::new(),
        };
        write_mutation_classification_artifact(
            &artifacts.run_dir,
            &MutationClassificationArtifact {
                schema_version: "1".to_string(),
                run_id: artifacts
                    .run_dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                classifications: vec![classification],
            },
        )
        .expect("classification written");
        let drafts = generate_patch_drafts(&artifacts.run_dir).expect("draft generated");
        fs::create_dir_all(artifacts.run_dir.join("sandbox/patch-draft-1/evidence"))
            .expect("sandbox evidence dir");
        write_json(
            &artifacts
                .run_dir
                .join("sandbox/patch-draft-1/evidence/result.json"),
            &json!({
                "schema_version": "1",
                "patch_draft_id": "patch-draft-1",
                "lifecycle_state": "applied",
                "evidence_path": "sandbox/patch-draft-1/evidence/result.json"
            }),
        )
        .expect("sandbox result written");
        fs::create_dir_all(artifacts.run_dir.join("mutation")).expect("mutation dir exists");
        write_json(
            &artifacts.run_dir.join("mutation/rerun-orchestration.json"),
            &json!({
                "patch_draft_id": "patch-draft-1",
                "comparison_artifact_path": "mutation/run-comparison-before--after.json",
                "evolve_evidence_path": "mutation/rerun-orchestration.json"
            }),
        )
        .expect("rerun orchestration written");
        write_json(
            &artifacts
                .run_dir
                .join("mutation/run-comparison-before--after.json"),
            &json!({
                "classification": "improved",
                "evidence_refs": ["evidence/workers/worker-1/world-state.json"]
            }),
        )
        .expect("comparison written");
        append_mutation_review_decision(
            &artifacts.run_dir,
            MutationReviewDecisionInput {
                patch_draft_id: drafts.drafts[0].id.clone(),
                state: MutationReviewState::Accepted,
                reason: "manual review accepted".to_string(),
                evidence_refs: vec!["mutation/rerun-orchestration.json".to_string()],
                reviewer: "test-reviewer".to_string(),
            },
        )
        .expect("review accepted");

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        let lifecycle = &model.mutation_lifecycle;
        assert_eq!(lifecycle.terminal_state, "accepted");
        assert!(lifecycle
            .command_hints
            .iter()
            .any(|hint| hint.contains("--accept")));
        assert!(lifecycle
            .command_hints
            .iter()
            .any(|hint| hint.contains("--reject")));
        for stage in [
            "proposed",
            "classified",
            "drafted",
            "sandboxed",
            "compared",
            "reviewed",
        ] {
            assert!(
                lifecycle
                    .stages
                    .iter()
                    .any(|item| item.id == stage && item.state != "missing"),
                "missing lifecycle stage {stage}"
            );
        }
        assert!(lifecycle
            .stages
            .iter()
            .any(|stage| stage.id == "reviewed" && stage.state == "accepted"));
        assert!(lifecycle.stages.iter().any(|stage| stage
            .evidence_refs
            .contains(&"mutation/rerun-orchestration.json".to_string())));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn dashboard_mutation_lifecycle_read_model_handles_empty_state() {
        let (root, artifacts) = create_test_run("dashboard-mutation-empty-read-model");
        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");
        assert_eq!(model.mutation_lifecycle.terminal_state, "missing");
        assert!(model.mutation_lifecycle.command_hints.is_empty());
        assert!(model
            .mutation_lifecycle
            .stages
            .iter()
            .all(|stage| stage.state == "missing"));

        fs::remove_dir_all(root).expect("fixture removed");
    }

    #[test]
    fn detects_complete_http_response_body_by_content_length() {
        assert!(http_response_has_complete_body(
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}"
        ));
        assert!(!http_response_has_complete_body(
            b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n{}"
        ));
        assert!(!http_response_has_complete_body(
            b"HTTP/1.1 200 OK\r\n\r\n{}"
        ));
    }

    #[test]
    fn selects_configured_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"page-1",
                "type":"page",
                "url":"http://wrong.example",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-1"
              },
              {
                "id":"page-2",
                "type":"page",
                "url":"http://right.example",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-2"
              }
            ]"#,
        )
        .expect("targets parse");
        let selection = CdpTargetSelection::target_id("page-2").expect("selection parses");

        let config = select_page_target(&targets, &selection).expect("configured target selected");

        assert_eq!(
            config.target_ws_url,
            "ws://127.0.0.1:9222/devtools/page/page-2"
        );
    }

    #[test]
    fn rejects_missing_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"browser",
                "type":"browser",
                "url":"",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/browser/abc"
              }
            ]"#,
        )
        .expect("targets parse");

        let error = first_page_target(&targets).expect_err("missing page fails");
        assert!(error.to_string().contains("no matching page CDP target"));
    }

    fn write_scenario_result_fixture(run_dir: &Path, status: &str) {
        let scenario_dir = run_dir.join("evidence/scenarios/bootstrap-smoke");
        fs::create_dir_all(&scenario_dir).expect("scenario dir created");
        fs::write(scenario_dir.join("world-state.json"), "{}\n").expect("world state written");
        fs::write(scenario_dir.join("frame-stats.json"), "{}\n").expect("frame stats written");
        fs::write(
            scenario_dir.join("scenario-result.json"),
            format!(
                "{{\n  \"scenario_id\": \"bootstrap-smoke\",\n  \"status\": \"{status}\",\n  \"evidence\": {{\n    \"world_state\": \"evidence/scenarios/bootstrap-smoke/world-state.json\",\n    \"frame_stats\": \"evidence/scenarios/bootstrap-smoke/frame-stats.json\"\n  }},\n  \"assertions\": []\n}}\n"
            ),
        )
        .expect("scenario result written");
        for (id, path, artifact) in [
            (
                "fixture-world-state",
                "evidence/scenarios/bootstrap-smoke/world-state.json",
                "world_state",
            ),
            (
                "fixture-frame-stats",
                "evidence/scenarios/bootstrap-smoke/frame-stats.json",
                "frame_stats",
            ),
            (
                "fixture-scenario-result",
                "evidence/scenarios/bootstrap-smoke/scenario-result.json",
                "scenario_result",
            ),
        ] {
            add_evidence_artifact(
                run_dir,
                id,
                "application/json",
                path,
                json!({ "artifact": artifact, "scenario_id": "bootstrap-smoke" }),
            )
            .expect("artifact indexed");
        }
    }

    fn append_evaluator_config(run_dir: &Path, evaluator_yaml: &str) {
        let mut seed =
            fs::read_to_string(run_dir.join("seed.snapshot.yaml")).expect("seed snapshot reads");
        seed.push_str("evaluator:\n");
        seed.push_str(evaluator_yaml);
        fs::write(run_dir.join("seed.snapshot.yaml"), seed).expect("seed snapshot updated");
    }

    fn json_path_equals(path: &str, expected: serde_json::Value) -> JsonPathAssertion {
        json_path_assertion(path, |assertion| {
            assertion.equals = Some(expected);
        })
    }

    fn json_path_assertion(
        path: &str,
        configure: impl FnOnce(&mut JsonPathAssertion),
    ) -> JsonPathAssertion {
        let mut assertion = JsonPathAssertion {
            path: path.to_string(),
            equals: None,
            not_equals: None,
            exists: None,
            contains: None,
            greater_than: None,
            less_than: None,
            count_equals: None,
            count_greater_than: None,
            count_less_than: None,
        };
        configure(&mut assertion);
        assertion
    }

    #[allow(clippy::too_many_arguments)]
    fn assertion_sources_for_test<'a>(
        world_state: &'a serde_json::Value,
        frame_stats: &'a serde_json::Value,
        runtime_events: &'a serde_json::Value,
        performance_metrics: &'a serde_json::Value,
        console_errors: &'a serde_json::Value,
        collision_evidence: &'a serde_json::Value,
        audio_evidence: &'a serde_json::Value,
        animation_evidence: &'a serde_json::Value,
    ) -> ScenarioAssertionSources<'a> {
        ScenarioAssertionSources {
            world_state: AssertionSource {
                value: world_state,
                evidence_ref: "evidence/world-state.json",
            },
            frame_stats: AssertionSource {
                value: frame_stats,
                evidence_ref: "evidence/frame-stats.json",
            },
            runtime_events: AssertionSource {
                value: runtime_events,
                evidence_ref: "evidence/cdp-trace-summary.json",
            },
            performance_metrics: AssertionSource {
                value: performance_metrics,
                evidence_ref: "evidence/performance-metrics.json",
            },
            console_errors: AssertionSource {
                value: console_errors,
                evidence_ref: "evidence/console-log.json",
            },
            collision_evidence: AssertionSource {
                value: collision_evidence,
                evidence_ref: "evidence/world-state.json",
            },
            audio_evidence: AssertionSource {
                value: audio_evidence,
                evidence_ref: "evidence/world-state.json",
            },
            animation_evidence: AssertionSource {
                value: animation_evidence,
                evidence_ref: "evidence/world-state.json",
            },
        }
    }

    fn test_png_bytes(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        bytes.extend_from_slice(&13u32.to_be_bytes());
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&[8, 6, 0, 0, 0]);
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes
    }

    fn create_test_run(prefix: &str) -> (PathBuf, RunArtifacts) {
        let root = unique_temp_dir(prefix);
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");
        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");
        (root, artifacts)
    }

    fn create_scene_only_mutation_fixture(
        prefix: &str,
    ) -> (PathBuf, RunArtifacts, MutationProposal, PathBuf, SceneHash) {
        let (root, artifacts) = create_test_run(prefix);
        let scene_path = root.join("scene.json");
        fs::write(
            &scene_path,
            include_str!("../../../examples/game-runtime/scene.json"),
        )
        .expect("scene written");
        add_evidence_artifact(
            &artifacts.run_dir,
            "scene-only-mutation-evidence",
            "application/json",
            "evidence/scene-only-mutation.json",
            json!({ "source": "scene-only-mutation-test" }),
        )
        .expect("evidence indexed");
        let proposal = create_mutation_proposal(
            &artifacts.run_dir,
            MutationProposalInput {
                reason: "scene-only mutation fixture".to_string(),
                evidence_id: "scene-only-mutation-evidence".to_string(),
                target: scene_path.to_string_lossy().to_string(),
                path: "components.transform.x".to_string(),
                from: "32".to_string(),
                to: "48".to_string(),
            },
        )
        .expect("proposal created");
        let scene = read_scene(&scene_path).expect("scene reads");
        let before_hash = hash_scene_document(&scene).expect("scene hashes");
        (root, artifacts, proposal, scene_path, before_hash)
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            unix_millis().expect("time works")
        ))
    }
}
