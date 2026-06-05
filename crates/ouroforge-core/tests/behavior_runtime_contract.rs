use ouroforge_core::behavior_runtime::{
    behavior_apply_transaction_read_model_from_json_str, BehaviorApplyTransactionArtifact,
    BehaviorArtifact, BehaviorDraftArtifact, BehaviorRuntimeStatus,
    BehaviorScenarioAssertionStatus, BehaviorScenarioAssertionSuite,
};
use ouroforge_core::{ProjectManifest, ScenarioPack, Seed};

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
fn behavior_cooldown_active_assertion_passes_for_fired_behavior_evidence() {
    use ouroforge_core::behavior_runtime::{
        BehaviorExecutionInput, BehaviorScenarioAssertion, BehaviorScenarioAssertionKind,
        BehaviorScenarioAssertionStatus, BehaviorWorldState,
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

    // The cooldown of a behavior is recorded as active once it applies actions, so the
    // assertion must observe the firing behavior in its own report's evidence.
    assert!(evidence.reports[0]
        .cooldown_behavior_ids
        .contains(&"player-jump".to_string()));

    let cooldown_template = |behavior_id: &str, report_index: u64| BehaviorScenarioAssertion {
        assertion_id: "cooldown-active".to_string(),
        kind: BehaviorScenarioAssertionKind::CooldownActive,
        report_index: Some(report_index),
        behavior_id: Some(behavior_id.to_string()),
        action_id: None,
        event: None,
        flag: None,
        expected_flag: None,
        state: None,
        entity_id: None,
        item: None,
        terminal_state: None,
    };

    let passing = cooldown_template("player-jump", 0);
    passing.validate().expect("cooldown assertion validates");
    assert_eq!(
        passing
            .evaluate(&evidence)
            .expect("cooldown assertion evaluates")
            .status,
        BehaviorScenarioAssertionStatus::Passed,
    );

    // A behavior that did not fire in the targeted report has no active cooldown evidence.
    let failing = cooldown_template("goal-finish", 0);
    assert_eq!(
        failing
            .evaluate(&evidence)
            .expect("cooldown assertion evaluates")
            .status,
        BehaviorScenarioAssertionStatus::Failed,
    );
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

#[test]
fn replay_key_distinguishes_initial_world_states_that_converge() {
    use ouroforge_core::behavior_runtime::{BehaviorExecutionInput, BehaviorWorldState};

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");

    // Same trigger/input, but two different starting worlds that converge to the
    // same post-execution report: one starts with `jumped: false`, the other has
    // no `jumped` flag at all. The mark-jumped action sets it true in both.
    let with_flag = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default()
            .with_flag("grounded", true)
            .with_flag("jumped", false)
            .with_position("player", 2, 5),
    );
    let without_flag = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("jump"),
        BehaviorWorldState::default()
            .with_flag("grounded", true)
            .with_position("player", 2, 5),
    );

    // The two runs produce identical applied actions and identical final world
    // state, so a key derived only from the post-execution report would collide.
    assert_eq!(
        with_flag.world_state, without_flag.world_state,
        "final world states converge"
    );
    assert_eq!(with_flag.applied_actions, without_flag.applied_actions);

    let bundle = artifact.evidence_bundle(vec![with_flag, without_flag]);
    // Different starting worlds must yield different replay keys so a consumer can
    // tell which run (and therefore which input world) it is meant to replay.
    assert_ne!(
        bundle.reports[0].replay_key, bundle.reports[1].replay_key,
        "differing initial world states must not share a replay key"
    );
    // The evidence carries the initial snapshot so the run is actually replayable.
    assert_eq!(
        bundle.reports[0].initial_world_state.flags.get("jumped"),
        Some(&false)
    );
    assert!(!bundle.reports[1]
        .initial_world_state
        .flags
        .contains_key("jumped"));
}

fn behavior_draft_fixture_str(name: &str) -> &'static str {
    match name {
        "valid" => {
            include_str!("../../../examples/behavior-draft-v1/valid/behavior-draft.valid.json")
        }
        "stale" => {
            include_str!("../../../examples/behavior-draft-v1/valid/behavior-draft.stale.json")
        }
        "missing-evidence-blocked" => include_str!(
            "../../../examples/behavior-draft-v1/valid/behavior-draft.missing-evidence-blocked.json"
        ),
        "unsupported-blocked" => include_str!(
            "../../../examples/behavior-draft-v1/valid/behavior-draft.unsupported-blocked.json"
        ),
        "unsafe-target" => include_str!(
            "../../../examples/behavior-draft-v1/invalid/behavior-draft.unsafe-target.json"
        ),
        "malformed-operation" => include_str!(
            "../../../examples/behavior-draft-v1/invalid/behavior-draft.malformed-operation.json"
        ),
        _ => panic!("unknown behavior draft fixture {name}"),
    }
}

