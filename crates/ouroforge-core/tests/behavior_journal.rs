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
