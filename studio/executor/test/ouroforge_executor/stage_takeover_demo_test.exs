defmodule OuroforgeExecutor.StageTakeoverDemoTest do
  use ExUnit.Case, async: false
  @moduletag :demo

  alias OuroforgeExecutor.StageTakeoverDemo

  test "M80 demo proves gated manual work and autonomous fallback" do
    demo = StageTakeoverDemo.run(runner: &__MODULE__.runner/3)

    assert demo.version == "m80-stage-takeover-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert StageTakeoverDemo.autonomous_first?(demo)
    assert StageTakeoverDemo.read_gated_write?(demo)
    refute demo.trusted_write_authority?
    assert demo.autonomous.status == :completed_without_human
    refute demo.autonomous.waited_for_human?
    refute demo.autonomous.human_surface_required?

    assert demo.takeover.status == :handed_back
    refute demo.takeover.locked?
    assert demo.takeover.resumed?
    assert length(demo.takeover.manual_work) == 1
    assert hd(demo.takeover.manual_work).status == :accepted
  end

  test "M80 rendered demo is conservative and names the gate path" do
    rendered =
      StageTakeoverDemo.run(runner: &__MODULE__.runner/3)
      |> Map.fetch!(:rendered)

    assert rendered =~ "M80 stage takeover and handback demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Gated write verified: true"
    assert rendered =~ "intervention-as-evidence routed through Rust CLI gates"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase = option(argv, "--stage-phase")
    stage_id = option(argv, "--stage-id")
    work_id = option(argv, "--manual-work-id") || "session"
    {"evidenceRef=runs/#{campaign_id}/stages/#{stage_id}/#{work_id}-#{phase}.json", 0}
  end

  defp option(argv, key) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^key, value] -> value
      _ -> nil
    end)
  end
end
