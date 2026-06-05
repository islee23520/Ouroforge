//! Desktop Packaging Capability Gate v1 contract (#732).

use ouroforge_core::export_capability_report::{
    CapabilityReport, CapabilityStatus, CAPABILITY_REPORT_SCHEMA_VERSION,
};
use ouroforge_core::export_profile::{ExportProfile, FUTURE_EXPORT_TARGETS};
use std::path::{Path, PathBuf};

const GATE_DOC: &str = include_str!("../../../docs/desktop-packaging-capability-gate-v1.md");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn report_fixture() -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/desktop-packaging-capability-report-v1/capability-report.fixture.json"),
    )
    .expect("capability report fixture readable")
}

#[test]
fn capability_report_marks_desktop_packaging_not_implemented() {
    let report = CapabilityReport::from_json_str(&report_fixture()).expect("report parses");
    assert_eq!(report.schema_version, CAPABILITY_REPORT_SCHEMA_VERSION);
    assert_eq!(report.capability, "desktop-packaging");
    assert_eq!(report.status, CapabilityStatus::Future);
    assert!(report.is_not_implemented());
    assert!(!report.implemented);
    assert!(report.platforms.contains(&"windows".to_string()));
    assert!(report.platforms.contains(&"macos".to_string()));
    assert!(report.platforms.contains(&"linux".to_string()));
    assert!(!report.requirements.is_empty());
}

#[test]
fn report_cannot_claim_implemented() {
    let mut value: serde_json::Value = serde_json::from_str(&report_fixture()).unwrap();
    value["implemented"] = serde_json::json!(true);
    let err =
        CapabilityReport::from_json_str(&value.to_string()).expect_err("implemented=true rejected");
    assert!(err.to_string().contains("not mark"));
}

#[test]
fn export_validation_still_blocks_desktop_targets() {
    // desktop-wrapper is a declared future/design-gated target...
    assert!(FUTURE_EXPORT_TARGETS.contains(&"desktop-wrapper"));
    // ...and export profile validation still rejects it.
    let valid = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&valid).unwrap();
    value["exportTarget"] = serde_json::json!("desktop-wrapper");
    let err = ExportProfile::from_json_str(&value.to_string())
        .expect_err("desktop-wrapper target rejected");
    let msg = err.to_string();
    assert!(msg.contains("desktop-wrapper"));
    assert!(msg.contains("future") || msg.contains("design-gated"));
}

#[test]
fn gate_doc_documents_requirements_and_governance() {
    assert!(GATE_DOC.contains("NOT implemented in v1"));
    assert!(GATE_DOC.to_lowercase().contains("windows"));
    assert!(GATE_DOC.to_lowercase().contains("signing"));
    assert!(GATE_DOC.to_lowercase().contains("notarization"));
    assert!(GATE_DOC.contains("desktop-wrapper"));
    assert!(GATE_DOC.contains("#1 remains"));
    assert!(GATE_DOC.contains("#23 remains"));
}
