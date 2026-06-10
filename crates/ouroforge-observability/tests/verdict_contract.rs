use ouroforge_observability::{
    checklist_ids, render_verdict, validate_bundle, write_rendered_verdict, VerdictOptions,
};
use std::fs;
use std::path::{Path, PathBuf};

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
    let parent = std::env::temp_dir().join(format!(
        "ouroforge-observability-test-{}",
        std::process::id()
    ));
    let dest = parent.join(name);
    let _ = fs::remove_dir_all(&parent);
    copy_dir_all(&source, &dest);
    dest
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
