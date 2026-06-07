//! Contract tests for UI/UX Flow, Onboarding and Accessibility v1 (#1660).
//!
//! Mirror of the runtime contract test
//! `examples/game-runtime/uiux-flow.test.cjs`. Validates the shared flow
//! contract fixture and the fail-closed negatives that keep the declared flow
//! deterministic, fully reachable, and accessibility-bearing.

use ouroforge_core::uiux_flow::{UiuxFlowContract, UIUX_FLOW_BOUNDARY, UIUX_FLOW_SCHEMA_VERSION};

fn valid_contract_json() -> &'static str {
    include_str!("../../../examples/game-runtime/uiux-flow-v1.json")
}

fn contract() -> UiuxFlowContract {
    UiuxFlowContract::from_json_str(valid_contract_json()).expect("valid uiux flow contract parses")
}

#[test]
fn valid_contract_loads_and_read_model_summarizes() {
    let flow = contract();
    assert_eq!(flow.schema_version, UIUX_FLOW_SCHEMA_VERSION);
    assert_eq!(flow.boundary, UIUX_FLOW_BOUNDARY);
    assert_eq!(flow.initial_screen, "title");

    let read = flow.read_model();
    assert_eq!(read.screen_count, 4);
    assert_eq!(read.accessibility_option_count, 3);
    assert_eq!(read.transition_count, 5);
    assert!(read.screen_kinds.iter().any(|k| k == "onboarding"));
    assert!(read.screen_kinds.iter().any(|k| k == "settings"));
    assert_eq!(read.boundary, UIUX_FLOW_BOUNDARY);
}

#[test]
fn duplicate_screen_id_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["screens"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "id": "title", "kind": "menu" }));
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("duplicate screen id must fail closed");
    assert!(err.to_string().contains("duplicate uiux screen id"));
}

#[test]
fn non_deterministic_transition_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["transitions"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "from": "title", "action": "start", "to": "hud" }));
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("non-deterministic transition must fail closed");
    assert!(err
        .to_string()
        .contains("non-deterministic uiux transition"));
}

#[test]
fn unreachable_screen_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["screens"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "id": "credits", "kind": "menu" }));
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("unreachable screen must fail closed");
    assert!(err.to_string().contains("unreachable"));
}

#[test]
fn missing_accessibility_options_are_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["accessibilityOptions"] = serde_json::json!([]);
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("missing accessibility options must fail closed");
    assert!(err
        .to_string()
        .contains("at least one accessibility option"));
}

#[test]
fn enum_default_outside_values_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    for option in json["accessibilityOptions"].as_array_mut().unwrap() {
        if option["id"] == "textScale" {
            option["default"] = serde_json::json!("enormous");
        }
    }
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("enum default outside values must fail closed");
    assert!(err
        .to_string()
        .contains("default must be one of its values"));
}

#[test]
fn toggle_default_must_be_boolean() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    for option in json["accessibilityOptions"].as_array_mut().unwrap() {
        if option["id"] == "highContrast" {
            option["default"] = serde_json::json!("true");
        }
    }
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("non-boolean toggle default must fail closed");
    assert!(err.to_string().contains("default must be a boolean"));
}

#[test]
fn initial_screen_must_be_declared() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["initialScreen"] = serde_json::json!("nonexistent");
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("undeclared initial screen must fail closed");
    assert!(err.to_string().contains("not a declared screen"));
}

#[test]
fn wrong_boundary_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["boundary"] = serde_json::json!("browser-can-write");
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("non-canonical boundary must fail closed");
    assert!(err
        .to_string()
        .contains("canonical read-only/proposal-only"));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let mut json: serde_json::Value = serde_json::from_str(valid_contract_json()).unwrap();
    json["schemaVersion"] = serde_json::json!("uiux-flow-v0");
    let err = UiuxFlowContract::from_json_str(&json.to_string())
        .expect_err("wrong schema version must fail closed");
    assert!(err.to_string().contains("schema version"));
}
