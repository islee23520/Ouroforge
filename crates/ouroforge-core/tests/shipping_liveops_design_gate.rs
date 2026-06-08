//! Design-gate regression for Shipping and LiveOps Layer-3 Re-evaluation v1 (#1697).
//!
//! The gate is documentation-only: all shipping/liveops capabilities remain
//! DEFER absent a #1508 Layer-3 GO, and no implementation authority is granted.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn gate_doc() -> String {
    std::fs::read_to_string(repo_root().join("docs/shipping-liveops-v1.md"))
        .expect("shipping/liveops design gate exists")
}

#[test]
fn design_gate_records_per_capability_defer_decisions() {
    let doc = gate_doc();
    for required in [
        "Issue: **#1697** (#1 Era H Milestone 45, paired with #1508 Layer-3 gate)",
        "Status: **ADR complete — DEFER native/store export, real-player telemetry,\nlive balancing, and update/patch pipelines.**",
        "Native/store export | **DEFER** | Requires a future #1508 Layer-3 shipping GO.",
        "Real-player telemetry | **DEFER** | Requires a future #1508 Layer-3 telemetry/data GO.",
        "Live balancing | **DEFER** | Requires a future #1508 Layer-3 live-ops GO.",
        "Update/patch pipeline | **DEFER** | Requires a future #1508 Layer-3 update/patch GO.",
        "**DEFER stands for all four capabilities.**",
    ] {
        assert!(doc.contains(required), "missing decision text: {required}");
    }
}

#[test]
fn design_gate_ties_decision_to_era_f_h_evidence_without_implementation() {
    let doc = gate_doc();
    for required in [
        "This is a design-gate ADR. It adds **no shipping/liveops implementation code**",
        "Era F — game-class and evidence expansion",
        "Era G — generation and content systems",
        "Era H Milestone 42 — Multi-Agent Production Pipeline v1",
        "Era H Milestone 43 — Autonomous Producer and Whole-Game Orchestration v1",
        "Era H Milestone 44 — Scaled Trust Gradient, Release Provenance and\n  Compliance v1",
        "The evidence supports safer local release-candidate preparation and audit. It\ndoes **not** prove a need for Layer-3 shipping/liveops surfaces.",
        "Every future follow-up must reuse existing surfaces before adding anything new",
    ] {
        assert!(doc.contains(required), "missing evidence/reuse text: {required}");
    }
}

#[test]
fn design_gate_preserves_governance_and_forbidden_boundaries() {
    let doc = gate_doc();
    for required in [
        "#1 remains open.",
        "#23 remains open.",
        "Browser, Studio, dashboard, and cockpit surfaces\nremain **read-only** for trusted state.",
        "High-risk and source-affecting changes never auto-apply.",
        "No release proceeds without compliance plus human go/no-go.",
        "No autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted\n  writes",
        "No production-ready engine, commercial-readiness, Godot replacement/parity, or\n  autonomous shipping claim is introduced.",
        "Distributed/Elixir remains NO-GO per ADR #92",
        "autonomy ends at a local web release candidate with synthetic and fixture-scoped\n  evidence",
    ] {
        assert!(doc.contains(required), "missing boundary text: {required}");
    }

    for forbidden_positive_claim in [
        "native/store export is GO",
        "real-player telemetry is GO",
        "live balancing is GO",
        "update/patch pipeline is GO",
        "browser/Studio trusted writes are authorized",
        "auto-merge is authorized",
        "human go/no-go can be bypassed",
        "production-ready claim is introduced",
        "Godot replacement claim is introduced",
    ] {
        assert!(
            !doc.contains(forbidden_positive_claim),
            "forbidden positive claim leaked: {forbidden_positive_claim}"
        );
    }
}
