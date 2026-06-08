import Config

config :ouroforge_executor,
  ouroforge_cli: System.get_env("OUROFORGE_CLI", "ouroforge")
