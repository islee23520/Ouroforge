//! Plugin asset metadata extension (#748).
//!
//! Lets a plugin declaratively extend asset metadata descriptions — naming the
//! asset type, the metadata fields it contributes, display labels, validation
//! hints, and the manifest integration keys those fields map to. This is a pure
//! descriptor: it adds no asset generation, importer/exporter execution, command
//! hooks, network references, or executable behavior. Validation is side-effect
//! free and fails closed on unsafe definitions.
//!
//! The manifest (#739) carries optional asset metadata descriptors validated
//! here, and the registry (#740) reports them for read-only inspection.

use crate::plugin_evidence::{
    require_allowed_value, require_local_id, require_local_text, require_unique_ids,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Allowlisted asset types a descriptor may annotate.
pub const ALLOWED_ASSET_TYPES: &[&str] = &[
    "sprite",
    "texture",
    "audio",
    "tilemap",
    "material",
    "prefab",
    "font",
    "animation",
];

/// Allowlisted metadata field value types.
pub const ALLOWED_FIELD_TYPES: &[&str] = &["string", "integer", "boolean", "enum", "tags"];

/// Tokens that signal asset generation, importer/exporter execution, command
/// hooks, or other executable behavior. Their presence in a type, field, or key
/// fails closed.
const BLOCKED_TOKENS: &[&str] = &[
    "import",
    "export",
    "generate",
    "hook",
    "command",
    "exec",
    "shell",
    "script",
    "network",
    "fetch",
    "download",
    "upload",
    "convert",
    "transcode",
    "render",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginAssetMetadataDescriptor {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
    #[serde(rename = "assetType")]
    pub asset_type: String,
    pub fields: Vec<PluginAssetMetadataField>,
    #[serde(rename = "manifestIntegrationKeys", default)]
    pub manifest_integration_keys: Vec<String>,
    #[serde(rename = "validationHints", default)]
    pub validation_hints: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginAssetMetadataField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub label: String,
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "allowedValues", default)]
    pub allowed_values: Vec<String>,
    #[serde(rename = "validationHint", default)]
    pub validation_hint: String,
}

