use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION: &str = "loop-coverage-attribution-v1";
const SUPPORTED_ARTIFACT_KINDS: &[&str] = &[
    "trusted-change",
    "run-artifact",
    "evidence-artifact",
    "verdict",
    "transaction",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoopCoverageProvenanceClass {
    LoopProduced,
    LoopVerified,
    Manual,
}
impl LoopCoverageProvenanceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoopProduced => "loop-produced",
            Self::LoopVerified => "loop-verified",
            Self::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoopCoverageAttributionStatus {
    Classified,
    MissingProvenance,
    Ambiguous,
    StaleRef,
    UnsupportedKind,
}
impl LoopCoverageAttributionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Classified => "classified",
            Self::MissingProvenance => "missing-provenance",
            Self::Ambiguous => "ambiguous",
            Self::StaleRef => "stale-ref",
            Self::UnsupportedKind => "unsupported-kind",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoopCoverageAttributionSignal {
    pub signal_kind: String,
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_hint: Option<LoopCoverageProvenanceClass>,
    #[serde(default)]
    pub stale: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoopCoverageAttributionArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub artifact_ref: String,
    pub artifact_kind: String,
    pub status: LoopCoverageAttributionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_class: Option<LoopCoverageProvenanceClass>,
    #[serde(default)]
    pub source_signals: Vec<LoopCoverageAttributionSignal>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub verdict_refs: Vec<String>,
    #[serde(default)]
    pub transaction_refs: Vec<String>,
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageAttributionReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub artifact_ref: String,
    pub artifact_kind: String,
    pub status: LoopCoverageAttributionStatus,
    pub provenance_class: Option<LoopCoverageProvenanceClass>,
    pub evidence_ref_count: usize,
    pub verdict_ref_count: usize,
    pub transaction_ref_count: usize,
    pub reasons: Vec<String>,
    pub boundary: String,
}

struct AttributionResolution {
    status: LoopCoverageAttributionStatus,
    provenance_class: Option<LoopCoverageProvenanceClass>,
    reasons: Vec<String>,
}

pub fn validate_loop_coverage_attribution(
    artifact: &LoopCoverageAttributionArtifact,
) -> Result<LoopCoverageAttributionReadModel> {
    if artifact.schema_version != LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION {
        return Err(anyhow!(
            "loop coverage attribution schemaVersion must be {LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION}"
        ));
    }
    validate_ref("artifactRef", &artifact.artifact_ref)?;
    validate_refs("evidenceRefs", &artifact.evidence_refs)?;
    validate_refs("verdictRefs", &artifact.verdict_refs)?;
    validate_refs("transactionRefs", &artifact.transaction_refs)?;
    for signal in &artifact.source_signals {
        if signal.signal_kind.trim().is_empty() {
            return Err(anyhow!("sourceSignals signalKind must be non-empty"));
        }
        validate_ref("sourceSignals sourceRef", &signal.source_ref)?;
    }
    if !artifact.boundary.contains("descriptive")
        || !artifact.boundary.contains("not quality")
        || !artifact.boundary.contains("no auto-apply")
        || !artifact.boundary.contains("read-only")
    {
        return Err(anyhow!("loop coverage attribution boundary must state descriptive authorship, not quality, no auto-apply, and read-only UI constraints"));
    }

    let resolution = resolve_attribution(artifact);
    if artifact.status != resolution.status {
        return Err(anyhow!(
            "declared status `{}` does not match computed status `{}`",
            artifact.status.as_str(),
            resolution.status.as_str()
        ));
    }
    if artifact.provenance_class != resolution.provenance_class {
        return Err(anyhow!(
            "declared provenanceClass does not match computed class for status `{}`",
            artifact.status.as_str()
        ));
    }
    match artifact.status {
        LoopCoverageAttributionStatus::Classified => {
            let class = artifact
                .provenance_class
                .ok_or_else(|| anyhow!("classified attribution requires provenanceClass"))?;
            if matches!(
                class,
                LoopCoverageProvenanceClass::LoopProduced
                    | LoopCoverageProvenanceClass::LoopVerified
            ) && (artifact.evidence_refs.is_empty() || artifact.verdict_refs.is_empty())
            {
                return Err(anyhow!(
                    "loop-produced/loop-verified attribution requires evidenceRefs and verdictRefs"
                ));
            }
        }
        _ if artifact.blocked_reasons.is_empty() => {
            return Err(anyhow!(
                "non-classified attribution state `{}` requires blockedReasons",
                artifact.status.as_str()
            ));
        }
        _ => {}
    }

    Ok(LoopCoverageAttributionReadModel {
        schema_version: LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION.to_string(),
        artifact_ref: artifact.artifact_ref.clone(),
        artifact_kind: artifact.artifact_kind.clone(),
        status: artifact.status,
        provenance_class: artifact.provenance_class,
        evidence_ref_count: artifact.evidence_refs.len(),
        verdict_ref_count: artifact.verdict_refs.len(),
        transaction_ref_count: artifact.transaction_refs.len(),
        reasons: resolution.reasons,
        boundary: artifact.boundary.clone(),
    })
}

fn resolve_attribution(artifact: &LoopCoverageAttributionArtifact) -> AttributionResolution {
    let mut reasons = Vec::new();
    if !SUPPORTED_ARTIFACT_KINDS.contains(&artifact.artifact_kind.as_str()) {
        reasons.push(format!(
            "unsupported artifactKind `{}`",
            artifact.artifact_kind
        ));
        return AttributionResolution {
            status: LoopCoverageAttributionStatus::UnsupportedKind,
            provenance_class: None,
            reasons,
        };
    }
    if artifact.source_signals.iter().any(|signal| signal.stale) {
        reasons.push("one or more sourceSignals are stale".to_string());
        return AttributionResolution {
            status: LoopCoverageAttributionStatus::StaleRef,
            provenance_class: None,
            reasons,
        };
    }
    let hinted: BTreeSet<_> = artifact
        .source_signals
        .iter()
        .filter_map(|signal| signal.class_hint)
        .collect();
    if hinted.len() > 1 {
        reasons.push("conflicting sourceSignals class hints are ambiguous".to_string());
        return AttributionResolution {
            status: LoopCoverageAttributionStatus::Ambiguous,
            provenance_class: None,
            reasons,
        };
    }
    if let Some(class) = hinted.iter().next().copied() {
        reasons.push(format!("classified as {}", class.as_str()));
        return AttributionResolution {
            status: LoopCoverageAttributionStatus::Classified,
            provenance_class: Some(class),
            reasons,
        };
    }
    if artifact.source_signals.is_empty()
        && artifact.evidence_refs.is_empty()
        && artifact.verdict_refs.is_empty()
    {
        reasons.push("no provenance, evidence, or verdict refs are present".to_string());
        return AttributionResolution {
            status: LoopCoverageAttributionStatus::MissingProvenance,
            provenance_class: None,
            reasons,
        };
    }
    reasons.push("no loop provenance hint found; defaulting conservatively to manual".to_string());
    AttributionResolution {
        status: LoopCoverageAttributionStatus::Classified,
        provenance_class: Some(LoopCoverageProvenanceClass::Manual),
        reasons,
    }
}

fn validate_refs(label: &str, refs: &[String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in refs {
        validate_ref(label, value)?;
        if !seen.insert(value) {
            return Err(anyhow!("{label} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}
fn validate_ref(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
    {
        return Err(anyhow!(
            "{label} must be a safe repo-relative or run-relative ref"
        ));
    }
    Ok(())
}
