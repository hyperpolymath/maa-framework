/- Statistical Mechanics and Thermodynamics of Computation

   Lean 4 formalization of thermodynamic foundations for CNO theory,
   proving connections to Landauer's Principle and reversible computing.

   Author: Jonathan D. A. Jewell
   Project: Absolute Zero
   License: MPL-2.0
-/

import CNO
import Mathlib.Data.Real.Basic
import Mathlib.Analysis.SpecialFunctions.Log.Basic

namespace StatMech

-- Use ℝ for real numbers
open Real

/-! ## Physical Constants -/

/-- Boltzmann constant (axiomatized as positive real) -/
axiom kB : ℝ
axiom kB_positive : kB > 0

/-- Temperature in Kelvin -/
axiom temperature : ℝ
axiom temperature_positive : temperature > 0

/-! ## Probability Distributions -/

/-- Probability distribution over program states -/
def StateDistribution : Type := CNO.ProgramState → ℝ

/-- Probabilities are non-negative -/
axiom prob_nonneg (P : StateDistribution) (s : CNO.ProgramState) :
  P s ≥ 0

/-- Probabilities are normalized (sum to 1) -/
axiom prob_normalized (P : StateDistribution) :
  ∃ (states : List CNO.ProgramState), states.foldl (fun acc s => acc + P s) 0 = 1

/-- Point distribution (all probability on one state) -/
def pointDist (s0 : CNO.ProgramState) : StateDistribution :=
  fun s => if s == s0 then 1 else 0

/-! ## Information-Theoretic Entropy -/

/-- Shannon entropy: H(P) = -Σ p(s) log₂ p(s)
    Measured in bits -/
axiom shannonEntropy : StateDistribution → ℝ

/-- Shannon entropy is non-negative -/
axiom shannon_entropy_nonneg (P : StateDistribution) :
  shannonEntropy P ≥ 0

/-- Point distributions have zero entropy -/
axiom shannon_entropy_point_zero (s : CNO.ProgramState) :
  shannonEntropy (pointDist s) = 0

/-- Change in entropy.
    `noncomputable` because `shannonEntropy` is an axiom of type ℝ; Lean
    cannot extract executable code for any definition that touches it. -/
noncomputable def entropyChange (P_initial P_final : StateDistribution) : ℝ :=
  shannonEntropy P_final - shannonEntropy P_initial

/-! ## Thermodynamic Entropy -/

/-- Boltzmann entropy: S = kB ln(2) H.
    `noncomputable` — uses `Real.log` (no executable code). -/
noncomputable def boltzmannEntropy (P : StateDistribution) : ℝ :=
  kB * log 2 * shannonEntropy P

/-- Boltzmann entropy is non-negative.

    `kB * log 2 * shannonEntropy P` is a product of three non-negative
    reals: `kB > 0` (axiom), `log 2 > 0` (`Real.log_pos` since 1 < 2),
    `shannonEntropy P ≥ 0` (axiom). -/
theorem boltzmann_entropy_nonneg (P : StateDistribution) :
    boltzmannEntropy P ≥ 0 := by
  unfold boltzmannEntropy
  have h_kB : (0 : ℝ) ≤ kB := le_of_lt kB_positive
  have h_log2 : (0 : ℝ) ≤ Real.log 2 :=
    le_of_lt (Real.log_pos (by norm_num : (1 : ℝ) < 2))
  have h_H : (0 : ℝ) ≤ shannonEntropy P := shannon_entropy_nonneg P
  exact mul_nonneg (mul_nonneg h_kB h_log2) h_H

/-! ## Landauer's Principle -/

/-- Energy dissipated by a computational process (Joules) -/
axiom energyDissipatedPhys : StateDistribution → StateDistribution → ℝ

/-- Landauer's Principle: Erasing information dissipates energy
    E_dissipated ≥ kT ln(2) × (-ΔS) when ΔS < 0 -/
axiom landauer_principle (P_initial P_final : StateDistribution) :
  let ΔS := shannonEntropy P_final - shannonEntropy P_initial
  ΔS < 0 →
  energyDissipatedPhys P_initial P_final ≥ kB * temperature * log 2 * (-ΔS)

/-- Landauer limit (energy per bit erased).
    `noncomputable` — `kB` and `temperature` are real-valued axioms. -/
noncomputable def landauer_limit : ℝ := kB * temperature * log 2

/-! ## CNO Thermodynamics -/

/-- Distribution after program execution -/
axiom postExecutionDist : CNO.Program → StateDistribution → StateDistribution

