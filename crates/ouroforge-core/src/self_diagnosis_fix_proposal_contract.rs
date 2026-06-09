//! Self-diagnosis and fix-proposal contract v1 (#2033 / Era L M70).
//!
//! This contract turns an attributed self-audit failure into bounded root-cause
//! hypotheses and a source-apply patch preview. It is intentionally a contract
//! and read model only: no verifier, no persistent store, no source mutation,
//! and no self-application authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::{
    classify_source_file_path_str, source_file_class_label_value, source_patch_sandbox_sha256_hex,
    SelfAuditBottleneckReport, SourceFileClassDecision, SourcePatchPreviewApplyStatus,
    SourcePatchPreviewArtifact, SourcePatchPreviewBaseRef, SourcePatchPreviewDiffStats,
    SourcePatchPreviewDiffSummary, SourcePatchPreviewEvidenceRef, SourcePatchPreviewHunkSummary,
    SourcePatchPreviewProducer, SourcePatchPreviewReadModelPrototype,
    SourcePatchPreviewRequiredTest, SourcePatchPreviewRiskLevel,
    SourcePatchPreviewRollbackExpectations, SourcePatchPreviewTarget,
    SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};

pub const SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION: &str =
    "self-diagnosis-fix-proposal-contract-v1";

pub const SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION: &str = "self-diagnosis-generator-input-v1";

pub const SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION: &str =
    "self-fix-proposal-generator-input-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfFixProposalGeneratorInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "baseBranch")]
    pub base_branch: String,
    #[serde(rename = "baseCommit")]
    pub base_commit: String,
    #[serde(rename = "targetPath")]
    pub target_path: String,
    #[serde(rename = "targetBeforeHash")]
    pub target_before_hash: String,
    #[serde(rename = "diffText")]
    pub diff_text: String,
    #[serde(rename = "expectedBehaviorChange")]
    pub expected_behavior_change: String,
    #[serde(rename = "noSelfApply")]
    pub no_self_apply: bool,
}

