//! Consolidated plugin contract tests.
//!
//! Merged from 10 individual plugin_*_contract.rs files:
//! - plugin_asset_metadata_contract.rs (#748)
//! - plugin_compatibility_contract.rs (#743)
//! - plugin_conflicts_contract.rs (#751)
//! - plugin_evidence_contract.rs
//! - plugin_extension_catalog_contract.rs (#741)
//! - plugin_fixture_pack_contract.rs (#749)
//! - plugin_manifest_contract.rs (#739)
//! - plugin_permission_contract.rs (#742)
//! - plugin_registry_contract.rs (#740)
//! - plugin_threat_model_contract.rs (#750)

// ---------------------------------------------------------------------------
// Shared imports
// ---------------------------------------------------------------------------
use ouroforge_core::plugin_asset_metadata::{
    validate_descriptors, PluginAssetMetadataDescriptor, ALLOWED_ASSET_TYPES, ALLOWED_FIELD_TYPES,
};
use ouroforge_core::plugin_compatibility::{
    evaluate, CURRENT_OUROFORGE_VERSION, SUPPORTED_PLUGIN_SCHEMA_VERSIONS,
};
use ouroforge_core::plugin_compatibility::PluginCompatibilityStatus;
use ouroforge_core::plugin_conflicts::{
    detect_conflicts, PluginConflictKind, PluginConflictSeverity,
};
use ouroforge_core::plugin_evidence::{
    write_plugin_registry_evidence, PluginRegistryEvidenceArtifact,
    PluginValidationStatus,
};
use ouroforge_core::plugin_evidence::PluginCompatibilityStatus as EvidenceCompatibilityStatus;
use ouroforge_core::plugin_extension_catalog::{
    is_allowed, validate_extension_point, ALLOWED_EXTENSION_POINT_IDS, CATALOG,
};
use ouroforge_core::plugin_manifest::{PluginManifest, PLUGIN_MANIFEST_SCHEMA_VERSION};
use ouroforge_core::plugin_permission::{
    is_allowed as permission_is_allowed, validate_permission, validate_permissions,
    ALLOWED_PERMISSIONS,
};
use ouroforge_core::plugin_registry::{
    discover_plugin_registry, discover_plugins_in_dir, PluginRegistryCompatibility,
    PluginRegistryStatus,
};
use ouroforge_core::plugin_threat_model::{checklist_ids, gate};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn fixture_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-fixture-pack-v1")
}

// ===========================================================================
// plugin_asset_metadata_contract.rs (#748)
// ===========================================================================

fn descriptor_json() -> serde_json::Value {
    serde_json::json!({
        "descriptorId": "sprite-metadata",
        "assetType": "sprite",
        "fields": [
            { "name": "pivot", "type": "enum", "label": "Pivot", "allowedValues": ["center", "bottom"] }
        ],
        "manifestIntegrationKeys": ["sprite_pivot"],
        "validationHints": ["pivot must be center or bottom"],
        "boundary": "Declarative read-only asset metadata descriptor with no asset generation, no command execution, and no network access."
    })
}

#[test]
fn allowlists_are_narrow() {
    assert!(ALLOWED_ASSET_TYPES.contains(&"sprite"));
    assert!(ALLOWED_FIELD_TYPES.contains(&"enum"));
    assert!(!ALLOWED_FIELD_TYPES.contains(&"blob"));
}

#[test]
fn valid_descriptor_validates() {
    let descriptor: PluginAssetMetadataDescriptor =
        serde_json::from_value(descriptor_json()).expect("parses");
    validate_descriptors("assetMetadata", &[descriptor]).expect("valid descriptor");
}

#[test]
fn unsafe_definitions_fail_closed() {
    // Importer/exporter hook field name.
    let mut hook = descriptor_json();
    hook["fields"] =
        serde_json::json!([{ "name": "export_target", "type": "string", "label": "Export" }]);
    let descriptor: PluginAssetMetadataDescriptor = serde_json::from_value(hook).expect("parses");
    assert!(validate_descriptors("assetMetadata", &[descriptor]).is_err());

    // Network reference in a validation hint.
    let mut net = descriptor_json();
    net["validationHints"] = serde_json::json!(["download from http://example.com"]);
    let descriptor: PluginAssetMetadataDescriptor = serde_json::from_value(net).expect("parses");
    assert!(validate_descriptors("assetMetadata", &[descriptor]).is_err());
}

#[test]
fn manifest_gates_asset_metadata() {
    let base = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "asset-plugin",
        "name": "Asset Plugin",
        "version": "1.0.0",
        "description": "Declares asset metadata.",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["assetMetadataProvider"],
        "extensionPoints": ["assets.metadata.readOnly"],
        "assetMetadata": [descriptor_json()],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    });
    PluginManifest::from_json_str(&base.to_string()).expect("asset metadata manifest validates");
}

// ===========================================================================
// plugin_compatibility_contract.rs (#743)
// ===========================================================================

#[test]
fn current_contract_constants_are_sane() {
    assert!(!CURRENT_OUROFORGE_VERSION.is_empty());
    assert!(SUPPORTED_PLUGIN_SCHEMA_VERSIONS.contains(&"ouroforge.plugin-manifest.v1"));
}

#[test]
fn compatible_plugin_may_contribute() {
    let report = evaluate("ouroforge.plugin-manifest.v1", "0.1.0", "1.0.0");
    assert_eq!(report.status, PluginCompatibilityStatus::Compatible);
    assert!(report.may_contribute());
}

