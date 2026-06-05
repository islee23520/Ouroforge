use super::{BehaviorEvidenceBundleArtifact, BehaviorEvidenceBundleStatus};
use crate::EvidenceArtifact;
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

const BOUNDARY: &str = "read-only structured behavior lifecycle evidence; dashboard/Studio surfaces must not execute scripts, eval, dynamic import, load plugins, run a command bridge, mutate source or trusted files, publish, deploy, auto-apply, or write generated runtime state.";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardBehaviorEvidence {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub bundle_count: usize,
    pub malformed_count: usize,
    pub lifecycle_ref_count: usize,
    pub observed_failure_count: usize,
    pub next_step_hypothesis_count: usize,
    pub blocked_reasons: Vec<String>,
    pub bundle_refs: Vec<String>,
    pub bundles: Vec<RunDashboardBehaviorEvidenceBundle>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardBehaviorEvidenceBundle {
    pub bundle_id: String,
    pub status: String,
    pub path: String,
    pub lifecycle_ref_count: usize,
    pub behavior_definition_ref_count: usize,
    pub runtime_event_ref_count: usize,
    pub scenario_outcome_ref_count: usize,
    pub draft_ref_count: usize,
    pub review_decision_ref_count: usize,
    pub apply_transaction_ref_count: usize,
    pub rollback_metadata_ref_count: usize,
    pub rerun_comparison_ref_count: usize,
    pub observed_failures: Vec<RunDashboardBehaviorObservedFailure>,
    pub next_step_hypotheses: Vec<RunDashboardBehaviorNextStepHypothesis>,
    pub evidence_refs: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub guardrails: Vec<String>,
    pub read_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardBehaviorObservedFailure {
    pub scenario_id: String,
    pub summary: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardBehaviorNextStepHypothesis {
    pub id: String,
    pub summary: String,
}

pub fn read_dashboard_behavior_evidence(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardBehaviorEvidence> {
    let artifacts = evidence
        .iter()
        .filter(|artifact| dashboard_artifact_is_behavior_evidence_bundle(artifact))
        .collect::<Vec<_>>();
    if artifacts.is_empty() {
        return Ok(RunDashboardBehaviorEvidence {
            present: false,
            empty_state: "No behavior evidence bundle is indexed for this run.".to_string(),
            status: "missing".to_string(),
            bundle_count: 0,
            malformed_count: 0,
            lifecycle_ref_count: 0,
            observed_failure_count: 0,
            next_step_hypothesis_count: 0,
            blocked_reasons: Vec::new(),
            bundle_refs: Vec::new(),
            bundles: Vec::new(),
            boundary: BOUNDARY.to_string(),
        });
    }

    let mut malformed_count = 0usize;
    let mut bundle_refs = Vec::new();
    let mut bundles = Vec::new();
    for artifact in artifacts {
        bundle_refs.push(artifact.path.clone());
        let path = run_dir.join(&artifact.path);
        let row = match std::fs::read_to_string(&path)
            .map_err(|error| error.to_string())
            .and_then(|input| {
                BehaviorEvidenceBundleArtifact::from_json_str(&input)
                    .map_err(|error| error.to_string())
            }) {
            Ok(bundle) => dashboard_bundle_row(artifact.path.clone(), bundle),
            Err(error) => {
                malformed_count += 1;
                malformed_bundle_row(artifact.path.clone(), error)
            }
        };
        bundles.push(row);
    }
    bundles.sort_by(|left, right| left.path.cmp(&right.path));
    bundle_refs.sort();
    bundle_refs.dedup();
    let valid_bundles = bundles
        .iter()
        .filter(|bundle| bundle.read_error.is_none())
        .collect::<Vec<_>>();
    let bundle_count = valid_bundles.len();
    let lifecycle_ref_count = valid_bundles
        .iter()
        .map(|bundle| bundle.lifecycle_ref_count)
        .sum();
    let observed_failure_count = valid_bundles
        .iter()
        .map(|bundle| bundle.observed_failures.len())
        .sum();
    let next_step_hypothesis_count = valid_bundles
        .iter()
        .map(|bundle| bundle.next_step_hypotheses.len())
        .sum();
    let mut blocked_reasons = valid_bundles
        .iter()
        .flat_map(|bundle| bundle.blocked_reasons.iter().cloned())
        .collect::<Vec<_>>();
    blocked_reasons.sort();
    blocked_reasons.dedup();
    let blocked_count = blocked_reasons.len();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if bundle_count > 0 {
        "ready"
    } else {
        "missing"
    };
    Ok(RunDashboardBehaviorEvidence {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        bundle_count,
        malformed_count,
        lifecycle_ref_count,
        observed_failure_count,
        next_step_hypothesis_count,
        blocked_reasons,
        bundle_refs,
        bundles,
        boundary: BOUNDARY.to_string(),
    })
}

fn dashboard_artifact_is_behavior_evidence_bundle(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("behavior_evidence_bundle")
        || artifact.id.contains("behavior-evidence-bundle")
        || artifact.path.contains("behavior-evidence-bundle")
}

fn dashboard_bundle_row(
    path: String,
    bundle: BehaviorEvidenceBundleArtifact,
) -> RunDashboardBehaviorEvidenceBundle {
    let lifecycle_ref_count = lifecycle_ref_count(&bundle);
    let evidence_refs = bundle
        .linked_evidence
        .iter()
        .map(|reference| reference.path.clone())
        .collect::<Vec<_>>();
    RunDashboardBehaviorEvidenceBundle {
        bundle_id: bundle.bundle_id,
        status: bundle_status_label(bundle.status).to_string(),
        path,
        lifecycle_ref_count,
        behavior_definition_ref_count: bundle.behavior_definition_refs.len(),
        runtime_event_ref_count: bundle.runtime_event_refs.len(),
        scenario_outcome_ref_count: bundle.scenario_outcome_refs.len(),
        draft_ref_count: bundle.draft_refs.len(),
        review_decision_ref_count: bundle.review_decision_refs.len(),
        apply_transaction_ref_count: bundle.apply_transaction_refs.len(),
        rollback_metadata_ref_count: bundle.rollback_metadata_refs.len(),
        rerun_comparison_ref_count: bundle.rerun_comparison_refs.len(),
        observed_failures: bundle
            .observed_failures
            .into_iter()
            .map(|failure| RunDashboardBehaviorObservedFailure {
                scenario_id: failure.scenario_id,
                summary: failure.summary,
                evidence_ref: failure.evidence_ref.path,
            })
            .collect(),
        next_step_hypotheses: bundle
            .next_step_hypotheses
            .into_iter()
            .map(|hypothesis| RunDashboardBehaviorNextStepHypothesis {
                id: hypothesis.id,
                summary: hypothesis.summary,
            })
            .collect(),
        evidence_refs,
        blocked_reasons: bundle.blocked_reasons,
        guardrails: bundle.guardrails,
        read_error: None,
    }
}

fn malformed_bundle_row(path: String, error: String) -> RunDashboardBehaviorEvidenceBundle {
    RunDashboardBehaviorEvidenceBundle {
        bundle_id: "malformed-behavior-evidence-bundle".to_string(),
        status: "malformed".to_string(),
        path,
        lifecycle_ref_count: 0,
        behavior_definition_ref_count: 0,
        runtime_event_ref_count: 0,
        scenario_outcome_ref_count: 0,
        draft_ref_count: 0,
        review_decision_ref_count: 0,
        apply_transaction_ref_count: 0,
        rollback_metadata_ref_count: 0,
        rerun_comparison_ref_count: 0,
        observed_failures: Vec::new(),
        next_step_hypotheses: Vec::new(),
        evidence_refs: Vec::new(),
        blocked_reasons: Vec::new(),
        guardrails: Vec::new(),
        read_error: Some(error),
    }
}

fn lifecycle_ref_count(bundle: &BehaviorEvidenceBundleArtifact) -> usize {
    bundle.behavior_definition_refs.len()
        + bundle.runtime_event_refs.len()
        + bundle.scenario_outcome_refs.len()
        + bundle.draft_refs.len()
        + bundle.review_decision_refs.len()
        + bundle.apply_transaction_refs.len()
        + bundle.rollback_metadata_refs.len()
        + bundle.rerun_comparison_refs.len()
}

fn bundle_status_label(status: BehaviorEvidenceBundleStatus) -> &'static str {
    match status {
        BehaviorEvidenceBundleStatus::Complete => "complete",
        BehaviorEvidenceBundleStatus::Partial => "partial",
        BehaviorEvidenceBundleStatus::Blocked => "blocked",
        BehaviorEvidenceBundleStatus::Stale => "stale",
    }
}
