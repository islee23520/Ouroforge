//! Optional human channel contract tests for #2042 / Era L M72.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    OptionalHumanChannelContract, OPTIONAL_HUMAN_CHANNEL_CONTRACT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn fixture() -> OptionalHumanChannelContract {
    OptionalHumanChannelContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/contract.fixture.json",
    ))
    .expect("optional human channel fixture validates")
}

#[test]
fn fixture_defines_optional_non_blocking_read_only_surfaces() {
    let contract = fixture();
    assert_eq!(
        contract.schema_version,
        OPTIONAL_HUMAN_CHANNEL_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(contract.title_id, "era-i-engine-builder-deckbuilder");
    assert!(!contract.autonomous_loop_blocks_on_channel);
    for surface in [
        &contract.oversight_surface,
        &contract.escape_hatch,
        &contract.taste_feedback,
    ] {
        assert!(surface.read_only);
        assert!(surface.optional);
        assert!(!surface.blocks_autonomous_loop);
        let forbidden = surface.forbidden_actions.join("\n");
        for required in [
            "trusted_write",
            "source_apply",
            "auto_apply",
            "block_loop",
            "new_verifier",
            "new_data_plane",
        ] {
            assert!(forbidden.contains(required));
        }
    }
}

#[test]
fn fixture_reuses_existing_pipeline_and_m57_m58_provenance() {
    let contract = fixture();
    let refs = contract
        .required_pipeline_refs
        .join("\n")
        .to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(refs.contains(required), "missing {required}");
    }
    let reuse = contract
        .reuse_milestone_refs
        .join("\n")
        .to_ascii_lowercase();
    for required in ["m57", "m58", "playtest", "fun", "taste", "provenance"] {
        assert!(reuse.contains(required), "missing reuse {required}");
    }
}

#[test]
fn blocking_or_new_verifier_drift_fails_closed() {
    let mut contract = fixture();
    contract.oversight_surface.blocks_autonomous_loop = true;
    let err = contract.validate().expect_err("blocking surface rejected");
    assert!(err.to_string().contains("must not block"));

    let mut contract = fixture();
    contract.no_new_verification_engine = false;
    let err = contract
        .validate()
        .expect_err("new verifier drift rejected");
    assert!(err.to_string().contains("verifier"));

    let mut contract = fixture();
    contract.taste_feedback.read_only = false;
    let err = contract.validate().expect_err("write surface rejected");
    assert!(err.to_string().contains("read-only"));
}

#[test]
fn docs_record_optional_channel_boundaries() {
    let doc = read_text("docs/optional-human-channel-contract-v1.md").to_ascii_lowercase();
    for required in [
        "optional",
        "non-blocking",
        "read-only",
        "stage health",
        "blockers",
        "diagnosis",
        "attribution",
        "escape-hatch",
        "taste/fun-feedback",
        "m57",
        "m58",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "fun/taste verdicts and release go/no-go remain human",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
