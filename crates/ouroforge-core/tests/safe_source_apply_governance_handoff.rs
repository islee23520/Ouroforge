const HANDOFF: &str = include_str!("../../../docs/safe-source-apply-governance-handoff.md");
const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const README: &str = include_str!("../../../README.md");
const SCOPE: &str = include_str!("../../../docs/safe-source-mutation-apply-v1.md");
const DOCS_INDEX: &str = include_str!("../../../docs/README.md");

#[test]
fn safe_source_apply_governance_marks_milestone_complete_without_closing_anchors() {
    for text in [HANDOFF, ROADMAP, SCOPE] {
        assert!(text.contains("Safe Source Mutation Apply v1"));
        assert!(text.contains("complete") || text.contains("Complete"));
    }
    assert!(README.contains("Safe Source Mutation Apply"));
    for issue in 699..=716 {
        assert!(
            HANDOFF.contains(&format!("#{issue}")) || ROADMAP.contains(&format!("#{issue}")),
            "missing issue #{issue} from governance evidence"
        );
    }
    assert!(HANDOFF.contains("#1 remains open"));
    assert!(HANDOFF.contains("#23 remains open"));
    assert!(DOCS_INDEX.contains("safe-source-apply-governance-handoff.md"));
}

#[test]
fn safe_source_apply_governance_preserves_required_gate_separation() {
    let combined = format!("{HANDOFF}\n{ROADMAP}\n{README}\n{SCOPE}").to_ascii_lowercase();
    for required in [
        "validated preview",
        "file-class",
        "diff integrity",
        "sandbox dry-run",
        "independent review",
        "stale-target",
        "rollback",
        "allowlisted verification",
        "post-apply",
        "audit ledger",
        "evidence bundle",
        "emergency hold",
        "read-only studio",
    ] {
        assert!(
            combined.contains(required),
            "missing required gate: {required}"
        );
    }
}

#[test]
fn safe_source_apply_governance_keeps_forbidden_claims_blocked() {
    let combined = format!("{HANDOFF}\n{ROADMAP}\n{README}\n{SCOPE}").to_ascii_lowercase();
    for boundary in [
        "unrestricted source mutation",
        "forbidden file-class",
        "dependency/ci/build-script mutation",
        "browser trusted writes",
        "command bridges",
        "auto-apply",
        "auto-merge",
        "self-approval",
        "reviewer bypass",
        "autonomous source repair",
        "secure-sandbox",
        "production-ready mutation",
        "current godot replacement",
    ] {
        assert!(combined.contains(boundary), "missing boundary: {boundary}");
    }

    for forbidden in [
        "unrestricted source mutation enabled",
        "forbidden file-class mutation enabled",
        "dependency/ci/build-script mutation enabled",
        "browser trusted writes enabled",
        "command bridges enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "autonomous source repair enabled",
        "secure sandbox guarantee enabled",
        "production-ready mutation enabled",
        "current godot replacement is implemented",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}