#[test]
fn incompatible_and_future_plugins_blocked_with_diagnostics() {
    let unsupported = evaluate("ouroforge.plugin-manifest.v9", "0.1.0", "");
    assert_eq!(unsupported.status, PluginCompatibilityStatus::Incompatible);
    assert!(!unsupported.may_contribute());
    assert!(!unsupported.diagnostics.is_empty());

    let future = evaluate("ouroforge.plugin-manifest.v1", "9.0.0", "");
    assert_eq!(future.status, PluginCompatibilityStatus::FutureVersion);
    assert!(!future.may_contribute());
    assert!(future.diagnostics[0].contains("upgrade Ouroforge"));

    let too_old = evaluate("ouroforge.plugin-manifest.v1", "0.0.1", "0.0.2");
    assert_eq!(too_old.status, PluginCompatibilityStatus::Incompatible);
    assert!(!too_old.may_contribute());
}

// ===========================================================================
// plugin_conflicts_contract.rs (#751)
// ===========================================================================

fn conflict_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-conflict-v1")
}

#[test]
fn registry_ordering_is_deterministic() {
    let registry = discover_plugins_in_dir(conflict_root()).expect("discovery");
    let paths: Vec<_> = registry
        .entries
        .iter()
        .map(|entry| entry.manifest_path.clone())
        .collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
}

#[test]
fn duplicate_ids_are_detected_and_fail() {
    let registry = discover_plugins_in_dir(conflict_root()).expect("discovery");
    let report = detect_conflicts(&registry);
    assert!(report.has_failures());

    let plugin_conflict = report
        .conflicts
        .iter()
        .find(|c| c.kind == PluginConflictKind::DuplicatePluginId)
        .expect("duplicate plugin id conflict");
    assert_eq!(plugin_conflict.severity, PluginConflictSeverity::Fail);
    assert_eq!(plugin_conflict.identifier, "fixture-duplicate-plugin");
    assert_eq!(plugin_conflict.plugins.len(), 2);

    let descriptor_conflict = report
        .conflicts
        .iter()
        .find(|c| c.kind == PluginConflictKind::DuplicateDescriptorId)
        .expect("duplicate descriptor id conflict");
    assert_eq!(descriptor_conflict.severity, PluginConflictSeverity::Fail);
    assert_eq!(descriptor_conflict.identifier, "shared-panel-descriptor");
    assert_eq!(
        descriptor_conflict.descriptor_kind.as_deref(),
        Some("dashboardPanel")
    );

    // Detection only reports; the boundary states no resolution/ordering.
    assert!(report.boundary.contains("without resolving"));
}

#[test]
fn clean_registry_has_no_conflicts() {
    // A single-entry discovery (the conflict tree's dup-a alone is not isolable
    // here, so reuse the manifest-v1 valid example which has unique ids).
    let registry = discover_plugins_in_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/plugin-discovery-v1/plugins/read-only-dashboard-panel"),
    )
    .expect("discovery");
    let report = detect_conflicts(&registry);
    assert!(report.is_clean());
}

// ===========================================================================
// plugin_evidence_contract.rs
// ===========================================================================

fn valid_fixture() -> &'static str {
    include_str!("../../../examples/plugin-registry-evidence-v1/valid/plugin-registry.sample.json")
}

fn invalid_fixture() -> &'static str {
    include_str!("../../../examples/plugin-registry-evidence-v1/invalid/executable-capability.json")
}

fn unsafe_dashboard_panel_fixture() -> &'static str {
    include_str!("../../../examples/plugin-registry-evidence-v1/invalid/unsafe-dashboard-panel-template.json")
}

fn scenario_template_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/valid/scenario-template-plugin.sample.json"
    )
}

fn scenario_coverage_v16_success_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/valid/scenario-coverage-v16-success-matrix.sample.json"
    )
}

fn scenario_coverage_v16_blocked_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/valid/scenario-coverage-v16-blocked-matrix.sample.json"
    )
}

fn unsafe_scenario_template_fixture() -> &'static str {
    include_str!("../../../examples/plugin-registry-evidence-v1/invalid/unsafe-scenario-template-network.json")
}

fn unsafe_scenario_template_executable_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/invalid/unsafe-scenario-template-executable-script.json"
    )
}

fn unsafe_scenario_template_command_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/invalid/unsafe-scenario-template-command-hook.json"
    )
}

fn unsafe_scenario_template_source_mutation_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/invalid/unsafe-scenario-template-source-mutation.json"
    )
}

fn unsafe_scenario_template_parameter_fixture() -> &'static str {
    include_str!(
        "../../../examples/plugin-registry-evidence-v1/invalid/unsafe-scenario-template-parameter-escape.json"
    )
}

fn scenario_coverage_v16_doc() -> &'static str {
    include_str!("../../../docs/scenario-coverage-v16-plugin-extension.md")
}

fn fixture_value() -> serde_json::Value {
    serde_json::from_str(valid_fixture()).expect("plugin registry fixture parses")
}

fn parse_value(value: serde_json::Value) -> Result<PluginRegistryEvidenceArtifact, anyhow::Error> {
    PluginRegistryEvidenceArtifact::from_json_str(
        &serde_json::to_string_pretty(&value).expect("fixture serializes"),
    )
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time works")
        .as_millis();
    std::env::temp_dir().join(format!("{prefix}-{}-{millis}", std::process::id()))
}

