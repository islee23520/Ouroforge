const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const README: &str = include_str!("../../../README.md");
const SCOPE: &str = include_str!("../../../docs/plugin-extension-system-v1.md");
const DEMO: &str = include_str!("../../../docs/plugin-extension-system-demo-v1.md");

#[test]
fn plugin_extension_governance_marks_milestone_complete_without_closing_anchors() {
    for text in [ROADMAP, SCOPE] {
        assert!(text.contains("Plugin / Extension System v1"));
    }
    assert!(README.contains("Plugin / Extension System v1"));
    assert!(
        ROADMAP.contains("Plugin / Extension System v1 governance refresh"),
        "roadmap is missing the dedicated plugin governance refresh section"
    );
    assert!(
        ROADMAP.contains("Plugin / Extension System v1 (#738-#754) is now complete"),
        "roadmap is missing the plugin completion prose"
    );
    for issue in 738..=754 {
        assert!(
            ROADMAP.contains(&format!("#{issue}")),
            "missing issue #{issue} from plugin governance evidence"
        );
    }
    // Governance anchors must stay open: the refresh records them as open and
    // never affirmatively claims they are closed.
    assert!(ROADMAP.contains("#1 and #23 remain open"));
    let roadmap_lower = ROADMAP.to_ascii_lowercase();
    for affirmative_closure in [
        "#1 is closed",
        "#23 is closed",
        "#1 and #23 are closed",
        "closes #1 and #23",
        "closed #1 and #23",
    ] {
        assert!(
            !roadmap_lower.contains(affirmative_closure),
            "roadmap affirmatively claims anchors closed: {affirmative_closure}"
        );
    }
}

#[test]
fn plugin_extension_governance_records_completed_declarative_capabilities() {
    let combined = format!("{ROADMAP}\n{README}\n{SCOPE}\n{DEMO}").to_ascii_lowercase();
    for required in [
        "manifest",
        "registry",
        "discovery",
        "extension point catalog",
        "capability",
        "compatibility",
        "descriptor evidence",
        "studio plugin browser",
        "fixture plugin pack",
        "threat-model gate",
        "load-order",
        "conflict detection",
        "cli inspection",
        "scenario coverage v16",
        "deterministic demo",
    ] {
        assert!(
            combined.contains(required),
            "missing completed declarative capability: {required}"
        );
    }
}

#[test]
fn plugin_extension_governance_keeps_forbidden_claims_blocked() {
    let combined = format!("{ROADMAP}\n{README}\n{SCOPE}\n{DEMO}").to_ascii_lowercase();
    for boundary in [
        "executable plugins",
        "arbitrary javascript",
        "native/dynamic library loading",
        "marketplace",
        "network plugin install/update",
        "dependency installation",
        "command execution",
        "source mutation",
        "publish/deploy",
        "ci/workflow mutation",
        "secure plugin sandbox",
        "godot-equivalent extension parity",
        "production-ready plugin ecosystem",
        "current godot replacement",
    ] {
        assert!(
            combined.contains(boundary),
            "missing blocked boundary: {boundary}"
        );
    }

    for forbidden in [
        "executable plugins enabled",
        "arbitrary javascript enabled",
        "native extensions enabled",
        "marketplace enabled",
        "network install enabled",
        "dependency installation enabled",
        "command execution enabled",
        "secure plugin sandbox guaranteed",
        "godot-equivalent parity achieved",
        "production-ready plugin ecosystem shipped",
        "current godot replacement is implemented",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden affirmative overclaim present: {forbidden}"
        );
    }
}
