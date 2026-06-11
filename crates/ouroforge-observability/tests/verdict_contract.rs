use ouroforge_observability::{
    checklist_ids, render_verdict, validate_bundle, write_rendered_verdict, VerdictOptions,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_COPY_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn strip_generated_at(markdown: &str) -> String {
    markdown
        .lines()
        .filter(|line| !line.starts_with("Generated at:"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn renders_collect_and_exit_contract_pass_product_fail_golden() {
    let options = VerdictOptions {
        generated_at: "2026-06-10T00:00:00Z".to_string(),
        ..Default::default()
    };
    let actual = render_verdict("fixtures/collect-and-exit-product-fail", &options).unwrap();
    let expected = include_str!("../fixtures/collect-and-exit-product-fail/verdict.expected.md");
    assert_eq!(actual, expected);
    assert!(actual.contains("Mechanical contract: `contract-pass`"));
    assert!(actual.contains("Product observation: `product-observed FAIL`"));
    assert!(actual.contains("missing_asset"));
    for id in checklist_ids() {
        assert!(actual.contains(id), "missing checklist id {id}");
    }
}

#[test]
fn stale_grid_won_does_not_satisfy_non_grid_replay_objective() {
    let temp = copy_fixture_to_temp("collect-and-exit-product-fail");
    let replay_path = temp.join("input-replay.json");
    let mut replay: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&replay_path).unwrap()).unwrap();
    let sequence = replay["objective_flag_sequence"].as_array_mut().unwrap();
    let final_flags = sequence
        .last_mut()
        .unwrap()
        .get_mut("goal_flags")
        .unwrap()
        .as_object_mut()
        .unwrap();
    final_flags.insert("exit_reached".to_string(), serde_json::json!(false));
    final_flags.insert("grid_won".to_string(), serde_json::json!(true));
    fs::write(&replay_path, serde_json::to_string(&replay).unwrap()).unwrap();
    refresh_manifest_digest(&temp, "input-replay.json");

    let verdict = render_verdict(
        &temp,
        &VerdictOptions {
            generated_at: "2026-06-10T00:00:03Z".to_string(),
            ..Default::default()
        },
    )
    .unwrap();

    assert!(verdict.contains("Replay: `collect-and-exit`"));
    assert!(verdict.contains("final exit_reached: false; final grid_won: true"));
    assert!(verdict.contains("Mechanical contract: `contract-fail`"));
    assert!(verdict.contains("Product observation: `product-observed FAIL`"));
    let _ = fs::remove_dir_all(temp);
}

#[test]
fn rendered_verdict_is_byte_identical_except_timestamp_line() {
    let a = render_verdict(
        "fixtures/collect-and-exit-product-fail",
        &VerdictOptions {
            generated_at: "2026-06-10T00:00:00Z".to_string(),
            ..Default::default()
        },
    )
    .unwrap();
    let b = render_verdict(
        "fixtures/collect-and-exit-product-fail",
        &VerdictOptions {
            generated_at: "2026-06-10T00:00:01Z".to_string(),
            ..Default::default()
        },
    )
    .unwrap();
    assert_ne!(a, b);
    assert_eq!(strip_generated_at(&a), strip_generated_at(&b));
}

#[test]
fn rendered_verdict_is_stable_across_equivalent_bundle_paths() {
    let options = VerdictOptions {
        generated_at: "2026-06-10T00:00:00Z".to_string(),
        ..Default::default()
    };
    let relative = render_verdict("fixtures/collect-and-exit-product-fail", &options).unwrap();
    let absolute = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collect-and-exit-product-fail");
    let absolute_render = render_verdict(&absolute, &options).unwrap();
    assert_eq!(relative, absolute_render);
}

#[test]
fn write_rendered_verdict_refreshes_manifest_digest() {
    let temp = copy_fixture_to_temp("collect-and-exit-product-fail");
    let options = VerdictOptions {
        generated_at: "2026-06-10T00:00:02Z".to_string(),
        ..Default::default()
    };
    write_rendered_verdict(&temp, &options).unwrap();
    validate_bundle(&temp).unwrap();
    let verdict = fs::read_to_string(temp.join("verdict.md")).unwrap();
    assert!(verdict.contains("Generated at: 2026-06-10T00:00:02Z"));
    let _ = fs::remove_dir_all(temp);
}

fn copy_fixture_to_temp(name: &str) -> PathBuf {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name);
    let unique = TEMP_COPY_COUNTER.fetch_add(1, Ordering::SeqCst);
    let parent = std::env::temp_dir().join(format!(
        "ouroforge-observability-test-{}-{unique}",
        std::process::id()
    ));
    let dest = parent.join(name);
    let _ = fs::remove_dir_all(&parent);
    copy_dir_all(&source, &dest);
    dest
}

fn refresh_manifest_digest(bundle_root: &Path, artifact_path: &str) {
    let manifest_path = bundle_root.join("manifest.json");
    let mut manifest: Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
    let digest = hex_sha256(&fs::read(bundle_root.join(artifact_path)).unwrap());
    let inventory = manifest["artifact_inventory"].as_array_mut().unwrap();
    let entry = inventory
        .iter_mut()
        .find(|entry| entry["path"].as_str() == Some(artifact_path))
        .unwrap();
    entry["sha256"] = Value::String(digest);
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap() + "\n",
    )
    .unwrap();
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn copy_dir_all(source: &Path, dest: &Path) {
    fs::create_dir_all(dest).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let dest_path = dest.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path);
        } else {
            fs::copy(entry.path(), dest_path).unwrap();
        }
    }
}
