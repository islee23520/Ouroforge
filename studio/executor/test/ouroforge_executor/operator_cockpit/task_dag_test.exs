defmodule OuroforgeExecutor.OperatorCockpit.TaskDAGTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.TaskDAG

  test "M67-3 renders dependencies and runnable frontier without scheduling" do
    dag = TaskDAG.fixture(:normal)

    assert dag.version == "m67-3"
    assert dag.boundary == :read_only_task_dag_progress
    assert dag.edges == [%{from: "project", to: "inspect"}, %{from: "seed", to: "project"}]
    assert dag.runnable_frontier == []
    assert Enum.find(dag.nodes, &(&1.task_id == "project")).status == :in_flight
    refute dag.trusted_write_authority?
  end

  test "M67-3 fixtures cover waiting, retrying, budget, backpressure, blocked, and skipped work" do
    fixtures = TaskDAG.fixtures()

    assert fixtures.waiting.runnable_frontier == ["seed"]
    assert fixtures.retrying.retrying_tasks == ["seed"]
    assert fixtures.budget_limited.notes |> Enum.any?(&String.contains?(&1, "human judgment"))

    assert fixtures.backpressured.notes
           |> Enum.any?(&String.contains?(&1, "local executor capacity"))

    assert fixtures.blocked.blocked_tasks == ["seed"]
    assert fixtures.skipped.skipped_tasks == ["project"]

    assert Enum.find(fixtures.skipped.nodes, &(&1.task_id == "project")).status ==
             :skipped_by_control_plane
  end

  test "M67-3 render is copy-only and exposes wait gates" do
    rendered = :waiting |> TaskDAG.fixture() |> TaskDAG.render()

    assert rendered =~ "Task DAG m67-task-dag-fixture: read-only progress"
    assert rendered =~ "Runnable frontier: seed"
    assert rendered =~ "Wait gates: project waits for seed; inspect waits for project"
    assert rendered =~ "Trusted writes: false"
    assert rendered =~ "no task is launched by this view"
    refute rendered =~ "run now"
    refute rendered =~ "write evidence"
  end
end
