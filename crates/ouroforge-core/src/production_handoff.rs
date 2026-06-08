//! Handoff Artifacts and Conflict Resolution v1 (#1676).
//!
//! This module records role-to-role handoffs and resolves conflicts between
//! them deterministically. It reuses the existing role set, the evidence/journal
//! refs, and the review/apply/trust-gradient path; it does **not** add a new
//! writer, runtime, or orchestration engine, and it never performs a trusted
//! write or an auto-merge.
//!
//! Each handoff carries the owning (`fromRole`) and receiving (`toRole`) roles,
//! the artifact class being handed off, the base evidence ref the receiver
//! builds on, and the proposal ref routed through review/apply. The ledger
//! classifies each handoff deterministically:
//!
//! - `clean` — no overlapping concurrent edit and no stale ref; resolution
//!   `accepted` (ready to route through review/apply, never auto-applied);
//! - `conflict` — two or more handoffs edit the same artifact class from the
//!   same base; resolution `blocked` — conflicts are surfaced and preserved,
//!   never silently merged or auto-resolved by promotion;
//! - `stale` — the handoff references stale evidence; resolution `needs-fix`.
//!
//! The ledger is inert local evidence. Promotion of any accepted handoff still
//! flows only through the existing review/apply/trust-gradient path, and a human
//! retains the release go/no-go.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCTION_HANDOFF_SCHEMA_VERSION: &str = "production-handoff-v1";

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

/// Deterministic handoff classifications.
const HANDOFF_STATUSES: &[&str] = &["clean", "conflict", "stale"];

