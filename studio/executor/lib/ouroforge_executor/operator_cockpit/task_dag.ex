defmodule OuroforgeExecutor.OperatorCockpit.TaskDAG do
  @moduledoc """
  M67-3 read-only task DAG/progress model for the operator cockpit.

  This module renders dependencies, retries, skipped work, wait gates, blocked
  tasks, and the current runnable frontier from plan plus executor observation
  inputs. It does not schedule, assign, retry, skip, or write anything.
  """

  alias OuroforgeExecutor.{ProductionPlan, ProgressSurface}

  defstruct version: "m67-3",
            boundary: :read_only_task_dag_progress,
            plan_id: nil,
            nodes: [],
            edges: [],
            runnable_frontier: [],
            wait_gates: [],
            skipped_tasks: [],
            retrying_tasks: [],
            blocked_tasks: [],
            trusted_write_authority?: false,
            notes: []

  def from_inputs(%ProductionPlan{} = plan, ledger \\ %{}, control_plane \\ %{})
      when is_map(ledger) and is_map(control_plane) do
    surface = ProgressSurface.from_artifacts(plan, ledger, control_plane)
    status_by_id = Map.new(surface.tasks, &{&1.task_id, &1})
    completed = completed_ids(surface)
    assigned = ids_with_status(surface, :in_flight)

    runnable =
      ProductionPlan.ready_set(plan, %{completed_task_ids: completed, assigned_task_ids: assigned})

    skipped = skipped_ids(control_plane)

    %__MODULE__{
      plan_id: plan.plan_id,
      nodes: Enum.map(plan.tasks, &node(&1, status_by_id, skipped)),
      edges: edges(plan),
      runnable_frontier: Enum.map(runnable, & &1.id) -- skipped,
      wait_gates: wait_gates(plan, completed, skipped),
      skipped_tasks: skipped,
      retrying_tasks: ids_with_status(surface, :retrying),
      blocked_tasks:
        ids_with_status(surface, :blocked) ++ ids_with_status(surface, :failed_by_kernel_evidence),
      notes: notes(surface, runnable, skipped)
    }
  end

  def render(%__MODULE__{} = dag) do
    [
      "Task DAG #{dag.plan_id}: read-only progress",
      "Runnable frontier: #{render_ids(dag.runnable_frontier)}",
      "Retrying: #{render_ids(dag.retrying_tasks)}",
      "Skipped: #{render_ids(dag.skipped_tasks)}",
      "Blocked: #{render_ids(dag.blocked_tasks)}",
      "Wait gates: #{render_wait_gates(dag.wait_gates)}",
      "Trusted writes: #{dag.trusted_write_authority?}",
      "Nodes: #{Enum.map_join(dag.nodes, "; ", &render_node/1)}",
      "Edges: #{Enum.map_join(dag.edges, ", ", &"#{&1.from}->#{&1.to}")}",
      "Notes: #{Enum.join(dag.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def fixture(state) do
    plan = fixture_plan()

    case state do
      :normal -> from_inputs(plan, evidence(["seed"]), %{assigned_task_ids: ["project"]})
      :waiting -> from_inputs(plan, evidence([]), %{})
      :retrying -> from_inputs(plan, evidence([]), %{"retryingTaskIds" => ["seed"]})
      :budget_limited -> from_inputs(plan, evidence([]), %{budget_limited?: true})
      :backpressured -> from_inputs(plan, evidence([]), %{backpressure_depth: 3})
      :blocked -> from_inputs(plan, evidence([]), %{"blockedTaskIds" => ["seed"]})
      :skipped -> from_inputs(plan, evidence(["seed"]), %{skipped_task_ids: ["project"]})
    end
  end

  def fixtures do
    [:normal, :waiting, :retrying, :budget_limited, :backpressured, :blocked, :skipped]
    |> Map.new(&{&1, fixture(&1)})
  end

  defp node(task, status_by_id, skipped) do
    status =
      if task.id in skipped,
        do: :skipped_by_control_plane,
        else: Map.fetch!(status_by_id, task.id).status

    %{
      task_id: task.id,
      depends_on: task.depends_on,
      command_family: task.kind,
      role: task.role,
      status: status,
      source: source_for(status, Map.fetch!(status_by_id, task.id))
    }
  end

  defp source_for(:skipped_by_control_plane, _task), do: :executor_state
  defp source_for(_status, task), do: task.source

  defp edges(plan) do
    plan.tasks
    |> Enum.flat_map(fn task -> Enum.map(task.depends_on, &%{from: &1, to: task.id}) end)
    |> Enum.sort_by(&{&1.from, &1.to})
  end

  defp wait_gates(plan, completed, skipped) do
    plan.tasks
    |> Enum.reject(&(&1.id in completed or &1.id in skipped))
    |> Enum.flat_map(fn task ->
      missing = Enum.reject(task.depends_on, &(&1 in completed or &1 in skipped))
      if missing == [], do: [], else: [%{task_id: task.id, waiting_for: missing}]
    end)
  end

  defp completed_ids(surface), do: ids_with_status(surface, :completed_by_kernel_evidence)

  defp ids_with_status(surface, status) do
    surface.tasks
    |> Enum.filter(&(&1.status == status))
    |> Enum.map(& &1.task_id)
  end

  defp skipped_ids(%{skipped_task_ids: values}) when is_list(values),
    do: Enum.map(values, &to_string/1)

  defp skipped_ids(%{"skippedTaskIds" => values}) when is_list(values),
    do: Enum.map(values, &to_string/1)

  defp skipped_ids(_), do: []

  defp notes(surface, runnable, skipped) do
    []
    |> maybe_note(
      runnable != [],
      "runnable frontier is descriptive only; no task is launched by this view"
    )
    |> maybe_note(skipped != [], "skipped tasks are displayed, not removed from Rust-owned truth")
    |> maybe_note(
      Map.get(surface.control_plane, :budget_limited?) == true,
      "budget-limited frontier requires human judgment"
    )
    |> maybe_note(
      Map.get(surface.control_plane, :backpressure_depth, 0) > 0,
      "backpressure is local executor capacity only"
    )
    |> Enum.reverse()
  end

  defp maybe_note(notes, true, note), do: [note | notes]
  defp maybe_note(notes, false, _note), do: notes

  defp render_ids([]), do: "none"
  defp render_ids(ids), do: Enum.join(ids, ", ")

  defp render_wait_gates([]), do: "none"

  defp render_wait_gates(gates),
    do: Enum.map_join(gates, "; ", &"#{&1.task_id} waits for #{Enum.join(&1.waiting_for, ",")}")

  defp render_node(node), do: "#{node.task_id}=#{node.status}"

  defp fixture_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m67-task-dag-fixture",
      "tasks" => [task("seed", []), task("project", ["seed"]), task("inspect", ["project"])]
    })
  end

  defp task(id, depends_on) do
    %{
      "taskId" => id,
      "functionAgent" => "executor",
      "role" => "local-control-plane",
      "kind" => "cli-drive",
      "dependsOn" => depends_on,
      "inputs" => ["input:#{id}"],
      "outputs" => ["output:#{id}"]
    }
  end

  defp evidence(ids) do
    %{
      "entries" =>
        Enum.map(
          ids,
          &%{"taskId" => &1, "status" => "completed", "evidenceRef" => "runs/m67/#{&1}.json"}
        )
    }
  end
end
