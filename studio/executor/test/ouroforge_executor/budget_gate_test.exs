defmodule OuroforgeExecutor.BudgetGateTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.BudgetGate

  @fixture_root Path.expand("../../../../examples/producer-budget-gates-v1", __DIR__)

  defp fixture(name), do: Path.join(@fixture_root, name) |> BudgetGate.from_file!()

  defp continue_fixture do
    @fixture_root
    |> Path.join("human-gate.fixture.json")
    |> File.read!()
    |> String.replace(~s("status": "pending"), ~s("status": "approved"))
    |> BudgetGate.from_json!()
  end

  test "continues only while inside M43 budget and approved human gates" do
    decision = continue_fixture()

    assert decision.status == :continue
    refute BudgetGate.halt?(decision)
    assert decision.may_assign?
    assert decision.may_drive_cli?
    assert decision.condition_id == nil
    assert decision.diagnosis =~ "within budget"
    assert [%{boundary: :control_plane_only}] = decision.telemetry
  end

  test "halts assignment and CLI drive when iteration or cost budget is exhausted" do
    decision = fixture("budget-halt.fixture.json")

    assert decision.status == :halted_budget_exhausted
    assert BudgetGate.halt?(decision)
    refute decision.may_assign?
    refute decision.may_drive_cli?
    assert decision.stop_reason == "budget-exhausted"
    assert decision.condition_id == "stop-budget"
    assert decision.diagnosis =~ "budget exhausted"
    assert decision.diagnosis =~ decision.last_evidence_ref
  end

  test "blocks on pending mandatory human gates before checking no-progress" do
    decision = fixture("human-gate.fixture.json")

    assert decision.status == :blocked_human_gate
    assert decision.stop_reason == "human-approval-required"
    assert decision.condition_id == "stop-human"
    assert decision.pending_human_gate_ids == ["gate-vision"]
    refute decision.may_assign?
    refute decision.may_drive_cli?
  end

  test "records non-convergence diagnosis at the no-progress window" do
    decision = fixture("no-progress.fixture.json")

    assert decision.status == :stopped_no_progress
    assert decision.stop_reason == "no-progress"
    assert decision.condition_id == "stop-no-progress"
    assert decision.diagnosis =~ "no progress"
    assert decision.diagnosis =~ decision.last_evidence_ref
    refute decision.may_assign?
    refute decision.may_drive_cli?
  end

  test "fails closed on malformed M43 budget shape instead of inventing defaults" do
    assert_raise ArgumentError, ~r/maxIterations must be a positive integer/, fn ->
      %{
        "schemaVersion" => "producer-budget-gates-v1",
        "policyId" => "bad-budget",
        "orchestrationRef" => "demo",
        "budget" => %{"maxIterations" => 0, "maxCostUnits" => 1, "noProgressWindow" => 1},
        "usage" => %{
          "iterationCount" => 0,
          "costUnits" => 0,
          "noProgressSteps" => 0,
          "lastEvidenceRef" => "evidence/demo.json"
        },
        "stopConditions" => [
          %{"conditionId" => "stop-budget", "reason" => "budget-exhausted"},
          %{"conditionId" => "stop-human", "reason" => "human-approval-required"},
          %{"conditionId" => "stop-no-progress", "reason" => "no-progress"}
        ],
        "humanApprovalGates" => [%{"gateId" => "gate", "status" => "approved"}],
        "evidenceRefs" => ["evidence/demo.json"],
        "boundary" => "control-plane only"
      }
      |> BudgetGate.evaluate!()
    end
  end
end
