//! Mutation Risk-Tier Classifier v1 (#1477, #1 Era E Milestone 22 Trust Gradient).
//!
//! Classifies each mutation proposal into a risk tier and decides whether it is
//! ever eligible for bounded auto-apply. Only low-risk data/scene mutations that
//! pass all four gates at high confidence with fresh refs are auto-apply
//! eligible; everything else — and anything ambiguous, missing, or
//! source-affecting — resolves conservatively to manual-only. It classifies
//! eligibility only; it applies nothing and runs nothing.
//!
//! Authorized by the Trust Gradient design gate (docs/trust-gradient-design.md),
//! which records the GO decision, the T0/T1/T2 risk-tier model, and the mandatory
//! safety properties. This module owns tier assignment for that model; its
//! default for any uncertainty is the highest applicable risk (never eligible).

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION: &str = "trust-gradient-risk-tier-v1";

/// Inclusive high-confidence threshold. A proposal with confidence below this
/// value (or with no confidence at all) is never auto-apply eligible.
pub const TRUST_GRADIENT_HIGH_CONFIDENCE_THRESHOLD: f64 = 0.9;

const BOUNDARY: &str = "descriptive risk classification: eligibility only, not quality, \
no auto-apply, no auto-merge, no self-approval; Rust/local owned, browser read-only";

/// Path fragments that mark a candidate as source-affecting (T2) regardless of
/// the declared mutation kind. Fail-closed: if a scope path matches any of
/// these, the proposal can never be auto-apply eligible.
const SOURCE_AFFECTING_FRAGMENTS: &[&str] = &[
    "src/",
    "crates/",
    "build.rs",
    "scripts/",
    ".github/",
    "cargo.toml",
    "cargo.lock",
    "package.json",
    "package-lock.json",
    "makefile",
    ".sh",
    ".rs",
    ".yml",
    ".yaml",
    ".toml",
];

/// Declared kind of a mutation proposal. Only [`MutationKind::SceneOnlyData`] is
/// ever a candidate for auto-apply; every other kind is review-required (T1) or
/// source-affecting/ambiguous (T2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutationKind {
    /// Low-risk scene-only data edit within the existing scene-only contract.
    SceneOnlyData,
    /// Data change whose blast radius the four gates do not fully verify.
    SceneTransition,
    Manifest,
    ScenarioPack,
    PromotionMatrix,
    /// Source-affecting / high-risk kinds — never auto-apply.
    SourceCode,
    BuildScript,
    CiWorkflow,
    Dependency,
    RepoHistory,
    /// Cannot be conservatively classified — treated as highest risk.
    Ambiguous,
    /// Unrecognized kind — fail closed to highest risk. `#[serde(other)]` makes
    /// this the catch-all so a new or misspelled `mutationKind` deserializes to
    /// `Unknown` (classified `RiskTier::High` / `ManualOnly`) instead of failing
    /// to parse before it can be classified.
    #[serde(other)]
    Unknown,
}

impl MutationKind {
    /// Tier implied by the declared kind before scope/confidence/gate checks.
    fn base_tier(self) -> RiskTier {
        match self {
            MutationKind::SceneOnlyData => RiskTier::Low,
            MutationKind::SceneTransition
            | MutationKind::Manifest
            | MutationKind::ScenarioPack
            | MutationKind::PromotionMatrix => RiskTier::Medium,
            MutationKind::SourceCode
            | MutationKind::BuildScript
            | MutationKind::CiWorkflow
            | MutationKind::Dependency
            | MutationKind::RepoHistory
            | MutationKind::Ambiguous
            | MutationKind::Unknown => RiskTier::High,
        }
    }
}

/// Outcome of a single gate on rerun. Anything other than [`GateOutcome::Pass`]
/// disqualifies auto-apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GateOutcome {
    Pass,
    Fail,
    Missing,
    Stale,
}

impl GateOutcome {
    fn is_pass(self) -> bool {
        matches!(self, GateOutcome::Pass)
    }
}

