defmodule OuroforgeExecutor.OperatorCockpit.CampaignStatus do
  @moduledoc """
  M67-2 read-only campaign status model for the local operator cockpit.

  The model summarizes where a campaign is, which task is active, which tasks
  are waiting, and which Rust-owned evidence references exist. It is derived
  only from executor state, telemetry-shaped inputs, and `ouroforge` CLI/evidence
  references; it never mutates run state or trusted artifacts.
  """

  alias OuroforgeExecutor.OperatorCockpit.Contract
  alias OuroforgeExecutor.{ProductionPlan, ProgressSurface}

  defstruct version: "m67-2",
            boundary: :read_only_campaign_status,
            campaign_id: nil,
            status: :unknown,
            active_task: nil,
            waiting_tasks: [],
            retrying_tasks: [],
            blocked_tasks: [],
            evidence_refs: [],
            sources: [],
            human_judgment: [],
            trusted_write_authority?: false,
            notes: []

  def from_inputs(%ProductionPlan{} = plan, ledger \\ %{}, control_plane \\ %{})
      when is_map(ledger) and is_map(control_plane) do
    surface = ProgressSurface.from_artifacts(plan, ledger, control_plane)
    active = Enum.find(surface.tasks, &(&1.status == :in_flight))
    waiting = waiting_tasks(surface.tasks)
    retrying = ids_by_status(surface.tasks, :retrying)

    blocked =
      ids_by_status(surface.tasks, :blocked) ++
        ids_by_status(surface.tasks, :failed_by_kernel_evidence)

    %__MODULE__{
      campaign_id: plan.plan_id,
      status: campaign_status(surface, active, waiting, retrying, blocked),
      active_task: task_summary(active),
      waiting_tasks: Enum.map(waiting, &task_summary/1),
      retrying_tasks: retrying,
      blocked_tasks: blocked,
      evidence_refs: surface.kernel_refs,
      sources: sources(surface),
      human_judgment: human_judgment(blocked, surface),
      notes: notes(surface, active, waiting, retrying, blocked)
    }
  end

  def render(%__MODULE__{} = model) do
    [
      "Campaign #{model.campaign_id}: #{model.status}",
      "Boundary: read-only; trusted writes: #{model.trusted_write_authority?}",
      "Active task: #{render_task(model.active_task)}",
      "Waiting tasks: #{render_task_ids(model.waiting_tasks)}",
      "Retrying tasks: #{render_ids(model.retrying_tasks)}",
      "Blocked tasks: #{render_ids(model.blocked_tasks)}",
      "Evidence refs: #{render_ids(model.evidence_refs)}",
      "Sources: #{model.sources |> Enum.map(&Atom.to_string/1) |> Enum.join(", ")}",
      "Human judgment: #{render_ids(model.human_judgment)}",
      "Notes: #{Enum.join(model.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def fixtures do
    Enum.map(
      [:normal, :waiting, :retrying, :budget_limited, :backpressured, :blocked],
      fn state ->
        {state, fixture(state)}
      end
    )
    |> Map.new()
  end

  def fixture(state) do
    plan = fixture_plan()

    case state do
      :normal ->
        from_inputs(plan, evidence(["seed"]), %{assigned_task_ids: ["project"], active_workers: 1})

      :waiting ->
        from_inputs(plan, evidence([]), %{queued_ready_tasks: 1})

      :retrying ->
        from_inputs(plan, evidence([]), %{"retryingTaskIds" => ["seed"], retry_attempts: 1})

      :budget_limited ->
        from_inputs(plan, evidence([]), %{budget_limited?: true, stop_gate: :budget_exhausted})

      :backpressured ->
        from_inputs(plan, evidence([]), %{backpressure_depth: 2, queued_ready_tasks: 2})

      :blocked ->
        from_inputs(plan, evidence([]), %{
          "blockedTaskIds" => ["seed"],
          :stop_gate => :human_decision_required
        })
    end
  end

  defp campaign_status(surface, active, waiting, retrying, blocked) do
    cond do
      blocked != [] ->
        :blocked_requires_human_judgment

      control_flag?(surface, :budget_limited?) ->
        :budget_limited_requires_human_judgment

      control_integer(surface, :backpressure_depth) > 0 ->
        :backpressured

      retrying != [] ->
        :retrying

      active != nil ->
        :running

      waiting != [] ->
        :waiting

      Map.get(surface.counts, :completed_by_kernel_evidence, 0) ==
          Map.get(surface.counts, :total, 0) ->
        :complete_by_kernel_evidence

      true ->
        :queued
    end
  end

  defp waiting_tasks(tasks) do
    Enum.filter(tasks, &(&1.status == :queued))
  end

  defp ids_by_status(tasks, status) do
    tasks
    |> Enum.filter(&(&1.status == status))
    |> Enum.map(& &1.task_id)
  end

  defp task_summary(nil), do: nil

  defp task_summary(task) do
    %{
      task_id: task.task_id,
      role: task.role,
      command_family: task.command_family,
      status: task.status,
      source: task.source,
      evidence_ref: task.kernel_ref
    }
  end

  defp sources(surface) do
    surface.tasks
    |> Enum.map(& &1.source)
    |> Enum.concat([:executor_state])
    |> Enum.uniq()
    |> Enum.filter(&Contract.traceable_source?(normalize_source(&1)))
    |> Enum.map(&normalize_source/1)
    |> Enum.uniq()
    |> Enum.sort()
  end

  defp normalize_source(:rust_kernel_artifact), do: :ouroforge_cli_output
  defp normalize_source(:executor_control_plane_observation), do: :executor_state
  defp normalize_source(source), do: source

  defp human_judgment([], surface) do
    if control_flag?(surface, :budget_limited?), do: [:ambiguous_go_no_go], else: []
  end

  defp human_judgment(_blocked, _surface), do: [:ambiguous_go_no_go]

  defp notes(surface, active, waiting, retrying, blocked) do
    []
    |> maybe_note(active != nil, "active task is an executor observation, not artifact truth")
    |> maybe_note(waiting != [], "waiting tasks are pending dependency or worker availability")
    |> maybe_note(retrying != [], "retry state remains control-plane only")
    |> maybe_note(
      blocked != [],
      "blocked state requires operator judgment before any go/no-go decision"
    )
    |> maybe_note(
      control_flag?(surface, :budget_limited?),
      "budget stop gate requires human review"
    )
    |> maybe_note(
      control_integer(surface, :backpressure_depth) > 0,
      "backpressure is local executor utilization only"
    )
    |> Enum.reverse()
  end

  defp maybe_note(notes, true, note), do: [note | notes]
  defp maybe_note(notes, false, _note), do: notes

  defp control_flag?(surface, key), do: Map.get(surface.control_plane, key) == true
  defp control_integer(surface, key), do: Map.get(surface.control_plane, key, 0)

  defp render_task(nil), do: "none"
  defp render_task(task), do: "#{task.task_id} (#{task.status}, #{task.source})"

  defp render_task_ids([]), do: "none"
  defp render_task_ids(tasks), do: tasks |> Enum.map(& &1.task_id) |> Enum.join(", ")
  defp render_ids([]), do: "none"
  defp render_ids(values), do: Enum.join(values, ", ")

  defp fixture_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m67-campaign-status-fixture",
      "tasks" => [
        task("seed", []),
        task("project", ["seed"]),
        task("inspect", ["project"])
      ]
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
        Enum.map(ids, fn id ->
          %{"taskId" => id, "status" => "completed", "evidenceRef" => "runs/m67/#{id}.json"}
        end)
    }
  end
end
