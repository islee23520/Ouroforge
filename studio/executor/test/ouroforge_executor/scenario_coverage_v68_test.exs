defmodule OuroforgeExecutor.ScenarioCoverageV68Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{LiveSteering, LiveSteeringDemo, SteeringDirective}

  @repo_root Path.expand("../../../..", __DIR__)
  @matrix_path Path.join(
                 @repo_root,
                 "examples/live-steering-directives-v1/scenario-coverage-v68/matrix.fixture.json"
               )

  defp decode_json!(path) do
    path
    |> File.read!()
    |> OuroforgeExecutor.JSON.decode!()
  end

  test "coverage v68 matrix records no-bypass and zero-human rows" do
    matrix = decode_json!(@matrix_path)
    assert matrix["schemaVersion"] == "scenario-coverage-v68-live-campaign-steering-directives-v1"
    assert matrix["coverageVersion"] == 68
    assert matrix["autonomyInvariants"]["humanInputRequired"] == false
    assert matrix["autonomyInvariants"]["loopCompletesWithoutHuman"] == true
    assert matrix["autonomyInvariants"]["studioTrustedWriteAuthority"] == false
    assert matrix["autonomyInvariants"]["elixirOwnsArtifactSemantics"] == false
    assert matrix["autonomyInvariants"]["rustDataPlaneOwnsValidation"] == true

    row_ids = Enum.map(matrix["rows"], & &1["id"])

    for required <- [
          "live-steering-directives-recorded-through-gates",
          "no-raw-bypass-from-elixir-control-plane",
          "loop-completes-without-human-input",
          "mandatory-human-regression-fails-closed",
          "pause-is-control-state-not-artifact-mutation",
          "coverage-v68-boundaries"
        ] do
      assert required in row_ids
    end

    assert Enum.all?(matrix["rows"], &(&1["status"] == "pass"))
  end

  test "demo keeps intervention gated and autonomous fallback available" do
    demo = LiveSteeringDemo.run(runner: &__MODULE__.runner/3)

    assert LiveSteeringDemo.autonomous_first?(demo)
    assert LiveSteeringDemo.read_gated_write?(demo)
    assert demo.trusted_write_authority? == false
    assert demo.autonomous.accepted_directives == []
    assert demo.autonomous.ready_task_ids != []
    assert Enum.all?(demo.steered.accepted_directives, &(&1.status == :accepted))
    assert Enum.all?(demo.steered.evidence_refs, &String.contains?(&1, "directives"))
  end

  test "mandatory human and raw directive regressions fail before scheduling trust" do
    invalid = %{
      SteeringDirective.new(%{
        id: "v68-invalid-raw",
        campaign_id: "m77-live-steering-demo",
        action: :raw_write,
        target: "artifact://scene",
        reason: "try to bypass the gate",
        actor_id: "human-operator",
        issued_at: "2026-06-09T00:00:00Z",
        base_refs: ["runs/m77/live-state.json"]
      })
      | action: :raw_write
    }

    assert {:error, _} = SteeringDirective.validate(invalid)

    paused =
      LiveSteering.consume(LiveSteeringDemo.fixture_plan(), [pause_directive()],
        runner: &__MODULE__.runner/3
      )

    assert paused.paused?
    assert paused.trusted_write_authority? == false
    assert paused.ready_task_ids == []
    assert Enum.any?(paused.notes, &String.contains?(&1, "not artifact mutation"))
  end

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase = option(argv, "--directive-phase")
    id = option(argv, "--directive-id")
    {"evidenceRef=runs/#{campaign_id}/directives/#{id}-#{phase}.json", 0}
  end

  defp option(argv, key) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^key, value] -> value
      _ -> nil
    end)
  end

  defp pause_directive do
    SteeringDirective.new(%{
      id: "v68-pause-control-state",
      campaign_id: "m77-live-steering-demo",
      action: :pause,
      reason: "operator pauses to inspect recorded evidence",
      actor_id: "human-operator",
      issued_at: "2026-06-09T00:00:00Z",
      base_refs: ["runs/m77/live-state.json"]
    })
  end
end
