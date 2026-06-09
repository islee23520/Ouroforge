//! Scenario Coverage v81 regression suite for #2181 / Era O M92.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::godot_2d_adapter_ir::{parse_godot_2d_source_files, GodotSourceFile};
use ouroforge_core::import_verification_report::{
    validate_import_verification_report, verify_godot_import, verify_godot_import_ir,
    IMPORT_VERIFICATION_REPORT_BOUNDARY, IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION,
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
    read_json("examples/godot-2d-adapter-v1/scenario-coverage-v81/matrix.fixture.json")
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
fire={"events":[{"physical_keycode":32}]}
"#
            .to_string(),
        },
        GodotSourceFile {
            path: "main.tscn".to_string(),
            contents: format!(
                r#"
[gd_scene load_steps=3 format=3 uid="uid://scenario-v81"]

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
fn v81_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v81-import-verification-fidelity-report-v1"
    );
    assert_eq!(matrix["coverageVersion"], 81);
    assert_eq!(matrix["issueRef"], "#2181");
    assert_eq!(matrix["milestone"], "Era O M92");
    assert_eq!(
        matrix["demoRef"],
        "examples/godot-2d-adapter-v1/import-verification-demo/run-demo.sh"
    );

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v81.import-verification-report-demo-evidence",
        "v81.lossy-import-not-clean",
        "v81.no-auto-port-without-oracle",
        "v81.deterministic-state-hash-break-fails",
        "v81.coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v81 row {required}");
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
    assert_eq!(invariants["skeletonSmokeIsPortEquivalence"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v81_import_verification_demo_report_is_honest_and_replayable() {
    let report = verify_godot_import(sample_project_root()).expect("sample report");
    validate_import_verification_report(&report).expect("report validates");

    assert_eq!(
        report.schema_version,
        IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION
    );
    assert_eq!(report.boundary, IMPORT_VERIFICATION_REPORT_BOUNDARY);
    assert_eq!(report.source_engine, "godot");
    assert!(report.source_ir_hash.starts_with("sha256:"));
    assert!(report.mapping_state_hash.starts_with("sha256:"));
    assert!(report.logic_handoff_state_hash.starts_with("sha256:"));
    assert!(report.verification_state_hash.starts_with("sha256:"));
    assert_eq!(
        report.skeleton_verification.runner,
        "openchrome-local-skeleton-smoke"
    );
    assert_eq!(report.skeleton_verification.status, "passed");
    assert!(
        report
            .skeleton_verification
            .deterministic_state_hash_required
    );
    assert!(
        report
            .skeleton_verification
            .perceptual_render_secondary_only
    );
    assert!(report.fidelity_report.clean > 0);
    assert!(report.fidelity_report.flagged > 0);
    assert!(report.fidelity_report.rederive > 0);
    assert!(report.fidelity_report.red > 0);
    assert!(report
        .fidelity_report
        .oracle_rule
        .contains("No imported unit is ported"));
    assert!(report
        .fidelity_report
        .clean_room_notice
        .contains("never copied or translated"));
    assert!(!report.re_derivation_tasks.is_empty());
    assert!(report.claimed_ported_units.is_empty());
    assert!(report
        .oracle_records
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));
    assert_eq!(report.provenance.origin, "godot");
    assert!(report.provenance.clean_room_source_only);
    assert!(!report.provenance.decompiled_source_copied);
    assert!(report.data_shapes.no_elixir_artifact_semantics);
}

#[test]
fn v81_lossy_import_and_behavioral_gaps_cannot_be_laundered_green() {
    let ir = parse_godot_2d_source_files("v81-lossy", source_files_with_player_x(10))
        .expect("lossy fixture parses");
    let mut report = verify_godot_import_ir("v81-lossy", &ir).expect("report succeeds");

    assert!(report.fidelity_report.flagged > 0, "{report:#?}");
    assert!(report.fidelity_report.rederive > 0, "{report:#?}");
    assert!(report.fidelity_report.red > 0, "{report:#?}");
    assert!(report
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("Partial")
            || gap.contains("Unsupported")
            || gap.contains("inventoried only")));
    assert!(report
        .re_derivation_tasks
        .iter()
        .all(|task| task.target_era == "Era R"));

    report.fidelity_report.red = 0;
    let err = validate_import_verification_report(&report)
        .expect_err("lossy import with re-derive gaps cannot be graded clean");
    assert!(err.to_string().contains("clean import"), "{err}");
}

#[test]
fn v81_no_auto_port_without_oracle_or_auto_translation_claim() {
    let mut report = verify_godot_import(sample_project_root()).expect("sample report");
    assert!(report.claimed_ported_units.is_empty());
    report
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_import_verification_report(&report).expect_err("ungated claim fails");
    assert!(err.to_string().contains("cannot claim ported units"));

    let mut report = verify_godot_import(sample_project_root()).expect("sample report");
    report.oracle_records[0].ported_claim_allowed = true;
    let err = validate_import_verification_report(&report).expect_err("oracle bypass fails");
    assert!(err.to_string().contains("oracle-missing"));
}

#[test]
fn v81_deterministic_state_hash_is_stable_and_tamper_break_fails() {
    let first_ir = parse_godot_2d_source_files("v81-determinism", source_files_with_player_x(10))
        .expect("first parses");
    let second_ir = parse_godot_2d_source_files("v81-determinism", source_files_with_player_x(10))
        .expect("second parses");
    let changed_ir = parse_godot_2d_source_files("v81-determinism", source_files_with_player_x(11))
        .expect("changed parses");

    let first = verify_godot_import_ir("v81-determinism", &first_ir).expect("first report");
    let second = verify_godot_import_ir("v81-determinism", &second_ir).expect("second report");
    let changed = verify_godot_import_ir("v81-determinism", &changed_ir).expect("changed report");

    assert_eq!(first, second);
    assert_eq!(
        first.verification_state_hash,
        second.verification_state_hash
    );
    assert_ne!(first.source_ir_hash, changed.source_ir_hash);
    assert_ne!(first.mapping_state_hash, changed.mapping_state_hash);
    assert_ne!(
        first.verification_state_hash,
        changed.verification_state_hash
    );

    let mut tampered = first.clone();
    tampered
        .fidelity_report
        .gap_summary
        .push("tampered stale report".to_string());
    let err = validate_import_verification_report(&tampered).expect_err("stale hash fails");
    assert!(
        err.to_string().contains("state hash") && err.to_string().contains("canonical"),
        "{err}"
    );
}

#[test]
fn v81_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v81-import-verification-fidelity-report.md")
        .to_ascii_lowercase();
    for required in [
        "scenario coverage v81",
        "import verification and fidelity report",
        "one-way on-ramp",
        "source-project/open-text",
        "clean-room",
        "no auto-port",
        "passing oracle",
        "openchrome-local-skeleton-smoke",
        "skeleton-shape evidence only",
        "not port equivalence",
        "lossy import",
        "yellow/red",
        "deterministic",
        "state hashes",
        "rust remains the data plane",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
