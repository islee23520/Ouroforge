use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_PROTOTYPE_EVIDENCE_BUNDLE_SCHEMA_VERSION: &str = "gdd-prototype-evidence-bundle-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeEvidenceStatus {
    Pass,
    Fail,
    Partial,
    MissingRun,
    Unsupported,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeScenarioVerdict {
    Pass,
    Fail,
    Skipped,
    Unsupported,
    Missing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeRequirementCoverageStatus {
    Satisfied,
    Failed,
    Partial,
    Skipped,
    Unsupported,
    Missing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeEvidenceBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
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
    #[serde(rename = "scenarioVerdicts")]
    pub scenario_verdicts: Vec<GddPrototypeScenarioEvidence>,
    #[serde(rename = "requirementCoverage")]
    pub requirement_coverage: Vec<GddPrototypeRequirementCoverage>,
    #[serde(rename = "generatedArtifacts")]
    pub generated_artifacts: Vec<String>,
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
pub struct GddPrototypeScenarioEvidence {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "scenarioRef")]
    pub scenario_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    pub verdict: GddPrototypeScenarioVerdict,
    #[serde(rename = "runOutputRefs")]
    pub run_output_refs: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRequirementCoverage {
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    pub status: GddPrototypeRequirementCoverageStatus,
    #[serde(rename = "scenarioIds")]
    pub scenario_ids: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "failureSummary")]
    pub failure_summary: String,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeJournalSummary {
    #[serde(rename = "journalRef")]
    pub journal_ref: String,
    #[serde(rename = "gddSatisfaction")]
    pub gdd_satisfaction: String,
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
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeEvidenceBundleReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    pub status: String,
    #[serde(rename = "scenarioCount")]
    pub scenario_count: usize,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "failedRequirementCount")]
    pub failed_requirement_count: usize,
    #[serde(rename = "unsupportedRequirementCount")]
    pub unsupported_requirement_count: usize,
    #[serde(rename = "scenarioVerdictCounts")]
    pub scenario_verdict_counts: BTreeMap<String, usize>,
    #[serde(rename = "coverageStatusCounts")]
    pub coverage_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
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

    pub fn read_model(&self) -> GddPrototypeEvidenceBundleReadModel {
        let mut scenario_verdict_counts = BTreeMap::new();
        for scenario in &self.scenario_verdicts {
            *scenario_verdict_counts
                .entry(scenario_verdict_label(&scenario.verdict).to_string())
                .or_insert(0) += 1;
        }
        let mut coverage_status_counts = BTreeMap::new();
        for coverage in &self.requirement_coverage {
            *coverage_status_counts
                .entry(coverage_status_label(&coverage.status).to_string())
                .or_insert(0) += 1;
        }
        GddPrototypeEvidenceBundleReadModel {
            schema_version: self.schema_version.clone(),
            bundle_id: self.bundle_id.clone(),
            run_id: self.run_id.clone(),
            status: evidence_status_label(&self.status).to_string(),
            scenario_count: self.scenario_verdicts.len(),
            requirement_count: self.requirement_coverage.len(),
            failed_requirement_count: self
                .requirement_coverage
                .iter()
                .filter(|coverage| coverage.status == GddPrototypeRequirementCoverageStatus::Failed)
                .count(),
            unsupported_requirement_count: self
                .requirement_coverage
                .iter()
                .filter(|coverage| {
                    matches!(
                        coverage.status,
                        GddPrototypeRequirementCoverageStatus::Unsupported
                            | GddPrototypeRequirementCoverageStatus::Skipped
                            | GddPrototypeRequirementCoverageStatus::Missing
                    )
                })
                .count(),
            scenario_verdict_counts,
            coverage_status_counts,
            validation_summary: vec![
                "prototype runs are judged against linked GDD requirements and scenario verdicts".to_string(),
                "pass, fail, partial, missing-run, unsupported, and stale evidence states remain explicit".to_string(),
                "journal summaries record GDD satisfaction, observed gaps, and scoped next-step hypotheses without trusted writes".to_string(),
            ],
            compatibility_notes: vec![
                "GDD, requirements, feasibility, prototype bundle, review decision, apply artifacts, scenarios, run outputs, evidence, and journal artifacts remain separate".to_string(),
                "dashboard/Studio consumers use this read model as read-only or draft-only data".to_string(),
                "generated run/evidence output remains untracked unless explicitly fixture-scoped".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD prototype evidence bundle read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROTOTYPE_EVIDENCE_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!("GDD prototype evidence bundle schemaVersion must be {GDD_PROTOTYPE_EVIDENCE_BUNDLE_SCHEMA_VERSION}"));
        }
        require_local_id("GDD prototype evidence bundle bundleId", &self.bundle_id)?;
        require_local_id("GDD prototype evidence bundle runId", &self.run_id)?;
        for (field, value) in [
            ("GDD prototype evidence bundle gddRef", &self.gdd_ref),
            (
                "GDD prototype evidence bundle requirementExtractionRef",
                &self.requirement_extraction_ref,
            ),
            (
                "GDD prototype evidence bundle feasibilityGateRef",
                &self.feasibility_gate_ref,
            ),
            (
                "GDD prototype evidence bundle prototypeBundleRef",
                &self.prototype_bundle_ref,
            ),
            (
                "GDD prototype evidence bundle reviewDecisionRef",
                &self.review_decision_ref,
            ),
            (
                "GDD prototype evidence bundle applyArtifactRef",
                &self.apply_artifact_ref,
            ),
        ] {
            require_local_ref(field, value)?;
        }
        require_nonempty(
            "GDD prototype evidence bundle scenarioVerdicts",
            self.scenario_verdicts.len(),
        )?;
        require_nonempty(
            "GDD prototype evidence bundle requirementCoverage",
            self.requirement_coverage.len(),
        )?;
        if self.scenario_verdicts.len() > 24 || self.requirement_coverage.len() > 48 {
            return Err(anyhow!("GDD prototype evidence bundle is overbroad for v1"));
        }
        let mut scenario_ids = BTreeSet::new();
        for scenario in &self.scenario_verdicts {
            scenario.validate()?;
            if !scenario_ids.insert(scenario.scenario_id.clone()) {
                return Err(anyhow!(
                    "GDD prototype evidence bundle scenarioId `{}` is duplicated",
                    scenario.scenario_id
                ));
            }
        }
        let mut requirement_ids = BTreeSet::new();
        for coverage in &self.requirement_coverage {
            coverage.validate(&scenario_ids)?;
            if !requirement_ids.insert(coverage.requirement_id.clone()) {
                return Err(anyhow!(
                    "GDD prototype evidence bundle requirementId `{}` is duplicated",
                    coverage.requirement_id
                ));
            }
        }
        for scenario in &self.scenario_verdicts {
            for requirement_id in &scenario.requirement_ids {
                if !requirement_ids.contains(requirement_id) {
                    return Err(anyhow!("GDD prototype evidence bundle scenario `{}` links requirement `{requirement_id}` missing from requirementCoverage", scenario.scenario_id));
                }
            }
        }
        validate_local_ref_list(
            "GDD prototype evidence bundle generatedArtifacts",
            &self.generated_artifacts,
            true,
        )?;
        self.journal_summary.validate()?;
        for proposal in &self.next_mutation_proposals {
            proposal.validate(&requirement_ids)?;
        }
        validate_string_list(
            "GDD prototype evidence bundle blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        self.validate_status_consistency()?;
        require_text("GDD prototype evidence bundle boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence-gated prototype generation",
            "not autonomous unrestricted game creation",
            "untrusted until rust/local validation",
            "review-gated apply",
            "browser read-only or draft-only",
            "generated run/evidence output remains untracked",
            "no direct trusted writes",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD prototype evidence bundle boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn validate_status_consistency(&self) -> Result<()> {
        let failed = self
            .requirement_coverage
            .iter()
            .any(|coverage| coverage.status == GddPrototypeRequirementCoverageStatus::Failed)
            || self
                .scenario_verdicts
                .iter()
                .any(|scenario| scenario.verdict == GddPrototypeScenarioVerdict::Fail);
        let unsupported = self.requirement_coverage.iter().any(|coverage| {
            matches!(
                coverage.status,
                GddPrototypeRequirementCoverageStatus::Partial
                    | GddPrototypeRequirementCoverageStatus::Unsupported
                    | GddPrototypeRequirementCoverageStatus::Skipped
                    | GddPrototypeRequirementCoverageStatus::Missing
            )
        }) || self.scenario_verdicts.iter().any(|scenario| {
            matches!(
                scenario.verdict,
                GddPrototypeScenarioVerdict::Unsupported
                    | GddPrototypeScenarioVerdict::Skipped
                    | GddPrototypeScenarioVerdict::Missing
            )
        });
        let blocked = !self.blocked_reasons.is_empty()
            || self
                .scenario_verdicts
                .iter()
                .any(|scenario| !scenario.blocked_reasons.is_empty())
            || self
                .requirement_coverage
                .iter()
                .any(|coverage| !coverage.blocked_reasons.is_empty());
        match self.status {
            GddPrototypeEvidenceStatus::Pass if failed || unsupported || blocked => Err(anyhow!("passing GDD prototype evidence bundle must not include failed, missing, unsupported, skipped, stale, or blocked evidence"))?,
            GddPrototypeEvidenceStatus::Fail if !failed => Err(anyhow!("failing GDD prototype evidence bundle requires failed scenario or requirement evidence"))?,
            GddPrototypeEvidenceStatus::Partial | GddPrototypeEvidenceStatus::MissingRun | GddPrototypeEvidenceStatus::Unsupported | GddPrototypeEvidenceStatus::Stale if !unsupported && !blocked => Err(anyhow!("non-passing GDD prototype evidence bundle requires visible unsupported, missing, stale, skipped, or blocked evidence"))?,
            _ => {}
        }
        Ok(())
    }
}

impl GddPrototypeScenarioEvidence {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "GDD prototype evidence bundle scenarioId",
            &self.scenario_id,
        )?;
        require_local_ref(
            "GDD prototype evidence bundle scenarioRef",
            &self.scenario_ref,
        )?;
        validate_local_id_list(
            "GDD prototype evidence bundle scenario.requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype evidence bundle scenario.runOutputRefs",
            &self.run_output_refs,
            self.verdict == GddPrototypeScenarioVerdict::Pass
                || self.verdict == GddPrototypeScenarioVerdict::Fail,
        )?;
        validate_string_list(
            "GDD prototype evidence bundle scenario.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if matches!(
            self.verdict,
            GddPrototypeScenarioVerdict::Skipped
                | GddPrototypeScenarioVerdict::Unsupported
                | GddPrototypeScenarioVerdict::Missing
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!("GDD prototype evidence bundle skipped/unsupported/missing scenario `{}` requires blockedReasons", self.scenario_id));
        }
        Ok(())
    }
}

