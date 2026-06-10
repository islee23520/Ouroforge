use std::path::{Path, PathBuf};

use ouroforge_core::tilemap_authoring::{
    TilemapSourceArtifact, TILEMAP_SOURCE_PATH_PREFIX, TILEMAP_SOURCE_PATH_SUFFIX,
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
    assert!(map
        .markers
        .iter()
        .any(|marker| marker.marker_id == "spawn-player"));
    assert!(map
        .guardrails
        .join(" ")
        .contains("not write trusted tilemap files"));
}

#[test]
fn scenario_coverage_v104_is_not_landed_before_final_m123_issue() {
    assert!(!repo_root()
        .join("examples/tilemap-authoring-v1/scenario-coverage-v104/matrix.fixture.json")
        .exists());
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
