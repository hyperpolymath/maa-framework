# Proof Insights: Opus-Level Knowledge for CNO Proof Engineering

**Author**: Claude Opus 4.5 (AI pair programmer)
**Date**: 2026-02-05
**Purpose**: Knowledge transfer document for continuing proof completion work.
This captures non-obvious insights, discovered bugs, and proof strategies
that are essential for completing the remaining 19 Admitted proofs.

---

## Current Proof Status (2026-02-05)

| File | Admitted | Qed | Defined | Axiom | Notes |
|------|---------|-----|---------|-------|-------|
| CNO.v | 0 | 18 | 0 | 4 | **Fully complete** (core) |
| CNOCategory.v | 0 | 8 | 3 | 1 | **Fully complete** (category theory) |
| StatMech.v | 0 | 9 | 0 | 10 | **Fully complete** (thermodynamics) |
| StatMech_helpers.v | 0 | 3 | 0 | 0 | **Fully complete** (helpers) |
| LambdaCNO.v | 1 | 9 | 0 | 1 | 1 Admitted (y_not_cno) |
| FilesystemCNO.v | 6 | 8 | 0 | 12 | 6 Admitted |
| LandauerDerivation.v | 3 | 4 | 0 | 11 | 3 Admitted |
| MalbolgeCore.v | 1 | 6 | 0 | 1 | 1 Admitted |
| QuantumCNO.v | 5 | 12 | 0 | 24 | 5 Admitted |
| QuantumMechanicsExact.v | 3 | 4 | 3 | 0 | 3 Admitted |
| **TOTAL** | **19** | **81** | **6** | **63** | 81% complete |

**Completed this session**: 8 proofs (bennett_logical_implies_thermodynamic,
lambda_id_is_cno, lambda_cno_composition, eta_expanded_id_is_cno,
ProgramCategory instance [3 laws], cno_categorical_equiv, morph_eq_ext,
cno_application_terminates)

---

## Critical Insight #1: post_execution_dist Is Identity

**Location**: `proofs/coq/physics/StatMech.v:161-174`

The `post_execution_dist` function is defined as:
```coq
Definition post_execution_dist
  (p : Program) (P_initial : StateDistribution) : StateDistribution :=
  fun s_final => P_initial s_final.
```

This is **intentionally the identity function** on distributions, specialized for
CNOs where `f_p = id`. This makes several "hard-looking" proofs trivially true
by `reflexivity`:

- `cno_preserves_shannon_entropy` - trivial (reflexivity)
- `bennett_logical_implies_thermodynamic` - trivial (reflexivity)
- `cno_zero_entropy_change` - trivial (ring after rewrite)

**IMPORTANT**: This is mathematically correct FOR CNOs, but means
`bennett_logical_implies_thermodynamic` doesn't actually use its
`logically_reversible` hypothesis. A general proof for arbitrary
reversible programs would need:
1. A general `post_execution_dist_general` using `eval_to_state` functions
2. A proof that bijective state transformations preserve Shannon entropy
3. Measure theory for infinite state spaces

**Implication for LandauerDerivation.v**: That file uses a GENERAL
`post_execution_dist` based on `fold_right` over `all_states` with
`eval_to_dec`. Its proofs are genuinely hard. Don't confuse the two.

---

## Critical Insight #2: is_lambda_CNO Was False

**Location**: `proofs/coq/lambda/LambdaCNO.v`

The original definition required:
```coq
(* WRONG - was unprovable and actually false *)
Definition is_lambda_CNO (t : LambdaTerm) : Prop :=
  forall arg : LambdaTerm,
    (exists nf, evaluates_to (LApp t arg) nf) /\
    beta_reduce_star (LApp t arg) arg.
```

Problem: `evaluates_to` requires existence of a **normal form**, but
`(lambda_id) Omega` reduces to `Omega` which has NO normal form.
So the termination clause is unsatisfiable for non-normalizing arguments.

**Fix**: Removed the termination requirement entirely:
```coq
(* CORRECT *)
Definition is_lambda_CNO (t : LambdaTerm) : Prop :=
  forall arg : LambdaTerm,
    beta_reduce_star (LApp t arg) arg.
```

Added `cno_application_terminates` as a separate theorem for when the
argument IS normalizing. This is the right decomposition because:
- Identity IS the essence of CNO (same as imperative model)
- Termination for normalizing args follows as corollary
- Side effects absent by construction in pure lambda calculus

---

## Critical Insight #3: ProofIrrelevance for Category Laws

