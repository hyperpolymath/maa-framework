import Lake
open Lake DSL

package cno where
  version := "0.1.0"

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"

lean_lib CNO where
  -- Core CNO library

@[default_target]
lean_exe absolute_zero where
  root := `CNO
