defmodule OuroforgeExecutor.ScenarioCoverageV67Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.HumanArtifactIntakeDemo
  alias OuroforgeExecutor.HumanArtifactIntakeSurface

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        intake_id: "scenario-v67-human-intake-001",
        artifact_id: "scenario-v67-card-human-001",
        artifact_kind: "card",
        target_ref: "projects/demo/cards/card-spark.json",
        target_base_ref: "hash:v67-card-base-before",
        author: "human:local-author",
        author_provenance_ref: "runs/v67/provenance/human-author.json",
        original_payload: ~s({"name":"Spark","cost":1,"text":"Deal 1 damage."})
      },
      overrides
    )
  end

  test "v67 Studio captures read + gated-write and routes only to Rust validation" do
    assert {:ok, capture} = HumanArtifactIntakeSurface.capture(attrs())
    assert capture.humanProvenance
    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.routeCli == ["human-artifact-intake", "validate"]
    refute capture.directArtifactWrite
    refute capture.rawBypassRequested
    refute capture.studioTrustedWriteAuthority
    refute capture.humanRequiredForAutonomousLoop
    assert capture.cliFallbackSupported
    assert capture.boundary =~ "Rust = data plane"
    assert capture.boundary =~ "Elixir/OTP + Phoenix LiveView = control + presentation"
    assert capture.boundary =~ "#1 and #23 remain open"

    assert {:ok, submission} = HumanArtifactIntakeSurface.to_rust_submission(capture)
    assert submission["author"] == "human:local-author"
    assert submission["routeCli"] == ["human-artifact-intake", "validate"]
    assert submission["directArtifactWrite"] == false
    assert submission["studioTrustedWriteAuthority"] == false
  end

  test "v67 Studio rejects raw bypass, trusted writes, and mandatory humans" do
    assert {:error, :raw_bypass_forbidden} =
             HumanArtifactIntakeSurface.capture(attrs(%{original_payload: "raw_apply_bypass"}))

    assert {:ok, capture} = HumanArtifactIntakeSurface.capture(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{capture | directArtifactWrite: true} |> HumanArtifactIntakeSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true}
             |> HumanArtifactIntakeSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | humanRequiredForAutonomousLoop: true}
             |> HumanArtifactIntakeSurface.validate()
  end

  test "v67 autonomous default completes without human artifact input" do
    demo = HumanArtifactIntakeDemo.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert Enum.any?(demo.evidence_refs, &String.contains?(&1, "no-human"))
    assert demo.boundary =~ "loop completes without human"
  end

  test "v67 failed Rust gate blocks human-authored artifact without raw apply" do
    runner = fn argv, artifact ->
      assert argv == ["human-artifact-intake", "validate"]
      assert artifact["status"] == "blocked"
      assert Enum.any?(artifact["gateResults"], &(&1["status"] == "failed"))
      assert artifact["readGatedWrite"]
      assert artifact["directArtifactWrite"] == false
      assert artifact["rawBypassRequested"] == false
      assert artifact["studioTrustedWriteAuthority"] == false
      {:error, %{"readyForReviewApply" => false, "blockedReasons" => ["Evaluator:Failed"]}}
    end

    assert {:ok, demo} = HumanArtifactIntakeDemo.blocked_human_intake_demo(runner: runner)
    assert demo.status == :blocked_by_gate
    assert demo.rust_result["readyForReviewApply"] == false
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
  end
end
