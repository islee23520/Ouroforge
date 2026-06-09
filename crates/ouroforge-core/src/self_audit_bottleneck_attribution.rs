//! Bottleneck attribution over real-build evidence v1 (#2029 / Era L M69).
//!
//! This module is a deterministic read-model over the existing dogfood evidence
//! pipeline. It consumes the #2028 self-audit attribution contract plus
//! ledger/journal/verdict/loop-coverage-linked signals and ranks the milestone
//! most responsible for a failed or slow real-build step. It does not execute a
//! verifier, create a persistent store, or grant apply authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    SelfAuditAttributionContract, SelfAuditMilestoneMapping,
    SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION,
};

pub const SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION: &str =
    "self-audit-bottleneck-attribution-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfAuditSignalStatus {
    Pass,
    Slow,
    Fail,
}

impl SelfAuditSignalStatus {
    fn weight(self) -> u32 {
        match self {
            Self::Pass => 0,
            Self::Slow => 40,
            Self::Fail => 100,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Slow => "slow",
            Self::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditBottleneckSignal {
    #[serde(rename = "signalId")]
    pub signal_id: String,
    #[serde(rename = "signalKind")]
    pub signal_kind: String,
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    #[serde(rename = "loopStage")]
    pub loop_stage: String,
    #[serde(rename = "gateKind")]
    pub gate_kind: String,
    pub status: SelfAuditSignalStatus,
    #[serde(default, rename = "elapsedMs")]
    pub elapsed_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditBottleneckInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "contractSchemaVersion")]
    pub contract_schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    pub signals: Vec<SelfAuditBottleneckSignal>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditBottleneckReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "contractSchemaVersion")]
    pub contract_schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "rankedBottlenecks")]
    pub ranked_bottlenecks: Vec<SelfAuditBottleneckRank>,
    #[serde(rename = "unattributedSignals")]
    pub unattributed_signals: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditBottleneckRank {
    pub rank: usize,
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "issueRef")]
    pub issue_ref: String,
    #[serde(rename = "gateKind")]
    pub gate_kind: String,
    #[serde(rename = "loopStage")]
    pub loop_stage: String,
    pub score: u64,
    #[serde(rename = "signalIds")]
    pub signal_ids: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "attributionRefs")]
    pub attribution_refs: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug)]
struct RankAccumulator<'a> {
    mapping: &'a SelfAuditMilestoneMapping,
    score: u64,
    signal_ids: BTreeSet<String>,
    evidence_refs: BTreeSet<String>,
    attribution_refs: BTreeSet<String>,
    reasons: BTreeSet<String>,
}

pub fn self_audit_bottleneck_input_from_json_str(input: &str) -> Result<SelfAuditBottleneckInput> {
    let parsed: SelfAuditBottleneckInput = serde_json::from_str(input)
        .map_err(|err| anyhow!("failed to parse self-audit bottleneck input: {err}"))?;
    parsed.validate()?;
    Ok(parsed)
}