impl GddPrototypeRequirementCoverage {
    fn validate(&self, scenario_ids: &BTreeSet<String>) -> Result<()> {
        require_local_id(
            "GDD prototype evidence bundle requirementId",
            &self.requirement_id,
        )?;
        validate_local_id_list(
            "GDD prototype evidence bundle coverage.scenarioIds",
            &self.scenario_ids,
            true,
        )?;
        for scenario_id in &self.scenario_ids {
            if !scenario_ids.contains(scenario_id) {
                return Err(anyhow!("GDD prototype evidence bundle requirement `{}` links missing scenarioId `{scenario_id}`", self.requirement_id));
            }
        }
        validate_local_ref_list(
            "GDD prototype evidence bundle coverage.evidenceRefs",
            &self.evidence_refs,
            self.status == GddPrototypeRequirementCoverageStatus::Satisfied
                || self.status == GddPrototypeRequirementCoverageStatus::Failed
                || self.status == GddPrototypeRequirementCoverageStatus::Partial,
        )?;
        if matches!(
            self.status,
            GddPrototypeRequirementCoverageStatus::Failed
                | GddPrototypeRequirementCoverageStatus::Partial
        ) {
            require_text(
                "GDD prototype evidence bundle coverage.failureSummary",
                &self.failure_summary,
            )?;
        }
        if self.status == GddPrototypeRequirementCoverageStatus::Satisfied
            && !self.failure_summary.trim().is_empty()
        {
            return Err(anyhow!("GDD prototype evidence bundle satisfied requirements must not include failureSummary"));
        }
        validate_string_list(
            "GDD prototype evidence bundle coverage.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if matches!(
            self.status,
            GddPrototypeRequirementCoverageStatus::Skipped
                | GddPrototypeRequirementCoverageStatus::Unsupported
                | GddPrototypeRequirementCoverageStatus::Missing
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!("GDD prototype evidence bundle skipped/unsupported/missing requirement `{}` requires blockedReasons", self.requirement_id));
        }
        Ok(())
    }
}

