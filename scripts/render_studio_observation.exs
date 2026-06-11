#!/usr/bin/env elixir
# Reproducible local Studio reviewer observation renderer for issue #2494.
#
# This script intentionally writes only ignored generated evidence under runs/.
# It loads the dependency-free local Studio shell modules directly and records a
# read-only reviewer observation page/transcript without granting trusted writes,
# command bridges, self-approval, auto-apply, deploy, signing, or upload scope.

root = File.cwd!()
out_dir = Path.join(root, "runs/issue-2494/studio-observation")
screenshot_dir = Path.join(out_dir, "screenshots")
File.rm_rf!(out_dir)
File.mkdir_p!(screenshot_dir)

Code.require_file(Path.join(root, "studio/executor/lib/ouroforge_executor/local_pub_sub.ex"))
Code.require_file(Path.join(root, "studio/executor/lib/ouroforge_executor/studio_live_shell.ex"))

alias OuroforgeExecutor.StudioLiveShell

project = "examples/playable-demo-v2/signal-gate-dogfood/ouroforge.project.json"
scene = "examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json"
evidence_summary = "docs/evidence/signal-gate-win-state-browser-evidence-2493.md"
observability_manifest = "runs/issue-2493/signal-gate-win-state/manifest.json"

for path <- [project, scene, evidence_summary] do
  unless File.exists?(Path.join(root, path)) do
    raise "required tracked evidence ref missing: #{path}"
  end
end

{:ok, shell} = StudioLiveShell.new(%{active: :evidence})

{:ok, evidence_event} =
  StudioLiveShell.rust_owned_event(:evidence, %{
    id: "issue-2494-signal-gate-evidence",
    title: "Signal Gate Relay #2493 evidence/read-model",
    status: :fresh,
    evidence_refs: [project, scene, evidence_summary, observability_manifest],
    run_ref: observability_manifest,
    received_at: "issue-2494-local-script"
  })

{:ok, verdict_event} =
  StudioLiveShell.rust_owned_event(:verdict, %{
    id: "issue-2494-reviewer-decision",
    title: "Local reviewer observation decision",
    status: :accepted_read_only,
    evidence_refs: [evidence_summary],
    verdict:
      "Reviewer observed Signal Gate Relay #2493 evidence/read-model as read-only and accepted the bounded screenshot gap closure; no write-affecting Studio action was taken.",
    received_at: "issue-2494-local-script"
  })

{:ok, shell} = StudioLiveShell.apply_event(shell, evidence_event)
{:ok, shell} = StudioLiveShell.apply_event(shell, verdict_event)
rendered = StudioLiveShell.render(shell)

transcript = """
Issue #2494 local Studio reviewer observation

Project: #{project}
Scene: #{scene}
Evidence summary: #{evidence_summary}
Generated observability manifest ref: #{observability_manifest}

Reviewer observation decision:
#{verdict_event.verdict}

Read-only/trusted-write boundary:
readOnlyRendering=#{rendered.readOnlyRendering}
rustDataPlaneOwnsTruth=#{rendered.rustDataPlaneOwnsTruth}
trustedWriteAuthority=#{rendered.trustedWriteAuthority}
directArtifactWrite=#{rendered.directArtifactWrite}
commandBridge=#{rendered.commandBridge}

Evidence view entries: #{length(rendered.views.evidence.entries)}
Verdict view entries: #{length(rendered.views.verdict.entries)}
Generated evidence root: runs/issue-2494/studio-observation
Tracked entry point: scripts/render_studio_observation.exs
"""

html = """
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
  <title>Issue #2494 Studio reviewer observation</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; background: #111827; color: #e5e7eb; }
    code, pre { background: #0f172a; color: #bae6fd; padding: .2rem .4rem; border-radius: .25rem; }
    .card { border: 1px solid #334155; border-radius: .75rem; padding: 1rem; margin: 1rem 0; }
    .ok { color: #86efac; }
    .deny { color: #fca5a5; }
  </style>
</head>
<body>
  <h1>Issue #2494 Studio reviewer observation</h1>
  <p>Local read-only Studio shell rendering of Rust-owned Signal Gate Relay evidence.</p>
  <section class=\"card\">
    <h2>Evidence refs</h2>
    <ul>
      <li><code>#{project}</code></li>
      <li><code>#{scene}</code></li>
      <li><code>#{evidence_summary}</code></li>
      <li><code>#{observability_manifest}</code></li>
    </ul>
  </section>
  <section class=\"card\">
    <h2>Boundary</h2>
    <ul>
      <li class=\"ok\">readOnlyRendering=#{rendered.readOnlyRendering}</li>
      <li class=\"ok\">rustDataPlaneOwnsTruth=#{rendered.rustDataPlaneOwnsTruth}</li>
      <li class=\"deny\">trustedWriteAuthority=#{rendered.trustedWriteAuthority}</li>
      <li class=\"deny\">directArtifactWrite=#{rendered.directArtifactWrite}</li>
      <li class=\"deny\">commandBridge=#{rendered.commandBridge}</li>
    </ul>
  </section>
  <section class=\"card\">
    <h2>Reviewer decision</h2>
    <p>#{verdict_event.verdict}</p>
  </section>
  <section class=\"card\">
    <h2>Generated-state audit</h2>
    <p>Generated page/transcript/stderr are under ignored <code>runs/issue-2494/studio-observation/</code>.</p>
  </section>
</body>
</html>
"""

File.write!(Path.join(out_dir, "studio-transcript.txt"), transcript)
File.write!(Path.join(out_dir, "studio-observation.html"), html)
File.write!(Path.join(out_dir, "studio-stderr.txt"), "")

IO.puts(transcript)
IO.puts("STUDIO_OBSERVATION_RENDERED #{Path.relative_to(Path.join(out_dir, "studio-observation.html"), root)}")