/// Deterministic resolutions. None of them auto-applies or auto-merges.
const RESOLUTIONS: &[&str] = &["accepted", "blocked", "needs-fix"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionHandoffLedger {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    pub handoffs: Vec<ProductionHandoff>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: ProductionHandoffDashboardCompat,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionHandoff {
    #[serde(rename = "handoffId")]
    pub handoff_id: String,
    /// The owning role handing the artifact off.
    #[serde(rename = "fromRole")]
    pub from_role: String,
    /// The receiving role; must differ from `fromRole`.
    #[serde(rename = "toRole")]
    pub to_role: String,
    /// The artifact class being handed off.
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    /// The base evidence ref the receiver builds on (the conflict key).
    #[serde(rename = "baseRef")]
    pub base_ref: String,
    /// The proposal evidence ref routed through review/apply.
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    /// Refs known to be stale; a non-empty list makes the handoff `stale`.
    #[serde(rename = "staleRefs", default)]
    pub stale_refs: Vec<String>,
    /// The handoff ids this one conflicts with; must equal the computed set.
    #[serde(rename = "conflictsWith", default)]
    pub conflicts_with: Vec<String>,
    /// `clean` / `conflict` / `stale`; must match the computed classification.
    pub status: String,
    /// `accepted` / `blocked` / `needs-fix`; must match the computed resolution.
    pub resolution: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionHandoffDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A deterministic observability record for a single handoff.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandoffObservation {
    #[serde(rename = "handoffId")]
    pub handoff_id: String,
    #[serde(rename = "fromRole")]
    pub from_role: String,
    #[serde(rename = "toRole")]
    pub to_role: String,
    #[serde(rename = "artifactClass")]
    pub artifact_class: String,
    pub status: String,
    pub resolution: String,
    #[serde(rename = "conflictsWith")]
    pub conflicts_with: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionHandoffReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub milestone: String,
    #[serde(rename = "handoffCount")]
    pub handoff_count: usize,
    #[serde(rename = "cleanCount")]
    pub clean_count: usize,
    #[serde(rename = "conflictCount")]
    pub conflict_count: usize,
    #[serde(rename = "staleCount")]
    pub stale_count: usize,
    #[serde(rename = "acceptedCount")]
    pub accepted_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "needsFixCount")]
    pub needs_fix_count: usize,
    pub observations: Vec<HandoffObservation>,
    /// The blocked/needs-fix subset (the fail-closed evidence), sorted.
    pub unresolved: Vec<HandoffObservation>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProductionHandoffLedger {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let ledger: Self = serde_json::from_str(input)
            .context("failed to parse Production Handoff Ledger JSON")?;
        ledger.validate()?;
        Ok(ledger)
    }

    /// The deterministic set of handoff ids that conflict with `handoff`: other
    /// handoffs editing the same artifact class from the same base ref.
    pub fn computed_conflicts(&self, handoff: &ProductionHandoff) -> Vec<String> {
        let mut conflicts: Vec<String> = self
            .handoffs
            .iter()
            .filter(|other| {
                other.handoff_id != handoff.handoff_id
                    && other.artifact_class == handoff.artifact_class
                    && other.base_ref == handoff.base_ref
            })
            .map(|other| other.handoff_id.clone())
            .collect();
        conflicts.sort();
        conflicts.dedup();
        conflicts
    }

    /// Deterministic classification: stale takes precedence over conflict, which
    /// takes precedence over clean.
    pub fn computed_status(&self, handoff: &ProductionHandoff) -> String {
        if !handoff.stale_refs.is_empty() {
            return "stale".to_string();
        }
        if !self.computed_conflicts(handoff).is_empty() {
            return "conflict".to_string();
        }
        "clean".to_string()
    }

    pub fn computed_resolution(&self, handoff: &ProductionHandoff) -> String {
        match self.computed_status(handoff).as_str() {
            "stale" => "needs-fix".to_string(),
            "conflict" => "blocked".to_string(),
            _ => "accepted".to_string(),
        }
    }

    fn observation(&self, handoff: &ProductionHandoff) -> HandoffObservation {
        let status = self.computed_status(handoff);
        let resolution = self.computed_resolution(handoff);
        let reason = match status.as_str() {
            "stale" => format!(
                "handoff `{}` references {} stale ref(s); needs-fix, not promoted",
                handoff.handoff_id,
                handoff.stale_refs.len()
            ),
            "conflict" => format!(
                "handoff `{}` edits `{}` from base `{}` concurrently with {}; blocked, never auto-merged",
                handoff.handoff_id,
                handoff.artifact_class,
                handoff.base_ref,
                self.computed_conflicts(handoff).join(", ")
            ),
            _ => format!(
                "handoff `{}` is clean; accepted to route through review/apply, not auto-applied",
                handoff.handoff_id
            ),
        };
        HandoffObservation {
            handoff_id: handoff.handoff_id.clone(),
            from_role: handoff.from_role.clone(),
            to_role: handoff.to_role.clone(),
            artifact_class: handoff.artifact_class.clone(),
            status,
            resolution,
            conflicts_with: self.computed_conflicts(handoff),
            reason,
        }
    }

    /// Builds the deterministic, order-independent observability read model.
    pub fn read_model(&self) -> ProductionHandoffReadModel {
        let mut observations: Vec<HandoffObservation> =
            self.handoffs.iter().map(|h| self.observation(h)).collect();
        observations.sort();
        let count_status = |s: &str| observations.iter().filter(|o| o.status == s).count();
        let count_resolution = |r: &str| observations.iter().filter(|o| o.resolution == r).count();
        let unresolved: Vec<HandoffObservation> = observations
            .iter()
            .filter(|o| o.resolution != "accepted")
            .cloned()
            .collect();

        ProductionHandoffReadModel {
            schema_version: self.schema_version.clone(),
            milestone: self.milestone.clone(),
            handoff_count: self.handoffs.len(),
            clean_count: count_status("clean"),
            conflict_count: count_status("conflict"),
            stale_count: count_status("stale"),
            accepted_count: count_resolution("accepted"),
            blocked_count: count_resolution("blocked"),
            needs_fix_count: count_resolution("needs-fix"),
            observations,
            unresolved,
            validation_summary: vec![
                "each handoff carries the owning and receiving roles, the artifact class, the base evidence ref, and a proposal ref routed through review/apply".to_string(),
                "two handoffs editing the same artifact class from the same base ref conflict; the conflict is blocked and preserved, never silently merged or auto-resolved by promotion".to_string(),
                "a handoff with stale refs is needs-fix; classification and resolution are deterministic and fail closed when declared values disagree with the computed ones".to_string(),
            ],
            compatibility_notes: vec![
                "reuses the Milestone 13 role set, the evidence/journal refs, and the review/apply/trust-gradient path; no new writer, runtime, or orchestration engine".to_string(),
                "non-mutating read model with no auto-merge, auto-apply, self-approval, or trusted mutation authority".to_string(),
                "resolutions are descriptive evidence; promotion flows only through the existing review/apply/trust-gradient path and a human release gate".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize production handoff read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_HANDOFF_SCHEMA_VERSION {
            return Err(anyhow!(
                "production handoff schemaVersion must be {PRODUCTION_HANDOFF_SCHEMA_VERSION}"
            ));
        }
        require_text("production handoff milestone", &self.milestone)?;

        require_nonempty("production handoff handoffs", self.handoffs.len())?;
        if self.handoffs.len() > 512 {
            return Err(anyhow!("production handoff ledger is overbroad for v1"));
        }
        let mut handoff_ids = BTreeSet::new();
        for handoff in &self.handoffs {
            handoff.validate()?;
            if !handoff_ids.insert(handoff.handoff_id.as_str()) {
                return Err(anyhow!(
                    "production handoff id `{}` is duplicated",
                    handoff.handoff_id
                ));
            }
        }

        // Deterministic cross-checks: declared conflicts/status/resolution must
        // match the computed values, or the ledger fails closed.
        for handoff in &self.handoffs {
            let computed_conflicts = self.computed_conflicts(handoff);
            let mut declared = handoff.conflicts_with.clone();
            declared.sort();
            declared.dedup();
            if declared != computed_conflicts {
                return Err(anyhow!(
                    "production handoff `{}` conflictsWith {:?} does not match the computed set {:?}",
                    handoff.handoff_id,
                    declared,
                    computed_conflicts
                ));
            }
            for id in &computed_conflicts {
                if !handoff_ids.contains(id.as_str()) {
                    return Err(anyhow!(
                        "production handoff `{}` conflicts with unknown handoff `{id}`",
                        handoff.handoff_id
                    ));
                }
            }
            let computed_status = self.computed_status(handoff);
            if handoff.status != computed_status {
                return Err(anyhow!(
                    "production handoff `{}` status `{}` does not match computed status `{computed_status}`",
                    handoff.handoff_id,
                    handoff.status
                ));
            }
            let computed_resolution = self.computed_resolution(handoff);
            if handoff.resolution != computed_resolution {
                return Err(anyhow!(
                    "production handoff `{}` resolution `{}` does not match computed resolution `{computed_resolution}`",
                    handoff.handoff_id,
                    handoff.resolution
                ));
            }
        }

        self.dashboard_compat.validate()?;

        require_text("production handoff boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "proposal-only",
            "deterministic",
            "fail closed",
            "review/apply",
            "trust gradient",
            "read-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "production handoff boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl ProductionHandoff {
    fn validate(&self) -> Result<()> {
        require_id("production handoff handoffId", &self.handoff_id)?;
        require_role("production handoff fromRole", &self.from_role)?;
        require_role("production handoff toRole", &self.to_role)?;
        if self.from_role == self.to_role {
            return Err(anyhow!(
                "production handoff `{}` fromRole and toRole must differ",
                self.handoff_id
            ));
        }
        require_label("production handoff artifactClass", &self.artifact_class)?;
        require_ref("production handoff baseRef", &self.base_ref)?;
        require_ref("production handoff proposalRef", &self.proposal_ref)?;
        let mut seen_stale = BTreeSet::new();
        for stale in &self.stale_refs {
            require_ref("production handoff staleRefs", stale)?;
            if !seen_stale.insert(stale.as_str()) {
                return Err(anyhow!(
                    "production handoff `{}` repeats stale ref `{stale}`",
                    self.handoff_id
                ));
            }
        }
        for id in &self.conflicts_with {
            require_id("production handoff conflictsWith", id)?;
            if id == &self.handoff_id {
                return Err(anyhow!(
                    "production handoff `{}` cannot conflict with itself",
                    self.handoff_id
                ));
            }
        }
        if !HANDOFF_STATUSES.contains(&self.status.as_str()) {
            return Err(anyhow!(
                "production handoff `{}` status `{}` is unsupported",
                self.handoff_id,
                self.status
            ));
        }
        if !RESOLUTIONS.contains(&self.resolution.as_str()) {
            return Err(anyhow!(
                "production handoff `{}` resolution `{}` is unsupported",
                self.handoff_id,
                self.resolution
            ));
        }
        Ok(())
    }
}

impl ProductionHandoffDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "production handoff dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text("production handoff dashboardCompat.surface", &self.surface)?;
        require_nonempty(
            "production handoff dashboardCompat.columns",
            self.columns.len(),
        )?;
        for column in &self.columns {
            require_plain_text("production handoff dashboardCompat.columns", column)?;
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
                "{field} contains forbidden production handoff authority text `{forbidden}`"
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
        // forbidden-authority terms and list connectors. The nearest negation
        // has the least intervening text, so if even it leaves a non-filler word
        // (e.g. `allow`), no farther negation can scope either.
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

    fn handoff(id: &str, from: &str, to: &str, class: &str, base: &str) -> ProductionHandoff {
        ProductionHandoff {
            handoff_id: id.to_string(),
            from_role: from.to_string(),
            to_role: to.to_string(),
            artifact_class: class.to_string(),
            base_ref: base.to_string(),
            proposal_ref: format!("evidence/proposals/{id}.json"),
            stale_refs: Vec::new(),
            conflicts_with: Vec::new(),
            status: "clean".to_string(),
            resolution: "accepted".to_string(),
        }
    }

    fn ledger(handoffs: Vec<ProductionHandoff>) -> ProductionHandoffLedger {
        ProductionHandoffLedger {
            schema_version: PRODUCTION_HANDOFF_SCHEMA_VERSION.to_string(),
            milestone: "era-h-milestone-42".to_string(),
            handoffs,
            dashboard_compat: ProductionHandoffDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["handoffId".to_string(), "resolution".to_string()],
            },
            boundary:
                "Role handoffs are proposal-only; conflict resolution is deterministic; conflicting and stale handoffs fail closed and are routed through the review/apply path under the trust gradient, not auto-merged and not auto-applied; dashboards stay read-only with no self-approval."
                    .to_string(),
        }
    }

    /// Reconciles declared status/resolution/conflicts with the computed ones so
    /// the test builder always produces a valid ledger for the happy path.
    fn reconcile(mut l: ProductionHandoffLedger) -> ProductionHandoffLedger {
        let snapshot = l.clone();
        for h in &mut l.handoffs {
            h.conflicts_with = snapshot.computed_conflicts(h);
            h.status = snapshot.computed_status(h);
            h.resolution = snapshot.computed_resolution(h);
        }
        l
    }

    #[test]
    fn clean_handoff_is_accepted() {
        let l = reconcile(ledger(vec![handoff(
            "h1",
            "designer",
            "level-designer",
            "design-brief",
            "evidence/base/brief.json",
        )]));
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.clean_count, 1);
        assert_eq!(read.accepted_count, 1);
        assert!(read.unresolved.is_empty());
    }

    #[test]
    fn concurrent_edits_conflict_and_block_deterministically() {
        let l = reconcile(ledger(vec![
            handoff(
                "h1",
                "designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
            handoff(
                "h2",
                "level-designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
        ]));
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.conflict_count, 2);
        assert_eq!(read.blocked_count, 2);
        // Deterministic, symmetric conflict sets.
        let h1 = read
            .observations
            .iter()
            .find(|o| o.handoff_id == "h1")
            .unwrap();
        assert_eq!(h1.conflicts_with, vec!["h2".to_string()]);
        assert!(h1.reason.contains("never auto-merged"));
    }

    #[test]
    fn stale_refs_need_fix() {
        let mut l = ledger(vec![handoff(
            "h1",
            "asset-import-planner",
            "qa-agent",
            "asset-proposal",
            "evidence/base/asset.json",
        )]);
        l.handoffs[0].stale_refs = vec!["evidence/base/asset.json".to_string()];
        let l = reconcile(l);
        l.validate().expect("valid");
        let read = l.read_model();
        assert_eq!(read.stale_count, 1);
        assert_eq!(read.needs_fix_count, 1);
        assert_eq!(read.unresolved.len(), 1);
    }

    #[test]
    fn different_base_is_a_chain_not_a_conflict() {
        let l = reconcile(ledger(vec![
            handoff(
                "h1",
                "designer",
                "reviewer",
                "scene-draft",
                "evidence/base/a.json",
            ),
            handoff(
                "h2",
                "level-designer",
                "reviewer",
                "scene-draft",
                "evidence/base/b.json",
            ),
        ]));
        l.validate().expect("valid");
        assert_eq!(l.read_model().conflict_count, 0);
        assert_eq!(l.read_model().clean_count, 2);
    }

    #[test]
    fn declared_status_mismatch_fails_closed() {
        let mut l = reconcile(ledger(vec![handoff(
            "h1",
            "designer",
            "reviewer",
            "design-brief",
            "evidence/base/brief.json",
        )]));
        l.handoffs[0].status = "conflict".to_string();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("does not match computed status"));
    }

    #[test]
    fn declared_conflicts_mismatch_fails_closed() {
        let mut l = reconcile(ledger(vec![
            handoff(
                "h1",
                "designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
            handoff(
                "h2",
                "level-designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
        ]));
        l.handoffs[0].conflicts_with = Vec::new();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("does not match the computed set"));
    }

    #[test]
    fn same_role_handoff_is_rejected() {
        let mut l = ledger(vec![handoff(
            "h1",
            "designer",
            "designer",
            "design-brief",
            "evidence/base/brief.json",
        )]);
        l.handoffs[0].conflicts_with = Vec::new();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("fromRole and toRole must differ"));
    }

    #[test]
    fn read_model_is_order_independent() {
        let forward = reconcile(ledger(vec![
            handoff(
                "h1",
                "designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
            handoff(
                "h2",
                "level-designer",
                "reviewer",
                "scene-draft",
                "evidence/base/scene.json",
            ),
            handoff(
                "h3",
                "qa-agent",
                "critic",
                "asset-proposal",
                "evidence/base/asset.json",
            ),
        ]));
        let mut reversed = forward.clone();
        reversed.handoffs.reverse();
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }

    #[test]
    fn unrelated_negation_does_not_whitelist_authority() {
        // An unrelated leading negation must not suppress a positive claim.
        assert!(contains_positive_phrase(
            "no restrictions allow auto-merge",
            "auto-merge"
        ));
        assert!(contains_positive_phrase(
            "with no oversight we permit self-approval",
            "self-approval"
        ));
        // A directly negated list keeps every listed item negated.
        assert!(!contains_positive_phrase(
            "no auto-apply or self-approval",
            "auto-apply"
        ));
        assert!(!contains_positive_phrase(
            "no auto-apply or self-approval",
            "self-approval"
        ));
        // Immediate negation still scopes for the verb forms used in boundaries.
        assert!(!contains_positive_phrase(
            "not auto-merged and not auto-applied",
            "auto-merge"
        ));
        assert!(!contains_positive_phrase(
            "not auto-merged and not auto-applied",
            "auto-apply"
        ));
    }

    #[test]
    fn boundary_with_unrelated_negation_authority_is_rejected() {
        let mut l = reconcile(ledger(vec![handoff(
            "h1",
            "designer",
            "reviewer",
            "design-brief",
            "evidence/base/brief.json",
        )]));
        // All required tokens are present, but an affirmative auto-merge claim
        // hides behind an unrelated negation; the scan must still reject it.
        l.boundary = "Role handoffs are proposal-only; conflict resolution is deterministic; handoffs fail closed and route through the review/apply path under the trust gradient; dashboards stay read-only; no restrictions allow auto-merge.".to_string();
        let err = l.validate().unwrap_err().to_string();
        assert!(err.contains("forbidden production handoff authority text"));
    }
}
