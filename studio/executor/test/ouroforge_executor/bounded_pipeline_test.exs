defmodule OuroforgeExecutor.BoundedPipelineTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{BoundedPipeline, ProductionPlan}

  defp task(id, depends_on, kind \\ "cli-drive") do
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

  defp plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m65-bounded-pipeline",
      "tasks" => [
        task("a-long", []),
        task("b-short", []),
        task("c-after-b", ["b-short"]),
        task("d-after-c", ["c-after-b"])
      ]
    })
  end

  test "bounded concurrency never exceeds the configured worker cap under load" do
    run =
      BoundedPipeline.run_adaptive(plan(),
        worker_limit: 2,
        task_durations_ms: %{"a-long" => 10, "b-short" => 1, "c-after-b" => 1, "d-after-c" => 1}
      )

    assert run.max_active <= 2
    assert run.completed_task_ids == ["b-short", "c-after-b", "d-after-c", "a-long"]
    assert Enum.any?(run.telemetry, &match?(%{event: :assigned, worker_id: "worker-1"}, &1))
    assert Enum.any?(run.telemetry, &match?(%{event: :assigned, worker_id: "worker-2"}, &1))
  end

  test "backpressure keeps excess ready work pending instead of growing an unbounded queue" do
    crowded =
      ProductionPlan.from_map!(%{
        "schemaVersion" => "producer-plan-v1",
        "planId" => "m65-crowded",
        "tasks" => Enum.map(1..5, &task("task-#{&1}", []))
      })

    run = BoundedPipeline.run_adaptive(crowded, worker_limit: 2)

    assert run.max_active == 2
    assert run.max_backpressure_depth == 3

    assert %{event: :backpressure, pending_task_ids: ["task-3", "task-4", "task-5"]} =
             Enum.find(run.telemetry, &(&1.event == :backpressure))
  end

  test "command-family caps prevent over-assignment even when workers are idle" do
    mixed =
      ProductionPlan.from_map!(%{
        "schemaVersion" => "producer-plan-v1",
        "planId" => "m65-family-caps",
        "tasks" => [
          task("apply-1", [], "trusted-apply"),
          task("apply-2", [], "trusted-apply"),
          task("inspect-1", [], "inspect")
        ]
      })

    run =
      BoundedPipeline.run_adaptive(mixed,
        worker_limit: 3,
        command_family_limits: %{"trusted-apply" => 1, "inspect" => 2},
        task_durations_ms: %{"apply-1" => 5, "apply-2" => 5, "inspect-1" => 1}
      )

    assert run.max_active == 2

    first_backpressure = Enum.find(run.telemetry, &(&1.event == :backpressure))
    assert first_backpressure.pending_task_ids == ["apply-2"]
  end

  test "budget halts block new assignments and CLI drive" do
    halted = %{may_assign?: false, may_drive_cli?: false, diagnosis: "budget exhausted"}
    run = BoundedPipeline.run_adaptive(plan(), worker_limit: 2, budget_decision: halted)

    assert run.completed_task_ids == []
    assert run.max_active == 0

    assert [%{event: :budget_halted, diagnosis: "budget exhausted"}, %{event: :run_finished}] =
             run.telemetry
  end

  test "idle workers pull newly ready work and improve utilization without changing verdict bytes" do
    durations = %{"a-long" => 10, "b-short" => 1, "c-after-b" => 1, "d-after-c" => 1}
    verdict = :erlang.term_to_binary(%{verdict: :manual_cli_golden})

    adaptive =
      BoundedPipeline.run_adaptive(plan(),
        worker_limit: 2,
        task_durations_ms: durations,
        verdict_bytes: verdict
      )

    fixed =
      BoundedPipeline.run_fixed_pool(plan(),
        worker_limit: 2,
        task_durations_ms: durations,
        verdict_bytes: verdict
      )

    assert adaptive.verdict_bytes == verdict
    assert fixed.verdict_bytes == verdict
    assert adaptive.completed_task_ids |> Enum.sort() == fixed.completed_task_ids |> Enum.sort()
    assert adaptive.makespan_ms < fixed.makespan_ms
    assert adaptive.worker_utilization > fixed.worker_utilization
    assert adaptive.throughput_per_ms > fixed.throughput_per_ms
  end
end
