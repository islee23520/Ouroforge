// Consolidated from 15 individual gdd_*_contract.rs files.
// Each original file is now a `mod` block to scope per-file helpers (fixture/read_fixture)
// without collisions.  Shared `use` imports are deduplicated at the top where possible.

// ---------------------------------------------------------------------------
// gdd_asset_placeholder_plan_contract
// ---------------------------------------------------------------------------
mod asset_placeholder_plan {
    use ouroforge_core::gdd_asset_placeholder_plan::GddAssetPlaceholderPlanArtifact;
    use std::{fs, path::PathBuf};
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-asset-placeholder-plan-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }
    #[test]
    fn asset_plan_fixtures_validate_and_export_read_models() {
        for name in [
            "asset-plan.valid.fixture.json",
            "asset-plan.missing.fixture.json",
            "asset-plan.stale.fixture.json",
            "asset-plan.unsupported.fixture.json",
        ] {
            let artifact = GddAssetPlaceholderPlanArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.asset_entry_count, artifact.asset_entries.len());
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("license/source")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("no asset generation")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("remote fetch enabled"));
        }
    }
    #[test]
    fn invalid_asset_plan_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/asset-plan.missing-license.fixture.json",
                "license/source notes",
            ),
            ("invalid/asset-plan.remote-ref.fixture.json", "remote refs"),
            (
                "invalid/asset-plan.generated-root.fixture.json",
                "generated-root",
            ),
            (
                "invalid/asset-plan.unsupported-type.fixture.json",
                "unsupported asset type",
            ),
            (
                "invalid/asset-plan.stale-manifest-ref.fixture.json",
                "missing from declared manifestRefs",
            ),
            (
                "invalid/asset-plan.proprietary-ambiguous.fixture.json",
                "proprietary/copyright ambiguity",
            ),
            (
                "invalid/asset-plan.unsafe-path.fixture.json",
                "forbidden traversal",
            ),
            (
                "invalid/asset-plan.stale-no-blocker.fixture.json",
                "stale manifest refs",
            ),
            (
                "invalid/asset-plan.manifest-generated-root.fixture.json",
                "generated-root or evidence output",
            ),
            (
                "invalid/asset-plan.boundary-negation-bypass.fixture.json",
                "forbidden GDD asset authority text",
            ),
        ] {
            let error = GddAssetPlaceholderPlanArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }
    #[test]
    fn asset_plan_docs_keep_governance_and_wording_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-asset-placeholder-plan-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #652"));
        assert!(docs.contains("non-mutating"));
        assert!(docs.contains("placeholder assets or known local refs"));
        assert!(docs.contains("No asset generation"));
        assert!(docs.contains("license/source"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "remote fetch enabled",
            "asset generation enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_design_brief_contract
// ---------------------------------------------------------------------------
mod design_brief {
    use ouroforge_core::gdd_design_brief::{
        GddDesignBriefArtifact, GddDesignBriefStatus, GddTargetGameClass,
        GDD_DESIGN_BRIEF_SCHEMA_VERSION,
    };
    fn valid_fixture() -> &'static str {
        include_str!("../../../examples/gdd-design-brief-v1/design-brief.valid.fixture.json")
    }
    #[test]
    fn gdd_design_brief_accepts_valid_partial_and_blocked_fixtures() {
        let ready =
            GddDesignBriefArtifact::from_json_str(valid_fixture()).expect("ready brief parses");
        assert_eq!(ready.schema_version, GDD_DESIGN_BRIEF_SCHEMA_VERSION);
        assert_eq!(ready.status, GddDesignBriefStatus::Ready);
        assert_eq!(
            ready.target_game_class,
            GddTargetGameClass::Small2dPrototype
        );
        assert_eq!(ready.core_loop.steps.len(), 5);
        let partial = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/design-brief.partial.fixture.json"
        ))
        .expect("partial brief parses");
        assert_eq!(partial.status, GddDesignBriefStatus::Partial);
        let blocked = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/design-brief.blocked.fixture.json"
        ))
        .expect("blocked brief parses");
        assert_eq!(blocked.status, GddDesignBriefStatus::Blocked);
        assert!(!blocked.blocked_reasons.is_empty());
    }
    #[test]
    fn gdd_design_brief_rejects_unsafe_refs_and_unknown_fields() {
        let unsafe_ref = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/invalid/design-brief.unsafe-ref.fixture.json"
        ))
        .expect_err("remote asset/style refs are rejected");
        assert!(
            unsafe_ref
                .to_string()
                .contains("forbidden GDD/prototype authority text")
                || unsafe_ref
                    .to_string()
                    .contains("local fixture/reference roots")
        );
        let mut value: serde_json::Value =
            serde_json::from_str(valid_fixture()).expect("fixture json");
        value["generationPrompt"] = serde_json::json!("make a full game");
        let unknown = GddDesignBriefArtifact::from_json_str(&value.to_string())
            .expect_err("generation authority fields are rejected");
        assert!(unknown
            .to_string()
            .contains("failed to parse GDD Design Brief JSON"));
    }
    #[test]
    fn gdd_design_brief_rejects_validation_drift_for_ready_briefs() {
        for (fixture, expected) in [
            (
                include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.overbroad-scope.fixture.json"),
                "overbroad or out-of-scope",
            ),
            (
                include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.contradictory.fixture.json"),
                "contradictory requirements",
            ),
            (
                include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.unclear-win-loss.fixture.json"),
                "must be concrete for ready design briefs",
            ),
            (
                include_str!("../../../examples/gdd-design-brief-v1/invalid/design-brief.unsupported-asset-kind.fixture.json"),
                "is not supported in v1",
            ),
        ] {
            let error = GddDesignBriefArtifact::from_json_str(fixture).expect_err(expected);
            assert!(error.to_string().contains(expected), "{error:?}");
        }
    }

    #[test]
    fn gdd_design_brief_rejects_missing_core_loop_acceptance_and_target_class_drift() {
        let base: serde_json::Value = serde_json::from_str(valid_fixture()).expect("fixture json");
        let cases = [
            (
                {
                    let mut value = base.clone();
                    value["coreLoop"]["steps"] = serde_json::json!([]);
                    value
                },
                "coreLoop.steps must not be empty",
            ),
            (
                {
                    let mut value = base.clone();
                    value["coreLoop"]["steps"] = serde_json::json!(["move"]);
                    value
                },
                "at least two concrete steps",
            ),
            (
                {
                    let mut value = base.clone();
                    value["targetGameClass"] = serde_json::json!("autonomous-full-game");
                    value
                },
                "failed to parse GDD Design Brief JSON",
            ),
            (
                {
                    let mut value = base.clone();
                    value["acceptanceGoals"] = serde_json::json!([]);
                    value
                },
                "acceptanceGoals must not be empty",
            ),
            (
                {
                    let mut value = base.clone();
                    value["assetStyleRefs"][0]["license"] = serde_json::json!("unknown");
                    value
                },
                "license must be explicit",
            ),
        ];

        for (value, expected) in cases {
            let error =
                GddDesignBriefArtifact::from_json_str(&value.to_string()).expect_err(expected);
            assert!(error.to_string().contains(expected), "{error:?}");
        }
    }

    #[test]
    fn gdd_design_brief_read_model_preserves_display_compatibility() {
        let ready =
            GddDesignBriefArtifact::from_json_str(valid_fixture()).expect("ready brief parses");
        let read_model = ready.read_model();
        assert_eq!(read_model.schema_version, GDD_DESIGN_BRIEF_SCHEMA_VERSION);
        assert_eq!(read_model.brief_id, "collect-and-exit-brief");
        assert_eq!(read_model.status, "ready");
        assert_eq!(read_model.target_game_class, "small2d-prototype");
        assert_eq!(read_model.core_loop_step_count, 5);
        assert_eq!(read_model.mechanic_count, 2);
        assert_eq!(read_model.asset_style_ref_count, 1);
        assert_eq!(read_model.acceptance_goal_count, 1);
        assert_eq!(read_model.blocked_reason_count, 0);
        assert!(read_model
            .validation_summary
            .iter()
            .any(|item| item.contains("ready brief has concrete core loop")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|item| item.contains("display-only read model")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|item| item.contains("no prototype generation")));

        let json = ready.read_model_json().expect("read model serializes");
        let value: serde_json::Value = serde_json::from_str(&json).expect("read model json parses");
        assert_eq!(value["briefId"], "collect-and-exit-brief");
        assert_eq!(value["coreLoopStepCount"], 5);
        assert_eq!(value["compatibilityNotes"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn gdd_design_brief_read_model_keeps_partial_blocked_and_malformed_states_visible() {
        let partial = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/design-brief.partial.fixture.json"
        ))
        .expect("partial brief parses");
        let partial_model = partial.read_model();
        assert_eq!(partial_model.status, "partial");
        assert_eq!(partial_model.blocked_reason_count, 0);
        assert!(partial_model
            .compatibility_notes
            .iter()
            .any(|item| item.contains("validated summary counts")));

        let blocked = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/design-brief.blocked.fixture.json"
        ))
        .expect("blocked brief parses");
        let blocked_model = blocked.read_model();
        assert_eq!(blocked_model.status, "blocked");
        assert_eq!(blocked_model.blocked_reason_count, 1);
        assert!(blocked_model
            .validation_summary
            .iter()
            .any(|item| item.contains("blocked reason(s) remain visible")));

        let malformed = GddDesignBriefArtifact::from_json_str(include_str!(
            "../../../examples/gdd-design-brief-v1/invalid/design-brief.unclear-win-loss.fixture.json"
        ))
        .expect_err("malformed/unclear ready briefs do not get display read models");
        assert!(malformed
            .to_string()
            .contains("must be concrete for ready design briefs"));
    }

    #[test]
    fn gdd_design_brief_requires_blocked_reasons_top_level_field() {
        // The published gdd-design-brief-v1 contract lists blockedReasons as a required
        // top-level field, so a brief that omits it must fail closed rather than silently
        // defaulting to an empty list.
        let mut value: serde_json::Value =
            serde_json::from_str(valid_fixture()).expect("fixture json");
        value
            .as_object_mut()
            .expect("brief object")
            .remove("blockedReasons");
        let missing = GddDesignBriefArtifact::from_json_str(&value.to_string())
            .expect_err("missing blockedReasons is rejected");
        assert!(missing
            .to_string()
            .contains("failed to parse GDD Design Brief JSON"));
    }
    #[test]
    fn gdd_design_brief_docs_audit_generation_boundary() {
        let doc = include_str!("../../../docs/gdd-design-brief-v1.md");
        assert!(doc.contains("Issue: #645"));
        assert!(doc.contains("input validation, not generation authority"));
        assert!(doc.contains("GDD-derived output remains untrusted"));
        assert!(doc.contains("no autonomous unrestricted game creation"));
        assert!(doc.contains("#1 remains"));
        assert!(doc.contains("#23 remains"));
    }
}

