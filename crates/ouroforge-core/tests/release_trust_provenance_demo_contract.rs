use ouroforge_core::release_auto_apply::{decide_release_auto_apply, ReleaseAutoApplyRequest};
use ouroforge_core::release_compliance_gate::{
    evaluate_release_compliance, ComplianceVerdictStatus, ReleaseComplianceGateInput,
};
use ouroforge_core::release_provenance_bundle::{ReleaseProvenanceBundle, ReleaseProvenanceStatus};
use ouroforge_core::trust_gradient_audit::AutoApplyAuditLog;
use ouroforge_core::trust_gradient_auto_apply::AutoApplyOutcome;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_demo(name: &str) -> String {
    let path = repo_root().join(format!("examples/release-trust-provenance-v1/demo/{name}"));
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"))
}

#[test]
fn release_trust_provenance_demo_smoke_asserts_rollback_compliance_block_and_complete_bundle() {
    let auto_apply = ReleaseAutoApplyRequest::from_json_str(&read_demo("eligible-auto-apply.json"))
        .expect("eligible auto-apply fixture parses");
    let auto_apply_decision =
        decide_release_auto_apply(&auto_apply, &AutoApplyAuditLog::new()).expect("decision");
    assert_eq!(auto_apply_decision.outcome, AutoApplyOutcome::AutoApplied);
    let rollback = auto_apply_decision
        .rollback_command
        .as_deref()
        .expect("eligible fixture carries rollback command");
    assert!(rollback.starts_with("ouroforge rollback --transaction txn-release-trust-demo-001"));
    assert!(rollback.contains(
        " --reverse examples/release-trust-provenance-v1/demo/reverse/txn-release-trust-demo-001.json"
    ));
    assert!(auto_apply_decision
        .reasons
        .iter()
        .any(|reason| reason.contains("human release gate remains pending")));

    let blocked = ReleaseComplianceGateInput::from_json_str(&read_demo(
        "compliance.blocked-missing-license.json",
    ))
    .expect("blocked compliance fixture parses");
    let blocked_verdict = evaluate_release_compliance(&blocked).expect("blocked verdict");
    assert_eq!(blocked_verdict.status, ComplianceVerdictStatus::Blocked);
    assert!(blocked_verdict
        .reasons
        .iter()
        .any(|reason| reason.contains("missing license")));
    assert!(blocked_verdict.boundary.contains("no release authority"));

    let bundle = ReleaseProvenanceBundle::from_json_str(&read_demo("release-bundle.complete.json"))
        .expect("complete release bundle parses");
    let bundle_eval = bundle.evaluate_with_root(&repo_root());
    assert_eq!(
        bundle_eval.computed_status,
        ReleaseProvenanceStatus::Complete
    );
    assert!(bundle_eval.status_consistent, "{bundle_eval:#?}");
    assert!(bundle_eval.replayable, "{bundle_eval:#?}");
    assert!(bundle_eval.issues.is_empty(), "{bundle_eval:#?}");
    for required in [
        "intent",
        "content",
        "assets",
        "qa",
        "per-change-provenance",
        "compliance",
        "release-candidate",
    ] {
        assert_eq!(
            bundle_eval.link_states.get(required).map(String::as_str),
            Some("present"),
            "{bundle_eval:#?}"
        );
    }
    assert_eq!(
        bundle_eval
            .per_change_bundle_states
            .get("change-bundle-release-trust-demo")
            .map(String::as_str),
        Some("complete"),
        "{bundle_eval:#?}"
    );
}

#[test]
fn release_trust_provenance_demo_docs_preserve_boundaries_and_governance() {
    let docs =
        std::fs::read_to_string(repo_root().join("docs/release-trust-provenance-v1-demo.md"))
            .expect("demo docs exist");
    for required in [
        "composes the existing release auto-apply",
        "release compliance gate",
        "per-release provenance bundle",
        "browser and Studio surfaces remain read-only",
        "no network access",
        "fixture-scoped",
        "#1 and #23 remain open",
    ] {
        assert!(docs.contains(required), "docs missing {required}");
    }
    for forbidden_boundary in [
        "auto-merge",
        "self-approve",
        "production-ready",
        "quality/fun",
        "Godot replacement/parity",
        "autonomous shipping",
    ] {
        assert!(
            docs.contains(forbidden_boundary),
            "docs must explicitly forbid {forbidden_boundary}"
        );
    }

    let auto_apply = ReleaseAutoApplyRequest::from_json_str(&read_demo("eligible-auto-apply.json"))
        .expect("eligible fixture parses");
    assert!(auto_apply.boundary.contains("proposal-only"));
    assert!(auto_apply.boundary.contains("read-only"));
    assert!(auto_apply
        .boundary
        .contains("human release go/no-go remains pending"));

    let bundle = ReleaseProvenanceBundle::from_json_str(&read_demo("release-bundle.complete.json"))
        .expect("bundle fixture parses");
    assert!(bundle.generated_state.generated);
    assert!(bundle.generated_state.tracked);
    assert!(bundle.generated_state.fixture_scoped);
    assert!(bundle.boundary.contains("Milestone 25"));
    assert!(bundle.boundary.contains("no parallel provenance engine"));
}