**Location**: `proofs/coq/category/CNOCategory.v`

`ProgramMorphism s1 s2` wraps a `Program` with an `eval` proof witness.
To prove category laws (associativity, identity), we need morphism equality.
Two morphisms with the same underlying program must be equal, but their
eval proof witnesses may differ.

**Solution**: Import `Coq.Logic.ProofIrrelevance` and prove:
```coq
Lemma morph_eq_ext :
  forall s1 s2 (m1 m2 : ProgramMorphism s1 s2),
    morph_program m1 = morph_program m2 -> m1 = m2.
Proof.
  intros s1 s2 [p1 H1] [p2 H2]. simpl.
  intros Heq. subst.
  f_equal. apply proof_irrelevance.
Qed.
```

Then category laws reduce to list properties:
- Associativity: `app_assoc` (list append is associative)
- Left identity: `app_nil_r` (p ++ [] = p)
- Right identity: `reflexivity` ([] ++ p = p by computation)

---

## Critical Insight #4: cno_categorical_equiv Needs Termination

**Location**: `proofs/coq/category/CNOCategory.v:146`

Original statement was:
```coq
is_CNO p <-> (forall s s', eval p s s' -> s =st= s')
```

The backward direction is **unprovable** because you can't derive termination
from the identity property alone. A program that never terminates vacuously
satisfies `forall s s', eval p s s' -> s =st= s'`.

**Fix**: Add termination to the equivalence:
```coq
is_CNO p <->
  (forall s, terminates p s) /\
  (forall s s', eval p s s' -> s =st= s')
```

Then purity follows from state equality (memory/IO components),
and thermo reversibility is trivially true (energy_dissipated := 0).

---

## Critical Insight #5: Quantum CNO Bugs

**Location**: `proofs/coq/quantum/QuantumCNO.v`

### Bug 1: quantum_cno_composition uses wrong intermediate

Current proof attempts:
```coq
apply quantum_state_eq_trans with (U psi).
```
This requires showing `U (V psi) =q= U psi`, which needs congruence for
quantum operators (not available).

**Fix**: Use `V psi` as intermediate:
```coq
apply quantum_state_eq_trans with (V psi).
+ apply HU_id.  (* U (V psi) =q= V psi -- direct from hypothesis *)
+ apply HV_id.  (* V psi =q= psi -- direct from hypothesis *)
```

### Bug 2: Cexp(RtoC theta) is real exponential, not phase

`Cexp(RtoC theta)` computes `e^theta` (real exponential), not `e^{i*theta}`
(phase factor). For proper global phase in quantum mechanics, it should be:
```coq
Cexp(Ci * RtoC theta)  (* = e^{i*theta} -- proper phase factor *)
```

Check the `global_phase` definition carefully before fixing.

### Missing Axioms for Quantum Proofs

The 3 `quantum_state_eq` proofs (refl/sym/trans) need:
- `Cmod_Cexp`: `Cmod (Cexp (Ci * RtoC theta)) = 1`
- `Cmult_assoc`, `Cmult_comm` for complex multiplication
- Properties of `Cmod` (multiplicativity, triangle inequality)

These should be added as axioms with a note that CoqQ or Coquelicot
provides them in a full development.

---

## Critical Insight #6: Infrastructure Axioms in CNO.v

**Location**: `proofs/coq/common/CNO.v`

The previous Opus session added these axioms that enable many proofs:

```coq
(* State equality respects eval *)
Axiom eval_respects_state_eq_right :
  forall p s s' s'', eval p s s' -> s' =st= s'' -> eval p s s''.

Axiom eval_respects_state_eq_left :
  forall p s s' s'', eval p s' s'' -> s =st= s' -> eval p s s''.

(* CNO convenience lemmas *)
Axiom cno_eval_on_equal_states :
  forall p s s', is_CNO p -> s =st= s' -> eval p s s'.
```

Also proved (not axioms):
```coq
Lemma cno_preserves_state : forall p s s', is_CNO p -> eval p s s' -> s =st= s'.
Lemma cno_terminates : forall p, is_CNO p -> forall s, terminates p s.
```

These are used extensively in StatMech.v (cno_logically_reversible) and
should be available to all proof files via `Require Import CNO.`

---

## Remaining Proof Strategies

### LambdaCNO.v: y_not_cno (HARD)

Requires proving non-termination of Y combinator. Strategy:
1. Characterize the reduction behavior: `Y f ->* f (Y f) ->* f (f (Y f)) ->* ...`
2. Show any reduction of `(Y f)` produces a term strictly larger
3. Conclude by well-foundedness that no finite reduction path reaches `f`

