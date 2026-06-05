const REPRO_DOC: &str = include_str!("../../../docs/godot-plus-demo-reproducibility-v1.md");
const DOCS_README: &str = include_str!("../../../docs/README.md");

#[test]
fn godot_plus_demo_reproducibility_doc_covers_required_workflows() {
    for term in [
        "How to run locally",
        "How to inspect in Studio",
        "How to run scenarios and QA",
        "How to export a local package",
        "How to view evidence and journal",
        "Known limitations",
        "Generated-state policy",
        "gh issue view 795",
        "gh issue view 1",
        "gh issue view 23",
    ] {
        assert!(REPRO_DOC.contains(term), "doc missing {term}");
    }
}

#[test]
fn godot_plus_demo_reproducibility_doc_preserves_boundaries() {
    let lower_doc = REPRO_DOC.to_ascii_lowercase();
    for term in [
        "scoped evidence-native agentic workflow",
        "small local 2d vertical slice",
        "read-only",
        "draft-only",
        "review-gated",
        "no direct studio trusted source writes",
        "no browser command bridge",
        "no auto-apply",
        "no executable plugin runtime",
        "no marketplace",
        "no publish",
        "no deploy",
        "no commercial release",
        "no production-ready",
        "no godot replacement",
        "#1 and #23 remain open",
    ] {
        assert!(lower_doc.contains(term), "doc missing guardrail {term}");
    }
}

#[test]
fn docs_readme_links_godot_plus_demo_reproducibility_doc() {
    assert!(DOCS_README.contains("godot-plus-demo-reproducibility-v1.md"));
}
