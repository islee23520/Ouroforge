use ouroforge_core::behavior_draft::{
    BehaviorDraftArtifact, BehaviorDraftOperationStatus, BehaviorDraftValidationStatus,
};

fn read_fixture(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(path)).expect("fixture reads")
}

#[test]
fn behavior_draft_accepts_valid_untrusted_fixture() {
    let input = read_fixture(
        "examples/agent-generated-behavior-draft-v1/behavior-draft.valid.fixture.json",
    );

    let artifact = BehaviorDraftArtifact::from_json_str(&input).expect("draft validates");

    assert_eq!(artifact.schema_version, "agent-generated-behavior-draft-v1");
    assert_eq!(
        artifact.validation_status,
        BehaviorDraftValidationStatus::Drafted
    );
    assert_eq!(
        artifact.proposed_behaviors[0].status,
        BehaviorDraftOperationStatus::Proposed
    );
    assert!(artifact.author.untrusted);
}

#[test]
fn behavior_draft_accepts_visible_blocked_states() {
    for (path, status) in [
        (
            "examples/agent-generated-behavior-draft-v1/behavior-draft.stale.fixture.json",
            BehaviorDraftValidationStatus::StaleTarget,
        ),
        (
            "examples/agent-generated-behavior-draft-v1/behavior-draft.missing-evidence.fixture.json",
            BehaviorDraftValidationStatus::MissingEvidence,
        ),
        (
            "examples/agent-generated-behavior-draft-v1/behavior-draft.blocked.fixture.json",
            BehaviorDraftValidationStatus::Blocked,
        ),
    ] {
        let input = read_fixture(path);

        let artifact = BehaviorDraftArtifact::from_json_str(&input).expect("draft validates");

        assert_eq!(artifact.validation_status, status);
        assert!(!artifact.blocked_reasons.is_empty());
    }
}

#[test]
fn behavior_draft_rejects_malformed_or_unsafe_fixtures() {
    for (path, expected) in [
        (
            "examples/agent-generated-behavior-draft-v1/invalid/duplicate-operation.fixture.json",
            "duplicate behavior draft proposedBehaviors.operationId",
        ),
        (
            "examples/agent-generated-behavior-draft-v1/invalid/malformed-evidence.fixture.json",
            "linkedEvidence must be JSON evidence under evidence/behavior-drafts",
        ),
        (
            "examples/agent-generated-behavior-draft-v1/invalid/status-drift.fixture.json",
            "drafted behavior draft requires proposed operations",
        ),
        (
            "examples/agent-generated-behavior-draft-v1/invalid/unsafe-target.fixture.json",
            "must not escape the repository",
        ),
        (
            "examples/agent-generated-behavior-draft-v1/invalid/unsupported-without-block.fixture.json",
            "unsupported behavior draft requires blockedReasons",
        ),
        (
            "examples/agent-generated-behavior-draft-v1/invalid/forbidden-runtime-text.fixture.json",
            "is forbidden because #619 does not authorize scripts",
        ),
    ] {
        let input = read_fixture(path);

        let error = BehaviorDraftArtifact::from_json_str(&input).expect_err("fixture fails");

        assert!(
            error.to_string().contains(expected),
            "{} did not contain {}",
            error,
            expected
        );
    }
}
