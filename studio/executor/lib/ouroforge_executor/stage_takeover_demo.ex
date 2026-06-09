defmodule OuroforgeExecutor.StageTakeoverDemo do
  @moduledoc """
  Scripted M80 stage takeover and handback demo.

  The demo proves two paths:

    * autonomous default: no human takes over, so the stage loop completes
      without the Studio surface;
    * human takeover: a local operator locks one stage, manual work is captured
      as evidence/provenance metadata, handback re-verifies through Rust CLI gate
      phases, and the executor resumes without Elixir artifact write authority.
  """

  alias OuroforgeExecutor.StageTakeover

  defstruct version: "m80-stage-takeover-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            autonomous: nil,
            takeover: nil,
            rendered: nil,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            notes: []

  def run(opts) do
    runner = Keyword.fetch!(opts, :runner)
    {:ok, server} = StageTakeover.start_link([])

    autonomous = autonomous_default_demo()
    {:ok, taken_over} = StageTakeover.take_over(server, takeover_attrs(), runner: runner)

    {:ok, captured} =
      StageTakeover.capture_manual_work(server, taken_over.stage_id, manual_work_attrs())

    {:ok, handed_back} = StageTakeover.handback(server, captured.stage_id, runner: runner)

    demo = %__MODULE__{
      autonomous: autonomous,
      takeover: handed_back,
      gated_write_verified?: gated_write_verified?(handed_back),
      autonomous_fallback_verified?: autonomous_fallback_verified?(autonomous),
      notes: [
        "autonomous fallback completes without the human surface",
        "manual work is intervention-as-evidence routed through Rust CLI gates",
        "Elixir stores local session state only; Rust remains data-plane truth"
      ]
    }

    %__MODULE__{demo | rendered: render(demo)}
  end

  def autonomous_default_demo do
    %{
      demo_id: "m80-stage-takeover-autonomous-default",
      stage_id: "stage-m80-art-pass",
      status: :completed_without_human,
      completed_without_human?: true,
      waited_for_human?: false,
      human_surface_required?: false,
      cli_fallback_supported?: true,
      trusted_write_performed?: false,
      evidence_refs: [
        "runs/m80/demo/autonomous-stage-completed.json",
        "runs/m80/demo/no-human-surface-required.json"
      ]
    }
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.trusted_write_authority? == false and demo.gated_write_verified? and
      demo.takeover.status == :handed_back and demo.takeover.resumed? and
      Enum.all?(demo.takeover.manual_work, &(&1.status == :accepted))
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.human_surface_required? == false
  end

  def render(%__MODULE__{} = demo) do
    work_refs =
      demo.takeover.manual_work
      |> Enum.flat_map(
        &[&1.validation_evidence_ref, &1.record_evidence_ref, &1.reverify_evidence_ref]
      )
      |> Enum.join(", ")

    [
      "M80 stage takeover and handback demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Gated write verified: #{demo.gated_write_verified?}",
      "Stage status after handback: #{demo.takeover.status}; resumed: #{demo.takeover.resumed?}",
      "Manual work evidence refs: #{work_refs}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  defp autonomous_fallback_verified?(autonomous) do
    Map.get(autonomous, :completed_without_human?) == true and
      Map.get(autonomous, :waited_for_human?) == false
  end

  defp gated_write_verified?(session) do
    session.trusted_write_authority? == false and session.status == :handed_back and
      session.resumed? and session.handback_evidence_ref not in [nil, ""] and
      session.manual_work != [] and
      Enum.all?(session.manual_work, fn work ->
        work.status == :accepted and present?(work.validation_evidence_ref) and
          present?(work.record_evidence_ref) and present?(work.reverify_evidence_ref)
      end)
  end

  defp present?(value), do: is_binary(value) and value != ""

  defp takeover_attrs do
    %{
      stage_id: "stage-m80-art-pass",
      campaign_id: "campaign-m80-demo",
      actor_id: "local-human",
      reason: "operator takes over a bounded stage to attach manual evidence"
    }
  end

  defp manual_work_attrs do
    %{
      id: "manual-work-m80-demo-001",
      summary: "Manual stage work captured with before/after evidence for reverify.",
      actor_id: "local-human",
      base_ref: "runs/m80/demo/stage-m80-art-pass.before.json",
      evidence_ref: "runs/m80/demo/stage-m80-art-pass.manual-work.json",
      provenance_ref: "runs/m80/demo/stage-m80-art-pass.provenance.json"
    }
  end
end