fn create_run_dir(prefix: &str) -> PathBuf {
    let run_dir = unique_temp_dir(prefix);
    fs::create_dir_all(run_dir.join("evidence")).expect("evidence dir created");
    fs::write(run_dir.join("evidence/index.json"), r#"{"artifacts":[]}"#)
        .expect("evidence index written");
    run_dir
}

#[test]
fn scenario_coverage_v16_defines_plugin_extension_regression_matrix() {
    let doc = scenario_coverage_v16_doc();

    for required in [
        "Issue: #753",
        "PES10.16.valid-manifest",
        "PES10.16.registry-discovery",
        "PES10.16.dashboard-panel",
        "PES10.16.studio-display",
        "PES10.16.scenario-template",
        "PES10.16.asset-metadata-descriptor",
        "PES10.16.compatible-version",
        "PES10.16.evidence-bundle",
        "PES10.16.block-arbitrary-js",
        "PES10.16.block-command-execution",
        "PES10.16.block-dependency-install",
        "PES10.16.block-network-install-update",
        "PES10.16.block-credential-access",
        "PES10.16.block-source-mutation",
        "PES10.16.block-export-publish-deploy",
        "PES10.16.block-path-traversal",
        "PES10.16.block-duplicate-ids",
        "PES10.16.block-incompatible-version",
        "PES10.16.block-native-extension",
        "PES10.16.block-ci-mutation",
        "generated-state",
        "no-executable-plugin",
        "no-network-install",
        "no-command-execution",
        "no-publish/deploy",
        "#1 remains the broad roadmap/governance anchor",
        "#23 remains the protected",
    ] {
        assert!(
            doc.contains(required),
            "Scenario Coverage v16 doc missing required scenario/boundary: {required}"
        );
    }

    let non_goals = doc
        .split("## Explicit non-goals")
        .nth(1)
        .expect("Scenario Coverage v16 non-goals section exists");
    for forbidden_scope in [
        "executable plugins",
        "arbitrary JavaScript",
        "native extensions",
        "plugin install/update from network",
        "dependency installation",
        "credential access",
        "shell command execution",
        "browser command bridges",
        "source mutation",
        "CI/workflow mutation",
        "export/publish/deploy mutation",
        "production-ready plugin ecosystem claims",
        "secure plugin sandbox claims",
        "Godot-equivalent extension parity",
        "current Godot replacement claims",
    ] {
        assert!(
            non_goals.contains(forbidden_scope),
            "Scenario Coverage v16 non-goals must retain {forbidden_scope}"
        );
    }
}

#[test]
fn scenario_coverage_v16_success_fixture_covers_allowed_plugin_descriptors() {
    let artifact =
        PluginRegistryEvidenceArtifact::from_json_str(scenario_coverage_v16_success_fixture())
            .expect("Scenario Coverage v16 success fixture validates");

    assert_eq!(
        artifact.registry_id,
        "plugin-registry-scenario-coverage-v16-success"
    );
    assert_eq!(artifact.plugins.len(), 4);
    assert!(artifact.generated_state.fixture_scoped);
    assert!(artifact
        .generated_state
        .tracked_policy
        .contains("generated runtime outputs remain ignored"));

    let read_model = artifact.read_model();
    assert_eq!(read_model.status, "valid");
    assert_eq!(read_model.plugin_count, 4);
    assert_eq!(read_model.blocked_count, 0);
    for expected in [
        "v16-valid-dashboard-panel:dashboardPanel",
        "v16-valid-studio-inspector:studioInspectorPanel",
        "v16-valid-scenario-template:scenarioTemplate",
        "v16-valid-asset-metadata:assetMetadataProvider",
    ] {
        assert!(
            read_model
                .capability_summary
                .iter()
                .any(|item| item == expected),
            "missing capability summary {expected}"
        );
    }
    for expected in [
        "v16-valid-dashboard-panel:dashboard.panels.readOnly",
        "v16-valid-studio-inspector:studio.inspector.readOnly",
        "v16-valid-scenario-template:scenario.templates.readOnly",
        "v16-valid-asset-metadata:assets.metadata.readOnly",
    ] {
        assert!(
            read_model
                .extension_point_summary
                .iter()
                .any(|item| item == expected),
            "missing extension point summary {expected}"
        );
    }
    assert!(read_model.dashboard_panel_summary.iter().any(
        |item| item == "v16-valid-dashboard-panel:v16-plugin-summary:pluginRegistrySummaryCard"
    ));

    let scenario_plugin = artifact
        .plugins
        .iter()
        .find(|plugin| plugin.plugin_id == "v16-valid-scenario-template")
        .expect("scenario template plugin exists");
    let template = &scenario_plugin.scenario_templates[0];
    assert_eq!(template.template_id, "v16-collect-goal-smoke");
    assert_eq!(template.expected_evidence_type, "scenarioPack");
    assert!(template.boundary.contains("no executable scripts"));
    assert!(template.boundary.contains("no source mutation hooks"));
}

#[test]
fn scenario_coverage_v16_blocked_fixture_keeps_unsafe_requests_visible_without_authority() {
    let artifact =
        PluginRegistryEvidenceArtifact::from_json_str(scenario_coverage_v16_blocked_fixture())
            .expect("Scenario Coverage v16 blocked fixture validates as evidence");

    assert_eq!(
        artifact.registry_id,
        "plugin-registry-scenario-coverage-v16-blocked"
    );
    assert_eq!(artifact.plugins.len(), 7);
    assert!(artifact.generated_state.fixture_scoped);

    let read_model = artifact.read_model();
    assert_eq!(read_model.status, "blocked");
    assert_eq!(read_model.plugin_count, 7);
    assert_eq!(read_model.blocked_count, 7);
    for expected in [
        "blocked-process-runner:requested local process execution outside the v1 declarative catalog",
        "blocked-package-installer:requested package installation outside the v1 declarative catalog",
        "blocked-secret-reader:requested secret access outside the v1 declarative catalog",
        "blocked-release-actor:requested publish deploy signing upload authority outside the v1 declarative catalog",
        "blocked-binary-loader:requested binary module loading outside the v1 declarative catalog",
        "blocked-ci-workflow-writer:requested workflow configuration mutation outside the v1 declarative catalog",
        "blocked-incompatible-version:unsupported manifest version for v1 compatibility",
    ] {
        assert!(
            read_model
                .blocked_reasons
                .iter()
                .any(|reason| reason == expected),
            "missing blocked reason {expected}"
        );
    }
    assert!(read_model.boundary.contains("without executing plugins"));
    assert!(read_model.boundary.contains("installing dependencies"));
    assert!(read_model.boundary.contains("publishing"));
    assert!(read_model.boundary.contains("deploying"));
}

#[test]
fn scenario_coverage_v16_rejects_additional_unsafe_fixture_drift() {
    let base: serde_json::Value = serde_json::from_str(scenario_coverage_v16_success_fixture())
        .expect("Scenario Coverage v16 success fixture parses");
    let cases = [
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["declaredCapabilities"] =
                    serde_json::json!(["dependencyInstall"]);
                value
            },
            "declaredCapabilities value `dependencyInstall` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["declaredCapabilities"] = serde_json::json!(["publishDeploy"]);
                value
            },
            "declaredCapabilities value `publishDeploy` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["declaredCapabilities"] =
                    serde_json::json!(["ciWorkflowMutation"]);
                value
            },
            "declaredCapabilities value `ciWorkflowMutation` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["declaredCapabilities"] =
                    serde_json::json!(["nativeExtension"]);
                value
            },
            "declaredCapabilities value `nativeExtension` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["blockedReasons"] =
                    serde_json::json!(["requested credential access"]);
                value["plugins"][0]["validationStatus"] = serde_json::json!("blocked");
                value["plugins"][0]["compatibilityStatus"] = serde_json::json!("incompatible");
                value
            },
            "credential",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][1]["pluginId"] = value["plugins"][0]["pluginId"].clone();
                value
            },
            "must be unique",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["evidenceRefs"][0]["path"] =
                    serde_json::json!("runs/plugin-registry/../escape.json");
                value
            },
            "without traversal",
        ),
    ];

    for (value, expected) in cases {
        let error = parse_value(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

#[test]
fn plugin_registry_evidence_accepts_fixture_and_read_model() {
    let artifact = PluginRegistryEvidenceArtifact::from_json_str(valid_fixture())
        .expect("plugin registry evidence validates");

    assert_eq!(artifact.registry_id, "plugin-registry-fixture");
    assert_eq!(artifact.plugins.len(), 2);
    assert_eq!(
        artifact.plugins[0].validation_status,
        PluginValidationStatus::Valid
    );
    assert_eq!(
        artifact.plugins[1].compatibility_status,
        EvidenceCompatibilityStatus::Incompatible
    );
    assert!(artifact.boundary.contains("no executable plugin"));

    let read_model = artifact.read_model();
    assert_eq!(
        read_model.schema_version,
        "ouroforge.plugin-registry-evidence-read-model.v1"
    );
    assert_eq!(read_model.status, "blocked");
    assert_eq!(read_model.plugin_count, 2);
    assert_eq!(read_model.blocked_count, 1);
    assert!(read_model
        .capability_summary
        .iter()
        .any(|summary| summary == "read-only-dashboard-panel:dashboardPanel"));
    assert!(read_model
        .extension_point_summary
        .iter()
        .any(|summary| summary == "read-only-dashboard-panel:dashboard.panels.readOnly"));
    assert!(read_model.boundary.contains("without executing plugins"));
    assert!(read_model
        .boundary
        .contains("allowlisted dashboard panel descriptors"));
    assert!(read_model.boundary.contains("writing trusted files"));
    assert!(read_model
        .dashboard_panel_summary
        .iter()
        .any(|summary| summary
            == "read-only-dashboard-panel:plugin-registry-summary:pluginRegistrySummaryCard"));
}

#[test]
fn plugin_registry_fixture_records_descriptor_evidence_fields() {
    let artifact = PluginRegistryEvidenceArtifact::from_json_str(valid_fixture())
        .expect("plugin registry evidence validates");

    let dashboard_panel = artifact
        .plugins
        .iter()
        .find(|plugin| plugin.plugin_id == "read-only-dashboard-panel")
        .expect("read-only dashboard fixture plugin exists");
    assert_eq!(
        dashboard_panel.manifest_hash,
        "fnv1a64-canonical-json-v1:1111222233334444"
    );
    assert_eq!(
        dashboard_panel.validation_status,
        PluginValidationStatus::Valid
    );
    assert_eq!(
        dashboard_panel.compatibility_status,
        EvidenceCompatibilityStatus::Compatible
    );
    assert_eq!(dashboard_panel.declared_capabilities, ["dashboardPanel"]);
    assert_eq!(
        dashboard_panel.extension_points,
        ["dashboard.panels.readOnly"]
    );
    assert_eq!(dashboard_panel.evidence_refs.len(), 1);
    assert_eq!(
        dashboard_panel.evidence_refs[0].path,
        "runs/plugin-registry-fixture/plugin-evidence/read-only-dashboard-panel.validation.json"
    );
    assert_eq!(dashboard_panel.dashboard_panels.len(), 1);
    let descriptor = &dashboard_panel.dashboard_panels[0];
    assert_eq!(descriptor.panel_id, "plugin-registry-summary");
    assert_eq!(descriptor.title, "Plugin registry summary");
    assert_eq!(descriptor.data_source_key, "pluginRegistry.summary");
    assert_eq!(descriptor.template_ref, "pluginRegistrySummaryCard");
    assert_eq!(descriptor.layout_hint, "summary");
    assert_eq!(descriptor.display_hints, ["compact", "blocked-count"]);
    assert!(descriptor.boundary.contains("no JavaScript"));
    assert!(dashboard_panel.blocked_reasons.is_empty());

    let blocked_panel = artifact
        .plugins
        .iter()
        .find(|plugin| plugin.plugin_id == "blocked-command-panel")
        .expect("blocked fixture plugin remains visible as evidence");
    assert_eq!(
        blocked_panel.manifest_hash,
        "fnv1a64-canonical-json-v1:aaaabbbbccccdddd"
    );
    assert_eq!(
        blocked_panel.validation_status,
        PluginValidationStatus::Blocked
    );
    assert_eq!(
        blocked_panel.compatibility_status,
        EvidenceCompatibilityStatus::Incompatible
    );
    assert_eq!(
        blocked_panel.declared_capabilities,
        ["studioInspectorPanel"]
    );
    assert_eq!(
        blocked_panel.extension_points,
        ["studio.inspector.readOnly"]
    );
    assert!(blocked_panel
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("executable command authority")));

    let read_model = artifact.read_model();
    assert_eq!(read_model.status, "blocked");
    assert!(read_model.blocked_reasons.iter().any(|reason| reason
        == "blocked-command-panel:manifest requested executable command authority outside the v1 declarative catalog"));
}

#[test]
fn plugin_registry_evidence_rejects_executable_or_unsafe_descriptors() {
    let error = PluginRegistryEvidenceArtifact::from_json_str(invalid_fixture())
        .expect_err("executable capability is blocked");
    assert!(
        format!("{error:?}").contains("not in the v1 allowlist"),
        "{error:?}"
    );

    let cases = [
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["manifestPath"] = serde_json::json!("../plugin.json");
                value
            },
            "manifestPath must stay inside the local project tree",
        ),
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["validationStatus"] = serde_json::json!("blocked");
                value["plugins"][0]["blockedReasons"] = serde_json::json!([]);
                value
            },
            "blocked status requires blockedReasons",
        ),
        (
            {
                let mut value = fixture_value();
                value["boundary"] = serde_json::json!("declarative only");
                value
            },
            "boundary must state `no executable plugin`",
        ),
    ];

    for (value, expected) in cases {
        let error = parse_value(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }

    let error = PluginRegistryEvidenceArtifact::from_json_str(unsafe_dashboard_panel_fixture())
        .expect_err("remote template ref is blocked");
    assert!(format!("{error:?}").contains("https://"), "{error:?}");

    let hostile_panel_cases = [
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["dashboardPanels"][0]["title"] =
                    serde_json::json!("<script>alert(1)</script>");
                value
            },
            "<script",
        ),
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["dashboardPanels"][0]["dataSourceKey"] =
                    serde_json::json!("javascript:alert(1)");
                value
            },
            "javascript:",
        ),
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["dashboardPanels"][0]["templateRef"] =
                    serde_json::json!("https://example.com/remote-panel.js");
                value
            },
            "https://",
        ),
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["dashboardPanels"][0]["layoutHint"] =
                    serde_json::json!("onclick=alert(1)");
                value
            },
            "onclick=",
        ),
        (
            {
                let mut value = fixture_value();
                value["plugins"][0]["dashboardPanels"][0]["templateRef"] =
                    serde_json::json!("commandHook");
                value
            },
            "templateRef value `commandHook` is not in the v1 allowlist",
        ),
    ];

    for (value, expected) in hostile_panel_cases {
        let error = parse_value(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }

    let mut missing_panel = fixture_value();
    missing_panel["plugins"][0]["dashboardPanels"] = serde_json::json!([]);
    let error = parse_value(missing_panel).expect_err("dashboard extension requires descriptor");
    assert!(
        format!("{error:?}")
            .contains("dashboard.panels.readOnly requires at least one dashboardPanels descriptor"),
        "{error:?}"
    );
}

