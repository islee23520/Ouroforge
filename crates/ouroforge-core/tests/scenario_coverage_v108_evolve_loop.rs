//! Scenario Coverage v108: evidence-linked evolve loop (#2381-#2383).

use ouroforge_core::behavior_runtime::{BehaviorDiagnosticSeverity, BehaviorRuntimeDiagnostic};
use ouroforge_core::evolve_iteration_journal::{
    EvolveIterationEntry, EvolveIterationJournal, EvolveIterationOutcome,
    EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION,
};
use ouroforge_core::live_failure_classifier::{
    LiveFailureClass, LiveFailureClassification, LiveFailureClassifierStatus, LiveFailureSignal,
    LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION,
};
use ouroforge_core::product_backlog_handoff::{
    ProductBacklogHandoff, ProductBacklogStatus, ProductClosureClassification,
    PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION,
};
use ouroforge_core::product_gap_taxonomy::{product_gap_category_ids, product_gap_severity_ids};
use std::collections::BTreeSet;

fn signal(
    id: &str,
    class: LiveFailureClass,
    category: &str,
    severity: &str,
    owner: &str,
) -> LiveFailureSignal {
    LiveFailureSignal {
        signal_id: id.to_string(),
        class,
        category: category.to_string(),
        severity: severity.to_string(),
        next_owner: owner.to_string(),
        evidence_refs: vec![format!("runs/v108/{id}.json")],
        observed_behavior: format!("observed {id}"),
        expected_behavior: format!("expected {id}"),
        product_impact: "visible product-observed failure".to_string(),
        recommended_backlog_action: "triage with owner and rerun after fix".to_string(),
        runtime_diagnostics: vec![BehaviorRuntimeDiagnostic {
            severity: BehaviorDiagnosticSeverity::Warning,
            code: "unsupportedAction".to_string(),
            message: "diagnostic reused from runtime model".to_string(),
            behavior_id: Some("behavior-1".to_string()),
            item_id: None,
        }],
    }
}

fn classification() -> LiveFailureClassification {
    LiveFailureClassification {
        schema_version: LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION.to_string(),
        classification_id: "classification-v108".to_string(),
        bundle_ref: "runs/v108/bundle/manifest.json".to_string(),
        required_artifact_refs: vec![
            "runs/v108/bundle/manifest.json".to_string(),
            "runs/v108/bundle/console.jsonl".to_string(),
            "runs/v108/bundle/screenshots".to_string(),
        ],
        missing_artifact_refs: vec!["runs/v108/bundle/screenshots".to_string()],
        signals: vec![
            signal(
                "console",
                LiveFailureClass::ConsoleRuntime,
                "runtime_ux",
                "major",
                "runtime",
            ),
            signal(
                "objective",
                LiveFailureClass::GameplayObjective,
                "dogfood_game_quality",
                "blocking",
                "product",
            ),
            signal(
                "visual",
                LiveFailureClass::VisualReadability,
                "renderer_quality",
                "polish",
                "runtime",
            ),
            signal(
                "input",
                LiveFailureClass::InputControl,
                "input_control",
                "major",
                "runtime",
            ),
            signal(
                "authoring",
                LiveFailureClass::Authoring,
                "editor_workflow",
                "major",
                "studio",
            ),
            signal(
                "missing",
                LiveFailureClass::EvidenceMissing,
                "qa_evaluator_depth",
                "blocking",
                "qa",
            ),
            signal(
                "flake",
                LiveFailureClass::FlakeInconclusive,
                "qa_evaluator_depth",
                "major",
                "qa",
            ),
        ],
        blocked_reasons: vec![],
        boundary: "evidence-based read-only backlog routing; no automatic fix".to_string(),
    }
}

fn base_entry(id: &str, applied: bool, outcome: EvolveIterationOutcome) -> EvolveIterationEntry {
    EvolveIterationEntry {
        iteration_id: id.to_string(),
        hypothesis: "fix the planted live failure".to_string(),
        before_bundle_ref: format!("runs/v108/{id}/before.json"),
        proposal_ref: format!("runs/v108/{id}/proposal.json"),
        review_ref: applied.then(|| format!("runs/v108/{id}/review.json")),
        apply_ref: applied.then(|| format!("runs/v108/{id}/apply.json")),
        after_bundle_ref: applied.then(|| format!("runs/v108/{id}/after.json")),
        comparison_ref: applied.then(|| format!("runs/v108/{id}/comparison.json")),
        evidence_refs: vec![format!("runs/v108/{id}/evidence.json")],
        outcome,
        decision: "record result and route next action".to_string(),
        next_action: "create backlog candidate".to_string(),
        applied_change: applied,
    }
}

