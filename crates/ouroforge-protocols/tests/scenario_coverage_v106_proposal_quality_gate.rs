use ouroforge_protocols::proposal_quality_gate::{
    evaluate_proposal_quality_gate_json, proposal_quality_gate_rules, ProposalQualityGateStatus,
};

#[test]
fn scenario_coverage_v106_good_bounded_proposal_passes_rule_based_gate() {
    let report = evaluate_proposal_quality_gate_json(include_str!(
        "fixtures/proposal-quality-gate-v1/accepted/bounded.json"
    ))
    .unwrap();
    report.validate().unwrap();
    assert_eq!(report.status, ProposalQualityGateStatus::Passed);
    assert!(report.findings.is_empty());
    assert_eq!(
        report.rule_catalog.len(),
        proposal_quality_gate_rules().len()
    );
    assert!(!report.llm_sole_gate);
}

#[test]
fn scenario_coverage_v106_every_rule_has_negative_fixture() {
    for (fixture, rule_id) in [
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/missing-evidence.json"),
            "evidence-required",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/unsupported-file-class.json"),
            "supported-file-class",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/broad-scope.json"),
            "bounded-scope",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/missing-rollback.json"),
            "rollback-required",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/missing-expected-impact.json"),
            "expected-impact-required",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/self-approval.json"),
            "no-self-approval",
        ),
        (
            include_str!("fixtures/proposal-quality-gate-v1/rejected/hidden-authority.json"),
            "no-hidden-authority",
        ),
    ] {
        let report = evaluate_proposal_quality_gate_json(fixture).unwrap();
        assert_eq!(
            report.status,
            ProposalQualityGateStatus::Failed,
            "{rule_id} should fail"
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.rule_id == rule_id),
            "expected {rule_id}, got {:?}",
            report.findings
        );
        assert!(!report.llm_sole_gate);
    }
}
