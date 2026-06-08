defmodule OuroforgeExecutor.Application do
  @moduledoc """
  OTP application root for the local Studio executor control plane.

  Milestone 63 intentionally starts no workers. Later milestones may attach
  schedulers, supervisors, budget guards, retry loops, backpressure controllers,
  and telemetry processes here. Canonical data-plane state remains owned by the
  Rust `ouroforge` CLI and its artifacts.
  """

  use Application

  @impl true
  def start(_type, _args) do
    children = []
    Supervisor.start_link(children, strategy: :one_for_one, name: OuroforgeExecutor.Supervisor)
  end
end
