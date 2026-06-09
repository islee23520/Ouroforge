//! Scenario Coverage v79 regression suite for #2175 / Era O M90.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::godot_2d_adapter_ir::{
    parse_godot_2d_project, parse_godot_2d_source_files, GodotSourceFile,
};
use ouroforge_core::ir_mapping_fidelity_classifier::{
    map_godot_ir_to_ouroforge, validate_mapping_artifact, MappingFidelityGrade,
    IR_MAPPING_BOUNDARY, IR_MAPPING_SCHEMA_VERSION,
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
    read_json("examples/godot-2d-adapter-v1/scenario-coverage-v79/matrix.fixture.json")
}

fn sample_project_root() -> PathBuf {
    repo_root().join("examples/godot-2d-adapter-v1/sample-project")
}

fn source_files_with_player_x(x: i64) -> Vec<GodotSourceFile> {
    vec![
        GodotSourceFile {
            path: "project.godot".to_string(),
            contents: r#"
[input]
move_left={"events":[{"physical_keycode":65}]}
"#
            .to_string(),
        },
        GodotSourceFile {
            path: "main.tscn".to_string(),
            contents: format!(
                r#"
[gd_scene load_steps=2 format=3 uid="uid://scenario-v79"]

[ext_resource type="Texture2D" path="res://player.png" id="tex_player"]

[node name="Root" type="Node2D"]

[node name="Player" type="Sprite2D" parent="."]
position = Vector2({x}, 20)
texture = ExtResource("tex_player")
script = ExtResource("player_script")

[node name="Hitbox" type="Area2D" parent="Player"]
collision_layer = 1
collision_mask = 1

[node name="Particles" type="GPUParticles2D" parent="."]

[connection signal="pressed" from="Player" to="." method="_on_player_pressed"]
"#
            ),
        },
    ]
}

#[test]
fn v79_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v79-ir-mapping-fidelity-classifier-v1"
    );
    assert_eq!(matrix["coverageVersion"], 79);
    assert_eq!(matrix["issueRef"], "#2175");
    assert_eq!(matrix["milestone"], "Era O M90");

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v79.ir-to-native-mapping-report",
        "v79.lossy-import-not-clean",
        "v79.no-auto-port-without-oracle",
        "v79.deterministic-state-hash-break-fails",
        "v79.coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v79 row {required}");
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
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v79_ir_to_native_mapping_report_is_honest_and_replayable() {
    let ir = parse_godot_2d_project(sample_project_root()).expect("Godot fixture parses");
    let artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");
    validate_mapping_artifact(&artifact).expect("fixture mapping validates");

    assert_eq!(artifact.schema_version, IR_MAPPING_SCHEMA_VERSION);
    assert_eq!(artifact.boundary, IR_MAPPING_BOUNDARY);
    assert!(artifact.state_hash.starts_with("sha256:"));
    assert!(artifact.claimed_ported_units.is_empty());
    assert!(artifact.fidelity_report.green > 0);
    assert!(artifact.fidelity_report.yellow > 0);
    assert!(artifact.fidelity_report.red > 0);
    assert!(artifact
        .fidelity_report
        .oracle_rule
        .contains("No mapped unit is ported"));
    assert!(artifact
        .mapping_records
        .iter()
        .any(|record| record.reason.contains("Declarative skeleton")));
    assert!(artifact
        .oracle_records
        .iter()
        .all(|oracle| !oracle.ported_claim_allowed));
}

#[test]
fn v79_lossy_import_and_behavioral_gaps_cannot_be_laundered_green() {
    let ir = parse_godot_2d_source_files("v79-lossy", source_files_with_player_x(10))
        .expect("lossy fixture parses with explicit gaps");
    let mut artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");

    assert!(artifact.fidelity_report.yellow > 0);
    assert!(artifact.fidelity_report.red > 0);
    assert!(artifact
        .behavioral_units
        .iter()
        .all(|unit| unit.fidelity_grade == MappingFidelityGrade::Red));
    assert!(artifact
        .behavioral_units
        .iter()
        .any(|unit| unit.era_r_status.contains("clean-room")
            || unit.era_r_status.contains("unsupported")));
    assert!(artifact
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("script") || gap.contains("Unsupported")));

    artifact.fidelity_report.red = 0;
    let err = validate_mapping_artifact(&artifact)
        .expect_err("lossy or behavior-bearing import claimed clean must fail");
    assert!(
        err.to_string().contains("behavioral or unsupported units"),
        "{err}"
    );
}

#[test]
fn v79_no_auto_port_without_oracle_or_auto_translation_claim() {
    let ir = parse_godot_2d_project(sample_project_root()).expect("Godot fixture parses");
    let mut artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");
    assert!(artifact.claimed_ported_units.is_empty());
    assert!(artifact
        .oracle_records
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));

    artifact
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_mapping_artifact(&artifact).expect_err("ungated port claim must fail");
    assert!(err.to_string().contains("cannot claim ported units"));
}

#[test]
fn v79_deterministic_state_hash_is_stable_and_tamper_break_fails() {
    let first_ir = parse_godot_2d_source_files("v79-determinism", source_files_with_player_x(10))
        .expect("first parses");
    let second_ir = parse_godot_2d_source_files("v79-determinism", source_files_with_player_x(10))
        .expect("second parses");
    let changed_ir = parse_godot_2d_source_files("v79-determinism", source_files_with_player_x(11))
        .expect("changed parses");

    let first = map_godot_ir_to_ouroforge(&first_ir).expect("first maps");
    let second = map_godot_ir_to_ouroforge(&second_ir).expect("second maps");
    let changed = map_godot_ir_to_ouroforge(&changed_ir).expect("changed maps");

    assert_eq!(first, second);
    assert_eq!(first.state_hash, second.state_hash);
    assert_ne!(first.state_hash, changed.state_hash);

    let mut tampered = first.clone();
    tampered.scenes[0].entities[0].name.push_str("-tampered");
    let err = validate_mapping_artifact(&tampered).expect_err("stale state hash must fail");
    assert!(
        err.to_string().contains("state hash") && err.to_string().contains("canonical"),
        "{err}"
    );
}

#[test]
fn v79_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v79-ir-mapping-fidelity-classifier.md")
        .to_ascii_lowercase();
    for required in [
        "scenario coverage v79",
        "ir to ouroforge mapping",
        "fidelity classifier",
        "one-way on-ramp",
        "source-project/open-text",
        "clean-room",
        "no auto-port",
        "passing oracle",
        "lossy import",
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
