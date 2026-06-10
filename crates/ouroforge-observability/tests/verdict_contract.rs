use ouroforge_observability::{checklist_ids, render_verdict, VerdictOptions};

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
