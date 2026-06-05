# Studio Export / Package Inspection Panel v1

Issue: #767
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only export/package evidence inspection; no export/publish/deploy
execution.

The export / package inspection panel integrates Build / Export / Packaging
evidence into Studio **read-only**. It consumes existing export evidence and
package descriptors and never executes export, publish, deploy, sign, upload, or
arbitrary commands. Execution stays in trusted CLI/harness paths.

## What it shows

- Export profile and export plan steps.
- Package status.
- Artifacts with checksums, sizes, and per-artifact status.
- Verification verdict.
- Generated-state warnings.
- Publish/release blocked status.

## Boundary

- **No export / publish / deploy / sign / upload / command execution.** The panel
  is pure display of escaped exported JSON. There are no run/publish/deploy/sign/
  upload controls, and no export/publish functions are exported. Publish/release
  is shown as blocked from Studio even when trusted evidence reports it allowed.
- Rust/local trusted CLI/harness owns export execution, packaging, verification,
  and trusted file boundaries.
- No claim of native/mobile/store export, release automation, production-ready
  editor, Godot replacement, or full Godot editor parity.
- Governance issues #1 and #23 remain open.
