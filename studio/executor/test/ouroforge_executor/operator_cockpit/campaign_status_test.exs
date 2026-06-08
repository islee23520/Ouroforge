defmodule OuroforgeExecutor.OperatorCockpit.CampaignStatusTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.CampaignStatus

  test "M67-2 exposes current campaign state without trusted write authority" do
    model = CampaignStatus.fixture(:normal)

    assert model.version == "m67-2"
    assert model.boundary == :read_only_campaign_status
    assert model.status == :running
    assert model.active_task.task_id == "project"
    assert model.active_task.source == :executor_control_plane_observation
    assert model.evidence_refs == ["runs/m67/seed.json"]
    assert model.sources == [:executor_state, :ouroforge_cli_output]
    refute model.trusted_write_authority?
  end

  test "M67-2 fixtures cover normal, waiting, retrying, budget, backpressure, and blocked states" do
    fixtures = CampaignStatus.fixtures()

    assert fixtures.normal.status == :running
    assert fixtures.waiting.status == :waiting
    assert fixtures.retrying.status == :retrying
    assert fixtures.budget_limited.status == :budget_limited_requires_human_judgment
    assert fixtures.backpressured.status == :backpressured
    assert fixtures.blocked.status == :blocked_requires_human_judgment
    assert fixtures.blocked.human_judgment == [:ambiguous_go_no_go]
  end

  test "M67-2 render is operator-readable and explicitly read-only" do
    rendered = :blocked |> CampaignStatus.fixture() |> CampaignStatus.render()

    assert rendered =~ "Boundary: read-only; trusted writes: false"
    assert rendered =~ "Blocked tasks: seed"
    assert rendered =~ "Human judgment: ambiguous_go_no_go"
    assert rendered =~ "blocked state requires operator judgment"
    refute rendered =~ "approve automatically"
    refute rendered =~ "write ledger"
  end
end
