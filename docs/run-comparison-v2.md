# Run Comparison v2 / Semantic Evidence Diff

Run Comparison v2 enriches the existing Rust CLI `compare` artifact with deterministic semantic sections while preserving the legacy top-level `classification`, `before`, `after`, `deltas`, `evidence_refs`, and `unsupported` fields.

## Contract

- Comparisons remain local-first and file-based.
- The Rust CLI is the trusted comparison writer.
- The browser dashboard only displays existing comparison artifacts; it does not compute comparisons.
- Semantic reasons are deterministic and evidence-derived; no AI summary or subjective gameplay scoring is added.
- Required run artifacts (`run.json`, `verdict.json`, `evidence/index.json`) remain strict requirements.
- Optional semantic evidence read failures are recorded as warnings instead of panics.
- Generated run, dashboard, and comparison artifacts remain untracked.

## Semantic sections

`semantic.schemaVersion = run-semantic-diff-v1` includes:

- `reasons`: deterministic top-level explanations for meaningful changes.
- `scenarios`: scenario status changes, including missing/present transitions.
- `worldState`: bounded scalar path changes/additions/removals.
- `events`: bounded event additions/removals from supported event-like artifacts.
- `performance`: bounded frame/performance scalar changes plus partial-data warnings.
- `evidence`: evidence artifact additions/removals.
- `transactionProvenance`: before/after scene edit transaction provenance when present.
- `warnings`: malformed or missing optional evidence notices.

## Dashboard compatibility

Dashboard exports include `comparison.artifacts[].semantic` when the artifact has a semantic section. Legacy comparison artifacts without `semantic` remain readable and render an explicit semantic empty-state.

## AL2.5.4 integration evidence

Commands run on this branch:

```bash
cargo fmt --check
cargo test --workspace
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
BEFORE_RUN=$(ls -td runs/run-* | sed -n '2p')
AFTER_RUN=$(ls -td runs/run-* | head -1)
test -n "$BEFORE_RUN"
test -n "$AFTER_RUN"
cargo run -p ouroforge-cli -- compare "$BEFORE_RUN" "$AFTER_RUN" --output-dir "$AFTER_RUN/comparisons"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output .omx/tmp/semantic-diff-v2/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

Evidence run pair:

- Before: `runs/run-1780405360519-36952`
- After: `runs/run-1780405368359-37223`
- Comparison artifact: `runs/run-1780405368359-37223/comparisons/run-comparison-run-1780405360519-36952--run-1780405368359-37223.json`

Top semantic reasons from CLI output:

- `[regressed] scenario_verdict: scenario bootstrap-smoke changed from passed to missing`
- `[improved] scenario_verdict: scenario feature-surface-smoke changed from missing to passed`
- `[regressed] scenario_verdict: scenario objective-contact changed from passed to missing`
- `[improved] scenario_verdict: scenario objective-contact-integration changed from missing to passed`
- `[changed] evidence_artifacts: 38 evidence artifacts added, 38 removed`

Compatibility caveats:

- Comparing different seed shapes can produce intentional `missing` scenario transitions; this is deterministic and visible rather than hidden.
- Top-level legacy classification can remain `no_change` when verdict/scenario counts match, even while semantic sections identify scenario identity/evidence changes.
- Dashboard semantic rendering is read-only and does not apply or accept mutations.
