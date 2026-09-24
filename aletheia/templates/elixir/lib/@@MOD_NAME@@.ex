# SPDX-License-Identifier: MPL-2.0
defmodule @@MOD_CAMEL@@ do
  @moduledoc """
  Core library for @@PROJECT_NAME@@.

  Replace these sample functions with your own.
  """

  @doc """
  Clamp `value` into the inclusive range `lo..hi`.

  Raises if `lo > hi`, because an inverted range is a programming error
  rather than a silent no-op.

  ## Examples

      iex> @@MOD_CAMEL@@.clamp(5, 0, 10)
      5
      iex> @@MOD_CAMEL@@.clamp(99, 0, 10)
      10
  """
  @spec clamp(non_neg_integer(), non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  def clamp(value, lo, hi) when lo <= hi do
    value |> max(lo) |> min(hi)
  end

  @doc """
  Integer mean of two non-negative integers, rounded towards zero.

  The naive `div(a + b, 2)` is fine on the BEAM, where integers are
  arbitrary precision; this form is kept because it is explicit about
  intent and mirrors the other templates in this estate.

  ## Examples

      iex> @@MOD_CAMEL@@.mean_floor(8, 11)
      9
  """
  @spec mean_floor(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  def mean_floor(a, b) do
    Bitwise.band(a, b) + Bitwise.bsr(Bitwise.bxor(a, b), 1)
  end

  @doc "Returns the project version."
  @spec version() :: String.t()
  def version, do: "0.1.0"
end
