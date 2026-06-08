//! Contract test for Deduplication and Novelty Metrics v1 (#1650).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. These tests machine-check the dedup/novelty contract over a
//! generated proposal set (#1649): duplicates are detected, a repetitive set is
//! flagged low-novelty against a declared threshold, the metrics are
//! deterministic, and an optional Milestone 28 difficulty signal refines the
//! dedup. The metrics are *computed* from existing artifacts/evidence (a
//! content digest, not a similarity engine) and are descriptive only — never a
//! quality, fun, or taste claim. Malformed inputs fail closed.

use std::collections::BTreeMap;
use std::path::PathBuf;

use ouroforge_core::content_novelty::{
    compute_novelty, CONTENT_NOVELTY_SCHEMA_VERSION, DEFAULT_NOVELTY_THRESHOLD,
};
use ouroforge_core::content_scale_generation::{
    generate_campaign, CampaignBrief, CampaignProposalSet,
};

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn generated_set(fixture: &str) -> CampaignProposalSet {
    let path: PathBuf = repo_root()
        .join("examples/generative-front-door")
        .join(fixture);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {fixture}"));
    let brief = CampaignBrief::from_json_str(&text).expect("fixture campaign brief parses");
    generate_campaign(&brief, FIXED_NOW_MS).expect("fixture campaign generates")
}

fn no_signals() -> BTreeMap<String, String> {
    BTreeMap::new()
}

#[test]
fn duplicate_detection_flags_the_repeated_item() {
    let set = generated_set("content-novelty-campaign-v1.json");
    let report = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &no_signals())
        .expect("novelty report computes");

    report.validate().expect("report validates");
    assert_eq!(report.schema_version, CONTENT_NOVELTY_SCHEMA_VERSION);
    assert_eq!(report.item_count, 4);
    assert_eq!(report.distinct_count, 3);
    assert_eq!(report.duplicate_count, 1);

    // Exactly one duplicate cluster, containing the two structurally identical
    // items (different ids).
    assert_eq!(report.duplicate_groups.len(), 1);
    let group = &report.duplicate_groups[0];
    assert_eq!(
        group.item_ids,
        vec![
            "generative-novelty-a-1".to_string(),
            "generative-novelty-a-2".to_string()
        ]
    );

    // The second item is the flagged duplicate; the first is not.
    let a1 = report
        .items
        .iter()
        .find(|i| i.item_id == "generative-novelty-a-1")
        .unwrap();
    let a2 = report
        .items
        .iter()
        .find(|i| i.item_id == "generative-novelty-a-2")
        .unwrap();
    assert!(!a1.is_duplicate);
    assert!(a2.is_duplicate);
    assert_eq!(a2.duplicate_of.as_deref(), Some("generative-novelty-a-1"));
    assert_eq!(a1.signature, a2.signature);

    // A mostly-distinct set is not flagged low-novelty at the default threshold.
    assert!((report.novelty_ratio - 0.75).abs() < 1e-9);
    assert!(!report.low_novelty);
}

#[test]
fn low_novelty_set_is_flagged() {
    let set = generated_set("content-novelty-campaign-low-v1.json");
    let report = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &no_signals())
        .expect("novelty report computes");

    assert_eq!(report.item_count, 3);
    assert_eq!(report.distinct_count, 1);
    assert_eq!(report.duplicate_count, 2);
    assert!((report.novelty_ratio - (1.0 / 3.0)).abs() < 1e-9);
    assert!(
        report.low_novelty,
        "a repetitive set must be flagged low-novelty, ratio {}",
        report.novelty_ratio
    );
}

#[test]
fn metrics_are_deterministic() {
    let set = generated_set("content-novelty-campaign-v1.json");
    let first = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &no_signals()).expect("first");
    let second = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &no_signals()).expect("second");
    assert_eq!(first, second);
}

#[test]
fn difficulty_signal_refines_dedup() {
    // Reusing the Milestone 28 difficulty/solver signal: two items with
    // identical content but a different measured difficulty are no longer
    // treated as duplicates.
    let set = generated_set("content-novelty-campaign-v1.json");
    let mut signals = BTreeMap::new();
    signals.insert(
        "generative-novelty-a-1".to_string(),
        "difficulty:easy".to_string(),
    );
    signals.insert(
        "generative-novelty-a-2".to_string(),
        "difficulty:hard".to_string(),
    );

    let report = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &signals).expect("computes");
    assert_eq!(
        report.duplicate_count, 0,
        "distinct difficulty must break the tie"
    );
    assert_eq!(report.distinct_count, 4);
    assert!(report.duplicate_groups.is_empty());
}

#[test]
fn out_of_range_threshold_fails_closed() {
    let set = generated_set("content-novelty-campaign-v1.json");
    let error = compute_novelty(&set, 1.5, &no_signals())
        .expect_err("an out-of-range threshold must fail closed");
    assert!(
        error.to_string().contains("threshold must be in"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_novelty_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/content-novelty-v1.md"))
        .expect("content-novelty doc exists");
    assert!(
        doc.contains("#1650"),
        "Content-Novelty v1 doc records this issue (#1650)"
    );
    assert!(
        doc.contains("read/measure") || doc.contains("never destructive"),
        "doc records the non-destructive dedup contract"
    );
}
