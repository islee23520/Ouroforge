use std::path::{Path, PathBuf};

use ouroforge_core::tilemap_authoring::{
    tilemap_base_digest, tilemap_reachability_report_from_json_str,
    validate_tilemap_draft_against_base, TilemapDraftArtifact, TilemapReachabilityDiagnostic,
    TilemapReachabilityStatus, TilemapSourceArtifact, TILEMAP_SOURCE_PATH_PREFIX,
    TILEMAP_SOURCE_PATH_SUFFIX,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"))
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

#[test]
fn tilemap_source_spec_names_shared_format_and_path_convention() {
    let docs = read_text("docs/tilemap-authoring-v1.md");
    assert!(docs.contains("Issue: #2369"));
    assert!(docs.contains("ouroforge.tilemap-source.v1"));
    assert!(docs.contains("examples/tilemap-authoring-v1/maps/<map-id>.tilemap.json"));
    assert!(docs.contains("M128 content and"));
    assert!(docs.contains("#1 and #23 remain open"));
    assert!(!docs.contains("production-ready editor"));

    let map = TilemapSourceArtifact::from_json_str(&read_text(
        "examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json",
    ))
    .expect("valid dogfood tilemap validates");
    assert_eq!(
        map.source_path,
        format!("{TILEMAP_SOURCE_PATH_PREFIX}valid-dogfood{TILEMAP_SOURCE_PATH_SUFFIX}")
    );
    assert_eq!(map.layers.len(), 4);
}

#[test]
fn tilemap_draft_uses_base_relative_digest_and_validates_operations() {
    let base = read_text("examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json");
    let draft = TilemapDraftArtifact::from_json_str(&read_text(
        "examples/tilemap-authoring-v1/drafts/valid-dogfood-open-lane.tilemap-draft.json",
    ))
    .expect("draft fixture parses");
    assert_eq!(draft.target.base_digest, tilemap_base_digest(&base));

    let preview = validate_tilemap_draft_against_base(&draft, &base).expect("draft validates");
    assert!(preview.generated_preview_only);
    assert_eq!(preview.changed_cells.len(), 2);
    assert_eq!(preview.affected_collision_cells[0].x, 3);
    assert_eq!(
        preview.affected_trigger_markers,
        vec!["trigger-key".to_string()]
    );

    let mut stale = draft.clone();
    stale.target.base_digest = "fnv64:0000000000000000".to_string();
    let error =
        validate_tilemap_draft_against_base(&stale, &base).expect_err("stale draft rejects");
    assert!(error.to_string().contains("baseDigest is stale"));
}

#[test]
fn tilemap_source_fixture_json_shape_is_stable_for_m128_content() {
    let map = read_json("examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json");
    assert_eq!(map["schemaVersion"], "ouroforge.tilemap-source.v1");
    assert_eq!(
        map["pathConvention"],
        "examples/tilemap-authoring-v1/maps/<map-id>.tilemap.json"
    );
    assert_eq!(
        map["sourcePath"],
        "examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json"
    );
}

#[test]
fn tilemap_draft_preview_evidence_is_generated_not_source() {
    let evidence =
        read_json("examples/tilemap-authoring-v1/evidence/valid-dogfood-draft-preview.json");
    assert_eq!(
        evidence["schemaVersion"],
        "ouroforge.tilemap-draft-preview-evidence.v1"
    );
    assert_eq!(evidence["issueRef"], "#2370");
    assert_eq!(evidence["generatedPreviewOnly"], true);
    assert_eq!(evidence["changedCells"].as_array().unwrap().len(), 2);
    assert!(evidence["boundary"]
        .as_str()
        .unwrap()
        .contains("not browser trusted write"));
    assert!(repo_root()
        .join(evidence["draftRef"].as_str().unwrap())
        .exists());
    assert!(repo_root()
        .join(evidence["targetTilemapRef"].as_str().unwrap())
        .exists());
}

#[test]
fn blocked_map_fails_with_named_reachability_diagnostic() {
    let report = tilemap_reachability_report_from_json_str(
        &read_text("examples/tilemap-authoring-v1/maps/blocked-dogfood.tilemap.json"),
        None,
    )
    .expect("blocked map produces report");
    assert_eq!(report.status, TilemapReachabilityStatus::Blocked);
    assert!(report
        .diagnostics
        .contains(&TilemapReachabilityDiagnostic::ObjectiveUnreachable));
    assert!(report.objective_path.is_empty());
}

#[test]
fn reachable_map_links_generated_scenario_assertion_draft() {
    let report = tilemap_reachability_report_from_json_str(
        &read_text("examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json"),
        Some(&read_text(
            "examples/tilemap-authoring-v1/evidence/valid-dogfood-live-replay.json",
        )),
    )
    .expect("valid map produces report");
    assert_eq!(report.status, TilemapReachabilityStatus::Passed);
    assert!(report.diagnostics.is_empty());
    assert!(report
        .scenario_assertion_draft_ref
        .ends_with("valid-dogfood-assertion-draft.json"));
    assert!(repo_root()
        .join(&report.scenario_assertion_draft_ref)
        .exists());
    assert!(report.boundary.contains("no browser trusted writes"));
}

#[test]
fn valid_map_requires_and_accepts_live_replay_objective_evidence() {
    let map = read_text("examples/tilemap-authoring-v1/maps/valid-dogfood.tilemap.json");
    let without_replay = tilemap_reachability_report_from_json_str(&map, None)
        .expect("valid map without live replay reports gap");
    assert_eq!(without_replay.status, TilemapReachabilityStatus::Blocked);
    assert!(without_replay
        .diagnostics
        .contains(&TilemapReachabilityDiagnostic::NoLiveReplayEvidence));

    let report = tilemap_reachability_report_from_json_str(
        &map,
        Some(&read_text(
            "examples/tilemap-authoring-v1/evidence/valid-dogfood-live-replay.json",
        )),
    )
    .expect("valid map with live replay passes");
    assert_eq!(report.status, TilemapReachabilityStatus::Passed);
    assert!(report.diagnostics.is_empty());
    assert_eq!(report.objective_path.last().unwrap().x, 4);
    let live_replay_ref = report.live_replay_ref.unwrap();
    assert!(live_replay_ref.ends_with("valid-dogfood-live-replay.json"));
    assert!(repo_root().join(live_replay_ref).exists());
}

#[test]
fn scenario_coverage_v104_lands_final_m123_suite() {
    let matrix =
        read_json("examples/tilemap-authoring-v1/scenario-coverage-v104/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v104-tilemap-level-editor-v1"
    );
    assert_eq!(matrix["coverageVersion"], 104);
    assert_eq!(matrix["issueRef"], "#2371");
    let rows = matrix["rows"].as_array().expect("rows array");
    for required in [
        "v104.tilemap-source-format-and-path-convention",
        "v104.base-relative-digest-draft-preview",
        "v104.blocked-map-named-diagnostic",
        "v104.valid-map-live-replay-objective-reached",
    ] {
        assert!(
            rows.iter().any(|row| row["id"] == required),
            "missing {required}"
        );
    }
    for row in rows {
        assert_eq!(row["status"], "pass");
        assert!(repo_root()
            .join(row["evidenceRef"].as_str().unwrap())
            .exists());
    }
    assert_eq!(matrix["invariants"]["browserTrustedWritesAllowed"], false);
    assert_eq!(matrix["invariants"]["baseDigestRequired"], true);
    assert_eq!(matrix["invariants"]["validFixtureRequiresLiveReplay"], true);
}
