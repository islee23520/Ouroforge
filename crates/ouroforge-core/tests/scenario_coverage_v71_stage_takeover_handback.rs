//! Scenario Coverage v71 regression suite for #2076 / Era M M80.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn matrix() -> Value {
    read_json("examples/stage-takeover-handback-v1/scenario-coverage-v71/matrix.fixture.json")
}

#[test]
fn v71_matrix_records_stage_takeover_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v71-stage-takeover-handback-v1"
    );
    assert_eq!(matrix["coverageVersion"], 71);
    assert_eq!(matrix["issueRef"], "#2076");
    assert_eq!(matrix["milestone"], "Era M M80");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();

    for required in [
        "stage-takeover-locks-local-session-only",
        "manual-work-captured-as-evidence",
        "handback-reverifies-through-rust-gates",
        "no-raw-bypass-from-elixir-control-plane",
        "loop-completes-without-human-input",
        "coverage-v71-boundaries",
    ] {
        assert!(ids.contains(required), "missing v71 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }
}

#[test]
fn v71_autonomy_two_plane_and_handback_invariants_fail_closed() {
    let invariants = matrix()["autonomyInvariants"].clone();
    assert_eq!(invariants["humanInputRequired"], false);
    assert_eq!(invariants["loopCompletesWithoutHuman"], true);
    assert_eq!(invariants["manualWorkRequiresEvidence"], true);
    assert_eq!(invariants["handbackRequiresReverify"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["rustDataPlaneOwnsValidation"], true);
    assert_eq!(invariants["hostedCollaborativeStudioDeferred"], true);
    assert_eq!(invariants["cliFallbackIntact"], true);

    let boundary = matrix()["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();

    for required in [
        "opt-in",
        "evidence/provenance metadata",
        "validated, recorded, and reverified",
        "existing rust cli gates",
        "elixir/phoenix is local control and presentation only",
        "never writes artifacts",
        "rust remains the data plane",
        "zero human input",
        "no new write path",
        "no new data store",
        "no hosted",
        "no mandatory human",
        "cli fallback remains intact",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v71_elixir_control_plane_has_no_raw_artifact_write_calls() {
    let lib_dir = repo_root().join("studio/executor/lib");
    let mut offenders = Vec::new();

    fn visit(dir: &Path, offenders: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).expect("read studio lib dir") {
            let entry = entry.expect("studio lib entry");
            let path = entry.path();
            if path.is_dir() {
                visit(&path, offenders);
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("ex") {
                continue;
            }

            let text = std::fs::read_to_string(&path).expect("read elixir source");
            for raw_write in [
                "File.write",
                ":file.write",
                "File.open",
                ":file.open",
                "File.rm",
                ":file.delete",
            ] {
                if text.contains(raw_write) {
                    offenders.push(format!("{} contains {raw_write}", path.display()));
                }
            }
        }
    }

    visit(&lib_dir, &mut offenders);
    assert!(
        offenders.is_empty(),
        "Elixir control plane raw-write bypass candidates: {offenders:#?}"
    );
}

#[test]
fn v71_docs_record_coverage_and_guardrails() {
    let doc =
        read_text("docs/scenario-coverage-v71-stage-takeover-handback.md").to_ascii_lowercase();

    for required in [
        "coverage v71",
        "human takeover is opt-in",
        "autonomous loop",
        "evidence and provenance metadata",
        "rust cli validation",
        "control and presentation",
        "rust remains the data plane",
        "no hosted",
        "cli fallback",
        "fun/taste",
        "release go/no-go",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
