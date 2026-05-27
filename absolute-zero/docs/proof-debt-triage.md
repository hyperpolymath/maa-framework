<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Proof Debt — Per-Marker Triage (Coq Axioms)

Companion to [`docs/proof-debt.md`](./proof-debt.md) (the standards#203
seed). This file classifies **every Coq `Axiom` declaration** in this
repo into the three dispositions from the standards#203
trusted-base-reduction policy:

- **AXIOM** — legitimate model-layer assumption (TRUSTED-BASE / §(c)).
  The marker encodes a physical postulate, an opaque-primitive
  specification, or a metatheoretic assumption that cannot be
  discharged within the working logic.
- **DISCHARGE** — a real provable proposition that is currently stated
  as `Axiom` for expedience. Targeted for a future proof PR (§(a)).
- **PROPERTY-TEST** — empirical claim better validated by a refutation
  budget under §(b); typically decidability over opaque types.

## Scope

This pass triages **only the 72 Coq Axioms**. Out of scope here, but
still markers in `check-trusted-base.sh`:

- 52 Lean 4 `axiom` declarations (FilesystemCNO.lean, LambdaCNO.lean,
  QuantumCNO.lean, StatMech.lean).
- 7 Idris2 `postulate`s in `src/abi/Layout.idr` (tracked by [#27]).
- 0 Coq `Admitted.` or `admit.`
- 0 Lean `sorry`, 0 Agda postulates, 0 Rust `unsafePerformIO` /
  `unsafeCoerce`.

`docs/proof-debt.md` records **129** total markers (seeded 2026-05-26).
`check-trusted-base.sh` against `origin/main` today reports **124** —
five fewer, reflecting in-flight closures since the seed.

[#27]: https://github.com/hyperpolymath/absolute-zero/issues/27

## Summary

| Disposition | Count | %    |
|-------------|------:|-----:|
| AXIOM       |    52 |  72% |
| DISCHARGE   |    17 |  24% |
| PROPERTY-TEST |   3 |   4% |
| **Total**   |  **72** | 100% |

## Per-axiom table

### `proofs/coq/quantum/QuantumMechanicsExact.v` (3)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
| 249  | `X_gate_unitary` | AXIOM | Pauli-X is a primitive quantum gate; unitarity is its defining property in the model. |
| 316  | `unitary_preserves_entropy` | AXIOM | Quantum statmech postulate (von Neumann entropy invariant under unitary). |
| 393  | `no_cloning` | AXIOM | Fundamental quantum theorem; standardly taken as physical postulate in this style of axiomatisation. |

### `proofs/coq/quantum/QuantumCNO.v` (29)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
|  31  | `kB_positive` | AXIOM | Boltzmann constant — physical constant. |
|  35  | `temperature_positive` | AXIOM | Temperature scalar — physical precondition. |
|  45  | `dim_positive` | AXIOM | Hilbert-space dimensionality precondition. |
|  68  | `inner_product_conj_sym` | AXIOM | Inner product space axiom (conjugate symmetry). |
|  73  | `inner_product_linear` | AXIOM | Inner product space axiom (linearity). |
|  80  | `inner_product_pos_def` | AXIOM | Inner product space axiom (positive definiteness). |
| 113  | `X_gate_unitary` | AXIOM | Quantum gate primitive (duplicate of QuantumMechanicsExact:249 — see followup). |
| 117  | `Y_gate_unitary` | AXIOM | Quantum gate primitive. |
| 121  | `Z_gate_unitary` | AXIOM | Quantum gate primitive. |
| 125  | `H_gate_unitary` | AXIOM | Quantum gate primitive. |
| 129  | `CNOT_gate_unitary` | AXIOM | Quantum gate primitive. |
| 150  | `Cexp_zero` | AXIOM | Complex exponential algebra (would move to DISCHARGE if `Complex.v` defines `Cexp` constructively — currently the file has 0 axioms but no `Cexp`). |
| 153  | `Cexp_neg` | AXIOM | Complex exponential algebra (see L150 note). |
| 156  | `Cexp_add` | AXIOM | Complex exponential algebra (see L150 note). |
| 163  | `Cconj_Cexp` | AXIOM | Complex exponential algebra (see L150 note). |
| 258  | `global_phase_unitary` | DISCHARGE | Derivable from gate algebra: `(e^{iθ} U)` is unitary iff `U` is. |
| 283  | `X_gate_not_identity` | DISCHARGE | Existence proof; exhibit `|0⟩` as witness once a concrete basis state is in the model. |
| 296  | `H_gate_not_identity` | DISCHARGE | Existence proof; exhibit `|0⟩` as witness. |
| 361  | `von_neumann_nonneg` | AXIOM | Quantum statmech — von Neumann entropy non-negativity. |
| 366  | `von_neumann_pure_zero` | AXIOM | `S(|ψ⟩⟨ψ|) = 0` for pure states. |
| 372  | `unitary_preserves_entropy` | AXIOM | Duplicate of QuantumMechanicsExact:316 (see followup). |
| 391  | `no_cloning` | AXIOM | Duplicate of QuantumMechanicsExact:393 (see followup). |
| 421  | `measure_identity_commutes` | AXIOM | Measurement postulate. |
| 487  | `unitary_inverse_property` | DISCHARGE | Follows from `is_unitary` definition (`U†U = I`). |
| 538  | `quantum_landauer_bound` | AXIOM | Physical postulate (quantum Landauer). |
| 545  | `unitary_zero_entropy_change` | DISCHARGE | Derivable from `unitary_preserves_entropy` + entropy definition. |
| 551  | `reversible_quantum_zero_dissipation` | DISCHARGE | Derivable from `quantum_landauer_bound` + unitarity. |
| 584  | `fidelity_bound` | DISCHARGE | Provable from `inner_product_pos_def` + Cauchy-Schwarz. |
| 587  | `approximate_cno` | AXIOM | Definitional / structural — encodes a relation, not a derivable fact. |

### `proofs/coq/category/CNOCategory.v` (1)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
| 323  | `hom_functor` | DISCHARGE | Standard categorical construction (Hom(C,A,-)); should be a `Definition` building the functor record, not an `Axiom`. |

### `proofs/coq/filesystem/FilesystemCNO.v` (13)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
|  96  | `fs_eq_dec` | PROPERTY-TEST | Decidable equality over a list of entries containing opaque `FileContent`; needs an oracle, or a §(b) budget over a concrete content type. |
| 104  | `mkdir_rmdir_inverse` | AXIOM | POSIX-semantics specification (model-layer). |
| 114  | `create_unlink_inverse` | AXIOM | POSIX-semantics specification. |
| 124  | `read_write_identity` | AXIOM | POSIX-semantics specification. |
| 130  | `chmod_identity` | AXIOM | POSIX-semantics specification. |
| 136  | `chown_identity` | AXIOM | POSIX-semantics specification. |
| 142  | `rename_identity` | AXIOM | POSIX-semantics specification. |
| 147  | `rename_inverse` | AXIOM | POSIX-semantics specification. |
| 300  | `mkdir_not_identity` | DISCHARGE | Existence proof; exhibit one concrete `fs` lacking the path. |
| 316  | `write_different_not_identity` | DISCHARGE | Existence proof; exhibit one concrete content mismatch. |
| 397  | `transaction_cno` | DISCHARGE | Composite theorem; derivable from primitive `_inverse` axioms once a `transaction` definition is in place. |
| 421  | `mkdir_idempotent` | DISCHARGE | Follows from the `mkdir_rmdir_inverse` family + a stronger semantics for repeat `mkdir`. |
| 453  | `snapshot_restore_identity` | DISCHARGE | Composite theorem; derivable from primitive `_identity` / `_inverse` axioms. |

### `proofs/coq/physics/StatMech.v` (10)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
|  25  | `kB_positive` | AXIOM | Physical constant (duplicate — see consolidation followup). |
|  30  | `temperature_positive` | AXIOM | Physical precondition (duplicate). |
|  39  | `prob_nonneg` | AXIOM | Kolmogorov probability axiom. |
|  45  | `prob_normalized` | AXIOM | Kolmogorov probability axiom (Σp = 1). |
|  51  | `state_dec` | PROPERTY-TEST | Decidable equality over opaque `ProgramState`; needs oracle or §(b) budget. |
|  67  | `shannon_entropy_nonneg` | AXIOM | Shannon entropy core inequality. |
|  72  | `shannon_entropy_point_zero` | AXIOM | `H(δ_x) = 0`. |
|  77  | `shannon_entropy_maximum` | AXIOM | `H ≤ log n` (Gibbs inequality). |
| 132  | `landauer_principle` | AXIOM | Physical postulate (Landauer's principle). |
| 229  | `reversible_zero_dissipation` | DISCHARGE | Derivable from `landauer_principle` + reversibility hypothesis. |

### `proofs/coq/physics/LandauerDerivation.v` (14)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
|  28  | `kB_positive` | AXIOM | Physical constant (duplicate of StatMech:25). |
|  32  | `temperature_positive` | AXIOM | Physical precondition (duplicate). |
|  40  | `prob_nonneg` | AXIOM | Kolmogorov axiom (duplicate of StatMech:39). |
|  43  | `prob_normalized` | AXIOM | Kolmogorov axiom (duplicate of StatMech:45). |
|  48  | `state_eq_dec` | PROPERTY-TEST | Decidable equality over opaque `ProgramState` (duplicate of StatMech:51). |
|  63  | `shannon_entropy_nonneg` | AXIOM | Duplicate of StatMech:67. |
|  67  | `shannon_entropy_point_zero` | AXIOM | Duplicate of StatMech:72. |
|  71  | `shannon_entropy_uniform_max` | AXIOM | Variant of Gibbs inequality for uniform distributions. |
|  81  | `shannon_entropy_additive` | DISCHARGE | Chain rule of entropy; provable from the definition of `H(X,Y)` given independence hypothesis. |
| 126  | `second_law` | AXIOM | Physical postulate (second law of thermodynamics). |
| 181  | `entropy_change_erasure` | AXIOM | Landauer–Bennett result. |
| 197  | `isothermal_work_bound` | AXIOM | Thermodynamic bound (Helmholtz free energy). |
| 277  | `cno_preserves_shannon_entropy` | DISCHARGE | Should follow from the CNO definition (`state in = state out`) + functional Shannon entropy. |
| 326  | `cno_zero_energy_dissipation_derived` | DISCHARGE | Name literally says `_derived`; the file appears to admit this rather than discharge it. |

### `proofs/coq/lambda/LambdaCNO.v` (2)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
| 356  | `y_not_cno` | AXIOM | Non-termination claim about the Y combinator; the file's leading comment justifies axiomatisation ("requires step-indexed semantics or coinduction") and explicitly declares it safely axiomatised. |
| 376  | `eta_equivalence` | AXIOM | η-equivalence is not derivable under β-only reduction; needs an extra reduction rule or extensional equality. Metatheoretic assumption. |

## Follow-ups surfaced by triage

These are concrete sub-projects that fall out of the table. Each is
its own PR-sized piece of work — none of them is in scope for this
triage PR.

1. **De-duplicate physics constants.** `kB_positive` and
   `temperature_positive` are axiomatised three times (QuantumCNO,
   StatMech, LandauerDerivation). Move to a shared `Physics.Constants`
   module and import.
2. **De-duplicate quantum laws.** `unitary_preserves_entropy` and
   `no_cloning` appear in both `QuantumMechanicsExact.v` and
   `QuantumCNO.v` with the same name. Pick one as canonical.
3. **De-duplicate decidability + probability + Shannon axioms.**
   `prob_nonneg`, `prob_normalized`, `state_eq_dec`/`state_dec`,
   `shannon_entropy_nonneg`, `shannon_entropy_point_zero` are
   duplicated between `StatMech.v` and `LandauerDerivation.v`.
4. **Constructively define `Cexp` in `Complex.v`.** `Complex.v` has
   zero axioms today; if it defines `Cexp` from a power series (or
   imports from a Coq reals/complex stdlib), the four `Cexp_*` axioms
   in QuantumCNO collapse to DISCHARGE.
5. **17 DISCHARGE cluster.** The DISCHARGE column is the work backlog
   for subsequent proof PRs. The lowest-hanging are the four `_derived`
   / `_not_identity` / `unitary_*` axioms in QuantumCNO that fall out
   directly from the existing definitions.

## Methodology

1. Inventory generated by:
   ```bash
   grep -nE '^[[:space:]]*Axiom[[:space:]]' proofs/coq/**/*.v
   ```
2. Each axiom was classified by reading its declared type and the
   nearest doc-comment / section header.
3. Disposition follows the standards#203 schema; the user-facing
   vocabulary (DISCHARGE / PROPERTY-TEST / AXIOM) maps 1:1 to
   §(a) / §(b) / §(c) in `docs/proof-debt.md`.
4. Counts triple-checked against the per-file `Axiom` tally
   (`3+29+1+13+10+14+2 = 72`).

---

🤖 Phase 1 triage by Claude Code, 2026-05-27.
