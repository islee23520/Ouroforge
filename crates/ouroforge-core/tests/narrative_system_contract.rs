//! Contract tests for Narrative/Dialogue/Event System v1 (#1661).
//!
//! Covers the three required behaviors: event triggering, dialogue state, and
//! save/restore parity, plus the fail-closed negatives that keep the
//! dialogue/event system deterministic and trusted-only.

use ouroforge_core::narrative_system::{
    NarrativeDefinition, NarrativeState, NARRATIVE_SYSTEM_BOUNDARY, NARRATIVE_SYSTEM_SCHEMA_VERSION,
};

fn valid_definition_json() -> &'static str {
    include_str!("../../../examples/narrative-system-v1/valid/narrative.definition.json")
}

fn definition() -> NarrativeDefinition {
    NarrativeDefinition::from_json_str(valid_definition_json())
        .expect("valid narrative definition parses")
}

#[test]
fn valid_definition_loads_and_initial_state_is_clean() {
    let def = definition();
    assert_eq!(def.story_id, "demo-story");
    let state = def.initial_state();
    assert_eq!(state.schema_version, NARRATIVE_SYSTEM_SCHEMA_VERSION);
    assert_eq!(state.boundary, NARRATIVE_SYSTEM_BOUNDARY);
    assert_eq!(state.current_node.as_deref(), Some("intro"));
    // No flags set at the start, so no events have fired.
    assert!(state.flags.values().all(|v| !v));
    assert!(state.fired_events.is_empty());
}

#[test]
fn dialogue_state_advances_through_branches_and_ends() {
    let def = definition();
    let state = def.initial_state();

    // Choosing "accept" routes intro -> accepted.
    let s1 = def.advance(&state, Some("accept")).expect("advance accept");
    assert_eq!(s1.current_node.as_deref(), Some("accepted"));
    assert_eq!(s1.visited_nodes, vec!["intro"]);

    // accepted is linear -> farewell.
    let s2 = def.advance(&s1, None).expect("advance accepted");
    assert_eq!(s2.current_node.as_deref(), Some("farewell"));

    // farewell is terminal -> dialogue ends.
    let s3 = def.advance(&s2, None).expect("advance farewell");
    assert!(s3.is_ended());
    assert_eq!(s3.visited_nodes, vec!["intro", "accepted", "farewell"]);

    // Advancing an ended dialogue fails closed.
    let err = def
        .advance(&s3, None)
        .expect_err("ended dialogue must fail closed");
    assert!(err.to_string().contains("already ended"));
}

#[test]
fn events_trigger_deterministically_from_flag_conditions() {
    let def = definition();
    let state = def.initial_state();

    // Entering intro sets metKing, which fires logAudience -> audienceLogged.
    let s1 = def.advance(&state, Some("accept")).expect("advance accept");
    assert_eq!(s1.flags.get("metKing"), Some(&true));
    assert!(s1.fired_events.contains("logAudience"));
    assert_eq!(s1.flags.get("audienceLogged"), Some(&true));
    assert!(!s1.fired_events.contains("blessQuest"));

    // Entering accepted sets acceptedQuest, which fires blessQuest -> questBlessed.
    let s2 = def.advance(&s1, None).expect("advance accepted");
    assert!(s2.fired_events.contains("blessQuest"));
    assert_eq!(s2.flags.get("questBlessed"), Some(&true));

    // Replaying the same choices reproduces the same state.
    let replay = def
        .advance(
            &def.advance(&def.initial_state(), Some("accept")).unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(s2, replay);
}

#[test]
fn declining_routes_to_farewell_without_quest_events() {
    let def = definition();
    let state = def.initial_state();
    let s1 = def
        .advance(&state, Some("decline"))
        .expect("advance decline");
    assert_eq!(s1.current_node.as_deref(), Some("farewell"));
    // metKing still set (intro processed), but the quest branch never runs.
    assert!(s1.fired_events.contains("logAudience"));
    assert!(!s1.fired_events.contains("blessQuest"));
    assert_eq!(s1.flags.get("acceptedQuest"), Some(&false));
}

#[test]
fn save_restore_round_trips_unchanged() {
    let def = definition();
    let state = def
        .advance(
            &def.advance(&def.initial_state(), Some("accept")).unwrap(),
            None,
        )
        .unwrap();

    let json = state.to_json().expect("serialize");
    let restored = NarrativeState::from_json_str(&json).expect("deserialize");
    assert_eq!(state, restored);
    def.validate_state(&restored)
        .expect("restored state is consistent");

    // Advancing after restore continues deterministically to the end.
    let ended = def.advance(&restored, None).expect("advance after restore");
    assert!(ended.is_ended());
}

#[test]
fn missing_choice_for_branching_node_is_rejected() {
    let def = definition();
    let err = def
        .advance(&def.initial_state(), None)
        .expect_err("branching node without a choice must fail closed");
    assert!(err.to_string().contains("requires a choice"));
}

#[test]
fn invalid_choice_is_rejected() {
    let def = definition();
    let err = def
        .advance(&def.initial_state(), Some("nope"))
        .expect_err("invalid choice must fail closed");
    assert!(err.to_string().contains("no choice"));
}

#[test]
fn ambiguous_node_with_next_and_choices_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_definition_json()).unwrap();
    for node in json["nodes"].as_array_mut().unwrap() {
        if node["id"] == "accepted" {
            node["choices"] = serde_json::json!([{ "action": "x", "to": "farewell" }]);
        }
    }
    let err = NarrativeDefinition::from_json_str(&json.to_string())
        .expect_err("ambiguous node must fail closed");
    assert!(err.to_string().contains("ambiguous"));
}

