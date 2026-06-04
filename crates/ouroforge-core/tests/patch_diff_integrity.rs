use ouroforge_core::{
    inspect_unified_patch_diff_for_preview, parse_unified_patch_diff_integrity,
    validate_unified_patch_diff_for_preview, PatchDiffFileStatus, PatchDiffIntegrityLimits,
    PatchDiffIntegrityWarningKind,
};

fn warning_kinds(
    report: &ouroforge_core::PatchDiffIntegrityReport,
) -> Vec<PatchDiffIntegrityWarningKind> {
    report
        .warnings
        .iter()
        .map(|warning| warning.kind.clone())
        .collect()
}

#[test]
fn patch_diff_integrity_parses_valid_two_file_unified_diff_fixture() {
    let report = parse_unified_patch_diff_integrity(include_str!(
        "../../../examples/patch-diff-integrity-v1/valid/two-file-basic.diff"
    ))
    .expect("valid two-file unified diff fixture should parse");

    assert_eq!(report.file_count, 2);
    assert_eq!(report.hunk_count, 2);
    assert_eq!(report.counts.added, 3);
    assert_eq!(report.counts.removed, 2);
    assert_eq!(report.counts.context, 3);
    assert!(
        report.warnings.is_empty(),
        "unexpected warnings: {:?}",
        report.warnings
    );

    let first = &report.files[0];
    assert_eq!(first.old_path, "docs/example.md");
    assert_eq!(first.new_path, "docs/example.md");
    assert_eq!(first.status, PatchDiffFileStatus::Modified);
    assert_eq!(first.hunks[0].old_start, 1);
    assert_eq!(first.hunks[0].new_start, 1);
    assert_eq!(first.hunks[0].old_lines, first.hunks[0].actual_old_lines);
    assert_eq!(first.hunks[0].new_lines, first.hunks[0].actual_new_lines);
}

#[test]
fn patch_diff_integrity_parses_valid_new_file_unified_diff_fixture() {
    let report = parse_unified_patch_diff_integrity(include_str!(
        "../../../examples/patch-diff-integrity-v1/valid/new-file-basic.diff"
    ))
    .expect("valid new-file unified diff fixture should parse");

    assert_eq!(report.file_count, 1);
    assert_eq!(report.hunk_count, 1);
    assert_eq!(report.counts.added, 3);
    assert_eq!(report.counts.removed, 0);
    assert_eq!(report.files[0].old_path, "/dev/null");
    assert_eq!(
        report.files[0].new_path,
        "examples/patch-diff-integrity-v1/new-fixture.txt"
    );
    assert_eq!(report.files[0].status, PatchDiffFileStatus::Added);
    let kinds = warning_kinds(&report);
    assert!(
        kinds.contains(&PatchDiffIntegrityWarningKind::ModeChange),
        "new-file mode metadata should be surfaced for later rejection: {kinds:?}"
    );
}

#[test]
fn patch_diff_integrity_reports_malformed_parser_failures_from_fixtures() {
    let cases = [
        (
            "missing_new_file_header",
            include_str!(
                "../../../examples/patch-diff-integrity-v1/invalid/missing_new_file_header.diff"
            ),
            PatchDiffIntegrityWarningKind::HunkLineCountMismatch,
        ),
        (
            "hunk_count_mismatch",
            include_str!(
                "../../../examples/patch-diff-integrity-v1/invalid/hunk_count_mismatch.diff"
            ),
            PatchDiffIntegrityWarningKind::HunkLineCountMismatch,
        ),
        (
            "orphan_hunk",
            include_str!("../../../examples/patch-diff-integrity-v1/invalid/orphan_hunk.diff"),
            PatchDiffIntegrityWarningKind::MissingFileHeader,
        ),
    ];

    for (name, fixture, expected_warning) in cases {
        let report = parse_unified_patch_diff_integrity(fixture).unwrap_or_else(|error| {
            panic!("{name} should produce parser warnings, not fail hard: {error}")
        });
        let kinds = warning_kinds(&report);
        assert!(
            kinds.contains(&expected_warning),
            "{name} expected {expected_warning:?}, got {kinds:?}"
        );
    }
}

