use ouroforge_core::gdd_design_brief::{
    GddDesignBriefArtifact, GddDesignBriefStatus, GddTargetGameClass,
    GDD_DESIGN_BRIEF_SCHEMA_VERSION,
};
fn valid_fixture() -> &'static str {
    include_str!("../../../examples/gdd-design-brief-v1/design-brief.valid.fixture.json")
}
#[test]
fn gdd_design_brief_accepts_valid_partial_and_blocked_fixtures() {
    let ready = GddDesignBriefArtifact::from_json_str(valid_fixture()).expect("ready brief parses");
    assert_eq!(ready.schema_version, GDD_DESIGN_BRIEF_SCHEMA_VERSION);
    assert_eq!(ready.status, GddDesignBriefStatus::Ready);
    assert_eq!(
        ready.target_game_class,
        GddTargetGameClass::Small2dPrototype
    );
    assert_eq!(ready.core_loop.steps.len(), 5);
    let partial = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/design-brief.partial.fixture.json"
    ))
    .expect("partial brief parses");
    assert_eq!(partial.status, GddDesignBriefStatus::Partial);
    let blocked = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/design-brief.blocked.fixture.json"
    ))
    .expect("blocked brief parses");
    assert_eq!(blocked.status, GddDesignBriefStatus::Blocked);
    assert!(!blocked.blocked_reasons.is_empty());
}
#[test]
fn gdd_design_brief_rejects_unsafe_refs_and_unknown_fields() {
    let unsafe_ref = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/invalid/design-brief.unsafe-ref.fixture.json"
    ))
    .expect_err("remote asset/style refs are rejected");
    assert!(
        unsafe_ref
            .to_string()
            .contains("forbidden GDD/prototype authority text")
            || unsafe_ref
                .to_string()
                .contains("local fixture/reference roots")
    );
    let mut value: serde_json::Value = serde_json::from_str(valid_fixture()).expect("fixture json");
    value["generationPrompt"] = serde_json::json!("make a full game");
    let unknown = GddDesignBriefArtifact::from_json_str(&value.to_string())
        .expect_err("generation authority fields are rejected");
    assert!(unknown
        .to_string()
        .contains("failed to parse GDD Design Brief JSON"));
}
#[test]
fn gdd_design_brief_rejects_validation_drift_for_ready_briefs() {
    for (fixture, expected) in [
        (
            include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.overbroad-scope.fixture.json"),
            "overbroad or out-of-scope",
        ),
        (
            include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.contradictory.fixture.json"),
            "contradictory requirements",
        ),
        (
            include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.unclear-win-loss.fixture.json"),
            "must be concrete for ready design briefs",
        ),
        (
            include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.unsupported-asset-kind.fixture.json"),
            "is not supported in v1",
        ),
    ] {
        let error = GddDesignBriefArtifact::from_json_str(fixture).expect_err(expected);
        assert!(error.to_string().contains(expected), "{error:?}");
    }
}

#[test]
fn gdd_design_brief_rejects_missing_core_loop_acceptance_and_target_class_drift() {
    let base: serde_json::Value = serde_json::from_str(valid_fixture()).expect("fixture json");
    let cases = [
        (
            {
                let mut value = base.clone();
                value["coreLoop"]["steps"] = serde_json::json!([]);
                value
            },
            "coreLoop.steps must not be empty",
        ),
        (
            {
                let mut value = base.clone();
                value["coreLoop"]["steps"] = serde_json::json!(["move"]);
                value
            },
            "at least two concrete steps",
        ),
        (
            {
                let mut value = base.clone();
                value["targetGameClass"] = serde_json::json!("autonomous-full-game");
                value
            },
            "failed to parse GDD Design Brief JSON",
        ),
        (
            {
                let mut value = base.clone();
                value["acceptanceGoals"] = serde_json::json!([]);
                value
            },
            "acceptanceGoals must not be empty",
        ),
        (
            {
                let mut value = base.clone();
                value["assetStyleRefs"][0]["license"] = serde_json::json!("unknown");
                value
            },
            "license must be explicit",
        ),
    ];

    for (value, expected) in cases {
        let error = GddDesignBriefArtifact::from_json_str(&value.to_string()).expect_err(expected);
        assert!(error.to_string().contains(expected), "{error:?}");
    }
}

#[test]
fn gdd_design_brief_read_model_preserves_display_compatibility() {
    let ready = GddDesignBriefArtifact::from_json_str(valid_fixture()).expect("ready brief parses");
    let read_model = ready.read_model();
    assert_eq!(read_model.schema_version, GDD_DESIGN_BRIEF_SCHEMA_VERSION);
    assert_eq!(read_model.brief_id, "collect-and-exit-brief");
    assert_eq!(read_model.status, "ready");
    assert_eq!(read_model.target_game_class, "small2d-prototype");
    assert_eq!(read_model.core_loop_step_count, 5);
    assert_eq!(read_model.mechanic_count, 2);
    assert_eq!(read_model.asset_style_ref_count, 1);
    assert_eq!(read_model.acceptance_goal_count, 1);
    assert_eq!(read_model.blocked_reason_count, 0);
    assert!(read_model
        .validation_summary
        .iter()
        .any(|item| item.contains("ready brief has concrete core loop")));
    assert!(read_model
        .compatibility_notes
        .iter()
        .any(|item| item.contains("display-only read model")));
    assert!(read_model
        .compatibility_notes
        .iter()
        .any(|item| item.contains("no prototype generation")));

    let json = ready.read_model_json().expect("read model serializes");
    let value: serde_json::Value = serde_json::from_str(&json).expect("read model json parses");
    assert_eq!(value["briefId"], "collect-and-exit-brief");
    assert_eq!(value["coreLoopStepCount"], 5);
    assert_eq!(value["compatibilityNotes"].as_array().unwrap().len(), 3);
}

#[test]
fn gdd_design_brief_read_model_keeps_partial_blocked_and_malformed_states_visible() {
    let partial = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/design-brief.partial.fixture.json"
    ))
    .expect("partial brief parses");
    let partial_model = partial.read_model();
    assert_eq!(partial_model.status, "partial");
    assert_eq!(partial_model.blocked_reason_count, 0);
    assert!(partial_model
        .compatibility_notes
        .iter()
        .any(|item| item.contains("validated summary counts")));

    let blocked = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/design-brief.blocked.fixture.json"
    ))
    .expect("blocked brief parses");
    let blocked_model = blocked.read_model();
    assert_eq!(blocked_model.status, "blocked");
    assert_eq!(blocked_model.blocked_reason_count, 1);
    assert!(blocked_model
        .validation_summary
        .iter()
        .any(|item| item.contains("blocked reason(s) remain visible")));

    let malformed = GddDesignBriefArtifact::from_json_str(include_str!(
        "../../../examples/gdd-design-brief-v1/invalid/design-brief.unclear-win-loss.fixture.json"
    ))
    .expect_err("malformed/unclear ready briefs do not get display read models");
    assert!(malformed
        .to_string()
        .contains("must be concrete for ready design briefs"));
}

#[test]
fn gdd_design_brief_docs_audit_generation_boundary() {
    let doc = include_str!("../../../docs/gdd-design-brief-v1.md");
    assert!(doc.contains("Issue: #645"));
    assert!(doc.contains("input validation, not generation authority"));
    assert!(doc.contains("GDD-derived output remains untrusted"));
    assert!(doc.contains("no autonomous unrestricted game creation"));
    assert!(doc.contains("#1 remains"));
    assert!(doc.contains("#23 remains"));
}
