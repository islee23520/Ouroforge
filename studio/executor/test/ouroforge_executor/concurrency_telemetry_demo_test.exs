defmodule OuroforgeExecutor.ConcurrencyTelemetryDemoTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.ConcurrencyTelemetryDemo

  @moduletag :demo

  test "load demo holds bounds, improves utilization, emits telemetry, and preserves golden parity" do
    repo_root = Path.expand("../../../..", __DIR__)
    test_pid = self()
    handler_id = {__MODULE__, self(), System.unique_integer([:positive])}

    :telemetry.attach_many(
      handler_id,
      [
        [:ouroforge_executor, :demo, :run_started],
        [:ouroforge_executor, :progress, :snapshot],
        [:ouroforge_executor, :demo, :run_finished]
      ],
      fn event, measurements, metadata, _config ->
        send(test_pid, {:demo_telemetry, event, measurements, metadata})
      end,
      nil
    )

    try do
      runner = fn "ouroforge", argv, opts ->
        command_opts = Keyword.merge([cd: repo_root, stderr_to_stdout: true], opts)
        System.cmd("cargo", ["run", "-q", "-p", "ouroforge-cli", "--" | argv], command_opts)
      end

      report = ConcurrencyTelemetryDemo.run(runner: runner)

      assert report.golden_parity?
      assert report.verdict_bytes_unchanged?
      assert Enum.all?(report.manual_transcript, &(&1.status == 0))
      assert Enum.all?(report.executor_transcript, &(&1.status == 0))
      assert report.bounds_held?
      assert report.adaptive.max_active <= report.worker_limit
      assert report.adaptive.max_backpressure_depth > 0
      assert report.utilization_improved?
      assert report.adaptive.worker_utilization > report.fixed_pool.worker_utilization
      assert report.adaptive.throughput_per_ms > report.fixed_pool.throughput_per_ms

      assert report.telemetry_surface.boundary == :read_only_control_plane_surface
      assert report.telemetry_surface.control_plane.trusted_write_authority == false
      assert report.telemetry_surface.counts.completed_by_kernel_evidence == 8

      assert_receive {:demo_telemetry, [:ouroforge_executor, :demo, :run_started],
                      start_measurements, start_metadata}

      assert start_measurements.worker_limit == report.worker_limit
      assert start_metadata.plan_id == "m65-concurrency-telemetry-demo"

      assert_receive {:demo_telemetry, [:ouroforge_executor, :progress, :snapshot],
                      snapshot_measurements, snapshot_metadata}

      assert snapshot_measurements.total_tasks == 8
      assert snapshot_measurements.completed_tasks == 8
      assert snapshot_measurements.worker_utilization == report.adaptive.worker_utilization
      assert snapshot_measurements.fixed_pool_utilization == report.fixed_pool.worker_utilization
      assert snapshot_metadata.trusted_write_authority == false
      assert snapshot_metadata.boundary == :read_only_control_plane_surface

      assert_receive {:demo_telemetry, [:ouroforge_executor, :demo, :run_finished],
                      finish_measurements, finish_metadata}

      assert finish_measurements.completed_tasks == 8
      assert finish_metadata.plan_id == "m65-concurrency-telemetry-demo"
    after
      :telemetry.detach(handler_id)
    end
  end
end
