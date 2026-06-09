defmodule OuroforgeExecutor.MigrationImportSession do
  @moduledoc """
  OTP session state for the M94 Migration UX wizard.

  The GenServer stores only ephemeral in-progress Studio interaction state and
  broadcasts presentation updates. It never writes artifacts; Rust CLI/evidence
  remains canonical product truth.
  """

  use GenServer

  alias OuroforgeExecutor.{LocalPubSub, MigrationUX}

  defstruct [:wizard, :report, fixForwardLinks: [], status: :idle, events: []]

  def start_link(opts) do
    name = Keyword.get(opts, :name, __MODULE__)
    GenServer.start_link(__MODULE__, opts, name: name)
  end

  def configure(pid, attrs), do: GenServer.call(pid, {:configure, attrs})
  def import(pid, opts \\ []), do: GenServer.call(pid, {:import, opts})
  def attach_report(pid, report), do: GenServer.call(pid, {:attach_report, report})
  def state(pid), do: GenServer.call(pid, :state)

  @impl true
  def init(opts) do
    {:ok, %__MODULE__{events: [{:started, Keyword.get(opts, :session_id, "migration-session")}]}}
  end

  @impl true
  def handle_call({:configure, attrs}, _from, state) do
    with {:ok, wizard} <- MigrationUX.new_wizard(attrs) do
      next = %{
        state
        | wizard: wizard,
          status: :configured,
          events: [{:configured, wizard.sourceEngine} | state.events]
      }

      broadcast(next, :configured)
      {:reply, {:ok, wizard}, next}
    else
      error -> {:reply, error, state}
    end
  end

  def handle_call({:import, _opts}, _from, %{wizard: nil} = state),
    do: {:reply, {:error, :not_configured}, state}

  def handle_call({:import, opts}, _from, state) do
    with {:ok, result} <- MigrationUX.import(state.wizard, opts) do
      next = %{
        state
        | status: :import_invoked,
          events: [{:import_invoked, result.routeCli} | state.events]
      }

      broadcast(next, :import_invoked)
      {:reply, {:ok, result}, next}
    else
      error -> {:reply, error, state}
    end
  end

  def handle_call({:attach_report, report}, _from, state) do
    with {:ok, view} <- MigrationUX.report_view(report),
         {:ok, links} <- MigrationUX.fix_forward_links(view) do
      next = %{
        state
        | report: view,
          fixForwardLinks: links,
          status: :report_ready,
          events: [{:report_ready, view.reportId} | state.events]
      }

      broadcast(next, :report_ready)
      {:reply, {:ok, view, links}, next}
    else
      error -> {:reply, error, state}
    end
  end

  def handle_call(:state, _from, state), do: {:reply, state, state}

  defp broadcast(state, event) do
    LocalPubSub.broadcast(MigrationUX.pubsub_topic(), %{event: event, status: state.status})
  end
end
