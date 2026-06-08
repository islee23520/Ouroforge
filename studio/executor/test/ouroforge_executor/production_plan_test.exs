defmodule OuroforgeExecutor.ProductionPlanTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.ProductionPlan

  @fixture Path.expand(
             "../../../../examples/producer-plan-v1/design-intent.valid.fixture.json",
             __DIR__
           )

  defp plan do
    # Minimal fixture matching the Rust-owned producer-plan-v1 artifact shape.
    %{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "demo-production-plan",
      "tasks" => [
        task("task-b", []),
        task("task-a", []),
        task("task-c", ["task-a", "task-b"]),
        task("task-d", ["task-c"])
      ]
    }
    |> ProductionPlan.from_map!()
  end

  defp task(id, depends_on) do
    %{
      "taskId" => id,
      "functionAgent" => "requirements",
      "role" => "designer",
      "kind" => "proposal-task",
      "dependsOn" => depends_on,
      "inputs" => ["input:#{id}"],
      "outputs" => ["output:#{id}"],
      "proposalOnly" => true,
      "expectedVerification" => ["verify #{id}"]
    }
  end

  defp json_plan_fixture do
    ~S({
      "schemaVersion": "producer-plan-v1",
      "planId": "json-production-plan",
      "tasks": [
        {
          "taskId": "first",
          "functionAgent": "design-brief",
          "role": "designer",
          "kind": "intent-review",
          "dependsOn": [],
          "inputs": ["intent"],
          "outputs": ["brief"],
          "proposalOnly": true,
          "expectedVerification": ["verify"]
        },
        {
          "taskId": "second",
          "functionAgent": "requirements",
          "role": "designer",
          "kind": "requirement-extraction",
          "dependsOn": ["first"],
          "inputs": ["brief"],
          "outputs": ["requirements"],
          "proposalOnly": true,
          "expectedVerification": ["verify"]
        }
      ]
    })
  end

  test "reads producer-plan-v1 JSON without mutating kernel artifacts" do
    json = json_plan_fixture()
    parsed = ProductionPlan.from_json!(json)

    assert parsed.plan_id == "json-production-plan"
    assert Enum.map(parsed.tasks, & &1.id) == ["first", "second"]
  end

  test "computes deterministic ready sets from completed dependencies" do
    assert Enum.map(ProductionPlan.ready_set(plan()), & &1.id) == ["task-a", "task-b"]

    assert Enum.map(ProductionPlan.ready_set(plan(), %{completed_task_ids: ["task-a"]}), & &1.id) ==
             [
               "task-b"
             ]

    assert Enum.map(
             ProductionPlan.ready_set(plan(), %{completed_task_ids: ["task-a", "task-b"]}),
             & &1.id
           ) == [
             "task-c"
           ]
  end

  test "assignment is replayable and stable by sorted task id and worker id" do
    state = %{completed_task_ids: [], assigned_task_ids: []}

    first = ProductionPlan.assign_ready(plan(), ["worker-2", "worker-1"], state)
    second = ProductionPlan.assign_ready(plan(), ["worker-1", "worker-2"], state)

    assert first == second

    assert first == [
             %{
               task_id: "task-a",
               worker_id: "worker-1",
               role: "designer",
               function_agent: "requirements"
             },
             %{
               task_id: "task-b",
               worker_id: "worker-2",
               role: "designer",
               function_agent: "requirements"
             }
           ]
  end

  test "assigned tasks are not reassigned before completion" do
    state = %{assigned_task_ids: ["task-a"]}

    assert Enum.map(ProductionPlan.ready_set(plan(), state), & &1.id) == ["task-b"]
  end

  test "completion order is topological and deterministic" do
    assert ProductionPlan.completion_order(plan()) == ["task-b", "task-a", "task-c", "task-d"]
  end

  test "fails closed on missing dependencies and cycles" do
    missing = %{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "missing-dependency-plan",
      "tasks" => [task("task-a", ["missing"])]
    }

    assert_raise ArgumentError, ~r/depends on missing task missing/, fn ->
      ProductionPlan.from_map!(missing)
    end

    cyclic = %{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "cyclic-plan",
      "tasks" => [task("task-a", ["task-b"]), task("task-b", ["task-a"])]
    }

    assert_raise ArgumentError, ~r/dependency cycle/, fn ->
      ProductionPlan.from_map!(cyclic)
    end
  end

  test "repo still carries the Rust-owned M43 design intent fixture" do
    assert File.exists?(@fixture)
  end
end
