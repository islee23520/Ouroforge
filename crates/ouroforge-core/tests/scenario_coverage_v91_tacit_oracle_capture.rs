//! Scenario Coverage v91 regression suite for #2227 / Era R M109.

use std::path::{Path, PathBuf};

use ouroforge_core::legacy_logic_ingestion::{
    analyze_legacy_logic, EraRHandoffState, FidelityGrade, LegacyLogicSource, LegacyLogicSourceKind,
};
use ouroforge_core::tacit_oracle_capture::{
    capture_tacit_oracles, AnswerConfidence, CapturedOracleStatus, InterrogationQuestion,
    ObservedBehaviorTrace, TacitAnswerRecord,
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

fn units() -> Vec<ouroforge_core::legacy_logic_ingestion::BehavioralUnitRecord> {
    analyze_legacy_logic(&[LegacyLogicSource {
        path: "Assets/Scripts/PlayerController.cs".to_string(),
        kind: LegacyLogicSourceKind::CSharpSource,
        source_only_attestation: true,
        text: r#"
using UnityEngine;
using UnityEngine.SceneManagement;
public class PlayerController : MonoBehaviour {
  void Update() {
    if (Input.GetButtonDown("Jump")) { body.AddForce(Vector2.up); }
  }
  void OnTriggerEnter2D(Collider2D other) {
    if (other.name == "Exit") { SceneManager.LoadScene("Win"); }
  }
}
"#
        .to_string(),
    }])
    .unwrap()
    .behavioral_units
}

fn question(unit_id: &str) -> InterrogationQuestion {
    InterrogationQuestion {
        id: "q.v91.intent".to_string(),
        unit_id: unit_id.to_string(),
        prompt: "What observable outcome should this unit produce?".to_string(),
        resolves: "observable outcome and deterministic oracle tolerance".to_string(),
        required: true,
    }
}

fn answer(provenance: &str) -> TacitAnswerRecord {
    TacitAnswerRecord {
        question_id: "q.v91.intent".to_string(),
        author: "authorized-designer".to_string(),
        answer: "The action must produce one deterministic state transition on the same fixed tick and never double-apply during one frame.".to_string(),
        confidence: AnswerConfidence::High,
        provenance_refs: vec![provenance.to_string()],
    }
}

fn trace(unit_id: &str, hash: &str) -> ObservedBehaviorTrace {
    ObservedBehaviorTrace {
        id: "trace.v91.001".to_string(),
        unit_id: unit_id.to_string(),
        stimulus: "frame 10: perform source-independent action".to_string(),
        observed_events: vec![
            "action_consumed".to_string(),
            "state_transition=1".to_string(),
        ],
        state_hash: hash.to_string(),
        source_provenance: "source-run/open-text-trace.json".to_string(),
        secondary_render_digest: None,
    }
}

#[test]
fn v91_matrix_records_rows_and_boundaries() {
    let matrix = read_json(
        "examples/tacit-oracle-capture-demo-v1/scenario-coverage-v91/matrix.fixture.json",
    );
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v91-tacit-oracle-capture-v1"
    );
    assert_eq!(matrix["coverageVersion"], 91);
    assert_eq!(matrix["issueRef"], "#2227");

    let rows = matrix["rows"].as_array().unwrap();
    for required in [
        "v91.captured-oracle-is-reexpress-ready-not-ported",
        "v91.no-auto-port-without-oracle",
        "v91.blocked-provenance-not-green",
        "v91.deterministic-state-hash-break-fails",
        "v91.rust-owned-no-studio-trusted-write",
    ] {
        assert!(
            rows.iter().any(|row| row["id"] == required),
            "missing {required}"
        );
    }
    for row in rows {
        assert_eq!(row["status"], "pass");
        assert!(repo_root()
            .join(row["evidenceRef"].as_str().unwrap())
            .exists());
    }
    let invariants = &matrix["invariants"];
    assert_eq!(invariants["autoPortWithoutOracleAllowed"], false);
    assert_eq!(invariants["lossyOrIncompleteOracleMayGradeGreen"], false);
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
}