/// The four-gate verdict (mechanical, runtime, visual, semantic) for a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrustGradientGateVerdicts {
    pub mechanical: GateOutcome,
    pub runtime: GateOutcome,
    pub visual: GateOutcome,
    pub semantic: GateOutcome,
}

impl TrustGradientGateVerdicts {
    fn all_pass(&self) -> bool {
        self.mechanical.is_pass()
            && self.runtime.is_pass()
            && self.visual.is_pass()
            && self.semantic.is_pass()
    }
}

/// Risk tier assigned to a proposal. Mirrors the T0/T1/T2 model in the design
/// gate (Low = T0, Medium = T1, High = T2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
}

impl RiskTier {
    /// Highest (most conservative) of two tiers.
    fn max(self, other: RiskTier) -> RiskTier {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }

    fn rank(self) -> u8 {
        match self {
            RiskTier::Low => 0,
            RiskTier::Medium => 1,
            RiskTier::High => 2,
        }
    }
}

/// Whether a proposal is ever eligible for bounded auto-apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AutoApplyEligibility {
    AutoApplyEligible,
    ManualOnly,
}

/// Input descriptor for a mutation proposal to be classified.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MutationProposalDescriptor {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "mutationKind")]
    pub mutation_kind: MutationKind,
    #[serde(rename = "scopePaths", default)]
    pub scope_paths: Vec<String>,
    /// Decision confidence in `[0, 1]`. Absent confidence is never eligible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub gates: TrustGradientGateVerdicts,
    /// Whether all referenced evidence is current. Absent/false is treated as a
    /// stale ref (conservative, never eligible).
    #[serde(rename = "refsFresh", default)]
    pub refs_fresh: bool,
}

impl MutationProposalDescriptor {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let descriptor: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse mutation proposal descriptor: {err}"))?;
        descriptor.validate()?;
        Ok(descriptor)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        if self.proposal_ref.trim().is_empty() {
            return Err(anyhow!("proposalRef must not be empty"));
        }
        if let Some(confidence) = self.confidence {
            if !(0.0..=1.0).contains(&confidence) {
                return Err(anyhow!("confidence must be within [0, 1]: {confidence}"));
            }
        }
        Ok(())
    }

    fn has_source_affecting_path(&self) -> bool {
        self.scope_paths.iter().any(|path| {
            let lowered = path.to_ascii_lowercase();
            SOURCE_AFFECTING_FRAGMENTS
                .iter()
                .any(|fragment| lowered.contains(fragment))
        })
    }
}

/// Result of classifying a mutation proposal.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RiskTierClassification {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub tier: RiskTier,
    pub eligibility: AutoApplyEligibility,
    pub reasons: Vec<String>,
    pub boundary: String,
}

