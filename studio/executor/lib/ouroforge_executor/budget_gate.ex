defmodule OuroforgeExecutor.BudgetGate do
  @moduledoc """
  Control-plane runtime budget and stop-condition evaluator.

  This module consumes the Rust-owned `producer-budget-gates-v1` policy/read-model
  shape from Milestone 43. It does not redefine artifact semantics, write
  evidence, append ledgers, or certify completion; it only decides whether the
  executor may assign/drive more local work before the next `ouroforge` CLI
  boundary.
  """

  alias OuroforgeExecutor.JSON

  @schema_version "producer-budget-gates-v1"
  @required_stop_reasons [
    "budget-exhausted",
    "human-approval-required",
    "no-progress"
  ]

  defmodule Decision do
    @moduledoc false
    defstruct [
      :policy_id,
      :status,
      :stop_reason,
      :condition_id,
      :diagnosis,
      :last_evidence_ref,
      pending_human_gate_ids: [],
      budget: %{},
      usage: %{},
      may_assign?: false,
      may_drive_cli?: false,
      telemetry: []
    ]
  end

  def from_file!(path) when is_binary(path) do
    path |> File.read!() |> from_json!()
  end

  def from_json!(json) when is_binary(json) do
    json |> JSON.decode!() |> evaluate!()
  end

  def evaluate!(%{"schemaVersion" => @schema_version} = policy) do
    validate!(policy)

    budget = Map.fetch!(policy, "budget")
    usage = Map.fetch!(policy, "usage")
    stop_conditions = Map.fetch!(policy, "stopConditions")
    pending_gate_ids = pending_human_gate_ids(policy)
    last_evidence_ref = Map.fetch!(usage, "lastEvidenceRef")

    {status, stop_reason, diagnosis} =
      classify(budget, usage, pending_gate_ids, last_evidence_ref)

    condition_id = if stop_reason, do: condition_id_for!(stop_conditions, stop_reason), else: nil

    %Decision{
      policy_id: Map.fetch!(policy, "policyId"),
      status: status,
      stop_reason: stop_reason,
      condition_id: condition_id,
      diagnosis: diagnosis,
      last_evidence_ref: last_evidence_ref,
      pending_human_gate_ids: pending_gate_ids,
      budget: budget,
      usage: usage,
      may_assign?: status == :continue,
      may_drive_cli?: status == :continue,
      telemetry: telemetry(status, budget, usage, pending_gate_ids, last_evidence_ref)
    }
  end

  def evaluate!(%{"schemaVersion" => other}) do
    raise ArgumentError, "unsupported producer budget gate schemaVersion #{inspect(other)}"
  end

  def evaluate!(_), do: raise(ArgumentError, "producer budget gate policy requires schemaVersion")

  def halt?(%Decision{status: :continue}), do: false
  def halt?(%Decision{}), do: true

  defp validate!(policy) do
    required_fields = [
      "policyId",
      "orchestrationRef",
      "budget",
      "usage",
      "stopConditions",
      "humanApprovalGates",
      "evidenceRefs",
      "boundary"
    ]

    Enum.each(required_fields, fn field ->
      unless Map.has_key?(policy, field),
        do: raise(ArgumentError, "producer budget gate missing #{field}")
    end)

    budget = Map.fetch!(policy, "budget")
    usage = Map.fetch!(policy, "usage")

    positive_integer!(budget, "maxIterations")
    positive_integer!(budget, "maxCostUnits")
    positive_integer!(budget, "noProgressWindow")
    non_negative_integer!(usage, "iterationCount")
    non_negative_integer!(usage, "costUnits")
    non_negative_integer!(usage, "noProgressSteps")
    non_empty_string!(usage, "lastEvidenceRef")

    if budget["noProgressWindow"] > budget["maxIterations"] do
      raise ArgumentError, "producer budget gate noProgressWindow must not exceed maxIterations"
    end

    validate_stop_conditions!(Map.fetch!(policy, "stopConditions"))
    validate_human_gates!(Map.fetch!(policy, "humanApprovalGates"))
    :ok
  end

  defp classify(budget, usage, pending_gate_ids, evidence_ref) do
    cond do
      usage["iterationCount"] >= budget["maxIterations"] or
          usage["costUnits"] >= budget["maxCostUnits"] ->
        {:halted_budget_exhausted, "budget-exhausted",
         "budget exhausted: iterations #{usage["iterationCount"]}/#{budget["maxIterations"]} cost #{usage["costUnits"]}/#{budget["maxCostUnits"]}; diagnosis evidence #{evidence_ref}"}

      pending_gate_ids != [] ->
        {:blocked_human_gate, "human-approval-required",
         "human approval required: pending gates #{Enum.join(pending_gate_ids, ",")}; diagnosis evidence #{evidence_ref}"}

      usage["noProgressSteps"] >= budget["noProgressWindow"] ->
        {:stopped_no_progress, "no-progress",
         "no progress: trailing #{usage["noProgressSteps"]} steps reached window #{budget["noProgressWindow"]}; diagnosis evidence #{evidence_ref}"}

      true ->
        {:continue, nil,
         "within budget, all mandatory human approval gates approved, and no-progress window not reached"}
    end
  end

  defp pending_human_gate_ids(policy) do
    policy
    |> Map.fetch!("humanApprovalGates")
    |> Enum.filter(&(Map.get(&1, "status") == "pending"))
    |> Enum.map(&Map.fetch!(&1, "gateId"))
    |> Enum.sort()
  end

  defp condition_id_for!(stop_conditions, reason) do
    stop_conditions
    |> Enum.find(&(Map.get(&1, "reason") == reason))
    |> case do
      %{"conditionId" => condition_id} -> condition_id
      _ -> raise ArgumentError, "producer budget gate missing stop condition for #{reason}"
    end
  end

  defp validate_stop_conditions!(conditions) when is_list(conditions) and conditions != [] do
    reasons = Enum.map(conditions, &Map.get(&1, "reason"))

    Enum.each(@required_stop_reasons, fn reason ->
      unless reason in reasons do
        raise ArgumentError, "producer budget gate missing required stop condition #{reason}"
      end
    end)
  end

  defp validate_stop_conditions!(_),
    do: raise(ArgumentError, "producer budget gate stopConditions must not be empty")

  defp validate_human_gates!(gates) when is_list(gates) and gates != [] do
    Enum.each(gates, fn gate ->
      non_empty_string!(gate, "gateId")
      status = Map.get(gate, "status")

      unless status in ["pending", "approved"] do
        raise ArgumentError,
              "producer budget gate unsupported human gate status #{inspect(status)}"
      end
    end)
  end

  defp validate_human_gates!(_),
    do: raise(ArgumentError, "producer budget gate humanApprovalGates must not be empty")

  defp telemetry(status, budget, usage, pending_gate_ids, evidence_ref) do
    [
      %{
        kind: :budget_gate,
        status: status,
        iteration_count: usage["iterationCount"],
        max_iterations: budget["maxIterations"],
        cost_units: usage["costUnits"],
        max_cost_units: budget["maxCostUnits"],
        no_progress_steps: usage["noProgressSteps"],
        no_progress_window: budget["noProgressWindow"],
        pending_human_gate_ids: pending_gate_ids,
        evidence_ref: evidence_ref,
        boundary: :control_plane_only
      }
    ]
  end

  defp positive_integer!(map, field) do
    value = Map.get(map, field)

    unless is_integer(value) and value > 0 do
      raise ArgumentError, "producer budget gate #{field} must be a positive integer"
    end
  end

  defp non_negative_integer!(map, field) do
    value = Map.get(map, field)

    unless is_integer(value) and value >= 0 do
      raise ArgumentError, "producer budget gate #{field} must be a non-negative integer"
    end
  end

  defp non_empty_string!(map, field) do
    value = Map.get(map, field)

    unless is_binary(value) and value != "" do
      raise ArgumentError, "producer budget gate #{field} must be a non-empty string"
    end
  end
end
