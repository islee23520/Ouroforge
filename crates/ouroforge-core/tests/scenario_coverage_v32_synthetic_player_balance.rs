//! Scenario Coverage v32 (#1610).
use std::path::{Path, PathBuf};
use serde_json::Value;
fn root() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../..") }
fn read_json(rel: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(rel)).expect("read")).expect("json")
}
#[test]
fn v32_matrix_and_golden() {
    let m = read_json("examples/synthetic-player-balance-v1/scenario-coverage-v32/matrix.fixture.json");
    assert_eq!(m["issue"], 1610);
    assert!(root().join(m["personaContract"].as_str().unwrap()).is_file());
    let g = read_json(m["compareGolden"].as_str().unwrap());
    assert_eq!(g["fixtureScoped"], true);
}
#[test]
fn v32_doc() {
    let doc = std::fs::read_to_string(root().join("docs/scenario-coverage-v32.md")).unwrap();
    assert!(doc.contains("#1") && doc.contains("fixture-scoped"));
}