pub fn attribute_self_audit_bottlenecks(
    contract: &SelfAuditAttributionContract,
    input: &SelfAuditBottleneckInput,
) -> Result<SelfAuditBottleneckReport> {
    contract.validate()?;
    input.validate()?;
    if input.contract_schema_version != contract.schema_version {
        return Err(anyhow!(
            "bottleneck input contractSchemaVersion must match the loaded self-audit contract"
        ));
    }
    if input.title_id != contract.title_id {
        return Err(anyhow!(
            "bottleneck input titleId `{}` does not match contract titleId `{}`",
            input.title_id,
            contract.title_id
        ));
    }

    let mut accumulators: BTreeMap<String, RankAccumulator<'_>> = BTreeMap::new();
    let mut unattributed = BTreeSet::new();

    for signal in &input.signals {
        if signal.status == SelfAuditSignalStatus::Pass {
            continue;
        }
        let Some(mapping) = best_mapping_for_signal(contract, signal) else {
            unattributed.insert(signal.signal_id.clone());
            continue;
        };
        let score = signal.status.weight() as u64 + signal.elapsed_ms / 100;
        let entry = accumulators
            .entry(mapping.milestone_id.clone())
            .or_insert_with(|| RankAccumulator {
                mapping,
                score: 0,
                signal_ids: BTreeSet::new(),
                evidence_refs: mapping.evidence_refs.iter().cloned().collect(),
                attribution_refs: mapping.attribution_refs.iter().cloned().collect(),
                reasons: BTreeSet::new(),
            });
        entry.score += score;
        entry.signal_ids.insert(signal.signal_id.clone());
        entry.evidence_refs.insert(signal.source_ref.clone());
        entry.reasons.insert(format!(
            "{} signal `{}` matched {} at {} ({})",
            signal.status.as_str(),
            signal.signal_kind,
            mapping.gate_kind,
            mapping.milestone_id,
            signal.summary
        ));
    }

    let mut ranked: Vec<_> = accumulators
        .into_values()
        .map(|acc| SelfAuditBottleneckRank {
            rank: 0,
            milestone_id: acc.mapping.milestone_id.clone(),
            issue_ref: acc.mapping.issue_ref.clone(),
            gate_kind: acc.mapping.gate_kind.clone(),
            loop_stage: acc.mapping.loop_stage.clone(),
            score: acc.score,
            signal_ids: acc.signal_ids.into_iter().collect(),
            evidence_refs: acc.evidence_refs.into_iter().collect(),
            attribution_refs: acc.attribution_refs.into_iter().collect(),
            reasons: acc.reasons.into_iter().collect(),
        })
        .collect();
    ranked.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.milestone_id.cmp(&right.milestone_id))
            .then_with(|| left.gate_kind.cmp(&right.gate_kind))
    });
    for (index, row) in ranked.iter_mut().enumerate() {
        row.rank = index + 1;
    }

    Ok(SelfAuditBottleneckReport {
        schema_version: SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION.to_string(),
        contract_schema_version: contract.schema_version.clone(),
        title_id: contract.title_id.clone(),
        ranked_bottlenecks: ranked,
        unattributed_signals: unattributed.into_iter().collect(),
        boundary: input.boundary.clone(),
    })
}

fn best_mapping_for_signal<'a>(
    contract: &'a SelfAuditAttributionContract,
    signal: &SelfAuditBottleneckSignal,
) -> Option<&'a SelfAuditMilestoneMapping> {
    contract
        .milestone_mappings
        .iter()
        .filter(|mapping| mapping_matches_signal(mapping, signal))
        .max_by(|left, right| {
            mapping_match_score(left, signal)
                .cmp(&mapping_match_score(right, signal))
                .then_with(|| right.milestone_id.cmp(&left.milestone_id))
        })
}

fn mapping_matches_signal(
    mapping: &SelfAuditMilestoneMapping,
    signal: &SelfAuditBottleneckSignal,
) -> bool {
    mapping
        .failure_signal_kinds
        .iter()
        .any(|kind| kind == &signal.signal_kind)
        || mapping.gate_kind == signal.gate_kind
        || mapping.loop_stage == signal.loop_stage
}

fn mapping_match_score(
    mapping: &SelfAuditMilestoneMapping,
    signal: &SelfAuditBottleneckSignal,
) -> u8 {
    let mut score = 0;
    if mapping
        .failure_signal_kinds
        .iter()
        .any(|kind| kind == &signal.signal_kind)
    {
        score += 4;
    }
    if mapping.gate_kind == signal.gate_kind {
        score += 2;
    }
    if mapping.loop_stage == signal.loop_stage {
        score += 1;
    }
    score
}

