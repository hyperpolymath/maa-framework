<!-- SPDX-License-Identifier: MPL-2.0 -->
# Proof Systems

## Why six?

Each prover trades off three axes:

| | Constructive | Automated | Mathlib |
|---|---|---|---|
| Coq         | ✅ | ⚪ | ⚪ |
| Lean 4      | ✅ | ⚪ | ✅ |
| Agda        | ✅ | ⚪ | ⚪ |
| Isabelle    | ⚪ | ⚪ | ✅ |
| Mizar       | ⚪ | ⚪ | ✅ |
| Z3          | ⚪ | ✅ | ⚪ |

A property proved in **all six** is robust against any single backend's
foundational quirks. A property proved in *some but not others*
flags a foundational dependency worth understanding.

## What lives where

### `proofs/coq/` (the load-bearing system)
* `common/CNO.v` — core CNO definitions and properties
* `physics/StatMech.v` — Landauer bound axiomatised
* `physics/LandauerDerivation.v` — Landauer bound derived (the two
  together form ADR-002's "dual formalisation")
* `category/CNOCategory.v` — CNOs as identity morphisms
* `lambda/LambdaCNO.v` — lambda calculus CNOs
* `quantum/QuantumCNO.v` — quantum CNOs
* `quantum/QuantumMechanicsExact.v` — exact QM formulation
* `malbolge/MalbolgeCore.v` — Malbolge VM semantics
* `filesystem/FilesystemCNO.v` — filesystem CNOs

Status: 11/11 files compile, 0 Admitted (post 2026-05-18 rescue).
73 Axioms + 42 Parameters, all owner-classified as legitimate
model-layer assumptions. See [`docs/PROOF-CLASSIFICATION.adoc`](../PROOF-CLASSIFICATION.adoc).

### `proofs/lean4/`
* `CNO.lean`, `StatMech.lean`, `CNOCategory.lean`, `LambdaCNO.lean`,
  `QuantumCNO.lean`, `FilesystemCNO.lean`
* Built with `lake build`; mathlib + 6 lean_lib targets;
  1631/1632 green (verified 2026-05-20).

### `proofs/z3/`
SMT proofs of decidable fragments. Fast counterexample search for
candidate CNO claims before committing to a constructive proof.

### `proofs/agda/`
Type-level CNO certificates exploiting dependent types.

### `proofs/isabelle/`, `proofs/mizar/`
Cross-validation. Catches Coq-specific tactic quirks; connects
Mizar's standard maths library.

## Reproducible build

```bash
just build-coq      # coqc -R common CNO physics/StatMech.v ...
just build-lean     # cd proofs/lean4 && lake build
just build-agda     # agda CNO.agda
just build-isabelle # isabelle build
just build-z3       # z3 *.smt2 (when present)
just verify         # all of the above + ABI + Rust
```

## Discharged axioms (recent)

* **ADR-007** (2026-05-20, PR #24): `eval_deterministic`
  Axiom → Theorem via `step_deterministic_strong`
* **ADR-008** (2026-05-20, PR #32): deleted unsound
  `eval_respects_state_eq_{left,right}` axioms; weakened
  `logically_reversible` to observational reversibility (`=st=`)
* **ADR-009** (2026-05-25, PR #41 / #40):
  deleted unsound Idris2 `alignmentMatchesPlatformWord` postulate;
  consolidated div/mod axioms into shared
  `AbsoluteZero.ABI.Proofs.DivMod`

See [Audit Trail](Audit-Trail.md) for the full ledger.

## Idris2 ABI (the seventh)

The Idris2 surface in `src/abi/` isn't a proof system per se, but
carries formal alignment + size proofs for the C FFI boundary. See
[ABI](ABI.md).
