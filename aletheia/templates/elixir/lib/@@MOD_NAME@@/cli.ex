# SPDX-License-Identifier: MPL-2.0
defmodule @@MOD_CAMEL@@.CLI do
  @moduledoc """
  Command-line entry point for @@PROJECT_NAME@@.
  """

  @usage """
  @@PROJECT_NAME@@ 0.1.0

  usage:
    @@PROJECT_NAME@@ clamp <value> <lo> <hi>   clamp a value into a range
    @@PROJECT_NAME@@ mean  <a> <b>             integer mean, rounded down
    @@PROJECT_NAME@@ --help                    show this message
  """

  @doc """
  Runs the CLI with the given argument list.

  Returns an exit status. The Justfile's `run` recipe wires this up via
  `mix run -e '@@MOD_CAMEL@@.CLI.main(System.argv())'`, which exits with
  the status this returns.
  """
  @spec main([String.t()]) :: non_neg_integer()
  def main([]) do
    IO.puts(@usage)
    0
  end

  def main([flag]) when flag in ["--help", "-h"] do
    IO.puts(@usage)
    0
  end

  def main(["clamp", value, lo, hi]) do
    with {value, ""} <- Integer.parse(value),
         {lo, ""} <- Integer.parse(lo),
         {hi, ""} <- Integer.parse(hi),
         true <- lo <= hi do
      IO.puts(@@MOD_CAMEL@@.clamp(value, lo, hi))
      0
    else
      _ -> fail()
    end
  end

  def main(["mean", a, b]) do
    with {a, ""} <- Integer.parse(a),
         {b, ""} <- Integer.parse(b) do
      IO.puts(@@MOD_CAMEL@@.mean_floor(a, b))
      0
    else
      _ -> fail()
    end
  end

  def main(_), do: fail()

  defp fail do
    IO.puts(:stderr, "error: unrecognised arguments\n\n#{@usage}")
    1
  end
end