/-- The mechanism connecting `postExecutionDist` to per-state semantics.
    `postExecutionDist` is an axiom in this model — without an axiom
    that ties it to actual program behaviour, no result of the form
    "running a state-preserving program leaves the distribution alone"
    can be proved. This axiom states the minimum required link:
    a program that pointwise preserves states leaves the distribution
    fixed. -/
axiom postExecutionDist_id_of_state_preserving
  (p : CNO.Program) (P : StateDistribution)
  (h : ∀ s, CNO.ProgramState.eq (CNO.eval p s) s) :
  postExecutionDist p P = P

/-- CNOs preserve Shannon entropy.

    With the `postExecutionDist_id_of_state_preserving` axiom, this is
    a trivial rewrite: a CNO is state-preserving by definition, so the
    distribution is unchanged, so its entropy is unchanged. -/
theorem cno_preserves_shannon_entropy (p : CNO.Program) (P : StateDistribution) :
    CNO.isCNO p →
    shannonEntropy (postExecutionDist p P) = shannonEntropy P := by
  intro h_cno
  rw [postExecutionDist_id_of_state_preserving p P h_cno.2.1]

/-- Corollary: CNOs have zero entropy change -/
theorem cno_zero_entropy_change (p : CNO.Program) (P : StateDistribution) :
    CNO.isCNO p →
    entropyChange P (postExecutionDist p P) = 0 := by
  intro h_cno
  unfold entropyChange
  rw [cno_preserves_shannon_entropy p P h_cno]
  simp

/-- Reversible processes dissipate no energy -/
axiom reversible_zero_dissipation (P_initial P_final : StateDistribution) :
  shannonEntropy P_initial = shannonEntropy P_final →
  energyDissipatedPhys P_initial P_final = 0

/-- Main Theorem: CNOs dissipate zero energy.
    `reversible_zero_dissipation` wants `H P_initial = H P_final`;
    `cno_preserves_shannon_entropy` gives the symmetric direction
    `H (postExecutionDist p P) = H P`, so `.symm` flips it. -/
theorem cno_zero_energy_dissipation (p : CNO.Program) (P : StateDistribution) :
    CNO.isCNO p →
    energyDissipatedPhys P (postExecutionDist p P) = 0 := by
  intro h_cno
  apply reversible_zero_dissipation
  exact (cno_preserves_shannon_entropy p P h_cno).symm

/-! ## Bennett's Reversible Computing -/

/-- A program is logically reversible if it's bijective -/
def logicallyReversible (p : CNO.Program) : Prop :=
  ∃ p_inv : CNO.Program,
    ∀ s s', CNO.eval p s = s' →
      CNO.eval p_inv s' = s

/-- Lift `ProgramState.eq` (componentwise; uses `Memory.eq` pointwise on
    the function field) to propositional equality on `ProgramState`.
    Memory equality requires `funext` (Lean 4 admits it). -/
theorem ProgramState_eq_of_state_eq (s1 s2 : CNO.ProgramState)
    (h : CNO.ProgramState.eq s1 s2) : s1 = s2 := by
  obtain ⟨hmem, hregs, hio, hpc⟩ := h
  have hmem_fn : s1.memory = s2.memory := funext hmem
  cases s1; cases s2
  congr

/-- CNOs are trivially logically reversible.

    From `isCNO p` we have `ProgramState.eq (eval p s) s` for every `s`.
    Lifting to propositional equality (via `funext` on the memory
    function field) gives `eval p s = s`. Then
    `eval p s' = eval p (eval p s) = eval p s = s`. -/
theorem cno_logically_reversible (p : CNO.Program) :
    CNO.isCNO p → logicallyReversible p := by
  intro h_cno
  refine ⟨p, ?_⟩
  intro s s' h_eval
  -- Goal: eval p s' = s
  rw [← h_eval]
  -- Goal: eval p (eval p s) = s
  have h_eq : CNO.eval p s = s :=
    ProgramState_eq_of_state_eq _ _ (h_cno.2.1 s)
  rw [h_eq]
  exact h_eq

/-! ## Thermodynamic Efficiency -/

/-- CNOs achieve maximum thermodynamic efficiency -/
theorem cno_maximum_efficiency (p : CNO.Program) (P : StateDistribution) :
    CNO.isCNO p →
    energyDissipatedPhys P (postExecutionDist p P) = 0 := by
  intro h_cno
  exact cno_zero_energy_dissipation p P h_cno

end StatMech
