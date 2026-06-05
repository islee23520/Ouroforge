//! Desktop Packaging Capability Gate v1 (#732).
//!
//! A capability report declares the status of a packaging capability that is not
//! implemented in v1. For desktop packaging it records the future platforms and
//! requirement areas and is validated fail closed so it can never claim the
//! capability is implemented. See `docs/desktop-packaging-capability-gate-v1.md`.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const CAPABILITY_REPORT_SCHEMA_VERSION: &str = "desktop-packaging-capability-report-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityStatus {
    /// Designed for the future; not implemented now.
    Future,
    /// Explicitly design-gated behind a separate issue; not implemented now.
    Gated,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CapabilityReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub capability: String,
    pub status: CapabilityStatus,
    /// Must be false in v1: the capability is not implemented.
    pub implemented: bool,
    pub platforms: Vec<String>,
    pub requirements: Vec<String>,
    pub boundary: String,
}

impl CapabilityReport {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let report: Self =
            serde_json::from_str(input).context("failed to parse Capability Report JSON")?;
        report.validate()?;
        Ok(report)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize capability report")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CAPABILITY_REPORT_SCHEMA_VERSION {
            return Err(anyhow!(
                "capability report schemaVersion must be {CAPABILITY_REPORT_SCHEMA_VERSION}"
            ));
        }
        require_text("capability report capability", &self.capability)?;
        if self.implemented {
            return Err(anyhow!(
                "capability report must not mark a v1-gated capability as implemented"
            ));
        }
        if self.platforms.is_empty() {
            return Err(anyhow!("capability report must list target platforms"));
        }
        if self.requirements.is_empty() {
            return Err(anyhow!("capability report must list requirement areas"));
        }
        for value in self.platforms.iter().chain(self.requirements.iter()) {
            require_text("capability report entry", value)?;
        }
        let boundary = self.boundary.to_ascii_lowercase();
        if !boundary.contains("not implemented") {
            return Err(anyhow!(
                "capability report boundary must state the capability is not implemented"
            ));
        }
        Ok(())
    }

    pub fn is_not_implemented(&self) -> bool {
        !self.implemented
    }
}

fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 512 {
        return Err(anyhow!("{field} must be non-empty text up to 512 bytes"));
    }
    Ok(())
}
