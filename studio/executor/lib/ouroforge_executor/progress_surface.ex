defmodule OuroforgeExecutor.ProgressSurface do
  @moduledoc """
  Read-only progress and telemetry surface for local executor campaigns.

  The surface derives task state from Rust-owned ledger/evidence-shaped inputs and
  ephemeral executor control-plane state. It never writes artifacts, ledgers,
  evidence, verdicts, releases, or trust-gradient records, and it never treats
  executor process state as product truth.
  """

  alias OuroforgeExecutor.ProductionPlan

  defstruct [
    :plan_id,
    :boundary,
    tasks: [],
    counts: %{},
    kernel_refs: [],
    control_plane: %{}
  ]

  @event_prefix [:ouroforge_executor, :progress]
  @completed_statuses MapSet.new([
                        "completed",
                        "complete",
                        "done",
                        "applied",
                        "accepted",
                        "passed"
                      ])
  @failed_statuses MapSet.new(["failed", "rejected", "blocked"])

  def from_artifacts(%ProductionPlan{} = plan, ledger \\ %{}, control_plane \\ %{})
      when is_map(ledger) and is_map(control_plane) do
    entries = Map.get(ledger, "entries", [])
    entry_by_task = latest_entry_by_task(entries)
    assigned = assigned_task_ids(control_plane)
    retrying = retrying_task_ids(control_plane)
    blocked = blocked_task_ids(control_plane)
    completed = completed_task_ids(entry_by_task)

    tasks =
      Enum.map(plan.tasks, fn task ->
        entry = Map.get(entry_by_task, task.id)
        status = classify_task(task.id, entry, assigned, retrying, blocked, completed)

        %{
          task_id: task.id,
          role: task.role,
          function_agent: task.function_agent,
          command_family: task.kind,
          status: status,
          kernel_ref: kernel_ref(entry),
          source: source_for(status)
        }
      end)

    %__MODULE__{
      plan_id: plan.plan_id,
      boundary: :read_only_control_plane_surface,
      tasks: tasks,
      counts: counts(tasks),
      kernel_refs: kernel_refs(entries),
      control_plane: summarize_control_plane(control_plane)
    }
  end

  def emit(%__MODULE__{} = surface, event, measurements \\ %{}, metadata \\ %{})
      when is_atom(event) and is_map(measurements) and is_map(metadata) do
    :telemetry.execute(
      @event_prefix ++ [event],
      Map.merge(default_measurements(surface), measurements),
      Map.merge(default_metadata(surface), metadata)
    )

    :ok
  end

  def event_prefix, do: @event_prefix

  defp latest_entry_by_task(entries) do
    entries
    |> Enum.filter(&is_map/1)
    |> Enum.reduce(%{}, fn entry, acc ->
      case Map.get(entry, "taskId") do
        id when is_binary(id) and id != "" -> Map.put(acc, id, entry)
        _ -> acc
      end
    end)
  end

  defp classify_task(task_id, entry, assigned, retrying, blocked, completed) do
    cond do
      MapSet.member?(completed, task_id) -> :completed_by_kernel_evidence
      MapSet.member?(blocked, task_id) -> :blocked
      MapSet.member?(retrying, task_id) -> :retrying
      MapSet.member?(assigned, task_id) -> :in_flight
      entry_status(entry) in @failed_statuses -> :failed_by_kernel_evidence
      true -> :queued
    end
  end

  defp entry_status(nil), do: nil
  defp entry_status(entry), do: Map.get(entry, "status")

  defp completed_task_ids(entry_by_task) do
    entry_by_task
    |> Enum.filter(fn {_task_id, entry} -> entry_status(entry) in @completed_statuses end)
    |> Enum.map(fn {task_id, _entry} -> task_id end)
    |> MapSet.new()
  end

  defp assigned_task_ids(%{assigned_task_ids: values}) when is_list(values),
    do: string_set(values)

  defp assigned_task_ids(%{"assignedTaskIds" => values}) when is_list(values),
    do: string_set(values)

  defp assigned_task_ids(%{"assigned_task_ids" => values}) when is_list(values),
    do: string_set(values)

  defp assigned_task_ids(%{assignments: assignments}) when is_list(assignments),
    do: assignments |> Enum.map(&task_id_from_assignment!/1) |> string_set()

  defp assigned_task_ids(%{"assignments" => assignments}) when is_list(assignments),
    do: assignments |> Enum.map(&task_id_from_assignment!/1) |> string_set()

  defp assigned_task_ids(_), do: MapSet.new()

  defp retrying_task_ids(%{retrying_task_ids: values}) when is_list(values),
    do: string_set(values)

  defp retrying_task_ids(%{"retryingTaskIds" => values}) when is_list(values),
    do: string_set(values)

  defp retrying_task_ids(_), do: MapSet.new()

  defp blocked_task_ids(%{blocked_task_ids: values}) when is_list(values), do: string_set(values)

  defp blocked_task_ids(%{"blockedTaskIds" => values}) when is_list(values),
    do: string_set(values)

  defp blocked_task_ids(_), do: MapSet.new()

  defp task_id_from_assignment!(%{task_id: id}), do: id
  defp task_id_from_assignment!(%{"taskId" => id}), do: id
  defp task_id_from_assignment!(%{"task_id" => id}), do: id

  defp task_id_from_assignment!(other),
    do: raise(ArgumentError, "assignment missing task id: #{inspect(other)}")

  defp string_set(values), do: values |> Enum.map(&to_string/1) |> MapSet.new()

  defp kernel_ref(nil), do: nil

  defp kernel_ref(entry) do
    Map.get(entry, "evidenceRef") || Map.get(entry, "ledgerRef") || Map.get(entry, "verdictRef") ||
      Map.get(entry, "id")
  end

  defp source_for(:completed_by_kernel_evidence), do: :rust_kernel_artifact
  defp source_for(:failed_by_kernel_evidence), do: :rust_kernel_artifact
  defp source_for(_), do: :executor_control_plane_observation

  defp counts(tasks) do
    tasks
    |> Enum.frequencies_by(& &1.status)
    |> Map.put(:total, length(tasks))
  end

  defp kernel_refs(entries) do
    entries
    |> Enum.flat_map(fn entry ->
      [
        Map.get(entry, "evidenceRef"),
        Map.get(entry, "ledgerRef"),
        Map.get(entry, "verdictRef")
      ]
    end)
    |> Enum.reject(&is_nil/1)
    |> Enum.uniq()
    |> Enum.sort()
  end

  defp summarize_control_plane(control_plane) do
    %{
      active_workers: integer_or_zero(control_plane, [:active_workers, "activeWorkers"]),
      queued_ready_tasks:
        integer_or_zero(control_plane, [:queued_ready_tasks, "queuedReadyTasks"]),
      retry_attempts: integer_or_zero(control_plane, [:retry_attempts, "retryAttempts"]),
      trusted_write_authority: false,
      budget_limited?:
        Map.get(control_plane, :budget_limited?) == true or
          Map.get(control_plane, "budgetLimited") == true or
          Map.get(control_plane, "budget_limited?") == true,
      stop_gate:
        Map.get(control_plane, :stop_gate) || Map.get(control_plane, "stopGate") ||
          Map.get(control_plane, "stop_gate"),
      backpressure_depth:
        integer_or_zero(control_plane, [:backpressure_depth, "backpressureDepth"])
    }
  end

  defp integer_or_zero(map, keys) do
    keys
    |> Enum.find_value(0, fn key ->
      case Map.get(map, key) do
        value when is_integer(value) and value >= 0 -> value
        _ -> nil
      end
    end)
  end

  defp default_measurements(%__MODULE__{} = surface) do
    %{
      total_tasks: Map.get(surface.counts, :total, 0),
      completed_tasks: Map.get(surface.counts, :completed_by_kernel_evidence, 0),
      in_flight_tasks: Map.get(surface.counts, :in_flight, 0),
      queued_tasks: Map.get(surface.counts, :queued, 0),
      blocked_tasks: Map.get(surface.counts, :blocked, 0),
      retrying_tasks: Map.get(surface.counts, :retrying, 0)
    }
  end

  defp default_metadata(%__MODULE__{} = surface) do
    %{
      plan_id: surface.plan_id,
      boundary: surface.boundary,
      kernel_refs: surface.kernel_refs,
      trusted_write_authority: false
    }
  end
end
