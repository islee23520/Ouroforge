defmodule OuroforgeExecutor.OperatorCockpit.DemoTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.Demo

  def runner("ouroforge", argv, _opts), do: {"#{Enum.join(argv, " ")} valid\n", 0}

  test "M67-7 composes all cockpit panels into a read-only local demo" do
    demo =
      Demo.run(
        runner: &__MODULE__.runner/3,
        state: :blocked,
        telemetry: %{stop_gate: :human_decision_required}
      )

    assert demo.version == "m67-7"
    assert demo.boundary == :minimal_read_only_executor_cockpit_demo
    assert demo.panels == [:contract, :campaign_status, :task_dag, :runbook, :telemetry, :parity]
    assert Demo.read_only?(demo)
    assert Demo.golden_parity?(demo)
    assert demo.campaign_status.status == :blocked_requires_human_judgment
    assert demo.runbook.human_judgment_required?
  end

  test "M67-7 rendered demo is operator-readable and contains manual parity truth" do
    rendered =
      Demo.run(
        runner: &__MODULE__.runner/3,
        state: :backpressured,
        telemetry: %{backpressure_depth: 2, active_workers: 2, max_concurrency: 2}
      )
      |> Demo.render()

    assert rendered =~ "M67 minimal read-only executor cockpit demo"
    assert rendered =~ "Campaign m67-campaign-status-fixture"
    assert rendered =~ "Task DAG m67-task-dag-fixture"
    assert rendered =~ "Runbook m67-campaign-status-fixture"
    assert rendered =~ "Telemetry m67-campaign-status-fixture"
    assert rendered =~ "Parity panel: read-only golden/manual comparison"
    assert rendered =~ "Remote telemetry: false; trusted writes: false"
    assert rendered =~ "Manual fallback commands: ouroforge seed validate"
    refute rendered =~ "click to run"
    refute rendered =~ "write ledger"
    refute rendered =~ "approve automatically"
  end
end