#[test]
fn plugin_registry_scenario_template_descriptor_accepts_fixture_shape() {
    let artifact = PluginRegistryEvidenceArtifact::from_json_str(scenario_template_fixture())
        .expect("scenario template plugin registry fixture validates");
    assert_eq!(artifact.plugins.len(), 1);
    let plugin = &artifact.plugins[0];
    assert_eq!(plugin.plugin_id, "read-only-scenario-template");
    assert_eq!(plugin.declared_capabilities, ["scenarioTemplate"]);
    assert_eq!(plugin.extension_points, ["scenario.templates.readOnly"]);
    assert_eq!(plugin.scenario_templates.len(), 1);

    let template = &plugin.scenario_templates[0];
    assert_eq!(template.template_id, "collect-goal-smoke");
    assert_eq!(template.expected_evidence_type, "scenarioPack");
    assert_eq!(template.supported_game_types, ["platformer", "prototype"]);
    assert_eq!(template.tags, ["qa-smoke", "gdd-prototype"]);
    assert_eq!(template.parameters.len(), 2);
    assert_eq!(template.parameters[0].name, "goalId");
    assert!(template.parameters[0].required);
    assert_eq!(template.parameters[1].parameter_type, "enum");
    assert_eq!(
        template.parameters[1].allowed_values,
        ["easy", "normal", "hard"]
    );
    assert!(template.boundary.contains("no executable scripts"));
    assert!(template.boundary.contains("no source mutation hooks"));
}

