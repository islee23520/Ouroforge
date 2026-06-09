//! Bottleneck attribution tests for #2029 / Era L M69.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    attribute_self_audit_bottlenecks, self_audit_bottleneck_input_from_json_str,
    SelfAuditAttributionContract, SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn contract() -> SelfAuditAttributionContract {
    SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates")
}

#[test]
fn planted_failure_is_attributed_to_correct_milestone_with_evidence() {
    let input = self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("input validates");
    let report = attribute_self_audit_bottlenecks(&contract(), &input).expect("report builds");

    assert_eq!(
        report.schema_version,
        SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION
    );
    let top = report
        .ranked_bottlenecks
        .first()
        .expect("ranked bottleneck");
    assert_eq!(top.rank, 1);
    assert_eq!(top.milestone_id, "m68-real-title-run");
    assert_eq!(top.issue_ref, "#2025");
    assert_eq!(top.gate_kind, "four-gates");
    assert_eq!(top.loop_stage, "attribute");
    assert!(top
        .signal_ids
        .contains(&"planted-four-gates-fail".to_string()));
    assert!(top
        .signal_ids
        .contains(&"planted-provenance-incomplete".to_string()));
    assert!(top
        .evidence_refs
        .iter()
        .any(|reference| reference.ends_with("run/verdict.json")));
    assert!(top
        .attribution_refs
        .iter()
        .any(|reference| reference.contains("loop-coverage-attribution")));
    assert!(report.unattributed_signals.is_empty());
}

#[test]
fn bottlenecks_are_ranked_deterministically_and_passes_are_ignored() {
    let input = self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("input validates");
    let first = attribute_self_audit_bottlenecks(&contract(), &input).expect("first report");
    let second = attribute_self_audit_bottlenecks(&contract(), &input).expect("second report");
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(&second).unwrap()
    );

    assert!(first.ranked_bottlenecks.len() >= 2);
    assert!(first.ranked_bottlenecks[0].score > first.ranked_bottlenecks[1].score);
    let all_signals: BTreeSet<_> = first
        .ranked_bottlenecks
        .iter()
        .flat_map(|rank| rank.signal_ids.iter().map(String::as_str))
        .collect();
    assert!(!all_signals.contains("coverage-v60-pass"));
}

#[test]
fn input_reads_existing_artifacts_not_a_new_store_or_verifier() {
    let fixture: Value = serde_json::from_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("json fixture");
    let refs = fixture["evidenceRefs"].as_array().expect("refs");
    let joined = refs
        .iter()
        .map(|value| value.as_str().expect("string ref"))
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for required in ["ledger.jsonl", "journal.md", "verdict", "loop-coverage"] {
        assert!(
            joined.contains(required),
            "missing existing artifact {required}"
        );
    }
    for value in refs {
        let reference = value.as_str().expect("string ref");
        assert!(
            repo_root().join(reference).is_file(),
            "missing evidence ref {reference}"
        );
    }
    let boundary = fixture["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for forbidden in ["no new verification engine", "no new data plane"] {
        assert!(boundary.contains(forbidden), "missing {forbidden}");
    }
}

#[test]
fn autonomy_and_high_risk_boundaries_remain_machine_checked() {
    let input = self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("input validates");
    let report = attribute_self_audit_bottlenecks(&contract(), &input).expect("report builds");
    let lower = report.boundary.to_ascii_lowercase();
    for required in [
        "no human input",
        "source-apply",
        "trust-gradient",
        "never auto-applied",
        "human ring 2",
    ] {
        assert!(lower.contains(required), "missing boundary {required}");
    }
}
