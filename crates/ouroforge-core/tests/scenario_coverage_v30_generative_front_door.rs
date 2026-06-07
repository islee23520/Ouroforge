//! Scenario Coverage v30 — Generative Front Door (#1597).

use std::path::{Path, PathBuf};
use ouroforge_core::generative_intake::{intake_brief, GenerativeBrief};
use serde_json::Value;

fn root() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../..") }
fn read_json(rel: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(rel)).expect("read")).expect("json")
}
fn read_brief(rel: &str) -> GenerativeBrief {
    GenerativeBrief::from_json_str(&std::fs::read_to_string(root().join(rel)).expect("read")).expect("brief")
}
const MATRIX: &str = "examples/generative-front-door/scenario-coverage-v30/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v30.md";
const NOW: u128 = 1_725_000_000_000;

#[test]
fn v30_matrix_pinned() {
    let m = read_json(MATRIX);
    assert_eq!(m["schemaVersion"], "scenario-coverage-v30-generative-front-door-matrix-v1");
    assert_eq!(m["issue"], 1597);
}

#[test]
fn v30_intake_valid_and_invalid() {
    let m = read_json(MATRIX);
    let valid = read_brief(m["intake"]["valid"].as_str().unwrap());
    intake_brief(&valid, NOW).expect("valid intake");
    let invalid = read_brief(m["intake"]["invalid"].as_str().unwrap());
    assert!(intake_brief(&invalid, NOW).is_err());
}

#[test]
fn v30_promotion_and_accessibility_fixtures_exist() {
    let m = read_json(MATRIX);
    for section in ["promotion", "accessibility"] {
        for (_k, v) in m[section].as_object().unwrap() {
            let p = root().join(v.as_str().unwrap());
            assert!(p.is_file(), "fixture {}", p.display());
        }
    }
}

#[test]
fn v30_doc_governance() {
    let doc = std::fs::read_to_string(root().join(DOC)).expect("doc");
    assert!(doc.contains("#1") && doc.contains("#23") && doc.contains("fixture-scoped"));
}