pub fn generate_source_apply_fix_proposal(
    diagnosis: &SelfDiagnosisRecord,
    input: &SelfFixProposalGeneratorInput,
) -> Result<SourcePatchPreviewArtifact> {
    diagnosis.validate()?;
    input.validate()?;
    let primary = diagnosis
        .root_cause_hypotheses
        .first()
        .ok_or_else(|| anyhow!("diagnosis must include at least one root-cause hypothesis"))?;

    let class_report = classify_source_file_path_str(&input.target_path);
    if class_report.decision != SourceFileClassDecision::NeedsApproval {
        return Err(anyhow!(
            "fix proposal target must reuse source-apply high-risk/source classification requiring separate approval"
        ));
    }
    let (additions, deletions) = diff_line_counts(&input.diff_text);
    if additions + deletions == 0 {
        return Err(anyhow!(
            "fix proposal diffText must contain a concrete scoped change"
        ));
    }

    let mut evidence_paths = BTreeSet::new();
    evidence_paths.extend(diagnosis.based_on_refs.iter().cloned());
    evidence_paths.extend(primary.evidence_refs.iter().cloned());
    let linked_evidence = evidence_paths
        .into_iter()
        .map(|path| SourcePatchPreviewEvidenceRef {
            kind: evidence_kind_for_path(&path).to_string(),
            path,
        })
        .collect::<Vec<_>>();

    let target = SourcePatchPreviewTarget {
        path: input.target_path.clone(),
        before_hash: input.target_before_hash.clone(),
        after_hash: None,
        file_class: source_file_class_label_value(&class_report.class),
        review_level: class_report.review_level.clone(),
        classification_status: "restricted_separate_approval".to_string(),
        classification_rationale: format!(
            "{}; generated from diagnosis {} and kept proposal-only through source-apply",
            class_report.rationale, diagnosis.diagnosis_id
        ),
        blocked_reasons: vec![
            "high-risk/source-affecting Rust trust-boundary change requires thin human go/no-go"
                .to_string(),
            "M70 emits proposals only; sourceMutationApplyStatus remains blocked".to_string(),
        ],
    };

    let artifact_hash = format!(
        "sha256:{}",
        source_patch_sandbox_sha256_hex(
            format!(
                "{}\n{}\n{}\n{}",
                input.patch_preview_id, input.proposal_id, input.target_path, input.diff_text
            )
            .as_bytes(),
        )
    );

    let artifact = SourcePatchPreviewArtifact {
        schema_version: SOURCE_PATCH_PREVIEW_SCHEMA_VERSION.to_string(),
        patch_preview_id: input.patch_preview_id.clone(),
        proposal_id: input.proposal_id.clone(),
        created_at: input.created_at.clone(),
        producer: SourcePatchPreviewProducer {
            name: "era-l-self-diagnosis-fix-proposal-generator".to_string(),
            version: SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION.to_string(),
            trusted_boundary: "read-only proposal generator over diagnosis evidence; no source apply, no merge, no command execution".to_string(),
        },
        source_mutation_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        base_ref: SourcePatchPreviewBaseRef {
            branch: input.base_branch.clone(),
            commit: input.base_commit.clone(),
            target_freshness: "requires source-apply stale-target guard before any later sandbox/review step".to_string(),
        },
        stale_target_policy: "block if target hash or branch head changes before source-apply review".to_string(),
        artifact_hash,
        targets: vec![target],
        diff_summary: SourcePatchPreviewDiffSummary {
            summary: format!(
                "Scoped engine fix proposal for {} based on {}",
                diagnosis.attributed_milestone_id, primary.hypothesis_id
            ),
            diff_text: Some(input.diff_text.clone()),
            diff_stats: SourcePatchPreviewDiffStats {
                files_changed: 1,
                additions,
                deletions,
                binary_or_opaque: false,
                generated_origin: false,
                truncated: false,
            },
            hunks: vec![SourcePatchPreviewHunkSummary {
                path: input.target_path.clone(),
                summary: primary.proposed_fix_scope.clone(),
            }],
            large_diff_warning: None,
            binary_or_opaque_warning: None,
        },
        risk_level: SourcePatchPreviewRiskLevel::High,
        risk_ids: vec!["STM-01".to_string(), "STM-04".to_string()],
        linked_evidence,
        expected_behavior_change: input.expected_behavior_change.clone(),
        required_tests: vec![
            required_test(vec!["cargo", "fmt", "--check"]),
            required_test(vec![
                "cargo",
                "test",
                "-p",
                "ouroforge-core",
                "--test",
                "self_diagnosis_fix_proposal_contract",
                "--jobs",
                "2",
            ]),
        ],
        reviewer_checklist: vec![
            "Confirm diagnosis evidence refs still resolve to verdict/journal/ledger/loop-coverage artifacts.".to_string(),
            "Confirm target freshness and rollback evidence before any later source-apply transaction.".to_string(),
            "Human go/no-go required for this high-risk/source-affecting Rust trust-boundary change.".to_string(),
            "No auto-apply, no merge, and no command execution is authorized by this proposal.".to_string(),
        ],
        rollback_expectations: SourcePatchPreviewRollbackExpectations {
            required_before_apply: true,
            minimum_fields: vec![
                "beforeHash".to_string(),
                "reversePatchRef".to_string(),
                "verificationRerunRef".to_string(),
            ],
        },
        read_model_prototype: Some(SourcePatchPreviewReadModelPrototype {
            status: "blocked".to_string(),
            display_label: "High-risk engine fix proposal queued for human go/no-go".to_string(),
            file_class_summary: "rust_trust_boundary/restricted_separate_approval".to_string(),
            risk_summary: "source-affecting engine change; source-apply and trust-gradient required".to_string(),
            primary_blocked_reason: "high-risk source-affecting proposal only; no auto-apply at M70".to_string(),
            allowed_actions: vec!["read".to_string(), "review".to_string()],
            forbidden_actions: vec![
                "apply".to_string(),
                "merge".to_string(),
                "auto_apply".to_string(),
                "execute_tests".to_string(),
            ],
        }),
    };
    validate_source_apply_proposal(&artifact)?;
    Ok(artifact)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDiagnosisGeneratorInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "verdictRef")]
    pub verdict_ref: String,
    #[serde(rename = "verdictJson")]
    pub verdict_json: String,
    #[serde(rename = "journalRef")]
    pub journal_ref: String,
    #[serde(rename = "journalMarkdown")]
    pub journal_markdown: String,
    #[serde(rename = "ledgerRef")]
    pub ledger_ref: String,
    #[serde(rename = "ledgerJsonl")]
    pub ledger_jsonl: String,
    #[serde(rename = "loopCoverageAttributionRef")]
    pub loop_coverage_attribution_ref: String,
    #[serde(rename = "sourceApplyRef")]
    pub source_apply_ref: String,
    #[serde(rename = "trustGradientRef")]
    pub trust_gradient_ref: String,
    #[serde(rename = "noHumanInput")]
    pub no_human_input: bool,
}