/// Classify a mutation proposal into a risk tier and auto-apply eligibility.
///
/// Fail-closed: a proposal is [`AutoApplyEligibility::AutoApplyEligible`] (and
/// tier [`RiskTier::Low`]) only when every condition holds — kind is
/// scene-only data, no scope path is source-affecting, confidence is present
/// and at least the high-confidence threshold, all four gates pass, and refs are
/// fresh. Any failure resolves to [`AutoApplyEligibility::ManualOnly`] at the
/// most conservative applicable tier.
pub fn classify_mutation_risk_tier(
    descriptor: &MutationProposalDescriptor,
) -> Result<RiskTierClassification> {
    descriptor.validate()?;

    let mut reasons: Vec<String> = Vec::new();
    let mut tier = descriptor.mutation_kind.base_tier();
    let mut eligible = matches!(descriptor.mutation_kind, MutationKind::SceneOnlyData);

    if !eligible {
        reasons.push(format!(
            "mutation kind {:?} is not low-risk scene-only data",
            descriptor.mutation_kind
        ));
    }

    if descriptor.has_source_affecting_path() {
        eligible = false;
        tier = tier.max(RiskTier::High);
        reasons.push("scope contains a source-affecting path".to_string());
    }

    match descriptor.confidence {
        None => {
            eligible = false;
            tier = tier.max(RiskTier::Medium);
            reasons.push("confidence is missing".to_string());
        }
        Some(confidence) if confidence < TRUST_GRADIENT_HIGH_CONFIDENCE_THRESHOLD => {
            eligible = false;
            tier = tier.max(RiskTier::Medium);
            reasons.push(format!(
                "confidence {confidence} below high-confidence threshold {TRUST_GRADIENT_HIGH_CONFIDENCE_THRESHOLD}"
            ));
        }
        Some(_) => {}
    }

    if !descriptor.gates.all_pass() {
        eligible = false;
        tier = tier.max(RiskTier::Medium);
        reasons.push("not all four gates pass on rerun".to_string());
    }

    if !descriptor.refs_fresh {
        eligible = false;
        tier = tier.max(RiskTier::Medium);
        reasons.push("evidence refs are stale or unverified".to_string());
    }

    let eligibility = if eligible {
        // Eligibility implies the lowest tier; defense in depth keeps it Low.
        tier = RiskTier::Low;
        reasons.push(
            "low-risk scene-only data, source-free scope, high confidence, all gates pass, fresh refs"
                .to_string(),
        );
        AutoApplyEligibility::AutoApplyEligible
    } else {
        AutoApplyEligibility::ManualOnly
    };

    Ok(RiskTierClassification {
        schema_version: TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION.to_string(),
        proposal_ref: descriptor.proposal_ref.clone(),
        tier,
        eligibility,
        reasons,
        boundary: BOUNDARY.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_gates() -> TrustGradientGateVerdicts {
        TrustGradientGateVerdicts {
            mechanical: GateOutcome::Pass,
            runtime: GateOutcome::Pass,
            visual: GateOutcome::Pass,
            semantic: GateOutcome::Pass,
        }
    }

    fn eligible_descriptor() -> MutationProposalDescriptor {
        MutationProposalDescriptor {
            schema_version: TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION.to_string(),
            proposal_ref: "proposal-low".to_string(),
            mutation_kind: MutationKind::SceneOnlyData,
            scope_paths: vec!["examples/demo-game/scenes/level-1.json".to_string()],
            confidence: Some(0.95),
            gates: passing_gates(),
            refs_fresh: true,
        }
    }

    #[test]
    fn low_risk_scene_only_is_eligible() {
        let classification = classify_mutation_risk_tier(&eligible_descriptor()).unwrap();
        assert_eq!(classification.tier, RiskTier::Low);
        assert_eq!(
            classification.eligibility,
            AutoApplyEligibility::AutoApplyEligible
        );
    }

    #[test]
    fn missing_confidence_is_manual_only() {
        let mut descriptor = eligible_descriptor();
        descriptor.confidence = None;
        let classification = classify_mutation_risk_tier(&descriptor).unwrap();
        assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    }

    #[test]
    fn source_affecting_scope_is_high_manual() {
        let mut descriptor = eligible_descriptor();
        descriptor.scope_paths = vec!["crates/ouroforge-core/src/lib.rs".to_string()];
        let classification = classify_mutation_risk_tier(&descriptor).unwrap();
        assert_eq!(classification.tier, RiskTier::High);
        assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    }

    #[test]
    fn gate_regression_is_manual_only() {
        let mut descriptor = eligible_descriptor();
        descriptor.gates.semantic = GateOutcome::Fail;
        let classification = classify_mutation_risk_tier(&descriptor).unwrap();
        assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    }

    #[test]
    fn stale_refs_are_manual_only() {
        let mut descriptor = eligible_descriptor();
        descriptor.refs_fresh = false;
        let classification = classify_mutation_risk_tier(&descriptor).unwrap();
        assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    }

    #[test]
    fn ambiguous_kind_is_high_manual() {
        let mut descriptor = eligible_descriptor();
        descriptor.mutation_kind = MutationKind::Ambiguous;
        let classification = classify_mutation_risk_tier(&descriptor).unwrap();
        assert_eq!(classification.tier, RiskTier::High);
        assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    }
}
