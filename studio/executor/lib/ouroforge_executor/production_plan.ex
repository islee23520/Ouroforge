defmodule OuroforgeExecutor.ProductionPlan do
  @moduledoc """
  Read-only consumer for the Rust-owned M43 `producer-plan-v1` artifact.

  The executor treats the plan as data-plane input. This module parses and
  normalizes only the fields needed for deterministic scheduling; it does not
  validate Rust artifact semantics, mutate the artifact, or write scheduler
  state.
  """

  alias OuroforgeExecutor.JSON

  @schema_version "producer-plan-v1"
  @terminal_statuses MapSet.new(["completed", "complete", "done", "passed"])

  defstruct [:plan_id, :schema_version, tasks: []]

  defmodule Task do
    @moduledoc false
    defstruct [:id, :function_agent, :role, :kind, depends_on: [], inputs: [], outputs: []]
  end

  def from_file!(path) when is_binary(path) do
    path |> File.read!() |> from_json!()
  end

  def from_json!(json) when is_binary(json) do
    json |> JSON.decode!() |> from_map!()
  end

  def from_map!(%{"schemaVersion" => @schema_version, "planId" => plan_id, "tasks" => tasks})
      when is_binary(plan_id) and is_list(tasks) do
    plan = %__MODULE__{
      plan_id: plan_id,
      schema_version: @schema_version,
      tasks: Enum.map(tasks, &task_from_map!/1)
    }

    validate!(plan)
  end

  def from_map!(%{"schemaVersion" => other}) do
    raise ArgumentError, "unsupported production plan schemaVersion #{inspect(other)}"
  end

  def from_map!(_),
    do: raise(ArgumentError, "producer plan requires schemaVersion, planId, and tasks")

  def validate!(%__MODULE__{tasks: []}) do
    raise ArgumentError, "producer plan tasks must not be empty"
  end

  def validate!(%__MODULE__{} = plan) do
    ids = Enum.map(plan.tasks, & &1.id)

    duplicate = ids -- Enum.uniq(ids)
    if duplicate != [], do: raise(ArgumentError, "duplicate task id #{inspect(hd(duplicate))}")

    id_set = MapSet.new(ids)

    Enum.each(plan.tasks, fn task ->
      Enum.each(task.depends_on, fn dependency ->
        unless MapSet.member?(id_set, dependency) do
          raise ArgumentError, "task #{task.id} depends on missing task #{dependency}"
        end

        if dependency == task.id do
          raise ArgumentError, "task #{task.id} depends on itself"
        end
      end)
    end)

    ensure_acyclic!(plan.tasks)
    plan
  end

  def ready_set(%__MODULE__{} = plan, state \\ %{}) when is_map(state) do
    completed = completed_task_ids(state)
    assigned = assigned_task_ids(state)

    plan.tasks
    |> Enum.filter(fn task ->
      not MapSet.member?(completed, task.id) and
        not MapSet.member?(assigned, task.id) and
        Enum.all?(task.depends_on, &MapSet.member?(completed, &1))
    end)
    |> Enum.sort_by(& &1.id)
  end

  def assign_ready(%__MODULE__{} = plan, workers, state \\ %{})
      when is_list(workers) and is_map(state) do
    available_workers = Enum.sort(Enum.map(workers, &to_string/1))

    plan
    |> ready_set(state)
    |> Enum.zip(available_workers)
    |> Enum.map(fn {task, worker} ->
      %{task_id: task.id, worker_id: worker, role: task.role, function_agent: task.function_agent}
    end)
  end

  def completion_order(%__MODULE__{} = plan) do
    visit_all(plan.tasks)
  end

  defp task_from_map!(%{"taskId" => id} = map) when is_binary(id) do
    %Task{
      id: id,
      function_agent: string_field!(map, "functionAgent"),
      role: string_field!(map, "role"),
      kind: string_field!(map, "kind"),
      depends_on: string_list_field!(map, "dependsOn"),
      inputs: string_list_field!(map, "inputs"),
      outputs: string_list_field!(map, "outputs")
    }
  end

  defp task_from_map!(_), do: raise(ArgumentError, "producer plan task requires taskId")

  defp string_field!(map, field) do
    case Map.fetch(map, field) do
      {:ok, value} when is_binary(value) and value != "" -> value
      _ -> raise ArgumentError, "producer plan task requires string field #{field}"
    end
  end

  defp string_list_field!(map, field) do
    case Map.fetch(map, field) do
      {:ok, values} when is_list(values) ->
        if Enum.all?(values, &is_binary/1) do
          values
        else
          raise ArgumentError, "producer plan task requires string list field #{field}"
        end

      _ ->
        raise ArgumentError, "producer plan task requires string list field #{field}"
    end
  end

  defp completed_task_ids(%{completed_task_ids: values}),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp completed_task_ids(%{"completedTaskIds" => values}) when is_list(values),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp completed_task_ids(%{"completed_task_ids" => values}) when is_list(values),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp completed_task_ids(%{"completed" => values}) when is_list(values),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp completed_task_ids(%{"tasks" => task_states}) when is_map(task_states) do
    task_states
    |> Enum.filter(fn {_id, status} -> status in @terminal_statuses end)
    |> Enum.map(fn {id, _status} -> id end)
    |> MapSet.new()
  end

  defp completed_task_ids(_), do: MapSet.new()

  defp assigned_task_ids(%{assigned_task_ids: values}),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp assigned_task_ids(%{"assignedTaskIds" => values}) when is_list(values),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp assigned_task_ids(%{"assigned_task_ids" => values}) when is_list(values),
    do: MapSet.new(Enum.map(values, &to_string/1))

  defp assigned_task_ids(%{"assignments" => assignments}) when is_list(assignments) do
    assignments
    |> Enum.map(fn
      %{"taskId" => id} -> id
      %{"task_id" => id} -> id
      %{task_id: id} -> id
      other -> raise ArgumentError, "assignment missing task id: #{inspect(other)}"
    end)
    |> Enum.map(&to_string/1)
    |> MapSet.new()
  end

  defp assigned_task_ids(_), do: MapSet.new()

  defp ensure_acyclic!(tasks) do
    visit_all(tasks)
    :ok
  end

  defp visit_all(tasks) do
    by_id = Map.new(tasks, &{&1.id, &1})

    tasks
    |> Enum.reduce({MapSet.new(), []}, fn task, {visited, order} ->
      visit(task.id, by_id, MapSet.new(), visited, order)
    end)
    |> elem(1)
    |> Enum.reverse()
  end

  defp visit(id, by_id, visiting, visited, order) do
    cond do
      MapSet.member?(visited, id) ->
        {visited, order}

      MapSet.member?(visiting, id) ->
        raise ArgumentError, "producer plan dependency cycle includes task #{id}"

      true ->
        task = Map.fetch!(by_id, id)
        visiting = MapSet.put(visiting, id)

        {visited, order} =
          Enum.reduce(task.depends_on, {visited, order}, fn dependency, {visited, order} ->
            visit(dependency, by_id, visiting, visited, order)
          end)

        {MapSet.put(visited, id), [id | order]}
    end
  end
end
