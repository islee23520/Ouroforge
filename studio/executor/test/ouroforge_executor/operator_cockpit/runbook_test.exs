defmodule OuroforgeExecutor.OperatorCockpit.RunbookTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.Runbook

  test "M67-4 translates blocked states into human-readable causes" do
    runbook = Runbook.fixture(:blocked)

    assert runbook.version == "m67-4"
    assert runbook.boundary == :read_only_blocked_reason_runbook
    assert runbook.blocked?
    assert Enum.any?(runbook.reasons, &(&1.code == :human_decision))
    assert Enum.any?(runbook.reasons, &(&1.code == :kernel_failure))
    assert runbook.human_judgment_required?
    assert Runbook.copy_only?(runbook)
  end

  test "M67-4 fixtures cover waiting, retrying, budget, backpressure, and blocked reasons" do
    fixtures = Runbook.fixtures()

    assert Enum.any?(fixtures.waiting.reasons, &(&1.code == :dependency_wait))
    assert Enum.any?(fixtures.retrying.reasons, &(&1.code == :retry_backoff))
    assert Enum.any?(fixtures.budget_limited.reasons, &(&1.code == :budget_exhausted))
    assert Enum.any?(fixtures.backpressured.reasons, &(&1.code == :backpressure))
    assert Enum.any?(fixtures.blocked.reasons, &(&1.code == :human_decision))
  end

  test "M67-4 render is copy-only and forbids browser/UI execution" do
    rendered = :blocked |> Runbook.fixture() |> Runbook.render()

    assert rendered =~ "read-only blocked reason surface"
    assert rendered =~ "Human judgment required: true"
    assert rendered =~ "Trusted writes: false"
    assert rendered =~ "Executable actions: 0"
    assert rendered =~ "Copy-only suggestions:"
    assert rendered =~ "manual ouroforge CLI"
    refute rendered =~ "click to run"
    refute rendered =~ "execute command"
    refute rendered =~ "approve automatically"
  end
end
