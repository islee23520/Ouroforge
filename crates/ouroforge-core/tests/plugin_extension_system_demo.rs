//! Plugin / Extension System Demo v1 (#754).
//!
//! Walks the full v1 plugin/extension flow over the fixture plugin pack: local
//! registry discovery, manifest validation, valid descriptor rendering, blocked
//! diagnostics for invalid fixtures, plugin evidence output, and read-only
//! inspection. The demo discovers declarative descriptors only; it never
//! executes plugins, installs dependencies, contacts the network, mutates
//! source, publishes, or deploys.

use ouroforge_core::plugin_evidence::PluginRegistryEvidenceArtifact;
use ouroforge_core::plugin_registry::{discover_plugins_in_dir, PluginRegistryStatus};
use serde_json::Value;
use std::path::{Path, PathBuf};

const DEMO_DOC: &str = include_str!("../../../docs/plugin-extension-system-demo-v1.md");
const DEMO_FIXTURE: &str =
    include_str!("../../../examples/plugin-extension-system-demo-v1/demo-manifest.fixture.json");
const VALID_EVIDENCE: &str =
    include_str!("../../../examples/plugin-registry-evidence-v1/valid/plugin-registry.sample.json");
const INVALID_EVIDENCE: &str = include_str!(
    "../../../examples/plugin-registry-evidence-v1/invalid/executable-capability.json"
);

const STAGE_IDS: &[&str] = &[
    "BEP754.discover",
    "BEP754.validate-valid",
    "BEP754.validate-invalid",
    "BEP754.incompatible",
    "BEP754.evidence",
    "BEP754.evidence-blocked",
    "BEP754.read-only-inspection",
];

fn fixture_pack_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-fixture-pack-v1")
}

fn demo_manifest() -> Value {
    serde_json::from_str(DEMO_FIXTURE).expect("demo manifest parses")
}

#[test]
fn plugin_demo_manifest_declares_every_stage() {
    let manifest = demo_manifest();
    assert_eq!(manifest["schemaVersion"], "plugin-extension-system-demo-v1");
    assert_eq!(manifest["issue"], 754);
    let stages = manifest["stages"].as_array().expect("stages are an array");
    let ids: Vec<&str> = stages
        .iter()
        .map(|stage| stage["id"].as_str().expect("stage id"))
        .collect();
    for required in STAGE_IDS {
        assert!(ids.contains(required), "manifest missing stage {required}");
        assert!(DEMO_DOC.contains(required), "doc missing stage {required}");
    }
    assert_eq!(stages.len(), STAGE_IDS.len());
}

#[test]
fn plugin_demo_documents_boundaries_and_governance() {
    let lower_doc = DEMO_DOC.to_ascii_lowercase();
    let lower_fixture = DEMO_FIXTURE.to_ascii_lowercase();
    for term in [
        "read-only",
        "fixture-scoped",
        "generated",
        "marketplace",
        "credential",
        "command bridge",
        "production-ready",
        "godot replacement",
        "deploy",
        "network",
    ] {
        assert!(lower_doc.contains(term), "doc missing guardrail {term}");
        assert!(
            lower_fixture.contains(term),
            "fixture missing guardrail {term}"
        );
    }
    assert!(DEMO_DOC.contains("#1 remains open"));
    assert!(DEMO_DOC.contains("#23 remains open"));
}

#[test]
fn plugin_demo_discovers_valid_and_blocked_descriptors() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    let model = registry.read_model();

    // Valid descriptors appear; invalid and incompatible are surfaced.
    assert!(model.valid_count >= 1, "{model:?}");
    assert!(model.invalid_count >= 1, "{model:?}");
    assert!(model.incompatible_count >= 1, "{model:?}");

    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Valid)
    {
        assert!(
            !entry.declared_capabilities.is_empty() && !entry.extension_points.is_empty(),
            "valid plugin `{}` must contribute declarative descriptors",
            entry.plugin_id
        );
    }

    // Invalid plugins show blocked diagnostics and contribute no extension points.
    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Invalid)
    {
        assert!(
            !entry.validation_errors.is_empty(),
            "invalid plugin `{}` must report blocked diagnostics",
            entry.plugin_id
        );
        assert!(entry.extension_points.is_empty());
    }

    // Read-only inspection boundary is stated for rendering surfaces.
    let boundary = model.boundary.to_ascii_lowercase();
    assert!(boundary.contains("read-only"));
    assert!(boundary.contains("no plugin execution"));
}

#[test]
fn plugin_demo_evidence_output_validates_and_fails_closed() {
    // Valid evidence parses and exposes a read model for read-only inspection.
    let evidence =
        PluginRegistryEvidenceArtifact::from_json_str(VALID_EVIDENCE).expect("valid evidence");
    let read_model = evidence.read_model();
    let read_model_json =
        serde_json::to_string(&read_model).expect("read model serializes for inspection");
    assert!(!read_model_json.is_empty());

    // Evidence declaring an executable capability fails closed.
    assert!(
        PluginRegistryEvidenceArtifact::from_json_str(INVALID_EVIDENCE).is_err(),
        "executable-capability evidence must fail closed"
    );
}
