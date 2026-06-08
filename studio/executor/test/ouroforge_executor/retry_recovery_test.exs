defmodule OuroforgeExecutor.RetryRecoveryTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{LedgerRecovery, RetryPolicy}

  defp continue_gate, do: %{may_assign?: true, may_drive_cli?: true, diagnosis: "within budget"}

  defp halted_gate,
    do: %{may_assign?: false, may_drive_cli?: false, diagnosis: "budget exhausted"}

  test "retry policy backs off deterministically and halts at the configured limit" do
    policy = RetryPolicy.new(max_attempts: 3, base_backoff_ms: 25, max_backoff_ms: 60)

    assert %RetryPolicy.Decision{action: :retry, attempt: 2, delay_ms: 25} =
             RetryPolicy.decide(policy, 1, continue_gate(), :cli_failed)

    assert %RetryPolicy.Decision{action: :retry, attempt: 3, delay_ms: 50} =
             RetryPolicy.decide(policy, 2, continue_gate(), :cli_failed)

    assert %RetryPolicy.Decision{action: :halt, reason: :retry_exhausted, delay_ms: 0} =
             RetryPolicy.decide(policy, 3, continue_gate(), :cli_failed)
  end

  test "retry policy respects budget and stop-condition halts before retrying" do
    policy = RetryPolicy.new(max_attempts: 3)

    assert %RetryPolicy.Decision{
             action: :halt,
             reason: :budget_or_stop_condition,
             diagnosis: "budget exhausted"
           } = RetryPolicy.decide(policy, 1, halted_gate(), :cli_failed)
  end

  test "resume reconstructs completed work from Rust ledger even when checkpoint missed it" do
    tasks = [
      %{"taskId" => "draft"},
      %{"taskId" => "apply", "trustedWriteKey" => "mutation:apply:1"},
      %{"taskId" => "evaluate"}
    ]

    ledger = %{
      "entries" => [
        %{"taskId" => "draft", "status" => "completed"},
        %{
          "taskId" => "apply",
          "status" => "applied",
          "trustedWrite" => true,
          "idempotencyKey" => "mutation:apply:1"
        }
      ]
    }

    resume = LedgerRecovery.resume(tasks, ledger, %{})

    assert resume.completed_task_ids == ["apply", "draft"]
    assert resume.drive_task_ids == ["evaluate"]
    assert resume.trusted_write_keys == ["mutation:apply:1"]
    assert resume.blocked == []
  end

  test "resume blocks executor-local completion when Rust ledger evidence is absent" do
    tasks = [%{"taskId" => "apply", "trustedWriteKey" => "mutation:apply:missing"}]
    ledger = %{"entries" => []}
    checkpoint = %{"completedTaskIds" => ["apply"]}

    resume = LedgerRecovery.resume(tasks, ledger, checkpoint)

    assert resume.completed_task_ids == []
    assert resume.drive_task_ids == []

    assert [
             %{
               task_id: "apply",
               reason: :checkpoint_without_kernel_evidence,
               diagnosis: diagnosis
             }
           ] = resume.blocked

    assert diagnosis =~ "Rust ledger/evidence did not"
  end

  test "idempotent re-drive skips duplicate trusted writes and blocks on budget halt" do
    task = %{"taskId" => "apply", "trustedWriteKey" => "mutation:apply:1"}

    completed_ledger = %{
      "entries" => [
        %{
          "taskId" => "apply",
          "status" => "applied",
          "trustedWrite" => true,
          "idempotencyKey" => "mutation:apply:1"
        }
      ]
    }

    assert {:skip, :already_completed_by_kernel_ledger} =
             LedgerRecovery.redrive_decision(task, completed_ledger, continue_gate())

    assert {:block, %{reason: :budget_or_stop_condition}} =
             LedgerRecovery.redrive_decision(task, %{"entries" => []}, halted_gate())

    assert {:drive, %{task_id: "apply", idempotency_key: "mutation:apply:1"}} =
             LedgerRecovery.redrive_decision(task, %{"entries" => []}, continue_gate())
  end
end
