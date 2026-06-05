# Godot-Plus demo QA/playtest v1 (#788)

`examples/playable-demo-v2/collect-and-exit/qa-playtest-plan.json` defines a
bounded, read-only QA/playtest plan for the Collect-and-Exit demo. The plan maps
scenario-matrix rows to three local workers: deterministic win path, expected
hazard failure classification, and read-only integration surface audit.

The smoke `examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs`
runs the bounded local replays, writes a temporary generated evidence report
outside the repository, validates the expected negative-path classification, and
removes the report before exit.

Boundaries: no auto-fix, auto-apply, auto-merge, direct trusted source writes,
source mutation bypass, browser command bridge, arbitrary shell execution,
dependency install/update, credentialed operation, public deployment, release
signing/store publishing, executable plugin runtime, marketplace/network plugin
install, or Godot replacement/parity/production-ready claim. #1 and #23 remain
open.

Verification:

```bash
node --check examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs
```
