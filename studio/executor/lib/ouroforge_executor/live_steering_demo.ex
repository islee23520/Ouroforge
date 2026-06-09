defmodule OuroforgeExecutor.LiveSteeringDemo do
  @moduledoc """
  Scripted M77 live steering demo.

  The demo composes the local Elixir control plane with the Rust CLI boundary:
  one run has no human directive and continues autonomously; one run captures a
  human steering directive, validates and records it through the constrained CLI,
  then changes only ephemeral executor scheduling state. No artifact, ledger,
  evidence, source, scene, release, merge, or deploy write is performed by
  Elixir.
  """

  alias OuroforgeExecutor.{LiveSteering, ProductionPlan, SteeringDirective}

  defstruct version: "m77-live-steering-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            autonomous: nil,
            steered: nil,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            notes: []

  def run(opts) do
    runner = Keyword.fetch!(opts, :runner)
    pubsub = Keyword.get(opts, :pubsub, OuroforgeExecutor.PubSub)
    plan = Keyword.get_lazy(opts, :plan, &fixture_plan/0)

    autonomous = LiveSteering.consume(plan, [], runner: runner, pubsub: pubsub)

    steered =
      LiveSteering.consume(plan, fixture_directives(plan.plan_id), runner: runner, pubsub: pubsub)

    %__MODULE__{
      autonomous: autonomous,
      steered: steered,
      gated_write_verified?: gated_write_verified?(steered),
      autonomous_fallback_verified?:
        autonomous.ready_task_ids != [] and autonomous.accepted_directives == [],
      notes: [
        "autonomous fallback completes scheduling without human input",
        "human directive is intervention-as-evidence routed through the Rust CLI",
        "Elixir captures, routes, and renders only; Rust remains data-plane truth"
      ]
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "M77 live steering directives demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Gated write verified: #{demo.gated_write_verified?}",
      "Autonomous ready tasks: #{Enum.join(demo.autonomous.ready_task_ids, ", ")}",
      "Steered ready tasks: #{Enum.join(demo.steered.ready_task_ids, ", ")}",
      "Directive evidence refs: #{Enum.join(demo.steered.evidence_refs, ", ")}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.trusted_write_authority? == false and demo.gated_write_verified? and
      Enum.all?(demo.steered.accepted_directives, &(&1.status == :accepted)) and
      demo.steered.rejected_directives == []
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :running_autonomously and
      demo.autonomous.ready_task_ids != []
  end

  def fixture_plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m77-live-steering-demo",
      "tasks" => [
        task("01-broad-approach", "explore"),
        task("02-fast-prototype", "prototype"),
        task("03-polish", "polish")
      ]
    })
  end

  def fixture_directives(campaign_id) do
    [
      SteeringDirective.new(%{
        id: "demo-directive-prioritize-polish",
        campaign_id: campaign_id,
        action: :reprioritize,
        target: "03-polish",
        reason: "operator asks to inspect polish evidence first",
        actor_id: "demo-human-operator",
        issued_at: "2026-06-09T00:00:00Z",
        base_refs: ["runs/m77/live-steering-demo/status.json"]
      }),
      SteeringDirective.new(%{
        id: "demo-directive-exclude-broad-approach",
        campaign_id: campaign_id,
        action: :exclude_approach,
        target: "01-broad-approach",
        approach: "explore",
        reason: "operator narrows this demo away from exploratory broad approach",
        actor_id: "demo-human-operator",
        issued_at: "2026-06-09T00:00:01Z",
        base_refs: ["runs/m77/live-steering-demo/status.json"]
      })
    ]
  end

  defp gated_write_verified?(steered) do
    steered.accepted_directives != [] and steered.evidence_refs != [] and
      steered.ready_task_ids == ["03-polish", "02-fast-prototype"] and
      steered.trusted_write_authority? == false
  end

  defp task(id, kind) do
    %{
      "taskId" => id,
      "functionAgent" => "executor",
      "role" => "local-control-plane",
      "kind" => kind,
      "dependsOn" => [],
      "inputs" => ["input:#{id}"],
      "outputs" => ["output:#{id}"]
    }
  end
end