fn behavior_draft_fixture_value() -> serde_json::Value {
    serde_json::from_str(behavior_draft_fixture_str("valid")).expect("behavior draft fixture json")
}

fn parse_behavior_draft_str(input: &str) -> Result<BehaviorDraftArtifact, anyhow::Error> {
    BehaviorDraftArtifact::from_json_str(input)
}

fn parse_behavior_draft(value: serde_json::Value) -> Result<BehaviorDraftArtifact, anyhow::Error> {
    BehaviorDraftArtifact::from_json_str(
        &serde_json::to_string_pretty(&value).expect("draft fixture serializes"),
    )
}

#[test]
fn behavior_draft_artifact_accepts_valid_stale_and_blocked_fixtures() {
    use ouroforge_core::behavior_runtime::BehaviorDraftValidationStatus;

    let draft =
        parse_behavior_draft_str(behavior_draft_fixture_str("valid")).expect("draft validates");
    assert_eq!(draft.draft_id, "draft-jump-boost");
    assert_eq!(
        draft.validation_status,
        BehaviorDraftValidationStatus::Drafted
    );
    assert!(draft.untrusted_boundary.contains("does not apply"));

    let stale = parse_behavior_draft_str(behavior_draft_fixture_str("stale"))
        .expect("stale draft validates visibly");
    assert_eq!(
        stale.validation_status,
        BehaviorDraftValidationStatus::Stale
    );

    let blocked = parse_behavior_draft_str(behavior_draft_fixture_str("missing-evidence-blocked"))
        .expect("blocked missing evidence draft validates");
    assert_eq!(
        blocked.validation_status,
        BehaviorDraftValidationStatus::Blocked
    );

    let unsupported = parse_behavior_draft_str(behavior_draft_fixture_str("unsupported-blocked"))
        .expect("blocked unsupported behavior draft validates");
    assert_eq!(
        unsupported.validation_status,
        BehaviorDraftValidationStatus::Blocked
    );
}

