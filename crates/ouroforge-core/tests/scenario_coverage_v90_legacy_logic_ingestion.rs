//! Scenario Coverage v90 regression suite for #2223 / Era R M108.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::legacy_logic_ingestion::{
    analyze_legacy_logic, EngineCouplingKind, FidelityGrade, LegacyLogicSource,
    LegacyLogicSourceKind, LegacyLogicSourceStatus, OracleStatus,
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

fn matrix() -> Value {
    read_json("examples/legacy-logic-ingestion-v1/scenario-coverage-v90/matrix.fixture.json")
}

fn source_text(speed: &str) -> String {
    format!(
        r#"
using UnityEngine;
using UnityEngine.SceneManagement;
public class PlayerController : MonoBehaviour {{
  public Rigidbody2D body;
  void Update() {{
    var x = Input.GetAxis("Horizontal");
    body.velocity = new Vector2(x * {speed}, body.velocity.y);
  }}
  void OnTriggerEnter2D(Collider2D other) {{
    if (other.name == "Exit") {{ SceneManager.LoadScene("Win"); }}
  }}
}}
"#
    )
}

fn source_with_speed(speed: &str) -> LegacyLogicSource {
    LegacyLogicSource {
        path: "Assets/Scripts/PlayerController.cs".to_string(),
        kind: LegacyLogicSourceKind::CSharpSource,
        source_only_attestation: true,
        text: source_text(speed),
    }
}

#[test]
fn v90_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v90-legacy-logic-ingestion-v1"
    );
    assert_eq!(matrix["coverageVersion"], 90);
    assert_eq!(matrix["issueRef"], "#2223");
    assert_eq!(matrix["milestone"], "Era R M108");

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v90.accepted-source-catalogs-units-and-couplings",
        "v90.lossy-import-not-green",
        "v90.no-auto-port-without-oracle",
        "v90.deterministic-state-hash-break-fails",
        "v90.decompiled-source-rejected",
        "v90.coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v90 row {required}");
    }
    for row in rows {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidence ref");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }

    let invariants = &matrix["invariants"];
    assert_eq!(invariants["oneWayOnRamp"], true);
    assert_eq!(invariants["sourceProjectOpenTextOnly"], true);
    assert_eq!(invariants["cleanRoomReDerivation"], true);
    assert_eq!(invariants["autoPortWithoutOracleAllowed"], false);
    assert_eq!(invariants["lossyImportMayGradeGreen"], false);
    assert_eq!(invariants["deterministicDigestRequired"], true);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["decompiledSourceAllowed"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v90_accepted_source_catalogs_units_and_engine_couplings_read_only() {
    let analysis = analyze_legacy_logic(&[source_with_speed("4f")]).expect("analysis");
    assert!(analysis.boundary.contains("one-way"));
    assert!(analysis.boundary.contains("clean-room"));
    assert!(analysis
        .source_reports
        .iter()
        .all(|report| report.status == LegacyLogicSourceStatus::Accepted));
    assert!(analysis
        .ir_nodes
        .iter()
        .any(|node| node.name == "PlayerController"));
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
    assert_eq!(
        analysis.re_derivation_tasks.len(),
        analysis.behavioral_units.len()
    );
    assert!(analysis.fidelity_report.clean_room_source_only);
    assert!(analysis.fidelity_report.deterministic_analysis);
}

#[test]
fn v90_lossy_import_and_degraded_signature_are_not_green() {
    let il2cpp = LegacyLogicSource {
        path: "Il2Cpp/signatures.txt".to_string(),
        kind: LegacyLogicSourceKind::Il2CppSignatureDump,
        source_only_attestation: true,
        text: "Game.Enemy::FixedUpdate()\nGame.Enemy::OnCollisionEnter(UnityEngine.Collider)"
            .to_string(),
    };
    let analysis = analyze_legacy_logic(&[il2cpp]).expect("analysis");
    assert!(analysis
        .source_reports
        .iter()
        .any(|report| report.status == LegacyLogicSourceStatus::DegradedFallback));
    assert_eq!(analysis.fidelity_report.green_count, 0);
    assert!(analysis.fidelity_report.red_count >= 1);
    assert!(analysis
        .behavioral_units
        .iter()
        .all(|unit| unit.fidelity_grade != FidelityGrade::Green));
    assert!(analysis
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("oracle missing") || gap.contains("source physics")));
}

#[test]
fn v90_no_auto_port_without_oracle_or_translation_claim() {
    let analysis = analyze_legacy_logic(&[source_with_speed("4f")]).expect("analysis");
    assert!(analysis.fidelity_report.no_oracle_not_ported);
    for unit in &analysis.behavioral_units {
        assert_eq!(unit.oracle_status, OracleStatus::Missing);
        assert!(
            !unit.ported_claim_allowed,
            "unit falsely allowed port claim: {unit:#?}"
        );
        assert!(unit.gaps.iter().any(|gap| gap.contains("oracle missing")));
        assert!(unit.name.starts_with("Re-derive"));
    }
}

#[test]
fn v90_deterministic_state_hash_is_stable_and_catches_behavior_drift() {
    let first = analyze_legacy_logic(&[source_with_speed("4f")]).expect("analysis");
    let reordered_same = analyze_legacy_logic(&[source_with_speed("4f")]).expect("analysis");
    let changed = analyze_legacy_logic(&[source_with_speed("8f")]).expect("analysis");

    assert_eq!(
        first.deterministic_digest,
        reordered_same.deterministic_digest
    );
    assert_eq!(first.ir_nodes, reordered_same.ir_nodes);
    assert_ne!(
        first.deterministic_digest, changed.deterministic_digest,
        "source behavior change must alter the deterministic digest/state-hash"
    );
}

#[test]
fn v90_decompiled_or_unattested_source_is_rejected_red() {
    let rejected = LegacyLogicSource {
        path: "Assets/Scripts/Dump.cs".to_string(),
        kind: LegacyLogicSourceKind::CSharpSource,
        source_only_attestation: false,
        text: "// Decompiled with ILSpy\npublic class Dump { void Update() { UnityEngine.Debug.Log(1); } }".to_string(),
    };
    let analysis = analyze_legacy_logic(&[rejected]).expect("analysis");
    assert_eq!(
        analysis.source_reports[0].status,
        LegacyLogicSourceStatus::Rejected
    );
    assert!(analysis.ir_nodes.is_empty());
    assert!(analysis.behavioral_units.is_empty());
    assert_eq!(analysis.fidelity_report.green_count, 0);
    assert!(analysis.fidelity_report.red_count >= 1);
    assert!(analysis
        .fidelity_report
        .unsupported_or_blocked
        .iter()
        .any(|gap| gap.contains("clean-room") || gap.contains("source-only")));
}

#[test]
fn v90_docs_record_coverage_and_guardrails() {
    let doc =
        read_text("docs/scenario-coverage-v90-legacy-logic-ingestion.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v90",
        "semantic re-derivation",
        "source-owned",
        "read-only",
        "one-way on-ramp",
        "source-project/open-text",
        "decompiled",
        "no auto-port",
        "passing oracle",
        "yellow/red",
        "deterministic",
        "state-hash",
        "rust remains the data plane",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
