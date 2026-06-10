use ouroforge_protocols::proposal_workbench_model::ProposalWorkbenchModel;

#[test]
fn accepted_proposal_fixtures_parse_and_project_to_safe_source_apply_review() {
    for fixture in [
        include_str!("fixtures/proposal-workbench-v1/accepted/runtime-gameplay-proposal.json"),
        include_str!("fixtures/proposal-workbench-v1/accepted/authoring-proposal.json"),
    ] {
        let proposal = ProposalWorkbenchModel::from_json_str(fixture).unwrap();
        let review = proposal.safe_source_apply_review_section().unwrap();
        assert!(!review.evidence_refs.is_empty());
        assert!(!review.target_paths.is_empty());
        assert!(review
            .forbidden_authority
            .contains(&"self_apply".to_string()));
        assert!(review
            .forbidden_authority
            .contains(&"hidden_command".to_string()));
    }
}

#[test]
fn rejected_proposal_fixtures_fail_closed() {
    for (fixture, expected) in [
        (
            include_str!("fixtures/proposal-workbench-v1/rejected/missing-evidence.json"),
            "problemEvidenceRefs",
        ),
        (
            include_str!("fixtures/proposal-workbench-v1/rejected/hidden-command.json"),
            "forbidden proposal authority text",
        ),
        (
            include_str!("fixtures/proposal-workbench-v1/rejected/unbounded-diff.json"),
            "boundedChangeCount",
        ),
    ] {
        let err = ProposalWorkbenchModel::from_json_str(fixture)
            .unwrap_err()
            .to_string();
        assert!(err.contains(expected), "expected {expected:?}, got {err}");
    }
}
