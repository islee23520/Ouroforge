defmodule OuroforgeExecutor.StudioLocalDeliveryTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.StudioLocalDelivery

  test "local package manifest installs Studio beside Rust kernel without granting write authority" do
    assert {:ok, delivery} = StudioLocalDelivery.manifest()
    rendered = StudioLocalDelivery.render(delivery)

    assert rendered.schemaVersion == "ouroforge.studio-local-delivery.v1"
    assert rendered.packageId == "m86-local-studio-kernel-package-v1"
    assert rendered.studio == "studio/executor"
    assert rendered.rustKernel == "crates/ouroforge-cli"
    assert "cargo build --workspace --jobs 2" in rendered.installCommands
    assert Enum.any?(rendered.installCommands, &String.contains?(&1, "mix deps.get"))
    assert Enum.any?(rendered.runCommands, &String.contains?(&1, "ouroforge-cli"))
    assert Enum.any?(rendered.smokeChecks, &(&1.id == "rust-kernel-binary"))
    assert Enum.any?(rendered.smokeChecks, &(&1.id == "studio-beam-app"))

    assert rendered.localFirst
    assert rendered.singleUser
    assert rendered.cliFallbackSupported
    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneOwnsTruth
    assert rendered.generatedSmokeOnly
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge
    refute rendered.hostedCollaborative
    refute rendered.signingOrRelease
    refute rendered.deployOrPublish

    assert rendered.boundary =~ "two-plane"
    assert rendered.boundary =~ "hosted multi-user collaborative Studio Layer-3 DEFER"
    assert rendered.boundary =~ "CLI fallback completes without Studio or human input"
  end

  test "scope drift and raw bypass attempts fail closed" do
    assert {:error, :local_single_user_boundary_broken} =
             StudioLocalDelivery.manifest(%{hosted_collaborative: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLocalDelivery.manifest(%{autonomous_loop_requires_human: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLocalDelivery.manifest(%{cli_fallback_supported: false})

    assert {:error, :two_plane_boundary_broken} =
             StudioLocalDelivery.manifest(%{elixir_owns_artifact_semantics: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{direct_artifact_write: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{command_bridge: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{new_data_store: true})

    assert {:error, :release_or_delivery_scope_forbidden} =
             StudioLocalDelivery.manifest(%{signing_or_release: true})

    assert {:error, :release_or_delivery_scope_forbidden} =
             StudioLocalDelivery.manifest(%{deploy_or_publish: true})

    assert {:error, :raw_bypass_forbidden} =
             StudioLocalDelivery.manifest(%{raw_bypass_requested: true})
  end

  test "CLI fallback remains independent from the packaged Studio" do
    assert StudioLocalDelivery.cli_fallback_commands() == [
             "cargo build --workspace --jobs 2",
             "cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2"
           ]
  end
end
