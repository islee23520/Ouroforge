//! Plugin security / threat model gate (#750).
//!
//! Encodes the v1 plugin threat model as data so the documented risks and their
//! fail-closed controls have a single, testable source of truth. This module
//! authorizes nothing: it only enumerates threats and the existing controls that
//! block them, and provides a `gate` that fails if the model drifts (a threat
//! left uncontrolled). The companion document is `docs/plugin-threat-model-v1.md`.
//!
//! v1 is declarative and allowlisted. Executable plugins, marketplace, native
//! extensions, and network install/update are explicitly out of scope and are
//! deferred to separate, explicitly-authorized future work — never enabled here.

use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginThreatStatus {
    /// The risk is rejected outright by a fail-closed validator.
    Blocked,
    /// The risk is structurally prevented by a boundary/contract.
    Mitigated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PluginThreat {
    pub id: &'static str,
    pub risk: &'static str,
    pub control: &'static str,
    pub status: PluginThreatStatus,
}

/// The v1 plugin threat model. Each entry pairs a risk with the existing control
/// that blocks or mitigates it.
pub const THREAT_MODEL: &[PluginThreat] = &[
    PluginThreat {
        id: "untrusted-manifest",
        risk: "An untrusted plugin manifest declares unsupported fields or values.",
        control: "Manifests parse with deny_unknown_fields and fail-closed allowlist validation.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "symlink-escape",
        risk: "A symlinked manifest points outside the plugin tree.",
        control: "Discovery never follows symlinks; symlinked manifests are recorded as blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "path-traversal",
        risk: "A manifest path or reference traverses outside the plugin tree.",
        control: "Path validators reject leading slash, `..`, and backslash separators.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "arbitrary-js",
        risk: "A plugin smuggles arbitrary JavaScript or an executable entry point.",
        control: "Unknown executable fields are rejected and script/eval text fails closed.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "native-extension",
        risk: "A plugin requests native dynamic library / extension loading.",
        control: "The `native_extension` permission and native extension points are blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "dependency-install",
        risk: "A plugin requests dependency installation.",
        control: "The `install_dependency` permission and install extension points are blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "network-install",
        risk: "A plugin references network install/update or remote sources.",
        control: "URL text fails closed; discovery and validation perform no network access.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "credential-access",
        risk: "A plugin requests credential access.",
        control: "The `access_credentials` permission and credential text fail closed.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "source-mutation",
        risk: "A plugin requests source mutation.",
        control: "The `write_source` permission and source/write extension points are blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "export-publish-deploy",
        risk: "A plugin requests export/publish/deploy mutation.",
        control: "The `publish_export` permission and export/publish/deploy points are blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "ci-mutation",
        risk: "A plugin requests CI/workflow mutation.",
        control: "The `mutate_ci` permission and CI/workflow extension points are blocked.",
        status: PluginThreatStatus::Blocked,
    },
    PluginThreat {
        id: "studio-rendering",
        risk: "Studio/dashboard rendering of plugin content becomes code execution.",
        control: "Read models expose read-only descriptors with explicit read-only boundaries.",
        status: PluginThreatStatus::Mitigated,
    },
];

/// Stable ids of every threat in the model, for closure-evidence checklists.
pub fn checklist_ids() -> Vec<&'static str> {
    THREAT_MODEL.iter().map(|threat| threat.id).collect()
}

/// Fail-closed gate: the threat model must be non-empty, every threat must have
/// a non-empty risk/control and a blocked/mitigated status, and ids must be
/// unique. Drift (an uncontrolled or duplicated threat) fails the gate.
pub fn gate() -> Result<()> {
    if THREAT_MODEL.is_empty() {
        return Err(anyhow!("plugin threat model must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for threat in THREAT_MODEL {
        if threat.id.trim().is_empty()
            || threat.risk.trim().is_empty()
            || threat.control.trim().is_empty()
        {
            return Err(anyhow!(
                "plugin threat `{}` must have a non-empty id, risk, and control",
                threat.id
            ));
        }
        if !seen.insert(threat.id) {
            return Err(anyhow!("plugin threat id `{}` must be unique", threat.id));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_passes_for_current_model() {
        gate().expect("threat model gate passes");
    }

    #[test]
    fn model_covers_required_risks() {
        let ids = checklist_ids();
        for required in [
            "untrusted-manifest",
            "symlink-escape",
            "path-traversal",
            "arbitrary-js",
            "native-extension",
            "dependency-install",
            "network-install",
            "credential-access",
            "source-mutation",
            "export-publish-deploy",
            "ci-mutation",
            "studio-rendering",
        ] {
            assert!(ids.contains(&required), "threat model missing `{required}`");
        }
    }

    #[test]
    fn every_threat_is_blocked_or_mitigated() {
        for threat in THREAT_MODEL {
            assert!(matches!(
                threat.status,
                PluginThreatStatus::Blocked | PluginThreatStatus::Mitigated
            ));
        }
    }
}
