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
    assert!(unsafe_ref.to_string().contains("https://"));
    let mut value: serde_json::Value = serde_json::from_str(valid_fixture()).expect("fixture json");
    value["generationPrompt"] = serde_json::json!("make a full game");
    let unknown = GddDesignBriefArtifact::from_json_str(&value.to_string())
        .expect_err("generation authority fields are rejected");
    assert!(unknown
        .to_string()
        .contains("failed to parse GDD Design Brief JSON"));
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