impl SelfAuditBottleneckInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-audit bottleneck schemaVersion must be {SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION}"
            ));
        }
        if self.contract_schema_version != SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-audit bottleneck input must reference {SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        validate_refs("evidenceRefs", &self.evidence_refs, true)?;
        let evidence_text = self.evidence_refs.join("\n").to_ascii_lowercase();
        for required in ["ledger.jsonl", "journal.md", "verdict", "loop-coverage"] {
            if !evidence_text.contains(required) {
                return Err(anyhow!(
                    "evidenceRefs must include existing {required} pipeline artifact"
                ));
            }
        }
        validate_boundary(&self.boundary)?;
        if self.signals.is_empty() {
            return Err(anyhow!("signals must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for signal in &self.signals {
            signal.validate()?;
            if !ids.insert(signal.signal_id.as_str()) {
                return Err(anyhow!(
                    "signals contains duplicate signalId `{}`",
                    signal.signal_id
                ));
            }
        }
        Ok(())
    }
}

impl SelfAuditBottleneckSignal {
    fn validate(&self) -> Result<()> {
        require_id("signalId", &self.signal_id)?;
        require_id("signalKind", &self.signal_kind)?;
        require_ref("sourceRef", &self.source_ref)?;
        require_id("loopStage", &self.loop_stage)?;
        require_id("gateKind", &self.gate_kind)?;
        require_text("summary", &self.summary)?;
        Ok(())
    }
}

fn validate_boundary(boundary: &str) -> Result<()> {
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "read-only",
        "ledger.jsonl",
        "journal.md",
        "verdict",
        "loop-coverage",
        "no new verification engine",
        "no new data plane",
        "no human input",
        "source-apply",
        "trust-gradient",
        "never auto-applied",
        "human ring 2",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("boundary must mention {required}"));
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

    #[test]
    fn fails_closed_when_existing_pipeline_refs_are_missing() {
        let mut input = fixture_input();
        input.evidence_refs = vec!["examples/real-title-dogfood-v1/run/verdict.json".to_string()];
        let error = input
            .validate()
            .expect_err("missing existing refs rejected");
        assert!(error.to_string().contains("ledger.jsonl"));
    }

    #[test]
    fn ranks_failures_before_slowness_deterministically() {
        let contract = SelfAuditAttributionContract::from_json_str(include_str!(
            "../../../examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
        ))
        .unwrap();
        let report = attribute_self_audit_bottlenecks(&contract, &fixture_input()).unwrap();
        assert_eq!(
            report.ranked_bottlenecks[0].milestone_id,
            "m68-real-title-run"
        );
        assert!(report.ranked_bottlenecks[0].score > report.ranked_bottlenecks[1].score);
    }

    fn fixture_input() -> SelfAuditBottleneckInput {
        SelfAuditBottleneckInput {
            schema_version: SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION.to_string(),
            contract_schema_version: SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION.to_string(),
            title_id: "era-i-engine-builder-deckbuilder".to_string(),
            evidence_refs: vec![
                "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
                "examples/real-title-dogfood-v1/run/journal.md".to_string(),
                "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
                "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string(),
            ],
            signals: vec![
                SelfAuditBottleneckSignal {
                    signal_id: "sig-fail".to_string(),
                    signal_kind: "gate-fail".to_string(),
                    source_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
                    loop_stage: "attribute".to_string(),
                    gate_kind: "four-gates".to_string(),
                    status: SelfAuditSignalStatus::Fail,
                    elapsed_ms: 1000,
                    summary: "planted gate failure".to_string(),
                },
                SelfAuditBottleneckSignal {
                    signal_id: "sig-slow".to_string(),
                    signal_kind: "hidden-friction".to_string(),
                    source_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
                    loop_stage: "trace".to_string(),
                    gate_kind: "openchrome-demo".to_string(),
                    status: SelfAuditSignalStatus::Slow,
                    elapsed_ms: 500,
                    summary: "planted slow demo step".to_string(),
                },
            ],
            boundary: "Read-only bottleneck attribution over ledger.jsonl, journal.md, verdict, and loop-coverage evidence; no new verification engine and no new data plane; autonomous path has no human input; source-apply and trust-gradient keep high-risk/source-affecting changes never auto-applied; human Ring 2 fun/taste and release go/no-go remain outside automation.".to_string(),
        }
    }
}
