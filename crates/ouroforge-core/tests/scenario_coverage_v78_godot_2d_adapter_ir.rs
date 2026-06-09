//! Scenario Coverage v78 regression suite for #2171 / Era O M89.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::godot_2d_adapter_ir::{
    godot_2d_adapter_demo_report, parse_godot_2d_project, parse_godot_2d_source_files,
    validate_godot_2d_adapter_demo_report, FidelityGrade, GodotSourceFile,
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
    read_json("examples/godot-2d-adapter-v1/scenario-coverage-v78/matrix.fixture.json")
}

fn sample_project_root() -> PathBuf {
    repo_root().join("examples/godot-2d-adapter-v1/sample-project")
}

fn hash_ir(ir: &ouroforge_core::godot_2d_adapter_ir::GodotMigrationIr) -> String {
    ouroforge_core::export_hash::sha256_prefixed(
        &serde_json::to_vec(ir).expect("canonical IR serializes"),
    )
}

fn source_files_with_player_x(x: i64) -> Vec<GodotSourceFile> {
    vec![
        GodotSourceFile {
            path: "project.godot".to_string(),
            contents: r#"
[input]
jump={"events":[{"physical_keycode":32}]}
"#
            .to_string(),
        },
        GodotSourceFile {
            path: "main.tscn".to_string(),
            contents: format!(
                r#"
[gd_scene load_steps=2 format=3 uid="uid://scenario-v78"]

[ext_resource type="Texture2D" path="res://player.png" id="tex_player"]

[node name="Root" type="Node2D"]

[node name="Player" type="Sprite2D" parent="."]
position = Vector2({x}, 20)
texture = ExtResource("tex_player")
script = ExtResource("player_script")

[node name="Particles" type="GPUParticles2D" parent="."]

[connection signal="pressed" from="Player" to="." method="_on_player_pressed"]
"#
            ),
        },
        GodotSourceFile {
            path: "player.tres".to_string(),
            contents: r#"
[gd_resource type="Resource" format=3]
"#
            .to_string(),
        },
    ]
}

#[test]
fn v78_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v78-godot-2d-adapter-ir-v1"
    );
    assert_eq!(matrix["coverageVersion"], 78);
    assert_eq!(matrix["issueRef"], "#2171");
    assert_eq!(matrix["milestone"], "Era O M89");

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v78.godot-source-text-skeleton-import",
        "v78.lossy-import-not-clean",
        "v78.no-auto-port-without-oracle",
        "v78.deterministic-state-hash-break-fails",
        "v78.coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v78 row {required}");
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
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v78_godot_source_text_skeleton_import_is_read_only_and_honest() {
    let ir = parse_godot_2d_project(sample_project_root()).expect("Godot fixture parses");
    assert!(ir.boundary.contains("one-way"));
    assert!(ir.boundary.contains("clean-room"));
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".tscn"));
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".tres"));
    assert!(ir
        .source
        .accepted_formats
        .iter()
        .any(|fmt| fmt == "project.godot"));
    assert_eq!(ir.scenes.len(), 1);
    assert!(ir
        .scenes
        .iter()
        .flat_map(|scene| scene.nodes.iter())
        .any(|node| node.name == "Player" && node.fidelity_grade == FidelityGrade::Green));
    assert!(ir
        .logic_touchpoints
        .iter()
        .all(|touch| touch.era_r_status == "requires-clean-room-re-derivation"));
    assert!(ir
        .fidelity_report
        .clean_room_notice
        .contains("never copied or translated"));
}

#[test]
fn v78_lossy_import_and_logic_touchpoints_are_not_graded_clean() {
    let ir = parse_godot_2d_source_files("v78-lossy", source_files_with_player_x(10))
        .expect("lossy fixture parses with explicit gaps");
    assert!(ir.unsupported.iter().any(|feature| {
        feature.feature_kind == "GPUParticles2D" && feature.fidelity_grade == FidelityGrade::Red
    }));
    assert!(ir
        .logic_touchpoints
        .iter()
        .all(|touch| touch.fidelity_grade == FidelityGrade::Red));
    assert!(ir.fidelity_report.summary.red >= 2);
    assert!(ir
        .fidelity_report
        .records
        .iter()
        .any(|record| record.grade == FidelityGrade::Red
            && record.reason.contains("not translated logic")));
}

#[test]
fn v78_no_auto_port_without_oracle_or_ungated_claim() {
    let report = godot_2d_adapter_demo_report(sample_project_root()).expect("report builds");
    validate_godot_2d_adapter_demo_report(&report).expect("fixture report is valid");
    assert!(report.claimed_ported_units.is_empty());
    assert!(report.oracle_gate.contains("No unit is claimed ported"));

    let mut forged = report.clone();
    forged
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err =
        validate_godot_2d_adapter_demo_report(&forged).expect_err("ungated port claim must fail");
    assert!(
        err.to_string().contains("cannot claim ported units"),
        "{err}"
    );
}

#[test]
fn v78_deterministic_state_hash_is_stable_and_behavior_drift_changes_it() {
    let first = parse_godot_2d_source_files("v78-determinism", source_files_with_player_x(10))
        .expect("first fixture parses");
    let second = parse_godot_2d_source_files("v78-determinism", source_files_with_player_x(10))
        .expect("second fixture parses");
    let changed = parse_godot_2d_source_files("v78-determinism", source_files_with_player_x(11))
        .expect("changed fixture parses");

    assert_eq!(first, second);
    assert_eq!(hash_ir(&first), hash_ir(&second));
    assert_ne!(
        hash_ir(&first),
        hash_ir(&changed),
        "Godot declarative source drift must alter the deterministic state hash"
    );
}

#[test]
fn v78_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v78-godot-2d-adapter-ir.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v78",
        "godot 2d adapter",
        "source-project/open-text",
        ".tscn",
        ".tres",
        "one-way on-ramp",
        "clean-room",
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
