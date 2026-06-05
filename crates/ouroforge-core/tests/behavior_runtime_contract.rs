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
fn executes_supported_trigger_conditions_and_actions_deterministically() {
    use ouroforge_core::behavior_runtime::{
        BehaviorExecutionInput, BehaviorTerminalState, BehaviorWorldState,
    };

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let input = BehaviorExecutionInput::new("onInputAction").with_input_action("jump");
    let world = BehaviorWorldState::default()
        .with_flag("grounded", true)
        .with_position("player", 10, 10);

    let first = artifact.execute(input.clone(), world.clone());
    let replay = artifact.execute(input, world);

    assert_eq!(
        first, replay,
        "same input and world replay deterministically"
    );
    assert_eq!(
        first
            .applied_actions
            .iter()
            .map(|action| action.action_id.as_str())
            .collect::<Vec<_>>(),
        vec!["mark-jumped", "jump-motion", "jump-audio"]
    );
    assert_eq!(first.world_state.flags.get("jumped"), Some(&true));
    assert_eq!(first.world_state.entity_positions["player"].x, 10);
    assert_eq!(first.world_state.entity_positions["player"].y, 8);
    assert_eq!(first.world_state.audio_intents[0].intent, "jump");
    assert_eq!(first.world_state.terminal_state, None);

    let goal = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("goalReached"),
        BehaviorWorldState::default().with_item("key"),
    );

    assert_eq!(
        goal.applied_actions
            .iter()
            .map(|action| action.action_id.as_str())
            .collect::<Vec<_>>(),
        vec!["win", "victory-event"]
    );
    assert_eq!(
        goal.world_state.terminal_state,
        Some(BehaviorTerminalState::Win)
    );
    assert_eq!(goal.world_state.events, vec!["goalReached", "victory"]);
}

#[test]
fn conditions_block_actions_and_unsupported_actions_remain_warning_only() {
    use ouroforge_core::behavior_runtime::{BehaviorExecutionInput, BehaviorWorldState};

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let blocked = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default().with_flag("grounded", false),
    );

    assert!(blocked.applied_actions.is_empty());
    assert!(!blocked.world_state.flags.contains_key("jumped"));

    let unsupported = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.unsupported.json"
    ))
    .expect("unsupported kinds are loadable with warnings");
    let report = unsupported.execute(
        BehaviorExecutionInput::new("onScriptHook"),
        BehaviorWorldState::default(),
    );

    assert!(report.applied_actions.is_empty());
    assert_eq!(report.diagnostics.len(), 4);
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "unsupportedAction"));
}

#[test]
fn evidence_bundle_exposes_runtime_reports_replay_keys_and_boundary() {
    use ouroforge_core::behavior_runtime::{BehaviorExecutionInput, BehaviorWorldState};

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let first = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default()
            .with_flag("grounded", true)
            .with_position("player", 2, 5),
    );
    let replay = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default()
            .with_flag("grounded", true)
            .with_position("player", 2, 5),
    );
    let goal = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("goalReached"),
        BehaviorWorldState::default().with_item("key"),
    );

    let bundle = artifact.evidence_bundle(vec![first, replay, goal]);

    assert_eq!(
        bundle.schema_version,
        "ouroforge.behavior-runtime-evidence.v1"
    );
    assert_eq!(bundle.summary.report_count, 3);
    assert_eq!(bundle.summary.applied_action_count, 8);
    assert_eq!(bundle.summary.diagnostic_count, 0);
    assert_eq!(bundle.summary.terminal_state_count, 1);
    assert_eq!(bundle.reports[0].replay_key, bundle.reports[1].replay_key);
    assert_ne!(bundle.reports[1].replay_key, bundle.reports[2].replay_key);
    assert_eq!(
        bundle.reports[0].applied_action_ids,
        vec!["mark-jumped", "jump-motion", "jump-audio"]
    );
    assert_eq!(
        bundle.reports[2].applied_action_ids,
        vec!["win", "victory-event"]
    );
    assert!(bundle
        .trusted_boundary
        .execution_mode
        .contains("structured-data-only"));
    assert!(bundle
        .trusted_boundary
        .disallowed_actions
        .iter()
        .any(|action| action == "dynamic import"));
}

