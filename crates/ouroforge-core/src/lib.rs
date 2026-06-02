use anyhow::{anyhow, Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::{Component, Path, PathBuf};
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

    let summary = EvolveSummary {
        status: "proposed".to_string(),
        proposals_created: proposal_ids.len(),
        proposal_ids,
        reason: "failed verdict produced deterministic placeholder mutation proposal".to_string(),
    };
    append_ledger_event(
        run_dir,
        "evolve.completed",
        "evolve-cli",
        json!({
            "status": summary.status,
            "proposals_created": summary.proposals_created,
            "proposal_ids": summary.proposal_ids
        }),
    )?;
    update_journal(run_dir)?;
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
    let proposals = read_mutation_proposals(run_dir)?.proposals;
    let journal = render_journal(&seed, &evidence, &ledger, &verdict, &proposals);
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
                "scenario_results": 0
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
            "scenario_results": scenario_results.len()
        }),
    };
    write_json(&run_dir.join("verdict.json"), &json!(verdict))?;
    Ok(verdict)
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

    let mut evidence_paths = Vec::new();
    let mut result_paths = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    for scenario in &seed.scenarios {
        let result = run_scenario(config, &mut client, scenario)?;
        evidence_paths.extend(result.evidence_paths);
        result_paths.push(result.result_path);
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    Ok(ScenarioRunSummary {
        scenarios: seed.scenarios.len(),
        completed: result_paths.len(),
        passed,
        failed,
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
    if let Ok(console_logs) = client.evaluate_json("window.__OUROFORGE_CONSOLE__ || []") {
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
    if let Ok(performance_metrics) = client
        .enable_performance()
        .and_then(|_| client.performance_metrics())
    {
        scenario_performance_metric_count = Some(count_cdp_metrics(&performance_metrics));
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
    trace_paths.push(trace_path);

    let assertions = evaluate_scenario_assertions(scenario, &world_state, &frame_stats);
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
                "evidence_path": if assertion["target"] == "world_state" { &world_state_path } else { &frame_stats_path }
            }),
        )?;
    }
    let passed = assertions
        .iter()
        .all(|assertion| assertion["passed"].as_bool() == Some(true));
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
                "console_logs": console_paths.clone(),
                "performance_metrics": performance_paths.clone(),
                "cdp_trace_summaries": trace_paths.clone()
            },
            "assertions": assertions
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
            "console_log_paths": console_paths.clone(),
            "performance_metric_paths": performance_paths.clone(),
            "cdp_trace_summary_paths": trace_paths.clone(),
            "result_path": result_path
        }),
    )?;
    let mut evidence_paths = replay_paths;
    evidence_paths.extend(snapshot_paths);
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

