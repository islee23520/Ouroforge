//! Export Profile Schema v1 contract (#721).
//!
//! Valid local web export profiles parse and validate; blocked/future targets,
//! unsafe paths, traversal, backslashes, generated-state misuse, and missing
//! entry scenes fail closed with actionable diagnostics. Validation is pure:
//! it executes no commands and writes no artifacts.

use ouroforge_core::export_profile::{
    ExportProfile, RuntimeProbeMode, ALLOWED_EXPORT_TARGETS, BLOCKED_EXPORT_TARGETS,
    EXPORT_PROFILE_SCHEMA_VERSION,
};

const VALID: &str =
    include_str!("../../../examples/export-profile-v1/export-profile.valid.fixture.json");

fn invalid(name: &str) -> String {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-profile-v1/invalid")
        .join(name);
    std::fs::read_to_string(&base).unwrap_or_else(|err| panic!("read {}: {err}", base.display()))
}

#[test]
fn valid_web_local_profile_parses_and_validates() {
    let profile = ExportProfile::from_json_str(VALID).expect("valid profile parses");
    assert_eq!(profile.schema_version, EXPORT_PROFILE_SCHEMA_VERSION);
    assert_eq!(profile.export_target, "web-local");
    assert!(profile.target_is_allowed());
    assert_eq!(profile.runtime_probe_mode, RuntimeProbeMode::Preserve);
    assert_eq!(profile.asset_roots.len(), 2);
    assert_eq!(profile.verification_scenario_ids.len(), 2);
}

#[test]
fn allowed_targets_are_exactly_the_two_v1_web_targets() {
    assert_eq!(ALLOWED_EXPORT_TARGETS, &["web-local", "web-static-bundle"]);
    for target in BLOCKED_EXPORT_TARGETS {
        assert!(!ALLOWED_EXPORT_TARGETS.contains(target));
    }
}

#[test]
fn blocked_target_fails_closed_with_actionable_diagnostic() {
    let err = ExportProfile::from_json_str(&invalid("blocked-target.json"))
        .expect_err("blocked target rejected");
    let msg = err.to_string();
    assert!(msg.contains("steam"));
    assert!(msg.contains("blocked"));
    assert!(msg.contains("export-target-matrix-v1.md"));
}

#[test]
fn future_target_fails_closed_as_design_gated() {
    let err = ExportProfile::from_json_str(&invalid("future-target.json"))
        .expect_err("future target rejected");
    let msg = err.to_string();
    assert!(msg.contains("desktop-wrapper"));
    assert!(msg.contains("future") || msg.contains("design-gated"));
}

#[test]
fn path_traversal_output_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("path-traversal-output.json"))
        .expect_err("traversal output rejected");
    assert!(err.to_string().contains(".."));
}

#[test]
fn backslash_path_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("backslash-path.json"))
        .expect_err("backslash path rejected");
    assert!(err.to_string().contains("backslash"));
}

#[test]
fn generated_state_source_input_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("generated-output-source.json"))
        .expect_err("generated-state source rejected");
    assert!(err.to_string().contains("generated state"));
}

#[test]
fn output_outside_staging_root_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("output-not-staging.json"))
        .expect_err("non-staging output rejected");
    assert!(err.to_string().contains("staging root"));
}

#[test]
fn missing_entry_scene_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("missing-entry-scene.json"))
        .expect_err("missing entry scene rejected");
    assert!(err.to_string().contains("entryScene"));
}

#[test]
fn bad_schema_version_is_rejected() {
    let err = ExportProfile::from_json_str(&invalid("bad-schema-version.json"))
        .expect_err("bad schema version rejected");
    assert!(err.to_string().contains(EXPORT_PROFILE_SCHEMA_VERSION));
}

#[test]
fn unknown_fields_are_rejected() {
    let mut value: serde_json::Value = serde_json::from_str(VALID).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("publish".to_string(), serde_json::json!(true));
    let err = ExportProfile::from_json_str(&value.to_string()).expect_err("unknown field rejected");
    assert!(err
        .to_string()
        .contains("failed to parse Export Profile JSON"));
}
