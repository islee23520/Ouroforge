use ouroforge_core::behavior_runtime::{BehaviorDiagnosticSeverity, BehaviorRuntimeDiagnostic};
use ouroforge_core::live_failure_classifier::{
    LiveFailureClass, LiveFailureClassification, LiveFailureClassifierStatus, LiveFailureSignal,
    LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION,
};
use ouroforge_core::product_gap_taxonomy::{product_gap_category_ids, product_gap_severity_ids};

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
        evidence_refs: vec![format!("runs/classifier/{id}.json")],
        observed_behavior: "observed live failure".to_string(),
        expected_behavior: "expected live success".to_string(),
        product_impact: "blocks honest product-observed closure".to_string(),
        recommended_backlog_action: "route to owner backlog".to_string(),
        runtime_diagnostics: vec![BehaviorRuntimeDiagnostic {
            severity: BehaviorDiagnosticSeverity::Warning,
            code: "unsupportedAction".to_string(),
            message: "reused runtime diagnostic".to_string(),
            behavior_id: Some("behavior-1".to_string()),
            item_id: None,
        }],
    }
}

fn classification() -> LiveFailureClassification {
    LiveFailureClassification {
        schema_version: LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION.to_string(),
        classification_id: "classification-2381".to_string(),
        bundle_ref: "runs/classifier/manifest.json".to_string(),
        required_artifact_refs: vec![
            "runs/classifier/manifest.json".to_string(),
            "runs/classifier/screenshots".to_string(),
        ],
        missing_artifact_refs: vec!["runs/classifier/screenshots".to_string()],
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

#[test]
fn classifier_reuses_product_gap_taxonomy_and_runtime_diagnostics() {
    assert!(product_gap_category_ids()
        .unwrap()
        .contains(&"qa_evaluator_depth".to_string()));
    assert_eq!(
        product_gap_severity_ids().unwrap(),
        vec!["blocking", "major", "polish"]
    );
    let artifact = classification();
    artifact.validate().expect("classification validates");
    let model = artifact.read_model();
    assert_eq!(model.status, LiveFailureClassifierStatus::Blocked);
    assert_eq!(model.signal_count, 7);
    assert!(artifact
        .signals
        .iter()
        .all(|signal| !signal.runtime_diagnostics.is_empty()));
    assert!(model.forbidden_actions.contains(&"auto_fix".to_string()));
}

#[test]
fn missing_required_artifact_requires_evidence_missing_classification() {
    let mut artifact = classification();
    artifact
        .signals
        .retain(|signal| signal.class != LiveFailureClass::EvidenceMissing);
    let error = artifact
        .validate()
        .expect_err("missing evidence must fail closed")
        .to_string();
    assert!(error.contains("evidence-missing"), "{error}");
}
