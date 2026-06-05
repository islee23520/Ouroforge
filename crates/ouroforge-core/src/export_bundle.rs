//! Web Export Bundle v1 (#723).
//!
//! Assembles a local, runnable static web bundle from a validated
//! [`ExportPlan`]. Assembly is plan-driven: it copies only the entry scene and
//! the asset roots the plan declares (it does not discover arbitrary files),
//! writes every output under the caller-provided staging root (never outside
//! it), refuses blocked source paths, and emits an HTML/CSS/runtime bootstrap
//! that preserves the `window.__OUROFORGE__` runtime probe wiring.
//!
//! This slice does not produce the rich asset manifest (#724) or checksums
//! (#727); it produces the runnable bundle skeleton those slices build on.

use crate::export_plan::{ExportPlan, PlannedInputKind};
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Result of assembling a bundle. Paths are package-relative (relative to the
/// bundle root) and sorted for deterministic reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleReport {
    pub bundle_root: PathBuf,
    pub package_files: Vec<String>,
    pub asset_files: Vec<String>,
    pub entry_scene_package_path: String,
    pub probe_mode: String,
}

/// Assemble the web bundle described by `plan`. Source inputs are resolved
/// relative to `repo_root`; all outputs are written under `staging_root`, which
/// becomes the bundle root.
pub fn assemble_web_bundle(
    plan: &ExportPlan,
    repo_root: &Path,
    staging_root: &Path,
) -> Result<BundleReport> {
    fs::create_dir_all(staging_root)
        .with_context(|| format!("failed to create staging root {}", staging_root.display()))?;
    let staging_root = staging_root
        .canonicalize()
        .with_context(|| format!("failed to resolve staging root {}", staging_root.display()))?;

    let mut written: BTreeMap<String, ()> = BTreeMap::new();
    let mut asset_files: Vec<String> = Vec::new();

    // 1. Copy the entry scene (a declared source input) into scene/<basename>.
    let entry_source = plan
        .source_inputs
        .iter()
        .find(|i| i.kind == PlannedInputKind::EntryScene)
        .ok_or_else(|| anyhow!("export plan has no entry scene input"))?;
    refuse_blocked_source(plan, &entry_source.path)?;
    let entry_basename = last_segment(&entry_source.path);
    let entry_package_path = format!("scene/{entry_basename}");
    let entry_bytes = fs::read(repo_root.join(&entry_source.path))
        .with_context(|| format!("failed to read entry scene {}", entry_source.path))?;
    staged_write(&staging_root, &entry_package_path, &entry_bytes)?;
    written.insert(entry_package_path.clone(), ());

    // 2. Copy each declared asset root into assets/<segment>/<relpath>.
    for input in plan
        .source_inputs
        .iter()
        .filter(|i| i.kind == PlannedInputKind::AssetRoot)
    {
        refuse_blocked_source(plan, &input.path)?;
        let segment = last_segment(&input.path);
        let source_dir = repo_root.join(&input.path);
        for rel in collect_files(&source_dir)? {
            let source_rel = format!("{}/{}", input.path.trim_end_matches('/'), rel);
            if is_blocked(plan, &source_rel) || rel.split('/').any(|p| p.starts_with('.')) {
                continue;
            }
            let package_path = format!("assets/{segment}/{rel}");
            let bytes = fs::read(source_dir.join(&rel))
                .with_context(|| format!("failed to read asset {source_rel}"))?;
            staged_write(&staging_root, &package_path, &bytes)?;
            written.insert(package_path.clone(), ());
            asset_files.push(package_path);
        }
    }
    asset_files.sort();

    // 3. Write the generated HTML / CSS / runtime bootstrap. The bootstrap
    //    preserves the window.__OUROFORGE__ probe surface and loads the entry
    //    scene plus the copied asset list (no arbitrary discovery at runtime).
    let bootstrap = render_bootstrap(&entry_package_path, &asset_files);
    staged_write(&staging_root, "runtime/bootstrap.js", bootstrap.as_bytes())?;
    written.insert("runtime/bootstrap.js".to_string(), ());

    staged_write(&staging_root, "styles.css", STYLES_CSS.as_bytes())?;
    written.insert("styles.css".to_string(), ());

    let html = render_index_html(&plan.export_target);
    staged_write(&staging_root, "index.html", html.as_bytes())?;
    written.insert("index.html".to_string(), ());

    Ok(BundleReport {
        bundle_root: staging_root,
        package_files: written.into_keys().collect(),
        asset_files,
        entry_scene_package_path: entry_package_path,
        probe_mode: "preserve".to_string(),
    })
}

/// Write `bytes` to `package_rel` under `staging_root`, guaranteeing the
/// resolved path stays inside the bundle.
fn staged_write(staging_root: &Path, package_rel: &str, bytes: &[u8]) -> Result<()> {
    let rel = Path::new(package_rel);
    if rel.is_absolute()
        || package_rel.contains('\\')
        || rel.components().any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(anyhow!(
            "refusing to write unsafe package path `{package_rel}`"
        ));
    }
    let target = staging_root.join(rel);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    // The parent now exists and is created under staging_root; confirm it.
    let resolved_parent = target
        .parent()
        .expect("write target has a parent")
        .canonicalize()
        .with_context(|| format!("failed to resolve parent for {package_rel}"))?;
    if !resolved_parent.starts_with(staging_root) {
        return Err(anyhow!(
            "refusing to write `{package_rel}` outside the staging root"
        ));
    }
    fs::write(&target, bytes).with_context(|| format!("failed to write {}", target.display()))?;
    Ok(())
}

