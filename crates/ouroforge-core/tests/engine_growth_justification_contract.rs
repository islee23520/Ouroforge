//! Engine-Growth Demand Justification Gate v1 contract (#1495).

use ouroforge_core::engine_growth_justification::{
    EngineGrowthEvaluationStatus, EngineGrowthFindingKind, EngineGrowthFindingSeverity,
    EngineGrowthJustificationArtifact, ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(repo_root().join(format!(
        "examples/engine-growth-justification-v1/{name}.json"
    )))
    .unwrap_or_else(|err| panic!("engine growth fixture {name} readable: {err}"))
}

#[test]
fn justified_engine_growth_links_capability_to_satisfied_rung_gate() {
    let artifact = EngineGrowthJustificationArtifact::from_json_str(&fixture("justified"))
        .expect("justified artifact parses");

    assert_eq!(
        artifact.schema_version,
        ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION
    );

    let evaluation = artifact.evaluate();
    assert_eq!(evaluation.status, EngineGrowthEvaluationStatus::Justified);
    assert!(evaluation.findings.is_empty());
    assert_eq!(evaluation.capabilities.len(), 1);
    assert_eq!(
        evaluation.capabilities[0].capability_id,
        "engine.capability.plan-pruner"
    );
    assert_eq!(
        evaluation.capabilities[0].rung_gate_ref.as_deref(),
        Some("rung-gate.authoring-scale.v1")
    );
    assert!(!evaluation.auto_block);
    assert!(evaluation
        .governance_note
        .contains("auditable governance signal"));
}

#[test]
fn capability_without_rung_gate_is_unjustified_finding_not_auto_block() {
    let artifact = EngineGrowthJustificationArtifact::from_json_str(&fixture("unjustified"))
        .expect("unjustified artifact still parses");

    let evaluation = artifact.evaluate();
    assert_eq!(evaluation.status, EngineGrowthEvaluationStatus::Unjustified);
    assert!(!evaluation.auto_block);
    assert_eq!(evaluation.findings.len(), 1);

    let finding = &evaluation.findings[0];
    assert_eq!(finding.kind, EngineGrowthFindingKind::MissingRungGate);
    assert_eq!(finding.severity, EngineGrowthFindingSeverity::Warning);
    assert_eq!(finding.capability_id, "engine.capability.runtime-expander");
    assert_eq!(finding.rung_gate_ref, None);
    assert!(finding.reason.contains("rung gate"));
}

#[test]
fn unsatisfied_prerequisite_is_flagged_as_unjustified() {
    let artifact =
        EngineGrowthJustificationArtifact::from_json_str(&fixture("missing-prerequisite"))
            .expect("missing prerequisite artifact parses");

    let evaluation = artifact.evaluate();
    assert_eq!(evaluation.status, EngineGrowthEvaluationStatus::Unjustified);
    assert!(!evaluation.auto_block);
    assert_eq!(evaluation.findings.len(), 1);

    let finding = &evaluation.findings[0];
    assert_eq!(
        finding.kind,
        EngineGrowthFindingKind::UnsatisfiedPrerequisite
    );
    assert_eq!(finding.severity, EngineGrowthFindingSeverity::Warning);
    assert_eq!(
        finding.capability_id,
        "engine.capability.parallel-scheduler"
    );
    assert_eq!(
        finding.rung_gate_ref.as_deref(),
        Some("rung-gate.parallelism.v1")
    );
    assert!(finding
        .reason
        .contains("engine.prerequisite.scheduler-safety"));
}

#[test]
fn stale_refs_are_explicit_unjustified_findings() {
    let artifact = EngineGrowthJustificationArtifact::from_json_str(&fixture("stale-refs"))
        .expect("stale refs artifact is structurally valid");

    let evaluation = artifact.evaluate();
    assert_eq!(evaluation.status, EngineGrowthEvaluationStatus::Unjustified);
    assert!(!evaluation.auto_block);
    assert_eq!(evaluation.findings.len(), 2);

    assert!(evaluation.findings.iter().any(|finding| {
        finding.kind == EngineGrowthFindingKind::StaleRungGateRef
            && finding.severity == EngineGrowthFindingSeverity::Error
            && finding.capability_id == "engine.capability.search-index"
            && finding.rung_gate_ref.as_deref() == Some("rung-gate.deleted.v1")
    }));
    assert!(evaluation.findings.iter().any(|finding| {
        finding.kind == EngineGrowthFindingKind::StalePrerequisiteRef
            && finding.severity == EngineGrowthFindingSeverity::Error
            && finding.reason.contains("engine.prerequisite.deleted")
    }));
}

#[test]
fn malformed_artifact_fails_before_governance_evaluation() {
    let mut value: serde_json::Value = serde_json::from_str(&fixture("justified")).unwrap();
    value["schemaVersion"] = serde_json::json!("engine-growth-justification-v0");

    let err = EngineGrowthJustificationArtifact::from_json_str(&value.to_string())
        .expect_err("schema mismatch rejected");
    assert!(err.to_string().contains("schemaVersion"));
}
