//! Era P 2.5D migration on-ramp governance refresh tests (#2203).

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
fn roadmap_records_era_p_completion_on_merged_evidence() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "Era P — 2.5D Migration On-Ramp",
        "complete on merged evidence",
        "Milestones 96-99 plus the M100 governance refresh",
        "#2191",
        "#2192-#2195 / Scenario Coverage v83",
        "#2196-#2199 / Scenario Coverage v84",
        "#2200-#2202 / Scenario Coverage v85",
        "#2203",
        "docs/era-p-2-5d-migration-on-ramp-governance-refresh.md",
    ] {
        assert!(
            roadmap.contains(required),
            "missing Era P evidence {required}"
        );
    }
}

#[test]
fn era_p_governance_reaffirms_on_ramp_boundaries() {
    let combined = format!(
        "{}\n{}",
        read("docs/roadmap.md"),
        read("docs/era-p-2-5d-migration-on-ramp-governance-refresh.md")
    );
    for required in [
        "one-way source-project/open-text presentation import",
        "no finished-game auto-port",
        "no live engine bridge",
        "no decompiled-code copying",
        "clean-room re-derivation tasks",
        "Deterministic state-hash evidence stays primary",
        "perceptual render comparison (SSIM/pixel-diff) stays secondary",
        "Studio owns no artifact semantics or trusted writes",
        "no new data store is authorized",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(combined.contains(required), "missing boundary {required}");
    }
}

#[test]
fn era_p_governance_keeps_forbidden_claims_out() {
    let roadmap = read("docs/roadmap.md");
    let start = roadmap
        .find("## Era P 2.5D migration on-ramp governance")
        .expect("Era P section exists");
    let end = roadmap[start..]
        .find("## Era R semantic re-derivation governance")
        .map(|offset| start + offset)
        .unwrap_or(roadmap.len());
    let section = &roadmap[start..end];
    for forbidden in [
        "finished-game auto-port is authorized",
        "live bridge is authorized",
        "embedded foreign runtime is authorized",
        "decompiled source is allowed",
        "trusted Studio writes are authorized",
        "a new data store is authorized by Era P",
        "perceptual render evidence is primary",
        "fun/feel is automated",
        "release go/no-go is automated",
        "Full 3D is GO",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden wording leaked {forbidden}"
        );
    }
}

#[test]
fn governance_anchor_live_check_is_opt_in() {
    // Hermetic by default: live GitHub anchor checks are opt-in so `cargo test`
    // does not depend on network / gh-auth / external issue state. Set
    // OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK=1 to verify issues #1/#23 are OPEN.
    if std::env::var("OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK").is_err() {
        return;
    }
    let root = repo_root();
    for issue in ["1", "23"] {
        let output = std::process::Command::new("gh")
            .args([
                "issue",
                "view",
                issue,
                "--repo",
                "shaun0927/Ouroforge",
                "--json",
                "state",
                "--jq",
                ".state",
            ])
            .current_dir(&root)
            .output()
            .unwrap_or_else(|error| panic!("gh issue {issue}: {error}"));
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "OPEN");
    }
}
