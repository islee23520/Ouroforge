defmodule OuroforgeExecutor.CLITest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.CLI

  defp runner(expected_argv, response \\ {"ok", 0}) do
    fn executable, argv, opts ->
      assert executable == "ouroforge-test"
      assert argv == expected_argv
      assert Keyword.get(opts, :stderr_to_stdout) == false
      response
    end
  end

  test "runs only frozen CLI surface commands and captures operational results" do
    argv = ["run", "seeds/platformer.yaml", "--workers", "2"]

    assert {:ok, result} =
             CLI.run(argv,
               executable: "ouroforge-test",
               runner: runner(argv, {"Run created: runs/demo", 0})
             )

    assert result.argv == argv
    assert result.status == 0
    assert result.stdout == "Run created: runs/demo"
    assert result.command_family == ["run"]
  end

  test "captures non-zero CLI status without converting it into product truth" do
    argv = ["evaluate", "runs/demo"]

    assert {:error, result} =
             CLI.run(argv,
               executable: "ouroforge-test",
               runner: runner(argv, {"verdict failed", 2})
             )

    assert result.status == 2
    assert result.stdout == "verdict failed"
    assert result.command_family == ["evaluate"]
  end

  test "blocks direct artifact and ledger write command families" do
    assert {:error, {:forbidden_cli_surface, ["ledger", "append"]}} =
             CLI.run(["ledger", "append", "runs/demo", "--kind", "executor"])

    assert {:error, {:forbidden_cli_surface, ["evidence", "add"]}} =
             CLI.run(["evidence", "add", "runs/demo", "--id", "executor"])
  end

  test "blocks commands outside the frozen surface" do
    assert {:error, {:outside_frozen_cli_surface, ["release", "publish"]}} =
             CLI.run(["release", "publish"])
  end

  test "routes trusted writes only through review/apply/trust-gradient command families" do
    review = [
      "mutation",
      "review",
      "runs/demo",
      "--decision",
      "accepted",
      "--reason",
      "human approved",
      "--reviewer",
      "human-reviewer",
      "--reviewer-type",
      "human"
    ]

    assert {:ok, result} =
             CLI.trusted_write(review, %{actor_id: "executor"},
               executable: "ouroforge-test",
               runner: runner(review)
             )

    assert result.command_family == ["mutation", "review"]

    assert {:error, {:not_trusted_write_route, ["evaluate"]}} =
             CLI.trusted_write(["evaluate", "runs/demo"], %{actor_id: "executor"})
  end

  test "prevents executor self-certification and non-human trusted review" do
    review = [
      "mutation",
      "review",
      "runs/demo",
      "--decision",
      "accepted",
      "--reason",
      "self approval",
      "--reviewer",
      "executor",
      "--reviewer-type",
      "human"
    ]

    assert {:error, {:self_certification_blocked, "executor"}} =
             CLI.trusted_write(review, %{actor_id: "executor"})

    agent_review = List.replace_at(review, Enum.find_index(review, &(&1 == "human")), "agent")

    assert {:error, {:review_must_be_human_for_trusted_write, "agent"}} =
             CLI.trusted_write(agent_review, %{actor_id: "executor"})
  end
end
