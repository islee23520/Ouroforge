//! Godot-Plus demo plugin usage contract (#792).
//!
//! Validates that the Collect and Exit demo uses Plugin / Extension System v1
//! through declarative descriptors only: read-only dashboard panel, read-only
//! scenario template, and read-only asset metadata. Discovery is local and
//! registry/evidence-shaped; it executes no plugin code, installs nothing,
//! contacts no network, and grants no trusted write authority.

use ouroforge_core::plugin_manifest::{PluginManifest, PLUGIN_MANIFEST_SCHEMA_VERSION};
use ouroforge_core::plugin_registry::{discover_plugins_in_dir, PluginRegistryStatus};
use serde_json::Value;
use std::path::PathBuf;

const DASHBOARD_MANIFEST: &str = include_str!(
    "../../../examples/playable-demo-v2/collect-and-exit/plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json"
);
const SCENARIO_MANIFEST: &str = include_str!(
    "../../../examples/playable-demo-v2/collect-and-exit/plugins/collect-and-exit-scenario-template/ouroforge.plugin.json"
);
const ASSET_MANIFEST: &str = include_str!(
    "../../../examples/playable-demo-v2/collect-and-exit/plugins/collect-and-exit-asset-metadata/ouroforge.plugin.json"
);
const PLUGIN_USAGE_EVIDENCE: &str =
    include_str!("../../../examples/playable-demo-v2/collect-and-exit/plugin-usage-evidence.json");

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn plugin_root() -> PathBuf {
    repo_root().join("examples/playable-demo-v2/collect-and-exit/plugins")
}

#[test]
fn demo_plugin_manifests_validate_as_declarative_descriptors() {
    let dashboard = PluginManifest::from_json_str(DASHBOARD_MANIFEST)
        .expect("dashboard plugin manifest validates");
    assert_eq!(dashboard.schema_version, PLUGIN_MANIFEST_SCHEMA_VERSION);
    assert_eq!(dashboard.plugin_id, "collect-and-exit-dashboard-panel");
    assert_eq!(dashboard.declared_capabilities, ["dashboardPanel"]);
    assert_eq!(dashboard.extension_points, ["dashboard.panels.readOnly"]);
    assert!(dashboard.boundary.contains("no executable code"));

    let scenario = PluginManifest::from_json_str(SCENARIO_MANIFEST)
        .expect("scenario template plugin manifest validates");
    assert_eq!(scenario.plugin_id, "collect-and-exit-scenario-template");
    assert_eq!(scenario.declared_capabilities, ["scenarioTemplate"]);
    assert_eq!(scenario.extension_points, ["scenario.templates.readOnly"]);
    assert_eq!(scenario.descriptor_refs.len(), 1);
    assert_eq!(scenario.descriptor_refs[0].kind, "scenarioTemplate");
    assert!(scenario.boundary.contains("no command execution"));

    let asset = PluginManifest::from_json_str(ASSET_MANIFEST)
        .expect("asset metadata plugin manifest validates");
    assert_eq!(asset.plugin_id, "collect-and-exit-asset-metadata");
    assert_eq!(asset.declared_capabilities, ["assetMetadataProvider"]);
    assert_eq!(asset.extension_points, ["assets.metadata.readOnly"]);
    assert_eq!(asset.asset_metadata.len(), 1);
    assert_eq!(
        asset.asset_metadata[0].descriptor_id,
        "collect-and-exit-demo-asset-metadata"
    );
    assert!(asset.boundary.contains("no network install"));
}

#[test]
fn demo_plugin_registry_reports_three_valid_metadata_only_plugins() {
    let registry = discover_plugins_in_dir(plugin_root()).expect("demo plugin registry discovers");
    let model = registry.read_model();
    assert_eq!(model.plugin_count, 3);
    assert_eq!(model.valid_count, 3);
    assert_eq!(model.invalid_count, 0);
    assert_eq!(model.blocked_count, 0);
    assert_eq!(model.incompatible_count, 0);
    assert!(model.boundary.contains("no plugin execution"));
    assert!(model.boundary.contains("no network install"));
    assert!(model.boundary.contains("no trusted writes"));

    for id in [
        "collect-and-exit-dashboard-panel",
        "collect-and-exit-scenario-template",
        "collect-and-exit-asset-metadata",
    ] {
        let entry = registry
            .entries
            .iter()
            .find(|entry| entry.plugin_id == id)
            .unwrap_or_else(|| panic!("missing plugin {id}"));
        assert_eq!(entry.validation_status, PluginRegistryStatus::Valid);
        assert!(entry.validation_errors.is_empty());
        assert!(entry
            .manifest_hash
            .starts_with("fnv1a64-canonical-json-v1:"));
    }

    assert!(model
        .capability_summary
        .contains(&"collect-and-exit-dashboard-panel:dashboardPanel".to_string()));
    assert!(model
        .capability_summary
        .contains(&"collect-and-exit-scenario-template:scenarioTemplate".to_string()));
    assert!(model
        .capability_summary
        .contains(&"collect-and-exit-asset-metadata:assetMetadataProvider".to_string()));
    assert!(model.asset_metadata_summary.contains(
        &"collect-and-exit-asset-metadata:collect-and-exit-demo-asset-metadata".to_string()
    ));
}

#[test]
fn demo_plugin_usage_evidence_records_guardrails_and_governance() {
    let evidence: Value =
        serde_json::from_str(PLUGIN_USAGE_EVIDENCE).expect("plugin usage evidence parses");
    assert_eq!(evidence["schemaVersion"], "demo-plugin-usage-evidence-v1");
    assert_eq!(evidence["issue"], 792);
    assert_eq!(evidence["plugins"].as_array().unwrap().len(), 3);
    assert_eq!(evidence["studioDashboardIntegration"]["trustedWrites"], 0);
    assert_eq!(
        evidence["studioDashboardIntegration"]["browserCommandBridge"],
        false
    );
    for key in [
        "noExecutablePluginRuntime",
        "noMarketplace",
        "noNetworkInstallUpdate",
        "noDependencyInstall",
        "noCredentialedOperation",
        "noPublishDeploySignUpload",
        "noDirectTrustedSourceWrite",
        "noAutoApply",
        "noAutoMerge",
    ] {
        assert_eq!(evidence["guardrails"][key], true, "guardrail {key}");
    }
    assert_eq!(
        evidence["governance"]["protectedIssuesMustRemainOpen"][0],
        1
    );
    assert_eq!(
        evidence["governance"]["protectedIssuesMustRemainOpen"][1],
        23
    );
}
