//! Standalone validator for docs/roadmap/milestone-classification-ledger.json.
//!
//! Run from the repository root:
//!
//! ```bash
//! rustc --edition=2021 --test docs/roadmap/milestone-ledger-validator.rs \
//!   -o /tmp/ouroforge-milestone-ledger-validator && \
//!   /tmp/ouroforge-milestone-ledger-validator --nocapture
//! ```
//!
//! This intentionally avoids adding a workspace crate or touching
//! `ouroforge-core::lib.rs` for a governance-only ledger check.

use std::fs;
use std::path::Path;

const LEDGER: &str = "docs/roadmap/milestone-classification-ledger.json";

#[derive(Debug)]
struct Entry {
    id: String,
    era: String,
    milestone_range: String,
    title: String,
    classification: String,
    confidence: String,
    evidence_refs: Vec<String>,
    gap_notes: String,
    rationale: String,
}

fn string_value(block: &str, key: &str) -> String {
    let needle = format!("\"{key}\": \"");
    let start = block
        .find(&needle)
        .unwrap_or_else(|| panic!("missing string field {key} in {block}"))
        + needle.len();
    let rest = &block[start..];
    let end = rest
        .find('"')
        .unwrap_or_else(|| panic!("unterminated string field {key}"));
    rest[..end].to_string()
}

fn array_values(block: &str, key: &str) -> Vec<String> {
    let needle = format!("\"{key}\": [");
    let start = block
        .find(&needle)
        .unwrap_or_else(|| panic!("missing array field {key} in {block}"))
        + needle.len();
    let rest = &block[start..];
    let end = rest
        .find(']')
        .unwrap_or_else(|| panic!("unterminated array field {key}"));
    rest[..end]
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim().trim_end_matches(',');
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map(str::to_string)
        })
        .collect()
}

fn parse_entries(text: &str) -> Vec<Entry> {
    let entries_start = text.find("\"entries\": [").expect("missing entries array");
    let entries_text = &text[entries_start..];
    entries_text
        .split("\n    {")
        .skip(1)
        .map(|raw| {
            let block = format!("{{{raw}");
            Entry {
                id: string_value(&block, "id"),
                era: string_value(&block, "era"),
                milestone_range: string_value(&block, "milestoneRange"),
                title: string_value(&block, "title"),
                classification: string_value(&block, "classification"),
                confidence: string_value(&block, "confidence"),
                evidence_refs: array_values(&block, "evidenceRefs"),
                gap_notes: string_value(&block, "gapNotes"),
                rationale: string_value(&block, "rationale"),
            }
        })
        .collect()
}

#[test]
fn ledger_has_required_shape_and_allowed_values() {
    let text = fs::read_to_string(LEDGER).expect("read ledger");
    assert!(text.contains("ouroforge.milestone-classification-ledger.v1"));
    assert!(text.contains("docs/product-observed-completion.md"));
    let entries = parse_entries(&text);
    assert!(entries.len() >= 23, "expected Era A-R plus mandatory groups");

    for entry in &entries {
        assert!(!entry.id.trim().is_empty(), "empty id");
        assert!(!entry.era.trim().is_empty(), "empty era for {}", entry.id);
        assert!(!entry.milestone_range.trim().is_empty(), "empty range for {}", entry.id);
        assert!(!entry.title.trim().is_empty(), "empty title for {}", entry.id);
        assert!(
            matches!(
                entry.classification.as_str(),
                "contract-complete" | "product-observed complete" | "product-observed fail" | "defer"
            ),
            "bad classification for {}: {}",
            entry.id,
            entry.classification
        );
        assert!(
            matches!(entry.confidence.as_str(), "low" | "medium" | "high"),
            "bad confidence for {}: {}",
            entry.id,
            entry.confidence
        );
        assert!(!entry.evidence_refs.is_empty(), "empty evidence refs for {}", entry.id);
        assert!(!entry.gap_notes.trim().is_empty(), "empty gap notes for {}", entry.id);
        assert!(!entry.rationale.trim().is_empty(), "empty rationale for {}", entry.id);
    }
}

#[test]
fn ledger_covers_required_eras_and_mandatory_groups() {
    let text = fs::read_to_string(LEDGER).expect("read ledger");
    let entries = parse_entries(&text);
    for era in 'A'..='R' {
        assert!(
            entries.iter().any(|entry| entry.era == era.to_string()),
            "missing era {era}"
        );
    }

    for id in [
        "group-runtime-foundation",
        "group-studio-surfaces",
        "group-generative-front-door-m30",
        "group-studio-adoption-m82-m87",
        "group-migration-m88-m95",
        "current-collect-and-exit-runtime-page",
    ] {
        assert!(entries.iter().any(|entry| entry.id == id), "missing {id}");
    }

    let collect_and_exit = entries
        .iter()
        .find(|entry| entry.id == "current-collect-and-exit-runtime-page")
        .expect("collect-and-exit entry");
    assert_eq!(collect_and_exit.classification, "contract-complete");
    assert!(collect_and_exit.gap_notes.contains("not product-observed"));
}

#[test]
fn ledger_evidence_refs_exist_for_repository_files() {
    let text = fs::read_to_string(LEDGER).expect("read ledger");
    let entries = parse_entries(&text);
    for entry in &entries {
        for evidence_ref in &entry.evidence_refs {
            if evidence_ref.starts_with("docs/")
                || evidence_ref.starts_with("examples/")
                || evidence_ref == "README.md"
            {
                assert!(
                    Path::new(evidence_ref).exists(),
                    "{} references missing evidence file {}",
                    entry.id,
                    evidence_ref
                );
            }
        }
    }
}
