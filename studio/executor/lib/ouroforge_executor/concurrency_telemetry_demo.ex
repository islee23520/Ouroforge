defmodule OuroforgeExecutor.ConcurrencyTelemetryDemo do
  @moduledoc """
  M65 load demo for bounded concurrency, utilization, and live telemetry.

  The demo compares a local adaptive control-plane schedule against a fixed-pool
  baseline while preserving the manual-vs-executor CLI golden-parity bytes from
  `OuroforgeExecutor.DemoCampaign`. It emits read-only telemetry snapshots and
  never writes kernel artifacts or trusted state.
  """

  alias OuroforgeExecutor.{BoundedPipeline, DemoCampaign, ProductionPlan, ProgressSurface}

  @worker_limit 3
  @durations %{
    "01-long-validate" => 10,
    "02-fast-seed" => 1,
    "03-after-seed" => 1,
    "04-after-three" => 1,
    "05-independent" => 2,
    "06-after-five" => 1,
    "07-inspect" => 2,
    "08-final" => 1
  }

  def run(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    command_opts = Keyword.get(opts, :command_opts, [])

    manual = DemoCampaign.run_manual(runner: runner, command_opts: command_opts)
    executor = DemoCampaign.run_executor(runner: runner, command_opts: command_opts)
    golden_parity = DemoCampaign.golden_parity?(manual, executor)
    verdict_bytes = :erlang.term_to_binary(executor)
    plan = load_plan()

    :ok = emit(:run_started, %{worker_limit: @worker_limit}, %{plan_id: plan.plan_id})

    adaptive =
      BoundedPipeline.run_adaptive(plan,
        worker_limit: @worker_limit,
        command_family_limits: %{
          "trusted-apply" => 1,
          "inspect" => 2,
          "cli-drive" => @worker_limit
        },
        task_durations_ms: @durations,
        verdict_bytes: verdict_bytes
      )

    fixed =
      BoundedPipeline.run_fixed_pool(plan,
        worker_limit: @worker_limit,
        command_family_limits: %{
          "trusted-apply" => 1,
          "inspect" => 2,
          "cli-drive" => @worker_limit
        },
        task_durations_ms: @durations,
        verdict_bytes: verdict_bytes
      )

    surface = progress_surface(plan, adaptive)

    :ok =
      ProgressSurface.emit(surface, :snapshot, %{
        worker_utilization: adaptive.worker_utilization,
        fixed_pool_utilization: fixed.worker_utilization,
        backpressure_depth: adaptive.max_backpressure_depth
      })

    :ok =
      emit(:run_finished, %{completed_tasks: length(adaptive.completed_task_ids)}, %{
        plan_id: plan.plan_id
      })

    %{
      manual_transcript: manual,
      executor_transcript: executor,
      golden_parity?: golden_parity,
      verdict_bytes_unchanged?:
        adaptive.verdict_bytes == fixed.verdict_bytes and adaptive.verdict_bytes == verdict_bytes,
      worker_limit: @worker_limit,
      adaptive: adaptive,
      fixed_pool: fixed,
      bounds_held?: adaptive.max_active <= @worker_limit,
      utilization_improved?: adaptive.worker_utilization > fixed.worker_utilization,
      telemetry_surface: surface
    }
  end

  def load_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m65-concurrency-telemetry-demo",
      "tasks" => [
        task("01-long-validate", [], "cli-drive"),
        task("02-fast-seed", [], "cli-drive"),
        task("03-after-seed", ["02-fast-seed"], "cli-drive"),
        task("04-after-three", ["03-after-seed"], "trusted-apply"),
        task("05-independent", [], "inspect"),
        task("06-after-five", ["05-independent"], "inspect"),
        task("07-inspect", [], "inspect"),
        task(
          "08-final",
          ["01-long-validate", "04-after-three", "06-after-five", "07-inspect"],
          "evaluate"
        )
      ]
    })
  end

  defp progress_surface(plan, adaptive) do
    ledger = %{
      "entries" =>
        Enum.map(adaptive.completed_task_ids, fn task_id ->
          %{
            "taskId" => task_id,
            "status" => "completed",
            "evidenceRef" => "runs/m65-demo/evidence/#{task_id}.json"
          }
        end)
    }

    ProgressSurface.from_artifacts(plan, ledger, %{
      active_workers: 0,
      queued_ready_tasks: 0,
      retry_attempts: 0
    })
  end

  defp task(id, depends_on, kind) do
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

  defp emit(event, measurements, metadata) do
    :telemetry.execute([:ouroforge_executor, :demo, event], measurements, metadata)
    :ok
  end
end
