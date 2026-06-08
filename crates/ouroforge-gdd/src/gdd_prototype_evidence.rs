use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const GDD_PROTOTYPE_EVIDENCE_SCHEMA_VERSION: &str = "gdd-prototype-evidence-bundle-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeEvidenceStatus {
    Pass,
    Fail,
    Partial,
    MissingRun,
    Stale,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeScenarioVerdict {
    Pass,
    Fail,
    Skipped,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeRequirementStatus {
    Satisfied,
    Failed,
    Partial,
    Skipped,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeEvidenceBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "evidenceBundleId")]
    pub evidence_bundle_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    pub status: GddPrototypeEvidenceStatus,
    #[serde(rename = "gddRef")]
    pub gdd_ref: String,
    #[serde(rename = "requirementExtractionRef")]
    pub requirement_extraction_ref: String,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "prototypeBundleRef")]
    pub prototype_bundle_ref: String,
    #[serde(rename = "reviewDecisionRef")]
    pub review_decision_ref: String,
    #[serde(rename = "applyArtifactRef")]
    pub apply_artifact_ref: String,
    #[serde(rename = "scenarioRefs")]
    pub scenario_refs: Vec<String>,
    #[serde(rename = "runOutputRefs")]
    pub run_output_refs: Vec<String>,
    #[serde(rename = "generatedArtifactRefs")]
    pub generated_artifact_refs: Vec<String>,
    #[serde(rename = "scenarioVerdicts")]
    pub scenario_verdicts: Vec<GddPrototypeScenarioOutcome>,
    #[serde(rename = "requirementCoverage")]
    pub requirement_coverage: Vec<GddPrototypeRequirementCoverage>,
    #[serde(rename = "journalSummary")]
    pub journal_summary: GddPrototypeJournalSummary,
    #[serde(rename = "nextMutationProposals")]
    pub next_mutation_proposals: Vec<GddPrototypeNextMutationProposal>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeScenarioOutcome {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub verdict: GddPrototypeScenarioVerdict,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRequirementCoverage {
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    pub status: GddPrototypeRequirementStatus,
    #[serde(rename = "scenarioIds")]
    pub scenario_ids: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeJournalSummary {
    #[serde(rename = "summaryId")]
    pub summary_id: String,
    #[serde(rename = "gddSatisfaction")]
    pub gdd_satisfaction: String,
    pub failures: Vec<String>,
    #[serde(rename = "observedGaps")]
    pub observed_gaps: Vec<String>,
    #[serde(rename = "nextStepHypotheses")]
    pub next_step_hypotheses: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeNextMutationProposal {
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    pub summary: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeEvidenceReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "evidenceBundleId")]
    pub evidence_bundle_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    pub status: String,
    #[serde(rename = "scenarioCounts")]
    pub scenario_counts: BTreeMap<String, usize>,
    #[serde(rename = "requirementCounts")]
    pub requirement_counts: BTreeMap<String, usize>,
    #[serde(rename = "failedRequirements")]
    pub failed_requirements: Vec<String>,
    #[serde(rename = "unsupportedRequirements")]
    pub unsupported_requirements: Vec<String>,
    #[serde(rename = "dashboardSummary")]
    pub dashboard_summary: Vec<String>,
    #[serde(rename = "journalSummary")]
    pub journal_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddPrototypeEvidenceBundleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Prototype Evidence Bundle JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddPrototypeEvidenceReadModel {
        let mut scenario_counts = BTreeMap::new();
        for outcome in &self.scenario_verdicts {
            *scenario_counts
                .entry(scenario_verdict_label(&outcome.verdict).to_string())
                .or_insert(0) += 1;
        }
        let mut requirement_counts = BTreeMap::new();
        let mut failed_requirements = Vec::new();
        let mut unsupported_requirements = Vec::new();
        for coverage in &self.requirement_coverage {
            *requirement_counts
                .entry(requirement_status_label(&coverage.status).to_string())
                .or_insert(0) += 1;
            if coverage.status == GddPrototypeRequirementStatus::Failed {
                failed_requirements.push(coverage.requirement_id.clone());
            }
            if coverage.status == GddPrototypeRequirementStatus::Unsupported {
                unsupported_requirements.push(coverage.requirement_id.clone());
            }
        }
        GddPrototypeEvidenceReadModel {
            schema_version: self.schema_version.clone(),
            evidence_bundle_id: self.evidence_bundle_id.clone(),
            run_id: self.run_id.clone(),
            status: evidence_status_label(&self.status).to_string(),
            scenario_counts,
            requirement_counts,
            failed_requirements,
            unsupported_requirements,
            dashboard_summary: vec![
                format!("scenarios:{}", self.scenario_verdicts.len()),
                format!("requirements:{}", self.requirement_coverage.len()),
                format!("generatedArtifacts:{}", self.generated_artifact_refs.len()),
                format!("nextMutations:{}", self.next_mutation_proposals.len()),
            ],
            journal_summary: vec![
                format!("gddSatisfaction:{}", self.journal_summary.gdd_satisfaction),
                format!("failures:{}", self.journal_summary.failures.len()),
                format!("observedGaps:{}", self.journal_summary.observed_gaps.len()),
                format!(
                    "nextStepHypotheses:{}",
                    self.journal_summary.next_step_hypotheses.len()
                ),
            ],
            compatibility_notes: vec![
                "dashboard/Studio consumers receive read-only prototype evidence and coverage state".to_string(),
                "GDD, requirements, feasibility, prototype bundle, review, apply, scenarios, run outputs, evidence, and journal remain linked but separated".to_string(),
                "missing, malformed, partial, unsupported, stale, and failed run states remain visible instead of being hidden".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD prototype evidence read model JSON")
    }

    pub fn journal_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("## GDD Prototype Evidence Journal\n\n");
        out.push_str("- Boundary: read-only GDD prototype evidence; no hidden command execution, browser trusted writes, auto-apply, auto-merge, or autonomous unrestricted game creation.\n");
        out.push_str(&format!(
            "- Bundle: `{}` status `{}`\n",
            self.evidence_bundle_id,
            evidence_status_label(&self.status)
        ));
        out.push_str(&format!(
            "- GDD satisfaction: {}\n",
            self.journal_summary.gdd_satisfaction
        ));
        out.push_str(&format!(
            "- Failed requirements: {}\n",
            join_or_none(&self.read_model().failed_requirements)
        ));
        out.push_str(&format!(
            "- Unsupported requirements: {}\n",
            join_or_none(&self.read_model().unsupported_requirements)
        ));
        out.push_str(&format!(
            "- Observed gaps: {}\n",
            join_or_none(&self.journal_summary.observed_gaps)
        ));
        out.push_str(&format!(
            "- Next-step hypotheses: {}\n",
            join_or_none(&self.journal_summary.next_step_hypotheses)
        ));
        out
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROTOTYPE_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD prototype evidence bundle schemaVersion must be {GDD_PROTOTYPE_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "GDD prototype evidence evidenceBundleId",
            &self.evidence_bundle_id,
        )?;
        require_local_id("GDD prototype evidence runId", &self.run_id)?;
        for (field, value) in [
            ("GDD prototype evidence gddRef", &self.gdd_ref),
            (
                "GDD prototype evidence requirementExtractionRef",
                &self.requirement_extraction_ref,
            ),
            (
                "GDD prototype evidence feasibilityGateRef",
                &self.feasibility_gate_ref,
            ),
            (
                "GDD prototype evidence prototypeBundleRef",
                &self.prototype_bundle_ref,
            ),
            (
                "GDD prototype evidence reviewDecisionRef",
                &self.review_decision_ref,
            ),
            (
                "GDD prototype evidence applyArtifactRef",
                &self.apply_artifact_ref,
            ),
        ] {
            require_local_ref(field, value)?;
        }
        validate_local_ref_list(
            "GDD prototype evidence scenarioRefs",
            &self.scenario_refs,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype evidence runOutputRefs",
            &self.run_output_refs,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype evidence generatedArtifactRefs",
            &self.generated_artifact_refs,
            false,
        )?;
        require_nonempty(
            "GDD prototype evidence scenarioVerdicts",
            self.scenario_verdicts.len(),
        )?;
        require_nonempty(
            "GDD prototype evidence requirementCoverage",
            self.requirement_coverage.len(),
        )?;
        for outcome in &self.scenario_verdicts {
            outcome.validate()?;
        }
        for coverage in &self.requirement_coverage {
            coverage.validate()?;
        }
        self.journal_summary.validate()?;
        for proposal in &self.next_mutation_proposals {
            proposal.validate()?;
        }
        validate_string_list(
            "GDD prototype evidence blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        self.validate_status_consistency()?;
        require_text("GDD prototype evidence boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence-gated prototype generation",
            "not autonomous unrestricted game creation",
            "gdd-derived output remains untrusted",
            "rust/local validation",
            "read-only or draft-only",
            "generated run/evidence output remains untracked",
            "no hidden command execution",
            "no browser trusted writes",
            "no auto-apply",
            "no auto-merge",
            "#1 remains open",
            "#23 remains open",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD prototype evidence boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn validate_status_consistency(&self) -> Result<()> {
        let failed_requirements = self
            .requirement_coverage
            .iter()
            .filter(|coverage| coverage.status == GddPrototypeRequirementStatus::Failed)
            .count();
        let unsupported_or_skipped = self
            .requirement_coverage
            .iter()
            .filter(|coverage| {
                matches!(
                    coverage.status,
                    GddPrototypeRequirementStatus::Skipped
                        | GddPrototypeRequirementStatus::Unsupported
                )
            })
            .count();
        let partial_requirements = self
            .requirement_coverage
            .iter()
            .filter(|coverage| coverage.status == GddPrototypeRequirementStatus::Partial)
            .count();
        let failed_scenarios = self
            .scenario_verdicts
            .iter()
            .filter(|outcome| outcome.verdict == GddPrototypeScenarioVerdict::Fail)
            .count();
        let missing_or_unsupported_scenarios = self
            .scenario_verdicts
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.verdict,
                    GddPrototypeScenarioVerdict::Skipped | GddPrototypeScenarioVerdict::Unsupported
                )
            })
            .count();
        let has_blockers = !self.blocked_reasons.is_empty()
            || self
                .scenario_verdicts
                .iter()
                .any(|outcome| !outcome.blocked_reasons.is_empty())
            || self
                .requirement_coverage
                .iter()
                .any(|coverage| !coverage.blocked_reasons.is_empty());
        match self.status {
            GddPrototypeEvidenceStatus::Pass => {
                if failed_requirements > 0
                    || unsupported_or_skipped > 0
                    || partial_requirements > 0
                    || failed_scenarios > 0
                    || missing_or_unsupported_scenarios > 0
                    || has_blockers
                    || self.run_output_refs.is_empty()
                {
                    return Err(anyhow!("passing GDD prototype evidence requires run outputs, passing scenarios, satisfied requirements, and no blockers"));
                }
            }
            GddPrototypeEvidenceStatus::Fail => {
                if failed_requirements == 0 || failed_scenarios == 0 {
                    return Err(anyhow!("failing GDD prototype evidence requires failed requirements and failed scenario verdicts"));
                }
            }
            GddPrototypeEvidenceStatus::Partial => {
                if partial_requirements == 0 && unsupported_or_skipped == 0 {
                    return Err(anyhow!("partial GDD prototype evidence requires partial, skipped, or unsupported requirements"));
                }
            }
            GddPrototypeEvidenceStatus::MissingRun => {
                if !self.run_output_refs.is_empty() || !has_blockers {
                    return Err(anyhow!("missing-run GDD prototype evidence requires no run outputs and explicit blocked reasons"));
                }
            }
            GddPrototypeEvidenceStatus::Stale | GddPrototypeEvidenceStatus::Blocked => {
                if !has_blockers {
                    return Err(anyhow!(
                        "stale or blocked GDD prototype evidence requires visible blocked reasons"
                    ));
                }
            }
        }
        Ok(())
    }
}

impl GddPrototypeScenarioOutcome {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype evidence scenarioId", &self.scenario_id)?;
        require_local_ref(
            "GDD prototype evidence scenario evidenceRef",
            &self.evidence_ref,
        )?;
        validate_local_id_list(
            "GDD prototype evidence scenario requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_string_list(
            "GDD prototype evidence scenario blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if matches!(
            self.verdict,
            GddPrototypeScenarioVerdict::Skipped | GddPrototypeScenarioVerdict::Unsupported
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "skipped or unsupported GDD prototype scenario verdict requires blockedReasons"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeRequirementCoverage {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype evidence requirementId", &self.requirement_id)?;
        validate_local_id_list(
            "GDD prototype evidence requirement scenarioIds",
            &self.scenario_ids,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype evidence requirement evidenceRefs",
            &self.evidence_refs,
            false,
        )?;
        validate_string_list(
            "GDD prototype evidence requirement blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.status == GddPrototypeRequirementStatus::Satisfied && self.evidence_refs.is_empty()
        {
            return Err(anyhow!(
                "satisfied GDD prototype requirement coverage requires evidenceRefs"
            ));
        }
        if matches!(
            self.status,
            GddPrototypeRequirementStatus::Failed
                | GddPrototypeRequirementStatus::Partial
                | GddPrototypeRequirementStatus::Skipped
                | GddPrototypeRequirementStatus::Unsupported
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "failed, partial, skipped, or unsupported GDD prototype requirements require blockedReasons"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeJournalSummary {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype evidence journal summaryId", &self.summary_id)?;
        require_text(
            "GDD prototype evidence journal gddSatisfaction",
            &self.gdd_satisfaction,
        )?;
        validate_string_list(
            "GDD prototype evidence journal failures",
            &self.failures,
            false,
        )?;
        validate_string_list(
            "GDD prototype evidence journal observedGaps",
            &self.observed_gaps,
            false,
        )?;
        validate_string_list(
            "GDD prototype evidence journal nextStepHypotheses",
            &self.next_step_hypotheses,
            false,
        )?;
        Ok(())
    }
}

impl GddPrototypeNextMutationProposal {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype evidence proposalId", &self.proposal_id)?;
        require_text("GDD prototype evidence proposal summary", &self.summary)?;
        validate_local_ref_list(
            "GDD prototype evidence proposal evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        Ok(())
    }
}

fn evidence_status_label(status: &GddPrototypeEvidenceStatus) -> &'static str {
    match status {
        GddPrototypeEvidenceStatus::Pass => "pass",
        GddPrototypeEvidenceStatus::Fail => "fail",
        GddPrototypeEvidenceStatus::Partial => "partial",
        GddPrototypeEvidenceStatus::MissingRun => "missing-run",
        GddPrototypeEvidenceStatus::Stale => "stale",
        GddPrototypeEvidenceStatus::Blocked => "blocked",
    }
}

fn scenario_verdict_label(verdict: &GddPrototypeScenarioVerdict) -> &'static str {
    match verdict {
        GddPrototypeScenarioVerdict::Pass => "pass",
        GddPrototypeScenarioVerdict::Fail => "fail",
        GddPrototypeScenarioVerdict::Skipped => "skipped",
        GddPrototypeScenarioVerdict::Unsupported => "unsupported",
    }
}

fn requirement_status_label(status: &GddPrototypeRequirementStatus) -> &'static str {
    match status {
        GddPrototypeRequirementStatus::Satisfied => "satisfied",
        GddPrototypeRequirementStatus::Failed => "failed",
        GddPrototypeRequirementStatus::Partial => "partial",
        GddPrototypeRequirementStatus::Skipped => "skipped",
        GddPrototypeRequirementStatus::Unsupported => "unsupported",
    }
}

fn validate_local_id_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_id(field, value)?;
    }
    Ok(())
}

fn validate_local_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_ref(field, value)?;
    }
    Ok(())
}

fn validate_string_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} contains forbidden traversal and must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/")
        || value.starts_with("dashboard-data/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, runs/, or dashboard-data/ refs"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic import",
        "command bridge",
        "local server bridge",
        "browser trusted write enabled",
        "auto-merge enabled",
        "auto-apply enabled",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "commercial readiness",
        "hosted/cloud",
        "plugin runtime",
        "native export",
        "asset generation enabled",
        "autonomous unrestricted game creation enabled",
        "arbitrary source mutation enabled",
        "arbitrary script execution enabled",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!("{field} contains forbidden wording `{forbidden}`"));
        }
    }
    Ok(())
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}