pub fn generate_self_diagnosis_record(
    input: &SelfDiagnosisGeneratorInput,
    attribution: &SelfAuditBottleneckReport,
) -> Result<SelfDiagnosisRecord> {
    input.validate()?;
    validate_attribution_for_diagnosis(input, attribution)?;

    let hypotheses = attribution
        .ranked_bottlenecks
        .iter()
        .map(|rank| {
            let mut evidence_refs = BTreeSet::new();
            evidence_refs.insert(input.verdict_ref.clone());
            evidence_refs.insert(input.journal_ref.clone());
            evidence_refs.insert(input.ledger_ref.clone());
            evidence_refs.insert(input.loop_coverage_attribution_ref.clone());
            evidence_refs.extend(rank.evidence_refs.iter().cloned());
            evidence_refs.extend(rank.attribution_refs.iter().cloned());

            let reason_summary = rank.reasons.join("; ");
            SelfRootCauseHypothesis {
                hypothesis_id: format!(
                    "{}-{}-root-cause",
                    rank.milestone_id, rank.gate_kind
                ),
                causal_chain: vec![
                    format!(
                        "detect: {} evidence exposed {} failure signals for {}",
                        input.verdict_ref,
                        rank.signal_ids.join(","),
                        rank.gate_kind
                    ),
                    format!(
                        "explain: {} and {} preserve the autonomous run narrative without human input",
                        input.journal_ref, input.ledger_ref
                    ),
                    format!(
                        "trace: ledger stages replay detect→explain→trace→attribute→propose and cite {}",
                        input.loop_coverage_attribution_ref
                    ),
                    format!(
                        "attribute: bottleneck attribution maps {} to {} ({}) because {}",
                        rank.gate_kind, rank.milestone_id, rank.issue_ref, reason_summary
                    ),
                    format!(
                        "propose: route the engine fix through {} with {} gating; no self-application",
                        input.source_apply_ref, input.trust_gradient_ref
                    ),
                ],
                evidence_refs: evidence_refs.into_iter().collect(),
                proposed_fix_scope: format!(
                    "Rust kernel/evaluator/source-apply contract for {} at {}; no game content mutation and no Elixir executor change.",
                    rank.gate_kind, rank.milestone_id
                ),
                confidence: if rank.score >= 150 {
                    "high".to_string()
                } else {
                    "medium".to_string()
                },
            }
        })
        .collect::<Vec<_>>();

    let record = SelfDiagnosisRecord {
        diagnosis_id: format!("{}-autonomous-root-cause-diagnosis", input.title_id),
        attributed_milestone_id: attribution.ranked_bottlenecks[0].milestone_id.clone(),
        attributed_issue_ref: attribution.ranked_bottlenecks[0].issue_ref.clone(),
        based_on_refs: vec![
            input.verdict_ref.clone(),
            input.journal_ref.clone(),
            input.ledger_ref.clone(),
            input.loop_coverage_attribution_ref.clone(),
            "self-audit-bottleneck-attribution".to_string(),
            input.source_apply_ref.clone(),
            input.trust_gradient_ref.clone(),
        ],
        root_cause_hypotheses: hypotheses,
    };
    record.validate()?;
    Ok(record)
}

