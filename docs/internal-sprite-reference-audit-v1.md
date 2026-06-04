# Internal Sprite Reference Audit v1

Issue #786 allows a minimal demo asset-pack path only when provenance stays safe.
The local RO-style reference folder is useful for render-readiness triage, but
its sprites are internal-use-only and must not become distributed evidence.

## Boundary

- Reference root: local internal sprite resource folder
- License scope: internal-use-only
- Git commit allowed: no
- Screenshot capture allowed: no
- Upload allowed: no
- Private file copying: no

The audit command reads local filenames and counts PNG frames. It does not copy
sprite files, generate screenshots, write manifest entries from private sprites,
or authorize browser-side uploads.

## Command

```bash
cargo run -p ouroforge-cli -- asset audit-internal-sprites <local-internal-sprite-root> --profile ro-vibe-v1
```

Use `--json` for local issue triage artifacts. Keep those artifacts ignored.
Committed docs and PR text should describe the local reference root generically;
absolute machine-local paths belong only in ignored terminal evidence.

## Readiness Profile

`ro-vibe-v1` checks for a tiny render-readiness sample:

- male Novice idle frame
- male Novice walk frame
- female Novice idle frame
- female Novice walk frame

Missing files produce `Issue note:` lines suitable for a follow-up issue body.
Present files only prove that local internal reference material is available for
manual render experiments; they do not create a redistributable asset pack.
