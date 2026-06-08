//! Role Agent Model and Artifact Ownership v1 (#1675).
//!
//! This module realizes the Milestone 13 role model as per-artifact ownership
//! evidence. It does not introduce a new orchestration engine, runtime, writer,
//! scheduler, or worker pool. It reuses the existing agent role set, evidence,
//! and review/apply/trust-gradient surfaces: each role is a specialized
//! capability that *proposes* output and owns a set of artifact classes; a role
//! never performs a direct trusted write.
//!
//! The model records:
//! - role definitions with the artifact classes each role owns (proposal scope);
//! - single-owner artifact ownership assignments (a class has exactly one owning
//!   role; duplicate ownership is a conflict and fails closed, never silently
//!   merged);
//! - role write attempts, each classified deterministically as `authorized`
//!   (a proposal by the owning role) or `rejected` (a direct trusted write, an
//!   unowned class, or a non-owning actor) — unauthorized writes are rejected
//!   fail-closed;
//! - observability records derived from the model for read-only surfaces.
//!
//! The model is inert local evidence. It does not execute commands, spawn
//! agents, apply changes, merge, publish, sign, deploy, auto-approve, or write
//! trusted browser state. Promotion of any proposal flows only through the
//! existing review/apply/trust-gradient path, and a human retains the release
//! go/no-go.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PRODUCTION_ROLES_SCHEMA_VERSION: &str = "production-roles-v1";

/// The Milestone 13 role set, mirrored here to keep the module self-contained
/// while staying consistent with the `AgentRoleModel` role universe in `lib.rs`.
/// No new roles are introduced.
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

/// The kinds of write a role may *attempt*. Only `proposal` can ever be
/// authorized; any direct trusted write is rejected fail-closed.
const WRITE_KINDS: &[&str] = &["proposal", "trusted-write"];

