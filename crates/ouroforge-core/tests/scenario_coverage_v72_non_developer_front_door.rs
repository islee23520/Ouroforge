//! Scenario Coverage v72 regression suite for #2081 / Era N M82.

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
    read_json(
        "examples/non-developer-generative-front-door-v1/scenario-coverage-v72/matrix.fixture.json",
    )
}

#[test]
fn v72_matrix_records_guided_front_door_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v72-non-developer-generative-front-door-v1"
    );
    assert_eq!(matrix["coverageVersion"], 72);
    assert_eq!(matrix["issueRef"], "#2081");
    assert_eq!(matrix["milestone"], "Era N M82");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();

    for required in [
        "guided-intake-captures-intervention-evidence",
        "proposal-preview-is-deterministic-and-proposal-only",
        "human-write-routes-through-rust-gates",
        "no-raw-bypass-from-elixir-guided-surface",
        "loop-completes-without-human-input",
        "coverage-v72-boundaries",
    ] {
        assert!(ids.contains(required), "missing v72 row {required}");
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
fn v72_autonomy_two_plane_and_proposal_only_invariants_fail_closed() {
    let invariants = matrix()["autonomyInvariants"].clone();
    assert_eq!(invariants["humanInputRequired"], false);
    assert_eq!(invariants["loopCompletesWithoutHuman"], true);
    assert_eq!(invariants["guidedInputIsInterventionEvidence"], true);
    assert_eq!(invariants["proposalPreviewIsDeterministic"], true);
    assert_eq!(invariants["proposalOnlyUntilReviewApply"], true);
    assert_eq!(invariants["humanWriteRoutesThroughRustGate"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["elixirOwnsProposalSemantics"], false);
    assert_eq!(invariants["autoApplyPerformed"], false);
    assert_eq!(invariants["reviewerBypass"], false);
    assert_eq!(invariants["rustDataPlaneOwnsValidation"], true);
    assert_eq!(invariants["hostedCollaborativeStudioDeferred"], true);
    assert_eq!(invariants["cliFallbackIntact"], true);

    let boundary = matrix()["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();

    for required in [
        "opt-in",
        "intervention-as-evidence",
        "deterministic proposal preview",
        "proposal-only",
        "generative-front-door validate",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence",
        "provenance",
        "elixir/phoenix is local control and presentation only",
        "never writes artifacts",
        "rust remains the data plane",
        "zero human input",
        "no raw bypass",
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
fn v72_elixir_guided_surface_has_no_raw_artifact_write_or_apply_authority() {
    let lib_dir = repo_root().join("studio/executor/lib/ouroforge_executor");
    let mut offenders = Vec::new();

    for filename in [
        "guided_generative_front_door.ex",
        "guided_generative_front_door_demo.ex",
    ] {
        let path = lib_dir.join(filename);
        let text = std::fs::read_to_string(&path).expect("read guided front-door elixir source");
        for raw_write in [
            "File.write",
            ":file.write",
            ":file.open",
            "File.open",
            "File.rm",
            "File.cp",
            "File.rename",
            "File.touch",
            ":file.delete",
            ":file.rename",
            "directArtifactWrite: true",
            "studioTrustedWriteAuthority: true",
            "rawBypassRequested: true",
            "elixirOwnsProposalSemantics: true",
            "autoApplyPerformed: true",
            "reviewerBypass: true",
            "trusted_write_authority?: true",
            "auto_apply_performed?: true",
        ] {
            if text.contains(raw_write) {
                offenders.push(format!("{} contains {raw_write}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Elixir guided front-door raw-write/apply bypass candidates: {offenders:#?}"
    );
}

#[test]
fn v72_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v72-non-developer-generative-front-door.md")
        .to_ascii_lowercase();

    for required in [
        "coverage v72",
        "human brief/conversation input is opt-in",
        "autonomous loop",
        "intervention-as-evidence",
        "proposal preview is deterministic",
        "proposal-only",
        "rust generative-front-door validation",
        "review/apply",
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
