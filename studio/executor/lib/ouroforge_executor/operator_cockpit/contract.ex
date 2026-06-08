defmodule OuroforgeExecutor.OperatorCockpit.Contract do
  @moduledoc """
  M67 read-only operator cockpit boundary contract.

  The cockpit is an Elixir/OTP local control-plane view over executor state,
  telemetry, and Rust-owned `ouroforge` CLI output. It never writes artifacts,
  ledgers, evidence, trust-gradient records, apply/release/merge/deploy state,
  and it never turns ambiguous operator states into automatic success.
  """

  @trusted_write_surfaces [
    :artifact,
    :ledger,
    :evidence,
    :trust_gradient,
    :apply,
    :release,
    :merge,
    :deploy
  ]

  @source_types [
    :executor_state,
    :executor_telemetry,
    :ouroforge_cli_output,
    :executor_evidence_path
  ]

  @human_judgment [:intent, :taste, :legal, :release, :ambiguous_go_no_go]

  @states [
    :normal,
    :waiting,
    :retrying,
    :budget_limited,
    :backpressured,
    :blocked
  ]

  defstruct version: "m67-1",
            mode: :read_only_local_operator_view,
            control_plane: :elixir_otp,
            data_plane: :rust_kernel,
            rust_boundary: :frozen_ouroforge_cli,
            local_only?: true,
            remote_workers?: false,
            hosted_dashboard?: false,
            remote_telemetry?: false,
            trusted_write_authority?: false,
            browser_execution_authority?: false,
            self_certification_authority?: false,
            states: @states,
            source_types: @source_types,
            forbidden_trusted_write_surfaces: @trusted_write_surfaces,
            human_judgment_required_for: @human_judgment,
            golden_parity: %{
              required_when_artifacts_are_involved?: true,
              manual_path: "ouroforge CLI",
              executor_path: "executor driving the same frozen ouroforge CLI",
              comparator: :byte_identical_output
            }

  def read_only_contract, do: %__MODULE__{}

  def forbidden_trusted_write_surfaces, do: @trusted_write_surfaces
  def source_types, do: @source_types
  def human_judgment_required_for, do: @human_judgment
  def supported_states, do: @states

  def read_only?(%__MODULE__{} = contract) do
    contract.local_only? == true and
      contract.remote_workers? == false and
      contract.hosted_dashboard? == false and
      contract.remote_telemetry? == false and
      contract.trusted_write_authority? == false and
      contract.browser_execution_authority? == false and
      contract.self_certification_authority? == false
  end

  def traceable_source?(source_type), do: source_type in @source_types

  def trusted_write_surface?(surface), do: surface in @trusted_write_surfaces

  def ambiguous_status(status) when status in [:blocked, :budget_limited],
    do: :requires_human_judgment

  def ambiguous_status(_), do: :operator_information_only

  def render_summary(%__MODULE__{} = contract \\ read_only_contract()) do
    [
      "M67 operator cockpit boundary: read-only local Studio UX",
      "Elixir/OTP is the local executor control plane; Rust kernel remains data-plane truth.",
      "Rust is touched only through the frozen ouroforge CLI surface.",
      "No direct artifact, ledger, evidence, trust-gradient, apply, release, merge, or deploy writes.",
      "Ambiguous go/no-go, intent, taste, legal, and release decisions require human judgment.",
      "Artifact-affecting demos require byte-identical parity with the manual ouroforge CLI path.",
      "Local single-machine scope only: no hosted dashboard, remote workers, or remote telemetry.",
      "States covered: #{contract.states |> Enum.map(&Atom.to_string/1) |> Enum.join(", ")}."
    ]
    |> Enum.join("\n")
  end
end