/// The deterministic outcome classifications for a write attempt.
const ATTEMPT_OUTCOMES: &[&str] = &["authorized", "rejected"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionRoleOwnershipModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    pub roles: Vec<ProductionRoleDefinition>,
    pub ownership: Vec<ArtifactOwnershipAssignment>,
    #[serde(default)]
    pub attempts: Vec<RoleWriteAttempt>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: ProductionRolesDashboardCompat,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionRoleDefinition {
    /// One of the Milestone 13 roles (`SUPPORTED_ROLES`).
    pub role: String,
    pub purpose: String,
    /// The artifact classes this role owns (proposal scope only).
    #[serde(rename = "ownsArtifactClasses")]
    pub owns_artifact_classes: Vec<String>,
    /// Must be `true`: no role has trusted-write or merge authority.
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    /// Reuse anchor: a ref to the existing role/evidence artifact for this role.
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    /// Must name the self-approval / auto-apply / hidden-agent boundary, like
    /// the Milestone 13 role model.
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactOwnershipAssignment {
    /// The artifact class owned (e.g. `design-brief`). Single-owner.
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    /// The single owning role; must be a defined role that declares this class.
    #[serde(rename = "ownerRole")]
    pub owner_role: String,
    /// Reuse anchor: a ref to the existing ownership/evidence artifact.
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleWriteAttempt {
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    /// The role attempting the write; must be a defined role.
    #[serde(rename = "actorRole")]
    pub actor_role: String,
    /// `proposal` or `trusted-write`.
    #[serde(rename = "writeKind")]
    pub write_kind: String,
    /// Reuse anchor: a ref to the proposal evidence routed through review/apply.
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    /// `authorized` or `rejected`; must match the computed outcome (fail-closed).
    #[serde(rename = "declaredOutcome")]
    pub declared_outcome: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionRolesDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// An observability record for a single write attempt, derived deterministically
/// from the model and the ownership rules.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttemptObservation {
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    #[serde(rename = "actorRole")]
    pub actor_role: String,
    /// The owning role for the class, or empty when the class is unowned.
    #[serde(rename = "ownerRole")]
    pub owner_role: String,
    #[serde(rename = "writeKind")]
    pub write_kind: String,
    pub outcome: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionRolesReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    #[serde(rename = "roleCount")]
    pub role_count: usize,
    #[serde(rename = "ownershipCount")]
    pub ownership_count: usize,
    #[serde(rename = "attemptCount")]
    pub attempt_count: usize,
    #[serde(rename = "authorizedCount")]
    pub authorized_count: usize,
    #[serde(rename = "rejectedCount")]
    pub rejected_count: usize,
    /// Observability of ownership: each role mapped to its owned artifact classes
    /// (deterministically sorted).
    #[serde(rename = "ownershipByRole")]
    pub ownership_by_role: BTreeMap<String, Vec<String>>,
    /// Every recorded write attempt, deterministically sorted.
    pub observations: Vec<AttemptObservation>,
    /// The rejected subset, deterministically sorted (the fail-closed evidence).
    pub rejections: Vec<AttemptObservation>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProductionRoleOwnershipModel {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let model: Self = serde_json::from_str(input)
            .context("failed to parse Production Role Ownership Model JSON")?;
        model.validate()?;
        Ok(model)
    }

    /// Looks up the single owning role for an artifact class, if any.
    pub fn owner_of(&self, artifact_class: &str) -> Option<&str> {
        self.ownership
            .iter()
            .find(|assignment| assignment.artifact_class == artifact_class)
            .map(|assignment| assignment.owner_role.as_str())
    }

    /// Deterministically classifies a write attempt against the ownership model.
    /// Unauthorized writes are rejected fail-closed:
    /// - a direct trusted write is never authorized;
    /// - an unowned artifact class fails closed;
    /// - a non-owning actor is rejected.
    pub fn computed_outcome(&self, attempt: &RoleWriteAttempt) -> (String, String) {
        if attempt.write_kind == "trusted-write" {
            return (
                "rejected".to_string(),
                "direct trusted write is never authorized; proposals route through the review/apply/trust-gradient path".to_string(),
            );
        }
        match self.owner_of(&attempt.artifact_class) {
            None => (
                "rejected".to_string(),
                format!(
                    "no owning role for artifact class `{}`; unowned writes fail closed",
                    attempt.artifact_class
                ),
            ),
            Some(owner) if owner == attempt.actor_role => ("authorized".to_string(), String::new()),
            Some(owner) => (
                "rejected".to_string(),
                format!(
                    "actor role `{}` is not the owning role `{}` for artifact class `{}`",
                    attempt.actor_role, owner, attempt.artifact_class
                ),
            ),
        }
    }

    fn observation(&self, attempt: &RoleWriteAttempt) -> AttemptObservation {
        let (outcome, reason) = self.computed_outcome(attempt);
        AttemptObservation {
            attempt_id: attempt.attempt_id.clone(),
            artifact_class: attempt.artifact_class.clone(),
            actor_role: attempt.actor_role.clone(),
            owner_role: self
                .owner_of(&attempt.artifact_class)
                .unwrap_or("")
                .to_string(),
            write_kind: attempt.write_kind.clone(),
            outcome,
            reason,
        }
    }

    /// Builds the deterministic, order-independent observability read model.
    pub fn read_model(&self) -> ProductionRolesReadModel {
        let mut ownership_by_role: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for assignment in &self.ownership {
            ownership_by_role
                .entry(assignment.owner_role.clone())
                .or_default()
                .push(assignment.artifact_class.clone());
        }
        for classes in ownership_by_role.values_mut() {
            classes.sort();
        }

        let mut observations: Vec<AttemptObservation> =
            self.attempts.iter().map(|a| self.observation(a)).collect();
        observations.sort();
        let rejections: Vec<AttemptObservation> = observations
            .iter()
            .filter(|o| o.outcome == "rejected")
            .cloned()
            .collect();
        let authorized_count = observations.len() - rejections.len();

        ProductionRolesReadModel {
            schema_version: self.schema_version.clone(),
            milestone: self.milestone.clone(),
            role_count: self.roles.len(),
            ownership_count: self.ownership.len(),
            attempt_count: self.attempts.len(),
            authorized_count,
            rejected_count: rejections.len(),
            ownership_by_role,
            observations,
            rejections,
            validation_summary: vec![
                "each role is a Milestone 13 role that proposes output and owns a set of artifact classes; no role has trusted-write or merge authority".to_string(),
                "every artifact class has exactly one owning role; duplicate ownership is a conflict and fails closed, never silently merged".to_string(),
                "each write attempt is classified authorized only for a proposal by the owning role; trusted writes, unowned classes, and non-owning actors are rejected fail-closed".to_string(),
            ],
            compatibility_notes: vec![
                "reuses the Milestone 13 role set and existing evidence/review surfaces; no new orchestration engine, runtime, writer, or scheduler".to_string(),
                "non-mutating read model with no auto-apply, auto-merge, self-approval, or trusted mutation authority".to_string(),
                "ownership and outcomes are descriptive evidence; promotion flows only through the existing review/apply/trust-gradient path and a human release gate".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize production roles read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_ROLES_SCHEMA_VERSION {
            return Err(anyhow!(
                "production roles schemaVersion must be {PRODUCTION_ROLES_SCHEMA_VERSION}"
            ));
        }
        require_text("production roles milestone", &self.milestone)?;

        require_nonempty("production roles roles", self.roles.len())?;
        if self.roles.len() > 64 {
            return Err(anyhow!("production roles is overbroad for v1"));
        }
        let mut seen_roles = BTreeSet::new();
        let mut declared_owns: BTreeMap<String, String> = BTreeMap::new();
        for role in &self.roles {
            role.validate()?;
            if !seen_roles.insert(role.role.as_str()) {
                return Err(anyhow!(
                    "production roles role `{}` is duplicated",
                    role.role
                ));
            }
            for class in &role.owns_artifact_classes {
                if let Some(existing) = declared_owns.insert(class.clone(), role.role.clone()) {
                    return Err(anyhow!(
                        "production roles artifact class `{class}` is claimed by both `{existing}` and `{}`; a class has a single owning role",
                        role.role
                    ));
                }
            }
        }

        require_nonempty("production roles ownership", self.ownership.len())?;
        if self.ownership.len() > 256 {
            return Err(anyhow!("production roles ownership is overbroad for v1"));
        }
        let mut owned_classes = BTreeSet::new();
        for assignment in &self.ownership {
            assignment.validate()?;
            if !owned_classes.insert(assignment.artifact_class.as_str()) {
                return Err(anyhow!(
                    "production roles artifact class `{}` has more than one ownership assignment; conflicts fail closed",
                    assignment.artifact_class
                ));
            }
            // The owning role must be defined and must declare this class.
            match declared_owns.get(assignment.artifact_class.as_str()) {
                Some(role) if role == &assignment.owner_role => {}
                Some(role) => {
                    return Err(anyhow!(
                        "production roles ownership of `{}` assigns owner `{}` but role `{role}` declares it",
                        assignment.artifact_class,
                        assignment.owner_role
                    ));
                }
                None => {
                    return Err(anyhow!(
                        "production roles ownership of `{}` assigns owner `{}` which does not declare the class",
                        assignment.artifact_class,
                        assignment.owner_role
                    ));
                }
            }
        }
        // Every declared owned class must have a matching ownership assignment.
        for (class, role) in &declared_owns {
            if !owned_classes.contains(class.as_str()) {
                return Err(anyhow!(
                    "production roles role `{role}` declares artifact class `{class}` with no matching ownership assignment"
                ));
            }
        }

        if self.attempts.len() > 512 {
            return Err(anyhow!("production roles attempts are overbroad for v1"));
        }
        let mut attempt_ids = BTreeSet::new();
        for attempt in &self.attempts {
            attempt.validate()?;
            if !attempt_ids.insert(attempt.attempt_id.as_str()) {
                return Err(anyhow!(
                    "production roles attempt id `{}` is duplicated",
                    attempt.attempt_id
                ));
            }
            if !seen_roles.contains(attempt.actor_role.as_str()) {
                return Err(anyhow!(
                    "production roles attempt `{}` actorRole `{}` is not a defined role",
                    attempt.attempt_id,
                    attempt.actor_role
                ));
            }
            let (computed_outcome, _) = self.computed_outcome(attempt);
            if attempt.declared_outcome != computed_outcome {
                return Err(anyhow!(
                    "production roles attempt `{}` declaredOutcome `{}` does not match computed outcome `{computed_outcome}`",
                    attempt.attempt_id,
                    attempt.declared_outcome
                ));
            }
        }

        self.dashboard_compat.validate()?;

        require_text("production roles boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "proposal-only",
            "single owning role",
            "fail closed",
            "review/apply",
            "trust gradient",
            "read-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("production roles boundary must state `{required}`"));
            }
        }
        Ok(())
    }
}

impl ProductionRoleDefinition {
    fn validate(&self) -> Result<()> {
        require_role("production roles role", &self.role)?;
        require_text("production roles role purpose", &self.purpose)?;
        require_nonempty(
            "production roles role ownsArtifactClasses",
            self.owns_artifact_classes.len(),
        )?;
        let mut seen = BTreeSet::new();
        for class in &self.owns_artifact_classes {
            require_label("production roles role ownsArtifactClasses", class)?;
            if !seen.insert(class.as_str()) {
                return Err(anyhow!(
                    "production roles role `{}` repeats owned artifact class `{class}`",
                    self.role
                ));
            }
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "production roles role `{}` must be proposalOnly; no role has trusted-write authority",
                self.role
            ));
        }
        require_ref("production roles role evidenceRef", &self.evidence_ref)?;
        // Forbidden-action entries legitimately *name* the forbidden authority,
        // so they are validated as plain text (not the positive-phrase scan) and
        // must name the self-approval / auto-apply / hidden-agent boundary.
        require_nonempty(
            "production roles role forbiddenActions",
            self.forbidden_actions.len(),
        )?;
        for action in &self.forbidden_actions {
            require_plain_text("production roles role forbiddenActions", action)?;
        }
        let forbidden_text = self.forbidden_actions.join(" ").to_ascii_lowercase();
        for required in ["self-approval", "auto-apply", "hidden agent"] {
            if !forbidden_text.contains(required) {
                return Err(anyhow!(
                    "production roles role `{}` forbiddenActions must name the {required} boundary",
                    self.role
                ));
            }
        }
        Ok(())
    }
}

impl ArtifactOwnershipAssignment {
    fn validate(&self) -> Result<()> {
        require_label(
            "production roles ownership artifactClass",
            &self.artifact_class,
        )?;
        require_role("production roles ownership ownerRole", &self.owner_role)?;
        require_ref("production roles ownership evidenceRef", &self.evidence_ref)?;
        Ok(())
    }
}

impl RoleWriteAttempt {
    fn validate(&self) -> Result<()> {
        require_id("production roles attempt attemptId", &self.attempt_id)?;
        require_label(
            "production roles attempt artifactClass",
            &self.artifact_class,
        )?;
        require_role("production roles attempt actorRole", &self.actor_role)?;
        if !WRITE_KINDS.contains(&self.write_kind.as_str()) {
            return Err(anyhow!(
                "production roles attempt `{}` writeKind `{}` is unsupported",
                self.attempt_id,
                self.write_kind
            ));
        }
        require_ref("production roles attempt proposalRef", &self.proposal_ref)?;
        if !ATTEMPT_OUTCOMES.contains(&self.declared_outcome.as_str()) {
            return Err(anyhow!(
                "production roles attempt `{}` declaredOutcome `{}` is unsupported",
                self.attempt_id,
                self.declared_outcome
            ));
        }
        Ok(())
    }
}

impl ProductionRolesDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "production roles dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text("production roles dashboardCompat.surface", &self.surface)?;
        require_nonempty(
            "production roles dashboardCompat.columns",
            self.columns.len(),
        )?;
        for column in &self.columns {
            require_plain_text("production roles dashboardCompat.columns", column)?;
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

/// Non-empty text with the forbidden-authority scan (negation-aware), for free
/// text that must not assert a trusted-write/auto-apply/production authority.
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
                "{field} contains forbidden production roles authority text `{forbidden}`"
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
    const NEGATIONS: [&str; 7] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
        "not yet ",
    ];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`.
    const CONTRASTS: [&str; 6] = [
        " but ",
        " however ",
        " yet ",
        " whereas ",
        " nevertheless ",
        " though ",
    ];
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
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
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

    fn role(role: &str, classes: &[&str]) -> ProductionRoleDefinition {
        ProductionRoleDefinition {
            role: role.to_string(),
            purpose: format!("{role} proposes its owned artifacts for review"),
            owns_artifact_classes: classes.iter().map(|c| c.to_string()).collect(),
            proposal_only: true,
            evidence_ref: "docs/agent-role-model-v1.md".to_string(),
            forbidden_actions: vec![
                "no self-approval".to_string(),
                "no auto-apply".to_string(),
                "no hidden agent".to_string(),
            ],
        }
    }

    fn assignment(class: &str, owner: &str) -> ArtifactOwnershipAssignment {
        ArtifactOwnershipAssignment {
            artifact_class: class.to_string(),
            owner_role: owner.to_string(),
            evidence_ref: "docs/file-artifact-ownership-conflict-policy-v1.md".to_string(),
        }
    }

    fn attempt(id: &str, class: &str, actor: &str, kind: &str, outcome: &str) -> RoleWriteAttempt {
        RoleWriteAttempt {
            attempt_id: id.to_string(),
            artifact_class: class.to_string(),
            actor_role: actor.to_string(),
            write_kind: kind.to_string(),
            proposal_ref: "evidence/proposals/p.json".to_string(),
            declared_outcome: outcome.to_string(),
        }
    }

    fn model(attempts: Vec<RoleWriteAttempt>) -> ProductionRoleOwnershipModel {
        ProductionRoleOwnershipModel {
            schema_version: PRODUCTION_ROLES_SCHEMA_VERSION.to_string(),
            milestone: "era-h-milestone-42".to_string(),
            roles: vec![
                role("designer", &["design-brief"]),
                role("level-designer", &["scene-draft"]),
            ],
            ownership: vec![
                assignment("design-brief", "designer"),
                assignment("scene-draft", "level-designer"),
            ],
            attempts,
            dashboard_compat: ProductionRolesDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["artifactClass".to_string(), "outcome".to_string()],
            },
            boundary:
                "Role agents are proposal-only; each artifact class has a single owning role; unauthorized writes fail closed; promotion flows through the review/apply path under the trust gradient; dashboards stay read-only with no auto-apply, no auto-merge, and no self-approval"
                    .to_string(),
        }
    }

    #[test]
    fn owning_role_proposal_is_authorized() {
        let m = model(vec![attempt(
            "a1",
            "design-brief",
            "designer",
            "proposal",
            "authorized",
        )]);
        m.validate().expect("valid");
        let read = m.read_model();
        assert_eq!(read.authorized_count, 1);
        assert_eq!(read.rejected_count, 0);
        assert_eq!(read.ownership_by_role["designer"], vec!["design-brief"]);
    }

    #[test]
    fn non_owning_actor_is_rejected() {
        let m = model(vec![attempt(
            "a1",
            "design-brief",
            "level-designer",
            "proposal",
            "rejected",
        )]);
        m.validate().expect("valid");
        let read = m.read_model();
        assert_eq!(read.rejected_count, 1);
        assert!(read.rejections[0].reason.contains("is not the owning role"));
    }

    #[test]
    fn trusted_write_is_always_rejected() {
        let m = model(vec![attempt(
            "a1",
            "design-brief",
            "designer",
            "trusted-write",
            "rejected",
        )]);
        m.validate().expect("valid");
        assert_eq!(m.read_model().rejected_count, 1);
        assert!(m.read_model().rejections[0]
            .reason
            .contains("direct trusted write is never authorized"));
    }

    #[test]
    fn unowned_class_fails_closed() {
        let mut m = model(vec![attempt(
            "a1",
            "design-brief",
            "designer",
            "proposal",
            "authorized",
        )]);
        m.attempts[0].artifact_class = "scene-draft".to_string();
        // designer does not own scene-draft, so the proposal is rejected.
        m.attempts[0].declared_outcome = "rejected".to_string();
        m.validate().expect("valid");
        assert_eq!(m.read_model().rejected_count, 1);
    }

    #[test]
    fn declared_outcome_mismatch_fails_closed() {
        let m = model(vec![attempt(
            "a1",
            "design-brief",
            "level-designer",
            "proposal",
            "authorized",
        )]);
        let err = m.validate().unwrap_err().to_string();
        assert!(err.contains("does not match computed outcome"));
    }

    #[test]
    fn duplicate_ownership_is_a_conflict() {
        let mut m = model(vec![]);
        m.roles[1]
            .owns_artifact_classes
            .push("design-brief".to_string());
        let err = m.validate().unwrap_err().to_string();
        assert!(err.contains("single owning role"));
    }

    #[test]
    fn non_proposal_role_is_rejected() {
        let mut m = model(vec![]);
        m.roles[0].proposal_only = false;
        let err = m.validate().unwrap_err().to_string();
        assert!(err.contains("must be proposalOnly"));
    }

    #[test]
    fn read_model_is_order_independent() {
        let forward = model(vec![
            attempt("a1", "design-brief", "designer", "proposal", "authorized"),
            attempt("a2", "scene-draft", "designer", "proposal", "rejected"),
        ]);
        let mut reversed_attempts = forward.attempts.clone();
        reversed_attempts.reverse();
        let mut reversed = forward.clone();
        reversed.attempts = reversed_attempts;
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }

    #[test]
    fn boundary_must_state_governance() {
        let mut m = model(vec![]);
        m.boundary = "role agents own artifacts".to_string();
        assert!(m.validate().is_err());
    }
}
