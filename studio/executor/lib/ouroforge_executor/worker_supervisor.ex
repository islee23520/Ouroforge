defmodule OuroforgeExecutor.WorkerSupervisor do
  @moduledoc """
  DynamicSupervisor for local executor control-plane workers.

  Restart intensity is configurable so repeated worker crashes halt safely rather
  than spinning forever. The supervisor manages Elixir/OTP worker lifecycle only;
  it never owns Rust kernel artifacts or trusted write authority.
  """

  use DynamicSupervisor

  alias OuroforgeExecutor.Worker

  def start_link(opts \\ []) do
    name = Keyword.get(opts, :name, __MODULE__)
    DynamicSupervisor.start_link(__MODULE__, opts, name: name)
  end

  def start_worker(supervisor \\ __MODULE__, opts) when is_list(opts) do
    DynamicSupervisor.start_child(supervisor, Worker.child_spec(opts))
  end

  def count_children(supervisor \\ __MODULE__) do
    DynamicSupervisor.count_children(supervisor)
  end

  @impl true
  def init(opts) do
    DynamicSupervisor.init(
      strategy: :one_for_one,
      max_restarts: Keyword.get(opts, :max_restarts, 3),
      max_seconds: Keyword.get(opts, :max_seconds, 5)
    )
  end
end
