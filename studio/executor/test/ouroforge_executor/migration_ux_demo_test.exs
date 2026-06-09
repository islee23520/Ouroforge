defmodule OuroforgeExecutor.MigrationUXDemoTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.MigrationUXDemo

  test "scripted demo imports skeleton reports and keeps logic oracle-gated" do
    demo = MigrationUXDemo.run()

    assert :ok = MigrationUXDemo.validate(demo)
    assert demo.report.summary == %{green: 1, yellow: 1, red: 1}
    assert demo.claimedPortedUnits == []
    assert demo.noAutoPortClaim
    assert demo.oracleGated
    assert demo.cleanRoom
    refute demo.trustedWriteAuthority
    refute demo.directArtifactWrite
    assert Enum.any?(demo.fixForwardLinks, &(&1.targetEra == "Era R"))
    assert Enum.any?(demo.deterministicHashes, &String.starts_with?(&1, "sha256:"))
  end

  test "demo summary is reviewable and includes runnable commands" do
    summary = MigrationUXDemo.run() |> MigrationUXDemo.to_summary()

    assert summary["version"] == "migration-ux-studio-demo-v1"
    assert summary["fidelitySummary"] == %{"green" => 1, "yellow" => 1, "red" => 1}
    assert summary["claimedPortedUnits"] == []
    assert summary["fixForwardTargets"] == ["Era M", "Era R"]
    assert [godot, unity, seed] = summary["scriptedCommands"]
    assert Enum.take(godot, 2) == ["migration", "verify-demo"]
    assert Enum.take(unity, 2) == ["migration", "unity-demo"]
    assert seed == ["run", "seeds/migration-demo.yaml", "--workers", "2"]
  end

  test "rendered demo text states no auto-port and no trusted Studio writes" do
    text = MigrationUXDemo.run() |> MigrationUXDemo.render()

    assert text =~ "Migration UX Studio demo"
    assert text =~ "Fidelity: 🟢 1 / 🟡 1 / 🔴 1"
    assert text =~ "Claimed ported units: 0"
    assert text =~ "No auto-port claim: true"
    assert text =~ "Trusted writes by Studio: false"
  end

  test "demo validation fails closed on port claims or trusted writes" do
    demo = MigrationUXDemo.run()

    assert {:error, :ported_claim_forbidden} =
             %{demo | claimedPortedUnits: ["demo.player.logic"]} |> MigrationUXDemo.validate()

    assert {:error, :trusted_write_forbidden} =
             %{demo | trustedWriteAuthority: true} |> MigrationUXDemo.validate()
  end
end