#[test]
fn plugin_registry_scenario_template_descriptor_rejects_unsafe_shapes() {
    let error = PluginRegistryEvidenceArtifact::from_json_str(unsafe_scenario_template_fixture())
        .expect_err("network scenario template hint is blocked");
    assert!(format!("{error:?}").contains("https://"), "{error:?}");

    for (fixture, expected) in [
        (unsafe_scenario_template_executable_fixture(), "<script"),
        (unsafe_scenario_template_command_fixture(), "shell command"),
        (
            unsafe_scenario_template_source_mutation_fixture(),
            "boundary must state `no source mutation`",
        ),
        (
            unsafe_scenario_template_parameter_fixture(),
            "bounded local id",
        ),
    ] {
        let error = PluginRegistryEvidenceArtifact::from_json_str(fixture).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }

    let base: serde_json::Value =
        serde_json::from_str(scenario_template_fixture()).expect("scenario fixture parses");
    let cases = [
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["scenarioTemplates"][0]["description"] =
                    serde_json::json!("<script>alert(1)</script>");
                value
            },
            "<script",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["scenarioTemplates"][0]["parameters"][0]["name"] =
                    serde_json::json!("../escape");
                value
            },
            "bounded local id",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["scenarioTemplates"][0]["parameters"][1]["type"] =
                    serde_json::json!("script");
                value
            },
            "parameter type value `script` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["scenarioTemplates"][0]["expectedEvidenceType"] =
                    serde_json::json!("sourceMutation");
                value
            },
            "expectedEvidenceType value `sourceMutation` is not in the v1 allowlist",
        ),
        (
            {
                let mut value = base.clone();
                value["plugins"][0]["scenarioTemplates"] = serde_json::json!([]);
                value
            },
            "scenario.templates.readOnly requires at least one scenarioTemplates descriptor",
        ),
    ];

    for (value, expected) in cases {
        let error = parse_value(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

#[test]
fn plugin_registry_evidence_writer_persists_generated_evidence_without_execution() {
    let run_dir = create_run_dir("ouroforge-plugin-registry-evidence-writer");
    let artifact = PluginRegistryEvidenceArtifact::from_json_str(valid_fixture())
        .expect("plugin registry evidence validates");

    let evidence = write_plugin_registry_evidence(&run_dir, &artifact)
        .expect("plugin registry evidence writes");
    assert_eq!(
        evidence.path,
        "evidence/plugins/plugin-registry-fixture.json"
    );
    assert_eq!(evidence.metadata["artifact"], "plugin_registry_evidence");
    assert_eq!(evidence.metadata["pluginCount"], 2);
    assert_eq!(evidence.metadata["blockedCount"], 1);
    assert!(evidence.metadata["boundary"]
        .as_str()
        .expect("boundary string")
        .contains("no plugins were executed"));

    let written_path = run_dir.join(&evidence.path);
    let written: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&written_path).expect("written plugin evidence reads"),
    )
    .expect("written plugin evidence parses");
    assert_eq!(
        written["schemaVersion"],
        "ouroforge.plugin-registry-evidence.v1"
    );
    assert_eq!(
        written["plugins"].as_array().expect("plugins array").len(),
        2
    );
    assert!(
        !Path::new("plugins/read-only-dashboard-panel/plugin.json").is_absolute(),
        "fixture manifest paths remain relative descriptors"
    );

    let index: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_dir.join("evidence/index.json")).expect("index reads"),
    )
    .expect("index parses");
    assert_eq!(index["artifacts"].as_array().expect("artifacts").len(), 1);
    assert_eq!(index["artifacts"][0]["path"], evidence.path);

    fs::remove_file(&written_path).expect("simulate missing generated artifact with indexed id");
    let duplicate = write_plugin_registry_evidence(&run_dir, &artifact)
        .expect_err("duplicate evidence id is blocked before rewriting evidence");
    assert!(format!("{duplicate:?}").contains("id already exists"));
    assert!(
        !written_path.exists(),
        "duplicate indexed evidence is rejected before rewriting the generated artifact"
    );

    fs::remove_dir_all(run_dir).ok();
}

