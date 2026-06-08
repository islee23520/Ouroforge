defmodule OuroforgeExecutor.ScenarioCoverageV58Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{
    BoundedPipeline,
    DemoCampaign,
    ProgressSurface,
    SupervisedDemo
  }

  @coverage_doc Path.expand("../../../../docs/scenario-coverage-v58.md", __DIR__)

  def telemetry_handler(event, measurements, metadata, test_pid) do
    send(test_pid, {:v58_telemetry, event, measurements, metadata})
  end

  test "v58 records end-to-end autonomy coverage guardrails" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v58"
    assert doc =~ "#1950"
    assert doc =~ "V58.parity.manual_executor"
    assert doc =~ "V58.supervision.recovery"
    assert doc =~ "V58.budget.halt"
    assert doc =~ "V58.concurrency.telemetry"
    assert doc =~ "V58.boundary.autonomy"
    assert doc =~ "Elixir/OTP = control plane only"
    assert doc =~ "Rust kernel = data plane"
    assert doc =~ "frozen `ouroforge` CLI surface"
    assert doc =~ "#1 and #23 remain open"
  end

  test "composed autonomy envelope preserves parity, recovery, budgets, bounds, and telemetry" do
    runner = fn "ouroforge", argv, _opts ->
      {"#{Enum.join(argv, " ")} valid\n", 0}
    end

    manual = DemoCampaign.run_manual(runner: runner)
    executor = DemoCampaign.run_executor(runner: runner)

    assert DemoCampaign.golden_parity?(manual, executor)
    verdict_bytes = :erlang.term_to_binary(executor)

    recovery = SupervisedDemo.recovery_demo()
    assert recovery.completed_task_ids == ["crash-prone-worker", "trusted-apply"]
    assert recovery.drive_task_ids == ["post-recovery-evaluate"]
    assert recovery.trusted_write_keys == ["mutation:trusted-apply:1"]
    assert recovery.blocked == []

    budget_halt = SupervisedDemo.budget_halt_demo()
    assert budget_halt.status == :halted_budget_exhausted
    refute budget_halt.may_assign?
    refute budget_halt.may_drive_cli?

    plan = OuroforgeExecutor.ConcurrencyTelemetryDemo.load_plan()

    adaptive =
      BoundedPipeline.run_adaptive(plan,
        worker_limit: 3,
        command_family_limits: %{"trusted-apply" => 1, "inspect" => 2, "cli-drive" => 3},
        task_durations_ms: %{
          "01-long-validate" => 10,
          "02-fast-seed" => 1,
          "03-after-seed" => 1,
          "04-after-three" => 1,
          "05-independent" => 2,
          "06-after-five" => 1,
          "07-inspect" => 2,
          "08-final" => 1
        },
        verdict_bytes: verdict_bytes
      )

    fixed =
      BoundedPipeline.run_fixed_pool(plan,
        worker_limit: 3,
        command_family_limits: %{"trusted-apply" => 1, "inspect" => 2, "cli-drive" => 3},
        task_durations_ms: %{
          "01-long-validate" => 10,
          "02-fast-seed" => 1,
          "03-after-seed" => 1,
          "04-after-three" => 1,
          "05-independent" => 2,
          "06-after-five" => 1,
          "07-inspect" => 2,
          "08-final" => 1
        },
        verdict_bytes: verdict_bytes
      )

    assert adaptive.max_active <= 3
    assert adaptive.max_backpressure_depth > 0
    assert adaptive.worker_utilization > fixed.worker_utilization
    assert adaptive.throughput_per_ms > fixed.throughput_per_ms
    assert adaptive.verdict_bytes == fixed.verdict_bytes
    assert adaptive.verdict_bytes == verdict_bytes

    ledger = %{
      "entries" =>
        Enum.map(adaptive.completed_task_ids, fn task_id ->
          %{
            "taskId" => task_id,
            "status" => "completed",
            "evidenceRef" => "runs/v58/evidence/#{task_id}.json"
          }
        end)
    }

    surface =
      ProgressSurface.from_artifacts(plan, ledger, %{active_workers: 0, queued_ready_tasks: 0})

    assert surface.counts.completed_by_kernel_evidence == 8
    assert surface.control_plane.trusted_write_authority == false

    handler_id = {__MODULE__, self(), System.unique_integer([:positive])}
    event_name = ProgressSurface.event_prefix() ++ [:snapshot]
    :telemetry.attach(handler_id, event_name, &__MODULE__.telemetry_handler/4, self())

    try do
      assert :ok =
               ProgressSurface.emit(surface, :snapshot, %{
                 backpressure_depth: adaptive.max_backpressure_depth
               })

      assert_receive {:v58_telemetry, ^event_name, measurements, metadata}
      assert measurements.completed_tasks == 8
      assert measurements.backpressure_depth == adaptive.max_backpressure_depth
      assert metadata.trusted_write_authority == false
      assert metadata.boundary == :read_only_control_plane_surface
      assert Enum.count(metadata.kernel_refs) == 8
    after
      :telemetry.detach(handler_id)
    end
  end

  test "autonomy envelope exposes no release or trusted-write authority" do
    recovery = SupervisedDemo.recovery_demo()
    budget_halt = SupervisedDemo.budget_halt_demo()
    plan = OuroforgeExecutor.ConcurrencyTelemetryDemo.load_plan()
    surface = ProgressSurface.from_artifacts(plan, %{"entries" => []}, %{})

    refute Map.has_key?(surface, :release_authority)
    refute Map.has_key?(surface, :trusted_write_path)
    assert surface.control_plane.trusted_write_authority == false
    assert recovery.trusted_write_keys == ["mutation:trusted-apply:1"]
    refute budget_halt.may_drive_cli?
  end
end
