defmodule OuroforgeExecutor.StageTakeoverTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.StageTakeover

  defp takeover_attrs(overrides \\ %{}) do
    Map.merge(
      %{
        stage_id: "stage-m80-art-pass",
        campaign_id: "campaign-m80",
        actor_id: "local-human",
        reason: "operator takes over the art pass with evidence capture"
      },
      overrides
    )
  end

  defp manual_work_attrs(overrides \\ %{}) do
    Map.merge(
      %{
        id: "manual-work-001",
        summary: "Adjusted the candidate scene draft and attached provenance for reverify.",
        actor_id: "local-human",
        base_ref: "runs/m80/stages/stage-m80-art-pass/before.json",
        evidence_ref: "runs/m80/stages/stage-m80-art-pass/manual-work.json",
        provenance_ref: "runs/m80/stages/stage-m80-art-pass/provenance.json"
      },
      overrides
    )
  end

  test "takeover captures executor session lock without trusted write authority" do
    server = start_supervised!({StageTakeover, name: unique_name()})

    assert {:ok, session} =
             StageTakeover.take_over(server, takeover_attrs(), runner: &__MODULE__.runner/3)

    assert session.status == :taken_over
    assert session.locked?
    assert session.trusted_write_authority? == false
    assert session.human_required_for_autonomous_loop? == false
    assert session.boundary =~ "intervention-as-evidence"
    assert session.boundary =~ "read + gated-write"
    assert session.boundary =~ "two-plane"
    assert session.boundary =~ "local-first"
    assert session.takeover_evidence_ref =~ "takeover_validate"
    assert session.takeover_evidence_ref =~ "takeover_record"
  end

  test "manual work is captured as evidence and reverified on handback" do
    server = start_supervised!({StageTakeover, name: unique_name()})

    assert {:ok, _session} =
             StageTakeover.take_over(server, takeover_attrs(), runner: &__MODULE__.runner/3)

    assert {:ok, captured} =
             StageTakeover.capture_manual_work(server, "stage-m80-art-pass", manual_work_attrs())

    assert length(captured.manual_work) == 1
    assert hd(captured.manual_work).status == :pending

    assert {:ok, handed_back} =
             StageTakeover.handback(server, "stage-m80-art-pass", runner: &__MODULE__.runner/3)

    assert handed_back.status == :handed_back
    refute handed_back.locked?
    assert handed_back.resumed?
    assert handed_back.trusted_write_authority? == false
    assert handed_back.handback_evidence_ref =~ "handback_validate"
    assert handed_back.handback_evidence_ref =~ "handback_record"

    [work] = handed_back.manual_work
    assert work.status == :accepted
    assert work.validation_evidence_ref =~ "manual_work_validate"
    assert work.record_evidence_ref =~ "manual_work_record"
    assert work.reverify_evidence_ref =~ "manual_work_reverify"
  end

  test "raw bypass and missing manual evidence fail closed" do
    server = start_supervised!({StageTakeover, name: unique_name()})

    assert {:error, :raw_bypass_forbidden} =
             StageTakeover.take_over(
               server,
               takeover_attrs(%{reason: "please raw_write_bypass this stage"}),
               runner: &__MODULE__.runner/3
             )

    assert {:ok, _session} =
             StageTakeover.take_over(server, takeover_attrs(), runner: &__MODULE__.runner/3)

    assert {:error, {:missing_ref, :evidence_ref}} =
             StageTakeover.capture_manual_work(
               server,
               "stage-m80-art-pass",
               manual_work_attrs(%{evidence_ref: ""})
             )
  end

  test "handback cannot resume until manual work is present and gates pass" do
    server = start_supervised!({StageTakeover, name: unique_name()})

    assert {:ok, _session} =
             StageTakeover.take_over(server, takeover_attrs(), runner: &__MODULE__.runner/3)

    assert {:error, :manual_work_required_for_handback} =
             StageTakeover.handback(server, "stage-m80-art-pass", runner: &__MODULE__.runner/3)

    assert {:ok, _captured} =
             StageTakeover.capture_manual_work(server, "stage-m80-art-pass", manual_work_attrs())

    assert {:error, {:manual_work_not_verified, "manual-work-001", _reason}} =
             StageTakeover.handback(server, "stage-m80-art-pass",
               runner: &__MODULE__.failing_runner/3
             )
  end

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase = option(argv, "--stage-phase")
    stage_id = option(argv, "--stage-id")
    work_id = option(argv, "--manual-work-id") || "session"
    {"evidenceRef=runs/#{campaign_id}/stages/#{stage_id}/#{work_id}-#{phase}.json", 0}
  end

  def failing_runner("ouroforge", ["loop", "step", _campaign_id | argv], _opts) do
    if option(argv, "--stage-phase") == "manual_work_reverify" do
      {"reverify failed", 2}
    else
      runner("ouroforge", ["loop", "step", "campaign-m80" | argv], [])
    end
  end

  defp option(argv, key) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^key, value] -> value
      _ -> nil
    end)
  end

  defp unique_name do
    Module.concat(__MODULE__, "Server#{System.unique_integer([:positive])}")
  end
end
