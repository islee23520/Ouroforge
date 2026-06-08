//! Release Compliance Reviewer Gate v1 (#1693, #1 Era H Milestone 44).
//!
//! Adds a release-level compliance gate for content policy, age-rating signals,
//! and asset license/provenance completeness. This composes with the existing
//! reviewer/evaluator gate model by producing a gate verdict; it is not a new
//! evaluator, writer, release authority, or browser/Studio mutation surface.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const RELEASE_COMPLIANCE_GATE_SCHEMA_VERSION: &str = "release-compliance-gate-v1";

const BOUNDARY: &str = "release compliance reviewer gate: Rust/local read-only evidence, composes with existing reviewer/evaluator gates, browser and Studio read-only, no release authority, no auto-merge, no self-approval, no quality/fun or production-ready claim";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplianceVerdictStatus {
    Pass,
    Blocked,
    Malformed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseComplianceGateInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseCandidateRef")]
    pub release_candidate_ref: String,
    #[serde(rename = "policyChecks")]
    pub policy_checks: Vec<PolicyCheck>,
    #[serde(rename = "ageRatingSignals")]
    pub age_rating_signals: AgeRatingSignals,
    #[serde(rename = "assetChecks")]
    pub asset_checks: Vec<AssetComplianceCheck>,
    #[serde(rename = "humanGoNoGo")]
    pub human_go_no_go: HumanGoNoGo,
    #[serde(rename = "generatedState")]
    pub generated_state: GeneratedState,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyCheck {
    pub id: String,
    pub status: CheckStatus,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    #[serde(
        default,
        rename = "blockedReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckStatus {
    Pass,
    Violation,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgeRatingSignals {
    #[serde(rename = "declaredRating")]
    pub declared_rating: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    #[serde(default, rename = "unreviewedSignals")]
    pub unreviewed_signals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetComplianceCheck {
    pub id: String,
    #[serde(rename = "assetRef")]
    pub asset_ref: String,
    pub license: Option<String>,
    #[serde(rename = "provenanceRef")]
    pub provenance_ref: Option<String>,
    #[serde(rename = "qaGateRef")]
    pub qa_gate_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HumanGoNoGo {
    Pending,
    Approved,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedState {
    pub generated: bool,
    pub tracked: bool,
    #[serde(rename = "fixtureScoped")]
    pub fixture_scoped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseComplianceGateVerdict {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseCandidateRef")]
    pub release_candidate_ref: String,
    pub status: ComplianceVerdictStatus,
    pub reasons: Vec<String>,
    #[serde(rename = "composesWith")]
    pub composes_with: Vec<String>,
    #[serde(rename = "humanGoNoGo")]
    pub human_go_no_go: HumanGoNoGo,
    pub boundary: String,
}

impl ReleaseComplianceGateInput {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let parsed: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse release compliance gate input: {err}"))?;
        parsed.validate_shape()?;
        Ok(parsed)
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RELEASE_COMPLIANCE_GATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        require_ref("releaseCandidateRef", &self.release_candidate_ref)?;
        if self.policy_checks.is_empty() {
            return Err(anyhow!("policyChecks must not be empty"));
        }
        if self.asset_checks.is_empty() {
            return Err(anyhow!("assetChecks must not be empty"));
        }
        for check in &self.policy_checks {
            require_id("policy id", &check.id)?;
            require_ref("policy evidenceRef", &check.evidence_ref)?;
            if matches!(check.status, CheckStatus::Violation | CheckStatus::Missing)
                && check
                    .blocked_reason
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
            {
                return Err(anyhow!(
                    "blocked or missing policy checks require blockedReason"
                ));
            }
        }
        if self.age_rating_signals.declared_rating.trim().is_empty() {
            return Err(anyhow!("declaredRating must not be empty"));
        }
        require_ref(
            "age rating evidenceRef",
            &self.age_rating_signals.evidence_ref,
        )?;
        for signal in &self.age_rating_signals.unreviewed_signals {
            if signal.trim().is_empty() {
                return Err(anyhow!("unreviewedSignals must not contain blanks"));
            }
        }
        for asset in &self.asset_checks {
            require_id("asset id", &asset.id)?;
            require_ref("assetRef", &asset.asset_ref)?;
            if let Some(reference) = &asset.provenance_ref {
                require_ref("provenanceRef", reference)?;
            }
            if let Some(reference) = &asset.qa_gate_ref {
                require_ref("qaGateRef", reference)?;
            }
        }
        if self.generated_state.generated
            && self.generated_state.tracked
            && !self.generated_state.fixture_scoped
        {
            return Err(anyhow!(
                "generated compliance evidence may be tracked only when fixture-scoped"
            ));
        }
        let boundary = self.boundary.to_ascii_lowercase();
        for required in ["composes", "read-only", "human", "no release authority"] {
            if !boundary.contains(required) {
                return Err(anyhow!("boundary must state `{required}`"));
            }
        }
        Ok(())
    }
}

pub fn evaluate_release_compliance(
    input: &ReleaseComplianceGateInput,
) -> Result<ReleaseComplianceGateVerdict> {
    input.validate_shape()?;

    let mut reasons = Vec::new();
    for check in &input.policy_checks {
        match check.status {
            CheckStatus::Pass => {}
            CheckStatus::Violation => reasons.push(format!(
                "policy violation `{}`: {}",
                check.id,
                check.blocked_reason.as_deref().unwrap_or("violation")
            )),
            CheckStatus::Missing => reasons.push(format!(
                "policy check `{}` missing: {}",
                check.id,
                check
                    .blocked_reason
                    .as_deref()
                    .unwrap_or("missing evidence")
            )),
        }
    }

    if !input.age_rating_signals.unreviewed_signals.is_empty() {
        reasons.push(format!(
            "age-rating signals require human compliance review: {}",
            input.age_rating_signals.unreviewed_signals.join(", ")
        ));
    }

    for asset in &input.asset_checks {
        if asset
            .license
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            reasons.push(format!("asset `{}` missing license", asset.id));
        }
        if asset
            .provenance_ref
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            reasons.push(format!("asset `{}` missing provenance", asset.id));
        }
        if asset
            .qa_gate_ref
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            reasons.push(format!("asset `{}` missing asset QA gate", asset.id));
        }
    }

    if input.human_go_no_go == HumanGoNoGo::Missing {
        reasons.push("human release go/no-go evidence is missing".to_string());
    }

    let status = if reasons.is_empty() {
        ComplianceVerdictStatus::Pass
    } else {
        ComplianceVerdictStatus::Blocked
    };

    Ok(ReleaseComplianceGateVerdict {
        schema_version: RELEASE_COMPLIANCE_GATE_SCHEMA_VERSION.to_string(),
        release_candidate_ref: input.release_candidate_ref.clone(),
        status,
        reasons,
        composes_with: vec![
            "reviewer-gate".to_string(),
            "evaluator-declared-gate".to_string(),
            "release-provenance-bundle".to_string(),
        ],
        human_go_no_go: input.human_go_no_go,
        boundary: BOUNDARY.to_string(),
    })
}

fn require_id(field: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if valid {
        Ok(())
    } else {
        Err(anyhow!("{field} must be a bounded local id"))
    }
}

fn require_ref(field: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && !value.starts_with('/')
        && !value.contains("..")
        && !value.contains('\\')
        && !value.contains(';')
        && !value.contains("&&")
        && !value.contains('|');
    if valid {
        Ok(())
    } else {
        Err(anyhow!("{field} must be a safe local evidence ref"))
    }
}
