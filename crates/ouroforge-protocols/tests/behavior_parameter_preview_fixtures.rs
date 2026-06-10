use ouroforge_protocols::behavior_parameter_preview::{
    BehaviorParameterDraftRequest, BehaviorPreviewStatus, HazardPreviewOutcome,
};

#[test]
fn accepted_hazard_parameter_fixture_generates_deterministic_preview_bundle() {
    let request = BehaviorParameterDraftRequest::from_json_str(include_str!(
        "fixtures/behavior-parameter-preview-v1/accepted/hazard-contact-threshold.json"
    ))
    .unwrap();
    let preview = request.preview();
    preview.validate().unwrap();
    assert_eq!(preview.status, BehaviorPreviewStatus::PreviewReady);
    assert_eq!(preview.before.outcome, HazardPreviewOutcome::ContactAllowed);
    assert_eq!(preview.after.outcome, HazardPreviewOutcome::ContactFails);
    assert_ne!(
        preview.before.final_state_digest,
        preview.after.final_state_digest
    );
    assert!(preview
        .replay_api
        .required_methods
        .iter()
        .any(|method| method == "runReplay"));
    assert_eq!(preview.expected_scenario_impact.len(), 1);
    assert!(preview.expected_scenario_impact[0]
        .evidence_refs
        .iter()
        .any(|reference| reference.path.ends_with("replay-digest.json")));
}

#[test]
fn rejected_hazard_parameter_fixture_fails_with_named_diagnostic() {
    let fixture = include_str!("fixtures/behavior-parameter-preview-v1/rejected/out-of-range.json");
    let err = BehaviorParameterDraftRequest::from_json_str(fixture)
        .unwrap_err()
        .to_string();
    assert!(err.contains("parameter_out_of_range"), "{err}");
}
