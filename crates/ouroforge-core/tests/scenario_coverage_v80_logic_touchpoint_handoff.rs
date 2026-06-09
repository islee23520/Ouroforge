//! Scenario Coverage v80 regression suite for #2178 / Era O M91.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::godot_2d_adapter_ir::{parse_godot_2d_source_files, GodotSourceFile};
use ouroforge_core::logic_touchpoint_handoff::{
    detect_godot_logic_touchpoints, validate_logic_touchpoint_handoff, LogicCouplingKind,
    LogicHandoffFidelityGrade, LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY,
    LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION,
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
    read_json("examples/godot-2d-adapter-v1/scenario-coverage-v80/matrix.fixture.json")
}

fn source_files_with_player_x(x: i64) -> Vec<GodotSourceFile> {
    vec![
        GodotSourceFile {
            path: "project.godot".to_string(),
            contents: r#"
[input]
move_left={"events":[{"physical_keycode":65}]}
fire={"events":[{"physical_keycode":32}]}
"#
            .to_string(),
        },
        GodotSourceFile {
            path: "main.tscn".to_string(),
            contents: format!(
                r#"
[gd_scene load_steps=3 format=3 uid="uid://scenario-v80"]

[ext_resource type="Texture2D" path="res://player.png" id="tex_player"]

[node name="Root" type="Node2D"]

[node name="Player" type="Sprite2D" parent="."]
position = Vector2({x}, 20)
texture = ExtResource("tex_player")
script = ExtResource("player_script")
script_speed = 120
script_jump_enabled = true

[node name="Hitbox" type="Area2D" parent="Player"]
collision_layer = 1
collision_mask = 1

[node name="Particles" type="GPUParticles2D" parent="."]

[connection signal="pressed" from="Player" to="." method="_on_player_pressed"]
[connection signal="body_entered" from="Hitbox" to="Player" method="_on_hitbox_body_entered"]
"#
            ),
        },
    ]
}

#[test]
fn v80_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v80-logic-touchpoint-handoff-v1"
    );
    assert_eq!(matrix["coverageVersion"], 80);
    assert_eq!(matrix["issueRef"], "#2178");
    assert_eq!(matrix["milestone"], "Era O M91");

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v80.logic-touchpoint-handoff-report",
        "v80.lossy-import-not-clean",
        "v80.no-auto-port-without-oracle",
        "v80.deterministic-state-hash-break-fails",
        "v80.coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v80 row {required}");
    }
    for row in rows {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidence ref");
        assert!(
            repo_root().join(evidence_ref).exists(),
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
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["staleStateHashAllowed"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v80_logic_touchpoint_handoff_report_is_honest_and_replayable() {
    let ir = parse_godot_2d_source_files("v80-handoff", source_files_with_player_x(10))
        .expect("Godot text fixture parses");
    let artifact = detect_godot_logic_touchpoints(&ir).expect("handoff succeeds");
    validate_logic_touchpoint_handoff(&artifact).expect("fixture handoff validates");

    assert_eq!(
        artifact.schema_version,
        LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION
    );
    assert_eq!(artifact.boundary, LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY);
    assert!(artifact.state_hash.starts_with("sha256:"));
    assert!(artifact.claimed_ported_units.is_empty());
    assert_eq!(artifact.fidelity_report.green, 0);
    assert_eq!(artifact.fidelity_report.yellow, 0);
    assert!(artifact.fidelity_report.red >= 4, "{artifact:#?}");
    assert_eq!(artifact.behavioral_units.len(), artifact.touchpoints.len());
    assert_eq!(artifact.era_r_tasks.len(), artifact.touchpoints.len());
    assert_eq!(
        artifact.oracle_requirements.len(),
        artifact.touchpoints.len()
    );
    assert!(artifact
        .era_r_tasks
        .iter()
        .all(|task| task.target_era == "Era R"
            && task
                .required_evidence
                .iter()
                .any(|evidence| evidence.contains("deterministic state hash"))));
    assert!(artifact
        .fidelity_report
        .oracle_rule
        .contains("No logic touchpoint is ported"));
    assert!(artifact
        .fidelity_report
        .clean_room_notice
        .contains("never copied, translated, or auto-ported"));
}

#[test]
fn v80_lossy_import_and_behavioral_gaps_cannot_be_laundered_green() {
    let ir = parse_godot_2d_source_files("v80-lossy", source_files_with_player_x(10))
        .expect("lossy fixture parses with explicit gaps");
    let mut artifact = detect_godot_logic_touchpoints(&ir).expect("handoff succeeds");

    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "script-ref" && tp.coupling == LogicCouplingKind::Script));
    assert!(artifact.touchpoints.iter().any(
        |tp| tp.trigger_kind == "signal-connection" && tp.coupling == LogicCouplingKind::Input
    ));
    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "signal-connection"
            && tp.coupling == LogicCouplingKind::Physics));
    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "unsupported-feature"
            && tp.coupling == LogicCouplingKind::Rendering));
    assert!(artifact
        .behavioral_units
        .iter()
        .all(|unit| unit.fidelity_grade == LogicHandoffFidelityGrade::Red
            && unit.oracle_status == "missing"
            && !unit.ported_claim_allowed));
    assert!(artifact
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("inventoried only") || gap.contains("Unsupported")));

    artifact.fidelity_report.red = 0;
    let err = validate_logic_touchpoint_handoff(&artifact)
        .expect_err("lossy or behavior-bearing import claimed clean must fail");
    assert!(err.to_string().contains("cannot grade"), "{err}");
}

