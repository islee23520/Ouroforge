defmodule OuroforgeExecutor.ScenarioCoverageV56Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{BudgetGate, LedgerRecovery, RetryPolicy, Worker, WorkerSupervisor}

  @coverage_doc Path.expand("../../../../docs/scenario-coverage-v56.md", __DIR__)
  @budget_fixture Path.expand(
                    "../../../../examples/producer-budget-gates-v1/budget-halt.fixture.json",
                    __DIR__
                  )

  defp supervisor_name do
    Module.concat(__MODULE__, "Supervisor#{System.unique_integer([:positive])}")
  end

  defp continue_gate, do: %{may_assign?: true, may_drive_cli?: true, diagnosis: "within budget"}

  test "v56 records supervised executor coverage and guardrails" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v56"
    assert doc =~ "#1944"
    assert doc =~ "V56.supervision.crash_isolation"
    assert doc =~ "V56.supervision.restart_limit"
    assert doc =~ "V56.budget.stop_halts"
    assert doc =~ "V56.retry.backoff"
    assert doc =~ "V56.recovery.resume_idempotency"
    assert doc =~ "Elixir/OTP = control plane only; Rust kernel = data plane"
    assert doc =~ "executor reaches the kernel only via the frozen `ouroforge` CLI surface"
    assert doc =~ "#1 and #23 remain open"
  end

  test "crash isolation restarts flaky work while a sibling completes" do
    name = supervisor_name()
    {:ok, _sup} = start_supervised({WorkerSupervisor, name: name, max_restarts: 3})
    {:ok, attempts} = Agent.start_link(fn -> 0 end)
    test_pid = self()

    flaky_run = fn state ->
      attempt = Agent.get_and_update(attempts, &{&1 + 1, &1 + 1})
      send(test_pid, {:v56_flaky_attempt, attempt})

      if attempt == 1 do
        Process.exit(self(), :v56_injected_crash)
      else
        {:ok, %{task_id: state.task_id, attempt: attempt}}
      end
    end

    assert {:ok, _flaky_pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "v56-flaky",
               task_id: "v56-flaky-task",
               role: "producer-agent",
               run: flaky_run,
               notify: self()
             )

    assert {:ok, stable_pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "v56-stable",
               task_id: "v56-stable-task",
               role: "producer-agent",
               run: fn state -> {:ok, %{task_id: state.task_id}} end,
               notify: self()
             )

    assert_receive {:v56_flaky_attempt, 1}
    assert_receive {:v56_flaky_attempt, 2}
    assert_receive {:worker_completed, "v56-flaky", %{task_id: "v56-flaky-task", attempt: 2}}
    assert_receive {:worker_completed, "v56-stable", %{task_id: "v56-stable-task"}}
    assert Worker.status(stable_pid).status == :completed
  end

  test "restart limits halt safely without trusted-write side effects" do
    previous = Process.flag(:trap_exit, true)

    try do
      name = supervisor_name()
      {:ok, sup} = WorkerSupervisor.start_link(name: name, max_restarts: 1, max_seconds: 5)
      ref = Process.monitor(sup)
      test_pid = self()

      assert {:ok, _pid} =
               WorkerSupervisor.start_worker(name,
                 worker_id: "v56-crashy",
                 task_id: "v56-crashy-task",
                 role: "producer-agent",
                 run: fn _state ->
                   send(test_pid, :v56_crash_attempt)
                   Process.exit(self(), :v56_repeated_crash)
                 end,
                 notify: self()
               )

      assert_receive :v56_crash_attempt
      assert_receive :v56_crash_attempt
      assert_receive {:EXIT, ^sup, _reason}
      assert_receive {:DOWN, ^ref, :process, ^sup, _reason}
      refute_receive {:trusted_write, _}, 50
    after
      Process.flag(:trap_exit, previous)
    end
  end

  test "budget and stop halts prevent assignment and CLI drive" do
    decision = BudgetGate.from_file!(@budget_fixture)

    assert BudgetGate.halt?(decision)
    refute decision.may_assign?
    refute decision.may_drive_cli?
    assert decision.stop_reason == "budget-exhausted"
    assert decision.diagnosis =~ decision.last_evidence_ref
  end

  test "retry backoff is deterministic and bounded" do
    policy = RetryPolicy.new(max_attempts: 3, base_backoff_ms: 10, max_backoff_ms: 15)

    assert %RetryPolicy.Decision{action: :retry, attempt: 2, delay_ms: 10} =
             RetryPolicy.decide(policy, 1, continue_gate(), :cli_failed)

    assert %RetryPolicy.Decision{action: :retry, attempt: 3, delay_ms: 15} =
             RetryPolicy.decide(policy, 2, continue_gate(), :cli_failed)

    assert %RetryPolicy.Decision{action: :halt, reason: :retry_exhausted, delay_ms: 0} =
             RetryPolicy.decide(policy, 3, continue_gate(), :cli_failed)
  end

  test "resume idempotency is derived from Rust-owned ledger evidence" do
    tasks = [
      %{"taskId" => "draft"},
      %{"taskId" => "apply", "trustedWriteKey" => "mutation:apply:v56"},
      %{"taskId" => "verify"}
    ]

    ledger = %{
      "entries" => [
        %{"taskId" => "draft", "status" => "completed"},
        %{
          "taskId" => "apply",
          "status" => "applied",
          "trustedWrite" => true,
          "idempotencyKey" => "mutation:apply:v56"
        }
      ]
    }

    first = LedgerRecovery.resume(tasks, ledger, %{})
    second = LedgerRecovery.resume(tasks, ledger, %{"completedTaskIds" => ["draft", "apply"]})

    assert first == second
    assert first.completed_task_ids == ["apply", "draft"]
    assert first.drive_task_ids == ["verify"]
    assert first.trusted_write_keys == ["mutation:apply:v56"]
    assert first.blocked == []

    assert {:skip, :already_completed_by_kernel_ledger} =
             LedgerRecovery.redrive_decision(Enum.at(tasks, 1), ledger, continue_gate())
  end
end