#[test]
fn behavior_draft_artifact_rejects_unsafe_duplicate_missing_and_stale_drift() {
    let cases = [
        (
            serde_json::from_str(behavior_draft_fixture_str("unsafe-target"))
                .expect("unsafe target fixture json"),
            "scenePath must stay inside the local project tree",
        ),
        (
            {
                let mut value = behavior_draft_fixture_value();
                value["linkedEvidence"] = serde_json::json!([
                    {"id":"dup","kind":"scenario-result","path":"evidence/a.json","summary":"A"},
                    {"id":"dup","kind":"scenario-result","path":"evidence/b.json","summary":"B"}
                ]);
                value
            },
            "linkedEvidence.id must be unique",
        ),
        (
            {
                let mut value = behavior_draft_fixture_value();
                value["linkedEvidence"] = serde_json::json!([]);
                value
            },
            "linkedEvidence is required",
        ),
        (
            {
                let mut value = behavior_draft_fixture_value();
                value["validationStatus"] = serde_json::json!("stale");
                value["blockedReasons"] = serde_json::json!(["needs review"]);
                value
            },
            "stale status blockedReasons must mention stale target or hash drift",
        ),
    ];

    for (value, expected) in cases {
        let error = parse_behavior_draft(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

#[test]
fn behavior_draft_artifact_blocks_unsupported_behavior_and_malformed_operations() {
    let unsupported_behavior: serde_json::Value = serde_json::from_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.unsupported.json"
    ))
    .expect("unsupported behavior fixture json");
    let mut unsupported = behavior_draft_fixture_value();
    unsupported["proposedBehavior"] = unsupported_behavior.clone();
    let error = parse_behavior_draft(unsupported).expect_err("unsupported must be blocked");
    assert!(format!("{error:?}").contains("unsupported behavior must be blocked"));

    let mut blocked = behavior_draft_fixture_value();
    blocked["proposedBehavior"] = unsupported_behavior;
    blocked["validationStatus"] = serde_json::json!("blocked");
    blocked["blockedReasons"] = serde_json::json!(["unsupported behavior action requires review"]);
    parse_behavior_draft(blocked).expect("unsupported behavior can be visible and blocked");

    let malformed = serde_json::from_str(behavior_draft_fixture_str("malformed-operation"))
        .expect("malformed operation fixture json");
    let error = parse_behavior_draft(malformed).expect_err("malformed operation rejected");
    assert!(format!("{error:?}").contains("action id must not be empty"));
}

#[test]
fn behavior_draft_fixture_files_cover_valid_invalid_stale_missing_and_blocked() {
    for name in [
        "valid",
        "stale",
        "missing-evidence-blocked",
        "unsupported-blocked",
    ] {
        parse_behavior_draft_str(behavior_draft_fixture_str(name))
            .expect("valid behavior draft fixture");
    }

    for (name, expected) in [
        (
            "unsafe-target",
            "scenePath must stay inside the local project tree",
        ),
        ("malformed-operation", "action id must not be empty"),
    ] {
        let error = parse_behavior_draft_str(behavior_draft_fixture_str(name)).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

fn behavior_apply_fixture_str(name: &str) -> &'static str {
    match name {
        "ready" => {
            include_str!("../../../examples/behavior-apply-v1/valid/behavior-apply.ready.json")
        }
        "stale" => {
            include_str!("../../../examples/behavior-apply-v1/valid/behavior-apply.stale.json")
        }
        "unsafe-output" => include_str!(
            "../../../examples/behavior-apply-v1/invalid/behavior-apply.unsafe-output.json"
        ),
        "output-collision" => include_str!(
            "../../../examples/behavior-apply-v1/invalid/behavior-apply.output-collision.json"
        ),
        "unsupported-behavior" => include_str!(
            "../../../examples/behavior-apply-v1/invalid/behavior-apply.unsupported-behavior.json"
        ),
        _ => panic!("unknown behavior apply fixture {name}"),
    }
}

fn parse_behavior_apply_str(
    input: &str,
) -> Result<BehaviorApplyTransactionArtifact, anyhow::Error> {
    BehaviorApplyTransactionArtifact::from_json_str(input)
}

fn behavior_apply_fixture_value() -> serde_json::Value {
    serde_json::from_str(behavior_apply_fixture_str("ready")).expect("behavior apply fixture json")
}

fn parse_behavior_apply(
    value: serde_json::Value,
) -> Result<BehaviorApplyTransactionArtifact, anyhow::Error> {
    BehaviorApplyTransactionArtifact::from_json_str(
        &serde_json::to_string_pretty(&value).expect("apply fixture serializes"),
    )
}

#[test]
fn behavior_apply_transaction_accepts_ready_and_stale_fixture_contracts() {
    use ouroforge_core::behavior_runtime::BehaviorApplyTransactionStatus;

    let ready = parse_behavior_apply_str(behavior_apply_fixture_str("ready"))
        .expect("ready behavior apply transaction validates");
    assert_eq!(ready.transaction_id, "behavior-apply-jump-boost");
    assert_eq!(ready.draft_id, "draft-jump-boost");
    assert_eq!(
        ready.review_decision.review_decision_id,
        "review-behavior-jump-boost-accepted"
    );
    assert_eq!(
        ready.status,
        BehaviorApplyTransactionStatus::ReadyForTrustedApply
    );
    assert!(ready
        .transaction_output_ref
        .starts_with("runs/behavior-applies/"));
    assert_eq!(
        ready.target_hashes.expected_before_hash,
        ready.target.scene_hash
    );
    assert_eq!(
        ready.rollback_metadata.before_hash,
        ready.target_hashes.observed_before_hash
    );
    assert!(ready
        .trusted_boundary
        .to_ascii_lowercase()
        .contains("accepted review"));
    assert!(ready
        .trusted_boundary
        .to_ascii_lowercase()
        .contains("rollback"));
    assert!(ready
        .trusted_boundary
        .to_ascii_lowercase()
        .contains("no arbitrary"));

    let stale = parse_behavior_apply_str(behavior_apply_fixture_str("stale"))
        .expect("stale behavior apply transaction validates visibly");
    assert_eq!(stale.status, BehaviorApplyTransactionStatus::Stale);
    assert!(stale
        .blocked_reasons
        .join(" ")
        .contains("stale target hash"));
}

#[test]
fn behavior_apply_transaction_rejects_unsafe_outputs_and_unsupported_behavior() {
    for (name, expected) in [
        (
            "unsafe-output",
            "transactionOutputRef must stay inside the local project tree",
        ),
        ("output-collision", "not a scene source path"),
        (
            "unsupported-behavior",
            "unsupported behavior must remain blocked before apply",
        ),
    ] {
        let error = parse_behavior_apply_str(behavior_apply_fixture_str(name)).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

#[test]
fn behavior_apply_transaction_rejects_missing_review_blocker_and_guardrail_drift() {
    let cases = [
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["reviewDecision"]["reviewDecisionId"] = serde_json::json!("");
                value
            },
            "reviewDecisionId must not be empty",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["transactionOutputRef"] = serde_json::json!("tmp/behavior-apply.json");
                value
            },
            "transactionOutputRef must be under runs/ or .omx/",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["status"] = serde_json::json!("blocked");
                value["blockedReasons"] = serde_json::json!([]);
                value
            },
            "blocked status requires blockedReasons",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["trustedBoundary"] = serde_json::json!("accepted review only");
                value
            },
            "trustedBoundary must state `rollback`",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["reviewDecision"]["reviewerId"] = serde_json::json!("agent-author");
                value
            },
            "forbids self-approval",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["targetHashes"]["observedBeforeHash"] =
                    serde_json::json!("fnv1a64-canonical-json-v1:1111222233334444");
                value["rollbackMetadata"]["beforeHash"] =
                    serde_json::json!("fnv1a64-canonical-json-v1:1111222233334444");
                value
            },
            "requires fresh target hashes",
        ),
        (
            {
                let mut value = behavior_apply_fixture_value();
                value["rerunCommand"]["command"] = serde_json::json!("bash -c eval");
                value
            },
            "must stay on the local allowlist",
        ),
    ];

    for (value, expected) in cases {
        let error = parse_behavior_apply(value).expect_err(expected);
        assert!(format!("{error:?}").contains(expected), "{error:?}");
    }
}

