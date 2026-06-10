# Dogfood B3 Pipeline Dry-Run Evidence

## Metadata

- Blocker: B3 — canonical compact-demo pipeline dry-run evidence missing/incomplete
- Report version: `dogfood-pipeline-dry-run-v1`
- Demo spec: `.omx/dogfood-validation/demo-game-spec.md` (`dogfood-demo-spec-v1`)
- Claim matrix: `.omx/dogfood-validation/claim-coverage-matrix.md` (`dogfood-claim-coverage-v1`)
- Branch: `dogfood/b3-pipeline-dry-run-20260609214129`
- Base: `origin/main` at merge commit `4d52855a78e4aa70a4fa04e2dec31ab00e1fd2cc`
- Generated work directory: `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c`
- Generated-state policy: generated runs, dashboard data, comparisons, screenshots, and logs stayed outside the repository in the generated work directory and are not committed.

## Commands executed

All commands were local and non-destructive. The generated workspace symlinked `examples` to the repo so the CLI could resolve `examples/game-runtime` while writing `runs/` outside the repo.

```bash
work="/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c"
ln -s /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples "$work/examples"
cd "$work"

cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- seed validate /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- project validate /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- run /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml --project /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json --scenario-pack collect-and-exit --workers 2
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- evaluate runs/run-1781041430565-62207
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- journal show runs/run-1781041430565-62207
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- mutation list runs/run-1781041430565-62207
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- mutation review --defer --reason "B3 dry-run records proposal only; no apply" --evidence mutation/proposals.json runs/run-1781041430565-62207
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- run /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml --project /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json --scenario-pack collect-and-exit --workers 2
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- compare runs/run-1781041430565-62207 runs/run-1781041458557-98997 --output-dir comparisons
cargo run --manifest-path /Users/jh0927/Ouroforge-dogfood-b3-pipeline-dry-run-20260609214129/Cargo.toml -q -p ouroforge-cli -- dashboard export --runs-root runs --output dashboard-data.json
```

## Evidence summary

| Requirement | Verdict | Evidence / artifact path | Notes |
| --- | --- | --- | --- |
| B1 claim matrix reference | pass | `.omx/dogfood-validation/claim-coverage-matrix.md` | Matrix is merged on `origin/main`; B3 row remains unverified until this report is accepted. |
| B2 demo spec reference | pass | `.omx/dogfood-validation/demo-game-spec.md` | Spec version `dogfood-demo-spec-v1` anchors the Collect-and-Exit target. |
| Seed/spec validation | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/pipeline-step-1.log` | `Seed valid: playable-demo.collect-and-exit`. |
| Project validation/binding | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/pipeline-step-1.log`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/run.json` | `Project manifest valid: collect_and_exit_demo`; run output says `Run project bound: collect_and_exit_demo`. |
| Run evidence and run ID | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041458557-98997` | Two local generated runs were created for dry-run/replay comparison. |
| Evaluator/verdict output | failed-classified | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/verdict.json`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/evaluate-run1.log` | Verdict status `failed`; summary: `6 failure(s) found across 1 scenario result(s)`. |
| Journal output | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/journal.md`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/journal-show-run1.log` | Journal records seed, project context, reproducible command context, evidence, verdict summary, and mutation proposal. |
| Mutation/proposal output | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/mutation/proposals.json`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/mutation/review-decisions.json` | Proposal `mutation-1781041440664-1`; review decision `review-decision-1` deferred; no apply. |
| Replay/regression comparison | failed-classified | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/comparisons/run-comparison-run-1781041430565-62207--run-1781041458557-98997.json`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/compare.log` | Comparison is same-project but non-comparable for gate promotion because input replay refs are absent and both runs remain failed. |
| Dashboard/read model output | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/dashboard-data.json`; `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/dashboard-export.log` | Dashboard data exported from generated runs only. |
| Generated-state cleanup/retention | pass | `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c`; repo generated roots clean | Generated evidence is retained outside the repo for verifier inspection and not committed. |

## Classified failure details

The compact pipeline dry-run is reproducible but not green. The current canonical failure is a scenario/evidence mismatch, not missing B3 evidence:

- Run ID: `run-1781041430565-62207`
- Scenario: `collect-and-exit-source-smoke`
- Verdict: `failed`
- Failing evidence: `/var/folders/x3/tc9lm8n5781gnw3cqdk7lcym0000gp/T//ouroforge-b3-dry-run-LUcO0c/runs/run-1781041430565-62207/evidence/scenarios/collect-and-exit-source-smoke/scenario-result-1781041440655.json`
- Representative assertions:
  - `sceneId` actual `foundation-scene`, expected `collect-and-exit-scene`.
  - `componentModel.counts.hudValue` actual `0`, expected greater than `0`.
  - `goalFlags.key_collected` actual `null`, expected `true`.
  - `goalFlags.exit_reached` actual `null`, expected `true`.
  - `animation_evidence` path `0.mode` actual `null`, expected `sprite_frame`.

Gap classification: `failed-classified`. B3 should be accepted as pipeline dry-run evidence if the verifier agrees the evidence is complete; downstream implementation work should address the failed scenario/runtime mismatch rather than treating this report as a production/store readiness pass.

## Non-goals and guardrails

- #1 and #23 remain open.
- No Era Q full-3D M102–M106 implementation or activation.
- No hosted/cloud/multi-user scope.
- No trusted browser/source writes.
- No auto-port, live bridge, foreign runtime embedding, or source-runtime reproduction.
- No release automation, signing, upload, publishing, credential automation, native export, Steam publishing, production readiness, or store readiness claim.
- No mutation apply was executed; mutation review was deferred only.