const REQUIRED_PIPELINE_REFS: &[&str] = &[
    "verdict",
    "journal.md",
    "ledger.jsonl",
    "loop-coverage",
    "self-audit-bottleneck-attribution",
    "source-apply",
    "trust-gradient",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDiagnosisFixProposalContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    pub diagnosis: SelfDiagnosisRecord,
    #[serde(rename = "sourceApplyProposal")]
    pub source_apply_proposal: SourcePatchPreviewArtifact,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDiagnosisRecord {
    #[serde(rename = "diagnosisId")]
    pub diagnosis_id: String,
    #[serde(rename = "attributedMilestoneId")]
    pub attributed_milestone_id: String,
    #[serde(rename = "attributedIssueRef")]
    pub attributed_issue_ref: String,
    #[serde(rename = "basedOnRefs")]
    pub based_on_refs: Vec<String>,
    #[serde(rename = "rootCauseHypotheses")]
    pub root_cause_hypotheses: Vec<SelfRootCauseHypothesis>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfRootCauseHypothesis {
    #[serde(rename = "hypothesisId")]
    pub hypothesis_id: String,
    #[serde(rename = "causalChain")]
    pub causal_chain: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "proposedFixScope")]
    pub proposed_fix_scope: String,
    pub confidence: String,
}

impl SelfFixProposalGeneratorInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION {
            return Err(anyhow!(
                "fix proposal generator schemaVersion must be {SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION}"
            ));
        }
        for (label, value) in [
            ("patchPreviewId", &self.patch_preview_id),
            ("proposalId", &self.proposal_id),
            ("baseBranch", &self.base_branch),
            ("baseCommit", &self.base_commit),
        ] {
            require_id(label, value)?;
        }
        require_ref("targetPath", &self.target_path)?;
        if !self.target_path.starts_with("crates/") {
            return Err(anyhow!(
                "targetPath must be an engine Rust/source-apply target under crates/"
            ));
        }
        if !(self.target_before_hash.starts_with("sha256:")
            && self.target_before_hash.len() == "sha256:".len() + 64)
        {
            return Err(anyhow!("targetBeforeHash must use sha256:<64-hex>"));
        }
        require_text("createdAt", &self.created_at)?;
        require_text("diffText", &self.diff_text)?;
        require_text("expectedBehaviorChange", &self.expected_behavior_change)?;
        if !self.no_self_apply {
            return Err(anyhow!(
                "fix proposal generator must keep noSelfApply=true at M70"
            ));
        }
        Ok(())
    }
}

fn required_test(argv: Vec<&str>) -> SourcePatchPreviewRequiredTest {
    let argv = argv.into_iter().map(str::to_string).collect::<Vec<_>>();
    SourcePatchPreviewRequiredTest {
        command: argv.join(" "),
        argv,
        allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
        execution_authority: "copyable_not_executed_metadata".to_string(),
    }
}

fn diff_line_counts(diff_text: &str) -> (usize, usize) {
    let additions = diff_text
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .count();
    let deletions = diff_text
        .lines()
        .filter(|line| line.starts_with('-') && !line.starts_with("---"))
        .count();
    (additions, deletions)
}

fn evidence_kind_for_path(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.contains("verdict") {
        "verdict"
    } else if lower.contains("journal") {
        "journal"
    } else if lower.contains("ledger") {
        "ledger"
    } else if lower.contains("loop-coverage") {
        "loop-coverage-attribution"
    } else if lower.contains("source-apply") {
        "source-apply"
    } else if lower.contains("trust-gradient") {
        "trust-gradient"
    } else {
        "diagnosis-evidence"
    }
}

impl SelfDiagnosisGeneratorInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-diagnosis generator schemaVersion must be {SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        for (label, reference) in [
            ("verdictRef", &self.verdict_ref),
            ("journalRef", &self.journal_ref),
            ("ledgerRef", &self.ledger_ref),
            (
                "loopCoverageAttributionRef",
                &self.loop_coverage_attribution_ref,
            ),
            ("sourceApplyRef", &self.source_apply_ref),
            ("trustGradientRef", &self.trust_gradient_ref),
        ] {
            require_ref(label, reference)?;
        }
        require_contains_ci("verdictRef", &self.verdict_ref, "verdict")?;
        require_contains_ci("journalRef", &self.journal_ref, "journal.md")?;
        require_contains_ci("ledgerRef", &self.ledger_ref, "ledger.jsonl")?;
        require_contains_ci(
            "loopCoverageAttributionRef",
            &self.loop_coverage_attribution_ref,
            "loop-coverage",
        )?;
        require_contains_ci("sourceApplyRef", &self.source_apply_ref, "source-apply")?;
        require_contains_ci(
            "trustGradientRef",
            &self.trust_gradient_ref,
            "trust-gradient",
        )?;
        if !self.no_human_input {
            return Err(anyhow!(
                "self-diagnosis generator must run with noHumanInput=true"
            ));
        }
        validate_verdict_json(&self.verdict_json)?;
        validate_journal_markdown(&self.journal_markdown)?;
        validate_ledger_jsonl(&self.ledger_jsonl)?;
        Ok(())
    }
}

fn validate_attribution_for_diagnosis(
    input: &SelfDiagnosisGeneratorInput,
    attribution: &SelfAuditBottleneckReport,
) -> Result<()> {
    if attribution.title_id != input.title_id {
        return Err(anyhow!(
            "diagnosis input titleId `{}` does not match attribution titleId `{}`",
            input.title_id,
            attribution.title_id
        ));
    }
    if attribution.ranked_bottlenecks.is_empty() {
        return Err(anyhow!(
            "diagnosis requires at least one attributed failure bottleneck"
        ));
    }
    let boundary = attribution.boundary.to_ascii_lowercase();
    for required in [
        "no new verification engine",
        "no new data plane",
        "no human input",
        "source-apply",
        "trust-gradient",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!(
                "attribution boundary must preserve {required} for diagnosis"
            ));
        }
    }
    Ok(())
}

fn validate_verdict_json(verdict_json: &str) -> Result<()> {
    let verdict: serde_json::Value = serde_json::from_str(verdict_json)
        .map_err(|err| anyhow!("verdictJson must parse as existing verdict JSON: {err}"))?;
    for key in ["status", "summary"] {
        if verdict.get(key).is_none() {
            return Err(anyhow!("verdictJson must include {key}"));
        }
    }
    let text = verdict_json.to_ascii_lowercase();
    for required in ["fourgates", "designintegrity", "journal", "ledger"] {
        if !text.contains(required) {
            return Err(anyhow!(
                "verdictJson must preserve existing {required} evidence"
            ));
        }
    }
    Ok(())
}

fn validate_journal_markdown(journal_markdown: &str) -> Result<()> {
    require_text("journalMarkdown", journal_markdown)?;
    let lower = journal_markdown.to_ascii_lowercase();
    for required in [
        "openchrome",
        "four gates",
        "design-integrity",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "ring 2",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("journalMarkdown must mention {required}"));
        }
    }
    Ok(())
}

fn validate_ledger_jsonl(ledger_jsonl: &str) -> Result<()> {
    require_text("ledgerJsonl", ledger_jsonl)?;
    let mut stages = BTreeSet::new();
    let mut completed_without_human = false;
    let mut high_risk_auto_apply_false = false;
    for (line_index, line) in ledger_jsonl.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: serde_json::Value = serde_json::from_str(line).map_err(|err| {
            anyhow!(
                "ledgerJsonl line {} must parse as existing ledger event JSON: {err}",
                line_index + 1
            )
        })?;
        if let Some(stage) = entry
            .pointer("/payload/campaignStage")
            .and_then(|value| value.as_str())
        {
            stages.insert(stage.to_string());
        }
        if entry.get("event").and_then(|value| value.as_str()) == Some("dogfood.campaign.completed")
        {
            completed_without_human = entry
                .pointer("/payload/requiresHumanInputOnAutonomousPath")
                .and_then(|value| value.as_bool())
                == Some(false);
            high_risk_auto_apply_false = entry
                .pointer("/payload/highRiskAutoApply")
                .and_then(|value| value.as_bool())
                == Some(false);
        }
    }
    for required in [
        "detect",
        "explain",
        "trace",
        "attribute",
        "propose",
        "re-verify",
        "apply-or-queue",
    ] {
        if !stages.contains(required) {
            return Err(anyhow!("ledgerJsonl must include {required} stage"));
        }
    }
    if !completed_without_human {
        return Err(anyhow!(
            "ledgerJsonl must show the autonomous path completed with zero human input"
        ));
    }
    if !high_risk_auto_apply_false {
        return Err(anyhow!(
            "ledgerJsonl must show high-risk fixes are not auto-applied"
        ));
    }
    Ok(())
}

