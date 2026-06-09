//! Era Q M101 full-3D re-evaluation gate tests (#2204).

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn adr_records_defer_decision_and_go_gated_followups() {
    let adr = read("docs/full-3d-on-ramp-reevaluation-gate-v1.md");
    for required in [
        "**DEFER.**",
        "M102-M106 implementation work",
        "Per-capability GO/DEFER record",
        "glTF 3D scene import and normalization (M102)",
        "Deterministic 3D physics re-simulation (M103)",
        "Two-tier 3D evidence model (M104)",
        "3D logic re-derivation hand-off and demo (M105)",
        "Era Q outcome governance (M106)",
        "Do not create implementation work",
    ] {
        assert!(adr.contains(required), "missing DEFER evidence {required}");
    }
}

#[test]
fn adr_preserves_on_ramp_clean_room_and_two_plane_boundaries() {
    let combined = format!(
        "{}\n{}\n{}",
        read("docs/full-3d-on-ramp-reevaluation-gate-v1.md"),
        read("docs/roadmap.md"),
        read("docs/roadmap/active/era-q.md")
    );
    for required in [
        "source-project/open-text",
        "no shipped-build ripping",
        "no live bridge",
        "embedded Unity/Unreal/Godot runtime",
        "Clean-room re-derivation",
        "no unit is called `ported`",
        "deterministic state-hash evidence is primary",
        "perceptual render comparison (SSIM/pixel-diff",
        "secondary only",
        "re-simulated, never reproduced",
        "Rust owns artifact semantics",
        "Elixir/Phoenix Studio remains local control + presentation only",
        "No new trusted write path or data store",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(combined.contains(required), "missing boundary {required}");
    }
}

#[test]
fn roadmap_records_era_q_defer_without_authorizing_full_3d() {
    let roadmap = read("docs/roadmap.md");
    let progress = read("docs/roadmap/progress.json");
    for required in [
        "Era Q — Full-3D On-Ramp Re-evaluation",
        "M101 decision as **DEFER on merged evidence**",
        "docs/full-3d-on-ramp-reevaluation-gate-v1.md",
        "M102-M106 remain GO-gated",
        "\"status\": \"deferred\"",
        "M101 DEFER recorded; M102-M106 remain GO-gated",
    ] {
        let haystack = format!("{roadmap}\n{progress}");
        assert!(
            haystack.contains(required),
            "missing roadmap/progress record {required}"
        );
    }
}

#[test]
fn gate_keeps_forbidden_authority_out() {
    let docs = format!(
        "{}\n{}",
        read("docs/full-3d-on-ramp-reevaluation-gate-v1.md"),
        read("docs/roadmap.md")
    );
    for forbidden in [
        "Full 3D is GO",
        "M102 is authorized for implementation",
        "M103 is authorized for implementation",
        "perceptual render evidence is primary",
        "foreign runtime physics is reproduced",
        "live bridge is authorized",
        "embedded foreign runtime is authorized",
        "finished-game auto-port is authorized",
        "trusted Studio writes are authorized",
        "a new data store is authorized by Era Q",
        "decompiled code copying is allowed",
    ] {
        assert!(
            !docs.contains(forbidden),
            "forbidden wording leaked {forbidden}"
        );
    }
}
