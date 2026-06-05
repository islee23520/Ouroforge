//! Plugin capability/permission model (#742).
//!
//! Validates plugin-declared permissions against a fail-closed allowlist so a
//! plugin cannot request unsafe powers. Permissions are declarations only:
//! validating them grants no runtime power, performs no I/O, and never loads or
//! executes plugin code. Allowed permissions are read- and contribute-only;
//! every write/execute/network/credential/CI/native category fails closed.
//!
//! The manifest schema (#739) carries an optional `permissions` list validated
//! here, and the registry (#740) reports the declared permissions.

use anyhow::{anyhow, Result};
use std::collections::BTreeSet;

/// Allowed v1 plugin permissions. Read- and contribute-only.
pub const ALLOWED_PERMISSIONS: &[&str] = &[
    "read_docs",
    "read_evidence",
    "read_project_metadata",
    "contribute_scenario_template",
    "contribute_dashboard_panel",
    "contribute_studio_panel",
    "contribute_asset_metadata",
];

/// Explicitly blocked permissions with actionable reasons. These represent the
/// unsafe powers v1 must never grant.
const BLOCKED_PERMISSIONS: &[(&str, &str)] = &[
    ("write_source", "source mutation is not permitted"),
    ("run_command", "command execution is not permitted"),
    (
        "install_dependency",
        "dependency installation is not permitted",
    ),
    ("publish_export", "export/publish/deploy is not permitted"),
    ("access_credentials", "credential access is not permitted"),
    ("network_access", "network access is not permitted"),
    ("mutate_ci", "CI/workflow mutation is not permitted"),
    ("native_extension", "native extension is not permitted"),
    ("execute_script", "script execution is not permitted"),
];

/// Returns true if the permission is in the v1 allowlist.
pub fn is_allowed(permission: &str) -> bool {
    ALLOWED_PERMISSIONS.contains(&permission)
}

/// Validate a single declared permission. Fails closed for blocked permissions
/// (with a specific reason) and for unknown permissions.
pub fn validate_permission(permission: &str) -> Result<()> {
    if is_allowed(permission) {
        return Ok(());
    }
    if let Some((_, reason)) = BLOCKED_PERMISSIONS
        .iter()
        .find(|(name, _)| *name == permission)
    {
        return Err(anyhow!(
            "plugin permission `{permission}` is blocked: {reason}"
        ));
    }
    Err(anyhow!(
        "plugin permission `{permission}` is not in the v1 permission allowlist"
    ))
}

/// Validate a declared permission set: each permission must be allowlisted and
/// unique. An empty set is valid — permissions are optional.
pub fn validate_permissions(field: &str, permissions: &[String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for permission in permissions {
        validate_permission(permission).map_err(|error| anyhow!("{field}: {error}"))?;
        if !seen.insert(permission.as_str()) {
            return Err(anyhow!("{field}: permission `{permission}` must be unique"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_permissions_validate() {
        assert_eq!(ALLOWED_PERMISSIONS.len(), 7);
        for permission in ALLOWED_PERMISSIONS {
            validate_permission(permission).expect("allowed permission validates");
            assert!(is_allowed(permission));
        }
    }

    #[test]
    fn blocked_permissions_fail_with_reason() {
        for (permission, needle) in [
            ("write_source", "source mutation"),
            ("run_command", "command execution"),
            ("install_dependency", "dependency installation"),
            ("publish_export", "export/publish/deploy"),
            ("access_credentials", "credential access"),
            ("network_access", "network access"),
            ("mutate_ci", "CI/workflow mutation"),
            ("native_extension", "native extension"),
            ("execute_script", "script execution"),
        ] {
            let err = validate_permission(permission)
                .expect_err("blocked permission fails")
                .to_string();
            assert!(
                err.contains("blocked") && err.contains(needle),
                "permission `{permission}` expected `{needle}`, got `{err}`"
            );
        }
    }

    #[test]
    fn unknown_permission_fails_closed() {
        let err = validate_permission("super_admin")
            .expect_err("unknown permission fails")
            .to_string();
        assert!(err.contains("not in the v1 permission allowlist"), "{err}");
    }

    #[test]
    fn empty_permission_set_is_valid() {
        validate_permissions("permissions", &[]).expect("empty set valid");
    }

    #[test]
    fn duplicate_permissions_fail() {
        let permissions = vec!["read_docs".to_string(), "read_docs".to_string()];
        let err = validate_permissions("permissions", &permissions)
            .expect_err("duplicate fails")
            .to_string();
        assert!(err.contains("must be unique"), "{err}");
    }

    #[test]
    fn mixed_set_with_blocked_fails() {
        let permissions = vec!["read_docs".to_string(), "run_command".to_string()];
        assert!(validate_permissions("permissions", &permissions).is_err());
    }
}
