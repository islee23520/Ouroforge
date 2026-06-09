use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::godot_2d_adapter_ir::{FidelityGrade as AdapterFidelityGrade, GodotMigrationIr};

pub const LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION: &str = "logic-touchpoint-handoff-v1";
pub const LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY: &str = "one-way source-project/open-text logic inventory; clean-room Era R re-derivation hand-off; oracle-gated; no auto-port";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogicCouplingKind {
    Script,
    Signal,
    Input,
    Physics,
    Animation,
    Audio,
    Rendering,
    Scene,
    UnsupportedEngineFeature,
    UnknownEngineApi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogicHandoffFidelityGrade {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicTouchpointHandoffArtifact {
    pub schema_version: String,
    pub boundary: String,
    pub source_project: String,
    pub source_ir_hash: String,
    pub touchpoints: Vec<LogicTouchpointRecord>,
    pub behavioral_units: Vec<LogicBehavioralUnitRecord>,
    pub era_r_tasks: Vec<EraRReDerivationTask>,
    pub oracle_requirements: Vec<LogicOracleRequirement>,
    pub fidelity_report: LogicHandoffFidelityReport,
    pub state_hash: String,
    pub claimed_ported_units: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicTouchpointRecord {
    pub id: String,
    pub source_id: String,
    pub source_path: String,
    pub node_id: Option<String>,
    pub trigger_kind: String,
    pub symbol: Option<String>,
    pub exported_variables: Vec<String>,
    pub coupling: LogicCouplingKind,
    pub fidelity_grade: LogicHandoffFidelityGrade,
    pub era_r_status: String,
    pub gap_reason: String,
    pub clean_room_instruction: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicBehavioralUnitRecord {
    pub id: String,
    pub touchpoint_id: String,
    pub coupling: LogicCouplingKind,
    pub stimuli: Vec<String>,
    pub observed_outcomes: Vec<String>,
    pub oracle_status: String,
    pub fidelity_grade: LogicHandoffFidelityGrade,
    pub ported_claim_allowed: bool,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EraRReDerivationTask {
    pub id: String,
    pub unit_id: String,
    pub task: String,
    pub reason: String,
    pub target_era: String,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicOracleRequirement {
    pub unit_id: String,
    pub status: String,
    pub required_evidence: Vec<String>,
    pub ported_claim_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicHandoffFidelityReport {
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
    pub oracle_rule: String,
    pub gap_summary: Vec<String>,
    pub clean_room_notice: String,
}

pub fn detect_godot_logic_touchpoints(
    ir: &GodotMigrationIr,
) -> Result<LogicTouchpointHandoffArtifact> {
    let source_ir_hash = crate::export_hash::sha256_prefixed(&serde_json::to_vec(ir)?);
    let mut touchpoints = Vec::new();

    for touchpoint in &ir.logic_touchpoints {
        let coupling =
            coupling_for_touchpoint(&touchpoint.trigger_kind, touchpoint.symbol.as_deref());
        let id = format!("touchpoint:{}", touchpoint.id);
        touchpoints.push(LogicTouchpointRecord {
            id,
            source_id: touchpoint.id.clone(),
            source_path: touchpoint.source_path.clone(),
            node_id: touchpoint.node_id.clone(),
            trigger_kind: touchpoint.trigger_kind.clone(),
            symbol: touchpoint.symbol.clone(),
            exported_variables: sorted(touchpoint.exported_variables.clone()),
            coupling,
            fidelity_grade: LogicHandoffFidelityGrade::Red,
            era_r_status: "requires-clean-room-re-derivation".to_string(),
            gap_reason: "Behavior-bearing source-engine touchpoint is inventoried only; no code is copied, translated, or claimed ported.".to_string(),
            clean_room_instruction: "Re-derive in Era R from observed behavior plus interrogated intent; never translate or copy source/decompiled code.".to_string(),
            provenance: format!(
                "{}:{}:{}",
                touchpoint.provenance.source_path,
                touchpoint.provenance.line,
                touchpoint.provenance.section
            ),
        });
    }

    for unsupported in &ir.unsupported {
        let coupling = coupling_for_unsupported(&unsupported.feature_kind);
        let source_id = unsupported
            .node_id
            .clone()
            .unwrap_or_else(|| format!("{}:{}", unsupported.source_path, unsupported.feature_kind));
        let id = format!(
            "touchpoint:unsupported:{}:{}",
            unsupported.source_path, unsupported.feature_kind
        );
        touchpoints.push(LogicTouchpointRecord {
            id,
            source_id,
            source_path: unsupported.source_path.clone(),
            node_id: unsupported.node_id.clone(),
            trigger_kind: "unsupported-feature".to_string(),
            symbol: Some(unsupported.feature_kind.clone()),
            exported_variables: Vec::new(),
            coupling,
            fidelity_grade: LogicHandoffFidelityGrade::Red,
            era_r_status: "unsupported-or-human-redesign".to_string(),
            gap_reason: unsupported.reason.clone(),
            clean_room_instruction: unsupported.suggested_hand_off.clone(),
            provenance: format!(
                "{}:{}:{}",
                unsupported.provenance.source_path,
                unsupported.provenance.line,
                unsupported.provenance.section
            ),
        });
    }

    touchpoints.sort_by(|a, b| a.id.cmp(&b.id));
    touchpoints.dedup_by(|a, b| a.id == b.id);

    let mut behavioral_units = Vec::new();
    let mut era_r_tasks = Vec::new();
    let mut oracle_requirements = Vec::new();
    for touchpoint in &touchpoints {
        let unit_id = format!("behavioral-unit:{}", touchpoint.source_id);
        behavioral_units.push(LogicBehavioralUnitRecord {
            id: unit_id.clone(),
            touchpoint_id: touchpoint.id.clone(),
            coupling: touchpoint.coupling,
            stimuli: stimuli_for(touchpoint),
            observed_outcomes: vec!["not-captured-in-era-o".to_string()],
            oracle_status: "missing".to_string(),
            fidelity_grade: touchpoint.fidelity_grade,
            ported_claim_allowed: false,
            gaps: vec![
                "oracle missing".to_string(),
                "clean-room re-derivation required".to_string(),
                format!("coupling={:?}", touchpoint.coupling),
            ],
        });
        era_r_tasks.push(EraRReDerivationTask {
            id: format!("era-r-task:{}", touchpoint.source_id),
            unit_id: unit_id.clone(),
            task: "capture_oracle_then_reexpress_ouroforge_native_behavior".to_string(),
            reason: touchpoint.gap_reason.clone(),
            target_era: "Era R".to_string(),
            required_evidence: required_evidence_for(touchpoint),
        });
        oracle_requirements.push(LogicOracleRequirement {
            unit_id,
            status: "missing".to_string(),
            required_evidence: vec![
                "captured acceptance oracle".to_string(),
                "2d bit-exact deterministic state hash".to_string(),
                "clean-room intent/provenance record".to_string(),
            ],
            ported_claim_allowed: false,
        });
    }

    let red = touchpoints
        .iter()
        .filter(|record| record.fidelity_grade == LogicHandoffFidelityGrade::Red)
        .count();
    let yellow = touchpoints
        .iter()
        .filter(|record| record.fidelity_grade == LogicHandoffFidelityGrade::Yellow)
        .count();
    let green = touchpoints
        .iter()
        .filter(|record| record.fidelity_grade == LogicHandoffFidelityGrade::Green)
        .count();
    let mut gap_summary: Vec<_> = touchpoints
        .iter()
        .map(|record| format!("{}: {}", record.source_id, record.gap_reason))
        .collect();
    if gap_summary.is_empty() {
        gap_summary.push("no logic touchpoints detected in source-text skeleton".to_string());
    }

    let mut artifact = LogicTouchpointHandoffArtifact {
        schema_version: LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION.to_string(),
        boundary: LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY.to_string(),
        source_project: ir.source.root_label.clone(),
        source_ir_hash,
        touchpoints,
        behavioral_units,
        era_r_tasks,
        oracle_requirements,
        fidelity_report: LogicHandoffFidelityReport {
            green,
            yellow,
            red,
            oracle_rule: "No logic touchpoint is ported/equivalent until Era R re-derives Ouroforge-native behavior and a captured oracle passes; 2D requires bit-exact deterministic state hashes.".to_string(),
            gap_summary,
            clean_room_notice: "Touchpoints are inventory and hand-off records only; source/decompiled code is never copied, translated, or auto-ported.".to_string(),
        },
        state_hash: String::new(),
        claimed_ported_units: Vec::new(),
    };
    artifact.state_hash = handoff_state_hash(&artifact)?;
    validate_logic_touchpoint_handoff(&artifact)?;
    Ok(artifact)
}

pub fn validate_logic_touchpoint_handoff(artifact: &LogicTouchpointHandoffArtifact) -> Result<()> {
    if artifact.schema_version != LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported logic touchpoint handoff schema {}",
            artifact.schema_version
        ));
    }
    if artifact.boundary != LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY {
        return Err(anyhow!("logic touchpoint handoff boundary drifted"));
    }
    if !artifact.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "logic touchpoint handoff cannot claim ported units without passing oracle evidence"
        ));
    }
    if !artifact.state_hash.starts_with("sha256:") || artifact.state_hash.len() != 71 {
        return Err(anyhow!(
            "logic touchpoint handoff requires a sha256 deterministic state hash"
        ));
    }
    if artifact
        .oracle_requirements
        .iter()
        .any(|oracle| oracle.status != "passed" && oracle.ported_claim_allowed)
    {
        return Err(anyhow!(
            "oracle-missing logic unit cannot allow a ported claim"
        ));
    }
    if artifact
        .behavioral_units
        .iter()
        .any(|unit| unit.fidelity_grade == LogicHandoffFidelityGrade::Green)
    {
        return Err(anyhow!(
            "behavior-bearing logic touchpoints cannot be green in Era O"
        ));
    }
    if !artifact.behavioral_units.is_empty() && artifact.fidelity_report.red == 0 {
        return Err(anyhow!(
            "logic touchpoint handoff cannot grade behavior-bearing imports clean"
        ));
    }
    let expected_hash = handoff_state_hash(artifact)?;
    if artifact.state_hash != expected_hash {
        return Err(anyhow!(
            "logic touchpoint handoff state hash does not match canonical artifact"
        ));
    }
    Ok(())
}

fn handoff_state_hash(artifact: &LogicTouchpointHandoffArtifact) -> Result<String> {
    let mut canonical = artifact.clone();
    canonical.state_hash.clear();
    Ok(crate::export_hash::sha256_prefixed(&serde_json::to_vec(
        &canonical,
    )?))
}

fn coupling_for_touchpoint(trigger_kind: &str, symbol: Option<&str>) -> LogicCouplingKind {
    let trigger = trigger_kind.to_ascii_lowercase();
    let symbol = symbol.unwrap_or_default().to_ascii_lowercase();
    if trigger.contains("signal") {
        if symbol.contains("input") || symbol.contains("pressed") || symbol.contains("button") {
            LogicCouplingKind::Input
        } else if symbol.contains("collision") || symbol.contains("body") || symbol.contains("area")
        {
            LogicCouplingKind::Physics
        } else {
            LogicCouplingKind::Signal
        }
    } else if trigger.contains("script") {
        LogicCouplingKind::Script
    } else {
        LogicCouplingKind::UnknownEngineApi
    }
}

fn coupling_for_unsupported(feature_kind: &str) -> LogicCouplingKind {
    let lower = feature_kind.to_ascii_lowercase();
    if lower.contains("particle") || lower.contains("shader") || lower.contains("light") {
        LogicCouplingKind::Rendering
    } else if lower.contains("audio") {
        LogicCouplingKind::Audio
    } else if lower.contains("anim") || lower.contains("tween") {
        LogicCouplingKind::Animation
    } else if lower.contains("body") || lower.contains("collision") || lower.contains("joint") {
        LogicCouplingKind::Physics
    } else if lower.contains("navigation") || lower.contains("scene") {
        LogicCouplingKind::Scene
    } else {
        LogicCouplingKind::UnsupportedEngineFeature
    }
}

fn stimuli_for(touchpoint: &LogicTouchpointRecord) -> Vec<String> {
    match touchpoint.coupling {
        LogicCouplingKind::Input => vec!["source input or UI event".to_string()],
        LogicCouplingKind::Physics => vec!["source physics/collision event".to_string()],
        LogicCouplingKind::Signal => vec!["source signal/event emission".to_string()],
        LogicCouplingKind::Script => vec!["source script lifecycle or callback".to_string()],
        LogicCouplingKind::Animation => vec!["source animation/timeline event".to_string()],
        LogicCouplingKind::Audio => vec!["source audio event".to_string()],
        LogicCouplingKind::Rendering => vec!["source rendering/VFX event".to_string()],
        LogicCouplingKind::Scene => vec!["source scene transition or scene-tree event".to_string()],
        LogicCouplingKind::UnsupportedEngineFeature | LogicCouplingKind::UnknownEngineApi => {
            vec!["unsupported source-engine behavior".to_string()]
        }
    }
}

fn required_evidence_for(touchpoint: &LogicTouchpointRecord) -> Vec<String> {
    let mut evidence = vec![
        "captured human intent or observed behavior trace".to_string(),
        "Ouroforge-native re-expression artifact".to_string(),
        "2d bit-exact deterministic state hash".to_string(),
    ];
    if touchpoint.coupling == LogicCouplingKind::Rendering {
        evidence.push("perceptual render evidence is secondary only".to_string());
    }
    evidence
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

#[allow(dead_code)]
fn _adapter_grade_floor(grade: AdapterFidelityGrade) -> LogicHandoffFidelityGrade {
    match grade {
        AdapterFidelityGrade::Green => LogicHandoffFidelityGrade::Yellow,
        AdapterFidelityGrade::Yellow => LogicHandoffFidelityGrade::Yellow,
        AdapterFidelityGrade::Red => LogicHandoffFidelityGrade::Red,
    }
}
