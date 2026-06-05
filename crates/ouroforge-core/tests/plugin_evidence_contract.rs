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
    assert!(read_model.boundary.contains("writing trusted files"));
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
