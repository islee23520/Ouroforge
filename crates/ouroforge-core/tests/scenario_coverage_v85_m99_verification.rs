//! Scenario Coverage v85 regression suite for #2202 / Era P M99.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use ouroforge_core::gltf_25d_import::{
    example_report_from_fixture, verify_gltf_25d_import_report, Gltf25dFidelityRow,
    Gltf25dRenderSample,
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
fn passing_sample(state_hash: &str) -> Gltf25dRenderSample {
    Gltf25dRenderSample {
        sample_id: "v85-render-smoke".into(),
        expected_state_hash: state_hash.into(),
        observed_state_hash: state_hash.into(),
        ssim: 0.997,
        min_ssim: 0.985,
        pixel_diff: 0.004,
        max_pixel_diff: 0.010,
        render_evidence_ref: "examples/2-5d-gltf-import-v1/render-smoke.test.cjs".into(),
    }
}

#[test]
fn v85_matrix_records_rows_and_boundaries() {
    let matrix =
        read_json("examples/2-5d-gltf-import-v1/scenario-coverage-v85/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v85-m99-verification-v1"
    );
    assert_eq!(matrix["coverageVersion"], 85);
    assert_eq!(matrix["issueRef"], "#2202");
    assert_eq!(matrix["milestone"], "Era P M99");
    assert_eq!(
        matrix["contractRef"],
        "docs/2-5d-import-verification-fidelity-report-v1.md"
    );
    assert_eq!(
        matrix["parentContractRef"],
        "docs/2-5d-migration-on-ramp-scope-contract-v1.md"
    );
    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v85.m99-verification-report-pass",
        "v85.lossy-import-not-clean",
        "v85.no-auto-port-without-oracle",
        "v85.deterministic-state-hash-break-fails",
        "v85.perceptual-render-secondary-fails-on-tolerance",
        "v85.coverage-ledger-and-demo-script",
    ] {
        assert!(ids.contains(required), "missing v85 row {required}");
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
    for key in [
        "oneWayOnRamp",
        "sourceProjectOpenTextOnly",
        "cleanRoomReDerivation",
        "deterministicStateHashRequired",
        "stateHashBreakMustFail",
        "perceptualRenderSecondaryOnly",
        "stateHashPrimaryGateCovered",
        "perceptualToleranceGateCovered",
        "gapAttributionRequired",
        "claimedPortedUnitsEmpty",
        "rustOwnsArtifactTruth",
        "anchorsRemainOpen",
    ] {
        assert_eq!(invariants[key], true, "{key}");
    }
    for key in [
        "autoPortWithoutOracleAllowed",
        "lossyImportMayGradeGreen",
        "ungatedAutoTranslatedPortAllowed",
        "runtimeBridgeAllowed",
        "embeddedEngineRuntimeAllowed",
        "decompiledSourceCopied",
        "studioTrustedWriteAuthority",
        "elixirOwnsArtifactSemantics",
    ] {
        assert_eq!(invariants[key], false, "{key}");
    }
    let boundary = matrix["boundary"].as_str().expect("boundary");
    for required in [
        "Scenario Coverage v85",
        "source-project/open-text",
        "no auto-port",
        "deterministic state-hash primary",
        "perceptual SSIM/pixel-diff render secondary-only",
        "Rust-owned artifact truth",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v85_m99_verification_report_is_read_only_and_honest() {
    let import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    let verification = verify_gltf_25d_import_report(
        &import_report,
        passing_sample(&import_report.state_hash_primary),
    )
    .expect("M99 verification passes");
    verification.validate().expect("verification validates");
    assert_eq!(verification.verdict, "pass");
    assert!(verification.state_hash_gate.primary);
    assert_eq!(verification.state_hash_gate.status, "pass");
    assert!(!verification.perceptual_render_gate.primary);
    assert_eq!(verification.perceptual_render_gate.status, "pass");
    assert!(verification.perceptual_render_secondary_only);
    assert!(verification.claimed_ported_units.is_empty());
    assert!(verification
        .gap_attribution
        .iter()
        .any(|gap| gap.unit == "extension:VENDOR_custom_shader_note"));
    assert!(verification
        .data_shapes
        .iter()
        .any(|shape| shape.shape == "Verification report"));
}

#[test]
fn v85_lossy_import_and_gap_attribution_are_not_graded_clean() {
    let import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    let verification = verify_gltf_25d_import_report(
        &import_report,
        passing_sample(&import_report.state_hash_primary),
    )
    .expect("M99 verification passes");
    let non_green_units = verification
        .fidelity_rows
        .iter()
        .filter(|row| row.grade != "green")
        .map(|row| row.unit.as_str())
        .collect::<BTreeSet<_>>();
    assert!(non_green_units.contains("material:tile-unlit"));
    assert!(non_green_units.contains("extension:VENDOR_custom_shader_note"));
    for unit in non_green_units {
        assert!(
            verification
                .gap_attribution
                .iter()
                .any(|gap| gap.unit == unit),
            "missing gap attribution for {unit}"
        );
    }
}

#[test]
fn v85_no_auto_port_without_oracle_or_auto_translation_claim() {
    let mut import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    import_report.fidelity_rows.push(Gltf25dFidelityRow {
        unit: "verification:forged-equivalence".into(),
        grade: "green".into(),
        reason: "auto-translated ported 2.5D behavior without captured oracle".into(),
        oracle_required: false,
    });
    let err = verify_gltf_25d_import_report(
        &import_report,
        passing_sample(&import_report.state_hash_primary),
    )
    .expect_err("ungated port/auto-translation claim must fail");
    assert!(
        err.to_string().contains("must not claim units were ported"),
        "{err}"
    );
    let import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    let mut verification = verify_gltf_25d_import_report(
        &import_report,
        passing_sample(&import_report.state_hash_primary),
    )
    .expect("M99 verification passes");
    verification
        .claimed_ported_units
        .push("presentation:hero-billboard".into());
    let err = verification
        .validate()
        .expect_err("claimedPortedUnits must remain empty");
    assert!(err.to_string().contains("must not claim ported units"));
}

#[test]
fn v85_deterministic_state_hash_break_fails() {
    let import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    let mut stale = passing_sample(&import_report.state_hash_primary);
    stale.observed_state_hash =
        "sha256:0000000000000000000000000000000000000000000000000000000000000000".into();
    let verification = verify_gltf_25d_import_report(&import_report, stale)
        .expect("failed verification report still emits");
    assert_eq!(verification.verdict, "fail");
    assert_eq!(verification.state_hash_gate.status, "fail");
    assert_eq!(verification.perceptual_render_gate.status, "pass");
    let mut tampered = verify_gltf_25d_import_report(
        &import_report,
        passing_sample(&import_report.state_hash_primary),
    )
    .expect("M99 verification passes");
    tampered.state_hash_gate.detail.push_str(" tampered");
    let err = tampered
        .validate()
        .expect_err("tampered report hash must fail");
    assert!(err.to_string().contains("reportHash must match"));
}

#[test]
fn v85_perceptual_render_secondary_fails_on_tolerance() {
    let import_report = example_report_from_fixture().expect("fixture glTF normalizes");
    let mut render_diff = passing_sample(&import_report.state_hash_primary);
    render_diff.ssim = 0.900;
    render_diff.pixel_diff = 0.050;
    let verification = verify_gltf_25d_import_report(&import_report, render_diff)
        .expect("failed verification report still emits");
    assert_eq!(verification.verdict, "fail");
    assert_eq!(verification.state_hash_gate.status, "pass");
    assert_eq!(verification.perceptual_render_gate.status, "fail");
    assert!(!verification.perceptual_render_gate.primary);
    assert!(verification.perceptual_render_secondary_only);
}

#[test]
fn v85_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v85-m99-verification.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v85",
        "2.5d import verification",
        "source-project/open-text",
        "one-way on-ramp",
        "clean-room",
        "no live foreign engine bridge",
        "no auto-port",
        "passing oracle",
        "statehashprimary",
        "perceptual ssim/pixel-diff render evidence is secondary-only",
        "yellow/red",
        "gap attribution",
        "rust owns artifact truth",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
        "cargo test --workspace --jobs 2",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
    let demo = read_json("examples/2-5d-gltf-import-v1/m99-verification-demo-summary.fixture.json");
    assert_eq!(demo["schemaVersion"], "gltf-25d-m99-verification-demo-v1");
    assert_eq!(demo["issueRef"], "#2201");
    assert!(demo["fidelitySummary"]["claimedPortedUnits"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(demo["stateHashGate"]["status"], "pass");
    assert_eq!(demo["perceptualRenderGate"]["status"], "pass");
}
