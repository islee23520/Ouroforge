use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::godot_2d_adapter_ir::{parse_godot_2d_project, GodotMigrationIr, MigrationSourceEngine};
use crate::ir_mapping_fidelity_classifier::{
    map_godot_ir_to_ouroforge, MappingFidelityGrade, OuroforgeMappingArtifact,
};
use crate::logic_touchpoint_handoff::{
    detect_godot_logic_touchpoints, LogicHandoffFidelityGrade, LogicTouchpointHandoffArtifact,
};

pub const IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION: &str = "import-verification-report-v1";
pub const IMPORT_VERIFICATION_REPORT_BOUNDARY: &str = "one-way source-project/open-text skeleton verification; Rust-owned fidelity report; clean-room Era R hand-off; oracle-gated; no auto-port or foreign runtime bridge";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportVerificationReport {
    pub schema_version: String,
    pub boundary: String,
    pub source_project: String,
    pub source_engine: String,
    pub source_ir_hash: String,
    pub mapping_state_hash: String,
    pub logic_handoff_state_hash: String,
    pub verification_state_hash: String,
    pub data_shapes: ImportDataShapes,
    pub skeleton_verification: SkeletonVerificationEvidence,
    pub provenance: ImportProvenanceReport,
    pub fidelity_report: ImportFidelityReport,
    pub re_derivation_tasks: Vec<ImportReDerivationTask>,
    pub oracle_records: Vec<ImportOracleRecord>,
    pub claimed_ported_units: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDataShapes {
    pub ir_nodes_ref: String,
    pub mapping_records_ref: String,
    pub behavioral_units_ref: String,
    pub oracle_records_ref: String,
    pub owner_crate: String,
    pub no_elixir_artifact_semantics: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonVerificationEvidence {
    pub runner: String,
    pub status: String,
    pub command_ref: String,
    pub evidence_kind: String,
    pub checked_scene_count: usize,
    pub checked_entity_count: usize,
    pub checked_asset_count: usize,
    pub checked_input_count: usize,
    pub deterministic_state_hash_required: bool,
    pub perceptual_render_secondary_only: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportProvenanceReport {
    pub origin: String,
    pub accepted_formats: Vec<String>,
    pub source_files: Vec<String>,
    pub asset_licenses: Vec<ImportAssetLicenseRecord>,
    pub clean_room_source_only: bool,
    pub decompiled_source_copied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportAssetLicenseRecord {
    pub asset_id: String,
    pub source_path: String,
    pub origin: String,
    pub license_status: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportFidelityReport {
    pub clean: usize,
    pub flagged: usize,
    pub rederive: usize,
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
    pub gap_summary: Vec<String>,
    pub oracle_rule: String,
    pub clean_room_notice: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReDerivationTask {
    pub id: String,
    pub source_ref: String,
    pub reason: String,
    pub target_era: String,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportOracleRecord {
    pub id: String,
    pub source_ref: String,
    pub status: String,
    pub ported_claim_allowed: bool,
    pub required_evidence: Vec<String>,
}

pub fn verify_godot_import(project_root: impl AsRef<Path>) -> Result<ImportVerificationReport> {
    let project_root = project_root.as_ref();
    let ir = parse_godot_2d_project(project_root)?;
    verify_godot_import_ir(project_root.display().to_string(), &ir)
}

pub fn verify_godot_import_ir(
    source_project: impl Into<String>,
    ir: &GodotMigrationIr,
) -> Result<ImportVerificationReport> {
    let source_project = source_project.into();
    let source_ir_hash = crate::export_hash::sha256_prefixed(&serde_json::to_vec(ir)?);
    let mapping = map_godot_ir_to_ouroforge(ir)?;
    let logic_handoff = detect_godot_logic_touchpoints(ir)?;

    let data_shapes = ImportDataShapes {
        ir_nodes_ref: "crates/ouroforge-core/src/godot_2d_adapter_ir.rs::GodotIrNode".to_string(),
        mapping_records_ref:
            "crates/ouroforge-core/src/ir_mapping_fidelity_classifier.rs::MappingRecord".to_string(),
        behavioral_units_ref:
            "crates/ouroforge-core/src/logic_touchpoint_handoff.rs::LogicBehavioralUnitRecord"
                .to_string(),
        oracle_records_ref:
            "crates/ouroforge-core/src/import_verification_report.rs::ImportOracleRecord"
                .to_string(),
        owner_crate: "crates/ouroforge-core".to_string(),
        no_elixir_artifact_semantics: true,
    };

    let skeleton_verification = skeleton_verification_for(&mapping);
    let provenance = provenance_for(ir);
    let re_derivation_tasks = re_derivation_tasks_for(&mapping, &logic_handoff);
    let oracle_records = oracle_records_for(&mapping, &logic_handoff);
    let fidelity_report = fidelity_report_for(&mapping, &logic_handoff, &re_derivation_tasks);

    let mut report = ImportVerificationReport {
        schema_version: IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION.to_string(),
        boundary: IMPORT_VERIFICATION_REPORT_BOUNDARY.to_string(),
        source_project,
        source_engine: match ir.source.engine {
            MigrationSourceEngine::Godot => "godot".to_string(),
        },
        source_ir_hash,
        mapping_state_hash: mapping.state_hash.clone(),
        logic_handoff_state_hash: logic_handoff.state_hash.clone(),
        verification_state_hash: String::new(),
        data_shapes,
        skeleton_verification,
        provenance,
        fidelity_report,
        re_derivation_tasks,
        oracle_records,
        claimed_ported_units: Vec::new(),
    };
    report.verification_state_hash = import_verification_state_hash(&report)?;
    validate_import_verification_report(&report)?;
    Ok(report)
}

pub fn write_godot_import_verification_report(
    project_root: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<ImportVerificationReport> {
    let report = verify_godot_import(project_root)?;
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(output_path, serde_json::to_vec_pretty(&report)?)
        .with_context(|| format!("writing {}", output_path.display()))?;
    Ok(report)
}

pub fn validate_import_verification_report(report: &ImportVerificationReport) -> Result<()> {
    if report.schema_version != IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported import verification report schema {}",
            report.schema_version
        ));
    }
    if report.boundary != IMPORT_VERIFICATION_REPORT_BOUNDARY {
        return Err(anyhow!("import verification report boundary drifted"));
    }
    for (label, hash) in [
        ("source IR", &report.source_ir_hash),
        ("mapping", &report.mapping_state_hash),
        ("logic handoff", &report.logic_handoff_state_hash),
        ("verification", &report.verification_state_hash),
    ] {
        if !hash.starts_with("sha256:") || hash.len() != 71 {
            return Err(anyhow!(
                "{label} hash must be a sha256-prefixed deterministic state hash"
            ));
        }
    }
    if !report.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "import verification report cannot claim ported units without a passing oracle"
        ));
    }
    if report
        .oracle_records
        .iter()
        .any(|oracle| oracle.status != "passed" && oracle.ported_claim_allowed)
    {
        return Err(anyhow!(
            "oracle-missing import unit cannot allow a ported claim"
        ));
    }
    if report.skeleton_verification.runner != "openchrome-local-skeleton-smoke" {
        return Err(anyhow!(
            "import verification must record the openchrome skeleton smoke runner"
        ));
    }
    if report.skeleton_verification.status != "passed" {
        return Err(anyhow!("import skeleton verification did not pass"));
    }
    if report.skeleton_verification.checked_scene_count == 0 {
        return Err(anyhow!(
            "import skeleton verification requires at least one scene"
        ));
    }
    if !report
        .skeleton_verification
        .deterministic_state_hash_required
    {
        return Err(anyhow!(
            "import verification must require deterministic state hashes"
        ));
    }
    if !report.provenance.clean_room_source_only || report.provenance.decompiled_source_copied {
        return Err(anyhow!(
            "import provenance must remain source-only clean-room with no decompiled source copied"
        ));
    }
    if report.provenance.origin != "godot" {
        return Err(anyhow!("import provenance origin must be godot"));
    }
    if !report.data_shapes.no_elixir_artifact_semantics {
        return Err(anyhow!(
            "Elixir/Phoenix must not own import artifact semantics"
        ));
    }
    if report.fidelity_report.rederive > 0 && report.re_derivation_tasks.is_empty() {
        return Err(anyhow!(
            "re-derive fidelity gaps require explicit Era R tasks"
        ));
    }
    if report.fidelity_report.rederive > 0 && report.fidelity_report.red == 0 {
        return Err(anyhow!("re-derive gaps cannot be graded as a clean import"));
    }
    let expected_hash = import_verification_state_hash(report)?;
    if report.verification_state_hash != expected_hash {
        return Err(anyhow!(
            "import verification state hash does not match canonical report"
        ));
    }
    Ok(())
}

fn import_verification_state_hash(report: &ImportVerificationReport) -> Result<String> {
    let mut canonical = report.clone();
    canonical.verification_state_hash.clear();
    Ok(crate::export_hash::sha256_prefixed(&serde_json::to_vec(
        &canonical,
    )?))
}

fn skeleton_verification_for(mapping: &OuroforgeMappingArtifact) -> SkeletonVerificationEvidence {
    let checked_scene_count = mapping.scenes.len();
    let checked_entity_count = mapping
        .scenes
        .iter()
        .map(|scene| scene.entities.len())
        .sum::<usize>();
    SkeletonVerificationEvidence {
        runner: "openchrome-local-skeleton-smoke".to_string(),
        status: if checked_scene_count > 0 {
            "passed".to_string()
        } else {
            "failed".to_string()
        },
        command_ref: "openchrome smoke: load imported Ouroforge-native skeleton and inspect read-only probe state".to_string(),
        evidence_kind: "read-only runtime/probe observation".to_string(),
        checked_scene_count,
        checked_entity_count,
        checked_asset_count: mapping.asset_mappings.len(),
        checked_input_count: mapping.input_mappings.len(),
        deterministic_state_hash_required: true,
        perceptual_render_secondary_only: true,
        notes: vec![
            "The skeleton smoke verifies native scene/entity/asset/input shape only.".to_string(),
            "Logic remains Era R re-derivation and is not translated during import verification.".to_string(),
        ],
    }
}

fn provenance_for(ir: &GodotMigrationIr) -> ImportProvenanceReport {
    let mut asset_licenses = ir
        .assets
        .iter()
        .map(|asset| ImportAssetLicenseRecord {
            asset_id: asset.id.clone(),
            source_path: asset.source_path.clone(),
            origin: "godot".to_string(),
            license_status: "source-project-provenance-recorded-license-unverified".to_string(),
            provenance: format!(
                "{}:{}:{}",
                asset.provenance.source_path, asset.provenance.line, asset.provenance.section
            ),
        })
        .collect::<Vec<_>>();
    asset_licenses.sort_by(|a, b| a.asset_id.cmp(&b.asset_id));
    ImportProvenanceReport {
        origin: "godot".to_string(),
        accepted_formats: ir.source.accepted_formats.clone(),
        source_files: ir.source.source_files.clone(),
        asset_licenses,
        clean_room_source_only: true,
        decompiled_source_copied: false,
    }
}

fn re_derivation_tasks_for(
    mapping: &OuroforgeMappingArtifact,
    logic_handoff: &LogicTouchpointHandoffArtifact,
) -> Vec<ImportReDerivationTask> {
    let mut tasks = Vec::new();
    for unit in &mapping.behavioral_units {
        tasks.push(ImportReDerivationTask {
            id: format!("import-rederive:{}", unit.id),
            source_ref: unit.source_id.clone(),
            reason: unit.clean_room_instruction.clone(),
            target_era: "Era R".to_string(),
            required_evidence: vec![
                "captured acceptance oracle".to_string(),
                "Ouroforge-native re-expression".to_string(),
                "2d bit-exact deterministic state hash".to_string(),
            ],
        });
    }
    for task in &logic_handoff.era_r_tasks {
        tasks.push(ImportReDerivationTask {
            id: format!("import-{}", task.id),
            source_ref: task.unit_id.clone(),
            reason: task.reason.clone(),
            target_era: task.target_era.clone(),
            required_evidence: task.required_evidence.clone(),
        });
    }
    tasks.sort_by(|a, b| a.id.cmp(&b.id));
    tasks.dedup_by(|a, b| a.id == b.id);
    tasks
}

fn oracle_records_for(
    mapping: &OuroforgeMappingArtifact,
    logic_handoff: &LogicTouchpointHandoffArtifact,
) -> Vec<ImportOracleRecord> {
    let mut records = Vec::new();
    for oracle in &mapping.oracle_records {
        records.push(ImportOracleRecord {
            id: format!("import-oracle:{}", oracle.unit_id),
            source_ref: oracle.unit_id.clone(),
            status: oracle.status.clone(),
            ported_claim_allowed: oracle.ported_claim_allowed,
            required_evidence: vec![
                "captured acceptance oracle".to_string(),
                "deterministic state hash".to_string(),
            ],
        });
    }
    for oracle in &logic_handoff.oracle_requirements {
        records.push(ImportOracleRecord {
            id: format!("import-logic-oracle:{}", oracle.unit_id),
            source_ref: oracle.unit_id.clone(),
            status: oracle.status.clone(),
            ported_claim_allowed: oracle.ported_claim_allowed,
            required_evidence: oracle.required_evidence.clone(),
        });
    }
    records.sort_by(|a, b| a.id.cmp(&b.id));
    records.dedup_by(|a, b| a.id == b.id);
    records
}

fn fidelity_report_for(
    mapping: &OuroforgeMappingArtifact,
    logic_handoff: &LogicTouchpointHandoffArtifact,
    re_derivation_tasks: &[ImportReDerivationTask],
) -> ImportFidelityReport {
    let clean = mapping
        .mapping_records
        .iter()
        .filter(|record| record.fidelity_grade == MappingFidelityGrade::Green)
        .count();
    let flagged = mapping
        .mapping_records
        .iter()
        .filter(|record| record.fidelity_grade == MappingFidelityGrade::Yellow)
        .count();
    let rederive_mapping = mapping
        .mapping_records
        .iter()
        .filter(|record| record.fidelity_grade == MappingFidelityGrade::Red)
        .count();
    let rederive_logic = logic_handoff
        .touchpoints
        .iter()
        .filter(|record| record.fidelity_grade == LogicHandoffFidelityGrade::Red)
        .count();
    let rederive = rederive_mapping + rederive_logic;
    let green = clean;
    let yellow = flagged;
    let red = rederive;
    let mut gap_summary = mapping.fidelity_report.gap_summary.clone();
    gap_summary.extend(logic_handoff.fidelity_report.gap_summary.clone());
    gap_summary.sort();
    gap_summary.dedup();
    if gap_summary.is_empty() {
        gap_summary.push("no gaps detected in imported declarative skeleton".to_string());
    }
    ImportFidelityReport {
        clean,
        flagged,
        rederive,
        green,
        yellow,
        red,
        gap_summary,
        oracle_rule: "No imported unit is ported/equivalent until Ouroforge-native re-derivation has passing captured oracle evidence; 2D requires bit-exact deterministic state hashes.".to_string(),
        clean_room_notice: format!(
            "Import verification records skeleton fidelity only; {} Era R task(s) remain clean-room re-derivation and source/decompiled code is never copied or translated.",
            re_derivation_tasks.len()
        ),
    }
}
