defmodule OuroforgeExecutor.Application do
  @moduledoc """
  OTP application root for the local Studio executor control plane.

  The application starts the local worker DynamicSupervisor for executor
  control-plane processes only. Canonical data-plane state remains owned by the
  Rust `ouroforge` CLI and its artifacts.
  """

  use Application

  @impl true
  def start(_type, _args) do
    children = [OuroforgeExecutor.WorkerSupervisor]
    Supervisor.start_link(children, strategy: :one_for_one, name: OuroforgeExecutor.Supervisor)
  end
end
