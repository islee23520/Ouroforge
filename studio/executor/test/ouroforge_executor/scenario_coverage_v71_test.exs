defmodule OuroforgeExecutor.ScenarioCoverageV71Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{StageTakeover, StageTakeoverDemo}

  @repo_root Path.expand("../../../..", __DIR__)
  @matrix_path Path.join(
                 @repo_root,
                 "examples/stage-takeover-handback-v1/scenario-coverage-v71/matrix.fixture.json"
               )

  defp decode_json!(path), do: path |> File.read!() |> OuroforgeExecutor.JSON.decode!()

  test "coverage v71 matrix records no-bypass and zero-human rows" do
    matrix = decode_json!(@matrix_path)
    assert matrix["schemaVersion"] == "scenario-coverage-v71-stage-takeover-handback-v1"
    assert matrix["coverageVersion"] == 71
    assert matrix["autonomyInvariants"]["humanInputRequired"] == false
    assert matrix["autonomyInvariants"]["loopCompletesWithoutHuman"] == true
    assert matrix["autonomyInvariants"]["manualWorkRequiresEvidence"] == true
    assert matrix["autonomyInvariants"]["handbackRequiresReverify"] == true
    assert matrix["autonomyInvariants"]["studioTrustedWriteAuthority"] == false
    assert matrix["autonomyInvariants"]["elixirOwnsArtifactSemantics"] == false
    assert matrix["autonomyInvariants"]["rustDataPlaneOwnsValidation"] == true

    row_ids = Enum.map(matrix["rows"], & &1["id"])

    for required <- [
          "stage-takeover-locks-local-session-only",
          "manual-work-captured-as-evidence",
          "handback-reverifies-through-rust-gates",
          "no-raw-bypass-from-elixir-control-plane",
          "loop-completes-without-human-input",
          "coverage-v71-boundaries"
        ] do
      assert required in row_ids
    end

    assert Enum.all?(matrix["rows"], &(&1["status"] == "pass"))
  end

  test "demo keeps handback gated and autonomous fallback available" do
    demo = StageTakeoverDemo.run(runner: &__MODULE__.runner/3)

    assert StageTakeoverDemo.autonomous_first?(demo)
    assert StageTakeoverDemo.read_gated_write?(demo)
    assert demo.trusted_write_authority? == false
    assert demo.autonomous.completed_without_human?
    refute demo.autonomous.waited_for_human?
    assert demo.takeover.status == :handed_back
    assert Enum.all?(demo.takeover.manual_work, &(&1.status == :accepted))
  end

  test "mandatory human and missing evidence regressions fail closed" do
    server = start_supervised!({StageTakeover, name: unique_name()})

    assert {:ok, session} =
             StageTakeover.take_over(server, takeover_attrs(), runner: &__MODULE__.runner/3)

    assert {:error, {:missing_ref, :evidence_ref}} =
             StageTakeover.capture_manual_work(
               server,
               session.stage_id,
               manual_work_attrs(%{evidence_ref: ""})
             )

    assert {:error, :manual_work_required_for_handback} =
             StageTakeover.handback(server, session.stage_id, runner: &__MODULE__.runner/3)
  end

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase = option(argv, "--stage-phase")
    stage_id = option(argv, "--stage-id")
    work_id = option(argv, "--manual-work-id") || "session"
    {"evidenceRef=runs/#{campaign_id}/stages/#{stage_id}/#{work_id}-#{phase}.json", 0}
  end

  defp takeover_attrs do
    %{
      stage_id: "stage-m80-v71",
      campaign_id: "campaign-m80-demo",
      actor_id: "local-human",
      reason: "operator takes over a bounded stage for v71 coverage"
    }
  end

  defp manual_work_attrs(overrides) do
    Map.merge(
      %{
        id: "manual-work-v71",
        summary: "Manual work captured as evidence for v71 handback reverify.",
        actor_id: "local-human",
        base_ref: "runs/m80/v71/before.json",
        evidence_ref: "runs/m80/v71/manual-work.json",
        provenance_ref: "runs/m80/v71/provenance.json"
      },
      overrides
    )
  end

  defp option(argv, key) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^key, value] -> value
      _ -> nil
    end)
  end

  defp unique_name, do: Module.concat(__MODULE__, "Server#{System.unique_integer([:positive])}")
end
