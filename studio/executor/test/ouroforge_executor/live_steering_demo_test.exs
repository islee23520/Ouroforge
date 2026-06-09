defmodule OuroforgeExecutor.LiveSteeringDemoTest do
  use ExUnit.Case, async: false
  @moduletag :demo

  alias OuroforgeExecutor.LiveSteeringDemo

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase = find_arg(argv, "--directive-phase")
    id = find_arg(argv, "--directive-id")
    {"evidenceRef=runs/#{campaign_id}/directives/#{id}-#{phase}.json", 0}
  end

  defp find_arg(argv, key) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^key, value] -> value
      _ -> nil
    end)
  end

  test "M77 demo proves gated human steering and autonomous fallback" do
    demo = LiveSteeringDemo.run(runner: &__MODULE__.runner/3)

    assert demo.version == "m77-live-steering-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert LiveSteeringDemo.read_gated_write?(demo)
    assert LiveSteeringDemo.autonomous_first?(demo)

    assert demo.autonomous.ready_task_ids == [
             "01-broad-approach",
             "02-fast-prototype",
             "03-polish"
           ]

    assert demo.steered.ready_task_ids == ["03-polish", "02-fast-prototype"]
    assert length(demo.steered.accepted_directives) == 2
    refute demo.trusted_write_authority?
  end

  test "M77 rendered demo is conservative and does not imply raw writes" do
    rendered =
      LiveSteeringDemo.run(runner: &__MODULE__.runner/3)
      |> LiveSteeringDemo.render()

    assert rendered =~ "M77 live steering directives demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Gated write verified: true"
    assert rendered =~ "intervention-as-evidence routed through the Rust CLI"
    refute rendered =~ "write ledger directly"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end
end