#[test]
fn gameplay_logic_demo_v1_fixture_validates_project_pack_and_behavior_outcomes() {
    use ouroforge_core::behavior_runtime::{BehaviorExecutionInput, BehaviorWorldState};

    let project_root = std::path::Path::new("../../examples/gameplay-logic-demo-v1");
    let manifest = ProjectManifest::from_path(project_root.join("ouroforge.project.json"))
        .expect("gameplay logic demo project validates");
    assert_eq!(manifest.project.id, "gameplay_logic_demo_v1");
    assert!(manifest.generated.roots.iter().any(|root| root == "runs"));
    assert!(manifest
        .generated
        .roots
        .iter()
        .any(|root| root == "dashboard-data"));

    let seed = Seed::from_path(project_root.join("seeds/gameplay-logic-demo.yaml"))
        .expect("gameplay logic demo seed validates");
    assert_eq!(seed.id, "gameplay-logic-demo.v1");
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("not browser command execution")));

    let pack = ScenarioPack::from_path(
        project_root.join("scenarios/gameplay-logic-demo.scenario-pack.json"),
    )
    .expect("gameplay logic demo scenario pack validates");
    assert_eq!(
        pack.ordered_scenario_ids(),
        vec!["collect-key", "open-door-and-exit", "dash-and-patrol"]
    );

    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/gameplay-logic-demo-v1/behaviors/gameplay-logic-demo.behavior.json"
    ))
    .expect("gameplay logic demo behavior artifact validates");
    let runtime_state = artifact.runtime_state();
    assert_eq!(runtime_state.status, BehaviorRuntimeStatus::Ready);
    assert_eq!(runtime_state.counts.behavior_count, 5);
    assert_eq!(runtime_state.counts.trigger_count, 5);
    assert_eq!(runtime_state.counts.condition_count, 5);
    assert_eq!(runtime_state.counts.action_count, 13);
    assert_eq!(runtime_state.counts.state_machine_count, 2);
    assert_eq!(runtime_state.counts.ability_count, 1);
    assert!(runtime_state.diagnostics.is_empty());

    let key = artifact.execute(
        BehaviorExecutionInput::new("onCollision").with_event("keyPickup"),
        BehaviorWorldState::default().with_flag("keyVisible", true),
    );
    let door = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("doorReached"),
        BehaviorWorldState::default().with_item("key"),
    );
    let dash = artifact.execute(
        BehaviorExecutionInput::new("onInputAction").with_input_action("dash"),
        BehaviorWorldState::default()
            .with_flag("staminaReady", true)
            .with_position("player", 1, 2),
    );
    let patrol = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("patrolTick"),
        BehaviorWorldState::default()
            .with_flag("playerInHazardLane", true)
            .with_health("player", 3),
    );
    let exit = artifact.execute(
        BehaviorExecutionInput::new("onEvent").with_event("exitReached"),
        BehaviorWorldState::default().with_flag("doorOpen", true),
    );

    assert!(key.world_state.inventory.contains("key"));
    assert_eq!(door.world_state.flags.get("doorOpen"), Some(&true));
    assert_eq!(dash.world_state.entity_positions["player"].x, 4);
    assert_eq!(patrol.world_state.entity_health.get("player"), Some(&2));
    assert_eq!(
        exit.world_state.terminal_state,
        Some(ouroforge_core::behavior_runtime::BehaviorTerminalState::Win)
    );

    let evidence = artifact.evidence_bundle(vec![key, door, dash, patrol, exit]);
    assert_eq!(evidence.summary.report_count, 5);
    assert_eq!(evidence.summary.terminal_state_count, 1);
    assert!(evidence
        .trusted_boundary
        .execution_mode
        .contains("structured-data-only"));
    assert!(evidence
        .trusted_boundary
        .disallowed_actions
        .iter()
        .any(|action| action == "command bridge"));

    let suite: BehaviorScenarioAssertionSuite = serde_json::from_str(include_str!(
        "../../../examples/gameplay-logic-demo-v1/scenarios/gameplay-logic-demo.behavior-assertions.json"
    ))
    .expect("behavior assertion suite JSON parses");
    let result = suite
        .evaluate(&evidence)
        .expect("behavior assertion suite evaluates");
    assert_eq!(result.status, BehaviorScenarioAssertionStatus::Passed);
    assert!(result
        .assertions
        .iter()
        .all(|assertion| assertion.status == BehaviorScenarioAssertionStatus::Passed));
}

