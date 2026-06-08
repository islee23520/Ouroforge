defmodule OuroforgeExecutor.SupervisedDemoTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{SupervisedDemo, WorkerSupervisor}

  @moduletag :demo

  defp supervisor_name do
    Module.concat(__MODULE__, "Supervisor#{System.unique_integer([:positive])}")
  end

  test "supervised demo recovers an injected worker crash and preserves audit trail" do
    name = supervisor_name()
    {:ok, _sup} = start_supervised({WorkerSupervisor, name: name, max_restarts: 3})
    {:ok, attempts} = Agent.start_link(fn -> 0 end)
    test_pid = self()

    crash_then_recover = fn state ->
      attempt = Agent.get_and_update(attempts, &{&1 + 1, &1 + 1})
      send(test_pid, {:demo_attempt, attempt})

      if attempt == 1 do
        Process.exit(self(), :demo_injected_crash)
      else
        {:ok, %{task_id: state.task_id, recovered_after_attempt: attempt}}
      end
    end

    assert {:ok, _pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "demo-worker",
               task_id: "crash-prone-worker",
               role: "producer-agent",
               run: crash_then_recover,
               notify: self()
             )

    assert_receive {:demo_attempt, 1}
    assert_receive {:demo_attempt, 2}

    assert_receive {:worker_completed, "demo-worker",
                    %{task_id: "crash-prone-worker", recovered_after_attempt: 2}}

    recovery = SupervisedDemo.recovery_demo()

    assert recovery.completed_task_ids == ["crash-prone-worker", "trusted-apply"]
    assert recovery.drive_task_ids == ["post-recovery-evaluate"]
    assert recovery.trusted_write_keys == ["mutation:trusted-apply:1"]
    assert recovery.blocked == []

    assert Enum.map(recovery.audit, & &1.event) == [
             :worker_crashed,
             :ledger_replayed,
             :trusted_write_deduped
           ]
  end

  test "supervised demo halts safely at a producer budget gate" do
    budget_halt = SupervisedDemo.budget_halt_demo()

    assert budget_halt.status == :halted_budget_exhausted
    assert budget_halt.condition_id == "stop-budget"
    assert budget_halt.diagnosis =~ "budget exhausted"
    refute budget_halt.may_assign?
    refute budget_halt.may_drive_cli?
    assert Enum.map(budget_halt.audit, & &1.event) == [:budget_checked, :halted]
  end

  test "combined M64 audit keeps crash recovery and budget halt visible" do
    audit =
      SupervisedDemo.combined_audit(
        SupervisedDemo.recovery_demo(),
        SupervisedDemo.budget_halt_demo()
      )

    assert Enum.map(audit, & &1.event) == [
             :worker_crashed,
             :ledger_replayed,
             :trusted_write_deduped,
             :budget_checked,
             :halted
           ]
  end
end
