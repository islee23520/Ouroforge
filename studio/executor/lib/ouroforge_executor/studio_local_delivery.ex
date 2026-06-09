defmodule OuroforgeExecutor.StudioLocalDelivery do
  @moduledoc """
  Local Studio + Rust kernel packaging/install contract for M86 (#2096).

  This is a control/presentation-plane read model for a local developer package.
  It records the commands and smoke checks needed to install and run the local
  Phoenix Studio beside the Rust kernel. It does not execute commands, write
  trusted artifacts, validate artifact semantics, publish, sign, deploy, host,
  or create a new data store. Rust remains the data plane and every
  write-affecting action remains read + gated-write through existing gates.
  """

  @schema "ouroforge.studio-local-delivery.v1"
  @boundary "local-first single-user Studio + Rust kernel packaging; read + gated-write; intervention-as-evidence; two-plane Rust data plane owns artifact truth validation determinism evidence/provenance review/apply scene/source-apply evaluator gates; Elixir/Phoenix control + presentation renders captures routes only; no trusted writes; no raw bypass; no command bridge; no new store; CLI fallback completes without Studio or human input; hosted multi-user collaborative Studio Layer-3 DEFER; no signing installer updater app-store deploy publish cloud sync release channel; fun/taste and release go/no-go remain human; #1 and #23 remain open"

  @install_commands [
    "cargo build --workspace --jobs 2",
    "STU=$(git ls-files '**/mix.exs' | head -1)",
    "[ -n \"$STU\" ] && (cd \"$(dirname \"$STU\")\" && mix deps.get && mix compile)"
  ]

  @run_commands [
    "cargo run -p ouroforge-cli -- loop status",
    "cd studio/executor && mix run --no-halt"
  ]

  @smoke_checks [
    %{
      id: "rust-kernel-binary",
      path: "${CARGO_TARGET_DIR:-target}/debug/ouroforge",
      owner: :rust_data_plane
    },
    %{
      id: "studio-beam-app",
      path: "studio/executor/_build/dev/lib/ouroforge_executor/ebin/ouroforge_executor.app",
      owner: :elixir_control_plane
    }
  ]

  defstruct schemaVersion: @schema,
            packageId: "m86-local-studio-kernel-package-v1",
            studio: "studio/executor",
            rustKernel: "crates/ouroforge-cli",
            installCommands: @install_commands,
            runCommands: @run_commands,
            smokeChecks: @smoke_checks,
            boundary: @boundary,
            localFirst: true,
            singleUser: true,
            cliFallbackSupported: true,
            autonomousLoopRequiresHuman: false,
            readGatedWrite: true,
            interventionAsEvidence: true,
            rustDataPlaneOwnsTruth: true,
            elixirOwnsArtifactSemantics: false,
            trustedWriteAuthority: false,
            directArtifactWrite: false,
            rawBypassRequested: false,
            commandBridge: false,
            newDataStore: false,
            hostedCollaborative: false,
            signingOrRelease: false,
            deployOrPublish: false,
            generatedSmokeOnly: true

  def schema, do: @schema
  def boundary, do: @boundary
  def install_commands, do: @install_commands
  def run_commands, do: @run_commands
  def smoke_checks, do: @smoke_checks

  def manifest(attrs \\ %{}) when is_map(attrs) do
    delivery = %__MODULE__{
      localFirst: bool_value(attrs, :local_first, true),
      singleUser: bool_value(attrs, :single_user, true),
      cliFallbackSupported: bool_value(attrs, :cli_fallback_supported, true),
      autonomousLoopRequiresHuman: bool_value(attrs, :autonomous_loop_requires_human, false),
      readGatedWrite: bool_value(attrs, :read_gated_write, true),
      interventionAsEvidence: bool_value(attrs, :intervention_as_evidence, true),
      rustDataPlaneOwnsTruth: bool_value(attrs, :rust_data_plane_owns_truth, true),
      elixirOwnsArtifactSemantics: bool_value(attrs, :elixir_owns_artifact_semantics, false),
      trustedWriteAuthority: bool_value(attrs, :trusted_write_authority, false),
      directArtifactWrite: bool_value(attrs, :direct_artifact_write, false),
      rawBypassRequested: bool_value(attrs, :raw_bypass_requested, false),
      commandBridge: bool_value(attrs, :command_bridge, false),
      newDataStore: bool_value(attrs, :new_data_store, false),
      hostedCollaborative: bool_value(attrs, :hosted_collaborative, false),
      signingOrRelease: bool_value(attrs, :signing_or_release, false),
      deployOrPublish: bool_value(attrs, :deploy_or_publish, false),
      generatedSmokeOnly: bool_value(attrs, :generated_smoke_only, true)
    }

    with :ok <- validate(delivery) do
      {:ok, delivery}
    end
  end

  def validate(%__MODULE__{} = delivery) do
    cond do
      delivery.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      contains_raw_bypass?(delivery.boundary) or delivery.rawBypassRequested ->
        {:error, :raw_bypass_forbidden}

      not delivery.localFirst or not delivery.singleUser or delivery.hostedCollaborative ->
        {:error, :local_single_user_boundary_broken}

      not delivery.cliFallbackSupported or delivery.autonomousLoopRequiresHuman ->
        {:error, :autonomy_or_cli_fallback_broken}

      not delivery.readGatedWrite or not delivery.interventionAsEvidence ->
        {:error, :not_read_gated_write}

      not delivery.rustDataPlaneOwnsTruth or delivery.elixirOwnsArtifactSemantics ->
        {:error, :two_plane_boundary_broken}

      delivery.trustedWriteAuthority or delivery.directArtifactWrite or delivery.commandBridge or
          delivery.newDataStore ->
        {:error, :trusted_write_or_store_forbidden}

      delivery.signingOrRelease or delivery.deployOrPublish ->
        {:error, :release_or_delivery_scope_forbidden}

      not delivery.generatedSmokeOnly ->
        {:error, :trusted_artifact_write_forbidden}

      not boundary_complete?(delivery.boundary) ->
        {:error, :boundary_incomplete}

      true ->
        :ok
    end
  end

  def render(%__MODULE__{} = delivery) do
    %{
      schemaVersion: delivery.schemaVersion,
      packageId: delivery.packageId,
      studio: delivery.studio,
      rustKernel: delivery.rustKernel,
      installCommands: delivery.installCommands,
      runCommands: delivery.runCommands,
      smokeChecks: delivery.smokeChecks,
      boundary: delivery.boundary,
      localFirst: delivery.localFirst,
      singleUser: delivery.singleUser,
      cliFallbackSupported: delivery.cliFallbackSupported,
      readGatedWrite: delivery.readGatedWrite,
      interventionAsEvidence: delivery.interventionAsEvidence,
      rustDataPlaneOwnsTruth: delivery.rustDataPlaneOwnsTruth,
      trustedWriteAuthority: delivery.trustedWriteAuthority,
      directArtifactWrite: delivery.directArtifactWrite,
      commandBridge: delivery.commandBridge,
      hostedCollaborative: delivery.hostedCollaborative,
      signingOrRelease: delivery.signingOrRelease,
      deployOrPublish: delivery.deployOrPublish,
      generatedSmokeOnly: delivery.generatedSmokeOnly
    }
  end

  def cli_fallback_commands do
    [
      "cargo build --workspace --jobs 2",
      "cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2"
    ]
  end

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "local-first",
        "single-user",
        "read + gated-write",
        "intervention-as-evidence",
        "two-plane",
        "Rust data plane",
        "Elixir/Phoenix control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no trusted writes",
        "no raw bypass",
        "no command bridge",
        "no new store",
        "CLI fallback completes without Studio or human input",
        "hosted multi-user collaborative Studio Layer-3 DEFER",
        "no signing installer updater app-store deploy publish cloud sync release channel",
        "fun/taste and release go/no-go remain human",
        "#1 and #23 remain open"
      ],
      &String.contains?(boundary, &1)
    )
  end

  defp boundary_complete?(_), do: false

  defp contains_raw_bypass?(value) when is_binary(value) do
    String.contains?(String.downcase(value), ["raw_write_bypass", "raw_apply_bypass"])
  end

  defp contains_raw_bypass?(_), do: false

  defp bool_value(attrs, key, default) do
    Map.get(attrs, key, Map.get(attrs, Atom.to_string(key), default))
  end
end
