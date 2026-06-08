defmodule OuroforgeExecutor.OperatorCockpit.TelemetryPanelTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.TelemetryPanel

  test "M67-5 shows local queue, concurrency, budget, retry, and backpressure telemetry" do
    panel = TelemetryPanel.fixture(:backpressured)

    assert panel.version == "m67-5"
    assert panel.boundary == :read_only_local_telemetry_panel
    assert panel.queue_depth == 4
    assert panel.active_workers == 3
    assert panel.max_concurrency == 3
    assert panel.utilization == 1.0
    assert panel.backpressure == %{depth: 4, state: :backpressured}
    refute panel.remote_telemetry?
    refute panel.trusted_write_authority?
  end

  test "M67-5 fixtures cover waiting, retrying, budget-limited, backpressured, and blocked stop gates" do
    fixtures = TelemetryPanel.fixtures()

    assert fixtures.waiting.queue_depth == 2
    assert fixtures.retrying.retries.attempts == 2
    assert fixtures.budget_limited.budget.state == :exhausted_requires_human_judgment
    assert fixtures.budget_limited.stop_gate.human_judgment_required?
    assert fixtures.backpressured.backpressure.state == :backpressured
    assert fixtures.blocked.stop_gate.decision == :human_decision_required
  end

  test "M67-5 treats unknown stop-gate strings as untrusted human-review input" do
    panel =
      TelemetryPanel.from_inputs(
        OuroforgeExecutor.OperatorCockpit.CampaignStatus.fixture(:normal),
        OuroforgeExecutor.OperatorCockpit.TaskDAG.fixture(:waiting),
        %{"stopGate" => "not-a-known-stop-gate"}
      )

    assert panel.stop_gate.decision == :unknown_untrusted_input
    assert panel.stop_gate.human_judgment_required?
  end

  test "M67-5 render is operator-friendly and local-only" do
    rendered = :budget_limited |> TelemetryPanel.fixture() |> TelemetryPanel.render()

    assert rendered =~ "local read-only utilization"
    assert rendered =~ "Budget: 10/10 (exhausted_requires_human_judgment)"
    assert rendered =~ "Stop gate: budget_exhausted; human judgment=true"
    assert rendered =~ "Remote telemetry: false; trusted writes: false"
    refute rendered =~ "remote worker"
    refute rendered =~ "hosted dashboard"
    refute rendered =~ "write evidence"
  end
end
