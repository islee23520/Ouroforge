use std::path::{Path, PathBuf};

use ouroforge_core::import_verification_report::{
    validate_import_verification_report, verify_godot_import,
    write_godot_import_verification_report, IMPORT_VERIFICATION_REPORT_BOUNDARY,
    IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn sample_project_root() -> PathBuf {
    repo_root().join("examples/godot-2d-adapter-v1/sample-project")
}

#[test]
fn import_verification_report_composes_skeleton_smoke_and_fidelity() {
    let report = verify_godot_import(sample_project_root()).unwrap();

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
    assert!(report.skeleton_verification.checked_scene_count > 0);
    assert!(report.skeleton_verification.checked_entity_count > 0);
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
    assert!(!report.re_derivation_tasks.is_empty());
    assert!(report.claimed_ported_units.is_empty());
    assert!(report
        .oracle_records
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));
}

#[test]
fn provenance_records_godot_origin_assets_and_clean_room_boundary() {
    let report = verify_godot_import(sample_project_root()).unwrap();

    assert_eq!(report.provenance.origin, "godot");
    assert!(report
        .provenance
        .accepted_formats
        .iter()
        .any(|fmt| fmt == ".tscn"));
    assert!(report.provenance.clean_room_source_only);
    assert!(!report.provenance.decompiled_source_copied);
    assert!(!report.provenance.asset_licenses.is_empty());
    assert!(report.provenance.asset_licenses.iter().all(|asset| {
        asset.origin == "godot"
            && asset
                .license_status
                .contains("source-project-provenance-recorded")
            && !asset.provenance.is_empty()
    }));
    assert!(report
        .data_shapes
        .ir_nodes_ref
        .contains("godot_2d_adapter_ir.rs"));
    assert!(report
        .data_shapes
        .mapping_records_ref
        .contains("ir_mapping_fidelity_classifier.rs"));
    assert!(report
        .data_shapes
        .behavioral_units_ref
        .contains("logic_touchpoint_handoff.rs"));
    assert!(report.data_shapes.no_elixir_artifact_semantics);
}

#[test]
fn no_port_claim_or_clean_laundering_without_oracle() {
    let mut report = verify_godot_import(sample_project_root()).unwrap();
    report
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_import_verification_report(&report).unwrap_err();
    assert!(err.to_string().contains("cannot claim ported units"));

    let mut report = verify_godot_import(sample_project_root()).unwrap();
    report.oracle_records[0].ported_claim_allowed = true;
    let err = validate_import_verification_report(&report).unwrap_err();
    assert!(err.to_string().contains("oracle-missing"));

    let mut report = verify_godot_import(sample_project_root()).unwrap();
    report.fidelity_report.red = 0;
    let err = validate_import_verification_report(&report).unwrap_err();
    assert!(err.to_string().contains("clean import"));
}

#[test]
fn deterministic_verification_state_hash_catches_tampering() {
    let first = verify_godot_import(sample_project_root()).unwrap();
    let second = verify_godot_import(sample_project_root()).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first.verification_state_hash,
        second.verification_state_hash
    );

    let mut tampered = first.clone();
    tampered
        .fidelity_report
        .gap_summary
        .push("tampered".to_string());
    let err = validate_import_verification_report(&tampered).unwrap_err();
    assert!(
        err.to_string().contains("state hash") && err.to_string().contains("canonical"),
        "{err}"
    );
}

#[test]
fn write_report_persists_json_fixture_shape() {
    let output = repo_root().join("target/generated-test/import-verification-report.json");
    let _ = std::fs::remove_file(&output);
    let report = write_godot_import_verification_report(sample_project_root(), &output).unwrap();
    assert!(output.exists());
    let loaded: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    assert_eq!(
        loaded["schema_version"],
        IMPORT_VERIFICATION_REPORT_SCHEMA_VERSION
    );
    assert_eq!(
        loaded["verification_state_hash"],
        report.verification_state_hash
    );
}

#[test]
fn docs_record_shape_locations_and_boundaries() {
    let doc =
        std::fs::read_to_string(repo_root().join("docs/import-verification-fidelity-report-v1.md"))
            .unwrap()
            .to_ascii_lowercase();
    for required in [
        "import verification and fidelity report v1",
        "godotirnode",
        "mappingrecord",
        "logicbehavioralunitrecord",
        "importoraclerecord",
        "importverificationreport",
        "openchrome-local-skeleton-smoke",
        "origin=godot",
        "claimed_ported_units",
        "source/decompiled code is never copied",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
