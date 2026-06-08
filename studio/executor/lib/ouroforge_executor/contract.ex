defmodule OuroforgeExecutor.Contract do
  @moduledoc """
  Milestone 63 control-plane contract for the Studio executor.

  This module is declarative boundary metadata only. It does not execute the
  Rust kernel, write artifacts, certify reviews, or implement scheduling logic.
  The executor may drive the data plane only through the frozen `ouroforge` CLI
  surface recorded in `docs/distributed-elixir-design.md`.
  """

  @control_plane_responsibilities [
    :schedule,
    :supervise,
    :budget,
    :retry,
    :backpressure,
    :telemetry
  ]

  @data_plane_owner :rust_kernel

  @allowed_cli_surface [
    ["seed", "validate"],
    ["project", "validate"],
    ["asset", "validate"],
    ["asset", "audit-internal-sprites"],
    ["run"],
    ["browser", "smoke"],
    ["scenario", "run"],
    ["scenario", "promote-draft"],
    ["scenario", "promote"],
    ["evaluate"],
    ["evolve"],
    ["journal", "update"],
    ["journal", "show"],
    ["compare"],
    ["mutation", "create"],
    ["mutation", "review"],
    ["mutation", "apply-scene"],
    ["edit", "draft-preview"],
    ["edit", "draft-apply"],
    ["behavior", "draft", "validate"],
    ["behavior", "draft", "preview"],
    ["behavior", "apply", "transaction", "validate"],
    ["patch-preview", "validate"],
    ["patch-preview", "show"],
    ["ledger", "list"],
    ["evidence", "list"],
    ["dashboard", "export"],
    ["scene", "validate"],
    ["scene", "show"],
    ["scene", "reload-validate"],
    ["runtime-debug", "frame-budget", "validate"],
    ["runtime-debug", "frame-budget", "show"],
    ["plugin", "list"],
    ["plugin", "validate"],
    ["loop", "dry-run"],
    ["loop", "status"],
    ["loop", "resume"],
    ["loop", "step"],
    ["loop", "handoff"]
  ]

  @forbidden_cli_surface [
    ["ledger", "append"],
    ["evidence", "add"]
  ]

  @golden_parity %{
    required: true,
    comparator: :byte_identical_artifacts,
    manual_path: :ouroforge_cli,
    executor_path: :otp_control_plane_driving_ouroforge_cli,
    excludes: [:wall_clock_timestamps, :process_ids, :temporary_log_paths]
  }

  def control_plane_responsibilities, do: @control_plane_responsibilities
  def data_plane_owner, do: @data_plane_owner
  def allowed_cli_surface, do: @allowed_cli_surface
  def forbidden_cli_surface, do: @forbidden_cli_surface
  def golden_parity, do: @golden_parity

  def allowed_cli?(argv) when is_list(argv), do: argv in @allowed_cli_surface
  def forbidden_cli?(argv) when is_list(argv), do: argv in @forbidden_cli_surface

  def allowed_cli_family?(argv) when is_list(argv) do
    Enum.any?(@allowed_cli_surface, &prefix?(&1, argv))
  end

  def forbidden_cli_family?(argv) when is_list(argv) do
    Enum.any?(@forbidden_cli_surface, &prefix?(&1, argv))
  end

  def cli_family(argv) when is_list(argv) do
    Enum.find(@allowed_cli_surface ++ @forbidden_cli_surface, &prefix?(&1, argv)) || argv
  end

  defp prefix?(prefix, argv), do: Enum.take(argv, length(prefix)) == prefix
end
