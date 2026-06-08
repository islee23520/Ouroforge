defmodule OuroforgeExecutor.ScenarioCoverageV55Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{CLI, Contract, DemoCampaign, ProductionPlan}

  @coverage_doc Path.expand("../../../../docs/scenario-coverage-v55.md", __DIR__)

  defp task(id, depends_on, kind \\ "proposal-task") do
    %{
      "taskId" => id,
      "functionAgent" => "producer",
      "role" => "executor-control-plane",
      "kind" => kind,
      "dependsOn" => depends_on,
      "inputs" => ["input:#{id}"],
      "outputs" => ["output:#{id}"]
    }
  end

  defp m63_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "scenario-coverage-v55-m63",
      "tasks" => [
        task("01-plan", []),
        task("02-schedule", ["01-plan"]),
        task("03-cli-drive", ["02-schedule"], "cli-drive"),
        task("04-demo-parity", ["03-cli-drive"], "golden-parity")
      ]
    })
  end

  test "v55 records the executor-skeleton regression suite and issue" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v55"
    assert doc =~ "#1938"
    assert doc =~ "Elixir/OTP = control plane only; Rust kernel = data plane"
    assert doc =~ "executor reaches the kernel only via the frozen `ouroforge` CLI surface"
    assert doc =~ "#1 and #23 remain open"
  end

  test "plan and DAG consumption fail closed on missing dependencies and cycles" do
    assert ProductionPlan.completion_order(m63_plan()) == [
             "01-plan",
             "02-schedule",
             "03-cli-drive",
             "04-demo-parity"
           ]

    assert_raise ArgumentError, ~r/depends on missing task/, fn ->
      ProductionPlan.from_map!(%{
        "schemaVersion" => "producer-plan-v1",
        "planId" => "v55-missing-dependency",
        "tasks" => [task("orphan", ["missing"])]
      })
    end

    assert_raise ArgumentError, ~r/dependency cycle/, fn ->
      ProductionPlan.from_map!(%{
        "schemaVersion" => "producer-plan-v1",
        "planId" => "v55-cycle",
        "tasks" => [task("a", ["b"]), task("b", ["a"])]
      })
    end
  end

  test "scheduler remains deterministic and never reassigns in-flight work" do
    plan = m63_plan()

    assert Enum.map(ProductionPlan.ready_set(plan), & &1.id) == ["01-plan"]

    assert ProductionPlan.assign_ready(plan, ["worker-b", "worker-a"]) == [
             %{
               task_id: "01-plan",
               worker_id: "worker-a",
               role: "executor-control-plane",
               function_agent: "producer"
             }
           ]

    state = %{completed_task_ids: ["01-plan"], assigned_task_ids: ["02-schedule"]}
    assert ProductionPlan.ready_set(plan, state) == []

    state = %{completed_task_ids: ["01-plan", "02-schedule"], assigned_task_ids: []}
    assert Enum.map(ProductionPlan.ready_set(plan, state), & &1.id) == ["03-cli-drive"]
  end

  test "CLI drive is constrained to the frozen surface" do
    argv = ["project", "validate", "examples/playable-demo-v2/collect-and-exit"]

    runner = fn "ouroforge-v55", ^argv, opts ->
      assert Keyword.get(opts, :stderr_to_stdout) == false
      {"project valid", 0}
    end

    assert {:ok, result} = CLI.run(argv, executable: "ouroforge-v55", runner: runner)
    assert result.command_family == ["project", "validate"]
    assert result.stdout == "project valid"

    assert {:error, {:outside_frozen_cli_surface, ["release", "publish"]}} =
             CLI.run(["release", "publish"])
  end

  test "trusted writes stay routed and cannot self-certify" do
    refute Contract.allowed_cli_family?(["ledger", "append", "runs/demo"])
    refute Contract.allowed_cli_family?(["evidence", "add", "runs/demo"])

    review = [
      "mutation",
      "review",
      "runs/demo",
      "--decision",
      "accepted",
      "--reason",
      "human approved",
      "--reviewer",
      "executor",
      "--reviewer-type",
      "human"
    ]

    assert {:error, {:self_certification_blocked, "executor"}} =
             CLI.trusted_write(review, %{actor_id: "executor"})

    assert {:error, {:not_trusted_write_route, ["evaluate"]}} =
             CLI.trusted_write(["evaluate", "runs/demo"], %{actor_id: "executor"})
  end

  test "golden parity catches executor transcript drift from manual CLI" do
    runner = fn "ouroforge", argv, _opts ->
      {"#{Enum.join(argv, " ")} valid\n", 0}
    end

    manual = DemoCampaign.run_manual(runner: runner)
    executor = DemoCampaign.run_executor(runner: runner)

    assert DemoCampaign.golden_parity?(manual, executor)

    drifted_executor = List.update_at(executor, 0, &Map.put(&1, :stdout, "drift\n"))
    refute DemoCampaign.golden_parity?(manual, drifted_executor)
  end
end
