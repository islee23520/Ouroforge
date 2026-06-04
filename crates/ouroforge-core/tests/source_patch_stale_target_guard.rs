use ouroforge_core::{
    SourcePatchStaleTargetGuardArtifact, SourcePatchStaleTargetGuardStatus,
    SOURCE_PATCH_STALE_TARGET_GUARD_SCHEMA_VERSION,
};
use serde_json::json;

fn fixture() -> SourcePatchStaleTargetGuardArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-stale-target-guard-v1/stale-target-guard.sample.json"
    ))
    .expect("stale target guard fixture deserializes")
}

#[test]
fn source_patch_stale_target_guard_round_trips_fixture_without_apply_authority() {
    let artifact = fixture();

    assert_eq!(
        artifact.schema_version,
        SOURCE_PATCH_STALE_TARGET_GUARD_SCHEMA_VERSION
    );
    assert_eq!(artifact.status, SourcePatchStaleTargetGuardStatus::Fresh);
    assert_eq!(artifact.targets.len(), 1);
    assert_eq!(artifact.guard_results.len(), 2);
    assert!(artifact
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("do not apply patches")));

    let value = serde_json::to_value(&artifact).expect("guard serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
    assert!(value.get("browserCommandBridge").is_none());
    assert_eq!(
        value["targets"][0]["expectedBeforeHash"],
        value["targets"][0]["observedHash"]
    );

    let round_trip: SourcePatchStaleTargetGuardArtifact =
        serde_json::from_value(value).expect("serialized guard shape remains valid");
    assert_eq!(round_trip, artifact);
}

#[test]
fn source_patch_stale_target_guard_schema_rejects_unknown_apply_command_fields() {
    let mut value = serde_json::to_value(fixture()).expect("fixture serializes");
    value["applyCommand"] = json!("git apply patch.diff");

    let error = serde_json::from_value::<SourcePatchStaleTargetGuardArtifact>(value)
        .expect_err("unknown apply authority field must be rejected by schema");
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn source_patch_stale_target_guard_models_stale_and_blocked_evidence() {
    let mut artifact = fixture();
    artifact.status = SourcePatchStaleTargetGuardStatus::Stale;
    artifact.evidence_freshness.stale_reasons =
        vec!["review decision predates transaction".to_string()];
    artifact.targets[0].observed_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    artifact.targets[0].file_status = "changed_since_preview".to_string();
    artifact.blocked_reasons = vec!["target hash mismatch".to_string()];

    let value = serde_json::to_value(&artifact).expect("stale guard serializes");
    assert_eq!(value["status"], "stale");
    assert_eq!(value["targets"][0]["fileStatus"], "changed_since_preview");
    assert!(value["evidenceFreshness"]["staleReasons"][0]
        .as_str()
        .unwrap()
        .contains("review decision"));
    assert!(value["blockedReasons"][0]
        .as_str()
        .unwrap()
        .contains("target hash mismatch"));
}
