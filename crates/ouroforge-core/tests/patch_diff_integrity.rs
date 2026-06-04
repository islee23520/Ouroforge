use ouroforge_core::{
    parse_unified_patch_diff_integrity, PatchDiffFileStatus, PatchDiffIntegrityWarningKind,
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
