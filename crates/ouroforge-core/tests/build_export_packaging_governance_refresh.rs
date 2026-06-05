//! Roadmap/governance refresh after Build / Export / Packaging v1 (#736).

const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const BUILD_EXPORT: &str = include_str!("../../../docs/build-export-packaging-v1.md");

#[test]
fn roadmap_records_completed_build_export_packaging_without_overclaiming() {
    for completed in [
        "Build / Export / Packaging v1",
        "export target matrix",
        "export profile",
        "dry-run export plan",
        "deterministic staging",
        "local web bundle",
        "asset manifest",
        "runtime probe preservation",
        "checksums/provenance",
        "verification evidence",
        "Scenario Coverage v15",
        "read-only inspection",
        "release/publish blockers",
    ] {
        assert!(
            ROADMAP.contains(completed),
            "roadmap missing completed capability: {completed}"
        );
    }

    for remaining in [
        "Plugin / Extension System",
        "Full Studio Editor",
        "native/desktop/mobile/store export",
        "signing",
        "Godot-plus demonstration game",
    ] {
        assert!(
            BUILD_EXPORT.contains(remaining) || ROADMAP.contains(remaining),
            "remaining capability not separated: {remaining}"
        );
    }
}

#[test]
fn governance_refresh_keeps_forbidden_release_and_godot_claims_blocked() {
    let combined = format!("{ROADMAP}\n{BUILD_EXPORT}");
    for forbidden_boundary in [
        "public release",
        "deployment",
        "signing",
        "upload",
        "app-store/Steam/itch publishing",
        "credentialed release flow",
        "production-ready export",
        "commercial distribution",
        "multi-platform parity",
        "Godot replacement",
    ] {
        assert!(
            combined.contains(forbidden_boundary),
            "governance refresh must keep boundary visible: {forbidden_boundary}"
        );
    }
    assert!(combined.contains("#1 remains open"));
    assert!(combined.contains("#23 remains open"));
}
