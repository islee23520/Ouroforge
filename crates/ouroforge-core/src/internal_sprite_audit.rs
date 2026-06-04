use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Sentinel used in place of the real local reference root in every emitted
/// surface (report fields and error diagnostics) so the audit never leaks the
/// operator's local path. See #979: redact the reference root in all output.
const REDACTED_REFERENCE_ROOT: &str = "internal-local-root";

const RO_VIBE_REQUIRED_FILES: &[&str] = &[
    "ro-sprites-anim/body/male/Novice_job0/act0_dir0_f0.png",
    "ro-sprites-anim/body/male/Novice_job0/act1_dir0_f0.png",
    "ro-sprites-anim/body/female/Novice_job0/act0_dir0_f0.png",
    "ro-sprites-anim/body/female/Novice_job0/act1_dir0_f0.png",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalSpriteAuditProfile {
    RoVibeV1,
}

impl InternalSpriteAuditProfile {
    pub const fn id(self) -> &'static str {
        match self {
            Self::RoVibeV1 => "ro-vibe-v1",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "ro-vibe-v1" => Ok(Self::RoVibeV1),
            other => Err(anyhow!(
                "unsupported internal sprite audit profile: {other}"
            )),
        }
    }

    const fn required_files(self) -> &'static [&'static str] {
        match self {
            Self::RoVibeV1 => RO_VIBE_REQUIRED_FILES,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalSpriteAuditReport {
    pub profile: String,
    pub reference_root: String,
    pub reference_root_redacted: bool,
    pub distribution_policy: InternalSpriteDistributionPolicy,
    pub inventory: InternalSpriteInventory,
    pub render_readiness: InternalSpriteRenderReadiness,
    pub missing_required_files: Vec<String>,
    pub issue_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalSpriteDistributionPolicy {
    pub license_scope: String,
    pub git_commit_allowed: bool,
    pub screenshot_allowed: bool,
    pub upload_allowed: bool,
    pub copied_private_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalSpriteInventory {
    pub png_frames: usize,
    pub required_files: usize,
    pub present_required_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalSpriteRenderReadiness {
    pub status: String,
    pub ready: bool,
}

pub fn audit_internal_sprite_reference(
    reference_root: &Path,
    profile: InternalSpriteAuditProfile,
) -> Result<InternalSpriteAuditReport> {
    let metadata = fs::metadata(reference_root).with_context(|| {
        format!("internal sprite reference root not readable: {REDACTED_REFERENCE_ROOT}")
    })?;
    if !metadata.is_dir() {
        return Err(anyhow!(
            "internal sprite reference root is not a directory: {REDACTED_REFERENCE_ROOT}"
        ));
    }

    let required_files = profile.required_files();
    let missing_required_files = required_files
        .iter()
        .filter(|relative| !reference_root.join(relative).is_file())
        .map(|relative| (*relative).to_owned())
        .collect::<Vec<_>>();
    let ready = missing_required_files.is_empty();
    let issue_notes = missing_required_files
        .iter()
        .map(|relative| format!("Missing internal sprite reference for {relative}"))
        .collect::<Vec<_>>();

    Ok(InternalSpriteAuditReport {
        profile: profile.id().to_owned(),
        reference_root: REDACTED_REFERENCE_ROOT.to_owned(),
        reference_root_redacted: true,
        distribution_policy: InternalSpriteDistributionPolicy {
            license_scope: "internal-use-only".to_owned(),
            git_commit_allowed: false,
            screenshot_allowed: false,
            upload_allowed: false,
            copied_private_files: 0,
        },
        inventory: InternalSpriteInventory {
            png_frames: count_png_frames(reference_root)?,
            required_files: required_files.len(),
            present_required_files: required_files.len() - missing_required_files.len(),
        },
        render_readiness: InternalSpriteRenderReadiness {
            status: if ready { "ready" } else { "blocked" }.to_owned(),
            ready,
        },
        missing_required_files,
        issue_notes,
    })
}

fn count_png_frames(root: &Path) -> Result<usize> {
    let mut pending = vec![root.to_path_buf()];
    let mut png_frames = 0usize;
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(&path).with_context(|| {
            // Redact the operator's local root; keep only the (already-public)
            // path relative to it so diagnostics stay useful without leaking.
            let shown = path
                .strip_prefix(root)
                .ok()
                .filter(|relative| !relative.as_os_str().is_empty())
                .map(|relative| format!("{REDACTED_REFERENCE_ROOT}/{}", relative.display()))
                .unwrap_or_else(|| REDACTED_REFERENCE_ROOT.to_owned());
            format!("internal sprite directory not readable: {shown}")
        })? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if is_png_frame(&path) {
                png_frames += 1;
            }
        }
    }
    Ok(png_frames)
}

fn is_png_frame(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| !name.starts_with("._"))
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
}
