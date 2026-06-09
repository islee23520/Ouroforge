const COVERAGE_DOC: &str =
    include_str!("../../../docs/scenario-coverage-v76-studio-local-delivery.md");
const CONTRACT_DOC: &str =
    include_str!("../../../docs/studio-packaging-local-delivery-contract-v1.md");
const INSTALL_DOC: &str = include_str!("../../../docs/studio-local-packaging-install-ux-v1.md");
const DEMO_DOC: &str = include_str!("../../../docs/studio-local-delivery-demo-v1.md");
const SMOKE_SCRIPT: &str = include_str!("../../../scripts/studio-local-package-smoke.sh");
const DELIVERY_MODULE: &str =
    include_str!("../../../studio/executor/lib/ouroforge_executor/studio_local_delivery.ex");
const DEMO_MODULE: &str =
    include_str!("../../../studio/executor/lib/ouroforge_executor/studio_local_delivery_demo.ex");
const V76_ELIXIR_TEST: &str =
    include_str!("../../../studio/executor/test/ouroforge_executor/scenario_coverage_v76_test.exs");

#[test]
fn coverage_v76_doc_records_local_delivery_boundaries() {
    for needle in [
        "Scenario Coverage v76",
        "local-first and single-user only",
        "autonomous CLI loop completes without Studio and without human input",
        "read + gated-write",
        "intervention-as-evidence",
        "Rust remains the data plane",
        "Elixir/OTP + Phoenix LiveView is control + presentation only",
        "generated evidence only",
        "hosted",
        "release-channel",
        "raw-bypass",
        "#1 and #23 remain open",
    ] {
        assert!(
            COVERAGE_DOC.contains(needle),
            "missing coverage boundary: {needle}"
        );
    }
}

#[test]
fn local_install_and_demo_docs_preserve_cli_fallback_and_hosted_defer() {
    for doc in [CONTRACT_DOC, INSTALL_DOC, DEMO_DOC] {
        assert!(doc.contains("CLI fallback") || doc.contains("CLI commands"));
        assert!(doc.contains("read + gated-write"));
        assert!(doc.contains("Rust"));
        assert!(doc.contains("Elixir") || doc.contains("Phoenix"));
        assert!(doc.contains("hosted") || doc.contains("Hosted"));
        assert!(doc.contains("#1 and #23 remain open"));
    }

    assert!(INSTALL_DOC.contains("cargo build --workspace --jobs 2"));
    assert!(INSTALL_DOC.contains("mix deps.get"));
    assert!(INSTALL_DOC.contains("mix run --no-halt"));
    assert!(DEMO_DOC.contains("human packaging constraint"));
    assert!(DEMO_DOC
        .contains("Route the constraint to the existing Rust human-constraint validation gate"));
}

#[test]
fn smoke_script_records_generated_evidence_without_write_authority() {
    for needle in [
        "STUDIO_LOCAL_PACKAGE_SMOKE_OK",
        "runs/studio-local-package-smoke-v1",
        "\"readGatedWrite\": true",
        "\"interventionAsEvidence\": true",
        "\"rustDataPlaneOwnsTruth\": true",
        "\"elixirControlPresentationOnly\": true",
        "\"trustedWriteAuthority\": false",
        "\"directArtifactWrite\": false",
        "\"commandBridge\": false",
        "\"newDataStore\": false",
        "\"hostedCollaborative\": false",
        "\"cliFallbackSupported\": true",
        "\"autonomousLoopRequiresHuman\": false",
        "generated smoke evidence only",
    ] {
        assert!(
            SMOKE_SCRIPT.contains(needle),
            "missing smoke assertion: {needle}"
        );
    }
}

#[test]
fn elixir_delivery_contract_fails_closed_on_scope_drift() {
    for needle in [
        "trustedWriteAuthority: false",
        "directArtifactWrite: false",
        "rawBypassRequested: false",
        "commandBridge: false",
        "newDataStore: false",
        "hostedCollaborative: false",
        "signingOrRelease: false",
        "deployOrPublish: false",
        "generatedSmokeOnly: true",
        ":raw_bypass_forbidden",
        ":autonomy_or_cli_fallback_broken",
        ":two_plane_boundary_broken",
        ":trusted_write_or_store_forbidden",
        ":release_or_delivery_scope_forbidden",
    ] {
        assert!(
            DELIVERY_MODULE.contains(needle),
            "missing fail-closed delivery assertion: {needle}"
        );
    }
}

#[test]
fn demo_and_elixir_v76_assert_gated_write_and_no_human_fallback() {
    for needle in [
        "gated_write_verified?",
        "autonomous_fallback_verified?",
        "queued_for_rust_gate",
        "human_surface_required?: false",
        "waited_for_human?: false",
        "cli_fallback_supported?",
        "trusted_write_authority?: false",
        "direct_artifact_write?: false",
        "command_bridge?: false",
        "hosted_collaborative?: false",
    ] {
        assert!(
            DEMO_MODULE.contains(needle),
            "missing demo invariant: {needle}"
        );
    }

    for needle in [
        "Scenario Coverage v76",
        "StudioLocalDelivery.manifest",
        "StudioLocalDeliveryDemo.read_gated_write?",
        "StudioLocalDeliveryDemo.autonomous_first?",
        "StudioLocalDeliveryDemo.smoke_verified?",
        "raw_bypass_requested: true",
        "autonomous_loop_requires_human: true",
        "hosted_collaborative: true",
        "signing_or_release: true",
        "deploy_or_publish: true",
    ] {
        assert!(
            V76_ELIXIR_TEST.contains(needle),
            "missing v76 Elixir test assertion: {needle}"
        );
    }
}
