use ouroforge_observability::{artifact_inventory_paths, required_artifacts, validate_bundle};

#[test]
fn accepts_minimal_valid_bundle_fixture() {
    let report = validate_bundle("fixtures/minimal-valid").expect("fixture should validate");
    assert_eq!(report.run_id, "minimal-valid");
    assert_eq!(report.run_kind, "runtime");
    let inventory = artifact_inventory_paths("fixtures/minimal-valid").unwrap();
    for required in required_artifacts() {
        assert!(
            inventory.contains(*required),
            "missing inventory entry {required}"
        );
    }
}

#[test]
fn rejects_bundle_missing_verdict() {
    let error = validate_bundle("fixtures/missing-verdict").expect_err("missing verdict must fail");
    assert!(
        format!("{error:#}").contains("verdict.md"),
        "error should mention verdict.md, got {error:#}"
    );
}