fn journal() -> EvolveIterationJournal {
    EvolveIterationJournal {
        schema_version: EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION.to_string(),
        journal_id: "journal-v108".to_string(),
        entries: vec![
            base_entry("iter-accepted", true, EvolveIterationOutcome::Accepted),
            base_entry(
                "iter-inconclusive",
                false,
                EvolveIterationOutcome::Inconclusive,
            ),
        ],
        guardrails: vec![
            "append-only; applied changes require review/apply/evidence links".to_string(),
        ],
    }
}

#[test]
fn classifier_reuses_m117_taxonomy_and_runtime_diagnostics_for_all_live_classes() {
    let categories = product_gap_category_ids().expect("taxonomy categories");
    let severities = product_gap_severity_ids().expect("taxonomy severities");
    assert!(categories.contains(&"qa_evaluator_depth".to_string()));
    assert_eq!(severities, vec!["blocking", "major", "polish"]);

    let artifact = classification();
    artifact.validate().expect("valid classification");
    let read_model = artifact.read_model();
    assert_eq!(read_model.status, LiveFailureClassifierStatus::Blocked);
    assert_eq!(read_model.signal_count, 7);
    assert_eq!(read_model.missing_artifact_count, 1);
    assert!(read_model
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("missing required artifacts")));
    assert!(artifact
        .signals
        .iter()
        .all(|signal| !signal.runtime_diagnostics.is_empty()));
}

#[test]
fn missing_artifacts_must_classify_as_evidence_missing_not_pass() {
    let mut artifact = classification();
    artifact
        .signals
        .retain(|signal| signal.class != LiveFailureClass::EvidenceMissing);
    let error = artifact
        .validate()
        .expect_err("missing artifacts require evidence-missing")
        .to_string();
    assert!(error.contains("evidence-missing"), "{error}");
}

#[test]
fn journal_is_append_only_and_applied_changes_cannot_omit_links() {
    let journal = journal();
    journal.validate().expect("valid journal");
    let next = journal
        .append_entry(base_entry(
            "iter-regressed",
            true,
            EvolveIterationOutcome::Regressed,
        ))
        .expect("append entry");
    next.validate_is_append_of(&journal).expect("append-only");

    let mut rewritten = next.clone();
    rewritten.entries[0].decision = "rewritten".to_string();
    let error = rewritten
        .validate_is_append_of(&journal)
        .expect_err("rewrite rejected")
        .to_string();
    assert!(error.contains("rewrote entry"), "{error}");

    let mut missing_review = base_entry("iter-bad", true, EvolveIterationOutcome::Accepted);
    missing_review.review_ref = None;
    let error = missing_review
        .validate()
        .expect_err("applied change requires review")
        .to_string();
    assert!(error.contains("reviewRef"), "{error}");
}

#[test]
fn journal_reference_existence_checks_keep_outside_review_reconstructable() {
    let journal = journal();
    let mut refs = BTreeSet::new();
    for entry in &journal.entries {
        refs.insert(entry.before_bundle_ref.clone());
        refs.insert(entry.proposal_ref.clone());
        refs.extend(entry.evidence_refs.iter().cloned());
        for reference in [
            &entry.review_ref,
            &entry.apply_ref,
            &entry.after_bundle_ref,
            &entry.comparison_ref,
        ]
        .into_iter()
        .flatten()
        {
            refs.insert(reference.clone());
        }
    }
    journal
        .validate_references_exist(&refs)
        .expect("refs exist");
    refs.remove("runs/v108/iter-accepted/comparison.json");
    let error = journal
        .validate_references_exist(&refs)
        .expect_err("missing ref rejected")
        .to_string();
    assert!(error.contains("missing from known evidence set"), "{error}");
}

#[test]
fn backlog_handoff_blocks_product_observed_close_with_untriaged_blocking_items() {
    let handoff = ProductBacklogHandoff::from_classification_and_journal(
        "handoff-v108",
        "runs/v108/classification.json",
        &classification(),
        "runs/v108/journal.json",
        &journal(),
    )
    .expect("handoff");
    assert_eq!(
        handoff.schema_version,
        PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION
    );
    assert!(handoff
        .entries
        .iter()
        .any(|entry| { entry.category == "dogfood_game_quality" && entry.severity == "blocking" }));
    let evaluation =
        handoff.evaluate_closure(ProductClosureClassification::ProductObservedComplete);
    assert!(!evaluation.closure_allowed);
    assert!(!evaluation.blocking_items.is_empty());

    let mut triaged = handoff.clone();
    for entry in &mut triaged.entries {
        if entry.severity == "blocking" {
            entry.status = ProductBacklogStatus::Triaged;
        }
    }
    triaged.validate().expect("triaged handoff remains valid");
    assert!(
        triaged
            .evaluate_closure(ProductClosureClassification::ProductObservedComplete)
            .closure_allowed
    );
    assert!(
        handoff
            .evaluate_closure(ProductClosureClassification::ContractComplete)
            .closure_allowed
    );
}
