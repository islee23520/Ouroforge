# Studio Export Inspection Surface v1

Issue: #731
Roadmap anchor: #1 (Build / Export / Packaging milestone).
Status: read-only Studio inspection surface; no export behavior.

The Studio export inspection surface displays an exported export-evidence bundle
**read-only**. It shows the export target, output path, packaged asset count,
artifact checksum count, verification status and check count, the overall
verdict, and the always-blocked publish/release status, with a reference to the
export evidence bundle file.

## Boundary

- The browser/Studio surface cannot run export, publish, deploy, sign, upload,
  or any command. It is pure display of escaped exported JSON; no trusted-write
  or command-execution control is added.
- Publish/release targets are always blocked: only `web-local` and
  `web-static-bundle` are allowed local targets, and the surface always shows
  publish/deploy/sign/upload as blocked.
- Rust/local validation owns export validation, artifact writing, checksums, and
  evidence writing; the Studio surface remains read-only.

This surface makes no claim of production-ready export, secure distribution, or
Godot replacement. #1 and #23 remain open.
