defmodule OuroforgeExecutor.SupervisedDemo do
  @moduledoc """
  Bounded M64 supervised-executor demo helpers.

  The demo records control-plane audit entries for a worker crash/recovery path
  and a budget halt path. It does not write artifacts or ledger state; canonical
  data-plane truth remains in Rust-owned outputs reached through `ouroforge` CLI
  demos and fixture-shaped ledger/read-model inputs.
  """

  alias OuroforgeExecutor.{BudgetGate, LedgerRecovery}

  @budget_halt_fixture Path.expand(
                         "../../../../examples/producer-budget-gates-v1/budget-halt.fixture.json",
                         __DIR__
                       )

  def budget_halt_demo(path \\ @budget_halt_fixture) do
    decision = BudgetGate.from_file!(path)

    %{
      demo: :budget_halt,
      status: decision.status,
      condition_id: decision.condition_id,
      diagnosis: decision.diagnosis,
      may_assign?: decision.may_assign?,
      may_drive_cli?: decision.may_drive_cli?,
      audit: [
        %{event: :budget_checked, policy_id: decision.policy_id},
        %{event: :halted, reason: decision.stop_reason, evidence_ref: decision.last_evidence_ref}
      ]
    }
  end

  def recovery_demo(ledger \\ recovered_ledger(), checkpoint \\ %{}) do
    tasks = [
      %{"taskId" => "crash-prone-worker"},
      %{"taskId" => "trusted-apply", "trustedWriteKey" => "mutation:trusted-apply:1"},
      %{"taskId" => "post-recovery-evaluate"}
    ]

    resume = LedgerRecovery.resume(tasks, ledger, checkpoint)

    %{
      demo: :crash_recovery,
      completed_task_ids: resume.completed_task_ids,
      drive_task_ids: resume.drive_task_ids,
      blocked: resume.blocked,
      trusted_write_keys: resume.trusted_write_keys,
      audit: [
        %{event: :worker_crashed, task_id: "crash-prone-worker"},
        %{event: :ledger_replayed, completed_task_ids: resume.completed_task_ids},
        %{event: :trusted_write_deduped, keys: resume.trusted_write_keys}
      ]
    }
  end

  def combined_audit(recovery, budget_halt) do
    recovery.audit ++ budget_halt.audit
  end

  defp recovered_ledger do
    %{
      "entries" => [
        %{"taskId" => "crash-prone-worker", "status" => "completed"},
        %{
          "taskId" => "trusted-apply",
          "status" => "applied",
          "trustedWrite" => true,
          "idempotencyKey" => "mutation:trusted-apply:1"
        }
      ]
    }
  end
end