#[test]
fn dangling_node_reference_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_definition_json()).unwrap();
    for node in json["nodes"].as_array_mut().unwrap() {
        if node["id"] == "accepted" {
            node["next"] = serde_json::json!("ghost");
        }
    }
    let err = NarrativeDefinition::from_json_str(&json.to_string())
        .expect_err("dangling next must fail closed");
    assert!(err.to_string().contains("dangling next reference"));
}

#[test]
fn undeclared_flag_in_node_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_definition_json()).unwrap();
    for node in json["nodes"].as_array_mut().unwrap() {
        if node["id"] == "farewell" {
            node["setFlags"] = serde_json::json!(["notDeclared"]);
        }
    }
    let err = NarrativeDefinition::from_json_str(&json.to_string())
        .expect_err("undeclared flag must fail closed");
    assert!(err.to_string().contains("undeclared flag"));
}

#[test]
fn wrong_boundary_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_definition_json()).unwrap();
    json["boundary"] = serde_json::json!("browser-can-write");
    let err = NarrativeDefinition::from_json_str(&json.to_string())
        .expect_err("non-canonical boundary must fail closed");
    assert!(err
        .to_string()
        .contains("canonical read-only/proposal-only"));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_definition_json()).unwrap();
    json["schemaVersion"] = serde_json::json!("narrative-system-v0");
    let err = NarrativeDefinition::from_json_str(&json.to_string())
        .expect_err("wrong schema version must fail closed");
    assert!(err.to_string().contains("schema version"));
}

#[test]
fn state_from_foreign_story_is_rejected() {
    let def = definition();
    let mut state = def.initial_state();
    state.story_id = "other-story".to_string();
    let err = def
        .validate_state(&state)
        .expect_err("foreign story state must fail closed");
    assert!(err.to_string().contains("does not match"));
}

#[test]
fn fired_event_without_its_effect_flag_is_rejected() {
    // A restored state that marks an event fired while its effect flag is still
    // false would let evaluate_events skip the event forever; reject it.
    let def = definition();
    let mut state = def.initial_state();
    state.fired_events.insert("blessQuest".to_string());
    // questBlessed is still false in the initial state.
    let err = def
        .validate_state(&state)
        .expect_err("inconsistent fired-event state must fail closed");
    assert!(err.to_string().contains("effect flag"));
}

#[test]
fn read_model_exposes_read_only_summary() {
    let def = definition();
    let state = def.advance(&def.initial_state(), Some("accept")).unwrap();
    let read = def.read_model(&state);
    assert_eq!(read.story_id, "demo-story");
    assert_eq!(read.current_node.as_deref(), Some("accepted"));
    assert!(!read.ended);
    assert_eq!(read.fired_event_count, state.fired_events.len());
    assert_eq!(read.boundary, NARRATIVE_SYSTEM_BOUNDARY);
}