// ===========================================================================
// plugin_extension_catalog_contract.rs (#741)
// ===========================================================================

#[test]
fn catalog_is_narrow_and_read_only() {
    assert_eq!(CATALOG.len(), 6);
    for id in ALLOWED_EXTENSION_POINT_IDS {
        assert!(is_allowed(id));
        assert!(id.ends_with(".readOnly"), "catalog point must be read-only");
        validate_extension_point(id).expect("catalog point validates");
    }
}

#[test]
fn unknown_and_blocked_points_fail_closed() {
    assert!(validate_extension_point("dashboard.panels.preview").is_err());
    for blocked in [
        "source.write.now",
        "command.exec.run",
        "release.deploy.now",
        "runtime.script.inject",
        "native.dylib.load",
    ] {
        let err = validate_extension_point(blocked)
            .expect_err("blocked point fails")
            .to_string();
        assert!(err.contains("blocked"), "{err}");
    }
}

#[test]
fn manifest_validation_consumes_catalog() {
    // A manifest declaring an out-of-catalog extension point fails closed.
    let manifest = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "catalog-consumer",
        "name": "Catalog Consumer",
        "version": "1.0.0",
        "description": "Declares an out-of-catalog extension point.",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly", "dashboard.panels.preview"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    })
    .to_string();
    assert!(PluginManifest::from_json_str(&manifest).is_err());
}

// ===========================================================================
// plugin_fixture_pack_contract.rs (#749)
// ===========================================================================

