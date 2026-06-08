defmodule OuroforgeExecutor.LedgerRecovery do
  @moduledoc """
  Resume-from-ledger reconstruction for executor control-plane campaigns.

  Rust-owned ledger/evidence/read-model data is treated as durable truth. Local
  executor checkpoints are subordinate hints only: if they claim completion that
  the ledger/evidence does not prove, recovery fails closed to operator review.
  """

  defmodule Resume do
    @moduledoc false
    defstruct completed_task_ids: [], drive_task_ids: [], blocked: [], trusted_write_keys: []
  end

  @completed_statuses MapSet.new(["completed", "complete", "applied", "accepted", "passed"])

  def resume(tasks, ledger, checkpoint \\ %{}) when is_list(tasks) and is_map(ledger) do
    entries = Map.get(ledger, "entries", [])
    completed_by_task = completed_by_task(entries)
    completed_trusted_keys = completed_trusted_keys(entries)
    checkpoint_completed = MapSet.new(Map.get(checkpoint, "completedTaskIds", []))

    {completed, drive, blocked} =
      Enum.reduce(tasks, {[], [], []}, fn task, {completed, drive, blocked} ->
        task_id = task_id!(task)
        trusted_key = Map.get(task, "trustedWriteKey") || Map.get(task, :trusted_write_key)

        cond do
          trusted_key && MapSet.member?(completed_trusted_keys, trusted_key) ->
            {[task_id | completed], drive, blocked}

          MapSet.member?(completed_by_task, task_id) ->
            {[task_id | completed], drive, blocked}

          MapSet.member?(checkpoint_completed, task_id) ->
            {completed, drive,
             [
               %{
                 task_id: task_id,
                 reason: :checkpoint_without_kernel_evidence,
                 diagnosis:
                   "executor checkpoint claimed completion but Rust ledger/evidence did not"
               }
               | blocked
             ]}

          true ->
            {completed, [task_id | drive], blocked}
        end
      end)

    %Resume{
      completed_task_ids: Enum.sort(completed),
      drive_task_ids: Enum.reverse(drive),
      blocked: Enum.reverse(blocked),
      trusted_write_keys: completed_trusted_keys |> MapSet.to_list() |> Enum.sort()
    }
  end

  def redrive_decision(task, ledger, budget_decision) when is_map(task) and is_map(ledger) do
    resume = resume([task], ledger)
    task_id = task_id!(task)

    cond do
      task_id in resume.completed_task_ids ->
        {:skip, :already_completed_by_kernel_ledger}

      resume.blocked != [] ->
        {:block, hd(resume.blocked)}

      Map.get(budget_decision, :may_drive_cli?) == false ->
        {:block,
         %{
           task_id: task_id,
           reason: :budget_or_stop_condition,
           diagnosis: budget_decision.diagnosis
         }}

      true ->
        {:drive,
         %{
           task_id: task_id,
           idempotency_key: Map.get(task, "trustedWriteKey") || Map.get(task, :trusted_write_key)
         }}
    end
  end

  defp completed_by_task(entries) do
    entries
    |> Enum.filter(&(Map.get(&1, "status") in @completed_statuses))
    |> Enum.map(&Map.get(&1, "taskId"))
    |> Enum.reject(&is_nil/1)
    |> MapSet.new()
  end

  defp completed_trusted_keys(entries) do
    entries
    |> Enum.filter(
      &(Map.get(&1, "trustedWrite") == true and Map.get(&1, "status") in @completed_statuses)
    )
    |> Enum.map(&Map.get(&1, "idempotencyKey"))
    |> Enum.reject(&is_nil/1)
    |> MapSet.new()
  end

  defp task_id!(%{"taskId" => id}) when is_binary(id) and id != "", do: id
  defp task_id!(%{task_id: id}) when is_binary(id) and id != "", do: id
  defp task_id!(_), do: raise(ArgumentError, "resume task requires taskId")
end
