//! Contract tests for the declarative plugin manifest schema (#739).
//!
//! Confirms that valid fixture manifests parse and validate and that invalid
//! fixtures fail closed with actionable diagnostics. Validation performs no
//! command execution, code loading, network access, or trusted writes.

use ouroforge_core::plugin_manifest::{PluginManifest, PLUGIN_MANIFEST_SCHEMA_VERSION};

fn valid_dashboard() -> &'static str {
    include_str!("../../../examples/plugin-manifest-v1/valid/dashboard-panel-plugin.plugin.json")
}

fn valid_scenario() -> &'static str {
    include_str!("../../../examples/plugin-manifest-v1/valid/scenario-template-plugin.plugin.json")
}

fn invalid_executable_capability() -> &'static str {
    include_str!("../../../examples/plugin-manifest-v1/invalid/executable-capability.plugin.json")
}

fn invalid_unknown_extension_point() -> &'static str {
    include_str!("../../../examples/plugin-manifest-v1/invalid/unknown-extension-point.plugin.json")
}

fn invalid_unsafe_path_traversal() -> &'static str {
    include_str!("../../../examples/plugin-manifest-v1/invalid/unsafe-path-traversal.plugin.json")
}

fn invalid_incompatible_schema_version() -> &'static str {
    include_str!(
        "../../../examples/plugin-manifest-v1/invalid/incompatible-schema-version.plugin.json"
    )
}

fn invalid_executable_entrypoint_field() -> &'static str {
    include_str!(
        "../../../examples/plugin-manifest-v1/invalid/executable-entrypoint-field.plugin.json"
    )
}

#[test]
fn valid_dashboard_manifest_fixture_validates() {
    let manifest =
        PluginManifest::from_json_str(valid_dashboard()).expect("dashboard fixture validates");
    assert_eq!(manifest.schema_version, PLUGIN_MANIFEST_SCHEMA_VERSION);
    assert_eq!(manifest.plugin_id, "read-only-dashboard-panel");
    assert_eq!(manifest.declared_capabilities, ["dashboardPanel"]);
    assert_eq!(manifest.descriptor_refs.len(), 1);
    let model = manifest.read_model();
    assert_eq!(model.descriptor_ref_count, 1);
    assert_eq!(model.doc_count, 1);
    assert_eq!(model.asset_count, 1);
    assert!(model.boundary.to_ascii_lowercase().contains("read-only"));
}

#[test]
fn valid_scenario_manifest_fixture_validates() {
    let manifest =
        PluginManifest::from_json_str(valid_scenario()).expect("scenario fixture validates");
    assert_eq!(manifest.plugin_id, "read-only-scenario-template");
    assert_eq!(manifest.extension_points, ["scenario.templates.readOnly"]);
    assert_eq!(manifest.compatibility.max_ouroforge_version, "1.0.0");
}

#[test]
fn invalid_fixtures_fail_closed_with_diagnostics() {
    for (fixture, needle) in [
        (invalid_executable_capability(), "not in the v1 allowlist"),
        (invalid_unknown_extension_point(), "not in the v1 allowlist"),
        (invalid_unsafe_path_traversal(), "without traversal"),
        (
            invalid_incompatible_schema_version(),
            "not a supported schema version",
        ),
        (invalid_executable_entrypoint_field(), "unknown field"),
    ] {
        let err = format!(
            "{:#}",
            PluginManifest::from_json_str(fixture)
                .expect_err("invalid manifest fixture must fail closed")
        );
        assert!(
            err.contains(needle),
            "expected diagnostic containing `{needle}`, got `{err}`"
        );
    }
}
