//! Scenario Coverage v83 regression suite for #2195 / Era P M97.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::gltf_25d_import::{
    example_report_from_fixture, normalize_gltf_25d_import_from_str, Gltf25dFidelityRow,
    Gltf25dImportOptions,
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
    read_json("examples/2-5d-gltf-import-v1/scenario-coverage-v83/matrix.fixture.json")
}

fn fixture_options(source_path: &str) -> Gltf25dImportOptions {
    Gltf25dImportOptions {
        source_project_ref: "examples/2-5d-gltf-import-v1/source-project".to_string(),
        source_path: source_path.to_string(),
        unit_scale: 1.0,
        axis_convention: "gltf-y-up-right-handed-to-ouroforge-presentation".to_string(),
        color_space: "srgb-textures-linear-lighting".to_string(),
        viewport_width: 640,
        viewport_height: 360,
    }
}

#[test]
fn v83_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v83-gltf-25d-import-v1"
    );
    assert_eq!(matrix["coverageVersion"], 83);
    assert_eq!(matrix["issueRef"], "#2195");
    assert_eq!(matrix["milestone"], "Era P M97");
    assert_eq!(
        matrix["contractRef"],
        "docs/gltf-geometry-orthographic-camera-import-contract-v1.md"
    );
    assert_eq!(
        matrix["implementationRef"],
        "crates/ouroforge-core/src/gltf_25d_import.rs"
    );

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v83.gltf-geometry-ortho-import",
        "v83.lossy-import-not-clean",
        "v83.no-auto-port-without-oracle",
        "v83.deterministic-state-hash-break-fails",
        "v83.coverage-ledger-and-demo-script",
    ] {
        assert!(ids.contains(required), "missing v83 row {required}");
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
    assert_eq!(invariants["ungatedAutoTranslatedPortAllowed"], false);
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["stateHashBreakMustFail"], true);
    assert_eq!(invariants["perceptualRenderSecondaryOnly"], true);
    assert_eq!(invariants["runtimeBridgeAllowed"], false);
    assert_eq!(invariants["embeddedEngineRuntimeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);

    let boundary = matrix["boundary"].as_str().expect("boundary");
    for required in [
        "Scenario Coverage v83",
        "source-project/open-text",
        "no auto-port",
        "deterministic state-hash primary",
        "perceptual render secondary-only",
        "Rust-owned artifact truth",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v83_gltf_geometry_ortho_import_is_read_only_and_honest() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    report.validate().expect("report validates");
    assert!(report.boundary.contains("one-way source-project"));
    assert!(report.boundary.contains("no live engine bridge"));
    assert_eq!(report.native_scene.scene_kind, "2.5d-presentation");
    assert_eq!(report.native_scene.active_camera_id, "main-ortho");
    assert!(report
        .native_scene
        .nodes
        .iter()
        .any(|node| node.mesh_ref.as_deref() == Some("tile-mesh")));
    assert!(report
        .native_scene
        .cameras
        .iter()
        .any(|camera| camera.projection == "orthographic" && camera.fidelity_grade == "green"));
    assert!(report
        .native_scene
        .logic_authority
        .contains("cannot mutate gameplay truth"));
    assert!(report.state_hash_primary.starts_with("sha256:"));
    assert_eq!(
        report.perceptual_render_secondary.role,
        "secondary corroboration only"
    );
}

#[test]
fn v83_lossy_import_and_behavior_markers_are_not_graded_clean() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "extension:VENDOR_custom_shader_note" && row.grade == "yellow"));
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "material:tile-unlit" && row.grade == "yellow"));
    assert!(report
        .re_derivation_tasks
        .iter()
        .any(|task| task.unit == "logic"));
    assert!(report
        .re_derivation_tasks
        .iter()
        .any(|task| task.unit == "physics"));
    assert!(report
        .re_derivation_tasks
        .iter()
        .all(|task| task.era_r_input.contains("captured oracle")));
}

#[test]
fn v83_no_auto_port_without_oracle_or_auto_translation_claim() {
    let mut forged = example_report_from_fixture().expect("fixture glTF normalizes");
    forged.fidelity_rows.push(Gltf25dFidelityRow {
        unit: "logic:forged-controller".to_string(),
        grade: "green".to_string(),
        reason: "auto-translated and ported with no oracle evidence".to_string(),
        oracle_required: false,
    });
    let err = forged
        .validate()
        .expect_err("ungated port/auto-translation claim must fail");
    assert!(
        err.to_string().contains("must not claim units were ported"),
        "{err}"
    );
}

#[test]
fn v83_deterministic_state_hash_break_fails() {
    let mut stale = example_report_from_fixture().expect("fixture glTF normalizes");
    stale.native_scene.nodes[2].local_transform.translation.x += 1.0;
    let err = stale
        .validate()
        .expect_err("tampered native scene with stale hash must fail");
    assert!(
        err.to_string().contains("stateHashPrimary must match"),
        "{err}"
    );

    let original_source = read_text("examples/2-5d-gltf-import-v1/source/ortho-demo.gltf");
    let changed_source =
        original_source.replace("\"translation\": [2, 0, -1]", "\"translation\": [3, 0, -1]");
    let original = normalize_gltf_25d_import_from_str(
        &original_source,
        fixture_options("examples/2-5d-gltf-import-v1/source/ortho-demo.gltf"),
    )
    .expect("original normalizes");
    let changed = normalize_gltf_25d_import_from_str(
        &changed_source,
        fixture_options("examples/2-5d-gltf-import-v1/source/ortho-demo.changed.gltf"),
    )
    .expect("changed normalizes");
    assert_ne!(
        original.state_hash_primary, changed.state_hash_primary,
        "declarative source drift must alter the deterministic state hash"
    );
}

#[test]
fn v83_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v83-gltf-25d-import.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v83",
        "gltf geometry",
        "orthographic-camera",
        "source-project/open-text",
        "one-way on-ramp",
        "clean-room",
        "no auto-port",
        "passing oracle",
        "yellow/red",
        "statehashprimary",
        "perceptual secondary",
        "rust owns artifact truth",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
        "cargo test --workspace --jobs 2",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