impl PluginAssetMetadataDescriptor {
    pub fn validate(&self) -> Result<()> {
        require_local_id("plugin asset metadata descriptorId", &self.descriptor_id)?;
        require_allowed_value(
            "plugin asset metadata assetType",
            &self.asset_type,
            ALLOWED_ASSET_TYPES,
        )?;
        if self.fields.is_empty() {
            return Err(anyhow!("plugin asset metadata fields must not be empty"));
        }
        require_unique_ids(
            "plugin asset metadata fields.name",
            self.fields.iter().map(|field| field.name.as_str()),
        )?;
        for field in &self.fields {
            field.validate()?;
        }
        require_unique_ids(
            "plugin asset metadata manifestIntegrationKeys",
            self.manifest_integration_keys.iter().map(|k| k.as_str()),
        )?;
        for key in &self.manifest_integration_keys {
            require_local_id("plugin asset metadata manifestIntegrationKeys", key)?;
            reject_blocked_token("plugin asset metadata manifestIntegrationKeys", key)?;
        }
        for hint in &self.validation_hints {
            require_local_text("plugin asset metadata validationHints", hint)?;
        }
        require_local_text("plugin asset metadata boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "declarative",
            "read-only",
            "no asset generation",
            "no command",
            "no network",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "plugin asset metadata boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl PluginAssetMetadataField {
    fn validate(&self) -> Result<()> {
        require_local_id("plugin asset metadata field name", &self.name)?;
        reject_blocked_token("plugin asset metadata field name", &self.name)?;
        require_allowed_value(
            "plugin asset metadata field type",
            &self.field_type,
            ALLOWED_FIELD_TYPES,
        )?;
        require_local_text("plugin asset metadata field label", &self.label)?;
        require_unique_ids(
            "plugin asset metadata field allowedValues",
            self.allowed_values.iter().map(|v| v.as_str()),
        )?;
        for value in &self.allowed_values {
            require_local_id("plugin asset metadata field allowedValues", value)?;
        }
        if self.field_type == "enum" && self.allowed_values.is_empty() {
            return Err(anyhow!(
                "plugin asset metadata field `{}` of type enum requires allowedValues",
                self.name
            ));
        }
        if self.field_type != "enum" && !self.allowed_values.is_empty() {
            return Err(anyhow!(
                "plugin asset metadata field `{}` allowedValues are only supported for enum",
                self.name
            ));
        }
        if !self.validation_hint.is_empty() {
            require_local_text(
                "plugin asset metadata field validationHint",
                &self.validation_hint,
            )?;
        }
        Ok(())
    }
}

fn reject_blocked_token(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for token in BLOCKED_TOKENS {
        if lower.contains(token) {
            return Err(anyhow!(
                "{field} `{value}` is blocked: asset generation, import/export, command, or network behavior is not allowed"
            ));
        }
    }
    Ok(())
}

/// Validate a set of asset metadata descriptors: each valid and unique by id.
pub fn validate_descriptors(
    field: &str,
    descriptors: &[PluginAssetMetadataDescriptor],
) -> Result<()> {
    require_unique_ids(field, descriptors.iter().map(|d| d.descriptor_id.as_str()))?;
    for descriptor in descriptors {
        descriptor.validate()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_descriptor() -> serde_json::Value {
        serde_json::json!({
            "descriptorId": "sprite-metadata",
            "assetType": "sprite",
            "fields": [
                { "name": "pivot", "type": "enum", "label": "Pivot", "required": true, "allowedValues": ["center", "bottom"] },
                { "name": "frameCount", "type": "integer", "label": "Frame Count" }
            ],
            "manifestIntegrationKeys": ["sprite_pivot", "sprite_frames"],
            "validationHints": ["pivot must be center or bottom"],
            "boundary": "Declarative read-only asset metadata descriptor with no asset generation, no command execution, and no network access."
        })
    }

    fn parse(value: serde_json::Value) -> Result<PluginAssetMetadataDescriptor> {
        let descriptor: PluginAssetMetadataDescriptor = serde_json::from_value(value)?;
        descriptor.validate()?;
        Ok(descriptor)
    }

    #[test]
    fn valid_descriptor_validates() {
        let descriptor = parse(valid_descriptor()).expect("valid descriptor");
        assert_eq!(descriptor.asset_type, "sprite");
        assert_eq!(descriptor.fields.len(), 2);
    }

    #[test]
    fn rejects_unknown_asset_type() {
        let mut value = valid_descriptor();
        value["assetType"] = serde_json::json!("hologram");
        let err = format!("{:#}", parse(value).expect_err("unknown asset type"));
        assert!(err.contains("not in the v1 allowlist"), "{err}");
    }

    #[test]
    fn rejects_unknown_field_type() {
        let mut value = valid_descriptor();
        value["fields"] = serde_json::json!([{ "name": "x", "type": "blob", "label": "X" }]);
        let err = format!("{:#}", parse(value).expect_err("unknown field type"));
        assert!(err.contains("not in the v1 allowlist"), "{err}");
    }

    #[test]
    fn rejects_importer_hook_field() {
        let mut value = valid_descriptor();
        value["fields"] =
            serde_json::json!([{ "name": "import_hook", "type": "string", "label": "Import" }]);
        let err = format!("{:#}", parse(value).expect_err("importer hook blocked"));
        assert!(err.contains("blocked"), "{err}");
    }

    #[test]
    fn rejects_network_reference_in_hint() {
        let mut value = valid_descriptor();
        value["validationHints"] = serde_json::json!(["see https://example.com/spec"]);
        assert!(parse(value).is_err());
    }

    #[test]
    fn rejects_enum_without_allowed_values() {
        let mut value = valid_descriptor();
        value["fields"] =
            serde_json::json!([{ "name": "pivot", "type": "enum", "label": "Pivot" }]);
        let err = format!("{:#}", parse(value).expect_err("enum needs allowedValues"));
        assert!(err.contains("requires allowedValues"), "{err}");
    }

    #[test]
    fn rejects_missing_boundary_keywords() {
        let mut value = valid_descriptor();
        value["boundary"] = serde_json::json!("A friendly descriptor.");
        let err = format!("{:#}", parse(value).expect_err("boundary keywords"));
        assert!(err.contains("must state"), "{err}");
    }
}
