use ouroforge_core::{add_evidence_artifact, create_run, read_dashboard_run};

#[test]
fn dashboard_read_model_exposes_behavior_evidence_lifecycle() {
    let root = unique_temp_dir("behavior-dashboard");
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

    let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");

    assert!(model.behavior_evidence.present);
    assert_eq!(model.behavior_evidence.bundle_count, 1);
    assert_eq!(model.behavior_evidence.malformed_count, 0);
    assert_eq!(model.behavior_evidence.complete_count, 1);
    assert_eq!(model.behavior_evidence.partial_count, 0);
    assert_eq!(model.behavior_evidence.blocked_count, 0);
    assert_eq!(model.behavior_evidence.stale_count, 0);
    assert_eq!(model.behavior_evidence.lifecycle_ref_count, 8);
    assert_eq!(model.behavior_evidence.observed_failure_count, 1);
    assert_eq!(model.behavior_evidence.next_step_hypothesis_count, 1);
    assert!(model.behavior_evidence.boundary.contains("read-only"));
    assert!(model.behavior_evidence.boundary.contains("command bridge"));
    let bundle = model
        .behavior_evidence
        .bundles
        .first()
        .expect("bundle row present");
    assert_eq!(bundle.bundle_id, "behavior-evidence-jump-boost");
    assert_eq!(bundle.status, "complete");
    assert_eq!(bundle.path, evidence_path);
    assert!(bundle.read_error.is_none());
    assert!(bundle
        .observed_failures
        .iter()
        .any(|failure| failure.summary.contains("Cooldown")));
    assert!(bundle
        .next_step_hypotheses
        .iter()
        .any(|hypothesis| hypothesis.id == "hypothesis-rerun-after-rollback"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn dashboard_read_model_keeps_malformed_behavior_evidence_visible() {
    let root = unique_temp_dir("behavior-dashboard-malformed");
    let seed_path = root.join("seed.yaml");
    std::fs::write(&seed_path, VALID_SEED).expect("seed written");
    let artifacts = create_run(&seed_path, root.join("runs")).expect("run created");
    let evidence_path = "evidence/behavior/behavior-evidence-bundle.json";
    std::fs::create_dir_all(artifacts.run_dir.join("evidence/behavior")).expect("evidence dir");
    std::fs::write(artifacts.run_dir.join(evidence_path), "{not json")
        .expect("malformed bundle written");
    add_evidence_artifact(
        &artifacts.run_dir,
        "behavior-evidence-bundle",
        "application/json",
        evidence_path,
        serde_json::json!({ "artifact": "behavior_evidence_bundle" }),
    )
    .expect("behavior evidence indexed");

    let model = read_dashboard_run(&artifacts.run_dir).expect("dashboard run reads");

    assert!(model.behavior_evidence.present);
    assert_eq!(model.behavior_evidence.bundle_count, 0);
    assert_eq!(model.behavior_evidence.malformed_count, 1);
    assert_eq!(model.behavior_evidence.complete_count, 0);
    assert_eq!(model.behavior_evidence.partial_count, 0);
    assert_eq!(model.behavior_evidence.blocked_count, 0);
    assert_eq!(model.behavior_evidence.stale_count, 0);
    assert_eq!(model.behavior_evidence.status, "malformed");
    assert_eq!(model.behavior_evidence.bundles.len(), 1);
    assert!(model.behavior_evidence.bundles[0]
        .read_error
        .as_deref()
        .unwrap_or_default()
        .contains("failed to parse"));
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
id: behavior-dashboard
title: Behavior evidence dashboard fixture
goal: Render behavior evidence lifecycle in dashboard read models.
constraints:
  target: local
acceptance:
  - dashboard records behavior evidence lifecycle
scenarios:
  - id: bootstrap-smoke
    description: Smoke scenario
"#;
