//! Export Plan Generator v1 contract (#722).
//!
//! A plan is a deterministic dry-run derived from a validated export profile. It
//! separates inputs, outputs, blocked-file policy, and verification steps;
//! produces no side effects; and fails closed before bundle assembly on blocked
//! targets, unsafe paths, missing inputs, and duplicate/colliding inputs.

use ouroforge_core::export_plan::{
    ExportPlan, PlannedInputKind, PlannedOutputKind, EXPORT_PLAN_SCHEMA_VERSION,
};

const VALID_PROFILE: &str =
    include_str!("../../../examples/export-profile-v1/export-profile.valid.fixture.json");

fn invalid_profile(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-profile-v1/invalid")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

fn invalid_plan_input(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-plan-v1/invalid")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn plan_from_valid_profile_separates_inputs_outputs_and_steps() {
    let plan = ExportPlan::from_profile_json(VALID_PROFILE).expect("valid profile plans");
    assert_eq!(plan.schema_version, EXPORT_PLAN_SCHEMA_VERSION);
    assert_eq!(plan.plan_id, "plan_profile_collect_and_exit_web_local");
    assert_eq!(plan.profile_id, "profile_collect_and_exit_web_local");
    assert_eq!(plan.export_target, "web-local");
    assert_eq!(plan.output_dir, "dist/export/collect-and-exit");

    // Entry scene + two asset roots = three source inputs in declared order.
    assert_eq!(plan.source_inputs.len(), 3);
    assert_eq!(plan.source_inputs[0].kind, PlannedInputKind::EntryScene);
    assert_eq!(plan.source_inputs[1].kind, PlannedInputKind::AssetRoot);

    // html + bootstrap + 2 asset payloads + manifest + checksums = 6 outputs.
    assert_eq!(plan.generated_outputs.len(), 6);
    assert_eq!(plan.generated_outputs[0].kind, PlannedOutputKind::HtmlEntry);
    assert_eq!(
        plan.generated_outputs[0].staged_path,
        "dist/export/collect-and-exit/index.html"
    );
    assert!(plan
        .generated_outputs
        .iter()
        .any(|o| o.kind == PlannedOutputKind::AssetManifest));
    assert!(plan
        .generated_outputs
        .iter()
        .any(|o| o.kind == PlannedOutputKind::Checksums));

    // Every staged output stays inside the declared output directory.
    for output in &plan.generated_outputs {
        assert!(
            output
                .staged_path
                .starts_with("dist/export/collect-and-exit/"),
            "output {} escaped the staging dir",
            output.staged_path
        );
    }

    assert_eq!(plan.expected_artifacts.len(), plan.generated_outputs.len());

    // Standard steps plus one smoke per scenario id.
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "load-without-console-errors"));
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "runtime-probe-compatibility"));
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "scenario-smoke:scenario_reach_exit"));

    assert!(plan.blocked_files.iter().any(|p| p == "target/"));
    assert!(plan.blocked_files.iter().any(|p| p == ".git/"));
}

#[test]
fn plan_generation_is_deterministic() {
    let a = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    let b = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_dry_run_json().unwrap(), b.to_dry_run_json().unwrap());
}

#[test]
fn plan_json_round_trips() {
    let plan = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    let json = plan.to_dry_run_json().unwrap();
    let parsed: ExportPlan = serde_json::from_str(&json).unwrap();
    assert_eq!(plan, parsed);
}

#[test]
fn plan_fails_closed_on_blocked_target() {
    let err = ExportPlan::from_profile_json(&invalid_profile("blocked-target.json"))
        .expect_err("blocked target rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_unsafe_output_path() {
    let err = ExportPlan::from_profile_json(&invalid_profile("path-traversal-output.json"))
        .expect_err("unsafe output rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_missing_entry_scene() {
    let err = ExportPlan::from_profile_json(&invalid_profile("missing-entry-scene.json"))
        .expect_err("missing entry scene rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_duplicate_asset_root() {
    let err = ExportPlan::from_profile_json(&invalid_plan_input("duplicate-asset-root.json"))
        .expect_err("duplicate asset root rejected");
    assert!(err.to_string().contains("duplicate asset root"));
}

#[test]
fn plan_fails_closed_on_colliding_asset_segment() {
    let err = ExportPlan::from_profile_json(&invalid_plan_input("colliding-asset-segment.json"))
        .expect_err("colliding asset output segment rejected");
    assert!(err.to_string().contains("colliding asset output segment"));
}
