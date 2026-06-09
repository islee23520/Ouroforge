defmodule OuroforgeExecutor.LiveSteeringTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{LiveSteering, LocalPubSub, ProductionPlan, SteeringDirective}

  defp plan do
    ProductionPlan.from_map!(%{
      "schemaVersion" => "producer-plan-v1",
      "planId" => "m77-live-steering",
      "tasks" => [
        task("01-broad-approach", "explore"),
        task("02-pinned-approach", "prototype"),
        task("03-polish", "polish")
      ]
    })
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

  defp directive(attrs) do
    defaults = %{
      id: "directive-#{System.unique_integer([:positive])}",
      campaign_id: "m77-live-steering",
      actor_id: "human-operator",
      reason: "bounded live steering request",
      issued_at: "2026-06-09T00:00:00Z",
      base_refs: ["runs/m77/live-state.json"]
    }

    SteeringDirective.new(Map.merge(defaults, Map.new(attrs)))
  end

  def runner("ouroforge", ["loop", "step", campaign_id | argv], _opts) do
    phase =
      argv
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.find_value(fn
        ["--directive-phase", phase] -> phase
        _ -> nil
      end)

    id =
      argv
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.find_value(fn
        ["--directive-id", id] -> id
        _ -> nil
      end)

    {"evidenceRef=runs/#{campaign_id}/directives/#{id}-#{phase}.json", 0}
  end

  test "directive validation rejects raw or unsupported intervention shapes before CLI routing" do
    assert {:error, {:missing_text, :reason}} =
             %{directive(action: :pause) | reason: ""} |> SteeringDirective.validate()

    assert {:error, {:missing_text, :constraint}} =
             directive(action: :add_constraint) |> SteeringDirective.validate()

    assert {:ok, %SteeringDirective{action: :exclude_approach}} =
             directive(action: :exclude_approach, approach: "explore")
             |> SteeringDirective.validate()
  end

  test "validated directives are recorded via the constrained Rust CLI and steer ready work" do
    registry = Module.concat(__MODULE__, "Registry#{System.unique_integer([:positive])}")
    start_supervised!({Registry, keys: :duplicate, name: registry})
    assert {:ok, _} = LocalPubSub.subscribe(LiveSteering.topic("m77-live-steering"), registry)

    directives = [
      directive(action: :exclude_approach, approach: "explore", target: "01-broad-approach"),
      directive(action: :reprioritize, target: "03-polish"),
      directive(action: :add_constraint, constraint: "Keep evidence provenance visible")
    ]

    result =
      LiveSteering.consume(plan(), directives,
        runner: &__MODULE__.runner/3,
        pubsub: registry
      )

    assert result.status == :running_autonomously
    assert result.ready_task_ids == ["03-polish", "02-pinned-approach"]
    assert result.constraints == ["Keep evidence provenance visible"]
    assert result.excluded_approaches == ["explore"]
    assert length(result.accepted_directives) == 3
    assert result.rejected_directives == []
    assert result.trusted_write_authority? == false

    assert Enum.all?(result.accepted_directives, fn directive ->
             directive.status == :accepted and
               String.contains?(directive.record_evidence_ref, "directives")
           end)

    assert_receive {:ouroforge_executor_pubsub, "campaign:m77-live-steering:live-steering",
                    {:live_steering, ^result}}
  end

  test "pause and no-directive paths preserve autonomous-first behavior" do
    paused =
      LiveSteering.consume(plan(), [directive(action: :pause)], runner: &__MODULE__.runner/3)

    assert paused.status == :paused_by_validated_directive
    assert paused.paused?
    assert paused.ready_task_ids == []
    assert paused.notes |> Enum.any?(&String.contains?(&1, "not artifact mutation"))

    autonomous = LiveSteering.consume(plan(), [], runner: &__MODULE__.runner/3)
    assert autonomous.status == :running_autonomously
    assert autonomous.ready_task_ids == ["01-broad-approach", "02-pinned-approach", "03-polish"]
    assert autonomous.notes == ["no human directive supplied; autonomous loop continues"]
  end
end
