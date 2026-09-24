# SPDX-License-Identifier: MPL-2.0
defmodule @@MOD_CAMEL@@Test do
  use ExUnit.Case, async: true

  doctest @@MOD_CAMEL@@

  describe "clamp/3" do
    test "keeps a value already inside the range" do
      assert @@MOD_CAMEL@@.clamp(5, 0, 10) == 5
    end

    test "lifts a value below the range to the lower bound" do
      assert @@MOD_CAMEL@@.clamp(0, 1, 10) == 1
    end

    test "drops a value above the range to the upper bound" do
      assert @@MOD_CAMEL@@.clamp(99, 0, 10) == 10
    end

    test "handles a degenerate range" do
      assert @@MOD_CAMEL@@.clamp(7, 7, 7) == 7
    end

    test "is idempotent" do
      for value <- [0, 1, 5, 42, 10_000] do
        once = @@MOD_CAMEL@@.clamp(value, 10, 100)
        assert @@MOD_CAMEL@@.clamp(once, 10, 100) == once
      end
    end
  end

  describe "mean_floor/2" do
    test "matches the naive form on small inputs" do
      for a <- 0..63, b <- 0..63 do
        assert @@MOD_CAMEL@@.mean_floor(a, b) == div(a + b, 2)
      end
    end

    test "never exceeds either input" do
      for {a, b} <- [{0, 0}, {1, 2}, {7, 9}, {10_000, 0}] do
        assert @@MOD_CAMEL@@.mean_floor(a, b) <= max(a, b)
      end
    end
  end
end