#[test]
fn fixture_pack_exercises_valid_and_blocked_plugins() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    let model = registry.read_model();

    // Three valid plugins, three invalid, one incompatible.
    assert_eq!(model.valid_count, 3, "{:?}", model);
    assert_eq!(model.invalid_count, 3, "{:?}", model);
    assert_eq!(model.incompatible_count, 1, "{:?}", model);
    assert_eq!(model.blocked_count, 0);

    // Valid plugins contribute descriptors (capabilities/extension points).
    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Valid)
    {
        assert!(
            !entry.declared_capabilities.is_empty() && !entry.extension_points.is_empty(),
            "valid plugin `{}` must contribute descriptors",
            entry.plugin_id
        );
    }

    // Invalid plugins produce diagnostics and contribute nothing.
    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Invalid)
    {
        assert!(
            !entry.validation_errors.is_empty(),
            "invalid plugin `{}` must report diagnostics",
            entry.plugin_id
        );
        assert!(entry.extension_points.is_empty());
    }

    // Read-only rendering boundary is stated for downstream surfaces.
    let boundary = model.boundary.to_ascii_lowercase();
    assert!(boundary.contains("read-only"));
    assert!(boundary.contains("no plugin execution"));
}

#[test]
fn fixture_pack_capability_and_asset_summaries_are_present() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    let model = registry.read_model();
    assert!(model
        .capability_summary
        .iter()
        .any(|c| c.contains("dashboardPanel")));
    assert!(model
        .asset_metadata_summary
        .iter()
        .any(|a| a.contains("fixture-sprite-metadata")));
}

// ===========================================================================
// plugin_manifest_contract.rs (#739)
// ===========================================================================

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

// ===========================================================================
// plugin_permission_contract.rs (#742)
// ===========================================================================

#[test]
fn allowed_permissions_are_read_or_contribute_only() {
    assert_eq!(ALLOWED_PERMISSIONS.len(), 7);
    for permission in ALLOWED_PERMISSIONS {
        assert!(permission_is_allowed(permission));
        assert!(
            permission.starts_with("read_") || permission.starts_with("contribute_"),
            "permission `{permission}` must be read- or contribute-only"
        );
        validate_permission(permission).expect("allowed permission validates");
    }
}

#[test]
fn blocked_permissions_fail_closed() {
    for blocked in [
        "write_source",
        "run_command",
        "install_dependency",
        "publish_export",
        "access_credentials",
        "network_access",
        "mutate_ci",
        "native_extension",
        "execute_script",
    ] {
        let err = validate_permission(blocked)
            .expect_err("blocked permission fails")
            .to_string();
        assert!(err.contains("blocked"), "{err}");
    }
}

#[test]
fn empty_and_valid_permission_sets() {
    validate_permissions("permissions", &[]).expect("empty set valid");
    validate_permissions(
        "permissions",
        &["read_docs".to_string(), "read_evidence".to_string()],
    )
    .expect("valid set");
}

#[test]
fn manifest_enforces_permission_allowlist() {
    let base = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "perm-consumer",
        "name": "Permission Consumer",
        "version": "1.0.0",
        "description": "Declares permissions.",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    });

    let mut allowed = base.clone();
    allowed["permissions"] = serde_json::json!(["read_docs", "contribute_dashboard_panel"]);
    PluginManifest::from_json_str(&allowed.to_string()).expect("allowed permissions validate");

    let mut blocked = base;
    blocked["permissions"] = serde_json::json!(["read_docs", "execute_script"]);
    assert!(PluginManifest::from_json_str(&blocked.to_string()).is_err());
}

// ===========================================================================
// plugin_registry_contract.rs (#740)
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/plugin-discovery-v1")
}

#[test]
fn repository_discovery_finds_fixture_plugins() {
    let registry = discover_plugin_registry(repo_root()).expect("repo discovery succeeds");
    let model = registry.read_model();
    assert!(model.valid_count >= 1);
    assert!(model.invalid_count >= 1);
    assert!(model.incompatible_count >= 1);
    // Discovered fixture paths are reported relative to the repo root.
    assert!(registry.entries.iter().any(|entry| entry
        .manifest_path
        .starts_with("examples/plugin-discovery-v1/")));
}

#[test]
fn fixture_tree_reports_expected_states() {
    let registry = discover_plugins_in_dir(fixture_root()).expect("fixture discovery succeeds");

    let valid = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "read-only-dashboard-panel")
        .expect("valid fixture present");
    assert_eq!(valid.validation_status, PluginRegistryStatus::Valid);
    assert_eq!(valid.declared_capabilities, ["dashboardPanel"]);
    // Declared permissions are reported in the registry output (#742).
    assert!(valid.permissions.contains(&"read_docs".to_string()));
    assert!(valid
        .permissions
        .contains(&"contribute_dashboard_panel".to_string()));
    assert!(valid
        .manifest_hash
        .starts_with("fnv1a64-canonical-json-v1:"));

    let incompatible = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "legacy-schema-plugin")
        .expect("incompatible fixture present");
    assert_eq!(
        incompatible.validation_status,
        PluginRegistryStatus::Incompatible
    );

    let invalid = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "broken-capability-plugin")
        .expect("invalid fixture present");
    assert_eq!(invalid.validation_status, PluginRegistryStatus::Invalid);
    assert!(!invalid.validation_errors.is_empty());

    // Asset metadata descriptors are reported for read-only inspection (#748).
    let asset = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "read-only-asset-metadata")
        .expect("asset metadata fixture present");
    assert_eq!(asset.validation_status, PluginRegistryStatus::Valid);
    assert!(asset
        .asset_metadata_descriptors
        .contains(&"sprite-pivot-metadata".to_string()));

    // A structurally valid manifest that requires a newer engine is reported as
    // future-version and blocked from extension contribution (#743).
    let future = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "future-engine-plugin")
        .expect("future-version fixture present");
    assert_eq!(future.validation_status, PluginRegistryStatus::Incompatible);
    assert_eq!(
        future.compatibility_status,
        PluginRegistryCompatibility::FutureVersion
    );
    assert!(future.extension_points.is_empty());
    assert!(future
        .validation_errors
        .iter()
        .any(|d| d.contains("upgrade Ouroforge")));
}

