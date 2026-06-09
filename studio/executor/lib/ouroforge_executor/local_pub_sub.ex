defmodule OuroforgeExecutor.LocalPubSub do
  @moduledoc """
  Tiny local PubSub adapter for Studio/executor live-state fanout.

  It uses an OTP Registry owned by the local executor application. Broadcasts are
  presentation/control-plane messages only: they do not write artifacts, ledgers,
  evidence, source, scenes, or evaluator truth. Phoenix LiveView can subscribe to
  the same topics when mounted locally.
  """

  @default_registry OuroforgeExecutor.PubSub

  def child_spec(opts \\ []) do
    registry = Keyword.get(opts, :name, @default_registry)
    Registry.child_spec(keys: :duplicate, name: registry)
  end

  def subscribe(topic, registry \\ @default_registry) when is_binary(topic) and topic != "" do
    Registry.register(registry, topic, [])
  end

  def broadcast(topic, message, registry \\ @default_registry)
      when is_binary(topic) and topic != "" do
    Registry.dispatch(registry, topic, fn entries ->
      for {pid, _value} <- entries do
        send(pid, {:ouroforge_executor_pubsub, topic, message})
      end
    end)

    :ok
  end
end
