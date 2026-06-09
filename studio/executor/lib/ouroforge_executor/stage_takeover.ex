defmodule OuroforgeExecutor.StageTakeover do
  @moduledoc """
  Local executor session state for M80 stage takeover and handback.

  This GenServer is control-plane state only. It records who temporarily owns a
  stage, captures manual work as evidence/provenance metadata, and requires Rust
  CLI validation/record/reverify phases before handback can resume autonomous
  execution. It never writes artifacts or owns artifact semantics.
  """

  use GenServer

  alias OuroforgeExecutor.CLI

  defmodule ManualWork do
    @moduledoc false
    defstruct [
      :id,
      :summary,
      :actor_id,
      :base_ref,
      :evidence_ref,
      :provenance_ref,
      :validation_evidence_ref,
      :record_evidence_ref,
      :reverify_evidence_ref,
      status: :pending
    ]
  end

  defmodule StageSession do
    @moduledoc false
    defstruct [
      :stage_id,
      :campaign_id,
      :actor_id,
      :reason,
      :takeover_evidence_ref,
      :handback_evidence_ref,
      status: :autonomous,
      locked?: false,
      manual_work: [],
      rejected: [],
      resumed?: false,
      trusted_write_authority?: false,
      human_required_for_autonomous_loop?: false,
      boundary:
        "intervention-as-evidence; read + gated-write; two-plane; local-first; Rust data plane validates and records; Elixir/Phoenix control + presentation only; no raw bypass"
    ]
  end

  @type server :: GenServer.server()

  def start_link(opts \\ []) do
    name = Keyword.get(opts, :name, __MODULE__)
    GenServer.start_link(__MODULE__, %{}, name: name)
  end

  @impl true
  def init(state), do: {:ok, state}

  def take_over(server \\ __MODULE__, attrs, opts \\ []) do
    GenServer.call(server, {:take_over, attrs, runner_opts(opts)})
  end

  def capture_manual_work(server \\ __MODULE__, stage_id, attrs) do
    GenServer.call(server, {:capture_manual_work, stage_id, attrs})
  end

  def handback(server \\ __MODULE__, stage_id, opts \\ []) do
    GenServer.call(server, {:handback, stage_id, runner_opts(opts)})
  end

  def get(server \\ __MODULE__, stage_id) do
    GenServer.call(server, {:get, stage_id})
  end

  @impl true
  def handle_call({:take_over, attrs, opts}, _from, state) do
    session = new_session(attrs)

    reply =
      with :ok <- validate_takeover(session),
           {:ok, validation_ref} <- run_gate(session, :takeover_validate, opts),
           {:ok, record_ref} <- run_gate(session, :takeover_record, opts) do
        accepted = %StageSession{
          session
          | status: :taken_over,
            locked?: true,
            takeover_evidence_ref: join_refs(validation_ref, record_ref)
        }

        {{:ok, accepted}, Map.put(state, accepted.stage_id, accepted)}
      else
        {:error, reason} -> {{:error, reason}, state}
      end

    {response, next_state} = reply
    {:reply, response, next_state}
  end

  def handle_call({:capture_manual_work, stage_id, attrs}, _from, state) do
    reply =
      with {:ok, %StageSession{} = session} <- fetch_session(state, stage_id),
           :ok <- require_locked(session),
           %ManualWork{} = work <- new_manual_work(attrs),
           :ok <- validate_manual_work(work) do
        updated = %StageSession{session | manual_work: session.manual_work ++ [work]}
        {{:ok, updated}, Map.put(state, stage_id, updated)}
      else
        {:error, reason} -> {{:error, reason}, state}
      end

    {response, next_state} = reply
    {:reply, response, next_state}
  end

  def handle_call({:handback, stage_id, opts}, _from, state) do
    reply =
      with {:ok, %StageSession{} = session} <- fetch_session(state, stage_id),
           :ok <- require_locked(session),
           {:ok, verified_work} <- verify_manual_work(session, opts),
           {:ok, validation_ref} <- run_gate(session, :handback_validate, opts),
           {:ok, record_ref} <- run_gate(session, :handback_record, opts) do
        handed_back = %StageSession{
          session
          | status: :handed_back,
            locked?: false,
            manual_work: verified_work,
            handback_evidence_ref: join_refs(validation_ref, record_ref),
            resumed?: true
        }

        {{:ok, handed_back}, Map.put(state, stage_id, handed_back)}
      else
        {:error, reason} -> {{:error, reason}, state}
      end

    {response, next_state} = reply
    {:reply, response, next_state}
  end

  def handle_call({:get, stage_id}, _from, state) do
    {:reply, Map.fetch(state, stage_id), state}
  end

  defp new_session(attrs) do
    attrs = normalize_attrs(attrs)

    %StageSession{
      stage_id: value(attrs, :stage_id),
      campaign_id: value(attrs, :campaign_id),
      actor_id: value(attrs, :actor_id),
      reason: value(attrs, :reason)
    }
  end

  defp new_manual_work(attrs) do
    attrs = normalize_attrs(attrs)

    %ManualWork{
      id: value(attrs, :id),
      summary: value(attrs, :summary),
      actor_id: value(attrs, :actor_id),
      base_ref: value(attrs, :base_ref),
      evidence_ref: value(attrs, :evidence_ref),
      provenance_ref: value(attrs, :provenance_ref)
    }
  end

  defp validate_takeover(%StageSession{} = session) do
    with :ok <- require_text(:stage_id, session.stage_id),
         :ok <- require_text(:campaign_id, session.campaign_id),
         :ok <- require_text(:actor_id, session.actor_id),
         :ok <- require_text(:reason, session.reason),
         :ok <- reject_raw_bypass(session.reason) do
      :ok
    end
  end

  defp validate_manual_work(%ManualWork{} = work) do
    with :ok <- require_text(:id, work.id),
         :ok <- require_text(:summary, work.summary),
         :ok <- require_text(:actor_id, work.actor_id),
         :ok <- require_ref(:base_ref, work.base_ref),
         :ok <- require_ref(:evidence_ref, work.evidence_ref),
         :ok <- require_ref(:provenance_ref, work.provenance_ref),
         :ok <- reject_raw_bypass(work.summary) do
      :ok
    end
  end

  defp verify_manual_work(%StageSession{manual_work: []}, _opts),
    do: {:error, :manual_work_required_for_handback}

  defp verify_manual_work(%StageSession{} = session, opts) do
    session.manual_work
    |> Enum.reduce_while({:ok, []}, fn %ManualWork{} = work, {:ok, verified} ->
      with {:ok, validation_ref} <- run_gate(session, {:manual_work_validate, work}, opts),
           {:ok, record_ref} <- run_gate(session, {:manual_work_record, work}, opts),
           {:ok, reverify_ref} <- run_gate(session, {:manual_work_reverify, work}, opts) do
        accepted = %ManualWork{
          work
          | status: :accepted,
            validation_evidence_ref: validation_ref,
            record_evidence_ref: record_ref,
            reverify_evidence_ref: reverify_ref
        }

        {:cont, {:ok, verified ++ [accepted]}}
      else
        {:error, reason} -> {:halt, {:error, {:manual_work_not_verified, work.id, reason}}}
      end
    end)
  end

  defp run_gate(%StageSession{} = session, phase, opts) do
    phase_name = phase_name(phase)

    argv =
      [
        "loop",
        "step",
        session.campaign_id,
        "--stage-phase",
        phase_name,
        "--stage-id",
        session.stage_id,
        "--actor-id",
        session.actor_id,
        "--reason",
        session.reason
      ] ++ work_args(phase)

    case CLI.run(argv, opts) do
      {:ok, result} -> {:ok, evidence_ref(result, phase_name)}
      {:error, reason} -> {:error, reason}
    end
  end

  defp phase_name({phase, _work}), do: Atom.to_string(phase)
  defp phase_name(phase), do: Atom.to_string(phase)

  defp work_args({_phase, %ManualWork{} = work}) do
    [
      "--manual-work-id",
      work.id,
      "--manual-work-evidence-ref",
      work.evidence_ref,
      "--manual-work-provenance-ref",
      work.provenance_ref,
      "--base-ref",
      work.base_ref
    ]
  end

  defp work_args(_phase), do: []

  defp evidence_ref(%CLI.Result{stdout: stdout}, fallback) do
    stdout = String.trim(to_string(stdout))

    cond do
      stdout == "" ->
        fallback

      String.contains?(stdout, "evidenceRef=") ->
        stdout |> String.split("evidenceRef=") |> List.last()

      true ->
        stdout
    end
  end

  defp join_refs(left, right), do: Enum.join([left, right], ",")

  defp fetch_session(state, stage_id) do
    case Map.fetch(state, stage_id) do
      {:ok, session} -> {:ok, session}
      :error -> {:error, :unknown_stage}
    end
  end

  defp require_locked(%StageSession{locked?: true}), do: :ok
  defp require_locked(_), do: {:error, :stage_not_taken_over}

  defp require_text(_field, value) when is_binary(value) and value != "", do: :ok
  defp require_text(field, _value), do: {:error, {:missing_text, field}}

  defp require_ref(field, value) when is_binary(value) and value != "" do
    if String.contains?(value, ".."), do: {:error, {:invalid_ref, field}}, else: :ok
  end

  defp require_ref(field, _value), do: {:error, {:missing_ref, field}}

  defp reject_raw_bypass(value) when is_binary(value) do
    if String.contains?(String.downcase(value), ["raw_write_bypass", "raw_apply_bypass"]) do
      {:error, :raw_bypass_forbidden}
    else
      :ok
    end
  end

  defp reject_raw_bypass(_), do: :ok

  defp normalize_attrs(attrs) when is_list(attrs), do: Map.new(attrs)
  defp normalize_attrs(attrs) when is_map(attrs), do: attrs

  defp value(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key)) || Map.get(attrs, camelize(key))
  end

  defp camelize(key) do
    key
    |> Atom.to_string()
    |> String.split("_")
    |> then(fn [head | tail] -> head <> Enum.map_join(tail, "", &String.capitalize/1) end)
  end

  defp runner_opts(opts) do
    Keyword.take(opts, [:runner, :executable, :cd, :env, :stderr_to_stdout])
  end
end
