//! Self-audit attribution + acceptance demo tests for #2031 / Era L M69.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    run_self_audit_demo, self_audit_bottleneck_input_from_json_str, SelfAuditAcceptanceStatus,
    SelfAuditAttributionContract, SelfAuditEvidenceDocument, SELF_AUDIT_DEMO_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn contract() -> SelfAuditAttributionContract {
    SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates")
}

fn bottleneck_input() -> ouroforge_core::SelfAuditBottleneckInput {
    self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("bottleneck input validates")
}

fn doc(path: &str) -> SelfAuditEvidenceDocument {
    SelfAuditEvidenceDocument {
        source_ref: path.to_string(),
        document: read_json(path),
    }
}

fn planted_acceptance_evidence() -> Vec<SelfAuditEvidenceDocument> {
    let mut verdict = doc("examples/real-title-dogfood-v1/run/verdict.json");
    verdict.document["status"] = Value::String("fail".to_string());
    let mut provenance = doc("examples/real-title-dogfood-v1/release-provenance.complete.json");
    provenance.document["status"] = Value::String("incomplete".to_string());

    vec![
        verdict,
        provenance,
        doc("examples/real-title-dogfood-v1/demo/friction-summary.fixture.json"),
        doc("examples/real-title-dogfood-v1/demo/audit-trail.fixture.json"),
        doc("examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json"),
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
            document: serde_json::json!({"kind":"existing-ledger-jsonl-anchor"}),
        },
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/journal.md".to_string(),
            document: serde_json::json!({"kind":"existing-journal-anchor"}),
        },
        doc("examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json"),
    ]
}

#[test]
fn demo_outputs_ranked_bottlenecks_and_per_milestone_verdicts_for_planted_defect() {
    let report = run_self_audit_demo(
        &contract(),
        &bottleneck_input(),
        &planted_acceptance_evidence(),
        "planted-four-gates-fail",
    )
    .expect("self-audit demo report");

    assert_eq!(report.schema_version, SELF_AUDIT_DEMO_SCHEMA_VERSION);
    assert_eq!(report.title_id, "era-i-engine-builder-deckbuilder");
    assert_eq!(report.planted_defect_id, "planted-four-gates-fail");
    assert_eq!(
        report.output_summary.top_bottleneck_milestone_id,
        "m68-real-title-run"
    );
    assert_eq!(report.output_summary.top_bottleneck_issue_ref, "#2025");
    assert_eq!(report.acceptance_audit.milestone_verdicts.len(), 3);
    assert_eq!(
        report.output_summary.acceptance_status,
        SelfAuditAcceptanceStatus::Regressed
    );
    assert!(report
        .output_summary
        .regressed_milestone_ids
        .contains(&"m68-real-title-run".to_string()));
    assert!(report
        .bottleneck_attribution
        .ranked_bottlenecks
        .first()
        .expect("top bottleneck")
        .signal_ids
        .contains(&"planted-four-gates-fail".to_string()));

    let json = serde_json::to_value(&report).expect("serializes");
    assert!(json["bottleneckAttribution"]["rankedBottlenecks"].is_array());
    assert!(json["acceptanceAudit"]["milestoneVerdicts"].is_array());
}

#[test]
fn demo_preserves_autonomy_and_existing_pipeline_boundaries() {
    let report = run_self_audit_demo(
        &contract(),
        &bottleneck_input(),
        &planted_acceptance_evidence(),
        "planted-four-gates-fail",
    )
    .expect("self-audit demo report");

    assert!(!report.output_summary.human_input_required);
    assert!(!report.output_summary.high_risk_auto_applied);
    assert!(!report.output_summary.new_verification_engine);
    assert!(!report.output_summary.new_data_plane);
    assert_eq!(
        report.autonomous_loop_stages,
        [
            "detect",
            "explain",
            "trace",
            "attribute",
            "propose",
            "re-verify",
            "apply-or-queue"
        ]
    );

    let lower = report.boundary.to_ascii_lowercase();
    for required in [
        "openchrome",
        "scenario verdicts",
        "four gates",
        "design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "evolve",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "zero human input",
        "never auto-applied",
        "human ring 2",
    ] {
        assert!(lower.contains(required), "missing boundary {required}");
    }
}
