defmodule OuroforgeExecutor.BoundedPipeline do
  @moduledoc """
  Local bounded-concurrency and backpressure model for the executor control plane.

  This module schedules `OuroforgeExecutor.ProductionPlan` tasks into local worker
  slots. It is deliberately pure: it does not spawn `ouroforge`, write artifacts,
  append ledgers, emit evidence, or certify trusted writes. Later runtime code may
  use the same admission decisions before calling the CLI adapter.
  """

  alias OuroforgeExecutor.ProductionPlan

  defmodule Config do
    @moduledoc false
    defstruct worker_limit: 1, command_family_limits: %{}
  end

  defmodule Run do
    @moduledoc false
    defstruct [
      :mode,
      :makespan_ms,
      :worker_utilization,
      :throughput_per_ms,
      :verdict_bytes,
      completed_task_ids: [],
      max_active: 0,
      max_backpressure_depth: 0,
      busy_worker_ms: 0,
      available_worker_ms: 0,
      telemetry: []
    ]
  end

  @continue_gate %{may_assign?: true, may_drive_cli?: true, diagnosis: "within budget"}

  def config(opts \\ []) when is_list(opts) do
    worker_limit = Keyword.get(opts, :worker_limit, 1)
    family_limits = Keyword.get(opts, :command_family_limits, %{})

    positive_integer!(worker_limit, :worker_limit)

    family_limits =
      family_limits
      |> Enum.map(fn {family, limit} ->
        positive_integer!(limit, {:command_family_limit, family})
        {to_string(family), limit}
      end)
      |> Map.new()

    %Config{worker_limit: worker_limit, command_family_limits: family_limits}
  end

  def run_adaptive(%ProductionPlan{} = plan, opts \\ []) when is_list(opts) do
    run(plan, Keyword.put(opts, :mode, :adaptive))
  end

  def run_fixed_pool(%ProductionPlan{} = plan, opts \\ []) when is_list(opts) do
    run(plan, Keyword.put(opts, :mode, :fixed_pool))
  end

  defp run(%ProductionPlan{} = plan, opts) do
    config = Keyword.get_lazy(opts, :config, fn -> config(opts) end)
    durations = Keyword.get(opts, :task_durations_ms, %{})
    budget = Keyword.get(opts, :budget_decision, @continue_gate)
    verdict_bytes = Keyword.get(opts, :verdict_bytes, <<>>)
    mode = Keyword.fetch!(opts, :mode)

    initial = %{
      time: 0,
      completed: [],
      in_flight: [],
      next_worker: 1,
      busy_worker_ms: 0,
      max_active: 0,
      max_backpressure_depth: 0,
      telemetry: []
    }

    plan
    |> loop(config, durations, budget, mode, initial)
    |> finish(plan, config, mode, verdict_bytes)
  end

  defp loop(%ProductionPlan{} = plan, config, durations, budget, mode, state) do
    cond do
      length(state.completed) == length(plan.tasks) ->
        state

      budget_allows_drive?(budget) ->
        {state, assigned_count} = admit_ready(plan, config, durations, state)

        cond do
          state.in_flight == [] and assigned_count == 0 ->
            %{state | telemetry: event(state, :blocked, %{reason: :no_ready_work})}

          mode == :fixed_pool and assigned_count > 0 ->
            finish_batch(plan, config, durations, budget, mode, state)

          true ->
            finish_next(plan, config, durations, budget, mode, state)
        end

      state.in_flight != [] ->
        state = %{state | telemetry: event(state, :budget_halted, budget_event(budget))}
        finish_next(plan, config, durations, budget, mode, state)

      true ->
        %{state | telemetry: event(state, :budget_halted, budget_event(budget))}
    end
  end

  defp admit_ready(%ProductionPlan{} = plan, %Config{} = config, durations, state) do
    active_families = active_family_counts(state.in_flight)
    available_workers = available_workers(config, state.in_flight, state.next_worker)

    ready =
      ProductionPlan.ready_set(plan, %{
        completed_task_ids: state.completed,
        assigned_task_ids: Enum.map(state.in_flight, & &1.task_id)
      })

    {assigned, pending, _families, _workers} =
      Enum.reduce(ready, {[], [], active_families, available_workers}, fn task,
                                                                          {assigned, pending,
                                                                           families, workers} ->
        family = task.kind
        family_count = Map.get(families, family, 0)
        family_limit = Map.get(config.command_family_limits, family, config.worker_limit)

        case workers do
          [worker_id | rest] when family_count < family_limit ->
            attempt = %{
              task_id: task.id,
              worker_id: worker_id,
              family: family,
              started_at_ms: state.time,
              duration_ms: duration_for!(durations, task.id),
              role: task.role,
              function_agent: task.function_agent
            }

            {[attempt | assigned], pending, Map.update(families, family, 1, &(&1 + 1)), rest}

          _ ->
            {assigned, [task.id | pending], families, workers}
        end
      end)

    assigned = Enum.reverse(assigned)
    pending = Enum.reverse(pending)
    next_worker = max(state.next_worker, config.worker_limit + 1)

    telemetry =
      state.telemetry
      |> prepend_pending(state, pending)
      |> then(fn events ->
        Enum.reduce(assigned, events, fn attempt, events ->
          event(
            %{state | telemetry: events},
            :assigned,
            Map.take(attempt, [:task_id, :worker_id, :family])
          )
        end)
      end)

    state = %{
      state
      | in_flight: state.in_flight ++ assigned,
        next_worker: next_worker,
        max_active: max(state.max_active, length(state.in_flight) + length(assigned)),
        max_backpressure_depth: max(state.max_backpressure_depth, length(pending)),
        telemetry: telemetry
    }

    {state, length(assigned)}
  end

  defp finish_next(plan, config, durations, budget, mode, state) do
    next_end = state.in_flight |> Enum.map(&(&1.started_at_ms + &1.duration_ms)) |> Enum.min()

    {done, still_in_flight} =
      Enum.split_with(state.in_flight, &(&1.started_at_ms + &1.duration_ms == next_end))

    done = Enum.sort_by(done, & &1.task_id)
    elapsed_busy = Enum.reduce(done, 0, &(&1.duration_ms + &2))

    state = %{
      state
      | time: next_end,
        completed: state.completed ++ Enum.map(done, & &1.task_id),
        in_flight: still_in_flight,
        busy_worker_ms: state.busy_worker_ms + elapsed_busy,
        telemetry:
          Enum.reduce(done, state.telemetry, fn attempt, events ->
            event(
              %{state | time: next_end, telemetry: events},
              :completed,
              Map.take(attempt, [:task_id, :worker_id, :family])
            )
          end)
    }

    loop(plan, config, durations, budget, mode, state)
  end

  defp finish_batch(plan, config, durations, budget, mode, state) do
    batch_end = state.in_flight |> Enum.map(&(&1.started_at_ms + &1.duration_ms)) |> Enum.max()
    done = Enum.sort_by(state.in_flight, & &1.task_id)
    elapsed_busy = Enum.reduce(done, 0, &(&1.duration_ms + &2))

    state = %{
      state
      | time: batch_end,
        completed: state.completed ++ Enum.map(done, & &1.task_id),
        in_flight: [],
        busy_worker_ms: state.busy_worker_ms + elapsed_busy,
        telemetry:
          Enum.reduce(done, state.telemetry, fn attempt, events ->
            event(
              %{state | time: batch_end, telemetry: events},
              :completed,
              Map.take(attempt, [:task_id, :worker_id, :family])
            )
          end)
    }

    loop(plan, config, durations, budget, mode, state)
  end

  defp finish(state, %ProductionPlan{} = plan, %Config{} = config, mode, verdict_bytes) do
    makespan = state.time
    available = makespan * config.worker_limit
    completed_count = length(state.completed)

    %Run{
      mode: mode,
      completed_task_ids: state.completed,
      max_active: state.max_active,
      max_backpressure_depth: state.max_backpressure_depth,
      makespan_ms: makespan,
      busy_worker_ms: state.busy_worker_ms,
      available_worker_ms: available,
      worker_utilization: ratio(state.busy_worker_ms, available),
      throughput_per_ms: ratio(completed_count, makespan),
      verdict_bytes: verdict_bytes,
      telemetry:
        Enum.reverse(
          event(state, :run_finished, %{plan_id: plan.plan_id, completed: completed_count})
        )
    }
  end

  defp available_workers(%Config{worker_limit: limit}, in_flight, _next_worker) do
    active = in_flight |> Enum.map(& &1.worker_id) |> MapSet.new()

    1..limit
    |> Enum.map(&"worker-#{&1}")
    |> Enum.reject(&MapSet.member?(active, &1))
  end

  defp active_family_counts(in_flight) do
    Enum.reduce(in_flight, %{}, fn attempt, counts ->
      Map.update(counts, attempt.family, 1, &(&1 + 1))
    end)
  end

  defp duration_for!(durations, task_id) do
    duration = Map.get(durations, task_id, Map.get(durations, to_string(task_id), 1))
    positive_integer!(duration, {:task_duration_ms, task_id})
    duration
  end

  defp budget_allows_drive?(budget) do
    Map.get(budget, :may_assign?) != false and Map.get(budget, :may_drive_cli?) != false
  end

  defp budget_event(budget) do
    %{reason: :budget_or_stop_condition, diagnosis: Map.get(budget, :diagnosis)}
  end

  defp prepend_pending(events, _state, []), do: events

  defp prepend_pending(events, state, pending_task_ids) do
    event(%{state | telemetry: events}, :backpressure, %{pending_task_ids: pending_task_ids})
  end

  defp event(%{time: time, telemetry: telemetry}, event, attrs) do
    [Map.merge(%{event: event, at_ms: time}, attrs) | telemetry]
  end

  defp ratio(_numerator, 0), do: 0.0
  defp ratio(numerator, denominator), do: numerator / denominator

  defp positive_integer!(value, _field) when is_integer(value) and value > 0, do: :ok

  defp positive_integer!(value, field) do
    raise ArgumentError, "#{inspect(field)} must be a positive integer, got #{inspect(value)}"
  end
end
