//! Contract tests for Save/Profile and Run-History at Scale v1 (#1659).
//!
//! Covers the three required behaviors: multi-profile isolation, large-history
//! digest integrity, and migration/back-compat, plus the fail-closed negatives
//! that keep the save store deterministic and trusted-only.

use ouroforge_core::save_profile_scale::{
    RunHistoryEntry, SaveStore, SAVE_PROFILE_SCALE_BOUNDARY, SAVE_PROFILE_SCALE_SCHEMA_VERSION,
};

fn legacy_v0_json() -> &'static str {
    include_str!("../../../examples/save-profile-scale-v1/legacy/save-profile.v0.json")
}

fn entry(run_id: &str, digest: &str) -> RunHistoryEntry {
    RunHistoryEntry {
        run_id: run_id.to_string(),
        recorded_at: "2026-06-07T00:00:00Z".to_string(),
        replay_digest: digest.to_string(),
    }
}

#[test]
fn empty_store_has_canonical_shape() {
    let store = SaveStore::new();
    assert_eq!(store.schema_version, SAVE_PROFILE_SCALE_SCHEMA_VERSION);
    assert_eq!(store.boundary, SAVE_PROFILE_SCALE_BOUNDARY);
    assert!(store.profiles.is_empty());
    store.verify_integrity().expect("empty store is valid");
}

#[test]
fn profiles_are_isolated() {
    let mut store = SaveStore::new();
    store
        .append_run("alice", entry("a1", "deadbeef01"))
        .expect("append a1");
    store
        .append_run("bob", entry("b1", "deadbeef02"))
        .expect("append b1");
    store
        .append_run("alice", entry("a2", "deadbeef03"))
        .expect("append a2");

    let alice = store.profile("alice").expect("alice exists");
    let bob = store.profile("bob").expect("bob exists");
    assert_eq!(alice.run_history.len(), 2);
    assert_eq!(bob.run_history.len(), 1);
    // A write to alice did not alter bob's digest or history.
    assert_eq!(bob.run_history[0].run_id, "b1");
    assert_ne!(alice.history_digest, bob.history_digest);
    store.verify_integrity().expect("store is consistent");
}

#[test]
fn large_history_round_trips_with_digest_integrity() {
    let mut store = SaveStore::new();
    for i in 0..5000u32 {
        store
            .append_run(
                "scaler",
                entry(&format!("run-{i:05}"), &format!("digest-{i:05}")),
            )
            .expect("append");
    }
    let profile = store.profile("scaler").expect("scaler exists");
    assert_eq!(profile.run_history.len(), 5000);

    let json = store.to_json().expect("serialize");
    let restored = SaveStore::from_json_str(&json).expect("deserialize + verify");
    assert_eq!(store, restored);
    assert_eq!(restored.profile("scaler").unwrap().run_history.len(), 5000);
}

#[test]
fn tampered_large_history_fails_integrity() {
    let mut store = SaveStore::new();
    for i in 0..1000u32 {
        store
            .append_run(
                "scaler",
                entry(&format!("run-{i:05}"), &format!("digest-{i:05}")),
            )
            .expect("append");
    }
    // Tamper with one entry deep in the history without resealing the digest.
    let profile = store.profiles.get_mut("scaler").expect("scaler exists");
    profile.run_history[500].replay_digest = "tampered".to_string();
    let err = store
        .verify_integrity()
        .expect_err("tampered history must fail closed");
    assert!(err.to_string().contains("integrity check failed"));
}

#[test]
fn migration_from_legacy_v0_upgrades_and_seals_digest() {
    let store = SaveStore::from_legacy_v0_json(legacy_v0_json()).expect("migrate legacy v0");
    assert_eq!(store.schema_version, SAVE_PROFILE_SCALE_SCHEMA_VERSION);
    let profile = store
        .profile("legacy-hero")
        .expect("legacy profile migrated");
    assert_eq!(profile.schema_version, SAVE_PROFILE_SCALE_SCHEMA_VERSION);
    assert_eq!(profile.boundary, SAVE_PROFILE_SCALE_BOUNDARY);
    assert_eq!(profile.run_history.len(), 2);
    assert_eq!(profile.run_history[0].run_id, "run-001");
    // The migrated store is integrity-sealed and round-trips as v1.
    store.verify_integrity().expect("migrated store verifies");
    let json = store.to_json().expect("serialize");
    let restored = SaveStore::from_json_str(&json).expect("restore migrated store");
    assert_eq!(store, restored);
}

