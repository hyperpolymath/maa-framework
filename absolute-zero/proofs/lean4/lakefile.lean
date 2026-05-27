import Lake
open Lake DSL

package cno where
  version := v!"0.1.0"

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git" @ "v4.16.0"

-- Each proof file is exposed as its own library so `lake build` covers them
-- all. The `lean_exe absolute_zero` target was dropped — CNO.lean defines no
-- `main`, and the project's surface is theorem verification, not a binary.

@[default_target]
lean_lib CNO

@[default_target]
lean_lib CNOCategory

@[default_target]
lean_lib FilesystemCNO

@[default_target]
lean_lib LambdaCNO

@[default_target]
lean_lib QuantumCNO

@[default_target]
lean_lib StatMech