fn require_contains_ci(label: &str, value: &str, needle: &str) -> Result<()> {
    if value.to_ascii_lowercase().contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{label} must mention {needle}"))
    }
}

impl SelfDiagnosisFixProposalContract {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input).map_err(|err| {
            anyhow!("failed to parse self-diagnosis fix-proposal contract: {err}")
        })?;
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-diagnosis contract schemaVersion must be {SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        self.diagnosis.validate()?;
        validate_source_apply_proposal(&self.source_apply_proposal)?;
        validate_boundary(&self.boundary)?;
        validate_guardrails(&self.guardrails)?;
        Ok(())
    }
}

impl SelfDiagnosisRecord {
    fn validate(&self) -> Result<()> {
        require_id("diagnosisId", &self.diagnosis_id)?;
        require_id("attributedMilestoneId", &self.attributed_milestone_id)?;
        require_issue_ref("attributedIssueRef", &self.attributed_issue_ref)?;
        validate_refs("basedOnRefs", &self.based_on_refs, true)?;
        let joined = self.based_on_refs.join("\n").to_ascii_lowercase();
        for required in REQUIRED_PIPELINE_REFS {
            if !joined.contains(required) {
                return Err(anyhow!(
                    "basedOnRefs must include existing {required} evidence"
                ));
            }
        }
        if self.root_cause_hypotheses.is_empty() {
            return Err(anyhow!("rootCauseHypotheses must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for hypothesis in &self.root_cause_hypotheses {
            hypothesis.validate()?;
            if !ids.insert(hypothesis.hypothesis_id.as_str()) {
                return Err(anyhow!(
                    "rootCauseHypotheses contains duplicate hypothesisId `{}`",
                    hypothesis.hypothesis_id
                ));
            }
        }
        Ok(())
    }
}

impl SelfRootCauseHypothesis {
    fn validate(&self) -> Result<()> {
        require_id("hypothesisId", &self.hypothesis_id)?;
        validate_texts("causalChain", &self.causal_chain, true)?;
        if self.causal_chain.len() < 3 {
            return Err(anyhow!(
                "causalChain must include detect, explain/trace, and proposal links"
            ));
        }
        validate_refs("evidenceRefs", &self.evidence_refs, true)?;
        require_text("proposedFixScope", &self.proposed_fix_scope)?;
        match self.confidence.as_str() {
            "low" | "medium" | "high" => Ok(()),
            _ => Err(anyhow!("confidence must be low, medium, or high")),
        }
    }
}

fn validate_source_apply_proposal(proposal: &SourcePatchPreviewArtifact) -> Result<()> {
    if proposal.schema_version != SOURCE_PATCH_PREVIEW_SCHEMA_VERSION {
        return Err(anyhow!(
            "sourceApplyProposal must reuse {SOURCE_PATCH_PREVIEW_SCHEMA_VERSION}"
        ));
    }
    if proposal.source_mutation_apply_status != SourcePatchPreviewApplyStatus::Blocked {
        return Err(anyhow!(
            "sourceApplyProposal sourceMutationApplyStatus must remain blocked at M70"
        ));
    }
    if proposal.targets.is_empty() {
        return Err(anyhow!("sourceApplyProposal targets must not be empty"));
    }
    if proposal.linked_evidence.is_empty() {
        return Err(anyhow!(
            "sourceApplyProposal linkedEvidence must cite diagnosis evidence"
        ));
    }
    if proposal.required_tests.is_empty() {
        return Err(anyhow!(
            "sourceApplyProposal requiredTests must name re-verification commands"
        ));
    }
    let evidence_text = proposal
        .linked_evidence
        .iter()
        .map(|evidence| evidence.path.as_str())
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for required in ["verdict", "journal", "ledger", "loop-coverage"] {
        if !evidence_text.contains(required) {
            return Err(anyhow!(
                "sourceApplyProposal linkedEvidence must include {required} evidence"
            ));
        }
    }
    let checklist = proposal.reviewer_checklist.join("\n").to_ascii_lowercase();
    if matches!(
        proposal.risk_level,
        SourcePatchPreviewRiskLevel::High | SourcePatchPreviewRiskLevel::Critical
    ) && !(checklist.contains("human go/no-go") && checklist.contains("no auto-apply"))
    {
        return Err(anyhow!(
            "high-risk sourceApplyProposal must keep thin human go/no-go and no auto-apply checklist items"
        ));
    }
    let forbidden_actions = proposal
        .read_model_prototype
        .as_ref()
        .map(|model| model.forbidden_actions.join("\n").to_ascii_lowercase())
        .unwrap_or_default();
    if !(forbidden_actions.contains("apply") && forbidden_actions.contains("merge")) {
        return Err(anyhow!(
            "sourceApplyProposal read model must forbid apply and merge at M70"
        ));
    }
    Ok(())
}

fn validate_boundary(boundary: &str) -> Result<()> {
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "read-only",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "no self-application",
        "without a human",
        "never auto-applied",
        "human ring 2",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_guardrails(guardrails: &[String]) -> Result<()> {
    validate_texts("guardrails", guardrails, true)?;
    let joined = guardrails.join("\n").to_ascii_lowercase();
    for required in [
        "source-apply",
        "trust-gradient",
        "high-risk",
        "human go/no-go",
        "no new verification engine",
        "no new data plane",
    ] {
        if !joined.contains(required) {
            return Err(anyhow!("guardrails must mention {required}"));
        }
    }
    Ok(())
}

fn validate_refs(label: &str, refs: &[String], non_empty: bool) -> Result<()> {
    if non_empty && refs.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for reference in refs {
        require_ref(label, reference)?;
        if !seen.insert(reference) {
            return Err(anyhow!("{label} contains duplicate ref `{reference}`"));
        }
    }
    Ok(())
}

fn validate_texts(label: &str, values: &[String], non_empty: bool) -> Result<()> {
    if non_empty && values.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for value in values {
        require_text(label, value)?;
    }
    Ok(())
}

fn require_issue_ref(label: &str, value: &str) -> Result<()> {
    if value.starts_with('#') && value[1..].chars().all(|ch| ch.is_ascii_digit()) {
        Ok(())
    } else {
        Err(anyhow!("{label} must be a GitHub issue ref like #2033"))
    }
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains(';')
        || value.contains("&&")
        || value.contains('|')
    {
        return Err(anyhow!("{label} must be a safe local evidence ref"));
    }
    Ok(())
}

fn require_id(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        Ok(())
    } else {
        Err(anyhow!("{label} must be a bounded local id"))
    }
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{label} must not be empty"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_contract() -> SelfDiagnosisFixProposalContract {
        SelfDiagnosisFixProposalContract::from_json_str(include_str!(
            "../../../examples/real-title-dogfood-v1/self-diagnosis-fix-proposal-v1/contract.fixture.json"
        ))
        .expect("fixture contract validates")
    }

    #[test]
    fn fixture_contract_reuses_source_apply_preview_without_apply_authority() {
        let contract = fixture_contract();
        assert_eq!(
            contract.source_apply_proposal.schema_version,
            "patch-preview.v1"
        );
        assert_eq!(
            contract.source_apply_proposal.source_mutation_apply_status,
            SourcePatchPreviewApplyStatus::Blocked
        );
        assert_eq!(
            contract.source_apply_proposal.risk_level,
            SourcePatchPreviewRiskLevel::High
        );
    }

    #[test]
    fn high_risk_proposal_requires_human_go_no_go_checklist() {
        let mut contract = fixture_contract();
        contract.source_apply_proposal.reviewer_checklist = vec!["review evidence".to_string()];
        let error = contract.validate().expect_err("missing checklist rejected");
        assert!(error.to_string().contains("human go/no-go"));
    }
}
