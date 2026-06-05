//! Release / Publish Blocker v1 (#733).
//!
//! A fail-closed guard that keeps build/export/package work from drifting into
//! release, deployment, or commercial distribution. It scans export
//! configuration JSON (profiles, plans, package configs) for publish-oriented
//! fields and rejects them, and it audits public wording so only local-export
//! claims survive.
//!
//! Safety boundary (preserves the Milestone 15 command/write boundaries): this
//! module performs no command execution, no network access, and no filesystem
//! writes. It is pure policy validation over in-memory JSON and text.

use anyhow::{anyhow, Context, Result};
use serde_json::Value;

/// Boundary statement for the blocker.
pub const EXPORT_RELEASE_BLOCKER_BOUNDARY: &str =
    "Local export/package only; release, publish, deploy, sign, notarize, upload, host, and \
     credentialed/CI-release operations are blocked. This guard does not execute commands, access \
     the network, or write files.";

/// Substrings (matched against lowercased JSON object keys) that indicate a
/// publish/release/deploy/credential field and are blocked.
pub const BLOCKED_PUBLISH_KEY_TERMS: &[&str] = &[
    "publish",
    "deploy",
    "signing",
    "signature",
    "notar",
    "upload",
    "credential",
    "secret",
    "token",
    "apikey",
    "store",
    "steam",
    "itch",
    "hosted",
    "release",
    "visibility",
];

/// Forbidden wording in public/local-export text.
const FORBIDDEN_WORDING: &[&str] = &[
    "production-ready",
    "public release",
    "app store",
    "godot replacement",
    "secure distribution",
    "commercial release",
    "ready to publish",
    "deploy to",
    "deploy-to",
];

/// Scan a JSON value and return the dotted paths of any publish/release fields.
pub fn scan_for_publish_fields(value: &Value) -> Vec<String> {
    let mut hits = Vec::new();
    walk(value, String::new(), &mut hits);
    hits
}

fn walk(value: &Value, path: String, hits: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let key_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                if key_is_blocked(key) {
                    hits.push(key_path.clone());
                }
                walk(child, key_path, hits);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                walk(child, format!("{path}[{index}]"), hits);
            }
        }
        _ => {}
    }
}

fn key_is_blocked(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    BLOCKED_PUBLISH_KEY_TERMS
        .iter()
        .any(|term| lower.contains(term))
}

/// Fail closed if `json_str` contains any publish/release configuration field.
pub fn ensure_no_publish_config(json_str: &str) -> Result<()> {
    let value: Value =
        serde_json::from_str(json_str).context("failed to parse export config JSON")?;
    let hits = scan_for_publish_fields(&value);
    if !hits.is_empty() {
        return Err(anyhow!(
            "export config contains blocked publish/release fields: {}",
            hits.join(", ")
        ));
    }
    Ok(())
}

/// Fail closed if `text` makes a non-local (publish/release/distribution) claim.
pub fn audit_local_export_wording(text: &str) -> Result<()> {
    let lower = text.to_ascii_lowercase();
    for forbidden in FORBIDDEN_WORDING {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "export wording must stay local-export-only (forbidden phrase `{forbidden}`)"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_config_passes() {
        ensure_no_publish_config(r#"{"exportTarget":"web-local","outputDir":"dist/x"}"#).unwrap();
    }

    #[test]
    fn publish_field_is_blocked() {
        let err = ensure_no_publish_config(r#"{"exportTarget":"web-local","publish":true}"#)
            .expect_err("publish blocked");
        assert!(err.to_string().contains("publish"));
    }

    #[test]
    fn nested_credential_field_is_blocked() {
        let hits = scan_for_publish_fields(
            &serde_json::json!({"a": {"b": {"signingKey": "x"}}, "deployTarget": "prod"}),
        );
        assert!(hits.iter().any(|h| h == "a.b.signingKey"));
        assert!(hits.iter().any(|h| h == "deployTarget"));
    }

    #[test]
    fn wording_audit_blocks_release_claims() {
        assert!(audit_local_export_wording("A production-ready public release.").is_err());
        audit_local_export_wording("A local web export for inspection.").unwrap();
    }

    #[test]
    fn wording_audit_blocks_hyphenated_deploy_to() {
        // #733 requires rejecting `deploy-to` claims, not just `deploy to`.
        assert!(audit_local_export_wording("deploy-to production").is_err());
        assert!(audit_local_export_wording("Click to deploy to prod").is_err());
    }
}
