//! Scenario Coverage v82 regression suite for #2189 / Era O M94.

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
    read_json("examples/migration-ux-studio-demo/scenario-coverage-v82/matrix.fixture.json")
}

#[test]
fn v82_matrix_records_rows_boundaries_and_ledger_refs() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v82-migration-ux-studio-v1"
    );
    assert_eq!(matrix["coverageVersion"], 82);
    assert_eq!(matrix["issueRef"], "#2189");
    assert_eq!(matrix["milestone"], "Era O M94");
    assert_eq!(
        matrix["contractRef"],
        "docs/migration-ux-studio-contract-v1.md"
    );
    assert_eq!(
        matrix["demoRef"],
        "examples/migration-ux-studio-demo/run-demo.sh"
    );

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v82.studio-import-wizard-source-only",
        "v82.studio-fidelity-report-lossy-not-clean",
        "v82.studio-no-auto-port-without-oracle",
        "v82.studio-deterministic-hash-evidence-required",
        "v82.studio-no-trusted-elixir-write",
        "v82.coverage-ledger-and-demo-script",
    ] {
        assert!(ids.contains(required), "missing v82 row {required}");
    }

    for row in rows {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidence ref");
        assert!(
            repo_root().join(evidence_ref).exists(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }

    let invariants = &matrix["invariants"];
    assert_eq!(invariants["oneWayOnRamp"], true);
    assert_eq!(invariants["sourceProjectOpenTextOnly"], true);
    assert_eq!(invariants["cleanRoomReDerivation"], true);
    assert_eq!(invariants["autoPortWithoutOracleAllowed"], false);
    assert_eq!(invariants["lossyImportMayGradeGreen"], false);
    assert_eq!(invariants["ungatedAutoTranslatedPortAllowed"], false);
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["stateHashBreakMustFail"], true);
    assert_eq!(invariants["runtimeBridgeAllowed"], false);
    assert_eq!(invariants["embeddedEngineRuntimeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["newStudioDataStore"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);

    let boundary = matrix["boundary"].as_str().expect("boundary");
    for required in [
        "Scenario Coverage v82",
        "source-project-only",
        "no auto-port",
        "deterministic state-hash",
        "no trusted Elixir writes",
        "no new data store",
        "#1/#23 open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v82_demo_summary_records_honest_fidelity_and_no_port_claims() {
    let summary = read_json("examples/migration-ux-studio-demo/demo-summary.fixture.json");
    assert_eq!(summary["version"], "migration-ux-studio-demo-v1");
    assert_eq!(summary["fidelitySummary"]["green"], 1);
    assert_eq!(summary["fidelitySummary"]["yellow"], 1);
    assert_eq!(summary["fidelitySummary"]["red"], 1);
    assert_eq!(summary["claimedPortedUnits"].as_array().unwrap().len(), 0);
    assert_eq!(summary["noAutoPortClaim"], true);
    assert_eq!(summary["oracleGated"], true);
    assert_eq!(summary["cleanRoom"], true);
    assert_eq!(summary["studioTrustedWriteAuthority"], false);
    assert_eq!(summary["directArtifactWrite"], false);

    let commands = summary["scriptedCommands"].as_array().expect("commands");
    assert!(commands.iter().any(|cmd| cmd
        .as_array()
        .unwrap()
        .windows(2)
        .any(|w| w[0] == "migration" && w[1] == "verify-demo")));
    assert!(commands.iter().any(|cmd| cmd
        .as_array()
        .unwrap()
        .windows(2)
        .any(|w| w[0] == "migration" && w[1] == "unity-demo")));
    assert!(commands.iter().any(|cmd| cmd
        .as_array()
        .unwrap()
        .windows(2)
        .any(|w| w[0] == "run" && w[1] == "seeds/migration-demo.yaml")));

    let determinism = summary["determinism"].as_str().expect("determinism");
    assert!(determinism.contains("state-hash"));
    assert!(determinism.contains("perceptual secondary"));
}

#[test]
fn v82_studio_tests_lock_negative_regressions() {
    let ux_tests = read_text("studio/executor/test/ouroforge_executor/migration_ux_test.exs");
    for required in [
        "wizard rejects shipped builds",
        ":source_project_only",
        ":ported_claim_forbidden",
        ":red_without_era_r_task",
        ":invalid_fidelity_row",
        "ir_state_hash",
        "verification_state_hash",
        "trusted write",
    ] {
        assert!(
            ux_tests.contains(required),
            "migration_ux_test missing {required}"
        );
    }

    let demo_tests =
        read_text("studio/executor/test/ouroforge_executor/migration_ux_demo_test.exs");
    for required in [
        "claimedPortedUnits == []",
        "deterministicHashes",
        "trustedWriteAuthority",
        ":trusted_write_forbidden",
        "No auto-port claim: true",
    ] {
        assert!(
            demo_tests.contains(required),
            "migration_ux_demo_test missing {required}"
        );
    }
}

#[test]
fn v82_studio_lib_has_no_trusted_write_primitives_or_runtime_bridge() {
    for path in [
        "studio/executor/lib/ouroforge_executor/migration_ux.ex",
        "studio/executor/lib/ouroforge_executor/migration_import_session.ex",
        "studio/executor/lib/ouroforge_executor/migration_ux_demo.ex",
    ] {
        let text = read_text(path);
        for forbidden in [
            "File.write",
            ":file.write",
            ":file.open",
            "liveBridge: true",
            "embeddedEngineRuntime: true",
            "studioTrustedWriteAuthority: true",
        ] {
            assert!(
                !text.contains(forbidden),
                "{path} contains forbidden {forbidden}"
            );
        }
        assert!(
            text.contains("Rust") || text.contains("rust"),
            "{path} must mention Rust data-plane ownership"
        );
    }
}

#[test]
fn v82_docs_record_verification_and_open_anchor_boundaries() {
    let doc = read_text("docs/scenario-coverage-v82-migration-ux-studio.md");
    for required in [
        "Scenario Coverage v82",
        "one-way",
        "Source-project/open-text only",
        "A lossy import or behavior-bearing unit cannot be graded clean",
        "no row can be claimed `ported`",
        "Determinism is visible",
        "Studio does not write artifacts",
        "#1 and #23 remain open",
        "cargo test --workspace --jobs 2",
        "mix test",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