#[test]
fn behavior_scenario_assertions_validate_and_pass_against_evidence() {
    use ouroforge_core::behavior_runtime::{
        BehaviorExecutionInput, BehaviorScenarioAssertion, BehaviorScenarioAssertionKind,
        BehaviorScenarioAssertionStatus, BehaviorTerminalState, BehaviorWorldState,
    };

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let jump = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default()
            .with_flag("grounded", true)
            .with_position("player", 2, 5),
    );
    let goal = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("goalReached"),
        BehaviorWorldState::default().with_item("key"),
    );
    let evidence = artifact.evidence_bundle(vec![jump, goal]);
    let assertions = vec![
        BehaviorScenarioAssertion {
            assertion_id: "behavior-executed".to_string(),
            kind: BehaviorScenarioAssertionKind::BehaviorExecuted,
            report_index: Some(0),
            behavior_id: None,
            action_id: Some("jump-motion".to_string()),
            event: None,
            flag: None,
            expected_flag: None,
            state: None,
            entity_id: None,
            item: None,
            terminal_state: None,
        },
        BehaviorScenarioAssertion {
            assertion_id: "flag-changed".to_string(),
            kind: BehaviorScenarioAssertionKind::FlagChanged,
            report_index: Some(0),
            behavior_id: None,
            action_id: None,
            event: None,
            flag: Some("jumped".to_string()),
            expected_flag: Some(true),
            state: None,
            entity_id: None,
            item: None,
            terminal_state: None,
        },
        BehaviorScenarioAssertion {
            assertion_id: "event-emitted".to_string(),
            kind: BehaviorScenarioAssertionKind::EventEmitted,
            report_index: Some(1),
            behavior_id: None,
            action_id: None,
            event: Some("victory".to_string()),
            flag: None,
            expected_flag: None,
            state: None,
            entity_id: None,
            item: None,
            terminal_state: None,
        },
        BehaviorScenarioAssertion {
            assertion_id: "terminal-state".to_string(),
            kind: BehaviorScenarioAssertionKind::TerminalStateReached,
            report_index: Some(1),
            behavior_id: None,
            action_id: None,
            event: None,
            flag: None,
            expected_flag: None,
            state: None,
            entity_id: None,
            item: None,
            terminal_state: Some(BehaviorTerminalState::Win),
        },
    ];

    for assertion in assertions {
        assertion.validate().expect("assertion validates");
        let verdict = assertion.evaluate(&evidence).expect("assertion evaluates");

        assert_eq!(verdict.status, BehaviorScenarioAssertionStatus::Passed);
    }
}

#[test]
fn behavior_scenario_assertions_fail_and_reject_malformed_shapes() {
    use ouroforge_core::behavior_runtime::{
        BehaviorExecutionInput, BehaviorScenarioAssertion, BehaviorScenarioAssertionKind,
        BehaviorScenarioAssertionStatus, BehaviorWorldState,
    };

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let evidence = artifact.evidence_bundle(vec![artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default().with_flag("grounded", false),
    )]);
    let failed = BehaviorScenarioAssertion {
        assertion_id: "missing-action".to_string(),
        kind: BehaviorScenarioAssertionKind::BehaviorExecuted,
        report_index: Some(0),
        behavior_id: None,
        action_id: Some("jump-motion".to_string()),
        event: None,
        flag: None,
        expected_flag: None,
        state: None,
        entity_id: None,
        item: None,
        terminal_state: None,
    }
    .evaluate(&evidence)
    .expect("failed assertion still evaluates");

    assert_eq!(failed.status, BehaviorScenarioAssertionStatus::Failed);

    let malformed = BehaviorScenarioAssertion {
        assertion_id: "bad-flag".to_string(),
        kind: BehaviorScenarioAssertionKind::FlagChanged,
        report_index: None,
        behavior_id: None,
        action_id: None,
        event: None,
        flag: Some("jumped".to_string()),
        expected_flag: None,
        state: None,
        entity_id: None,
        item: None,
        terminal_state: None,
    };

    assert!(format!(
        "{:?}",
        malformed.validate().expect_err("missing expectedFlag")
    )
    .contains("expectedFlag is required"));
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
