use ouroforge_core::{
    classify_path, SourceApplyHighRiskBlocker, SourceApplyHighRiskClass, SourceApplyHighRiskStatus,
    SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION,
};

fn blocker(targets: &[&str]) -> SourceApplyHighRiskBlocker {
    SourceApplyHighRiskBlocker {
        schema_version: SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-709-1".to_string(),
        candidate_targets: targets.iter().map(|t| t.to_string()).collect(),
        guardrails: vec![
            "high-risk file classes are blocked unless a separate governance issue authorizes them"
                .to_string(),
        ],
    }
}

#[test]
fn allowed_source_like_targets_pass() {
    let blocker = blocker(&[
        "crates/ouroforge-core/src/source_apply_review_enforcement.rs",
        "examples/source-apply-v1/widget.js",
        "docs/source-apply-note.md",
    ]);
    blocker.validate().expect("valid");
    let evaluation = blocker.evaluate();
    assert_eq!(evaluation.status, SourceApplyHighRiskStatus::Allowed);
    assert!(evaluation.blocked_targets.is_empty());
    assert!(blocker.is_allowed());
}

#[test]
fn each_high_risk_class_is_blocked() {
    let cases = [
        ("Cargo.toml", SourceApplyHighRiskClass::DependencyManifest),
        ("Cargo.lock", SourceApplyHighRiskClass::Lockfile),
        (
            ".github/workflows/ci.yml",
            SourceApplyHighRiskClass::CiWorkflow,
        ),
        (
            "crates/ouroforge-core/build.rs",
            SourceApplyHighRiskClass::BuildScript,
        ),
        (
            "scripts/run-tests.sh",
            SourceApplyHighRiskClass::ShellScript,
        ),
        (".env", SourceApplyHighRiskClass::HiddenToolRoot),
        (
            "deploy/release.yaml",
            SourceApplyHighRiskClass::ReleaseOrPublish,
        ),
        (
            "target/debug/output.json",
            SourceApplyHighRiskClass::GeneratedOrLocalState,
        ),
    ];
    for (path, _expected) in cases {
        let (class, _reason) = classify_path(path)
            .unwrap_or_else(|| panic!("expected `{path}` to be classified high-risk"));
        let _ = class;
        let blocker = blocker(&[path]);
        let evaluation = blocker.evaluate();
        assert_eq!(
            evaluation.status,
            SourceApplyHighRiskStatus::Blocked,
            "`{path}` must be blocked"
        );
        assert!(!evaluation.blocked_targets.is_empty());
    }
}

#[test]
fn credential_files_are_blocked() {
    let (class, _) = classify_path("config/aws-credentials.json").expect("classified");
    assert_eq!(class, SourceApplyHighRiskClass::CredentialOrNetwork);
}

#[test]
fn unrecognized_extension_is_blocked_fail_closed() {
    let (class, _) = classify_path("examples/source-apply-v1/data.bin").expect("classified");
    assert_eq!(class, SourceApplyHighRiskClass::UnsupportedClass);
}

#[test]
fn path_traversal_is_blocked() {
    let (class, _) = classify_path("../../etc/passwd").expect("classified");
    assert_eq!(class, SourceApplyHighRiskClass::UnsupportedClass);
}

#[test]
fn allowed_source_extensions_are_not_high_risk() {
    for path in [
        "src/lib.rs",
        "examples/x/widget.ts",
        "examples/x/widget.tsx",
        "shaders/frag.wgsl",
    ] {
        assert!(classify_path(path).is_none(), "`{path}` should be allowed");
    }
}

#[test]
fn mixed_targets_block_the_whole_transaction() {
    let blocker = blocker(&[
        "src/lib.rs",
        "Cargo.toml", // one high-risk target blocks the apply
    ]);
    let evaluation = blocker.evaluate();
    assert_eq!(evaluation.status, SourceApplyHighRiskStatus::Blocked);
    assert_eq!(evaluation.allowed_targets.len(), 1);
    assert_eq!(evaluation.blocked_targets.len(), 1);
    assert!(evaluation.governance_note.contains("governance"));
}

#[test]
fn empty_candidates_fail_validation() {
    let mut blocker = blocker(&["src/lib.rs"]);
    blocker.candidate_targets.clear();
    assert!(blocker.validate().is_err());
}