// ---------------------------------------------------------------------------
// gdd_feasibility_gate_contract
// ---------------------------------------------------------------------------
mod feasibility_gate {
    use ouroforge_core::gdd_feasibility_gate::GddFeasibilityGateArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-feasibility-gate-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn feasibility_gate_state_fixtures_validate_and_export_read_models() {
        for name in [
            "feasibility.feasible.fixture.json",
            "feasibility.infeasible.fixture.json",
            "feasibility.deferred.fixture.json",
            "feasibility.downgraded.fixture.json",
            "feasibility.overbroad.fixture.json",
            "feasibility.blocked.fixture.json",
        ] {
            let artifact = GddFeasibilityGateArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(
                read_model.supported_mechanic_count,
                artifact.supported_mechanics.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("feasibility")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("read-only")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("auto-apply enabled"));
        }
    }

    #[test]
    fn invalid_feasibility_gate_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/feasibility.missing-mapping.fixture.json",
                "must not be empty",
            ),
            (
                "invalid/feasibility.unsupported-without-risk.fixture.json",
                "missing supported mechanics",
            ),
            (
                "invalid/feasibility.overlarge-without-risk.fixture.json",
                "overlarge scope",
            ),
            (
                "invalid/feasibility.missing-acceptance.fixture.json",
                "missing acceptance criteria",
            ),
            (
                "invalid/feasibility.missing-scenario.fixture.json",
                "missing scenario plan",
            ),
            (
                "invalid/feasibility.unsatisfied-prereq.fixture.json",
                "blockedReason",
            ),
            (
                "invalid/feasibility.defer-no-slice.fixture.json",
                "sliceRecommendation",
            ),
            (
                "invalid/feasibility.unsafe-boundary.fixture.json",
                "forbidden",
            ),
        ] {
            let error = GddFeasibilityGateArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn feasibility_gate_docs_keep_generation_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-feasibility-gate-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #648"));
        assert!(docs.contains("Prototype planning starts only after feasibility passes"));
        assert!(docs.contains("pass/fail/defer"));
        assert!(docs.contains("bounded slice"));
        assert!(docs.contains("not a prototype generator"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_gameplay_behavior_plan_contract
// ---------------------------------------------------------------------------
mod gameplay_behavior_plan {
    use ouroforge_core::gdd_gameplay_behavior_plan::GddGameplayBehaviorPlanArtifact;
    use std::{fs, path::PathBuf};
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-gameplay-behavior-plan-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }
    #[test]
    fn behavior_plan_fixtures_validate_and_export_read_models() {
        for name in [
            "behavior-plan.valid.fixture.json",
            "behavior-plan.unsupported.fixture.json",
            "behavior-plan.script-needed.fixture.json",
            "behavior-plan.partial.fixture.json",
            "behavior-plan.blocked.fixture.json",
            "behavior-plan.stale.fixture.json",
        ] {
            let artifact = GddGameplayBehaviorPlanArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(
                read_model.behavior_model_count,
                artifact.behavior_models.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("requirement")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("non-mutating")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("arbitrary script generation enabled"));
        }
    }
    #[test]
    fn invalid_behavior_plan_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/behavior-plan.missing-requirement.fixture.json",
                "missing from declared GDD requirements",
            ),
            (
                "invalid/behavior-plan.unsupported-no-blocker.fixture.json",
                "blockedReasons",
            ),
            (
                "invalid/behavior-plan.unsafe-ref.fixture.json",
                "forbidden traversal",
            ),
            (
                "invalid/behavior-plan.contradictory.fixture.json",
                "contradictory core loop behavior",
            ),
            (
                "invalid/behavior-plan.missing-proof.fixture.json",
                "proof expectation",
            ),
            (
                "invalid/behavior-plan.stale-no-blocker.fixture.json",
                "stale ref",
            ),
            (
                "invalid/behavior-plan.script-need-no-blocker.fixture.json",
                "blockedReasons",
            ),
        ] {
            let error = GddGameplayBehaviorPlanArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }
    #[test]
    fn behavior_plan_docs_keep_governance_and_script_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-gameplay-behavior-plan-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #651"));
        assert!(docs.contains("non-mutating"));
        assert!(docs.contains("gameplay-behavior-model-v1"));
        assert!(docs.contains("No arbitrary script generation"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "arbitrary script generation enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "production-ready claim enabled",
            "browser trusted write enabled",
            "autonomous unrestricted game creation enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_mechanics_mapping_contract
// ---------------------------------------------------------------------------
mod mechanics_mapping {
    use ouroforge_core::gdd_mechanics_mapping::GddMechanicsMappingArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-mechanics-mapping-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn valid_mechanics_mapping_fixtures_validate_and_export_read_models() {
        for name in [
            "mechanics.supported.fixture.json",
            "mechanics.unsupported.fixture.json",
            "mechanics.partial.fixture.json",
            "mechanics.contradictory.fixture.json",
            "mechanics.deferred.fixture.json",
        ] {
            let artifact = GddMechanicsMappingArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.mapping_count, artifact.mappings.len());
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("display-only")));
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("requirement")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("auto-apply enabled"));
        }
    }

    #[test]
    fn invalid_mechanics_mapping_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/mechanics.supported-missing-behavior.fixture.json",
                "supported behaviorModelRefs",
            ),
            (
                "invalid/mechanics.unsupported-no-recommendation.fixture.json",
                "recommendations must not be empty",
            ),
            (
                "invalid/mechanics.contradictory-no-blocker.fixture.json",
                "contradictory mapping",
            ),
            (
                "invalid/mechanics.overbroad-core-loop.fixture.json",
                "overbroad",
            ),
            (
                "invalid/mechanics.unsafe-boundary.fixture.json",
                "forbidden",
            ),
            (
                "invalid/mechanics.unknown-capability.fixture.json",
                "unknown capability",
            ),
        ] {
            let error = GddMechanicsMappingArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn mechanics_mapping_docs_keep_boundaries_and_governance() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-mechanics-mapping-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #647"));
        assert!(docs.contains("requirement ids"));
        assert!(docs.contains("unsupported mechanics"));
        assert!(docs.contains("not a prototype generator"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_project_scaffold_plan_contract
// ---------------------------------------------------------------------------
mod project_scaffold_plan {
    use ouroforge_core::gdd_project_scaffold_plan::GddProjectScaffoldPlanArtifact;
    use std::{fs, path::PathBuf};
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-project-scaffold-plan-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }
    #[test]
    fn scaffold_plan_fixtures_validate_and_export_read_models() {
        for name in [
            "scaffold.valid.fixture.json",
            "scaffold.stale.fixture.json",
            "scaffold.blocked.fixture.json",
            "scaffold.deferred.fixture.json",
        ] {
            let artifact = GddProjectScaffoldPlanArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.planned_file_count, artifact.files.len());
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("preview-only")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("display-only")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("direct trusted writes enabled"));
        }
    }
    #[test]
    fn invalid_scaffold_plan_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/scaffold.unsafe-path.fixture.json",
                "fixture/reference",
            ),
            (
                "invalid/scaffold.generated-root-collision.fixture.json",
                "collides",
            ),
            ("invalid/scaffold.duplicate-file.fixture.json", "duplicated"),
            (
                "invalid/scaffold.unsupported-template-no-blocker.fixture.json",
                "blockedReasons",
            ),
            (
                "invalid/scaffold.missing-feasibility-pass.fixture.json",
                "feasibilityState pass",
            ),
            (
                "invalid/scaffold.stale-no-blocker.fixture.json",
                "stale target",
            ),
            ("invalid/scaffold.overbroad.fixture.json", "overbroad"),
            (
                "invalid/scaffold.direct-write-command.fixture.json",
                "preview-only",
            ),
        ] {
            let error = GddProjectScaffoldPlanArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }
    #[test]
    fn scaffold_plan_docs_keep_preview_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-project-scaffold-plan-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #649"));
        assert!(docs.contains("Preview first"));
        assert!(docs.contains("no direct trusted writes"));
        assert!(docs.contains("not a prototype generator"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_prototype_apply_contract
// ---------------------------------------------------------------------------
mod prototype_apply {
    use ouroforge_core::gdd_prototype_apply::GddPrototypeApplyArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-prototype-apply-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn prototype_apply_fixtures_validate_and_export_read_models() {
        for name in [
            "apply.valid.fixture.json",
            "apply.missing-review.fixture.json",
            "apply.stale.fixture.json",
        ] {
            let artifact = GddPrototypeApplyArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.transaction_count, artifact.transactions.len());
            assert_eq!(
                read_model.rollback_target_count,
                artifact.rollback_metadata.targets.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("accepted independent review")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("untrusted until Rust/local validation")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("auto-apply enabled"));
        }
    }

    #[test]
    fn invalid_prototype_apply_fixtures_fail_closed() {
        for (name, expected) in [
            ("invalid/apply.self-approval.fixture.json", "self-approval"),
            (
                "invalid/apply.missing-review-ready.fixture.json",
                "accepted review",
            ),
            ("invalid/apply.auto-apply.fixture.json", "autoApply"),
            (
                "invalid/apply.source-like-target.fixture.json",
                "source-like fixture policy",
            ),
            (
                "invalid/apply.nested-manifest-target.fixture.json",
                "source-like fixture policy",
            ),
            (
                "invalid/apply.nested-source-target.fixture.json",
                "source-like fixture policy",
            ),
            (
                "invalid/apply.generated-output-collision.fixture.json",
                "generated-output collision",
            ),
            (
                "invalid/apply.stale-no-blocker.fixture.json",
                "stale target",
            ),
            (
                "invalid/apply.missing-behavior-ref.fixture.json",
                "behavior transactions require behaviorRefs",
            ),
            (
                "invalid/apply.missing-scenario-ref.fixture.json",
                "scenario transactions require scenarioRefs",
            ),
            (
                "invalid/apply.rollback-mismatch.fixture.json",
                "rollback metadata must match",
            ),
            (
                "invalid/apply.missing-asset-source.fixture.json",
                "assetSourceRefs must not be empty",
            ),
            (
                "invalid/apply.hidden-command.fixture.json",
                "restricted to local cargo/node",
            ),
            ("invalid/apply.unsafe-boundary.fixture.json", "forbidden"),
        ] {
            let error = GddPrototypeApplyArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn prototype_apply_docs_keep_review_gate_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-prototype-apply-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #656"));
        assert!(docs.contains("accepted review"));
        assert!(docs.contains("rollback metadata"));
        assert!(docs.contains("rerun command context"));
        assert!(docs.contains("generated-state audit"));
        assert!(docs.contains("Rust/local validation owns trusted persistence"));
        assert!(docs.contains("#1 remains open"));
        assert!(docs.contains("#23 remains open"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "self-approval enabled",
            "browser trusted write enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_prototype_draft_bundle_contract
// ---------------------------------------------------------------------------
mod prototype_draft_bundle {
    use ouroforge_core::gdd_prototype_draft_bundle::GddPrototypeDraftBundleArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-prototype-draft-bundle-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn prototype_draft_bundle_fixtures_validate_and_export_read_models() {
        for name in [
            "bundle.valid.fixture.json",
            "bundle.incomplete.fixture.json",
            "bundle.stale.fixture.json",
            "bundle.unsupported.fixture.json",
            "bundle.blocked.fixture.json",
        ] {
            let artifact = GddPrototypeDraftBundleArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.component_count, artifact.components.len());
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("review surface")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("display-only")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("direct trusted writes enabled"));
        }
    }

    #[test]
    fn invalid_prototype_draft_bundle_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/bundle.unsafe-ref.fixture.json",
                "fixture/reference",
            ),
            (
                "invalid/bundle.missing-component.fixture.json",
                "missing required component",
            ),
            (
                "invalid/bundle.duplicate-component.fixture.json",
                "duplicated",
            ),
            (
                "invalid/bundle.missing-scenario-no-blocker.fixture.json",
                "blockedReasons",
            ),
            (
                "invalid/bundle.stale-target-no-blocker.fixture.json",
                "stale target",
            ),
            (
                "invalid/bundle.missing-source-note.fixture.json",
                "sourceNoteRefs must not be empty",
            ),
            ("invalid/bundle.malformed-hash.fixture.json", "sha256"),
            ("invalid/bundle.unsafe-boundary.fixture.json", "forbidden"),
            ("invalid/bundle.overbroad.fixture.json", "overbroad"),
        ] {
            let error = GddPrototypeDraftBundleArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn prototype_draft_bundle_docs_keep_review_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-prototype-draft-bundle-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #655"));
        assert!(docs.contains("review surface only"));
        assert!(docs.contains("No direct trusted writes"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_prototype_evidence_bundle_contract
// ---------------------------------------------------------------------------
mod prototype_evidence_bundle {
    use ouroforge_core::gdd_prototype_evidence_bundle::GddPrototypeEvidenceBundleArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-prototype-evidence-bundle-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn prototype_evidence_bundle_fixtures_validate_and_export_read_models() {
        for name in [
            "evidence.pass.fixture.json",
            "evidence.fail.fixture.json",
            "evidence.missing-run.fixture.json",
            "evidence.partial.fixture.json",
            "evidence.unsupported.fixture.json",
            "evidence.stale.fixture.json",
        ] {
            let artifact = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.scenario_count, artifact.scenario_verdicts.len());
            assert_eq!(
                read_model.requirement_count,
                artifact.requirement_coverage.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("GDD requirements")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("remain separate")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("trusted writes enabled"));
        }
    }

    #[test]
    fn invalid_prototype_evidence_bundle_fixtures_fail_closed() {
        for (name, expected) in [
            ("invalid/evidence.unsafe-ref.fixture.json", "forbidden"),
            (
                "invalid/evidence.malformed-missing-run.fixture.json",
                "requires blockedReasons",
            ),
            (
                "invalid/evidence.partial-no-blocker.fixture.json",
                "non-passing",
            ),
            (
                "invalid/evidence.unsupported-no-blocker.fixture.json",
                "requires blockedReasons",
            ),
            (
                "invalid/evidence.stale-no-blocker.fixture.json",
                "non-passing",
            ),
            (
                "invalid/evidence.missing-requirement-link.fixture.json",
                "missing from requirementCoverage",
            ),
            (
                "invalid/evidence.missing-scenario-link.fixture.json",
                "links missing scenarioId",
            ),
            (
                "invalid/evidence.fail-without-failure.fixture.json",
                "requires failed scenario",
            ),
            ("invalid/evidence.unsafe-boundary.fixture.json", "forbidden"),
        ] {
            let error = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn prototype_evidence_bundle_docs_keep_governance_and_wording_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-prototype-evidence-bundle-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #657"));
        assert!(docs.contains("evidence-gated prototype generation"));
        assert!(docs.contains("not autonomous unrestricted game creation"));
        assert!(docs.contains("Generated run/evidence output remains untracked"));
        assert!(docs.contains("#1 remains open"));
        assert!(docs.contains("#23 remains open"));
        for forbidden in [
            "trusted writes enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
            "autonomous unrestricted game creation enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_prototype_evidence_contract
// ---------------------------------------------------------------------------
mod prototype_evidence {
    use ouroforge_core::gdd_prototype_evidence::GddPrototypeEvidenceBundleArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-prototype-evidence-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn prototype_evidence_fixtures_validate_and_export_read_models() {
        for name in [
            "evidence.pass.fixture.json",
            "evidence.fail.fixture.json",
            "evidence.missing-run.fixture.json",
            "evidence.partial.fixture.json",
            "evidence.stale.fixture.json",
        ] {
            let artifact = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(
                read_model.dashboard_summary[0],
                format!("scenarios:{}", artifact.scenario_verdicts.len())
            );
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("read-only prototype evidence")));
            assert!(artifact
                .journal_markdown()
                .contains("GDD Prototype Evidence Journal"));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("auto-apply enabled"));
        }
    }

    #[test]
    fn prototype_evidence_read_model_surfaces_failures_and_unsupported_requirements() {
        let failed = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(
            "evidence.fail.fixture.json",
        ))
        .expect("failed fixture validates");
        assert_eq!(failed.read_model().failed_requirements, ["req-exit"]);
        assert!(failed.journal_markdown().contains("exit trigger missing"));

        let partial = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(
            "evidence.partial.fixture.json",
        ))
        .expect("partial fixture validates");
        assert_eq!(
            partial.read_model().unsupported_requirements,
            ["req-hazard"]
        );
        assert!(partial
            .read_model()
            .dashboard_summary
            .iter()
            .any(|row| row == "requirements:3"));
    }

    #[test]
    fn invalid_prototype_evidence_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/evidence.pass-with-failure.fixture.json",
                "passing GDD prototype evidence requires",
            ),
            (
                "invalid/evidence.fail-without-failed-scenario.fixture.json",
                "failed scenario verdicts",
            ),
            (
                "invalid/evidence.missing-run-no-blocker.fixture.json",
                "missing-run GDD prototype evidence requires",
            ),
            (
                "invalid/evidence.unsupported-scenario-no-blocker.fixture.json",
                "skipped or unsupported GDD prototype scenario verdict requires",
            ),
            (
                "invalid/evidence.unsupported-requirement-no-blocker.fixture.json",
                "unsupported GDD prototype requirements require",
            ),
            (
                "invalid/evidence.unsafe-ref.fixture.json",
                "forbidden traversal",
            ),
            ("invalid/evidence.unsafe-boundary.fixture.json", "forbidden"),
            (
                "invalid/evidence.empty-journal.fixture.json",
                "must not be empty",
            ),
        ] {
            let error = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn prototype_evidence_docs_keep_journal_dashboard_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-prototype-evidence-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #657"));
        assert!(docs.contains("requirement coverage"));
        assert!(docs.contains("Dashboard/Studio compatibility is read-only"));
        assert!(docs.contains("journal summary"));
        assert!(docs.contains("Generated run/evidence output remains untracked"));
        assert!(docs.contains("#1 remains open"));
        assert!(docs.contains("#23 remains open"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "browser trusted write enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_prototype_task_graph_contract
// ---------------------------------------------------------------------------
mod prototype_task_graph {
    use ouroforge_core::gdd_prototype_task_graph::GddPrototypeTaskGraphArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-prototype-task-graph-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn prototype_task_graph_fixtures_validate_and_export_read_models() {
        for name in [
            "task-graph.valid.fixture.json",
            "task-graph.blocked.fixture.json",
        ] {
            let artifact = GddPrototypeTaskGraphArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.task_count, artifact.tasks.len());
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("dependency-checked task graph")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("untrusted until Rust/local validation")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("hidden command execution enabled"));
        }
    }

    #[test]
    fn invalid_prototype_task_graph_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/task-graph.cyclic.fixture.json",
                "cycle or out-of-order dependency",
            ),
            (
                "invalid/task-graph.missing-dependency.fixture.json",
                "missing dependency",
            ),
            (
                "invalid/task-graph.invalid-kind.fixture.json",
                "failed to parse",
            ),
            (
                "invalid/task-graph.conflicting-ownership.fixture.json",
                "conflicting file ownership",
            ),
            (
                "invalid/task-graph.missing-producer.fixture.json",
                "before a producer artifact exists",
            ),
            (
                "invalid/task-graph.apply-without-review.fixture.json",
                "must depend on a review-gate",
            ),
            (
                "invalid/task-graph.unsafe-boundary.fixture.json",
                "forbidden",
            ),
        ] {
            let error = GddPrototypeTaskGraphArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn prototype_task_graph_docs_keep_planning_and_governance_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-prototype-task-graph-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #654"));
        assert!(docs.contains("ordered dependency-checked task graph before apply"));
        assert!(docs.contains("does not execute hidden commands"));
        assert!(docs.contains("file ownership"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "hidden command execution enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "browser trusted write enabled",
            "production-ready claim enabled",
            "autonomous unrestricted game creation enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_requirement_extraction_contract
// ---------------------------------------------------------------------------
mod requirement_extraction {
    use ouroforge_core::gdd_requirement_extraction::GddRequirementExtractionArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-requirement-extraction-v1")
            .join(name)
    }

    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn valid_partial_and_blocked_extraction_fixtures_validate() {
        for name in [
            "requirements.valid.fixture.json",
            "requirements.partial.fixture.json",
            "requirements.blocked.fixture.json",
        ] {
            let artifact = GddRequirementExtractionArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.requirement_count, artifact.requirements.len());
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("display-only")));
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("source section")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("auto-apply enabled"));
        }
    }

    #[test]
    fn invalid_extraction_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/requirements.missing-source-ref.fixture.json",
                "missing sourceSectionRef",
            ),
            (
                "invalid/requirements.duplicate-id.fixture.json",
                "duplicated",
            ),
            (
                "invalid/requirements.invented-no-excerpt.fixture.json",
                "must include sourceExcerpt",
            ),
            (
                "invalid/requirements.invented-unlinked-excerpt.fixture.json",
                "sourceExcerpt is not present",
            ),
            (
                "invalid/requirements.conflict-no-blocker.fixture.json",
                "must include blockedReasons",
            ),
            (
                "invalid/requirements.low-confidence-no-blocker.fixture.json",
                "low-confidence",
            ),
            (
                "invalid/requirements.unsafe-boundary.fixture.json",
                "forbidden",
            ),
        ] {
            let error = GddRequirementExtractionArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn requirement_extraction_docs_keep_boundaries_and_governance() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-requirement-extraction-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #646"));
        assert!(docs.contains("LLM extraction is advisory only"));
        assert!(docs.contains("not a prototype generator"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_scenario_acceptance_plan_contract
// ---------------------------------------------------------------------------
mod scenario_acceptance_plan {
    use ouroforge_core::gdd_scenario_acceptance_plan::GddScenarioAcceptancePlanArtifact;
    use std::{fs, path::PathBuf};
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-scenario-acceptance-plan-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }
    #[test]
    fn scenario_plan_fixtures_validate_and_export_read_models() {
        for name in [
            "scenario-plan.valid.fixture.json",
            "scenario-plan.partial.fixture.json",
            "scenario-plan.blocked.fixture.json",
            "scenario-plan.unsupported.fixture.json",
            "scenario-plan.stale.fixture.json",
        ] {
            let artifact = GddScenarioAcceptancePlanArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(
                read_model.scenario_draft_count,
                artifact.scenario_drafts.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("requirement")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("no trusted test creation")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("trusted tests enabled"));
        }
    }
    #[test]
    fn invalid_scenario_plan_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/scenario-plan.missing-requirement.fixture.json",
                "missing from declared GDD requirements",
            ),
            (
                "invalid/scenario-plan.unsupported-mechanic.fixture.json",
                "mechanics mapping ids",
            ),
            (
                "invalid/scenario-plan.unsupported-assertion.fixture.json",
                "unsupported assertion",
            ),
            (
                "invalid/scenario-plan.unsafe-scenario-ref.fixture.json",
                "forbidden traversal",
            ),
            (
                "invalid/scenario-plan.contradictory-acceptance.fixture.json",
                "contradictory acceptance criteria",
            ),
            (
                "invalid/scenario-plan.missing-evidence.fixture.json",
                "evidenceNeeded must not be empty",
            ),
            (
                "invalid/scenario-plan.stale-no-blocker.fixture.json",
                "stale targets require blockedReasons",
            ),
            (
                "invalid/scenario-plan.unsupported-no-blocker.fixture.json",
                "unsupported checks or stale targets require blockedReasons",
            ),
        ] {
            let error = GddScenarioAcceptancePlanArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }
    #[test]
    fn scenario_plan_docs_keep_governance_and_wording_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-scenario-acceptance-plan-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #653"));
        assert!(docs.contains("non-mutating"));
        assert!(docs.contains("scenario drafts"));
        assert!(docs.contains("not trusted tests"));
        assert!(docs.contains("No hidden implementation of unsupported checks"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "trusted tests enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
            "autonomous unrestricted game creation enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_scene_level_plan_contract
// ---------------------------------------------------------------------------
mod scene_level_plan {
    use ouroforge_core::gdd_scene_level_plan::GddSceneLevelPlanArtifact;
    use std::{fs, path::PathBuf};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/gdd-scene-level-plan-v1")
            .join(name)
    }
    fn read_fixture(name: &str) -> String {
        fs::read_to_string(fixture(name)).expect(name)
    }

    #[test]
    fn scene_level_plan_fixtures_validate_and_export_read_models() {
        for name in [
            "scene-level-plan.valid.fixture.json",
            "scene-level-plan.unsupported.fixture.json",
            "scene-level-plan.partial.fixture.json",
            "scene-level-plan.blocked.fixture.json",
            "scene-level-plan.stale.fixture.json",
        ] {
            let artifact = GddSceneLevelPlanArtifact::from_json_str(&read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
            let read_model = artifact.read_model();
            assert_eq!(read_model.schema_version, artifact.schema_version);
            assert_eq!(read_model.level_intent_count, artifact.level_intents.len());
            assert_eq!(
                read_model.scene_plan_count,
                artifact.scene_generation_plans.len()
            );
            assert!(read_model
                .validation_summary
                .iter()
                .any(|note| note.contains("requirement")));
            assert!(read_model
                .compatibility_notes
                .iter()
                .any(|note| note.contains("non-mutating")));
            assert!(!artifact
                .read_model_json()
                .unwrap()
                .contains("direct scene write enabled"));
        }
    }

    #[test]
    fn invalid_scene_level_plan_fixtures_fail_closed() {
        for (name, expected) in [
            (
                "invalid/scene-level-plan.missing-requirement.fixture.json",
                "missing from declared GDD requirements",
            ),
            (
                "invalid/scene-level-plan.unsupported-no-blocker.fixture.json",
                "blockedReasons",
            ),
            (
                "invalid/scene-level-plan.unsafe-ref.fixture.json",
                "forbidden traversal",
            ),
            (
                "invalid/scene-level-plan.contradictory.fixture.json",
                "contradictory level goals",
            ),
            (
                "invalid/scene-level-plan.missing-proof.fixture.json",
                "objective proof expectation",
            ),
            (
                "invalid/scene-level-plan.stale-no-blocker.fixture.json",
                "stale target",
            ),
        ] {
            let error = GddSceneLevelPlanArtifact::from_json_str(&read_fixture(name))
                .expect_err(name)
                .to_string();
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    #[test]
    fn scene_level_plan_docs_keep_governance_and_wording_boundaries() {
        let docs = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("docs/gdd-scene-level-plan-v1.md"),
        )
        .expect("docs");
        assert!(docs.contains("Issue: #650"));
        assert!(docs.contains("non-mutating"));
        assert!(docs.contains("level-intent-v1"));
        assert!(docs.contains("scene-generation-plan-v1"));
        assert!(docs.contains("No direct scene or tilemap writes"));
        assert!(docs.contains("#1 remains"));
        assert!(docs.contains("#23 remains"));
        for forbidden in [
            "auto-apply enabled",
            "auto-merge enabled",
            "current Godot replacement is implemented",
            "production-ready claim enabled",
            "browser trusted write enabled",
            "autonomous unrestricted game creation enabled",
        ] {
            assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}

// ---------------------------------------------------------------------------
// gdd_to_prototype_demo_contract
// ---------------------------------------------------------------------------
mod to_prototype_demo {
    use ouroforge_core::{
        gdd_asset_placeholder_plan::GddAssetPlaceholderPlanArtifact,
        gdd_design_brief::GddDesignBriefArtifact, gdd_feasibility_gate::GddFeasibilityGateArtifact,
        gdd_gameplay_behavior_plan::GddGameplayBehaviorPlanArtifact,
        gdd_mechanics_mapping::GddMechanicsMappingArtifact,
        gdd_project_scaffold_plan::GddProjectScaffoldPlanArtifact,
        gdd_prototype_apply::GddPrototypeApplyArtifact,
        gdd_prototype_draft_bundle::GddPrototypeDraftBundleArtifact,
        gdd_prototype_evidence::GddPrototypeEvidenceBundleArtifact as GddPrototypeRunEvidenceArtifact,
        gdd_prototype_evidence_bundle::GddPrototypeEvidenceBundleArtifact as GddPrototypeEvidenceJournalBundleArtifact,
        gdd_prototype_task_graph::GddPrototypeTaskGraphArtifact,
        gdd_requirement_extraction::GddRequirementExtractionArtifact,
        gdd_scenario_acceptance_plan::GddScenarioAcceptancePlanArtifact,
        gdd_scene_level_plan::GddSceneLevelPlanArtifact,
    };
    use serde_json::Value;
    use std::{fs, path::PathBuf};

    fn repo_path(path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path)
    }

    fn read_repo(path: &str) -> String {
        fs::read_to_string(repo_path(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
    }

    fn manifest() -> Value {
        serde_json::from_str(&read_repo(
            "examples/gdd-to-prototype-demo-v1/demo.manifest.fixture.json",
        ))
        .expect("demo manifest json")
    }

    fn ref_at<'a>(manifest: &'a Value, key: &str) -> &'a str {
        manifest["artifactRefs"][key]
            .as_str()
            .unwrap_or_else(|| panic!("missing artifact ref {key}"))
    }

    #[test]
    fn gdd_to_prototype_demo_manifest_links_validated_artifacts_end_to_end() {
        let manifest = manifest();
        assert_eq!(manifest["schemaVersion"], "gdd-to-prototype-demo-v1");
        assert_eq!(manifest["issue"], 659);
        assert_eq!(manifest["status"], "evidence-gated-pass");
        assert!(repo_path(manifest["gddRef"].as_str().unwrap()).exists());

        GddDesignBriefArtifact::from_json_str(&read_repo(ref_at(&manifest, "designBrief")))
            .unwrap();
        GddRequirementExtractionArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "requirements",
        )))
        .unwrap();
        GddMechanicsMappingArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "mechanicsMapping",
        )))
        .unwrap();
        GddFeasibilityGateArtifact::from_json_str(&read_repo(ref_at(&manifest, "feasibilityGate")))
            .unwrap();
        GddProjectScaffoldPlanArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "scaffoldPlan",
        )))
        .unwrap();
        GddSceneLevelPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "sceneLevelPlan")))
            .unwrap();
        GddGameplayBehaviorPlanArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "behaviorPlan",
        )))
        .unwrap();
        GddAssetPlaceholderPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "assetPlan")))
            .unwrap();
        GddScenarioAcceptancePlanArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "scenarioPlan",
        )))
        .unwrap();
        GddPrototypeTaskGraphArtifact::from_json_str(&read_repo(ref_at(&manifest, "taskGraph")))
            .unwrap();
        GddPrototypeDraftBundleArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "draftBundle",
        )))
        .unwrap();
        GddPrototypeApplyArtifact::from_json_str(&read_repo(ref_at(&manifest, "reviewApply")))
            .unwrap();
        GddPrototypeRunEvidenceArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "runEvidence",
        )))
        .unwrap();
        GddPrototypeEvidenceJournalBundleArtifact::from_json_str(&read_repo(ref_at(
            &manifest,
            "evidenceJournalBundle",
        )))
        .unwrap();
    }

    #[test]
    fn gdd_to_prototype_demo_records_inert_commands_and_cleanup_policy() {
        let manifest = manifest();
        let commands = manifest["inertCommands"]
            .as_array()
            .expect("commands array");
        assert!(commands
            .iter()
            .all(|command| command.as_str().unwrap().starts_with("cargo ")
                || command.as_str().unwrap().starts_with("node ")));
        assert!(manifest["expectedEvidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item
                .as_str()
                .unwrap()
                .contains("read-only/read-model compatible")));
        assert!(manifest["cleanupPolicy"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("remain untracked")));
        assert!(manifest["assetSourceNotes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("placeholder/local fixture")));
    }

    #[test]
    fn gdd_to_prototype_demo_docs_keep_governance_and_wording_boundaries() {
        let docs = read_repo("docs/gdd-to-prototype-demo-v1.md");
        let manifest = read_repo("examples/gdd-to-prototype-demo-v1/demo.manifest.fixture.json");
        for text in [&docs, &manifest] {
            assert!(text.contains("Issue: #659") || text.contains("\"issue\": 659"));
            assert!(
                text.contains("Generated prototype drafts")
                    || text.contains("generated prototype drafts")
            );
            assert!(text.contains("placeholder") || text.contains("placeholders"));
            assert!(text.contains("#1 remains open"));
            assert!(text.contains("#23 remains open"));
            for forbidden in [
                "browser trusted write enabled",
                "auto-apply enabled",
                "auto-merge enabled",
                "autonomous unrestricted game creation enabled",
                "production-ready claim enabled",
                "current Godot replacement is implemented",
                "native export enabled",
                "plugin runtime enabled",
            ] {
                assert!(!text.contains(forbidden), "forbidden wording: {forbidden}");
            }
        }
    }
}