#[test]
fn discovery_refuses_generated_or_evidence_scan_root() {
    // A generated/evidence root used as the scan base would strip its own name from
    // every reported manifest path, so a plugin manifest under generated state could be
    // reported as valid/ok. Discovery must fail closed for such roots. (#752)
    let base = std::env::temp_dir().join(format!(
        "ouroforge-plugin-generated-root-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    for generated in ["evidence", "runs", "dashboard-data", ".omx"] {
        let scan_root = base.join(generated);
        let nested = scan_root.join("nested");
        std::fs::create_dir_all(&nested).expect("create nested dir");
        std::fs::write(
            nested.join("sneaky.plugin.json"),
            r#"{"schemaVersion":"ouroforge.plugin-manifest.v1","id":"sneaky","kind":"dashboardPanel"}"#,
        )
        .expect("write manifest");

        let err = discover_plugins_in_dir(&scan_root)
            .expect_err("generated/evidence scan root is rejected");
        assert!(
            err.to_string().contains(generated) && err.to_string().contains("generated"),
            "expected generated-root rejection for `{generated}`, got: {err}"
        );
    }
    std::fs::remove_dir_all(&base).ok();
}

#[test]
fn discovery_refuses_relative_descendant_of_generated_root() {
    // A relative scan base whose first repo-relative component is a generated root
    // (e.g. `evidence/nested`) descends into generated state and must fail closed,
    // even though the base's final component is an ordinary name. The refusal fires
    // before any filesystem access, so no fixture tree is required. (#1378)
    for generated in ["evidence", "runs", "dashboard-data", ".omx"] {
        let scan_root = std::path::Path::new(generated).join("nested");
        let err = discover_plugins_in_dir(&scan_root)
            .expect_err("relative descendant of a generated root is rejected");
        assert!(
            err.to_string().contains(generated) && err.to_string().contains("generated"),
            "expected generated-root rejection for `{generated}/nested`, got: {err}"
        );
    }
}

#[test]
fn discovery_allows_plugins_dir_under_generated_named_ancestor() {
    // Generated roots are defined relative to the repository root, so an unrelated
    // absolute ancestor whose name happens to be `evidence` must not disqualify a
    // legitimate plugins directory nested beneath it. (#752 follow-up)
    let scan_root = std::env::temp_dir()
        .join(format!(
            "ouroforge-evidence-ancestor-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ))
        .join("evidence")
        .join("work")
        .join("plugins");
    std::fs::create_dir_all(&scan_root).expect("create nested plugins dir");
    std::fs::write(
        scan_root.join("panel.plugin.json"),
        r#"{"schemaVersion":"ouroforge.plugin-manifest.v1","id":"panel","kind":"dashboardPanel"}"#,
    )
    .expect("write manifest");

    let registry = discover_plugins_in_dir(&scan_root)
        .expect("plugins dir under a generated-named ancestor is allowed");
    assert_eq!(registry.entries.len(), 1);

    // Clean up the unique top-level temp dir.
    if let Some(top) = scan_root.ancestors().nth(3) {
        std::fs::remove_dir_all(top).ok();
    }
}

// ===========================================================================
// plugin_threat_model_contract.rs (#750)
// ===========================================================================

#[test]
fn gate_and_checklist_are_complete() {
    gate().expect("threat model gate passes");
    assert_eq!(checklist_ids().len(), 12);
}

#[test]
fn privileged_permissions_fail_closed() {
    for permission in [
        "native_extension",
        "install_dependency",
        "access_credentials",
        "write_source",
        "publish_export",
        "mutate_ci",
        "run_command",
        "network_access",
        "execute_script",
    ] {
        assert!(
            validate_permission(permission).is_err(),
            "permission `{permission}` must fail closed"
        );
    }
}

#[test]
fn privileged_extension_points_fail_closed() {
    for point in [
        "source.write.now",
        "command.exec.run",
        "release.publish.now",
        "ci.workflow.mutate",
        "native.dylib.load",
    ] {
        assert!(
            validate_extension_point(point).is_err(),
            "extension point `{point}` must fail closed"
        );
    }
}

#[test]
fn network_reference_in_manifest_fails_closed() {
    let manifest = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "network-plugin",
        "name": "Network Plugin",
        "version": "1.0.0",
        "description": "fetch from https://example.com/payload",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    })
    .to_string();
    assert!(PluginManifest::from_json_str(&manifest).is_err());
}

#[test]
fn high_risk_fixtures_are_blocked_in_discovery() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    // arbitrary-js, blocked-capability, and unsafe-path are reported invalid;
    // none of them contribute extension points.
    for plugin_id in [
        "fixture-arbitrary-js",
        "fixture-blocked-capability",
        "fixture-unsafe-path",
    ] {
        let entry = registry
            .entries
            .iter()
            .find(|entry| entry.plugin_id == plugin_id)
            .unwrap_or_else(|| panic!("fixture `{plugin_id}` present"));
        assert_eq!(entry.validation_status, PluginRegistryStatus::Invalid);
        assert!(entry.extension_points.is_empty());
        assert!(!entry.validation_errors.is_empty());
    }
}