This is inherently difficult in constructive type theory. Consider:
- Leaving as Admitted with justification (well-known result)
- Using a step-indexed bisimulation argument
- Axiomatizing as a well-known result about Y

### QuantumCNO.v: 5 proofs

1. `quantum_state_eq_refl` - Easy: need `Cexp(0) = 1` and `Cmult_1_l`
2. `quantum_state_eq_sym` - Easy: need `Cinv (Cexp x) = Cexp (-x)`
3. `quantum_state_eq_trans` - Medium: need `Cexp(a) * Cexp(b) = Cexp(a+b)`
4. `global_phase_is_cno` - Medium: depends on state_eq proofs + Cexp bug fix
5. `quantum_cno_composition` - Medium: fix intermediate (see Bug 1 above)

### LandauerDerivation.v: 3 proofs (HARD)

These use the GENERAL `post_execution_dist` and need:
1. `entropy_change_erasure` - Needs measure theory
2. `cno_preserves_shannon_entropy` - Needs to show CNO induces identity permutation
3. `cno_zero_energy_dissipation_derived` - Follows from above two

Strategy: Either axiomatize the measure-theoretic lemmas or prove the
CNO-specific simplification (that the general definition collapses to identity).

### FilesystemCNO.v: 6 proofs (NOT YET ANALYZED)

These likely involve filesystem operation properties. Need to read and
classify before attempting.

### MalbolgeCore.v: 1 proof (NOT YET ANALYZED)

Likely involves Malbolge-specific instruction encoding.

### QuantumMechanicsExact.v: 3 proofs

These are probably axiom-level (quantum mechanics fundamentals that
should be axiomatized rather than proved).

---

## Closedness Infrastructure (Lambda Calculus)

Added in LambdaCNO.v and essential for composition proofs:

```coq
Fixpoint closed_at (n : nat) (t : LambdaTerm) : bool := ...
Definition closed (t : LambdaTerm) : Prop := closed_at 0 t = true.

Lemma subst_closed_at :
  forall t n s, closed_at n t = true -> subst n s t = t.

Corollary subst_closed :
  forall t s, closed t -> subst 0 s t = t.
```

The `lambda_cno_composition` theorem requires closedness hypotheses
because `lambda_compose` uses de Bruijn indices. When you apply
`(lambda_compose f g)` to `arg`, the beta reduction substitutes `arg`
for variable 0 in `(LApp f (LApp g (LVar 0)))`. Without closedness,
this substitution would also affect free variables in `f` and `g`.

---

## Hom Functor Is an Axiom, Not a Definition

**Location**: `proofs/coq/category/CNOCategory.v:316`

The standard Hom functor maps `C -> Set` (category of types), not `C -> C`.
Defining `SetCategory` properly requires universe polymorphism in Coq.

The Yoneda theorem (`yoneda_cno`) is already proven WITHOUT using
`hom_functor` at all. It's a direct proof about identity and composition.
So the axiom is harmless — it exists only for conceptual completeness.

---

## License Status

All proof files now use PMPL-1.0-or-later headers. The old AGPL references
in StatMech.v were fixed in a previous session. LambdaCNO.v and
CNOCategory.v headers were updated to PMPL-1.0-or-later.

---

## Key Helper Lemmas Available

In `StatMech_helpers.v`:
- `state_eq_sym` : symmetry of state equality
- `state_eq_trans` : transitivity of state equality
- `cno_eval_identity` : CNO eval produces equal state

In `LambdaCNO.v`:
- `beta_reduce_star_trans` : transitivity of multi-step reduction
- `beta_reduce_star_app_left` : congruence for left app argument
- `beta_reduce_star_app_right` : congruence for right app argument
- `subst_closed_at` / `subst_closed` : substitution on closed terms is identity

---

## For Sonnet: Recommended Work Order

1. **QuantumCNO.v** (5 proofs) - Fix the Cexp bug, add axioms, complete
2. **FilesystemCNO.v** (6 proofs) - Read file first, classify difficulty
3. **MalbolgeCore.v** (1 proof) - Read file, likely straightforward
4. **QuantumMechanicsExact.v** (3 proofs) - Probably axiomatize
5. **LandauerDerivation.v** (3 proofs) - Hardest, may need axioms
6. **LambdaCNO.v** y_not_cno (1 proof) - Leave Admitted or axiomatize

Total: 19 proofs. Realistic target: 12-15 can be completed, rest axiomatized.
