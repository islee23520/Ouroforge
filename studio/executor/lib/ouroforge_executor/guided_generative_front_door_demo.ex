defmodule OuroforgeExecutor.GuidedGenerativeFrontDoorDemo do
  @moduledoc """
  Scripted Era N / M82 guided generative front-door demo (#2080).

  The demo proves two paths:

    * autonomous default: no human uses the Studio surface, so the local CLI loop
      remains sufficient and does not wait for a person;
    * guided human intake: Studio captures a non-developer brief as
      intervention-as-evidence, renders a deterministic proposal preview, and
      routes the inert submission to the Rust validation gate before any
      review/apply decision.

  Elixir owns only control/presentation state here. It does not write artifacts,
  append ledgers/evidence, apply proposals, certify evaluator results, infer
  fun/taste, or own proposal semantics.
  """

  alias OuroforgeExecutor.GuidedGenerativeFrontDoor

  defstruct version: "m82-guided-generative-front-door-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            autonomous: nil,
            guided: nil,
            rendered: nil,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            auto_apply_performed?: false,
            notes: []

  def run(opts) when is_list(opts) do
    runner = Keyword.fetch!(opts, :runner)

    autonomous = GuidedGenerativeFrontDoor.autonomous_default_demo()
    {:ok, capture} = GuidedGenerativeFrontDoor.capture(guided_attrs())
    {:ok, routed} = GuidedGenerativeFrontDoor.route_to_rust(capture, runner: runner)

    demo = %__MODULE__{
      autonomous: autonomous,
      guided: %{capture: capture, routed: routed},
      gated_write_verified?: gated_write_verified?(capture, routed),
      autonomous_fallback_verified?: autonomous_fallback_verified?(autonomous),
      notes: [
        "non-developer brief is captured as intervention-as-evidence",
        "proposal preview is deterministic and proposal-only",
        "human write-affecting intent is routed through the Rust generative-front-door gate",
        "review/apply remains required before trusted writes",
        "autonomous fallback completes without the human surface"
      ]
    }

    %__MODULE__{demo | rendered: render(demo)}
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.trusted_write_authority? == false and demo.auto_apply_performed? == false and
      demo.gated_write_verified? and demo.guided.routed.review_apply_required? and
      demo.guided.routed.status == :verified_proposal
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.human_surface_required? == false and
      demo.autonomous.waited_for_human? == false
  end

  def render(%__MODULE__{} = demo) do
    capture = demo.guided.capture
    routed = demo.guided.routed
    preview = capture.preview

    [
      "M82 guided generative front-door demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}; auto-apply: #{demo.auto_apply_performed?}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Gated write verified: #{demo.gated_write_verified?}",
      "Template: #{capture.templateId}; preview status: #{preview["status"]}; proposal-only: #{preview["proposalOnly"]}",
      "Rust route: #{Enum.join(routed.route_cli, " ")}; review/apply required: #{routed.review_apply_required?}",
      "Evidence ref: #{routed.rust_result["evidenceRef"]}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  defp autonomous_fallback_verified?(autonomous) do
    Map.get(autonomous, :status) == :completed_without_human and
      Map.get(autonomous, :waited_for_human?) == false and
      Map.get(autonomous, :human_surface_required?) == false and
      Map.get(autonomous, :cli_fallback_supported?) == true and
      Map.get(autonomous, :trusted_write_performed?) == false
  end

  defp gated_write_verified?(%GuidedGenerativeFrontDoor{} = capture, routed) do
    capture.interventionAsEvidence and capture.readGatedWrite and capture.proposalOnly and
      capture.deterministicPreview and capture.directArtifactWrite == false and
      capture.studioTrustedWriteAuthority == false and capture.autoApplyPerformed == false and
      routed.status == :verified_proposal and routed.trusted_write_performed? == false and
      routed.auto_apply_performed? == false and routed.studio_trusted_write_authority? == false and
      routed.review_apply_required? == true and routed.rust_result["verifiedProposal"] == true and
      present?(routed.rust_result["evidenceRef"])
  end

  defp present?(value), do: is_binary(value) and value != ""

  defp guided_attrs do
    %{
      session_id: "m82-demo-guided-session",
      brief:
        "Create a tiny grid puzzle where the player collects a key before reaching the exit.",
      conversation_summary:
        "The non-developer wants a short, deterministic puzzle candidate with one lock, one key, and no combat.",
      template_id: "grid-puzzle",
      human_actor: "local-non-developer",
      base_intent_ref: "runs/m82/demo/guided-front-door.base-intent.json"
    }
  end
end