#[test]
fn v91_captured_oracle_is_reexpress_ready_not_ported() {
    let units = units();
    let unit_id = units[0].id.clone();
    let report = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("source-notes/intent.md")],
        &[trace(&unit_id, "fnv64:1111111111111111")],
    )
    .unwrap();
    let captured = report
        .sessions
        .iter()
        .find(|s| s.unit_id == unit_id)
        .unwrap();
    assert_eq!(captured.oracle_status, CapturedOracleStatus::Captured);
    assert_eq!(captured.fidelity_grade, FidelityGrade::Green);
    assert_eq!(captured.handoff_state, EraRHandoffState::Reexpress);
    assert!(!captured.ported_claim_allowed);
    assert_eq!(report.oracle_specs.len(), 1);
    assert!(!report.oracle_specs[0].ported_claim_allowed);
}

#[test]
fn v91_no_auto_port_without_oracle_or_partial_trace() {
    let units = units();
    let unit_id = units[0].id.clone();
    let report = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("source-notes/intent.md")],
        &[],
    )
    .unwrap();
    let partial = report
        .sessions
        .iter()
        .find(|s| s.unit_id == unit_id)
        .unwrap();
    assert_eq!(partial.oracle_status, CapturedOracleStatus::Missing);
    assert_eq!(partial.fidelity_grade, FidelityGrade::Yellow);
    assert_eq!(partial.handoff_state, EraRHandoffState::CaptureOracle);
    assert!(!partial.ported_claim_allowed);
    assert_eq!(report.fidelity_report.green_count, 0);
    assert!(report
        .re_derivation_tasks
        .iter()
        .any(|task| task.task == "capture_or_repair_oracle"));
}

#[test]
fn v91_blocked_provenance_is_red_not_laundered_green() {
    let units = units();
    let unit_id = units[0].id.clone();
    let report = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("decompiled/ilspy-dump.cs")],
        &[trace(&unit_id, "fnv64:2222222222222222")],
    )
    .unwrap();
    let blocked = report
        .sessions
        .iter()
        .find(|s| s.unit_id == unit_id)
        .unwrap();
    assert_eq!(blocked.oracle_status, CapturedOracleStatus::Blocked);
    assert_eq!(blocked.fidelity_grade, FidelityGrade::Red);
    assert_eq!(blocked.handoff_state, EraRHandoffState::RejectOrDefer);
    assert!(report.oracle_specs.is_empty());
    assert_eq!(report.fidelity_report.green_count, 0);
    assert!(!report.fidelity_report.blocked_oracle_refs.is_empty());
}

#[test]
fn v91_deterministic_digest_is_stable_and_state_hash_break_changes_it() {
    let units = units();
    let unit_id = units[0].id.clone();
    let first = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("source-notes/intent.md")],
        &[trace(&unit_id, "fnv64:aaaaaaaaaaaaaaaa")],
    )
    .unwrap();
    let same = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("source-notes/intent.md")],
        &[trace(&unit_id, "fnv64:aaaaaaaaaaaaaaaa")],
    )
    .unwrap();
    let changed = capture_tacit_oracles(
        &units,
        &[question(&unit_id)],
        &[answer("source-notes/intent.md")],
        &[trace(&unit_id, "fnv64:bbbbbbbbbbbbbbbb")],
    )
    .unwrap();
    assert_eq!(first.deterministic_digest, same.deterministic_digest);
    assert_ne!(first.deterministic_digest, changed.deterministic_digest);
}

#[test]
fn v91_docs_record_rust_studio_and_anchor_boundaries() {
    let doc = read_text("docs/scenario-coverage-v91-tacit-oracle-capture.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v91",
        "one-way on-ramp",
        "source-project/open-text",
        "no auto-port",
        "yellow/red",
        "state-hash primary",
        "rust remains the data plane",
        "no trusted-write",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