fn evaluate_scenario_assertions(
    scenario: &Scenario,
    world_state: &serde_json::Value,
    frame_stats: &serde_json::Value,
) -> Vec<serde_json::Value> {
    scenario
        .assertions
        .iter()
        .map(|assertion| {
            let (target, assertion) = match assertion {
                ScenarioAssertion::WorldState { world_state } => ("world_state", world_state),
                ScenarioAssertion::FrameStats { frame_stats } => ("frame_stats", frame_stats),
                ScenarioAssertion::RuntimeEvents { runtime_events } => {
                    ("runtime_events", runtime_events)
                }
                ScenarioAssertion::PerformanceMetrics {
                    performance_metrics,
                } => ("performance_metrics", performance_metrics),
                ScenarioAssertion::ConsoleErrors { console_errors } => {
                    ("console_errors", console_errors)
                }
                ScenarioAssertion::CollisionEvidence { collision_evidence } => {
                    ("collision_evidence", collision_evidence)
                }
                ScenarioAssertion::AudioEvidence { audio_evidence } => {
                    ("audio_evidence", audio_evidence)
                }
                ScenarioAssertion::AnimationEvidence { animation_evidence } => {
                    ("animation_evidence", animation_evidence)
                }
            };
            let null_source = serde_json::Value::Null;
            let source = match target {
                "world_state" => world_state,
                "frame_stats" => frame_stats,
                _ => &null_source,
            };
            let actual = read_json_path(source, &assertion.path)
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            let passed = assertion
                .equals
                .as_ref()
                .is_some_and(|expected| &actual == expected);
            json!({
                "target": target,
                "path": assertion.path,
                "operator": if assertion.equals.is_some() { "equals" } else { "unsupported_until_evaluation_pr" },
                "expected": assertion.equals,
                "actual": actual,
                "passed": passed
            })
        })
        .collect()
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

fn execute_scenario_step<T: CdpTransport>(
    client: &mut CdpClient<T>,
    step: &ScenarioStep,
) -> Result<()> {
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
        ScenarioStep::Snapshot { .. } | ScenarioStep::Restore { .. } => {
            return Err(anyhow!(
                "snapshot/restore steps require scenario evidence context"
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
    pub entities: Vec<SceneEntity>,
    #[serde(default = "empty_json_object")]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneBounds {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneEntity {
    pub id: String,
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
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneAnimation {
    pub mode: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneEdit {
    pub entity_id: String,
    pub path: String,
    pub value: serde_json::Value,
}

fn scene_schema_v1() -> String {
    "1".to_string()
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
    validate_scene_metadata("scene metadata", &scene.metadata)?;
    let mut ids = std::collections::BTreeSet::new();
    for entity in &scene.entities {
        validate_path_component("scene entity id", &entity.id)?;
        if !ids.insert(entity.id.clone()) {
            return Err(anyhow!("duplicate scene entity id: {}", entity.id));
        }
        validate_scene_color(&entity.sprite.color)?;
        if let Some(asset) = &entity.sprite.asset {
            validate_scene_local_asset_path("scene sprite asset", asset)?;
        }
        if entity.components.size.width <= 0 || entity.components.size.height <= 0 {
            return Err(anyhow!("scene entity {} size must be positive", entity.id));
        }
        if let Some(collider) = &entity.components.collider {
            validate_scene_collider(&entity.id, collider)?;
        }
        if let Some(animation) = &entity.components.animation {
            validate_scene_animation(&entity.id, animation)?;
        }
        if let Some(audio) = &entity.components.audio {
            validate_scene_audio(&entity.id, audio)?;
        }
        validate_scene_tags(&entity.id, &entity.tags)?;
        validate_scene_metadata(
            &format!("scene entity {} metadata", entity.id),
            &entity.metadata,
        )?;
    }
    Ok(())
}

fn validate_scene_collider(entity_id: &str, collider: &SceneCollider) -> Result<()> {
    if collider.shape != "aabb" {
        return Err(anyhow!(
            "scene entity {entity_id} collider shape must be aabb"
        ));
    }
    if !matches!(collider.body.as_str(), "static" | "dynamic") {
        return Err(anyhow!(
            "scene entity {entity_id} collider body must be static or dynamic"
        ));
    }
    if collider.size.width <= 0 || collider.size.height <= 0 {
        return Err(anyhow!(
            "scene entity {entity_id} collider size must be positive"
        ));
    }
    Ok(())
}

fn validate_scene_animation(entity_id: &str, animation: &SceneAnimation) -> Result<()> {
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
    if animation.frames.is_empty() {
        return Err(anyhow!(
            "scene entity {entity_id} animation frames must not be empty"
        ));
    }
    for frame in &animation.frames {
        validate_scene_color(&frame.color).with_context(|| {
            format!("scene entity {entity_id} animation frame color is invalid")
        })?;
    }
    Ok(())
}

fn validate_scene_audio(entity_id: &str, audio: &SceneAudio) -> Result<()> {
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
        if let Some(asset) = &event.asset {
            validate_scene_local_asset_path("scene audio asset", asset)?;
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
                "unsupported scene edit path `{}`; supported paths are sprite.color, components.transform.x/y, components.velocity.x/y, components.size.width/height, components.controllable",
                edit.path
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
    pub created_at_unix_ms: u128,
    pub evidence_count: usize,
    pub mutation_count: usize,
    pub journal_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardArtifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub metadata: serde_json::Value,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReadModel {
    pub summary: RunDashboardSummary,
    pub run: serde_json::Value,
    pub verdict: serde_json::Value,
    pub journal: String,
    pub evidence: Vec<EvidenceArtifact>,
    pub screenshots: Vec<RunDashboardArtifact>,
    pub console_logs: Vec<RunDashboardArtifact>,
    pub world_states: Vec<RunDashboardArtifact>,
    pub mutations: Vec<MutationProposal>,
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
    let journal = fs::read_to_string(run_dir.join("journal.md"))
        .with_context(|| format!("failed to read journal for {}", run_dir.display()))?;
    let evidence = read_evidence_index(run_dir)?.artifacts;
    let screenshots =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_screenshot)?;
    let console_logs =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_console_log)?;
    let world_states =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_world_state)?;
    let mutations = list_mutation_proposals(run_dir)?;
    Ok(RunDashboardReadModel {
        summary,
        run,
        verdict,
        journal,
        evidence,
        screenshots,
        console_logs,
        world_states,
        mutations,
    })
}

fn read_dashboard_run_summary(run_dir: &Path) -> Result<RunDashboardSummary> {
    let run = read_json_value(run_dir.join("run.json"))?;
    let evidence_count = read_evidence_index(run_dir)?.artifacts.len();
    let mutations = list_mutation_proposals(run_dir)?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
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
        created_at_unix_ms: run
            .get("created_at_unix_ms")
            .and_then(|value| value.as_u64())
            .map(u128::from)
            .unwrap_or(0),
        evidence_count,
        mutation_count: mutations.len(),
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
    let value = if artifact.kind == "application/json" || artifact.path.ends_with(".json") {
        Some(read_json_value(run_dir.join(&artifact.path))?)
    } else {
        None
    };
    Ok(RunDashboardArtifact {
        id: artifact.id.clone(),
        kind: artifact.kind.clone(),
        path: artifact.path.clone(),
        metadata: artifact.metadata.clone(),
        value,
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
            self.calls.push(
                params["expression"]
                    .as_str()
                    .expect("expression is present")
                    .to_string(),
            );
            Ok(json!({ "result": { "value": {} } }))
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
        let proposals = list_mutation_proposals(&artifacts.run_dir).expect("proposals list");
        assert_eq!(proposals[0].evidence_id, "failure-evidence");
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
            ],
        };

        let assertions = evaluate_scenario_assertions(
            &scenario,
            &json!({
                "tick": 2,
                "object": { "id": "probe-square" },
                "collisions": [{ "pairId": "goal:player" }]
            }),
            &json!({ "fixedDeltaMs": 16 }),
        );

        assert_eq!(assertions.len(), 4);
        assert_eq!(assertions[0]["passed"], true);
        assert_eq!(assertions[1]["passed"], true);
        assert_eq!(assertions[2]["passed"], false);
        assert_eq!(assertions[2]["actual"], "probe-square");
        assert_eq!(assertions[3]["passed"], true);
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
        assert_eq!(transport.calls[0], "window.__OUROFORGE__.step(3)");
        assert_eq!(
            transport.calls[1],
            "window.__OUROFORGE__.setInput({\"right\":true})"
        );
        assert_eq!(
            transport.calls[2],
            "window.__OUROFORGE__.setInput({\"right\":true})"
        );
        assert_eq!(transport.calls[3], "window.__OUROFORGE__.step(4)");
        assert_eq!(
            transport.calls[4],
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
                "Page.captureScreenshot" => Ok(json!({ "data": "iVBORw0KGgo=" })),
                _ => Ok(json!({})),
            }
        }
    }

    #[test]
    fn cdp_client_decodes_screenshot_data() {
        let mut client = CdpClient::new(ScreenshotTransport);

        let bytes = client.capture_screenshot_png().expect("screenshot decodes");

        assert_eq!(bytes, vec![137, 80, 78, 71, 13, 10, 26, 10]);
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
    fn scene_schema_v1_rejects_invalid_entities_and_paths() {
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

        let future_body = serde_json::from_value::<SceneDocument>(json!({
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
        let rejected = validate_scene(&future_body).expect_err("future body rejected");
        assert!(rejected
            .to_string()
            .contains("collider body must be static or dynamic"));

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
        assert_eq!(runs[0].evidence_count, 3);
        assert_eq!(runs[0].mutation_count, 1);

        let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run read");
        assert_eq!(model.summary.id, runs[0].id);
        assert_eq!(model.evidence.len(), 3);
        assert_eq!(model.screenshots.len(), 1);
        assert_eq!(model.world_states.len(), 1);
        assert_eq!(model.console_logs.len(), 1);
        assert_eq!(model.mutations.len(), 1);
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
        JsonPathAssertion {
            path: path.to_string(),
            equals: Some(expected),
            not_equals: None,
            exists: None,
            contains: None,
            greater_than: None,
            less_than: None,
            count_equals: None,
            count_greater_than: None,
            count_less_than: None,
        }
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

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            unix_millis().expect("time works")
        ))
    }
}
