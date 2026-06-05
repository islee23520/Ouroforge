use ouroforge_core::behavior_evidence::render_behavior_evidence_journal_section;
use ouroforge_core::{
    add_evidence_artifact, create_run, update_journal, EvidenceArtifact, EvidenceIndex,
};

fn evidence_index(path: &str) -> EvidenceIndex {
    EvidenceIndex {
        artifacts: vec![EvidenceArtifact {
            id: "behavior-evidence-bundle".to_string(),
            kind: "application/json".to_string(),
            path: path.to_string(),
            metadata: serde_json::json!({ "artifact": "behavior_evidence_bundle" }),
            added_at_unix_ms: 1,
        }],
    }
}

#[test]
fn behavior_evidence_journal_renders_lifecycle_failures_and_boundary() {
    let root = unique_temp_dir("behavior-evidence-journal");
    let evidence_path = "evidence/behavior/behavior-evidence-bundle.json";
    std::fs::create_dir_all(root.join("evidence/behavior")).expect("evidence dir");
    std::fs::write(
        root.join(evidence_path),
        include_str!(
            "../../../examples/behavior-evidence-bundle-v1/behavior-evidence-bundle.complete.json"
        ),
    )
    .expect("bundle fixture written");

    let journal = render_behavior_evidence_journal_section(&root, &evidence_index(evidence_path));

    assert!(journal.contains("## Behavior Evidence Lifecycle"));
    assert!(journal.contains("Lifecycle refs: definitions `1`"));
    assert!(journal.contains("Observed failure `jump_boost_cooldown_regression`"));
    assert!(journal.contains("Next-step hypothesis `hypothesis-rerun-after-rollback`"));
    assert!(journal.contains("no arbitrary script execution"));
    assert!(journal.contains(evidence_path));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn behavior_evidence_journal_keeps_malformed_bundle_visible() {
    let root = unique_temp_dir("behavior-evidence-journal-malformed");
    let evidence_path = "evidence/behavior/behavior-evidence-bundle.json";
    std::fs::create_dir_all(root.join("evidence/behavior")).expect("evidence dir");
    std::fs::write(root.join(evidence_path), "{not json").expect("malformed bundle written");

    let journal = render_behavior_evidence_journal_section(&root, &evidence_index(evidence_path));

    assert!(journal.contains("## Behavior Evidence Lifecycle"));
    assert!(journal.contains("Malformed behavior evidence bundle"));
    assert!(journal.contains(evidence_path));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn behavior_evidence_journal_rejects_unsafe_bundle_path_traversal() {
    let root = unique_temp_dir("behavior-evidence-journal-traversal");
    // The evidence subtree must exist so that an unfixed renderer could actually
    // traverse `..` out of it on disk.
    std::fs::create_dir_all(root.join("evidence/behavior")).expect("evidence dir");
    // Plant a *valid* bundle outside the evidence tree (but still inside run_dir)
    // at exactly the location the traversal ref resolves to.
    let planted = "planted-behavior-evidence-bundle.json";
    std::fs::write(
        root.join(planted),
        include_str!(
            "../../../examples/behavior-evidence-bundle-v1/behavior-evidence-bundle.complete.json"
        ),
    )
    .expect("planted bundle written");

    // `evidence/behavior/../../planted-...json` resolves to `<run_dir>/planted-...json`,
    // escaping the evidence tree the index is supposed to scope reads to.
    let unsafe_path = "evidence/behavior/../../planted-behavior-evidence-bundle.json";
    let journal = render_behavior_evidence_journal_section(&root, &evidence_index(unsafe_path));

    assert!(journal.contains("## Behavior Evidence Lifecycle"));
    assert!(journal.contains("Unsafe behavior evidence bundle path rejected"));
    // The out-of-tree bundle must never be parsed or rendered.
    assert!(!journal.contains("Lifecycle refs"));
    assert!(!journal.contains("Observed failure"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn update_journal_includes_indexed_behavior_evidence_bundle_section() {
    let root = unique_temp_dir("behavior-evidence-update-journal");
    let seed_path = root.join("seed.yaml");
    std::fs::write(&seed_path, VALID_SEED).expect("seed written");
    let artifacts = create_run(&seed_path, root.join("runs")).expect("run created");
    let evidence_path = "evidence/behavior/behavior-evidence-bundle.json";
    std::fs::create_dir_all(artifacts.run_dir.join("evidence/behavior")).expect("evidence dir");
    std::fs::write(
        artifacts.run_dir.join(evidence_path),
        include_str!(
            "../../../examples/behavior-evidence-bundle-v1/behavior-evidence-bundle.complete.json"
        ),
    )
    .expect("bundle fixture written");
    add_evidence_artifact(
        &artifacts.run_dir,
        "behavior-evidence-bundle",
        "application/json",
        evidence_path,
        serde_json::json!({ "artifact": "behavior_evidence_bundle" }),
    )
    .expect("behavior evidence indexed");

    let journal = update_journal(&artifacts.run_dir).expect("journal updates");

    assert!(journal.contains("## Behavior Evidence Lifecycle"));
    assert!(journal.contains("behavior-evidence-jump-boost"));
    assert!(journal.contains("Observed failure `jump_boost_cooldown_regression`"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn gameplay_logic_regression_v9_journal_renders_gl10_14_2_lifecycle_fixture() {
    let root = unique_temp_dir("gameplay-logic-regression-v9-journal");
    let evidence_path = "evidence/behavior-evidence-bundle.gl10.14.2.fixture.json";
    std::fs::create_dir_all(root.join("evidence")).expect("evidence dir");
    std::fs::write(
        root.join(evidence_path),
        include_str!(
            "../../../examples/gameplay-logic-regression-v9/evidence/behavior-evidence-bundle.gl10.14.2.fixture.json"
        ),
    )
    .expect("gl10.14.2 bundle fixture written");

    let journal = render_behavior_evidence_journal_section(&root, &evidence_index(evidence_path));

    assert!(journal.contains("## Behavior Evidence Lifecycle"));
    assert!(journal.contains("gameplay-logic-regression-v9-draft-apply-evidence"));
    assert!(journal.contains("Lifecycle refs: definitions `1`"));
    assert!(journal.contains("Observed failure `behavior-model-runtime-regression`"));
    assert!(journal.contains("inspect-read-model-next"));
    assert!(journal.contains("no arbitrary script execution"));
    std::fs::remove_dir_all(root).ok();
}

fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

const VALID_SEED: &str = r#"
id: behavior-evidence-journal
title: Behavior evidence journal fixture
goal: Render behavior evidence lifecycle in the journal.
constraints:
  target: local
acceptance:
  - journal records behavior evidence lifecycle
scenarios:
  - id: bootstrap-smoke
    description: Smoke scenario
"#;