#[test]
fn v80_no_auto_port_without_oracle_or_auto_translation_claim() {
    let ir = parse_godot_2d_source_files("v80-no-auto-port", source_files_with_player_x(10))
        .expect("fixture parses");
    let mut artifact = detect_godot_logic_touchpoints(&ir).expect("handoff succeeds");
    assert!(artifact.claimed_ported_units.is_empty());
    assert!(artifact
        .oracle_requirements
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));

    artifact
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_logic_touchpoint_handoff(&artifact).expect_err("ungated port claim fails");
    assert!(err.to_string().contains("cannot claim ported units"));

    let mut artifact = detect_godot_logic_touchpoints(&ir).expect("handoff succeeds");
    artifact.oracle_requirements[0].ported_claim_allowed = true;
    let err = validate_logic_touchpoint_handoff(&artifact).expect_err("oracle bypass fails");
    assert!(err.to_string().contains("oracle-missing"));
}

#[test]
fn v80_deterministic_state_hash_is_stable_and_tamper_break_fails() {
    let first_ir = parse_godot_2d_source_files("v80-determinism", source_files_with_player_x(10))
        .expect("first parses");
    let second_ir = parse_godot_2d_source_files("v80-determinism", source_files_with_player_x(10))
        .expect("second parses");
    let changed_ir = parse_godot_2d_source_files("v80-determinism", source_files_with_player_x(11))
        .expect("changed parses");

    let first = detect_godot_logic_touchpoints(&first_ir).expect("first handoff");
    let second = detect_godot_logic_touchpoints(&second_ir).expect("second handoff");
    let changed = detect_godot_logic_touchpoints(&changed_ir).expect("changed handoff");

    assert_eq!(first, second);
    assert_eq!(first.state_hash, second.state_hash);
    assert_ne!(first.source_ir_hash, changed.source_ir_hash);
    assert_ne!(first.state_hash, changed.state_hash);

    let mut tampered = first.clone();
    tampered.touchpoints[0]
        .clean_room_instruction
        .push_str(" tampered");
    let err = validate_logic_touchpoint_handoff(&tampered).expect_err("stale state hash fails");
    assert!(
        err.to_string().contains("state hash") && err.to_string().contains("canonical"),
        "{err}"
    );
}

#[test]
fn v80_docs_record_coverage_and_guardrails() {
    let doc =
        read_text("docs/scenario-coverage-v80-logic-touchpoint-handoff.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v80",
        "logic touchpoint detection",
        "re-derivation hand-off",
        "one-way on-ramp",
        "source-project/open-text",
        "clean-room",
        "no auto-port",
        "passing oracle",
        "lossy import",
        "red",
        "deterministic",
        "state-hash",
        "rust remains the data plane",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
