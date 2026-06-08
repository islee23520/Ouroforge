defmodule OuroforgeExecutor.OperatorCockpit.TelemetryPanel do
  @moduledoc """
  M67-5 local telemetry/utilization panel for the operator cockpit.

  The panel summarizes local executor queue depth, concurrency, budget
  consumption, retries, backpressure, and stop-gate decisions. Values are
  read-only control-plane telemetry observations and are never mirrored to a
  remote service or interpreted as Rust artifact truth.
  """

  alias OuroforgeExecutor.OperatorCockpit.{CampaignStatus, TaskDAG}

  defstruct version: "m67-5",
            boundary: :read_only_local_telemetry_panel,
            campaign_id: nil,
            queue_depth: 0,
            runnable_frontier: 0,
            active_workers: 0,
            max_concurrency: 0,
            utilization: 0.0,
            budget: %{used: 0, limit: nil, remaining: nil, state: :unbounded},
            retries: %{attempts: 0, task_ids: []},
            backpressure: %{depth: 0, state: :clear},
            stop_gate: %{decision: :none, human_judgment_required?: false},
            remote_telemetry?: false,
            trusted_write_authority?: false,
            notes: []

  def from_inputs(%CampaignStatus{} = status, %TaskDAG{} = dag, telemetry \\ %{})
      when is_map(telemetry) do
    active_workers =
      integer(telemetry, [:active_workers, "activeWorkers"], (status.active_task && 1) || 0)

    max_concurrency =
      integer(telemetry, [:max_concurrency, "maxConcurrency"], max(active_workers, 1))

    queue_depth = integer(telemetry, [:queue_depth, "queueDepth"], length(status.waiting_tasks))
    backpressure_depth = integer(telemetry, [:backpressure_depth, "backpressureDepth"], 0)

    retry_attempts =
      integer(telemetry, [:retry_attempts, "retryAttempts"], length(status.retrying_tasks))

    stop_decision = atomish(telemetry, [:stop_gate, "stopGate"], :none)

    %__MODULE__{
      campaign_id: status.campaign_id || dag.plan_id,
      queue_depth: queue_depth,
      runnable_frontier: length(dag.runnable_frontier),
      active_workers: active_workers,
      max_concurrency: max_concurrency,
      utilization: utilization(active_workers, max_concurrency),
      budget: budget(telemetry),
      retries: %{
        attempts: retry_attempts,
        task_ids: Enum.uniq(status.retrying_tasks ++ dag.retrying_tasks)
      },
      backpressure: %{
        depth: backpressure_depth,
        state: if(backpressure_depth > 0, do: :backpressured, else: :clear)
      },
      stop_gate: %{
        decision: stop_decision,
        human_judgment_required?:
          stop_decision in [
            :budget_exhausted,
            :human_decision_required,
            :ambiguous,
            :unknown_untrusted_input
          ]
      },
      notes: notes(queue_depth, backpressure_depth, retry_attempts, stop_decision)
    }
  end

  def render(%__MODULE__{} = panel) do
    [
      "Telemetry #{panel.campaign_id}: local read-only utilization",
      "Queue depth: #{panel.queue_depth}; runnable frontier: #{panel.runnable_frontier}",
      "Concurrency: #{panel.active_workers}/#{panel.max_concurrency} (#{Float.round(panel.utilization * 100, 1)}%)",
      "Budget: #{panel.budget.used}/#{panel.budget.limit || "unbounded"} (#{panel.budget.state})",
      "Retries: #{panel.retries.attempts} attempts for #{render_ids(panel.retries.task_ids)}",
      "Backpressure: #{panel.backpressure.state} depth=#{panel.backpressure.depth}",
      "Stop gate: #{panel.stop_gate.decision}; human judgment=#{panel.stop_gate.human_judgment_required?}",
      "Remote telemetry: #{panel.remote_telemetry?}; trusted writes: #{panel.trusted_write_authority?}",
      "Notes: #{Enum.join(panel.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def fixture(state) do
    status = CampaignStatus.fixture(state)
    dag = TaskDAG.fixture(if state == :normal, do: :waiting, else: state)

    telemetry =
      case state do
        :normal ->
          %{
            active_workers: 1,
            max_concurrency: 3,
            queue_depth: 1,
            budget_used: 2,
            budget_limit: 10
          }

        :waiting ->
          %{queue_depth: 2, max_concurrency: 3}

        :retrying ->
          %{retry_attempts: 2, max_concurrency: 3}

        :budget_limited ->
          %{budget_used: 10, budget_limit: 10, stop_gate: :budget_exhausted}

        :backpressured ->
          %{backpressure_depth: 4, queue_depth: 4, active_workers: 3, max_concurrency: 3}

        :blocked ->
          %{stop_gate: :human_decision_required, max_concurrency: 1}
      end

    from_inputs(status, dag, telemetry)
  end

  def fixtures do
    [:normal, :waiting, :retrying, :budget_limited, :backpressured, :blocked]
    |> Map.new(&{&1, fixture(&1)})
  end

  defp budget(telemetry) do
    used = integer(telemetry, [:budget_used, "budgetUsed"], 0)
    limit = nullable_integer(telemetry, [:budget_limit, "budgetLimit"])
    remaining = if is_integer(limit), do: max(limit - used, 0), else: nil

    state =
      cond do
        is_integer(limit) and used >= limit -> :exhausted_requires_human_judgment
        is_integer(limit) -> :bounded
        true -> :unbounded
      end

    %{used: used, limit: limit, remaining: remaining, state: state}
  end

  defp utilization(_active, 0), do: 0.0
  defp utilization(active, max), do: active / max

  defp notes(queue_depth, backpressure_depth, retry_attempts, stop_decision) do
    []
    |> maybe(queue_depth > 0, "queue depth is local executor pressure only")
    |> maybe(backpressure_depth > 0, "backpressure is not a remote telemetry signal")
    |> maybe(retry_attempts > 0, "retry counts describe control-plane attempts, not success")
    |> maybe(
      stop_decision != :none,
      "stop-gate decisions require operator context before continuing"
    )
    |> Enum.reverse()
  end

  defp maybe(notes, true, note), do: [note | notes]
  defp maybe(notes, false, _note), do: notes

  defp integer(map, keys, default) do
    Enum.find_value(keys, default, fn key ->
      value = Map.get(map, key)
      if is_integer(value) and value >= 0, do: value, else: nil
    end)
  end

  defp nullable_integer(map, keys) do
    Enum.find_value(keys, fn key ->
      value = Map.get(map, key)
      if is_integer(value) and value >= 0, do: value, else: nil
    end)
  end

  defp atomish(map, keys, default) do
    Enum.find_value(keys, default, fn key ->
      case Map.get(map, key) do
        value when value in [:none, :budget_exhausted, :human_decision_required, :ambiguous] ->
          value

        "none" ->
          :none

        "budget_exhausted" ->
          :budget_exhausted

        "human_decision_required" ->
          :human_decision_required

        "ambiguous" ->
          :ambiguous

        value when is_binary(value) ->
          :unknown_untrusted_input

        _ ->
          nil
      end
    end)
  end

  defp render_ids([]), do: "none"
  defp render_ids(ids), do: Enum.join(ids, ", ")
end
