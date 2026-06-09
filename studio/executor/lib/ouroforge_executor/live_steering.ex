defmodule OuroforgeExecutor.LiveSteering do
  @moduledoc """
  Live campaign steering for the local executor control plane.

  The module validates and records directives through `OuroforgeExecutor.CLI`,
  applies accepted directive effects to ephemeral executor scheduling decisions,
  and fans out live state for Studio presentation. It never writes trusted
  artifacts directly; Rust remains the data plane and source of truth.
  """

  alias OuroforgeExecutor.{CLI, LocalPubSub, ProductionPlan, SteeringDirective}

  defmodule Result do
    @moduledoc false
    defstruct [
      :campaign_id,
      :status,
      ready_task_ids: [],
      accepted_directives: [],
      rejected_directives: [],
      paused?: false,
      constraints: [],
      pinned_approaches: [],
      excluded_approaches: [],
      evidence_refs: [],
      trusted_write_authority?: false,
      notes: []
    ]
  end

  @doc """
  Validate, record, consume, and fan out live directives for a campaign plan.
  """
  def consume(%ProductionPlan{} = plan, directives, opts \\ []) when is_list(directives) do
    runner_opts = Keyword.take(opts, [:runner, :executable, :cd, :env, :stderr_to_stdout])
    state = Keyword.get(opts, :state, %{})
    pubsub = Keyword.get(opts, :pubsub, OuroforgeExecutor.PubSub)

    {accepted, rejected} = validate_and_record_many(directives, runner_opts)
    ready = ProductionPlan.ready_set(plan, state)
    steered_ready = steer_ready(ready, accepted)
    paused? = paused?(accepted)

    result = %Result{
      campaign_id: plan.plan_id,
      status: if(paused?, do: :paused_by_validated_directive, else: :running_autonomously),
      ready_task_ids: if(paused?, do: [], else: Enum.map(steered_ready, & &1.id)),
      accepted_directives: accepted,
      rejected_directives: rejected,
      paused?: paused?,
      constraints: constraints(accepted),
      pinned_approaches: approaches(accepted, :pin_approach),
      excluded_approaches: approaches(accepted, :exclude_approach),
      evidence_refs: evidence_refs(accepted),
      notes: notes(accepted, rejected, paused?)
    }

    :ok = LocalPubSub.broadcast(topic(plan.plan_id), {:live_steering, result}, pubsub)
    result
  end

  def validate_and_record(directive, opts \\ [])

  def validate_and_record(%SteeringDirective{} = directive, opts) do
    with {:ok, directive} <- SteeringDirective.validate(directive),
         {:ok, validation} <- CLI.run(SteeringDirective.to_cli_args(directive, :validate), opts),
         {:ok, record} <- CLI.run(SteeringDirective.to_cli_args(directive, :record), opts) do
      {:ok,
       SteeringDirective.accepted(
         directive,
         evidence_ref(validation, "validation"),
         evidence_ref(record, "record")
       )}
    else
      {:error, reason} -> {:error, SteeringDirective.rejected(directive, inspect(reason))}
    end
  end

  def validate_and_record(attrs, opts) do
    attrs |> SteeringDirective.new() |> validate_and_record(opts)
  end

  def steer_ready(tasks, directives) when is_list(tasks) and is_list(directives) do
    accepted = Enum.filter(directives, &match?(%SteeringDirective{status: :accepted}, &1))
    excluded_targets = targets_for(accepted, :exclude_approach)

    priority_targets =
      targets_for(accepted, :reprioritize) ++ targets_for(accepted, :pin_approach)

    tasks
    |> Enum.reject(&(&1.id in excluded_targets or &1.kind in excluded_targets))
    |> Enum.sort_by(fn task ->
      priority = Enum.find_index(priority_targets, &(&1 == task.id or &1 == task.kind))
      {if(priority == nil, do: 1, else: 0), priority || 0, task.id}
    end)
  end

  def topic(campaign_id) when is_binary(campaign_id) and campaign_id != "" do
    "campaign:#{campaign_id}:live-steering"
  end

  defp validate_and_record_many(directives, opts) do
    directives
    |> Enum.map(&validate_and_record(&1, opts))
    |> Enum.reduce({[], []}, fn
      {:ok, directive}, {accepted, rejected} -> {[directive | accepted], rejected}
      {:error, directive}, {accepted, rejected} -> {accepted, [directive | rejected]}
    end)
    |> then(fn {accepted, rejected} -> {Enum.reverse(accepted), Enum.reverse(rejected)} end)
  end

  defp evidence_ref(%CLI.Result{stdout: stdout}, fallback) do
    stdout = String.trim(to_string(stdout))

    cond do
      stdout == "" ->
        fallback

      String.contains?(stdout, "evidenceRef=") ->
        stdout |> String.split("evidenceRef=") |> List.last()

      true ->
        stdout
    end
  end

  defp paused?(directives) do
    last_pause =
      directives
      |> Enum.filter(&(&1.action in [:pause, :resume]))
      |> List.last()

    match?(%SteeringDirective{action: :pause}, last_pause)
  end

  defp targets_for(directives, action) do
    directives
    |> Enum.filter(&(&1.action == action))
    |> Enum.map(&(&1.target || &1.approach))
    |> Enum.reject(&is_nil/1)
  end

  defp constraints(directives) do
    directives
    |> Enum.filter(&(&1.action == :add_constraint))
    |> Enum.map(& &1.constraint)
  end

  defp approaches(directives, action) do
    directives
    |> Enum.filter(&(&1.action == action))
    |> Enum.map(& &1.approach)
    |> Enum.reject(&is_nil/1)
  end

  defp evidence_refs(directives) do
    directives
    |> Enum.flat_map(&[&1.validation_evidence_ref, &1.record_evidence_ref])
    |> Enum.reject(&is_nil/1)
    |> Enum.uniq()
  end

  defp notes([], [], _paused?), do: ["no human directive supplied; autonomous loop continues"]

  defp notes(accepted, rejected, paused?) do
    []
    |> maybe_note(
      accepted != [],
      "accepted directives were validated and recorded through the Rust CLI"
    )
    |> maybe_note(rejected != [], "rejected directives remain evidence candidates only")
    |> maybe_note(
      paused?,
      "pause is a validated control-plane steering state, not artifact mutation"
    )
    |> maybe_note(not paused?, "executor remains autonomous between directives")
    |> Enum.reverse()
  end

  defp maybe_note(notes, true, note), do: [note | notes]
  defp maybe_note(notes, false, _note), do: notes
end
