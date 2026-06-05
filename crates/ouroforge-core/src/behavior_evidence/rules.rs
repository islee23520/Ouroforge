use super::{validation, BehaviorEvidenceBundleArtifact, BehaviorEvidenceBundleStatus};
use crate::behavior_draft_validation::{require_text, validate_path_component};

pub(super) fn inspect_failures_and_hypotheses(
    bundle: &BehaviorEvidenceBundleArtifact,
    blocked_reasons: &mut Vec<String>,
) {
    for (index, failure) in bundle.observed_failures.iter().enumerate() {
        if let Err(error) = validate_path_component(
            &format!("observedFailures[{index}].scenarioId"),
            &failure.scenario_id,
        ) {
            blocked_reasons.push(error.to_string());
        }
        inspect_text(
            &format!("observedFailures[{index}].summary"),
            &failure.summary,
            blocked_reasons,
        );
        validation::inspect_ref(
            &format!("observedFailures[{index}].evidenceRef"),
            Some("scenario-outcome"),
            &failure.evidence_ref,
            blocked_reasons,
        );
    }
    for (index, hypothesis) in bundle.next_step_hypotheses.iter().enumerate() {
        if let Err(error) =
            validate_path_component(&format!("nextStepHypotheses[{index}].id"), &hypothesis.id)
        {
            blocked_reasons.push(error.to_string());
        }
        inspect_text(
            &format!("nextStepHypotheses[{index}].summary"),
            &hypothesis.summary,
            blocked_reasons,
        );
    }
}

pub(super) fn inspect_lifecycle_completeness(
    bundle: &BehaviorEvidenceBundleArtifact,
    blocked_reasons: &mut Vec<String>,
) {
    for (field, is_empty) in [
        (
            "behaviorDefinitionRefs",
            bundle.behavior_definition_refs.is_empty(),
        ),
        ("runtimeEventRefs", bundle.runtime_event_refs.is_empty()),
        (
            "scenarioOutcomeRefs",
            bundle.scenario_outcome_refs.is_empty(),
        ),
        ("draftRefs", bundle.draft_refs.is_empty()),
        ("linkedEvidence", bundle.linked_evidence.is_empty()),
    ] {
        if is_empty {
            blocked_reasons.push(format!("{field} must not be empty"));
        }
    }
    if matches!(bundle.status, BehaviorEvidenceBundleStatus::Complete) {
        inspect_complete_refs(bundle, blocked_reasons);
    }
    if matches!(
        bundle.status,
        BehaviorEvidenceBundleStatus::Blocked | BehaviorEvidenceBundleStatus::Stale
    ) && bundle.blocked_reasons.is_empty()
    {
        blocked_reasons.push("blocked or stale bundle requires blockedReasons".to_string());
    }
    validation::inspect_ref_group(
        "linkedEvidence",
        None,
        &bundle.linked_evidence,
        blocked_reasons,
    );
}

pub(super) fn inspect_guardrails(
    bundle: &BehaviorEvidenceBundleArtifact,
    blocked_reasons: &mut Vec<String>,
) {
    let joined = bundle.guardrails.join(" ").to_ascii_lowercase();
    for required in [
        "read-only",
        "no arbitrary script",
        "rust/local",
        "untracked",
    ] {
        if !joined.contains(required) {
            blocked_reasons.push(format!(
                "guardrails must state {required} behavior boundary"
            ));
        }
    }
}

pub(super) fn lifecycle_ref_count(bundle: &BehaviorEvidenceBundleArtifact) -> usize {
    bundle.behavior_definition_refs.len()
        + bundle.runtime_event_refs.len()
        + bundle.scenario_outcome_refs.len()
        + bundle.draft_refs.len()
        + bundle.review_decision_refs.len()
        + bundle.apply_transaction_refs.len()
        + bundle.rollback_metadata_refs.len()
        + bundle.rerun_comparison_refs.len()
}

fn inspect_text(field: &str, value: &str, blocked_reasons: &mut Vec<String>) {
    if let Err(error) = require_text(field, value) {
        blocked_reasons.push(error.to_string());
    }
    let normalized = value.to_ascii_lowercase();
    for forbidden in [
        "script",
        "eval",
        "dynamic import",
        "plugin loader",
        "command bridge",
    ] {
        if normalized.contains(forbidden) {
            blocked_reasons.push(format!(
                "{field} must not claim arbitrary executable scripting authority"
            ));
        }
    }
}

fn inspect_complete_refs(
    bundle: &BehaviorEvidenceBundleArtifact,
    blocked_reasons: &mut Vec<String>,
) {
    for (field, is_empty) in [
        ("reviewDecisionRefs", bundle.review_decision_refs.is_empty()),
        (
            "applyTransactionRefs",
            bundle.apply_transaction_refs.is_empty(),
        ),
        (
            "rollbackMetadataRefs",
            bundle.rollback_metadata_refs.is_empty(),
        ),
    ] {
        if is_empty {
            blocked_reasons.push(format!("complete bundle requires {field}"));
        }
    }
}
