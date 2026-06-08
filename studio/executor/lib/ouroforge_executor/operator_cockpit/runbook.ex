defmodule OuroforgeExecutor.OperatorCockpit.Runbook do
  @moduledoc """
  M67-4 blocked reason and copy-only runbook surface.

  This module translates blocked executor observations into plain-English causes
  and manual fallback suggestions. It deliberately returns text only: no callback,
  no executable action, no browser command, and no trusted-write authority.
  """

  alias OuroforgeExecutor.OperatorCockpit.{CampaignStatus, Contract, TaskDAG}

  defstruct version: "m67-4",
            boundary: :read_only_blocked_reason_runbook,
            campaign_id: nil,
            blocked?: false,
            reasons: [],
            suggestions: [],
            manual_cli_references: [],
            human_judgment_required?: false,
            executable_actions: [],
            trusted_write_authority?: false,
            sources: []

  @reason_copy %{
    dependency_wait: "A task is waiting for prerequisite evidence before it can become runnable.",
    retry_backoff:
      "A task is in retry/backoff; inspect the previous CLI output before deciding whether to continue.",
    budget_exhausted: "The local executor budget gate halted progress and needs operator review.",
    backpressure:
      "Local executor capacity is saturated; this is utilization pressure, not product truth.",
    human_decision:
      "The run is ambiguous and requires human judgment before any go/no-go decision.",
    kernel_failure:
      "Rust-owned CLI/evidence reported a failed or blocked task; inspect the referenced evidence.",
    unknown:
      "The cockpit cannot classify this blocked state automatically; treat it as requiring human judgment."
  }

  def from_models(%CampaignStatus{} = status, %TaskDAG{} = dag) do
    reason_codes = reason_codes(status, dag)

    %__MODULE__{
      campaign_id: status.campaign_id,
      blocked?: reason_codes != [],
      reasons: Enum.map(reason_codes, &%{code: &1, text: Map.fetch!(@reason_copy, &1)}),
      suggestions: suggestions(reason_codes),
      manual_cli_references: manual_cli_references(status, dag),
      human_judgment_required?: requires_human_judgment?(reason_codes),
      executable_actions: [],
      sources: Enum.uniq(status.sources ++ [:executor_state])
    }
  end

  def render(%__MODULE__{} = runbook) do
    [
      "Runbook #{runbook.campaign_id}: read-only blocked reason surface",
      "Blocked: #{runbook.blocked?}",
      "Human judgment required: #{runbook.human_judgment_required?}",
      "Trusted writes: #{runbook.trusted_write_authority?}",
      "Executable actions: #{length(runbook.executable_actions)}",
      "Reasons: #{Enum.map_join(runbook.reasons, "; ", &"#{&1.code}: #{&1.text}")}",
      "Copy-only suggestions: #{Enum.join(runbook.suggestions, " | ")}",
      "Manual CLI references: #{Enum.join(runbook.manual_cli_references, " | ")}",
      "Sources: #{Enum.map_join(runbook.sources, ", ", &Atom.to_string/1)}"
    ]
    |> Enum.join("\n")
  end

  def fixture(state) do
    status = CampaignStatus.fixture(state)
    dag = TaskDAG.fixture(if state == :normal, do: :waiting, else: state)
    from_models(status, dag)
  end

  def fixtures do
    [:waiting, :retrying, :budget_limited, :backpressured, :blocked]
    |> Map.new(&{&1, fixture(&1)})
  end

  defp reason_codes(status, dag) do
    []
    |> maybe(dag.wait_gates != [], :dependency_wait)
    |> maybe(status.retrying_tasks != [] or dag.retrying_tasks != [], :retry_backoff)
    |> maybe(status.status == :budget_limited_requires_human_judgment, :budget_exhausted)
    |> maybe(status.status == :backpressured, :backpressure)
    |> maybe(status.status == :blocked_requires_human_judgment, :human_decision)
    |> maybe(dag.blocked_tasks != [], :kernel_failure)
    |> Enum.reverse()
    |> case do
      [] -> []
      codes -> Enum.uniq(codes)
    end
  end

  defp suggestions(codes) do
    codes
    |> Enum.flat_map(fn
      :dependency_wait ->
        [
          "Copy the waiting task id and inspect prerequisite evidence with the manual ouroforge CLI path."
        ]

      :retry_backoff ->
        [
          "Copy the retrying task id and review the last captured stdout/stderr before choosing to resume."
        ]

      :budget_exhausted ->
        [
          "Review budget intent and decide manually whether the run should be retried with a new budget."
        ]

      :backpressure ->
        [
          "Wait for local worker capacity or reduce local concurrency; do not treat utilization as artifact success."
        ]

      :human_decision ->
        ["Escalate the ambiguous go/no-go state to a human operator."]

      :kernel_failure ->
        [
          "Open the referenced Rust-owned evidence and compare with the equivalent manual ouroforge CLI command."
        ]

      :unknown ->
        ["Stop and classify the blocker manually before continuing."]
    end)
    |> Enum.uniq()
  end

  defp manual_cli_references(status, dag) do
    ids =
      Enum.uniq(
        status.blocked_tasks ++ status.retrying_tasks ++ dag.blocked_tasks ++ dag.retrying_tasks
      )

    case ids do
      [] -> ["ouroforge evidence list", "ouroforge ledger list"]
      _ -> Enum.map(ids, &"ouroforge evidence list # inspect task #{&1}")
    end
  end

  defp requires_human_judgment?(codes),
    do: Enum.any?(codes, &(&1 in [:budget_exhausted, :human_decision, :kernel_failure, :unknown]))

  defp maybe(codes, true, code), do: [code | codes]
  defp maybe(codes, false, _code), do: codes

  def copy_only?(%__MODULE__{} = runbook) do
    runbook.executable_actions == [] and runbook.trusted_write_authority? == false and
      Enum.all?(runbook.sources, &Contract.traceable_source?/1)
  end
end
