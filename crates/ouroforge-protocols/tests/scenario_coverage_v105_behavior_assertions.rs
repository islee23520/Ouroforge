use ouroforge_protocols::behavior_authoring_spec::{
    BehaviorAction, BehaviorActionKind, BehaviorAuthoringDomain, BehaviorAuthoringSpec,
    BehaviorDraftBoundary, BehaviorEvidenceRef, BehaviorParameter, BehaviorParameterType,
    BehaviorParameterValue, BehaviorPreviewContract, BehaviorPreviewMode,
    BehaviorScenarioAssertion, BehaviorScenarioAssertionKind, BehaviorState, BehaviorTransition,
    BehaviorTrigger, BehaviorTriggerKind, BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION,
};
use ouroforge_protocols::behavior_scenario_assertions::{
    generate_behavior_scenario_draft, BehaviorScenarioDraftStatus,
};
use std::collections::BTreeMap;

#[test]
fn scenario_coverage_v105_generated_hazard_assertions_validate_as_scenario_pack() {
    let draft = generate_behavior_scenario_draft(&hazard_spec(BehaviorAuthoringDomain::Hazard));
    draft.validate().unwrap();
    assert_eq!(draft.status, BehaviorScenarioDraftStatus::DraftReady);
    assert!(draft.draft_only);
    let pack_json = draft.scenario_pack_json().unwrap();
    let pack: serde_json::Value = serde_json::from_str(&pack_json).unwrap();
    assert_eq!(pack["schemaVersion"], "scenario-pack-v1");
    assert_eq!(pack["id"], "hazard-route-timing-demo-behavior-draft");
    assert_eq!(
        pack["scenarioGroups"][0]["scenarios"][0]["id"],
        "hazard-route-timing-demo-hazard-pass-fail"
    );
    let assertions = pack["scenarioGroups"][0]["scenarios"][0]["assertions"]
        .as_array()
        .unwrap();
    assert_eq!(assertions.len(), 2);
    assert!(assertions
        .iter()
        .any(|assertion| assertion.get("world_flags").is_some()));
    assert!(assertions
        .iter()
        .any(|assertion| assertion.get("runtime_events").is_some()));
    assert!(draft.assertion_drafts.iter().all(|assertion| {
        assertion.evidence_path == "hazard_failed" || assertion.evidence_path == "events"
    }));
}

#[test]
fn scenario_coverage_v105_unsupported_behavior_assertions_fail_closed() {
    let draft = generate_behavior_scenario_draft(&hazard_spec(BehaviorAuthoringDomain::Npc));
    assert_eq!(draft.status, BehaviorScenarioDraftStatus::Rejected);
    assert!(draft
        .diagnostics
        .iter()
        .any(|diagnostic| { diagnostic.code.as_str() == "unsupported_behavior_domain" }));
}

fn hazard_spec(domain: BehaviorAuthoringDomain) -> BehaviorAuthoringSpec {
    BehaviorAuthoringSpec {
        schema_version: BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION.to_string(),
        behavior_id: "hazard-route-timing-demo".to_string(),
        domain,
        initial_state: "patrolling".to_string(),
        states: vec![
            BehaviorState {
                id: "patrolling".to_string(),
                label: "Patrolling".to_string(),
                tags: vec!["hazard".to_string()],
            },
            BehaviorState {
                id: "contact-failed".to_string(),
                label: "Contact failed".to_string(),
                tags: vec!["hazard".to_string()],
            },
        ],
        transitions: vec![BehaviorTransition {
            id: "contact-fails".to_string(),
            from: "patrolling".to_string(),
            to: "contact-failed".to_string(),
            trigger: BehaviorTrigger {
                kind: BehaviorTriggerKind::OnPlayerContact,
                parameters: BTreeMap::new(),
            },
            actions: vec![BehaviorAction {
                kind: BehaviorActionKind::EmitEvent,
                parameters: BTreeMap::from([(
                    "event".to_string(),
                    BehaviorParameterValue::TextId("hazard.contact.failed".to_string()),
                )]),
            }],
        }],
        parameter_schema: vec![BehaviorParameter {
            name: "contactFrameThreshold".to_string(),
            value_type: BehaviorParameterType::Integer,
            default: Some(BehaviorParameterValue::Integer(60)),
            min: Some(0),
            max: Some(600),
        }],
        preview: BehaviorPreviewContract {
            preview_mode: BehaviorPreviewMode::ScenarioAssertionGeneration,
            deterministic_seed: 42,
            expected_events: vec!["hazard.contact.failed".to_string()],
            evidence_refs: vec![BehaviorEvidenceRef {
                run_id: "run-after".to_string(),
                path: "runs/session-h-2374/behavior-preview-after.json".to_string(),
                digest: "sha256:after".to_string(),
            }],
        },
        scenario_assertions: vec![
            BehaviorScenarioAssertion {
                id: "hazard-fail-flag".to_string(),
                source: "contact-fails".to_string(),
                assertion_kind: BehaviorScenarioAssertionKind::WorldFlag,
                expected_path: "hazard_failed".to_string(),
                expected_value: BehaviorParameterValue::Bool(true),
            },
            BehaviorScenarioAssertion {
                id: "hazard-fail-event".to_string(),
                source: "contact-fails".to_string(),
                assertion_kind: BehaviorScenarioAssertionKind::RuntimeEvent,
                expected_path: "events".to_string(),
                expected_value: BehaviorParameterValue::TextId("hazard.contact.failed".to_string()),
            },
        ],
        draft_boundary: BehaviorDraftBoundary::SafeSourceApplyReviewRequired,
        guardrails: vec![
            "draft-only generated assertions; reviewer promotion required".to_string(),
        ],
    }
}