fn assert_blocked_fixture(name: &str, diff_text: &str, expected: &str) {
    let validation =
        inspect_unified_patch_diff_for_preview(diff_text, PatchDiffIntegrityLimits::default())
            .unwrap_or_else(|error| {
                panic!("{name} should inspect without hard parser failure: {error}")
            });
    assert_eq!(validation.status, "blocked", "{name} should be blocked");
    assert!(
        validation
            .blocked_reasons
            .iter()
            .any(|reason| reason.contains(expected)),
        "{name} expected blocker containing {expected:?}, got {:?}",
        validation.blocked_reasons
    );
    let error =
        validate_unified_patch_diff_for_preview(diff_text, PatchDiffIntegrityLimits::default())
            .expect_err("blocked integrity validation should reject before preview");
    assert!(error
        .to_string()
        .contains("patch diff integrity validation blocked"));
}

#[test]
fn patch_diff_integrity_blocks_unsafe_targets_before_preview() {
    let cases = [
        (
            "generated_target",
            include_str!("../../../examples/patch-diff-integrity-v1/unsafe/generated-target.diff"),
            "generated",
        ),
        (
            "traversal_target",
            include_str!("../../../examples/patch-diff-integrity-v1/unsafe/traversal-target.diff"),
            "absolute, rooted, prefixed, or parent-dir path",
        ),
        (
            "binary_target",
            include_str!("../../../examples/patch-diff-integrity-v1/unsafe/binary-target.diff"),
            "binary",
        ),
        (
            "mode_change",
            include_str!("../../../examples/patch-diff-integrity-v1/unsafe/mode-change.diff"),
            "mode",
        ),
        (
            "critical_delete",
            include_str!("../../../examples/patch-diff-integrity-v1/unsafe/critical-delete.diff"),
            "critical file deletion",
        ),
    ];

    for (name, fixture, expected) in cases {
        assert_blocked_fixture(name, fixture, expected);
    }
}

#[test]
fn patch_diff_integrity_blocks_file_and_line_limit_overflows() {
    let mut many_files = String::new();
    for index in 0..3 {
        many_files.push_str(&format!(
            "diff --git a/docs/file-{index}.md b/docs/file-{index}.md\n--- a/docs/file-{index}.md\n+++ b/docs/file-{index}.md\n@@ -1 +1 @@\n-old\n+new\n"
        ));
    }
    let file_limit = inspect_unified_patch_diff_for_preview(
        &many_files,
        PatchDiffIntegrityLimits {
            max_files: 2,
            max_changed_lines: 100,
        },
    )
    .expect("file-limit diff inspects");
    assert_eq!(file_limit.status, "blocked");
    assert!(file_limit
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("exceeding limit 2")));

    let line_limit = inspect_unified_patch_diff_for_preview(
        include_str!("../../../examples/patch-diff-integrity-v1/valid/two-file-basic.diff"),
        PatchDiffIntegrityLimits {
            max_files: 10,
            max_changed_lines: 1,
        },
    )
    .expect("line-limit diff inspects");
    assert_eq!(line_limit.status, "blocked");
    assert!(line_limit
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("exceeding limit 1")));
}

#[test]
fn patch_diff_integrity_allows_valid_fixture_for_later_preview_checks() {
    let validation = validate_unified_patch_diff_for_preview(
        include_str!("../../../examples/patch-diff-integrity-v1/valid/two-file-basic.diff"),
        PatchDiffIntegrityLimits::default(),
    )
    .expect("valid docs/examples diff should pass integrity preflight");
    assert_eq!(validation.status, "passed");
    assert!(validation.blocked_reasons.is_empty());
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("no source patch apply")));
}
