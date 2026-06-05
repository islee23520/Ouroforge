use ouroforge_core::plugin_evidence::{
    PluginCompatibilityStatus, PluginRegistryEvidenceArtifact, PluginValidationStatus,
};

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
