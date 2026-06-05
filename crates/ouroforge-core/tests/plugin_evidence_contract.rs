use ouroforge_core::plugin_evidence::{
    write_plugin_registry_evidence, PluginCompatibilityStatus, PluginRegistryEvidenceArtifact,
    PluginValidationStatus,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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
        PluginCompatibilityStatus::Incompatible
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
        PluginCompatibilityStatus::Compatible
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
        PluginCompatibilityStatus::Incompatible
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