impl GddPrototypeJournalSummary {
    fn validate(&self) -> Result<()> {
        require_local_ref(
            "GDD prototype evidence bundle journalRef",
            &self.journal_ref,
        )?;
        require_text(
            "GDD prototype evidence bundle gddSatisfaction",
            &self.gdd_satisfaction,
        )?;
        validate_string_list(
            "GDD prototype evidence bundle observedGaps",
            &self.observed_gaps,
            false,
        )?;
        validate_string_list(
            "GDD prototype evidence bundle nextStepHypotheses",
            &self.next_step_hypotheses,
            true,
        )?;
        Ok(())
    }
}

impl GddPrototypeNextMutationProposal {
    fn validate(&self, requirement_ids: &BTreeSet<String>) -> Result<()> {
        require_local_id(
            "GDD prototype evidence bundle proposalId",
            &self.proposal_id,
        )?;
        validate_local_id_list(
            "GDD prototype evidence bundle proposal.requirementIds",
            &self.requirement_ids,
            true,
        )?;
        for requirement_id in &self.requirement_ids {
            if !requirement_ids.contains(requirement_id) {
                return Err(anyhow!("GDD prototype evidence bundle proposal `{}` links missing requirement `{requirement_id}`", self.proposal_id));
            }
        }
        require_local_ref(
            "GDD prototype evidence bundle proposalRef",
            &self.proposal_ref,
        )?;
        validate_string_list(
            "GDD prototype evidence bundle proposal.blockedReasons",
            &self.blocked_reasons,
            false,
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
        GddPrototypeEvidenceStatus::Unsupported => "unsupported",
        GddPrototypeEvidenceStatus::Stale => "stale",
    }
}

fn scenario_verdict_label(verdict: &GddPrototypeScenarioVerdict) -> &'static str {
    match verdict {
        GddPrototypeScenarioVerdict::Pass => "pass",
        GddPrototypeScenarioVerdict::Fail => "fail",
        GddPrototypeScenarioVerdict::Skipped => "skipped",
        GddPrototypeScenarioVerdict::Unsupported => "unsupported",
        GddPrototypeScenarioVerdict::Missing => "missing",
    }
}

fn coverage_status_label(status: &GddPrototypeRequirementCoverageStatus) -> &'static str {
    match status {
        GddPrototypeRequirementCoverageStatus::Satisfied => "satisfied",
        GddPrototypeRequirementCoverageStatus::Failed => "failed",
        GddPrototypeRequirementCoverageStatus::Partial => "partial",
        GddPrototypeRequirementCoverageStatus::Skipped => "skipped",
        GddPrototypeRequirementCoverageStatus::Unsupported => "unsupported",
        GddPrototypeRequirementCoverageStatus::Missing => "missing",
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
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, or runs/ refs"
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
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
        "autonomous unrestricted game creation",
        "native export",
        "plugin runtime",
        "asset generation",
        "commercial readiness",
        "arbitrary source mutation",
        "arbitrary script execution",
        "trusted writes enabled",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/prototype authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 6] = ["no ", "not ", "without ", "avoid ", "forbid ", "forbidden "];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`.
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        let preceding = &hay[clause_start..idx];
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}
