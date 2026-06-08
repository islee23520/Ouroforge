//! Reviewer/Critic Promotion Gates v1 (#1678).
//!
//! This module records reviewer/critic promotion gates as inert local evidence.
//! It reuses the existing review/critic gate concept and the Milestone 22 trust
//! gradient (risk tiers `low`/`medium`/`high`); it does **not** add a new
//! orchestration engine, a new bypass, or any auto-apply authority. A gate never
//! applies, merges, or promotes anything: it classifies whether a proposal is
//! `blocked` or, at most, `promote-allowed` (cleared to route through the
//! existing review/apply/trust-gradient path, never auto-applied).
//!
//! Each gate binds a proposal to an implementer role, a reviewer role, and a
//! critic role (all distinct actors — no self-approval), a trust-gradient risk
//! tier, and the reviewer/critic decisions with audit evidence. The outcome is
//! computed deterministically and fails closed:
//!
//! - a critic `veto` blocks promotion regardless of the reviewer decision;
//! - promotion is blocked until the reviewer `approve`s (pending/reject block);
//! - `medium`/`high` risk additionally requires a critic `approve` (a pending
//!   critic blocks); higher risk requires stronger review;
//! - otherwise the gate is `promote-allowed` — cleared to route through
//!   review/apply, still never auto-applied.
//!
//! Reviewer/critic decisions are audited in the read model. A human retains the
//! release go/no-go.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCTION_REVIEW_GATES_SCHEMA_VERSION: &str = "production-review-gates-v1";

/// The Milestone 13 role set, mirrored to keep the module self-contained while
/// staying consistent with the `AgentRoleModel` role universe in `lib.rs`.
const SUPPORTED_ROLES: &[&str] = &[
    "designer",
    "gameplay-engineer",
    "level-designer",
    "asset-import-planner",
    "qa-agent",
    "performance-regression-agent",
    "reviewer",
    "critic",
    "build-release-candidate-agent",
];

/// Trust-gradient risk tiers (the Milestone 22 T0/T1/T2 model).
const RISK_TIERS: &[&str] = &["low", "medium", "high"];

/// Reviewer decisions.
const REVIEWER_DECISIONS: &[&str] = &["pending", "approve", "reject"];

/// Critic decisions.
const CRITIC_DECISIONS: &[&str] = &["pending", "approve", "veto"];

