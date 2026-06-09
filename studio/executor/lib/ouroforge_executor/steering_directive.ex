defmodule OuroforgeExecutor.SteeringDirective do
  @moduledoc """
  Live campaign steering directive captured by the Elixir control plane.

  A directive is intervention-as-evidence: opt-in human input that must be
  validated and recorded by the Rust data plane before it can influence trusted
  campaign behavior. This struct is presentation/control-plane state only and has
  no artifact write authority.
  """

  @actions [:reprioritize, :pin_approach, :exclude_approach, :add_constraint, :pause, :resume]
  @statuses [:pending, :accepted, :rejected, :blocked]

  defstruct [
    :id,
    :campaign_id,
    :action,
    :target,
    :approach,
    :constraint,
    :reason,
    :actor_id,
    :issued_at,
    :record_evidence_ref,
    :validation_evidence_ref,
    status: :pending,
    base_refs: []
  ]

  def actions, do: @actions
  def statuses, do: @statuses

  def new(attrs) when is_map(attrs) or is_list(attrs) do
    attrs = normalize_attrs(attrs)

    %__MODULE__{
      id: value(attrs, :id),
      campaign_id: value(attrs, :campaign_id),
      action: normalize_action(value(attrs, :action)),
      target: value(attrs, :target),
      approach: value(attrs, :approach),
      constraint: value(attrs, :constraint),
      reason: value(attrs, :reason),
      actor_id: value(attrs, :actor_id),
      issued_at: value(attrs, :issued_at),
      base_refs: List.wrap(value(attrs, :base_refs) || []),
      status: normalize_status(value(attrs, :status) || :pending),
      validation_evidence_ref: value(attrs, :validation_evidence_ref),
      record_evidence_ref: value(attrs, :record_evidence_ref)
    }
  end

  def validate(%__MODULE__{} = directive) do
    with :ok <- require_text(:id, directive.id),
         :ok <- require_text(:campaign_id, directive.campaign_id),
         :ok <- require_member(:action, directive.action, @actions),
         :ok <- require_member(:status, directive.status, @statuses),
         :ok <- require_text(:actor_id, directive.actor_id),
         :ok <- require_text(:reason, directive.reason),
         :ok <- require_text(:issued_at, directive.issued_at),
         :ok <- validate_action_payload(directive),
         :ok <- validate_refs(directive.base_refs) do
      {:ok, directive}
    end
  end

  def validate(other), do: {:error, {:invalid_directive, other}}

  def topic(%__MODULE__{campaign_id: campaign_id}), do: "campaign:#{campaign_id}:directives"

  def to_cli_args(%__MODULE__{} = directive, phase) when phase in [:validate, :record] do
    [
      "loop",
      "step",
      directive.campaign_id,
      "--directive-phase",
      Atom.to_string(phase),
      "--directive-id",
      directive.id,
      "--directive-action",
      Atom.to_string(directive.action),
      "--actor-id",
      directive.actor_id,
      "--reason",
      directive.reason
    ] ++ optional_cli_args(directive)
  end

  def accepted(%__MODULE__{} = directive, validation_ref, record_ref) do
    %__MODULE__{
      directive
      | status: :accepted,
        validation_evidence_ref: validation_ref,
        record_evidence_ref: record_ref
    }
  end

  def rejected(%__MODULE__{} = directive, reason) do
    %__MODULE__{directive | status: :rejected, record_evidence_ref: to_string(reason)}
  end

  defp normalize_attrs(attrs) when is_list(attrs), do: Map.new(attrs)
  defp normalize_attrs(attrs) when is_map(attrs), do: attrs

  defp value(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key)) || Map.get(attrs, camelize(key))
  end

  defp camelize(key) do
    key
    |> Atom.to_string()
    |> String.split("_")
    |> then(fn [head | tail] -> head <> Enum.map_join(tail, "", &String.capitalize/1) end)
  end

  defp normalize_action(action) when is_atom(action), do: action

  defp normalize_action(action) when is_binary(action) do
    action |> String.replace("-", "_") |> String.to_existing_atom()
  rescue
    ArgumentError -> :unknown
  end

  defp normalize_action(_), do: :unknown

  defp normalize_status(status) when is_atom(status), do: status

  defp normalize_status(status) when is_binary(status) do
    status |> String.replace("-", "_") |> String.to_existing_atom()
  rescue
    ArgumentError -> :unknown
  end

  defp normalize_status(_), do: :unknown

  defp require_text(_field, value) when is_binary(value) and value != "", do: :ok
  defp require_text(field, _value), do: {:error, {:missing_text, field}}

  defp require_member(field, value, values) do
    if value in values, do: :ok, else: {:error, {:unsupported_value, field, value}}
  end

  defp validate_action_payload(%__MODULE__{action: action} = directive) do
    case action do
      :reprioritize -> require_text(:target, directive.target)
      :pin_approach -> require_text(:approach, directive.approach)
      :exclude_approach -> require_text(:approach, directive.approach)
      :add_constraint -> require_text(:constraint, directive.constraint)
      :pause -> :ok
      :resume -> :ok
    end
  end

  defp validate_refs(refs) do
    if Enum.all?(refs, &(is_binary(&1) and &1 != "" and not String.contains?(&1, ".."))) do
      :ok
    else
      {:error, {:invalid_base_refs, refs}}
    end
  end

  defp optional_cli_args(%__MODULE__{} = directive) do
    []
    |> append_arg("--target", directive.target)
    |> append_arg("--approach", directive.approach)
    |> append_arg("--constraint", directive.constraint)
    |> append_many("--base-ref", directive.base_refs)
  end

  defp append_arg(args, _flag, nil), do: args
  defp append_arg(args, _flag, ""), do: args
  defp append_arg(args, flag, value), do: args ++ [flag, value]

  defp append_many(args, flag, values) do
    Enum.reduce(values, args, fn value, acc -> append_arg(acc, flag, value) end)
  end
end
