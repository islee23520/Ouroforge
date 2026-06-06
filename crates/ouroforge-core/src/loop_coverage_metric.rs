use crate::{LoopCoverageAttributionStatus, LoopCoverageProvenanceClass};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LOOP_COVERAGE_METRIC_SCHEMA_VERSION: &str = "loop-coverage-metric-v1";

const EPSILON: f64 = 0.000_001;

/// Artifact kinds the loop coverage metric contract supports as trusted-change
/// inputs (#1464). Any other kind is treated as `unsupported`, regardless of the
/// caller-supplied attribution status, so a stale or mislabeled `classified`
/// input cannot be counted as normal coverage.
const SUPPORTED_ARTIFACT_KINDS: &[&str] = &["trusted-change", "verdict"];

fn kind_supported(artifact_kind: &str) -> bool {
    SUPPORTED_ARTIFACT_KINDS.contains(&artifact_kind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoopCoverageVerdictState {
    Computed,
    InsufficientData,
    Regressed,
    Unsupported,
}

impl LoopCoverageVerdictState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Computed => "computed",
            Self::InsufficientData => "insufficient-data",
            Self::Regressed => "regressed",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageMetricInput {
    pub artifact_ref: String,
    pub artifact_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_class: Option<LoopCoverageProvenanceClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution_status: Option<LoopCoverageAttributionStatus>,
    #[serde(default)]
    pub loop_stage_refs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted_validation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageCounts {
    pub loop_produced: usize,
    pub loop_verified: usize,
    pub manual: usize,
    pub total_trusted: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageFractions {
    pub loop_produced: f64,
    pub loop_verified: f64,
    pub manual: f64,
    pub loop_covered: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageVerdict {
    pub state: LoopCoverageVerdictState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_loop_covered: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_loop_covered: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_threshold: Option<f64>,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCoverageEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_ref: Option<String>,
    #[serde(default)]
    pub inputs: Vec<LoopCoverageMetricInput>,
    pub counts: LoopCoverageCounts,
    pub fractions: LoopCoverageFractions,
    pub verdict: LoopCoverageVerdict,
    pub boundary: String,
}

pub fn compute_loop_coverage_evidence(
    project_id: Option<String>,
    run_id: Option<String>,
    baseline_ref: Option<String>,
    inputs: Vec<LoopCoverageMetricInput>,
    baseline: Option<&LoopCoverageEvidenceArtifact>,
    drop_threshold: f64,
) -> LoopCoverageEvidenceArtifact {
    let counts = compute_counts(&inputs);
    let fractions = compute_fractions(counts);
    let verdict = compute_verdict(&inputs, fractions, baseline, drop_threshold);
    LoopCoverageEvidenceArtifact {
        schema_version: LOOP_COVERAGE_METRIC_SCHEMA_VERSION.to_string(),
        project_id,
        run_id,
        baseline_ref,
        inputs,
        counts,
        fractions,
        verdict,
        boundary: "Loop coverage is descriptive authorship attribution only, not quality, not production readiness, no auto-apply authority, and read-only in dashboard and Studio surfaces.".to_string(),
    }
}

pub fn validate_loop_coverage_evidence(artifact: &LoopCoverageEvidenceArtifact) -> Result<()> {
    if artifact.schema_version != LOOP_COVERAGE_METRIC_SCHEMA_VERSION {
        return Err(anyhow!(
            "loop coverage evidence schemaVersion must be {LOOP_COVERAGE_METRIC_SCHEMA_VERSION}"
        ));
    }
    require_boundary(&artifact.boundary)?;
    for input in &artifact.inputs {
        validate_ref("artifactRef", &input.artifact_ref)?;
        if let Some(source_ref) = &input.source_ref {
            validate_ref("sourceRef", source_ref)?;
        }
        if let Some(validation_ref) = &input.trusted_validation_ref {
            validate_ref("trustedValidationRef", validation_ref)?;
        }
        for (stage, reference) in &input.loop_stage_refs {
            if stage.trim().is_empty() {
                return Err(anyhow!("loopStageRefs keys must be non-empty"));
            }
            validate_ref("loopStageRefs value", reference)?;
        }
    }
    if let Some(baseline_ref) = &artifact.baseline_ref {
        validate_ref("baselineRef", baseline_ref)?;
    }
    let computed_counts = compute_counts(&artifact.inputs);
    if artifact.counts != computed_counts {
        return Err(anyhow!("loop coverage counts do not match inputs"));
    }
    let computed_fractions = compute_fractions(computed_counts);
    assert_fraction(
        "loopProduced",
        artifact.fractions.loop_produced,
        computed_fractions.loop_produced,
    )?;
    assert_fraction(
        "loopVerified",
        artifact.fractions.loop_verified,
        computed_fractions.loop_verified,
    )?;
    assert_fraction(
        "manual",
        artifact.fractions.manual,
        computed_fractions.manual,
    )?;
    assert_fraction(
        "loopCovered",
        artifact.fractions.loop_covered,
        computed_fractions.loop_covered,
    )?;
    validate_verdict_shape(artifact)?;
    Ok(())
}

fn compute_counts(inputs: &[LoopCoverageMetricInput]) -> LoopCoverageCounts {
    let mut counts = LoopCoverageCounts {
        loop_produced: 0,
        loop_verified: 0,
        manual: 0,
        total_trusted: 0,
    };
    for input in inputs {
        if !kind_supported(&input.artifact_kind) {
            continue;
        }
        if input
            .attribution_status
            .is_some_and(|status| status != LoopCoverageAttributionStatus::Classified)
        {
            continue;
        }
        match input.provenance_class {
            Some(LoopCoverageProvenanceClass::LoopProduced) => {
                counts.loop_produced += 1;
                counts.total_trusted += 1;
            }
            Some(LoopCoverageProvenanceClass::LoopVerified) => {
                counts.loop_verified += 1;
                counts.total_trusted += 1;
            }
            Some(LoopCoverageProvenanceClass::Manual) => {
                counts.manual += 1;
                counts.total_trusted += 1;
            }
            None => {}
        }
    }
    counts
}

fn compute_fractions(counts: LoopCoverageCounts) -> LoopCoverageFractions {
    if counts.total_trusted == 0 {
        return LoopCoverageFractions {
            loop_produced: 0.0,
            loop_verified: 0.0,
            manual: 0.0,
            loop_covered: 0.0,
        };
    }
    let total = counts.total_trusted as f64;
    let loop_produced = counts.loop_produced as f64 / total;
    let loop_verified = counts.loop_verified as f64 / total;
    let manual = counts.manual as f64 / total;
    LoopCoverageFractions {
        loop_produced,
        loop_verified,
        manual,
        loop_covered: loop_produced + loop_verified,
    }
}

fn compute_verdict(
    inputs: &[LoopCoverageMetricInput],
    fractions: LoopCoverageFractions,
    baseline: Option<&LoopCoverageEvidenceArtifact>,
    drop_threshold: f64,
) -> LoopCoverageVerdict {
    let mut reasons = Vec::new();
    // Regression is only meaningful against a declared, finite, non-negative
    // tolerance (#1461). A negative or non-finite threshold could mark unchanged
    // coverage as regressed or suppress a real drop, so refuse to decide.
    if !drop_threshold.is_finite() || drop_threshold < 0.0 {
        return verdict(
            LoopCoverageVerdictState::InsufficientData,
            None,
            Some(fractions.loop_covered),
            None,
            vec!["dropThreshold must be a finite, non-negative tolerance".to_string()],
        );
    }
    if inputs.is_empty() {
        return verdict(
            LoopCoverageVerdictState::InsufficientData,
            None,
            Some(fractions.loop_covered),
            Some(drop_threshold),
            vec!["no trusted artifact inputs were supplied".to_string()],
        );
    }
    for input in inputs {
        // An artifact kind outside the metric contract is unsupported regardless
        // of the caller-supplied attribution status (#1464), so a stale or
        // mislabeled `classified` input cannot be counted as normal coverage.
        if !kind_supported(&input.artifact_kind) {
            return verdict(
                LoopCoverageVerdictState::Unsupported,
                None,
                Some(fractions.loop_covered),
                Some(drop_threshold),
                vec![format!(
                    "artifact `{}` has unsupported kind `{}`",
                    input.artifact_ref, input.artifact_kind
                )],
            );
        }
        match input.attribution_status {
            Some(LoopCoverageAttributionStatus::UnsupportedKind) => {
                return verdict(
                    LoopCoverageVerdictState::Unsupported,
                    None,
                    Some(fractions.loop_covered),
                    Some(drop_threshold),
                    vec![format!(
                        "artifact `{}` has unsupported attribution kind",
                        input.artifact_ref
                    )],
                );
            }
            Some(LoopCoverageAttributionStatus::MissingProvenance)
            | Some(LoopCoverageAttributionStatus::Ambiguous)
            | Some(LoopCoverageAttributionStatus::StaleRef) => reasons.push(format!(
                "artifact `{}` has attribution status `{}`",
                input.artifact_ref,
                input.attribution_status.unwrap().as_str()
            )),
            _ if input.provenance_class.is_none() => reasons.push(format!(
                "artifact `{}` is missing provenanceClass",
                input.artifact_ref
            )),
            _ => {}
        }
    }
    if !reasons.is_empty() {
        return verdict(
            LoopCoverageVerdictState::InsufficientData,
            None,
            Some(fractions.loop_covered),
            Some(drop_threshold),
            reasons,
        );
    }
    let Some(baseline) = baseline else {
        return verdict(
            LoopCoverageVerdictState::InsufficientData,
            None,
            Some(fractions.loop_covered),
            Some(drop_threshold),
            vec!["no baseline loop coverage evidence was supplied".to_string()],
        );
    };
    if validate_loop_coverage_evidence(baseline).is_err()
        || baseline.verdict.state == LoopCoverageVerdictState::Unsupported
        || baseline.counts.total_trusted == 0
    {
        return verdict(
            LoopCoverageVerdictState::InsufficientData,
            None,
            Some(fractions.loop_covered),
            Some(drop_threshold),
            vec!["baseline loop coverage evidence is missing or malformed".to_string()],
        );
    }
    let baseline_loop_covered = baseline.fractions.loop_covered;
    let drop = baseline_loop_covered - fractions.loop_covered;
    if drop > drop_threshold {
        verdict(
            LoopCoverageVerdictState::Regressed,
            Some(baseline_loop_covered),
            Some(fractions.loop_covered),
            Some(drop_threshold),
            vec![format!(
                "loop-covered fraction dropped by {:.6}, exceeding threshold {:.6}",
                drop, drop_threshold
            )],
        )
    } else {
        verdict(
            LoopCoverageVerdictState::Computed,
            Some(baseline_loop_covered),
            Some(fractions.loop_covered),
            Some(drop_threshold),
            vec!["loop coverage computed against baseline without regression".to_string()],
        )
    }
}

fn verdict(
    state: LoopCoverageVerdictState,
    baseline_loop_covered: Option<f64>,
    current_loop_covered: Option<f64>,
    drop_threshold: Option<f64>,
    reasons: Vec<String>,
) -> LoopCoverageVerdict {
    LoopCoverageVerdict {
        state,
        baseline_loop_covered,
        current_loop_covered,
        drop_threshold,
        reasons,
    }
}

fn validate_verdict_shape(artifact: &LoopCoverageEvidenceArtifact) -> Result<()> {
    if artifact.verdict.reasons.is_empty() {
        return Err(anyhow!("loop coverage verdict requires reasons"));
    }
    if let Some(threshold) = artifact.verdict.drop_threshold {
        if !threshold.is_finite() || threshold < 0.0 {
            return Err(anyhow!(
                "loop coverage dropThreshold must be a finite, non-negative tolerance"
            ));
        }
    }
    if matches!(
        artifact.verdict.state,
        LoopCoverageVerdictState::Computed | LoopCoverageVerdictState::Regressed
    ) {
        artifact
            .verdict
            .baseline_loop_covered
            .ok_or_else(|| anyhow!("computed/regressed verdict requires baselineLoopCovered"))?;
        artifact
            .verdict
            .current_loop_covered
            .ok_or_else(|| anyhow!("computed/regressed verdict requires currentLoopCovered"))?;
        artifact
            .verdict
            .drop_threshold
            .ok_or_else(|| anyhow!("computed/regressed verdict requires dropThreshold"))?;
    }
    Ok(())
}

fn require_boundary(boundary: &str) -> Result<()> {
    if !boundary.contains("descriptive")
        || !boundary.contains("not quality")
        || !boundary.contains("no auto-apply")
        || !boundary.contains("read-only")
    {
        return Err(anyhow!("loop coverage boundary must state descriptive authorship, not quality, no auto-apply, and read-only UI constraints"));
    }
    Ok(())
}

fn assert_fraction(field: &str, actual: f64, expected: f64) -> Result<()> {
    if !actual.is_finite() || (actual - expected).abs() > EPSILON {
        return Err(anyhow!(
            "loop coverage fraction {field} does not match inputs"
        ));
    }
    Ok(())
}

fn validate_ref(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must be non-empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.contains("..")
        || trimmed.chars().any(char::is_whitespace)
    {
        return Err(anyhow!(
            "{field} must be a local relative ref without whitespace or parent traversal"
        ));
    }
    Ok(())
}