/// Gate outcomes. Neither auto-applies; `promote-allowed` only clears the
/// proposal to route through the existing review/apply/trust-gradient path.
const GATE_OUTCOMES: &[&str] = &["blocked", "promote-allowed"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionReviewGateLedger {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    pub gates: Vec<ProductionReviewGate>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: ProductionReviewGatesDashboardCompat,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionReviewGate {
    #[serde(rename = "gateId")]
    pub gate_id: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    /// The role that produced the proposal.
    #[serde(rename = "implementerRole")]
    pub implementer_role: String,
    /// The independent reviewer role; distinct from implementer and critic.
    #[serde(rename = "reviewerRole")]
    pub reviewer_role: String,
    /// The independent critic role; distinct from implementer and reviewer.
    #[serde(rename = "criticRole")]
    pub critic_role: String,
    /// Trust-gradient risk tier: `low` / `medium` / `high`.
    #[serde(rename = "riskTier")]
    pub risk_tier: String,
    /// `pending` / `approve` / `reject`.
    #[serde(rename = "reviewerDecision")]
    pub reviewer_decision: String,
    /// `pending` / `approve` / `veto`.
    #[serde(rename = "criticDecision")]
    pub critic_decision: String,
    /// Audit evidence ref for the reviewer decision.
    #[serde(rename = "reviewerEvidenceRef")]
    pub reviewer_evidence_ref: String,
    /// Audit evidence ref for the critic decision.
    #[serde(rename = "criticEvidenceRef")]
    pub critic_evidence_ref: String,
    /// `blocked` / `promote-allowed`; must match the computed outcome.
    pub outcome: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionReviewGatesDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A deterministic audit record for a single promotion gate.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateAuditRecord {
    #[serde(rename = "gateId")]
    pub gate_id: String,
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    #[serde(rename = "riskTier")]
    pub risk_tier: String,
    #[serde(rename = "implementerRole")]
    pub implementer_role: String,
    #[serde(rename = "reviewerRole")]
    pub reviewer_role: String,
    #[serde(rename = "reviewerDecision")]
    pub reviewer_decision: String,
    #[serde(rename = "criticRole")]
    pub critic_role: String,
    #[serde(rename = "criticDecision")]
    pub critic_decision: String,
    pub outcome: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionReviewGatesReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    #[serde(rename = "gateCount")]
    pub gate_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "promoteAllowedCount")]
    pub promote_allowed_count: usize,
    #[serde(rename = "vetoCount")]
    pub veto_count: usize,
    /// The full audit trail (one record per gate), deterministically sorted.
    pub audit: Vec<GateAuditRecord>,
    /// The blocked subset (the fail-closed evidence), deterministically sorted.
    pub blocked: Vec<GateAuditRecord>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProductionReviewGateLedger {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let ledger: Self = serde_json::from_str(input)
            .context("failed to parse Production Review Gate Ledger JSON")?;
        ledger.validate()?;
        Ok(ledger)
    }

    /// Deterministically classifies a gate, failing closed. A critic veto blocks
    /// unconditionally; promotion is blocked until the reviewer approves; and
    /// medium/high risk additionally requires a critic approval.
    pub fn computed_outcome(&self, gate: &ProductionReviewGate) -> (String, String) {
        if gate.critic_decision == "veto" {
            return (
                "blocked".to_string(),
                format!(
                    "gate `{}` blocked: critic `{}` vetoed promotion",
                    gate.gate_id, gate.critic_role
                ),
            );
        }
        if gate.reviewer_decision != "approve" {
            return (
                "blocked".to_string(),
                format!(
                    "gate `{}` blocked until reviewed: reviewer decision is `{}`, not approve",
                    gate.gate_id, gate.reviewer_decision
                ),
            );
        }
        if matches!(gate.risk_tier.as_str(), "medium" | "high") && gate.critic_decision != "approve"
        {
            return (
                "blocked".to_string(),
                format!(
                    "gate `{}` blocked: `{}` risk requires an explicit critic approval, but critic decision is `{}`",
                    gate.gate_id, gate.risk_tier, gate.critic_decision
                ),
            );
        }
        (
            "promote-allowed".to_string(),
            format!(
                "gate `{}` promote-allowed: cleared to route through review/apply under the `{}` trust gradient, never auto-applied",
                gate.gate_id, gate.risk_tier
            ),
        )
    }

    fn audit_record(&self, gate: &ProductionReviewGate) -> GateAuditRecord {
        let (outcome, reason) = self.computed_outcome(gate);
        GateAuditRecord {
            gate_id: gate.gate_id.clone(),
            artifact_class: gate.artifact_class.clone(),
            risk_tier: gate.risk_tier.clone(),
            implementer_role: gate.implementer_role.clone(),
            reviewer_role: gate.reviewer_role.clone(),
            reviewer_decision: gate.reviewer_decision.clone(),
            critic_role: gate.critic_role.clone(),
            critic_decision: gate.critic_decision.clone(),
            outcome,
            reason,
        }
    }

    /// Builds the deterministic, order-independent audit read model.
    pub fn read_model(&self) -> ProductionReviewGatesReadModel {
        let mut audit: Vec<GateAuditRecord> =
            self.gates.iter().map(|g| self.audit_record(g)).collect();
        audit.sort();
        let blocked: Vec<GateAuditRecord> = audit
            .iter()
            .filter(|r| r.outcome == "blocked")
            .cloned()
            .collect();
        let veto_count = audit.iter().filter(|r| r.critic_decision == "veto").count();
        let promote_allowed_count = audit.len() - blocked.len();

        ProductionReviewGatesReadModel {
            schema_version: self.schema_version.clone(),
            milestone: self.milestone.clone(),
            gate_count: self.gates.len(),
            blocked_count: blocked.len(),
            promote_allowed_count,
            veto_count,
            audit,
            blocked,
            validation_summary: vec![
                "each gate binds a proposal to distinct implementer, reviewer, and critic roles (no self-approval) and a trust-gradient risk tier".to_string(),
                "promotion is blocked until the reviewer approves and the critic does not veto; medium/high risk additionally requires an explicit critic approval (higher risk, stronger review)".to_string(),
                "outcomes are deterministic and fail closed; promote-allowed only clears the proposal to route through review/apply and never auto-applies".to_string(),
            ],
            compatibility_notes: vec![
                "reuses the existing review/critic gate concept and the Milestone 22 trust gradient; no new orchestration engine and no reviewer bypass".to_string(),
                "non-mutating read model with no auto-apply, auto-merge, self-approval, or trusted mutation authority".to_string(),
                "promotion flows only through the existing review/apply/trust-gradient path and a human release gate".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize production review gates read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_REVIEW_GATES_SCHEMA_VERSION {
            return Err(anyhow!(
                "production review gates schemaVersion must be {PRODUCTION_REVIEW_GATES_SCHEMA_VERSION}"
            ));
        }
        require_text("production review gates milestone", &self.milestone)?;

        require_nonempty("production review gates gates", self.gates.len())?;
        if self.gates.len() > 512 {
            return Err(anyhow!(
                "production review gates ledger is overbroad for v1"
            ));
        }
        let mut gate_ids = BTreeSet::new();
        for gate in &self.gates {
            gate.validate()?;
            if !gate_ids.insert(gate.gate_id.as_str()) {
                return Err(anyhow!(
                    "production review gates gate id `{}` is duplicated",
                    gate.gate_id
                ));
            }
            let (computed_outcome, _) = self.computed_outcome(gate);
            if gate.outcome != computed_outcome {
                return Err(anyhow!(
                    "production review gates gate `{}` outcome `{}` does not match computed outcome `{computed_outcome}`",
                    gate.gate_id,
                    gate.outcome
                ));
            }
        }

        self.dashboard_compat.validate()?;

        require_text("production review gates boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "promotion",
            "review",
            "trust gradient",
            "fail closed",
            "read-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "production review gates boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl ProductionReviewGate {
    fn validate(&self) -> Result<()> {
        require_id("production review gates gateId", &self.gate_id)?;
        require_ref("production review gates proposalRef", &self.proposal_ref)?;
        require_label(
            "production review gates artifactClass",
            &self.artifact_class,
        )?;
        require_role(
            "production review gates implementerRole",
            &self.implementer_role,
        )?;
        require_role("production review gates reviewerRole", &self.reviewer_role)?;
        require_role("production review gates criticRole", &self.critic_role)?;
        // The implementer and the reviewer/critic must be distinct actors, and
        // the reviewer and critic are independent of each other (no self-approval).
        if self.reviewer_role == self.implementer_role {
            return Err(anyhow!(
                "production review gates gate `{}` reviewerRole must differ from implementerRole (no self-approval)",
                self.gate_id
            ));
        }
        if self.critic_role == self.implementer_role {
            return Err(anyhow!(
                "production review gates gate `{}` criticRole must differ from implementerRole (no self-approval)",
                self.gate_id
            ));
        }
        if self.critic_role == self.reviewer_role {
            return Err(anyhow!(
                "production review gates gate `{}` criticRole must differ from reviewerRole (independent gates)",
                self.gate_id
            ));
        }
        if !RISK_TIERS.contains(&self.risk_tier.as_str()) {
            return Err(anyhow!(
                "production review gates gate `{}` riskTier `{}` is unsupported",
                self.gate_id,
                self.risk_tier
            ));
        }
        if !REVIEWER_DECISIONS.contains(&self.reviewer_decision.as_str()) {
            return Err(anyhow!(
                "production review gates gate `{}` reviewerDecision `{}` is unsupported",
                self.gate_id,
                self.reviewer_decision
            ));
        }
        if !CRITIC_DECISIONS.contains(&self.critic_decision.as_str()) {
            return Err(anyhow!(
                "production review gates gate `{}` criticDecision `{}` is unsupported",
                self.gate_id,
                self.critic_decision
            ));
        }
        require_ref(
            "production review gates reviewerEvidenceRef",
            &self.reviewer_evidence_ref,
        )?;
        require_ref(
            "production review gates criticEvidenceRef",
            &self.critic_evidence_ref,
        )?;
        if !GATE_OUTCOMES.contains(&self.outcome.as_str()) {
            return Err(anyhow!(
                "production review gates gate `{}` outcome `{}` is unsupported",
                self.gate_id,
                self.outcome
            ));
        }
        Ok(())
    }
}

impl ProductionReviewGatesDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "production review gates dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text(
            "production review gates dashboardCompat.surface",
            &self.surface,
        )?;
        require_nonempty(
            "production review gates dashboardCompat.columns",
            self.columns.len(),
        )?;
        for column in &self.columns {
            require_plain_text("production review gates dashboardCompat.columns", column)?;
        }
        Ok(())
    }
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_role(field: &str, value: &str) -> Result<()> {
    require_label(field, value)?;
    if !SUPPORTED_ROLES.contains(&value) {
        return Err(anyhow!(
            "{field} `{value}` is not a supported Milestone 13 role"
        ));
    }
    Ok(())
}

fn require_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!("{field} must be a bounded local id"));
    }
    Ok(())
}

fn require_label(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!("{field} must be a bounded local label"));
    }
    Ok(())
}

fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("data:") {
        return Err(anyhow!("{field} remote refs are not allowed"));
    }
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} contains forbidden traversal and must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/")
        || value.starts_with("evidence/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, runs/, or evidence/ refs"
        ));
    }
    Ok(())
}

/// Non-empty text with the forbidden-authority scan (negation-aware).
fn require_text(field: &str, value: &str) -> Result<()> {
    require_plain_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic code loading",
        "command bridge",
        "local server bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "auto-fix",
        "self-approval",
        "reviewer bypass",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "quality guarantee",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden production review gates authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

/// Non-empty trimmed text, no authority scan.
fn require_plain_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 8] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
        "not yet ",
        "never ",
    ];
    // Terms allowed to sit between a negation token and the forbidden phrase so
    // a negated list such as `no auto-apply or self-approval` keeps every item
    // negated, while an unrelated negation such as `no restrictions allow
    // auto-merge` does NOT suppress the positive `auto-merge` claim (fail-closed).
    const LIST_FILLER: [&str; 19] = [
        "auto-merge",
        "auto-apply",
        "auto-fix",
        "self-approval",
        "reviewer bypass",
        "browser trusted write",
        "trusted source write",
        "trusted writes",
        "trusted write",
        "command bridge",
        "local server bridge",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "quality guarantee",
        " or ",
        " and ",
        " nor ",
        ",",
    ];
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed).
    const CONTRASTS: [&str; 6] = [
        " but ",
        " however ",
        " yet ",
        " whereas ",
        " nevertheless ",
        " though ",
    ];
    let hay = value;
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let mut clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        if let Some(reset) = CONTRASTS
            .iter()
            .filter_map(|c| {
                hay[clause_start..idx]
                    .rfind(c)
                    .map(|p| clause_start + p + c.len())
            })
            .max()
        {
            clause_start = reset;
        }
        let preceding = &hay[clause_start..idx];
        // A negation covers the phrase only when the gap between the nearest
        // negation token and the phrase contains nothing but other
        // forbidden-authority terms and list connectors.
        let negated = NEGATIONS.iter().any(|neg| {
            preceding.rfind(neg).is_some_and(|pos| {
                let mut between = preceding[pos + neg.len()..].to_string();
                for filler in LIST_FILLER {
                    between = between.replace(filler, " ");
                }
                between.chars().all(char::is_whitespace)
            })
        });
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gate(id: &str, tier: &str, reviewer: &str, critic: &str) -> ProductionReviewGate {
        ProductionReviewGate {
            gate_id: id.to_string(),
            proposal_ref: format!("evidence/proposals/{id}.json"),
            artifact_class: "design-brief".to_string(),
            implementer_role: "designer".to_string(),
            reviewer_role: "reviewer".to_string(),
            critic_role: "critic".to_string(),
            risk_tier: tier.to_string(),
            reviewer_decision: reviewer.to_string(),
            critic_decision: critic.to_string(),
            reviewer_evidence_ref: format!("evidence/review/{id}-reviewer.json"),
            critic_evidence_ref: format!("evidence/review/{id}-critic.json"),
            outcome: "blocked".to_string(),
        }
    }

    fn ledger(gates: Vec<ProductionReviewGate>) -> ProductionReviewGateLedger {
        ProductionReviewGateLedger {
            schema_version: PRODUCTION_REVIEW_GATES_SCHEMA_VERSION.to_string(),
            milestone: "era-h-milestone-42".to_string(),
            gates,
            dashboard_compat: ProductionReviewGatesDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["gateId".to_string(), "outcome".to_string()],
            },
            boundary:
                "Promotion is gated by an independent review and critic under the trust gradient; gates fail closed and stay read-only; cleared proposals route through the review/apply path with no auto-apply, no auto-merge, no reviewer bypass, and no self-approval; a human keeps the release go/no-go."
                    .to_string(),
        }
    }

    /// Reconciles declared outcome with the computed one for the happy path.
    fn reconcile(mut l: ProductionReviewGateLedger) -> ProductionReviewGateLedger {
        let snapshot = l.clone();
        for g in &mut l.gates {
            g.outcome = snapshot.computed_outcome(g).0;
        }
        l
    }

    #[test]
    fn pending_reviewer_blocks_promotion() {
        let l = reconcile(ledger(vec![gate("g1", "low", "pending", "pending")]));
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.blocked_count, 1);
        assert_eq!(read.promote_allowed_count, 0);
        assert!(read.blocked[0].reason.contains("blocked until reviewed"));
    }

    #[test]
    fn critic_veto_blocks_even_with_reviewer_approval() {
        let l = reconcile(ledger(vec![gate("g1", "low", "approve", "veto")]));
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.blocked_count, 1);
        assert_eq!(read.veto_count, 1);
        assert!(read.blocked[0].reason.contains("vetoed promotion"));
    }

    #[test]
    fn low_risk_approve_is_promote_allowed_without_critic() {
        let l = reconcile(ledger(vec![gate("g1", "low", "approve", "pending")]));
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.promote_allowed_count, 1);
        assert!(read.audit[0].reason.contains("never auto-applied"));
    }

    #[test]
    fn high_risk_requires_critic_approval() {
        let blocked = reconcile(ledger(vec![gate("g1", "high", "approve", "pending")]));
        assert_eq!(blocked.read_model().blocked_count, 1);
        let allowed = reconcile(ledger(vec![gate("g1", "high", "approve", "approve")]));
        assert_eq!(allowed.read_model().promote_allowed_count, 1);
    }

    #[test]
    fn declared_outcome_mismatch_fails_closed() {
        let mut l = reconcile(ledger(vec![gate("g1", "low", "pending", "pending")]));
        l.gates[0].outcome = "promote-allowed".to_string();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("does not match computed outcome"));
    }

    #[test]
    fn self_approval_is_rejected() {
        let mut l = reconcile(ledger(vec![gate("g1", "low", "approve", "pending")]));
        l.gates[0].reviewer_role = "designer".to_string(); // same as implementer
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("no self-approval"));
    }

    #[test]
    fn reviewer_and_critic_must_be_independent() {
        let mut l = reconcile(ledger(vec![gate("g1", "medium", "approve", "approve")]));
        l.gates[0].critic_role = "reviewer".to_string();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("independent gates"));
    }

    #[test]
    fn audit_is_order_independent() {
        let forward = reconcile(ledger(vec![
            gate("g1", "low", "approve", "pending"),
            gate("g2", "high", "approve", "veto"),
            gate("g3", "medium", "pending", "pending"),
        ]));
        let mut reversed = forward.clone();
        reversed.gates.reverse();
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }
}
