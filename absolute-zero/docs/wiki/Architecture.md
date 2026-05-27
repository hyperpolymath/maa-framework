<!-- SPDX-License-Identifier: MPL-2.0 -->
# Architecture

Three layers, loosely coupled.

## 1. Proof layer (`proofs/`)

The load-bearing verification. Six prover backends prove the same CNO
properties from different angles:

| Prover | Strength | What we prove with it |
|--------|----------|------------------------|
| **Coq** | Constructive, mature | Core CNO theory, statistical mechanics (Landauer), category, lambda, quantum, filesystem |
| **Lean 4** | Mathlib + tooling | Modern restatement, cross-validation |
| **Z3** | Automated SMT | Decidable fragments — fast counterexample search |
| **Agda** | Dependent types | Type-level CNO certificates |
| **Isabelle/HOL** | Production-grade | Industrial-style verification |
| **Mizar** | Mathematical library | Connection to standard maths corpus |

Multi-prover is intentional: each catches what the others miss. See
[Proof Systems](Proof-Systems.md) for the choice rationale.

## 2. Interpreter + ABI layer (`src/`, `interpreters/`, `ffi/`)

* **`src/abi/`** — Idris2 type declarations for the C ABI, with formal
  alignment + size proofs. See [ABI](ABI.md).
* **`interpreters/rescript/`** — Malbolge interpreter in ReScript with
  CNO detection. ReScript is the project's primary application
  language (per [`docs/CLAUDE.adoc`](../CLAUDE.adoc) language policy).
* **`ffi/zig/`** — Zig FFI shim that the Idris2 ABI binds to.

## 3. Examples + tooling (`examples/`, `verification/`, `Justfile`)

* **`examples/<lang>/`** — CNOs in 23+ languages (ada, brainfuck, c,
  clojure, cobol, cpp, csharp, elixir, erlang, fortran, fsharp, haskell,
  javascript, lisp, malbolge, ocaml, php, prolog, rust, scala, scheme,
  special-ops, whitespace). Banned-language examples (go, java, kotlin,
  swift, ruby, perl) were removed 2026-05-25 per project policy.
* **`verification/`** — top-level verify scripts (`verify-proofs.sh`,
  `setup-and-verify.sh`, `run-local-verification.sh`).
* **`Justfile`** — recipes for everything; `just build-all`, `just verify`, etc.

## ECHIDNA integration (external)

ECHIDNA is the estate-wide neurosymbolic prover gateway, separate repo.
absolute-zero calls into it via the `echidna-llm-mcp` BoJ cartridge —
**never directly**. See [ECHIDNA.adoc](../../ECHIDNA.adoc).

```
┌──────────────────────────┐         ┌──────────────────────────┐
│   absolute-zero          │ calls   │   ECHIDNA (separate)     │
│   - proofs/              ├────────►│   - neural tactics       │
│   - src/                 │ via BoJ │   - 105 prover backends  │
│   - interpreters/        │         │   - 66,674-proof corpus  │
└──────────────────────────┘         └──────────────────────────┘
```

## Data flow on `just verify`

```
verification/setup-and-verify.sh
  ├── proofs/coq         → coqc -R common CNO ...
  ├── proofs/lean4       → lake build
  ├── proofs/agda        → agda CNO.agda
  ├── proofs/isabelle    → isabelle build
  ├── proofs/z3          → z3 *.smt2
  ├── src/abi            → idris2 --build absolute-zero-abi.ipkg
  └── cargo build --release
```

## Why this taxonomy

The directory layout follows the [RSR-template-repo](../RSR_OUTLINE.adoc)
convention. Compliance state is tracked in [RSR_COMPLIANCE.adoc](../../RSR_COMPLIANCE.adoc).
