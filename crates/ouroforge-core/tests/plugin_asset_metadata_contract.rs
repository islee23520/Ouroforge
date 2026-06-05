//! Contract tests for the plugin asset metadata extension (#748).
//!
//! Confirms valid asset metadata descriptors validate, unsafe definitions
//! (generation/import/export hooks, command hooks, network references) fail
//! closed, and the manifest carries and gates asset metadata descriptors. No
//! asset generation/import/export execution is introduced.

use ouroforge_core::plugin_asset_metadata::{
    validate_descriptors, PluginAssetMetadataDescriptor, ALLOWED_ASSET_TYPES, ALLOWED_FIELD_TYPES,
};
use ouroforge_core::plugin_manifest::PluginManifest;

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
