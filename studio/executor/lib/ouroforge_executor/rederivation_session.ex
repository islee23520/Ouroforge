defmodule OuroforgeExecutor.ReDerivationSession do
  @moduledoc """
  OTP session state for the M113 re-derivation Studio surface.

  The GenServer stores ephemeral presentation/control-plane state and broadcasts
  updates for local LiveView subscribers. It never writes artifacts; Rust-owned
  evidence and gated `ouroforge` CLI/review paths remain authoritative.
  """

  use GenServer

  alias OuroforgeExecutor.{LocalPubSub, ReDerivationUX}

  defstruct [:surface, escalations: [], status: :idle, events: []]

  def start_link(opts) do
    name = Keyword.get(opts, :name, __MODULE__)
    GenServer.start_link(__MODULE__, opts, name: name)
  end

  def attach_evidence(pid, evidence), do: GenServer.call(pid, {:attach_evidence, evidence})

  def submit_intent_feel(pid, unit_id, note, opts \\ []),
    do: GenServer.call(pid, {:submit_intent_feel, unit_id, note, opts})

  def state(pid), do: GenServer.call(pid, :state)

  @impl true
  def init(opts) do
    {:ok,
     %__MODULE__{events: [{:started, Keyword.get(opts, :session_id, "rederivation-session")}]}}
  end

  @impl true
  def handle_call({:attach_evidence, evidence}, _from, state) do
    with {:ok, surface} <- ReDerivationUX.surface(evidence),
         {:ok, escalations} <- ReDerivationUX.escalation_queue(surface) do
      next = %{
        state
        | surface: surface,
          escalations: escalations,
          status: :evidence_ready,
          events: [{:evidence_ready, surface.projectId} | state.events]
      }

      broadcast(next, :evidence_ready)
      {:reply, {:ok, surface, escalations}, next}
    else
      error -> {:reply, error, state}
    end
  end

  def handle_call({:submit_intent_feel, _unit_id, _note, _opts}, _from, %{surface: nil} = state),
    do: {:reply, {:error, :not_configured}, state}

  def handle_call({:submit_intent_feel, unit_id, note, opts}, _from, state) do
    with %ReDerivationUX.Escalation{} = escalation <-
           Enum.find(state.escalations, &(&1.unitId == unit_id)),
         {:ok, result} <- ReDerivationUX.submit_intent_feel(escalation, note, opts) do
      next = %{
        state
        | status: :intent_feel_routed,
          events: [{:intent_feel_routed, unit_id} | state.events]
      }

      broadcast(next, :intent_feel_routed)
      {:reply, {:ok, result}, next}
    else
      nil -> {:reply, {:error, :no_escalation_for_unit}, state}
      error -> {:reply, error, state}
    end
  end

  def handle_call(:state, _from, state), do: {:reply, state, state}

  defp broadcast(state, event) do
    LocalPubSub.broadcast(ReDerivationUX.pubsub_topic(), %{event: event, status: state.status})
  end
end
