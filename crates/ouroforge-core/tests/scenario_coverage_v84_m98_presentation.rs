//! Scenario Coverage v84 regression suite for #2199 / Era P M98.

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
    read_json("examples/2-5d-gltf-import-v1/scenario-coverage-v84/matrix.fixture.json")
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
fn v84_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v84-m98-presentation-v1"
    );
    assert_eq!(matrix["coverageVersion"], 84);
    assert_eq!(matrix["issueRef"], "#2199");
    assert_eq!(matrix["milestone"], "Era P M98");
    assert_eq!(
        matrix["contractRef"],
        "docs/billboard-sprite-stack-presentation-contract-v1.md"
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
        "v84.m98-presentation-primitives",
        "v84.presentation-cannot-mutate-logic",
        "v84.lossy-presentation-not-clean",
        "v84.no-auto-port-without-oracle",
        "v84.deterministic-state-hash-break-fails",
        "v84.coverage-ledger-and-demo-script",
    ] {
        assert!(ids.contains(required), "missing v84 row {required}");
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
    assert_eq!(invariants["m98PresentationCannotMutateLogic"], true);
    assert_eq!(invariants["billboardCovered"], true);
    assert_eq!(invariants["spriteStackCovered"], true);
    assert_eq!(invariants["twoDInThreeDPlaneCovered"], true);
    assert_eq!(invariants["runtimeBridgeAllowed"], false);
    assert_eq!(invariants["embeddedEngineRuntimeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);

    let boundary = matrix["boundary"].as_str().expect("boundary");
    for required in [
        "Scenario Coverage v84",
        "source-project/open-text",
        "billboard, sprite-stack, and 2D-in-3D",
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
fn v84_m98_presentation_primitives_are_read_only_and_honest() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    report.validate().expect("report validates");
    let kinds = report
        .native_scene
        .presentation_layers
        .iter()
        .map(|layer| layer.kind.as_str())
        .collect::<Vec<_>>();
    assert_eq!(kinds, ["billboard", "sprite-stack", "2d-in-3d-plane"]);
    assert!(report.native_scene.presentation_layers.iter().all(|layer| {
        layer
            .authority
            .contains("cannot mutate deterministic logic/evidence")
    }));
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "presentation:hero-billboard" && row.grade == "green"));
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "presentation:crate-sprite-stack" && row.grade == "green"));
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "presentation:dialogue-plane-2d-in-3d" && row.grade == "green"));
    assert_eq!(
        report.perceptual_render_secondary.role,
        "secondary corroboration only"
    );
}

#[test]
fn v84_lossy_presentation_is_not_graded_clean() {
    let source = read_text("examples/2-5d-gltf-import-v1/source/ortho-demo.gltf");
    let unsupported = source.replace(
        "\"kind\": \"billboard\"",
        "\"kind\": \"runtime-driven-particle-billboard\"",
    );
    let err = normalize_gltf_25d_import_from_str(
        &unsupported,
        fixture_options("examples/2-5d-gltf-import-v1/source/unsupported-presentation.gltf"),
    )
    .expect_err("unsupported presentation primitive fails closed");
    assert!(
        err.to_string()
            .contains("unsupported M98 presentation primitive kind"),
        "{err}"
    );

    let logic_coupled = source.replace(
        "\"id\": \"hero-billboard\"",
        "\"id\": \"hero-billboard\", \"logicCoupling\": \"drives gameplay logic\"",
    );
    let report = normalize_gltf_25d_import_from_str(
        &logic_coupled,
        fixture_options("examples/2-5d-gltf-import-v1/source/logic-coupled-presentation.gltf"),
    )
    .expect("logic-coupled presentation normalizes as non-green fidelity");
    assert!(report.fidelity_rows.iter().any(|row| {
        row.unit == "presentation:hero-billboard"
            && row.grade == "yellow"
            && row.reason.contains("not auto-translated")
    }));
}

#[test]
fn v84_no_auto_port_without_oracle_or_auto_translation_claim() {
    let mut forged = example_report_from_fixture().expect("fixture glTF normalizes");
    forged.fidelity_rows.push(Gltf25dFidelityRow {
        unit: "presentation:forged-runtime-controller".to_string(),
        grade: "green".to_string(),
        reason: "auto-translated ported behavior-equivalent presentation logic without oracle"
            .to_string(),
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
fn v84_deterministic_state_hash_break_fails() {
    let mut stale = example_report_from_fixture().expect("fixture glTF normalizes");
    stale.native_scene.presentation_layers[0].sorting_key += 1;
    let err = stale
        .validate()
        .expect_err("tampered presentation layer with stale hash must fail");
    assert!(
        err.to_string().contains("stateHashPrimary must match"),
        "{err}"
    );

    let original_source = read_text("examples/2-5d-gltf-import-v1/source/ortho-demo.gltf");
    let changed_source = original_source.replace("\"sortingKey\": 40", "\"sortingKey\": 41");
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
        "M98 presentation source drift must alter the deterministic state hash"
    );
}

#[test]
fn v84_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v84-m98-presentation.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v84",
        "billboard",
        "sprite-stack",
        "2d-in-3d",
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
