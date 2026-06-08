defmodule OuroforgeExecutor.ProgressSurfaceTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{ProductionPlan, ProgressSurface}

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
      "planId" => "m65-progress-surface",
      "tasks" => [
        task("draft", []),
        task("apply", ["draft"], "trusted-apply"),
        task("verify", ["apply"], "evaluate"),
        task("blocked", [], "inspect")
      ]
    })
  end

  test "surface derives status from kernel artifacts and control-plane observations" do
    ledger = %{
      "entries" => [
        %{
          "taskId" => "draft",
          "status" => "completed",
          "evidenceRef" => "runs/demo/evidence/draft.json"
        },
        %{"taskId" => "verify", "status" => "failed", "ledgerRef" => "runs/demo/ledger.json"}
      ]
    }

    surface =
      ProgressSurface.from_artifacts(plan(), ledger, %{
        assigned_task_ids: ["apply"],
        blocked_task_ids: ["blocked"],
        active_workers: 1,
        queued_ready_tasks: 0,
        retry_attempts: 2
      })

    by_id = Map.new(surface.tasks, &{&1.task_id, &1})

    assert by_id["draft"].status == :completed_by_kernel_evidence
    assert by_id["draft"].source == :rust_kernel_artifact
    assert by_id["apply"].status == :in_flight
    assert by_id["apply"].source == :executor_control_plane_observation
    assert by_id["verify"].status == :failed_by_kernel_evidence
    assert by_id["blocked"].status == :blocked

    assert surface.boundary == :read_only_control_plane_surface
    assert surface.kernel_refs == ["runs/demo/evidence/draft.json", "runs/demo/ledger.json"]
    assert surface.control_plane.trusted_write_authority == false
    assert surface.counts.completed_by_kernel_evidence == 1
    assert surface.counts.in_flight == 1
  end

  test "telemetry event carries read-only progress measurements and metadata" do
    test_pid = self()
    handler_id = {__MODULE__, self(), System.unique_integer([:positive])}
    event_name = ProgressSurface.event_prefix() ++ [:snapshot]

    :telemetry.attach(
      handler_id,
      event_name,
      fn event, measurements, metadata, _config ->
        send(test_pid, {:telemetry_event, event, measurements, metadata})
      end,
      nil
    )

    try do
      surface =
        ProgressSurface.from_artifacts(
          plan(),
          %{
            "entries" => [
              %{
                "taskId" => "draft",
                "status" => "completed",
                "evidenceRef" => "runs/demo/evidence/draft.json"
              }
            ]
          },
          %{assigned_task_ids: ["apply"]}
        )

      assert :ok = ProgressSurface.emit(surface, :snapshot, %{backpressure_depth: 1})

      assert_receive {:telemetry_event, ^event_name, measurements, metadata}
      assert measurements.total_tasks == 4
      assert measurements.completed_tasks == 1
      assert measurements.in_flight_tasks == 1
      assert measurements.backpressure_depth == 1
      assert metadata.plan_id == "m65-progress-surface"
      assert metadata.boundary == :read_only_control_plane_surface
      assert metadata.trusted_write_authority == false
      assert metadata.kernel_refs == ["runs/demo/evidence/draft.json"]
    after
      :telemetry.detach(handler_id)
    end
  end

  test "surface exposes no trusted write or artifact truth authority" do
    surface = ProgressSurface.from_artifacts(plan(), %{"entries" => []}, %{})

    refute Map.has_key?(surface, :write_path)
    refute Map.has_key?(surface, :artifact_truth)
    assert surface.control_plane.trusted_write_authority == false
    assert Enum.all?(surface.tasks, &(&1.source == :executor_control_plane_observation))
  end
end