#[test]
fn duplicate_run_id_within_profile_is_rejected() {
    let mut store = SaveStore::new();
    store.append_run("p", entry("dup", "d1")).expect("first");
    let err = store
        .append_run("p", entry("dup", "d2"))
        .expect_err("duplicate run id must fail closed");
    assert!(err.to_string().contains("already in profile"));
    // The store is unchanged on the rejected append.
    assert_eq!(store.profile("p").unwrap().run_history.len(), 1);
    store.verify_integrity().expect("store still consistent");
}

#[test]
fn empty_run_id_and_empty_replay_digest_are_rejected() {
    let mut store = SaveStore::new();
    assert!(store.append_run("p", entry("", "d1")).is_err());
    assert!(store.append_run("p", entry("r1", "")).is_err());
    assert!(store.append_run("", entry("r1", "d1")).is_err());
}

#[test]
fn wrong_boundary_store_fails_integrity() {
    let mut store = SaveStore::new();
    store.append_run("p", entry("r1", "d1")).expect("append");
    store.boundary = "browser-can-write".to_string();
    let err = store
        .verify_integrity()
        .expect_err("non-canonical boundary must fail closed");
    assert!(err
        .to_string()
        .contains("canonical read-only/proposal-only"));
}

#[test]
fn key_not_matching_profile_id_fails_integrity() {
    let mut store = SaveStore::new();
    store.append_run("real", entry("r1", "d1")).expect("append");
    // Re-key the profile under a different map key.
    let profile = store.profiles.remove("real").expect("exists");
    store.profiles.insert("forged".to_string(), profile);
    let err = store
        .verify_integrity()
        .expect_err("mismatched key must fail closed");
    assert!(err.to_string().contains("does not match profile id"));
}

#[test]
fn restored_store_with_blank_profile_id_is_rejected() {
    // A crafted v1 save where the map key and profileId are both blank must not
    // pass restore even though key == profileId and the digest could match.
    let crafted = format!(
        r#"{{
        "schemaVersion": "{SAVE_PROFILE_SCALE_SCHEMA_VERSION}",
        "profiles": {{
            "": {{
                "schemaVersion": "{SAVE_PROFILE_SCALE_SCHEMA_VERSION}",
                "profileId": "",
                "runHistory": [],
                "historyDigest": "whatever",
                "boundary": "{SAVE_PROFILE_SCALE_BOUNDARY}"
            }}
        }},
        "boundary": "{SAVE_PROFILE_SCALE_BOUNDARY}"
    }}"#
    );
    let err = SaveStore::from_json_str(&crafted)
        .expect_err("blank profile id on restore must fail closed");
    assert!(err.to_string().contains("profile id must not be empty"));
}

#[test]
fn wrong_legacy_schema_version_is_rejected() {
    let bad = r#"{
        "schemaVersion": "save-profile-v9",
        "profileId": "p",
        "runHistory": []
    }"#;
    let err =
        SaveStore::from_legacy_v0_json(bad).expect_err("wrong legacy schema must fail closed");
    assert!(err.to_string().contains("legacy save schema version"));
}

#[test]
fn read_model_exposes_read_only_summary() {
    let mut store = SaveStore::new();
    store.append_run("alice", entry("a1", "d1")).expect("a1");
    store.append_run("alice", entry("a2", "d2")).expect("a2");
    store.append_run("bob", entry("b1", "d3")).expect("b1");
    let read = store.read_model();
    assert_eq!(read.profile_count, 2);
    assert_eq!(read.total_runs, 3);
    assert_eq!(read.run_counts.get("alice"), Some(&2));
    assert_eq!(read.run_counts.get("bob"), Some(&1));
    assert_eq!(read.boundary, SAVE_PROFILE_SCALE_BOUNDARY);
}
