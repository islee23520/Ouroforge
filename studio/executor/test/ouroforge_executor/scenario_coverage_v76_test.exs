defmodule OuroforgeExecutor.ScenarioCoverageV76Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{StudioLocalDelivery, StudioLocalDeliveryDemo}

  @coverage_doc Path.expand(
                  "../../../../docs/scenario-coverage-v76-studio-local-delivery.md",
                  __DIR__
                )

  test "v76 coverage document records local delivery boundaries" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v76"
    assert doc =~ "local-first and single-user only"
    assert doc =~ "read + gated-write"
    assert doc =~ "intervention-as-evidence"
    assert doc =~ "Rust remains the data plane"
    assert doc =~ "Elixir/OTP + Phoenix LiveView is control + presentation only"
    assert doc =~ "generated evidence only"
    assert doc =~ "raw-bypass"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v76 local delivery manifest exposes install run UX without write authority" do
    assert {:ok, delivery} = StudioLocalDelivery.manifest()
    rendered = StudioLocalDelivery.render(delivery)

    assert rendered.localFirst
    assert rendered.singleUser
    assert rendered.cliFallbackSupported
    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneOwnsTruth
    assert rendered.generatedSmokeOnly
    assert Enum.any?(rendered.installCommands, &String.contains?(&1, "cargo build"))
    assert Enum.any?(rendered.installCommands, &String.contains?(&1, "mix deps.get"))
    assert Enum.any?(rendered.runCommands, &String.contains?(&1, "ouroforge-cli"))
    assert Enum.any?(rendered.runCommands, &String.contains?(&1, "mix run --no-halt"))
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge
    refute rendered.hostedCollaborative
    refute rendered.signingOrRelease
    refute rendered.deployOrPublish
  end

  test "v76 drift regressions fail closed" do
    assert {:error, :raw_bypass_forbidden} =
             StudioLocalDelivery.manifest(%{raw_bypass_requested: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLocalDelivery.manifest(%{autonomous_loop_requires_human: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLocalDelivery.manifest(%{cli_fallback_supported: false})

    assert {:error, :two_plane_boundary_broken} =
             StudioLocalDelivery.manifest(%{elixir_owns_artifact_semantics: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{trusted_write_authority: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{direct_artifact_write: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{command_bridge: true})

    assert {:error, :trusted_write_or_store_forbidden} =
             StudioLocalDelivery.manifest(%{new_data_store: true})

    assert {:error, :local_single_user_boundary_broken} =
             StudioLocalDelivery.manifest(%{hosted_collaborative: true})

    assert {:error, :release_or_delivery_scope_forbidden} =
             StudioLocalDelivery.manifest(%{signing_or_release: true})

    assert {:error, :release_or_delivery_scope_forbidden} =
             StudioLocalDelivery.manifest(%{deploy_or_publish: true})
  end

  test "v76 demo keeps human package write gated and loop non-blocking" do
    demo = StudioLocalDeliveryDemo.run()

    assert StudioLocalDeliveryDemo.read_gated_write?(demo)
    assert StudioLocalDeliveryDemo.autonomous_first?(demo)
    assert StudioLocalDeliveryDemo.smoke_verified?(demo)
    assert demo.install_ux_visible?
    assert demo.intervention.status == :queued_for_rust_gate
    assert demo.intervention.kind == :constraint
    assert demo.intervention.gateFamily =~ "human constraint"
    assert demo.intervention.routeCli == ["evaluate"]
    assert demo.autonomous.status == :completed_without_human
    refute demo.autonomous.human_surface_required?
    refute demo.autonomous.waited_for_human?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.command_bridge?
    refute demo.hosted_collaborative?

    rendered = StudioLocalDeliveryDemo.render(demo)
    assert rendered =~ "M86 Studio local delivery demo"
    assert rendered =~ "Gated intervention verified: true"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "runs/studio-local-package-smoke-v1/smoke.json"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
  end
end
