use super::{
    rules, BehaviorEvidenceBundleArtifact, BehaviorEvidenceBundleValidation, BehaviorEvidenceRef,
    BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION,
};
use crate::behavior_draft_validation::{
    validate_path_component, validate_relative_artifact_path, validate_snapshot_hash,
};
use anyhow::{anyhow, Result};
use std::collections::BTreeSet;

const VALIDATION_SCHEMA_VERSION: &str = "behavior-evidence-bundle-validation-v1";

pub fn validate_behavior_evidence_bundle(
    bundle: &BehaviorEvidenceBundleArtifact,
) -> Result<BehaviorEvidenceBundleValidation> {
    let validation = inspect_behavior_evidence_bundle(bundle);
    if validation.is_blocked() {
        return Err(anyhow!(
            "behavior evidence bundle blocked: {}",
            validation.blocked_reasons.join("; ")
        ));
    }
    Ok(validation)
}

pub fn inspect_behavior_evidence_bundle(
    bundle: &BehaviorEvidenceBundleArtifact,
) -> BehaviorEvidenceBundleValidation {
    let mut blocked_reasons = Vec::new();
    inspect_identity(bundle, &mut blocked_reasons);
    inspect_ref_group(
        "behaviorDefinitionRefs",
        Some("behavior-definition"),
        &bundle.behavior_definition_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "runtimeEventRefs",
        Some("runtime-event-log"),
        &bundle.runtime_event_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "scenarioOutcomeRefs",
        Some("scenario-outcome"),
        &bundle.scenario_outcome_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "draftRefs",
        Some("behavior-draft"),
        &bundle.draft_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "reviewDecisionRefs",
        Some("behavior-review-decision"),
        &bundle.review_decision_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "applyTransactionRefs",
        Some("behavior-apply-transaction"),
        &bundle.apply_transaction_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "rollbackMetadataRefs",
        Some("behavior-rollback-metadata"),
        &bundle.rollback_metadata_refs,
        &mut blocked_reasons,
    );
    inspect_ref_group(
        "rerunComparisonRefs",
        Some("behavior-rerun-comparison"),
        &bundle.rerun_comparison_refs,
        &mut blocked_reasons,
    );
    rules::inspect_failures_and_hypotheses(bundle, &mut blocked_reasons);
    rules::inspect_lifecycle_completeness(bundle, &mut blocked_reasons);
    rules::inspect_guardrails(bundle, &mut blocked_reasons);
    blocked_reasons.sort();
    blocked_reasons.dedup();
    BehaviorEvidenceBundleValidation {
        schema_version: VALIDATION_SCHEMA_VERSION.to_string(),
        status: if blocked_reasons.is_empty() {
            "passed"
        } else {
            "blocked"
        }
        .to_string(),
        bundle_id: bundle.bundle_id.clone(),
        lifecycle_ref_count: rules::lifecycle_ref_count(bundle),
        blocked_reasons,
        guardrails: vec![
            "behavior evidence bundle is inert lifecycle audit data".to_string(),
            "structured behavior evidence remains separate from arbitrary executable scripting"
                .to_string(),
            "browser/dashboard/Studio surfaces may inspect bundle fields read-only only"
                .to_string(),
            "generated behavior evidence remains untracked unless fixture-scoped".to_string(),
        ],
    }
}

fn inspect_identity(bundle: &BehaviorEvidenceBundleArtifact, blocked_reasons: &mut Vec<String>) {
    if bundle.schema_version != BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION {
        blocked_reasons.push(format!(
            "schemaVersion must be {BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION}"
        ));
    }
    if let Err(error) = validate_path_component("bundleId", &bundle.bundle_id) {
        blocked_reasons.push(error.to_string());
    }
}

pub(super) fn inspect_ref_group(
    field: &str,
    expected_kind: Option<&str>,
    refs: &[BehaviorEvidenceRef],
    blocked_reasons: &mut Vec<String>,
) {
    let mut paths = BTreeSet::new();
    for (index, artifact_ref) in refs.iter().enumerate() {
        inspect_ref(
            &format!("{field}[{index}]"),
            expected_kind,
            artifact_ref,
            blocked_reasons,
        );
        if !paths.insert(artifact_ref.path.as_str()) {
            blocked_reasons.push(format!("duplicate {field}.path: {}", artifact_ref.path));
        }
    }
}

pub(super) fn inspect_ref(
    field: &str,
    expected_kind: Option<&str>,
    artifact_ref: &BehaviorEvidenceRef,
    blocked_reasons: &mut Vec<String>,
) {
    if let Some(expected_kind) = expected_kind {
        if artifact_ref.kind != expected_kind {
            blocked_reasons.push(format!("{field}.kind must be {expected_kind}"));
        }
    }
    if let Err(error) =
        validate_relative_artifact_path(&format!("{field}.path"), &artifact_ref.path)
    {
        blocked_reasons.push(error.to_string());
    }
    if let Some(hash) = &artifact_ref.hash {
        if let Err(error) = validate_snapshot_hash(&format!("{field}.hash"), hash) {
            blocked_reasons.push(error.to_string());
        }
    }
}
