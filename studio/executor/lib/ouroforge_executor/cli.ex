defmodule OuroforgeExecutor.CLI do
  @moduledoc """
  Constrained adapter for driving the Rust `ouroforge` data plane.

  This module is the only executor boundary that may spawn engine actions. It
  validates every argv against the M62/M63 frozen CLI surface, blocks direct
  artifact/ledger write command families, and captures stdout/stderr/exit status
  as operational control-plane state only. Kernel artifacts and verdicts remain
  Rust-owned product truth.
  """

  alias OuroforgeExecutor.Contract

  defmodule Result do
    @moduledoc """
    Operational result captured by the control plane.

    This is not product truth and must not be used to self-certify a trusted
    write. Canonical state remains in Rust-owned artifacts.
    """

    defstruct [:argv, :status, :stdout, :stderr, :command_family]
  end

  @trusted_write_families [
    ["mutation", "review"],
    ["mutation", "apply-scene"],
    ["edit", "draft-apply"],
    ["behavior", "apply", "transaction", "validate"],
    ["scenario", "promote"]
  ]

  def run(argv, opts \\ []) when is_list(argv) do
    with {:ok, argv} <- normalize_argv(argv),
         :ok <- ensure_allowed(argv) do
      invoke(argv, opts)
    end
  end

  def trusted_write(argv, context \\ %{}, opts \\ []) when is_list(argv) and is_map(context) do
    with {:ok, argv} <- normalize_argv(argv),
         :ok <- ensure_allowed(argv),
         :ok <- ensure_trusted_write_family(argv),
         :ok <- ensure_not_self_certifying(argv, context) do
      invoke(argv, opts)
    end
  end

  def allowed?(argv) when is_list(argv) do
    case normalize_argv(argv) do
      {:ok, normalized} -> Contract.allowed_cli_family?(normalized)
      {:error, _} -> false
    end
  end

  def forbidden?(argv) when is_list(argv) do
    case normalize_argv(argv) do
      {:ok, normalized} -> Contract.forbidden_cli_family?(normalized)
      {:error, _} -> true
    end
  end

  defp invoke(argv, opts) do
    executable =
      Keyword.get(
        opts,
        :executable,
        Application.get_env(:ouroforge_executor, :ouroforge_cli, "ouroforge")
      )

    runner = Keyword.get(opts, :runner, &System.cmd/3)

    command_opts =
      opts
      |> Keyword.take([:cd, :env, :stderr_to_stdout])
      |> Keyword.put_new(:stderr_to_stdout, false)

    case runner.(executable, argv, command_opts) do
      {stdout, 0} ->
        {:ok, result(argv, 0, stdout, "")}

      {output, status} when is_integer(status) ->
        {:error, result(argv, status, output, "")}
    end
  end

  defp result(argv, status, stdout, stderr) do
    %Result{
      argv: argv,
      status: status,
      stdout: stdout,
      stderr: stderr,
      command_family: Contract.cli_family(argv)
    }
  end

  defp normalize_argv(argv) do
    if Enum.all?(argv, &(is_binary(&1) and &1 != "")) do
      {:ok, argv}
    else
      {:error, {:invalid_argv, argv}}
    end
  end

  defp ensure_allowed(argv) do
    cond do
      Contract.forbidden_cli_family?(argv) ->
        {:error, {:forbidden_cli_surface, Contract.cli_family(argv)}}

      Contract.allowed_cli_family?(argv) ->
        :ok

      true ->
        {:error, {:outside_frozen_cli_surface, argv}}
    end
  end

  defp ensure_trusted_write_family(argv) do
    if Enum.any?(@trusted_write_families, &prefix?(&1, argv)) do
      :ok
    else
      {:error, {:not_trusted_write_route, Contract.cli_family(argv)}}
    end
  end

  defp ensure_not_self_certifying(argv, context) do
    actor_id =
      Map.get(context, :actor_id) || Map.get(context, "actorId") || Map.get(context, "actor_id")

    reviewer_id = reviewer_id(argv, context)
    reviewer_type = reviewer_type(argv, context)

    cond do
      prefix?(["mutation", "review"], argv) and reviewer_type != nil and reviewer_type != "human" ->
        {:error, {:review_must_be_human_for_trusted_write, reviewer_type}}

      actor_id != nil and reviewer_id != nil and actor_id == reviewer_id ->
        {:error, {:self_certification_blocked, actor_id}}

      true ->
        :ok
    end
  end

  defp reviewer_id(argv, context) do
    Map.get(context, :reviewer_id) || Map.get(context, "reviewerId") ||
      Map.get(context, "reviewer_id") ||
      option_value(argv, "--reviewer")
  end

  defp reviewer_type(argv, context) do
    value =
      Map.get(context, :reviewer_type) || Map.get(context, "reviewerType") ||
        Map.get(context, "reviewer_type") ||
        option_value(argv, "--reviewer-type")

    if value == nil, do: nil, else: value |> to_string() |> String.downcase()
  end

  defp option_value(argv, option) do
    argv
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.find_value(fn
      [^option, value] -> value
      _ -> nil
    end)
  end

  defp prefix?(prefix, argv), do: Enum.take(argv, length(prefix)) == prefix
end