#[test]
fn behavior_apply_transaction_read_model_preserves_rollback_rerun_and_evidence_refs() {
    let read_model =
        behavior_apply_transaction_read_model_from_json_str(behavior_apply_fixture_str("ready"))
            .expect("ready transaction read model");

    assert_eq!(
        read_model.schema_version,
        "ouroforge.behavior-apply-transaction-read-model.v1"
    );
    assert_eq!(read_model.transaction_id, "behavior-apply-jump-boost");
    assert_eq!(read_model.status, "ready_for_trusted_apply");
    assert!(read_model.trusted_apply_ready);
    assert!(read_model.target_hash_fresh);
    assert!(read_model
        .rollback_ref
        .ends_with("behavior-apply-jump-boost/rollback.json"));
    assert!(read_model
        .rerun_command
        .contains("behavior apply transaction validate"));
    assert_eq!(read_model.evidence_ref_count, 1);
    assert!(read_model
        .evidence_summary
        .iter()
        .any(|summary| summary.starts_with("review:")));
    assert!(read_model
        .evidence_summary
        .iter()
        .any(|summary| summary.starts_with("rollback:")));
    assert!(read_model
        .evidence_summary
        .iter()
        .any(|summary| summary.starts_with("rerun:")));
    assert!(read_model
        .evidence_summary
        .iter()
        .any(|summary| summary.starts_with("scenario-result:")));
    assert!(read_model.boundary.contains("Read-only"));
    assert!(read_model
        .boundary
        .contains("without writing trusted files"));
    assert!(read_model.boundary.contains("executing commands"));
    assert!(read_model.boundary.contains("self-approval"));
}

#[test]
fn behavior_apply_transaction_read_model_keeps_blocked_state_visible() {
    let read_model =
        behavior_apply_transaction_read_model_from_json_str(behavior_apply_fixture_str("stale"))
            .expect("stale transaction read model");

    assert_eq!(read_model.status, "stale");
    assert!(!read_model.trusted_apply_ready);
    assert!(!read_model.target_hash_fresh);
    assert!(read_model
        .blocked_reasons
        .join(" ")
        .contains("stale target hash"));
    assert!(read_model
        .evidence_summary
        .iter()
        .any(|summary| summary.starts_with("transaction-output:")));
}
