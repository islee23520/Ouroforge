defmodule OuroforgeExecutor.RetryPolicy do
  @moduledoc """
  Bounded retry/backoff policy for executor control-plane task attempts.

  The policy schedules local retries only while the current budget gate permits
  more assignment/CLI drive. It does not loop unboundedly, write artifacts, or
  decide product truth.
  """

  defstruct max_attempts: 3, base_backoff_ms: 100, max_backoff_ms: 1_000

  defmodule Decision do
    @moduledoc false
    defstruct [:action, :attempt, :delay_ms, :reason, :diagnosis]
  end

  def new(opts \\ []) do
    policy = %__MODULE__{
      max_attempts: Keyword.get(opts, :max_attempts, 3),
      base_backoff_ms: Keyword.get(opts, :base_backoff_ms, 100),
      max_backoff_ms: Keyword.get(opts, :max_backoff_ms, 1_000)
    }

    validate!(policy)
  end

  def decide(%__MODULE__{} = policy, attempt, budget_decision, failure_reason \\ :failed)
      when is_integer(attempt) and attempt >= 1 do
    cond do
      budget_halted?(budget_decision) ->
        %Decision{
          action: :halt,
          attempt: attempt,
          delay_ms: 0,
          reason: :budget_or_stop_condition,
          diagnosis: Map.get(budget_decision, :diagnosis, "budget/stop condition blocks retry")
        }

      attempt >= policy.max_attempts ->
        %Decision{
          action: :halt,
          attempt: attempt,
          delay_ms: 0,
          reason: :retry_exhausted,
          diagnosis:
            "retry exhausted after #{attempt}/#{policy.max_attempts} attempts: #{inspect(failure_reason)}"
        }

      true ->
        %Decision{
          action: :retry,
          attempt: attempt + 1,
          delay_ms: backoff_ms(policy, attempt),
          reason: :retryable_failure,
          diagnosis:
            "retrying attempt #{attempt + 1}/#{policy.max_attempts} after #{inspect(failure_reason)}"
        }
    end
  end

  defp validate!(%__MODULE__{} = policy) do
    if policy.max_attempts < 1, do: raise(ArgumentError, "max_attempts must be at least 1")

    if policy.base_backoff_ms < 0,
      do: raise(ArgumentError, "base_backoff_ms must be non-negative")

    if policy.max_backoff_ms < policy.base_backoff_ms,
      do: raise(ArgumentError, "max_backoff_ms must be >= base_backoff_ms")

    policy
  end

  defp backoff_ms(%__MODULE__{} = policy, attempt) do
    multiplier = Integer.pow(2, max(attempt - 1, 0))
    min(policy.base_backoff_ms * multiplier, policy.max_backoff_ms)
  end

  defp budget_halted?(%{may_drive_cli?: false}), do: true
  defp budget_halted?(%{may_assign?: false}), do: true
  defp budget_halted?(_), do: false
end
