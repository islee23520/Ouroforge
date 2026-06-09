use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const LEGACY_LOGIC_INGESTION_SCHEMA_VERSION: &str = "legacy.logic.ingestion.v1";
pub const LEGACY_LOGIC_INGESTION_BOUNDARY: &str = "source-project open/text read-only analysis; one-way on-ramp; clean-room re-derivation candidates only; no decompiled source copying; no engine runtime bridge; no ported claim without passing oracle";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicSource {
    pub path: String,
    pub kind: LegacyLogicSourceKind,
    pub text: String,
    #[serde(default)]
    pub source_only_attestation: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyLogicSourceKind {
    CSharpSource,
    Il2CppSignatureDump,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicAnalysis {
    pub schema_version: String,
    pub boundary: String,
    pub deterministic_digest: String,
    pub source_reports: Vec<LegacyLogicSourceReport>,
    pub ir_nodes: Vec<LegacyLogicIrNode>,
    pub call_edges: Vec<LegacyLogicCallEdge>,
    pub engine_api_touchpoints: Vec<EngineApiTouchpoint>,
    pub behavioral_units: Vec<BehavioralUnitRecord>,
    pub fidelity_report: LegacyLogicFidelityReport,
    pub re_derivation_tasks: Vec<ReDerivationTask>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicSourceReport {
    pub path: String,
    pub kind: LegacyLogicSourceKind,
    pub status: LegacyLogicSourceStatus,
    pub content_digest: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyLogicSourceStatus {
    Accepted,
    DegradedFallback,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicIrNode {
    pub id: String,
    pub source_path: String,
    pub kind: LegacyLogicIrNodeKind,
    pub name: String,
    pub parent_id: Option<String>,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyLogicIrNodeKind {
    Class,
    Method,
    Il2CppRecoveredSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicCallEdge {
    pub from_node_id: String,
    pub to_symbol: String,
    pub source_path: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineApiTouchpoint {
    pub id: String,
    pub node_id: String,
    pub source_path: String,
    pub line: usize,
    pub api: String,
    pub coupling: EngineCouplingKind,
    pub re_derivation_note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineCouplingKind {
    Lifecycle,
    Input,
    Physics,
    Scene,
    Ui,
    Animation,
    Audio,
    Rendering,
    Time,
    ObjectLifecycle,
    UnknownEngineApi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BehavioralUnitRecord {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub provenance_node_ids: Vec<String>,
    pub stimuli: Vec<String>,
    pub observed_outcomes: Vec<String>,
    pub engine_couplings: Vec<EngineCouplingKind>,
    pub oracle_status: OracleStatus,
    pub fidelity_grade: FidelityGrade,
    pub handoff_state: EraRHandoffState,
    pub ported_claim_allowed: bool,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleStatus {
    Missing,
    Captured,
    Passing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FidelityGrade {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EraRHandoffState {
    Interrogate,
    CaptureOracle,
    Reexpress,
    Verify,
    RejectOrDefer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogicFidelityReport {
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub no_oracle_not_ported: bool,
    pub clean_room_source_only: bool,
    pub deterministic_analysis: bool,
    pub unsupported_or_blocked: Vec<String>,
    pub gap_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReDerivationTask {
    pub unit_id: String,
    pub task: String,
    pub reason: String,
    pub handoff_state: EraRHandoffState,
}

pub fn analyze_legacy_logic(sources: &[LegacyLogicSource]) -> Result<LegacyLogicAnalysis> {
    if sources.is_empty() {
        return Err(anyhow!(
            "legacy logic ingestion requires at least one source"
        ));
    }

    let mut ordered = sources.to_vec();
    ordered.sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));

    let mut source_reports = Vec::new();
    let mut ir_nodes = Vec::new();
    let mut call_edges = BTreeSet::new();
    let mut touchpoints = BTreeSet::new();
    let mut blocked = Vec::new();

    for source in &ordered {
        validate_source_path(&source.path)?;
        let mut warnings = Vec::new();
        let content_digest = stable_digest(&source.text);
        let mut status = if source.source_only_attestation {
            LegacyLogicSourceStatus::Accepted
        } else {
            warnings.push(
                "missing source-only attestation; analysis rejected for semantic port claims"
                    .to_string(),
            );
            LegacyLogicSourceStatus::Rejected
        };

        if looks_like_decompiled_or_ripped(&source.text) {
            warnings.push("blocked decompiled/ripped-source marker detected; clean-room boundary requires source-project/open-text only".to_string());
            status = LegacyLogicSourceStatus::Rejected;
        }

        if source.kind == LegacyLogicSourceKind::Il2CppSignatureDump
            && status != LegacyLogicSourceStatus::Rejected
        {
            status = LegacyLogicSourceStatus::DegradedFallback;
            warnings.push("IL2CPP signature recovery is degraded metadata only; it cannot authorize source-code translation or a ported claim".to_string());
        }

        source_reports.push(LegacyLogicSourceReport {
            path: source.path.clone(),
            kind: source.kind,
            status,
            content_digest,
            warnings: warnings.clone(),
        });

        if status == LegacyLogicSourceStatus::Rejected {
            blocked.push(format!("{} rejected: {}", source.path, warnings.join("; ")));
            continue;
        }

        match source.kind {
            LegacyLogicSourceKind::CSharpSource => {
                parse_csharp_source(source, &mut ir_nodes, &mut call_edges, &mut touchpoints)
            }
            LegacyLogicSourceKind::Il2CppSignatureDump => {
                parse_il2cpp_signatures(source, &mut ir_nodes, &mut touchpoints)
            }
        }
    }

    ir_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    let call_edges: Vec<_> = call_edges.into_iter().collect();
    let engine_api_touchpoints: Vec<_> = touchpoints.into_iter().collect();
    let behavioral_units = extract_behavioral_units(&ir_nodes, &engine_api_touchpoints);
    let re_derivation_tasks = behavioral_units
        .iter()
        .map(|unit| ReDerivationTask {
            unit_id: unit.id.clone(),
            task: match unit.handoff_state {
                EraRHandoffState::Interrogate => "interrogate_intent".to_string(),
                EraRHandoffState::CaptureOracle => "capture_oracle".to_string(),
                EraRHandoffState::Reexpress => "deterministic_reexpression".to_string(),
                EraRHandoffState::Verify => "differential_verify".to_string(),
                EraRHandoffState::RejectOrDefer => "reject_or_defer".to_string(),
            },
            reason: unit.gaps.join("; "),
            handoff_state: unit.handoff_state,
        })
        .collect::<Vec<_>>();

    let fidelity_report = build_fidelity_report(&behavioral_units, blocked);
    let deterministic_digest = analysis_digest(
        &source_reports,
        &ir_nodes,
        &call_edges,
        &engine_api_touchpoints,
        &behavioral_units,
        &fidelity_report,
    );

    Ok(LegacyLogicAnalysis {
        schema_version: LEGACY_LOGIC_INGESTION_SCHEMA_VERSION.to_string(),
        boundary: LEGACY_LOGIC_INGESTION_BOUNDARY.to_string(),
        deterministic_digest,
        source_reports,
        ir_nodes,
        call_edges,
        engine_api_touchpoints,
        behavioral_units,
        fidelity_report,
        re_derivation_tasks,
    })
}

fn parse_csharp_source(
    source: &LegacyLogicSource,
    ir_nodes: &mut Vec<LegacyLogicIrNode>,
    call_edges: &mut BTreeSet<LegacyLogicCallEdge>,
    touchpoints: &mut BTreeSet<EngineApiTouchpoint>,
) {
    let mut current_class: Option<String> = None;
    let mut current_method: Option<String> = None;

    for (idx, raw_line) in source.text.lines().enumerate() {
        let line_no = idx + 1;
        let line = strip_line_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(class_name) = parse_class_name(line) {
            let id = node_id(&source.path, "class", &class_name, line_no);
            current_class = Some(id.clone());
            ir_nodes.push(LegacyLogicIrNode {
                id,
                source_path: source.path.clone(),
                kind: LegacyLogicIrNodeKind::Class,
                name: class_name,
                parent_id: None,
                line_start: line_no,
                line_end: line_no,
            });
        }
        if let Some(method_name) = parse_method_name(line) {
            let id = node_id(&source.path, "method", &method_name, line_no);
            current_method = Some(id.clone());
            ir_nodes.push(LegacyLogicIrNode {
                id,
                source_path: source.path.clone(),
                kind: LegacyLogicIrNodeKind::Method,
                name: method_name,
                parent_id: current_class.clone(),
                line_start: line_no,
                line_end: line_no,
            });
        }

        let node_id = current_method
            .clone()
            .or_else(|| current_class.clone())
            .unwrap_or_else(|| node_id(&source.path, "file", "top_level", 1));

        for symbol in parse_call_symbols(line) {
            call_edges.insert(LegacyLogicCallEdge {
                from_node_id: node_id.clone(),
                to_symbol: symbol,
                source_path: source.path.clone(),
                line: line_no,
            });
        }
        for (api, coupling) in detect_engine_api_touchpoints(line) {
            let id = format!(
                "touch:{}:{}:{}",
                sanitize(&source.path),
                line_no,
                sanitize(&api)
            );
            touchpoints.insert(EngineApiTouchpoint {
                id,
                node_id: node_id.clone(),
                source_path: source.path.clone(),
                line: line_no,
                api: api.clone(),
                coupling,
                re_derivation_note: format!(
                    "{} coupling must be re-derived from observed behavior and an oracle; do not translate source engine semantics",
                    api
                ),
            });
        }
    }
}

fn parse_il2cpp_signatures(
    source: &LegacyLogicSource,
    ir_nodes: &mut Vec<LegacyLogicIrNode>,
    touchpoints: &mut BTreeSet<EngineApiTouchpoint>,
) {
    for (idx, raw_line) in source.text.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw_line.trim();
        if !line.contains("::") || !line.contains('(') {
            continue;
        }
        let signature = line
            .split_whitespace()
            .find(|part| part.contains("::") && part.contains('('))
            .unwrap_or(line)
            .trim_matches(|c: char| c == ';' || c == ',')
            .to_string();
        let method_name = signature
            .rsplit("::")
            .next()
            .and_then(|part| part.split('(').next())
            .unwrap_or("unknown")
            .to_string();
        let id = node_id(&source.path, "il2cpp", &method_name, line_no);
        ir_nodes.push(LegacyLogicIrNode {
            id: id.clone(),
            source_path: source.path.clone(),
            kind: LegacyLogicIrNodeKind::Il2CppRecoveredSignature,
            name: signature.clone(),
            parent_id: None,
            line_start: line_no,
            line_end: line_no,
        });
        for (api, coupling) in detect_engine_api_touchpoints(line) {
            touchpoints.insert(EngineApiTouchpoint {
                id: format!("touch:{}:{}:{}", sanitize(&source.path), line_no, sanitize(&api)),
                node_id: id.clone(),
                source_path: source.path.clone(),
                line: line_no,
                api: api.clone(),
                coupling,
                re_derivation_note: format!(
                    "{} recovered from IL2CPP signature metadata only; requires clean-room interrogation/oracle before any re-expression",
                    api
                ),
            });
        }
    }
}

fn extract_behavioral_units(
    ir_nodes: &[LegacyLogicIrNode],
    touchpoints: &[EngineApiTouchpoint],
) -> Vec<BehavioralUnitRecord> {
    let node_by_id: BTreeMap<_, _> = ir_nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let mut by_node: BTreeMap<String, Vec<&EngineApiTouchpoint>> = BTreeMap::new();
    for touchpoint in touchpoints {
        by_node
            .entry(touchpoint.node_id.clone())
            .or_default()
            .push(touchpoint);
    }

    by_node
        .into_iter()
        .map(|(node_id, points)| {
            let node = node_by_id.get(node_id.as_str());
            let source_path = node
                .map(|node| node.source_path.clone())
                .or_else(|| points.first().map(|p| p.source_path.clone()))
                .unwrap_or_default();
            let name = node
                .map(|node| node.name.clone())
                .unwrap_or_else(|| "unknown_behavior".to_string());
            let mut couplings = points.iter().map(|p| p.coupling).collect::<BTreeSet<_>>();
            let mut stimuli = BTreeSet::new();
            let mut outcomes = BTreeSet::new();
            let mut gaps = BTreeSet::new();
            for coupling in &couplings {
                match coupling {
                    EngineCouplingKind::Input => {
                        stimuli.insert("input action or axis from source project".to_string());
                        outcomes.insert(
                            "Ouroforge deterministic state transition under captured input"
                                .to_string(),
                        );
                    }
                    EngineCouplingKind::Physics => {
                        stimuli.insert("collision/trigger/physics contact event".to_string());
                        outcomes.insert(
                            "deterministic re-simulated physics outcome and state hash".to_string(),
                        );
                        gaps.insert(
                            "source physics must be re-simulated, not reproduced".to_string(),
                        );
                    }
                    EngineCouplingKind::Scene => {
                        stimuli.insert("scene load or transition trigger".to_string());
                        outcomes
                            .insert("native scene/state transition with provenance".to_string());
                    }
                    EngineCouplingKind::Ui => {
                        stimuli.insert("UI event or binding".to_string());
                        outcomes.insert("native UI/read-model state outcome".to_string());
                    }
                    EngineCouplingKind::Animation => {
                        stimuli.insert("animation marker/state transition".to_string());
                        outcomes.insert(
                            "state-hash primary plus presentation fidelity report".to_string(),
                        );
                    }
                    EngineCouplingKind::Audio => {
                        stimuli.insert("audio trigger".to_string());
                        outcomes.insert(
                            "best-effort content fidelity with explicit gap report".to_string(),
                        );
                        gaps.insert(
                            "audio feel/fidelity remains best-effort unless later oracle covers it"
                                .to_string(),
                        );
                    }
                    EngineCouplingKind::Rendering => {
                        stimuli.insert("render/camera/presentation event".to_string());
                        outcomes.insert(
                            "state-hash primary; perceptual render comparison secondary"
                                .to_string(),
                        );
                    }
                    EngineCouplingKind::Time => {
                        stimuli.insert("time-step or timer tick".to_string());
                        outcomes.insert("deterministic fixed-step timing outcome".to_string());
                    }
                    EngineCouplingKind::Lifecycle | EngineCouplingKind::ObjectLifecycle => {
                        stimuli.insert("engine lifecycle event".to_string());
                        outcomes.insert(
                            "native lifecycle-equivalent state initialization or teardown"
                                .to_string(),
                        );
                    }
                    EngineCouplingKind::UnknownEngineApi => {
                        stimuli.insert("unknown engine API touchpoint".to_string());
                        outcomes.insert(
                            "requires human interrogation before oracle capture".to_string(),
                        );
                        gaps.insert(
                            "unknown engine API coupling requires manual classification"
                                .to_string(),
                        );
                    }
                }
            }
            if stimuli.is_empty() {
                stimuli.insert("observed source behavior stimulus to be captured".to_string());
            }
            if outcomes.is_empty() {
                outcomes.insert("source-independent outcome oracle required".to_string());
            }
            gaps.insert("oracle missing; no ported claim allowed".to_string());

            let has_red = couplings.contains(&EngineCouplingKind::UnknownEngineApi)
                || node
                    .map(|n| n.kind == LegacyLogicIrNodeKind::Il2CppRecoveredSignature)
                    .unwrap_or(false);
            let grade = if has_red {
                FidelityGrade::Red
            } else {
                FidelityGrade::Yellow
            };
            let handoff = if has_red {
                EraRHandoffState::Interrogate
            } else {
                EraRHandoffState::CaptureOracle
            };
            let engine_couplings = couplings.drain_filter_collect();

            BehavioralUnitRecord {
                id: format!("unit:{}", sanitize(&node_id)),
                name: format!("Re-derive {}", name),
                source_path,
                provenance_node_ids: vec![node_id.clone()],
                stimuli: stimuli.into_iter().collect(),
                observed_outcomes: outcomes.into_iter().collect(),
                engine_couplings,
                oracle_status: OracleStatus::Missing,
                fidelity_grade: grade,
                handoff_state: handoff,
                ported_claim_allowed: false,
                gaps: gaps.into_iter().collect(),
            }
        })
        .collect()
}

trait DrainBTreeSet<T> {
    fn drain_filter_collect(&mut self) -> Vec<T>;
}

impl<T: Ord + Copy> DrainBTreeSet<T> for BTreeSet<T> {
    fn drain_filter_collect(&mut self) -> Vec<T> {
        self.iter().copied().collect()
    }
}

fn build_fidelity_report(
    units: &[BehavioralUnitRecord],
    unsupported_or_blocked: Vec<String>,
) -> LegacyLogicFidelityReport {
    let green_count = units
        .iter()
        .filter(|u| u.fidelity_grade == FidelityGrade::Green)
        .count();
    let yellow_count = units
        .iter()
        .filter(|u| u.fidelity_grade == FidelityGrade::Yellow)
        .count();
    let red_count = units
        .iter()
        .filter(|u| u.fidelity_grade == FidelityGrade::Red)
        .count()
        + unsupported_or_blocked.len();
    let mut gap_summary = BTreeSet::new();
    for unit in units {
        for gap in &unit.gaps {
            gap_summary.insert(gap.clone());
        }
    }
    for blocked in &unsupported_or_blocked {
        gap_summary.insert(blocked.clone());
    }
    if units.is_empty() {
        gap_summary.insert("no behavioral units extracted; source remains unported".to_string());
    }

    LegacyLogicFidelityReport {
        green_count,
        yellow_count,
        red_count,
        no_oracle_not_ported: true,
        clean_room_source_only: true,
        deterministic_analysis: true,
        unsupported_or_blocked,
        gap_summary: gap_summary.into_iter().collect(),
    }
}

fn analysis_digest(
    source_reports: &[LegacyLogicSourceReport],
    ir_nodes: &[LegacyLogicIrNode],
    call_edges: &[LegacyLogicCallEdge],
    touchpoints: &[EngineApiTouchpoint],
    units: &[BehavioralUnitRecord],
    fidelity: &LegacyLogicFidelityReport,
) -> String {
    let value = serde_json::json!({
        "sourceReports": source_reports,
        "irNodes": ir_nodes,
        "callEdges": call_edges,
        "engineApiTouchpoints": touchpoints,
        "behavioralUnits": units,
        "fidelityReport": fidelity,
    });
    stable_digest(&value.to_string())
}

fn parse_class_name(line: &str) -> Option<String> {
    let tokens = tokens(line);
    tokens
        .windows(2)
        .find(|w| w[0] == "class")
        .map(|w| w[1].clone())
}

fn parse_method_name(line: &str) -> Option<String> {
    if !line.contains('(')
        || line.starts_with("if ")
        || line.starts_with("for ")
        || line.starts_with("while ")
        || line.starts_with("switch ")
    {
        return None;
    }
    let before = line.split('(').next()?.trim();
    let name = before.split_whitespace().last()?.trim();
    if name.is_empty() || !name.chars().next()?.is_ascii_alphabetic() {
        return None;
    }
    let method_markers = [
        "public",
        "private",
        "protected",
        "internal",
        "static",
        "override",
        "virtual",
        "void",
        "bool",
        "int",
        "float",
        "double",
        "string",
        "IEnumerator",
        "Task",
    ];
    if before
        .split_whitespace()
        .any(|token| method_markers.contains(&token))
    {
        Some(
            name.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .to_string(),
        )
    } else {
        None
    }
}

fn parse_call_symbols(line: &str) -> Vec<String> {
    let mut result = BTreeSet::new();
    for segment in line.split('(').take(16).skip(1) {
        let prefix = line[..line.find(segment).unwrap_or(line.len())].trim_end_matches('(');
        if let Some(symbol) = prefix
            .rsplit(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
            .next()
        {
            let symbol = symbol.trim_matches('.');
            if !symbol.is_empty()
                && !matches!(symbol, "if" | "for" | "while" | "switch" | "return" | "new")
            {
                result.insert(symbol.to_string());
            }
        }
    }
    result.into_iter().collect()
}

fn detect_engine_api_touchpoints(line: &str) -> Vec<(String, EngineCouplingKind)> {
    let patterns = [
        ("Input.", EngineCouplingKind::Input),
        ("GetAxis", EngineCouplingKind::Input),
        ("GetButton", EngineCouplingKind::Input),
        ("OnCollision", EngineCouplingKind::Physics),
        ("OnTrigger", EngineCouplingKind::Physics),
        ("Rigidbody", EngineCouplingKind::Physics),
        ("Collider", EngineCouplingKind::Physics),
        ("Physics", EngineCouplingKind::Physics),
        ("SceneManager", EngineCouplingKind::Scene),
        ("LoadScene", EngineCouplingKind::Scene),
        ("UnityEngine.UI", EngineCouplingKind::Ui),
        ("Button", EngineCouplingKind::Ui),
        ("TextMeshPro", EngineCouplingKind::Ui),
        ("Animator", EngineCouplingKind::Animation),
        ("Animation", EngineCouplingKind::Animation),
        ("AudioSource", EngineCouplingKind::Audio),
        ("AudioClip", EngineCouplingKind::Audio),
        ("Camera", EngineCouplingKind::Rendering),
        ("Renderer", EngineCouplingKind::Rendering),
        ("Time.", EngineCouplingKind::Time),
        ("Start", EngineCouplingKind::Lifecycle),
        ("Update", EngineCouplingKind::Lifecycle),
        ("FixedUpdate", EngineCouplingKind::Lifecycle),
        ("Awake", EngineCouplingKind::Lifecycle),
        ("Instantiate", EngineCouplingKind::ObjectLifecycle),
        ("Destroy", EngineCouplingKind::ObjectLifecycle),
        ("UnityEngine", EngineCouplingKind::UnknownEngineApi),
    ];
    let mut found = BTreeMap::new();
    for (needle, coupling) in patterns {
        if line.contains(needle) {
            found.insert(needle.to_string(), coupling);
        }
    }
    found.into_iter().collect()
}

fn strip_line_comment(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

fn tokens(line: &str) -> Vec<String> {
    line.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn node_id(path: &str, kind: &str, name: &str, line: usize) -> String {
    format!("{}:{}:{}:{}", kind, sanitize(path), sanitize(name), line)
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn validate_source_path(path: &str) -> Result<()> {
    if path.is_empty() || path.starts_with('/') || path.contains("..") || path.contains('\\') {
        return Err(anyhow!(
            "legacy logic source path must be safe repo-relative: {path}"
        ));
    }
    Ok(())
}

fn looks_like_decompiled_or_ripped(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    [
        "decompiled with",
        "generated by ilspy",
        "generated by dnspy",
        "assetstudio",
        "ripped from",
        "dump.cs",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn stable_digest(text: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player_source() -> LegacyLogicSource {
        LegacyLogicSource {
            path: "Assets/Scripts/PlayerController.cs".to_string(),
            kind: LegacyLogicSourceKind::CSharpSource,
            source_only_attestation: true,
            text: r#"
using UnityEngine;
using UnityEngine.SceneManagement;
public class PlayerController : MonoBehaviour {
  public Rigidbody2D body;
  void Update() {
    var x = Input.GetAxis("Horizontal");
    body.velocity = new Vector2(x * 4f, body.velocity.y);
    if (Input.GetButtonDown("Jump")) { body.AddForce(Vector2.up); }
  }
  void OnTriggerEnter2D(Collider2D other) {
    if (other.name == "Exit") { SceneManager.LoadScene("Win"); }
  }
}
"#
            .to_string(),
        }
    }

    #[test]
    fn csharp_analysis_extracts_read_only_units_and_couplings() {
        let analysis = analyze_legacy_logic(&[player_source()]).unwrap();
        assert_eq!(
            analysis.schema_version,
            LEGACY_LOGIC_INGESTION_SCHEMA_VERSION
        );
        assert!(analysis.boundary.contains("clean-room"));
        assert!(analysis
            .source_reports
            .iter()
            .all(|report| report.status == LegacyLogicSourceStatus::Accepted));
        assert!(analysis.ir_nodes.iter().any(
            |node| node.kind == LegacyLogicIrNodeKind::Class && node.name == "PlayerController"
        ));
        assert!(analysis
            .engine_api_touchpoints
            .iter()
            .any(|touch| touch.coupling == EngineCouplingKind::Input));
        assert!(analysis
            .engine_api_touchpoints
            .iter()
            .any(|touch| touch.coupling == EngineCouplingKind::Physics));
        assert!(analysis
            .engine_api_touchpoints
            .iter()
            .any(|touch| touch.coupling == EngineCouplingKind::Scene));
        assert!(!analysis.behavioral_units.is_empty());
        assert!(analysis
            .behavioral_units
            .iter()
            .all(|unit| unit.oracle_status == OracleStatus::Missing && !unit.ported_claim_allowed));
        assert!(analysis.fidelity_report.no_oracle_not_ported);
        assert_eq!(analysis.fidelity_report.green_count, 0);
        assert!(analysis.fidelity_report.yellow_count >= 1);
    }

    #[test]
    fn analysis_is_deterministic_regardless_of_input_order() {
        let il2cpp = LegacyLogicSource {
            path: "Il2Cpp/signatures.txt".to_string(),
            kind: LegacyLogicSourceKind::Il2CppSignatureDump,
            source_only_attestation: true,
            text: "Game.Enemy::FixedUpdate()\nGame.Enemy::OnCollisionEnter(UnityEngine.Collider)"
                .to_string(),
        };
        let a = analyze_legacy_logic(&[player_source(), il2cpp.clone()]).unwrap();
        let b = analyze_legacy_logic(&[il2cpp, player_source()]).unwrap();
        assert_eq!(a.deterministic_digest, b.deterministic_digest);
        assert_eq!(a.ir_nodes, b.ir_nodes);
        assert!(a
            .source_reports
            .iter()
            .any(|report| report.status == LegacyLogicSourceStatus::DegradedFallback));
        assert!(a.fidelity_report.red_count >= 1);
    }

    #[test]
    fn decompiled_or_unattested_sources_are_rejected_without_units() {
        let rejected = LegacyLogicSource {
            path: "Assets/Scripts/Dump.cs".to_string(),
            kind: LegacyLogicSourceKind::CSharpSource,
            source_only_attestation: false,
            text: "// Decompiled with ILSpy\npublic class Dump { void Update() { UnityEngine.Debug.Log(1); } }".to_string(),
        };
        let analysis = analyze_legacy_logic(&[rejected]).unwrap();
        assert_eq!(
            analysis.source_reports[0].status,
            LegacyLogicSourceStatus::Rejected
        );
        assert!(analysis.ir_nodes.is_empty());
        assert!(analysis.behavioral_units.is_empty());
        assert!(analysis.fidelity_report.red_count >= 1);
        assert!(analysis
            .fidelity_report
            .gap_summary
            .iter()
            .any(|gap| gap.contains("clean-room")));
    }

    #[test]
    fn unsafe_source_paths_fail_closed() {
        let bad = LegacyLogicSource {
            path: "../Assets/Bad.cs".to_string(),
            kind: LegacyLogicSourceKind::CSharpSource,
            source_only_attestation: true,
            text: "public class Bad {}".to_string(),
        };
        assert!(analyze_legacy_logic(&[bad]).is_err());
    }
}
