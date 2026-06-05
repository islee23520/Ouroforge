const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const SCOPE: &str = include_str!("../../../docs/full-studio-editor-v1.md");
const DEMO: &str = include_str!("../../../docs/full-studio-editor-integrated-demo-v1.md");
const COVERAGE: &str = include_str!("../../../docs/scenario-coverage-v17-full-studio-editor.md");

#[test]
fn roadmap_records_full_studio_editor_v1_completion_without_closing_anchors() {
    let roadmap = ROADMAP.replace('\n', " ");
    assert!(roadmap.contains("Full Studio Editor v1 (#757-#776)"));
    assert!(roadmap.contains("complete as a bounded local-first Studio foundation"));
    assert!(roadmap.contains("#1 and #23 remain open governance anchors"));

    for completed in [
        "integrated Studio overview",
        "project context",
        "scene tree",
        "entity/component inspection",
        "visual scene canvas",
        "draft-only authoring",
        "Safe Source Apply handoff previews",
        "asset browser",
        "scenario/playtest evidence",
        "evidence timeline",
        "export/package inspection",
        "plugin/extension descriptor inspection",
        "workspace persistence",
        "command palette",
        "accessibility/performance/diagnostics coverage",
        "fixture-scoped integrated demo",
        "Scenario Coverage v17 regression coverage",
    ] {
        assert!(
            roadmap.contains(completed),
            "missing completed capability: {completed}"
        );
    }

    for anchor in [
        "docs/full-studio-editor-v1.md",
        "docs/full-studio-editor-integrated-demo-v1.md",
        "docs/scenario-coverage-v17-full-studio-editor.md",
    ] {
        assert!(
            ROADMAP.contains(anchor),
            "missing current roadmap anchor: {anchor}"
        );
    }
}

#[test]
fn roadmap_separates_remaining_editor_gaps_from_completed_foundation() {
    let combined = format!("{ROADMAP}\n{SCOPE}\n{DEMO}\n{COVERAGE}").to_ascii_lowercase();
    for remaining in [
        "full godot parity",
        "native desktop editor",
        "advanced visual scripting",
        "full asset import pipeline",
        "executable editor plugins",
        "timeline/animation editor",
        "tilemap/terrain editor parity",
        "production collaboration features",
        "hosted/cloud",
        "marketplace",
        "native export/release readiness",
        "godot-plus demonstration game",
    ] {
        assert!(
            combined.contains(remaining),
            "missing remaining gap: {remaining}"
        );
    }
}

#[test]
fn full_studio_governance_keeps_forbidden_maturity_claims_blocked() {
    let combined = format!("{ROADMAP}\n{SCOPE}\n{DEMO}\n{COVERAGE}").to_ascii_lowercase();
    for boundary in [
        "does not directly write trusted source files",
        "execute shell commands",
        "publish",
        "deploy",
        "sign",
        "upload",
        "install or execute plugins",
        "auto-apply",
        "auto-merge",
        "self-approve",
        "bypass review",
        "mutate ci/workflows",
        "validated preview",
        "sandbox evidence",
        "accepted independent review",
        "stale-target checks",
        "rollback metadata",
        "allowlisted verification",
        "post-apply comparison",
        "audit ledger",
        "emergency hold",
    ] {
        assert!(combined.contains(boundary), "missing boundary: {boundary}");
    }

    for forbidden in [
        "production-ready editor is available",
        "current godot replacement is implemented",
        "full godot editor parity is implemented",
        "secure sandbox is guaranteed",
        "native export ready",
        "plugin runtime enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "browser trusted writes enabled",
        "command bridge enabled",
        "production collaboration enabled",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}
