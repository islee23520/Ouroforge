//! Fixture-backed demo for #2226 / Era R M109 tacit oracle capture.

use std::path::{Path, PathBuf};

use ouroforge_core::legacy_logic_ingestion::{
    analyze_legacy_logic, LegacyLogicSource, LegacyLogicSourceKind,
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

fn demo_units() -> Vec<ouroforge_core::legacy_logic_ingestion::BehavioralUnitRecord> {
    let source = LegacyLogicSource {
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
    };
    analyze_legacy_logic(&[source]).unwrap().behavioral_units
}

#[test]
fn tacit_oracle_capture_demo_records_honest_fidelity_summary() {
    let fixture = read_json("examples/tacit-oracle-capture-demo-v1/summary.fixture.json");
    assert_eq!(fixture["schemaVersion"], "tacit.oracle.capture.demo.v1");
    assert_eq!(fixture["issueRef"], "#2226");
    assert_eq!(fixture["summary"]["cleanRoomSourceOnly"], true);
    assert_eq!(fixture["summary"]["studioTrustedWriteAuthority"], false);
    assert_eq!(fixture["summary"]["foreignRuntimeBridgeAllowed"], false);

    let units = demo_units();
    assert!(
        units.len() >= 2,
        "demo source should yield captured + oracle-less units"
    );
    let captured_unit = units[0].id.clone();

    let report = capture_tacit_oracles(
        &units,
        &[InterrogationQuestion {
            id: "q.demo.jump.intent".to_string(),
            unit_id: captured_unit.clone(),
            prompt: "What player-visible result should the source Jump behavior produce?".to_string(),
            resolves: "jump outcome, timing, and repeated-input invariant".to_string(),
            required: true,
        }],
        &[TacitAnswerRecord {
            question_id: "q.demo.jump.intent".to_string(),
            author: "authorized-designer".to_string(),
            answer: "Jump must apply exactly one upward velocity change on the same fixed tick; it never double-applies during one frame.".to_string(),
            confidence: AnswerConfidence::High,
            provenance_refs: vec!["source-notes/player-controller-intent.md".to_string()],
        }],
        &[ObservedBehaviorTrace {
            id: "trace.demo.jump.001".to_string(),
            unit_id: captured_unit.clone(),
            stimulus: "frame 10: press Jump".to_string(),
            observed_events: vec![
                "jump_input_consumed".to_string(),
                "vertical_velocity_positive".to_string(),
                "jump_count=1".to_string(),
            ],
            state_hash: "fnv64:0f0e0d0c0b0a0908".to_string(),
            source_provenance: "source-run/open-text-observed-trace.json".to_string(),
            secondary_render_digest: None,
        }],
    )
    .unwrap();

    let captured = report
        .sessions
        .iter()
        .filter(|session| session.oracle_status == CapturedOracleStatus::Captured)
        .count();
    let missing = report
        .sessions
        .iter()
        .filter(|session| session.oracle_status == CapturedOracleStatus::Missing)
        .count();
    let port_claims = report
        .sessions
        .iter()
        .filter(|session| session.ported_claim_allowed)
        .count()
        + report
            .oracle_specs
            .iter()
            .filter(|spec| spec.ported_claim_allowed)
            .count();

    assert_eq!(
        captured,
        fixture["summary"]["capturedOracleUnits"].as_u64().unwrap() as usize
    );
    assert_eq!(
        missing,
        fixture["summary"]["oracleMissingUnits"].as_u64().unwrap() as usize
    );
    assert_eq!(port_claims, 0);
    assert_eq!(report.oracle_specs.len(), 1);
    assert_eq!(
        report.oracle_specs[0].primary_state_hash,
        "fnv64:0f0e0d0c0b0a0908"
    );
    assert!(report.fidelity_report.no_oracle_not_ported);
    assert!(report.fidelity_report.clean_room_source_only);
    assert!(report.fidelity_report.deterministic_capture);
    assert!(!report.fidelity_report.studio_trusted_write_authority);
}

#[test]
fn tacit_oracle_capture_demo_docs_preserve_boundaries() {
    let doc = read_text("docs/tacit-oracle-capture-demo-v1.md").to_ascii_lowercase();
    for required in [
        "clean-room",
        "oracle-missing",
        "not ported",
        "state-hash primary",
        "no source physics",
        "does not touch studio",
        "trusted-write",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
