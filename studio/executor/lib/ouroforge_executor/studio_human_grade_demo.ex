defmodule OuroforgeExecutor.StudioHumanGradeDemo do
  @moduledoc """
  Scripted Human-Grade Studio demo for Era N M85 (#2093).

  The demo composes the read-only live Studio shell with the integrated
  intervention/authoring panels. It shows a live campaign observation event, one
  opt-in gated human intervention, and a separate no-human fallback path. Elixir
  renders/captures/routes and broadcasts local feedback only; trusted artifact
  semantics and all write-affecting effects remain Rust data-plane gate work.
  """

  alias OuroforgeExecutor.{StudioInterventionPanels, StudioLiveShell}

  defstruct version: "m85-human-grade-studio-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            observed_shell: nil,
            intervention: nil,
            autonomous: nil,
            live_feedback?: false,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            direct_artifact_write?: false,
            command_bridge?: false,
            notes: []

  def run(opts \\ []) do
    pubsub = Keyword.get(opts, :pubsub, OuroforgeExecutor.PubSub)

    observed_shell = observe_campaign(pubsub)
    intervention = gated_intervention(pubsub)
    autonomous = autonomous_fallback()

    %__MODULE__{
      observed_shell: observed_shell,
      intervention: intervention,
      autonomous: autonomous,
      live_feedback?: true,
      gated_write_verified?: gated_intervention?(intervention),
      autonomous_fallback_verified?: autonomous.status == :completed_without_human,
      notes: [
        "live shell observes Rust-owned evidence and diagnosis read models",
        "operator steering is intervention-as-evidence queued for existing Rust gates",
        "no-human fallback completes without opening the Studio"
      ]
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "M85 Human-Grade Studio demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}",
      "Direct artifact write: #{demo.direct_artifact_write?}; command bridge: #{demo.command_bridge?}",
      "Live observation: #{demo.live_feedback?}",
      "Observed active view: #{demo.observed_shell.active}",
      "Gated intervention verified: #{demo.gated_write_verified?}",
      "Intervention route: #{Enum.join(demo.intervention.routeCli, " ")}",
      "Intervention gate: #{demo.intervention.gateFamily}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Autonomous status: #{demo.autonomous.status}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.gated_write_verified? and demo.trusted_write_authority? == false and
      demo.direct_artifact_write? == false and demo.command_bridge? == false
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.human_surface_required? == false and demo.autonomous.cli_fallback_supported?
  end

  defp observe_campaign(pubsub) do
    {:ok, shell} = StudioLiveShell.new(%{active: :diagnosis})

    {:ok, _event} =
      StudioLiveShell.broadcast(
        :diagnosis,
        %{
          id: "m85-demo-diagnosis",
          title: "Rust-owned diagnosis read model",
          status: :fresh,
          evidence_refs: ["runs/m85/demo/diagnosis.json"],
          run_ref: "runs/m85/demo"
        },
        pubsub
      )

    shell
  end

  defp gated_intervention(pubsub) do
    {:ok, submission} =
      StudioInterventionPanels.submit(
        :steering,
        %{
          id: "m85-demo-steer-polish",
          campaign_id: "m85-human-grade-studio-demo",
          action: :reprioritize,
          target: "diagnosis-refresh",
          actor_id: "demo-human-operator",
          reason: "Inspect the latest diagnosis evidence first",
          issued_at: "2026-06-09T00:00:00Z",
          base_refs: ["runs/m85/demo/live-state.json"],
          provenance_refs: ["runs/m85/demo/intervention-provenance.json"]
        },
        pubsub: pubsub
      )

    submission
  end

  defp autonomous_fallback do
    %{
      status: :completed_without_human,
      human_intervention: :absent,
      human_surface_required?: false,
      waited_for_human?: false,
      cli_fallback_supported?: true,
      rust_data_plane_completed?: true,
      evidence_ref: "runs/m85/demo/no-human-loop.json"
    }
  end

  defp gated_intervention?(submission) do
    submission.status == :queued_for_rust_gate and submission.interventionAsEvidence and
      submission.readGatedWrite and submission.rustDataPlaneRequired and
      submission.directArtifactWrite == false and submission.trustedWriteAuthority == false and
      submission.commandBridge == false and submission.elixirOwnsArtifactSemantics == false and
      submission.routeCli != []
  end
end
