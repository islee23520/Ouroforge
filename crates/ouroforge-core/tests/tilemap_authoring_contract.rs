use std::path::{Path, PathBuf};

use ouroforge_core::tilemap_authoring::{
    tilemap_base_digest, validate_tilemap_draft_against_base, TilemapDraftArtifact,
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
