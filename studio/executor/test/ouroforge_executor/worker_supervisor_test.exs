defmodule OuroforgeExecutor.WorkerSupervisorTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{Worker, WorkerSupervisor}

  defp supervisor_name do
    Module.concat(__MODULE__, "Supervisor#{System.unique_integer([:positive])}")
  end

  test "a worker crash is isolated and restarted without killing sibling work" do
    name = supervisor_name()
    {:ok, _sup} = start_supervised({WorkerSupervisor, name: name, max_restarts: 3})
    {:ok, attempts} = Agent.start_link(fn -> 0 end)

    test_pid = self()

    flaky_run = fn state ->
      attempt = Agent.get_and_update(attempts, &{&1 + 1, &1 + 1})
      send(test_pid, {:flaky_attempt, attempt})

      if attempt == 1 do
        Process.exit(self(), :flaky_worker_crash)
      else
        {:ok, %{attempt: attempt, task_id: state.task_id}}
      end
    end

    stable_run = fn state -> {:ok, %{task_id: state.task_id}} end

    assert {:ok, _flaky_pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "flaky",
               task_id: "task-flaky",
               role: "producer-agent",
               run: flaky_run,
               notify: self()
             )

    assert {:ok, stable_pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "stable",
               task_id: "task-stable",
               role: "producer-agent",
               run: stable_run,
               notify: self()
             )

    assert_receive {:flaky_attempt, 1}
    assert_receive {:flaky_attempt, 2}
    assert_receive {:worker_completed, "flaky", %{attempt: 2, task_id: "task-flaky"}}
    assert_receive {:worker_completed, "stable", %{task_id: "task-stable"}}

    assert Worker.status(stable_pid).status == :completed
    assert WorkerSupervisor.count_children(name).active == 2
  end

  test "restart limits halt safely on repeated failure without trusted write result" do
    previous = Process.flag(:trap_exit, true)

    try do
      name = supervisor_name()
      {:ok, sup} = WorkerSupervisor.start_link(name: name, max_restarts: 1, max_seconds: 5)
      ref = Process.monitor(sup)
      test_pid = self()

      always_crash = fn _state ->
        send(test_pid, :crashing_attempt)
        Process.exit(self(), :repeated_worker_crash)
      end

      assert {:ok, _pid} =
               WorkerSupervisor.start_worker(name,
                 worker_id: "crashy",
                 task_id: "task-crashy",
                 role: "producer-agent",
                 run: always_crash,
                 notify: self()
               )

      assert_receive :crashing_attempt
      assert_receive :crashing_attempt
      assert_receive {:EXIT, ^sup, _reason}
      assert_receive {:DOWN, ^ref, :process, ^sup, _reason}
      refute_receive {:trusted_write, _}, 50
    after
      Process.flag(:trap_exit, previous)
    end
  end

  test "worker blocks unsupported trusted-write shaped results" do
    name = supervisor_name()
    {:ok, _sup} = start_supervised({WorkerSupervisor, name: name})

    assert {:ok, pid} =
             WorkerSupervisor.start_worker(name,
               worker_id: "blocked",
               task_id: "task-blocked",
               role: "producer-agent",
               run: fn _ -> {:trusted_write, ["ledger", "append"]} end,
               notify: self()
             )

    assert_receive {:worker_blocked, "blocked",
                    {:forbidden_worker_result, {:trusted_write, ["ledger", "append"]}}}

    assert %{
             status: :blocked,
             result: {:forbidden_worker_result, {:trusted_write, ["ledger", "append"]}}
           } = Worker.status(pid)
  end
end
