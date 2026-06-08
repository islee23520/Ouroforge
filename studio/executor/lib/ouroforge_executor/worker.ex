defmodule OuroforgeExecutor.Worker do
  @moduledoc """
  Supervised control-plane worker for one executor task attempt.

  The worker owns only local operational state: task id, role, status, attempt
  result, and crash/restart behavior. It does not call the Rust kernel directly,
  write artifacts, append ledgers, emit evidence, or certify trusted writes.
  Kernel actions remain behind the constrained `OuroforgeExecutor.CLI` adapter.
  """

  use GenServer

  @enforce_keys [:worker_id, :task_id, :role, :run]
  defstruct [:worker_id, :task_id, :role, :run, :notify, status: :starting, result: nil]

  def child_spec(opts) when is_list(opts) do
    worker_id = Keyword.fetch!(opts, :worker_id)

    %{
      id: {__MODULE__, worker_id},
      start: {__MODULE__, :start_link, [opts]},
      restart: Keyword.get(opts, :restart, :permanent),
      shutdown: 5_000,
      type: :worker
    }
  end

  def start_link(opts) when is_list(opts) do
    GenServer.start_link(__MODULE__, opts)
  end

  def status(pid) when is_pid(pid) do
    GenServer.call(pid, :status)
  end

  @impl true
  def init(opts) do
    state = %__MODULE__{
      worker_id: Keyword.fetch!(opts, :worker_id),
      task_id: Keyword.fetch!(opts, :task_id),
      role: Keyword.fetch!(opts, :role),
      run: Keyword.fetch!(opts, :run),
      notify: Keyword.get(opts, :notify)
    }

    {:ok, state, {:continue, :run}}
  end

  @impl true
  def handle_continue(:run, %__MODULE__{} = state) do
    notify(state, {:worker_started, state.worker_id, state.task_id})

    case state.run.(state) do
      {:ok, result} ->
        state = %{state | status: :completed, result: result}
        notify(state, {:worker_completed, state.worker_id, result})
        {:noreply, state}

      {:error, reason} ->
        state = %{state | status: :blocked, result: reason}
        notify(state, {:worker_blocked, state.worker_id, reason})
        {:noreply, state}

      {:trusted_write, _} = forbidden ->
        state = %{state | status: :blocked, result: {:forbidden_worker_result, forbidden}}
        notify(state, {:worker_blocked, state.worker_id, state.result})
        {:noreply, state}

      other ->
        raise ArgumentError, "worker returned unsupported control-plane result #{inspect(other)}"
    end
  end

  @impl true
  def handle_call(:status, _from, %__MODULE__{} = state) do
    {:reply,
     %{
       worker_id: state.worker_id,
       task_id: state.task_id,
       role: state.role,
       status: state.status,
       result: state.result
     }, state}
  end

  defp notify(%__MODULE__{notify: nil}, _message), do: :ok
  defp notify(%__MODULE__{notify: pid}, message) when is_pid(pid), do: send(pid, message)
end
