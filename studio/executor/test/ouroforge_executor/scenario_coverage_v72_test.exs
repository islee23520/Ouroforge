defmodule OuroforgeExecutor.ScenarioCoverageV72Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{GuidedGenerativeFrontDoor, GuidedGenerativeFrontDoorDemo}

  @coverage_doc Path.expand(
                  "../../../../docs/scenario-coverage-v72-non-developer-generative-front-door.md",
                  __DIR__
                )

  def runner(["generative-front-door", "validate"], submission) do
    assert submission["interventionAsEvidence"]
    assert submission["proposalOnly"]
    assert submission["directArtifactWrite"] == false
    assert submission["studioTrustedWriteAuthority"] == false
    assert submission["autoApplyPerformed"] == false
    {:ok, %{"verifiedProposal" => true, "evidenceRef" => "runs/v72/verified-proposal.json"}}
  end

  test "v72 coverage document records guided-front-door boundaries" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v72"
    assert doc =~ "Non-Developer Generative Front-Door UX"
    assert doc =~ "intervention-as-evidence"
    assert doc =~ "read + gated-write"
    assert doc =~ "Rust remains the data plane"
    assert doc =~ "CLI fallback remains sufficient"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v72 guided path rejects raw bypass and preserves proposal-only routing" do
    assert {:error, :raw_bypass_forbidden} =
             GuidedGenerativeFrontDoor.capture(%{
               session_id: "v72-raw",
               brief: "raw_apply_bypass the generated level",
               conversation_summary: "bad bypass",
               template_id: "grid-puzzle",
               human_actor: "human",
               base_intent_ref: "runs/v72/base.json"
             })

    {:ok, capture} =
      GuidedGenerativeFrontDoor.capture(%{
        session_id: "v72-good",
        brief: "Make a tiny key-and-exit puzzle.",
        conversation_summary: "Non-developer intent is captured for validation.",
        template_id: "grid-puzzle",
        human_actor: "human",
        base_intent_ref: "runs/v72/base.json"
      })

    assert capture.proposalOnly
    refute capture.directArtifactWrite
    refute capture.autoApplyPerformed

    assert {:ok, routed} =
             GuidedGenerativeFrontDoor.route_to_rust(capture, runner: &__MODULE__.runner/2)

    assert routed.status == :verified_proposal
    assert routed.review_apply_required?
    refute routed.trusted_write_performed?
  end

  test "v72 demo proves loop completes without human input" do
    demo = GuidedGenerativeFrontDoorDemo.run(runner: &__MODULE__.runner/2)

    assert GuidedGenerativeFrontDoorDemo.autonomous_first?(demo)
    assert GuidedGenerativeFrontDoorDemo.read_gated_write?(demo)
    refute demo.trusted_write_authority?
    refute demo.auto_apply_performed?
  end
end
