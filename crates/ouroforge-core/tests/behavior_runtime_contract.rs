use ouroforge_core::behavior_runtime::{BehaviorArtifact, BehaviorRuntimeStatus};

fn valid_fixture() -> &'static str {
    include_str!("../../../examples/behavior-runtime-v1/valid/behavior-artifact.sample.json")
}

#[test]
fn loads_validated_behavior_artifact_into_runtime_state() {
    let artifact = BehaviorArtifact::from_json_str(valid_fixture()).expect("valid behavior parses");
    let state = artifact.runtime_state();

    assert_eq!(artifact.artifact_id, "behavior-runtime-demo");
    assert_eq!(state.status, BehaviorRuntimeStatus::Ready);
    assert_eq!(state.counts.behavior_count, 1);
    assert_eq!(state.counts.trigger_count, 2);
    assert_eq!(state.counts.condition_count, 1);
    assert_eq!(state.counts.action_count, 3);
    assert_eq!(state.counts.state_machine_count, 1);
    assert_eq!(state.counts.ability_count, 1);
    assert_eq!(state.loaded_behavior_ids, vec!["player-startup"]);
    assert_eq!(
        state
            .entity_behavior_ids
            .get("player")
            .expect("player mapping"),
        &vec!["player-startup".to_string()]
    );
    assert!(state.diagnostics.is_empty());
    assert!(state
        .trusted_boundary
        .execution_mode
        .contains("no arbitrary scripts"));
    assert!(state
        .trusted_boundary
        .disallowed_actions
        .iter()
        .any(|action| action == "eval"));
}

#[test]
fn unsupported_behavior_parts_are_warning_recorded_not_silently_accepted() {
    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.unsupported.json"
    ))
    .expect("unsupported kinds are loadable with warnings");
    let state = artifact.runtime_state();
    let codes = state
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();

    assert_eq!(state.status, BehaviorRuntimeStatus::ReadyWithWarnings);
    assert_eq!(
        codes,
        vec![
            "unsupportedTrigger",
            "unsupportedCondition",
            "unsupportedAction",
            "unsupportedEffect"
        ]
    );
    assert!(state
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.behavior_id.as_deref() == Some("unsupported-demo")));
}

#[test]
fn rejects_malformed_or_untrusted_behavior_artifacts_at_loader_boundary() {
    let invalid_cases = [
        (
            include_str!("../../../examples/behavior-runtime-v1/invalid/missing-validated-by.json"),
            "missing field `validatedBy`",
        ),
        (
            include_str!(
                "../../../examples/behavior-runtime-v1/invalid/not-passed-validation.json"
            ),
            "validatedBy.validationStatus must be passed",
        ),
        (
            include_str!("../../../examples/behavior-runtime-v1/invalid/bad-state-transition.json"),
            "state transition target must reference a declared state",
        ),
    ];

    for (input, expected_error) in invalid_cases {
        let error = BehaviorArtifact::from_json_str(input).expect_err(expected_error);

        assert!(format!("{error:?}").contains(expected_error), "{error:?}");
    }
}
