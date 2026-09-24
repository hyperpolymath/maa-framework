# SPDX-License-Identifier: MPL-2.0
defmodule @@MOD_CAMEL@@.MixProject do
  use Mix.Project

  def project do
    [
      app: :@@MOD_NAME@@,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      # EMPTY BY DESIGN: no Hex dependencies, so nothing is fetched at
      # build time and `mix compile` works with no network access.
      # See 0-AI-MANIFEST.a2ml, invariant 2.
      deps: []
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end
end
