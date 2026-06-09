defmodule OuroforgeExecutor.ReDerivationUXDemoTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.ReDerivationUXDemo

  test "scripted demo emits honest fidelity summary and no auto-port claim" do
    demo = ReDerivationUXDemo.run()

    assert :ok = ReDerivationUXDemo.validate(demo)
    assert demo.fidelitySummary == %{green: 1, yellow: 1, red: 1, escalated: 2}
    assert demo.claimedPortedUnits == []
    assert demo.noAutoPortClaim
    assert demo.oracleGated
    assert demo.deterministicStateHashPrimary
    assert demo.perceptualRenderSecondaryOnly
    assert demo.cleanRoom
    refute demo.trustedWriteAuthority
    refute demo.directArtifactWrite
    assert Enum.map(demo.escalations, & &1.unitId) == ["unit.jump-feel", "unit.shader-vfx"]
  end

  test "summary records runnable script and gated human escalation commands" do
    summary = ReDerivationUXDemo.run() |> ReDerivationUXDemo.to_summary()

    assert summary["version"] == "rederivation-ux-demo-v1"

    assert summary["fixtureRef"] ==
             "examples/rederivation-ux-demo-v1/fidelity-report.fixture.json"

    assert summary["fidelitySummary"] == %{
             "green" => 1,
             "yellow" => 1,
             "red" => 1,
             "escalated" => 2
           }

    assert summary["claimedPortedUnits"] == []
    assert summary["noAutoPortClaim"]
    assert summary["oracleGated"]
    assert summary["deterministicStateHashPrimary"]
    assert summary["perceptualRenderSecondaryOnly"]
    assert summary["trustedWriteAuthority"] == false
    assert [run, migration, preview | routed] = summary["scriptedCommands"]
    assert run == ["run", "seeds/migration-demo.yaml", "--workers", "2"]
    assert Enum.take(migration, 2) == ["migration", "verify-demo"]
    assert Enum.take(preview, 3) == ["behavior", "draft", "preview"]
    assert Enum.all?(routed, &(Enum.take(&1, 3) == ["behavior", "draft", "preview"]))
  end

  test "rendered demo states determinism and no Studio trusted writes" do
    text = ReDerivationUXDemo.run() |> ReDerivationUXDemo.render()

    assert text =~ "Re-derivation UX demo"
    assert text =~ "Fidelity: 🟢 1 / 🟡 1 / 🔴 1"
    assert text =~ "Claimed ported units: 0"
    assert text =~ "No auto-port claim: true"
    assert text =~ "Oracle gated: true"
    assert text =~ "State-hash primary: true"
    assert text =~ "Trusted writes by Studio: false"
  end

  test "demo validation fails closed on port claims missing oracle or trusted writes" do
    demo = ReDerivationUXDemo.run()

    assert {:error, :ported_claim_forbidden} =
             %{demo | claimedPortedUnits: ["unit.door-open"]} |> ReDerivationUXDemo.validate()

    assert {:error, :oracle_gate_missing} =
             %{demo | oracleGated: false} |> ReDerivationUXDemo.validate()

    assert {:error, :trusted_write_forbidden} =
             %{demo | trustedWriteAuthority: true} |> ReDerivationUXDemo.validate()
  end
end