fn refuse_blocked_source(plan: &ExportPlan, source_rel: &str) -> Result<()> {
    if is_blocked(plan, source_rel) {
        return Err(anyhow!(
            "export bundle refuses blocked source path `{source_rel}`"
        ));
    }
    Ok(())
}

fn is_blocked(plan: &ExportPlan, source_rel: &str) -> bool {
    let normalized = source_rel.trim_start_matches("./");
    plan.blocked_files.iter().any(|prefix| {
        normalized == prefix.trim_end_matches('/') || normalized.starts_with(prefix.as_str())
    })
}

/// Deterministically collect file paths under `dir`, relative to `dir`, sorted.
fn collect_files(dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    collect_into(dir, dir, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_into(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("failed to read dir {}", dir.display()))?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::result::Result<_, _>>()?;
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_into(root, &path, out)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .expect("entry is under root")
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

fn last_segment(path: &str) -> &str {
    path.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(path)
}

const STYLES_CSS: &str = "html,body{margin:0;height:100%;background:#101018;color:#dbe9ee;\
font-family:system-ui,sans-serif}#stage{display:block;margin:0 auto;image-rendering:pixelated}\n";

fn render_index_html(export_target: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n\
<title>Ouroforge local export ({target})</title>\n\
<meta name=\"ouroforge-export-target\" content=\"{target}\">\n\
<link rel=\"stylesheet\" href=\"styles.css\">\n</head>\n<body>\n\
<canvas id=\"stage\" width=\"320\" height=\"180\"></canvas>\n\
<script src=\"runtime/bootstrap.js\"></script>\n</body>\n</html>\n",
        target = export_target
    )
}

/// Render the runtime bootstrap. It exposes the v1 probe surface
/// (`window.__OUROFORGE__`) and loads the entry scene plus the packaged asset
/// list without discovering arbitrary files at runtime.
fn render_bootstrap(entry_scene_package_path: &str, asset_files: &[String]) -> String {
    let asset_array = asset_files
        .iter()
        .map(|a| format!("    {}", js_string(a)))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "// Generated by Ouroforge local web export (#723). Read-only local bundle:\n\
// no publish, deploy, sign, upload, network, or arbitrary command execution.\n\
'use strict';\n\
(function () {{\n\
  const ENTRY_SCENE = {entry};\n\
  const ASSET_FILES = [\n{assets}\n  ];\n\
  const events = [];\n\
  let tick = 0;\n\
  let paused = false;\n\
  let scene = null;\n\
  const clone = (value) => JSON.parse(JSON.stringify(value));\n\
  const record = (type, data) => {{ events.push({{ type, tick, data: data || null }}); }};\n\
  function worldState() {{\n\
    return clone({{ tick, paused, sceneId: scene && scene.sceneId, entry: ENTRY_SCENE, assets: ASSET_FILES }});\n\
  }}\n\
  function step(steps) {{\n\
    const count = Math.max(1, steps | 0);\n\
    if (!paused) tick += count;\n\
    record('probe.step', {{ count }});\n\
    return worldState();\n\
  }}\n\
  const probe = Object.freeze({{\n\
    probeVersion: 'ouroforge.export-runtime-probe.v1',\n\
    getWorldState() {{ return worldState(); }},\n\
    getFrameStats() {{ return clone({{ tick, paused, eventCount: events.length }}); }},\n\
    getEvents() {{ return clone(events); }},\n\
    step,\n\
    pause() {{ paused = true; record('probe.paused'); return this.getFrameStats(); }},\n\
    resume() {{ paused = false; record('probe.resumed'); return this.getFrameStats(); }},\n\
    setInput(next) {{ record('probe.input', next || {{}}); return worldState(); }},\n\
    snapshot() {{ return clone({{ tick, paused, events }}); }},\n\
    restore(snap) {{ if (snap) {{ tick = snap.tick | 0; paused = Boolean(snap.paused); }} return worldState(); }},\n\
  }});\n\
  const globalScope = typeof window !== 'undefined' ? window : globalThis;\n\
  globalScope.__OUROFORGE__ = probe;\n\
  async function boot() {{\n\
    try {{\n\
      const res = await fetch(ENTRY_SCENE);\n\
      scene = await res.json();\n\
      record('export.scene.loaded', {{ sceneId: scene.sceneId }});\n\
    }} catch (err) {{\n\
      record('export.scene.error', {{ message: String(err) }});\n\
    }}\n\
  }}\n\
  if (typeof fetch === 'function') {{ boot(); }}\n\
}})();\n",
        entry = js_string(entry_scene_package_path),
        assets = asset_array,
    )
}

/// Minimal JSON-string encoder for embedding paths safely in generated JS.
fn js_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}
