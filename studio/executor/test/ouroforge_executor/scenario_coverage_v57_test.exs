defmodule OuroforgeExecutor.ScenarioCoverageV57Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{
    BoundedPipeline,
    ConcurrencyTelemetryDemo,
    ProductionPlan,
    ProgressSurface
  }

  @coverage_doc Path.expand("../../../../docs/scenario-coverage-v57.md", __DIR__)

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

  defp load_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "scenario-coverage-v57",
      "tasks" => [
        task("01-long", [], "cli-drive"),
        task("02-fast", [], "cli-drive"),
        task("03-after-fast", ["02-fast"], "cli-drive"),
        task("04-trusted", ["03-after-fast"], "trusted-apply"),
        task("05-trusted", [], "trusted-apply"),
        task("06-inspect", [], "inspect")
      ]
    })
  end

  test "v57 records concurrency and telemetry coverage guardrails" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v57"
    assert doc =~ "#1949"
    assert doc =~ "V57.concurrency.bounds"
    assert doc =~ "V57.backpressure.pending"
    assert doc =~ "V57.work_stealing.utilization"
    assert doc =~ "V57.telemetry.read_only"
    assert doc =~ "Elixir/OTP = control plane only"
    assert doc =~ "Rust kernel = data plane"
    assert doc =~ "frozen `ouroforge` CLI surface"
    assert doc =~ "#1 and #23 remain open"
  end

  test "worker and command-family bounds hold under load" do
    run =
      BoundedPipeline.run_adaptive(load_plan(),
        worker_limit: 3,
        command_family_limits: %{"trusted-apply" => 1, "cli-drive" => 3, "inspect" => 1},
        task_durations_ms: %{
          "01-long" => 10,
          "02-fast" => 1,
          "03-after-fast" => 1,
          "04-trusted" => 1,
          "05-trusted" => 5,
          "06-inspect" => 2
        }
      )

    assert run.max_active <= 3
    assert run.max_backpressure_depth > 0

    trusted_assignments =
      run.telemetry
      |> Enum.filter(&match?(%{event: :assigned, family: "trusted-apply"}, &1))
      |> Enum.map(& &1.at_ms)

    assert trusted_assignments == Enum.sort(trusted_assignments)
    assert Enum.uniq(trusted_assignments) == trusted_assignments
  end

  test "backpressure keeps overflow pending in deterministic task order" do
    crowded =
      ProductionPlan.from_map!(%{
        "schemaVersion" => "producer-plan-v1",
        "planId" => "scenario-coverage-v57-crowded",
        "tasks" => Enum.map(1..6, &task("task-#{&1}", []))
      })

    run = BoundedPipeline.run_adaptive(crowded, worker_limit: 2)

    assert run.max_active == 2
    assert run.max_backpressure_depth == 4

    assert %{event: :backpressure, pending_task_ids: ["task-3", "task-4", "task-5", "task-6"]} =
             Enum.find(run.telemetry, &(&1.event == :backpressure))
  end

  test "idle-worker work stealing improves utilization without changing verdict bytes" do
    durations = %{
      "01-long" => 10,
      "02-fast" => 1,
      "03-after-fast" => 1,
      "04-trusted" => 1,
      "05-trusted" => 5,
      "06-inspect" => 2
    }

    verdict = :erlang.term_to_binary(%{manual_cli_verdict: :byte_identical})

    adaptive =
      BoundedPipeline.run_adaptive(load_plan(),
        worker_limit: 3,
        task_durations_ms: durations,
        verdict_bytes: verdict
      )

    fixed =
      BoundedPipeline.run_fixed_pool(load_plan(),
        worker_limit: 3,
        task_durations_ms: durations,
        verdict_bytes: verdict
      )

    assert adaptive.verdict_bytes == verdict
    assert fixed.verdict_bytes == verdict
    assert Enum.sort(adaptive.completed_task_ids) == Enum.sort(fixed.completed_task_ids)
    assert adaptive.worker_utilization > fixed.worker_utilization
    assert adaptive.throughput_per_ms > fixed.throughput_per_ms
  end

  test "telemetry snapshot is read-only and carries kernel refs without trusted-write authority" do
    test_pid = self()
    handler_id = {__MODULE__, self(), System.unique_integer([:positive])}
    event_name = ProgressSurface.event_prefix() ++ [:snapshot]

    :telemetry.attach(
      handler_id,
      event_name,
      fn event, measurements, metadata, _config ->
        send(test_pid, {:v57_snapshot, event, measurements, metadata})
      end,
      nil
    )

    try do
      surface =
        ProgressSurface.from_artifacts(
          load_plan(),
          %{
            "entries" => [
              %{
                "taskId" => "01-long",
                "status" => "completed",
                "evidenceRef" => "runs/v57/evidence/01-long.json"
              },
              %{
                "taskId" => "02-fast",
                "status" => "completed",
                "ledgerRef" => "runs/v57/ledger.json"
              }
            ]
          },
          %{
            assigned_task_ids: ["03-after-fast"],
            retrying_task_ids: ["04-trusted"],
            active_workers: 2
          }
        )

      assert :ok = ProgressSurface.emit(surface, :snapshot, %{backpressure_depth: 2})

      assert_receive {:v57_snapshot, ^event_name, measurements, metadata}
      assert measurements.completed_tasks == 2
      assert measurements.in_flight_tasks == 1
      assert measurements.retrying_tasks == 1
      assert measurements.backpressure_depth == 2
      assert metadata.kernel_refs == ["runs/v57/evidence/01-long.json", "runs/v57/ledger.json"]
      assert metadata.trusted_write_authority == false
      assert surface.boundary == :read_only_control_plane_surface
      assert surface.control_plane.trusted_write_authority == false
    after
      :telemetry.detach(handler_id)
    end
  end

  test "M65 demo remains a local read-only control-plane composition" do
    plan = ConcurrencyTelemetryDemo.load_plan()
    assert plan.plan_id == "m65-concurrency-telemetry-demo"
    assert Enum.count(plan.tasks) == 8
    assert Enum.any?(plan.tasks, &(&1.kind == "trusted-apply"))
    assert Enum.all?(plan.tasks, &(&1.role == "executor-control-plane"))
  end
end
