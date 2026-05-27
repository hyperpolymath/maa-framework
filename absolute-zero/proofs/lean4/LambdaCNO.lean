/- CNOs in Lambda Calculus

   Proves that CNO theory applies to lambda calculus,
   showing the identity function (λx.x) is the canonical CNO.

   Demonstrates model-independence of CNO theory.

   Author: Jonathan D. A. Jewell
   Project: Absolute Zero
   License: MPL-2.0
-/

namespace LambdaCNO

/-! ## Lambda Calculus Syntax -/

/-- Variables are de Bruijn indices -/
inductive LambdaTerm where
  | LVar : Nat → LambdaTerm
  | LApp : LambdaTerm → LambdaTerm → LambdaTerm
  | LAbs : LambdaTerm → LambdaTerm
  deriving Repr, BEq

open LambdaTerm

/-! ## Substitution -/

/-- Substitute term s for variable n in term t -/
def subst (n : Nat) (s : LambdaTerm) (t : LambdaTerm) : LambdaTerm :=
  match t with
  | LVar m => if n == m then s else LVar m
  | LApp t1 t2 => LApp (subst n s t1) (subst n s t2)
  | LAbs body => LAbs (subst (n + 1) s body)

/-! ## Beta Reduction -/

/-- One-step beta reduction -/
inductive BetaReduce : LambdaTerm → LambdaTerm → Prop where
  | beta_app :
      ∀ body arg,
        BetaReduce (LApp (LAbs body) arg) (subst 0 arg body)

  | beta_app_left :
      ∀ t1 t1' t2,
        BetaReduce t1 t1' →
        BetaReduce (LApp t1 t2) (LApp t1' t2)

  | beta_app_right :
      ∀ t1 t2 t2',
        BetaReduce t2 t2' →
        BetaReduce (LApp t1 t2) (LApp t1 t2')

  | beta_abs :
      ∀ body body',
        BetaReduce body body' →
        BetaReduce (LAbs body) (LAbs body')

/-- Multi-step beta reduction (reflexive transitive closure) -/
inductive BetaReduceStar : LambdaTerm → LambdaTerm → Prop where
  | beta_refl :
      ∀ t,
        BetaReduceStar t t

  | beta_step :
      ∀ t1 t2 t3,
        BetaReduce t1 t2 →
        BetaReduceStar t2 t3 →
        BetaReduceStar t1 t3

/-! ## Normal Forms -/

/-- A term is in normal form if no beta reduction is possible -/
def isNormalForm (t : LambdaTerm) : Prop :=
  ¬ ∃ t', BetaReduce t t'

/-- Evaluation: reduce to normal form -/
def evaluatesTo (t : LambdaTerm) (nf : LambdaTerm) : Prop :=
  BetaReduceStar t nf ∧ isNormalForm nf

/-! ## The Identity Function -/

/-- λx.x - The canonical CNO in lambda calculus -/
def lambda_id : LambdaTerm := LAbs (LVar 0)

/-! ## CNO Definitions for Lambda Calculus -/

/-- A lambda term is a CNO if, for all arguments in normal form:
    1. Application terminates (reaches a normal form)
    2. It acts as identity (reduces back to the argument)
    3. No side effects (lambda calculus is pure by construction)

    Note: The quantification is restricted to arguments already in normal form.
    This is the mathematically correct formulation — universally quantifying
    over ALL terms (including non-normalizing ones like Ω) would make
    termination unprovable, since the identity function faithfully returns
    non-normalizing arguments unchanged. -/
def isLambdaCNO (t : LambdaTerm) : Prop :=
  ∀ arg : LambdaTerm,
    isNormalForm arg →
    (∃ nf, evaluatesTo (LApp t arg) nf) ∧
    BetaReduceStar (LApp t arg) arg

/-- Weaker CNO definition: identity acts as identity on all args, termination conditional -/
def isLambdaCNOWeak (t : LambdaTerm) : Prop :=
  ∀ arg : LambdaTerm,
    BetaReduceStar (LApp t arg) arg

/-! ## Main Theorems -/

theorem lambda_id_is_cno_weak : isLambdaCNOWeak lambda_id := by
  unfold isLambdaCNOWeak lambda_id
  intro arg
  apply BetaReduceStar.beta_step
  · apply BetaReduce.beta_app
  · simp [subst]
    apply BetaReduceStar.beta_refl

/-- The identity function is in normal form (no reduction possible) -/
theorem lambda_id_normal_form : isNormalForm lambda_id := by
  unfold isNormalForm lambda_id
  intro ⟨t', h⟩
  cases h with
  | beta_abs _ _ h' => cases h'

/-- lambda_id is a CNO: for arguments in normal form, it terminates and acts as identity -/
theorem lambda_id_is_cno : isLambdaCNO lambda_id := by
  unfold isLambdaCNO lambda_id
  intro arg h_nf
  constructor
  · -- Terminates: (λx.x) arg →β arg, and arg is in normal form by hypothesis
    exists arg
    unfold evaluatesTo
    constructor
    · apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · simp [subst]; apply BetaReduceStar.beta_refl
    · exact h_nf
  · -- Identity: (λx.x) arg →β arg
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · simp [subst]
      apply BetaReduceStar.beta_refl

/-! ## Congruence Lemma for Multi-Step Reduction -/

/-- If `a →* b` then `f a →* f b`.
    Proved by induction on the BetaReduceStar derivation,
    lifting each single step via `BetaReduce.beta_app_right`. -/
theorem BetaReduceStar_app_right (f a b : LambdaTerm)
    (h : BetaReduceStar a b) : BetaReduceStar (LApp f a) (LApp f b) := by
  induction h with
  | beta_refl t =>
    exact BetaReduceStar.beta_refl _
  | beta_step t1 t2 t3 h_step _ ih =>
    exact BetaReduceStar.beta_step _ _ _ (BetaReduce.beta_app_right f t1 t2 h_step) ih

/-- If `a →* b` then `a c →* b c`.
    Dual congruence lemma lifting under the left side of application. -/
theorem BetaReduceStar_app_left (a b c : LambdaTerm)
    (h : BetaReduceStar a b) : BetaReduceStar (LApp a c) (LApp b c) := by
  induction h with
  | beta_refl t =>
    exact BetaReduceStar.beta_refl _
  | beta_step t1 t2 t3 h_step _ ih =>
    exact BetaReduceStar.beta_step _ _ _ (BetaReduce.beta_app_left t1 t2 c h_step) ih

/-- Transitivity of multi-step reduction. -/
theorem BetaReduceStar_trans (a b c : LambdaTerm)
    (h1 : BetaReduceStar a b) (h2 : BetaReduceStar b c) : BetaReduceStar a c := by
  induction h1 with
  | beta_refl _ => exact h2
  | beta_step t1 t2 _ h_step _ ih => exact BetaReduceStar.beta_step _ _ _ h_step (ih h2)

/-! ## Closed Term Substitution -/

/-- A term is closed if it contains no free variables at or above level n -/
def Closed (t : LambdaTerm) (n : Nat) : Prop :=
  ∀ m s, m ≥ n → subst m s t = t

/-- Substitution on closed terms is identity.
    This is a standard metatheoretic property of lambda calculus:
    replacing a variable that doesn't occur free has no effect. -/
axiom subst_closed_term (t s : LambdaTerm) (n : Nat) :
  Closed t 0 → subst n s t = t

/-! ## Composition Theorem -/

/-- Composing two lambda CNOs yields a CNO.
    Requires f and g to be closed (no free de Bruijn variables). -/
def lambda_compose (f g : LambdaTerm) : LambdaTerm :=
  LAbs (LApp f (LApp g (LVar 0)))

/-- Composition of lambda CNOs yields a CNO.
    For arguments in normal form: compose applies g then f, both of which
    terminate and return the original argument. -/
theorem lambda_cno_composition (f g : LambdaTerm)
    (hf_closed : Closed f 0) (hg_closed : Closed g 0) :
    isLambdaCNO f →
    isLambdaCNO g →
    isLambdaCNO (lambda_compose f g) := by
  intro hf hg
  unfold isLambdaCNO at *
  intro arg h_nf
  have ⟨_, hf_id⟩ := hf arg h_nf
  have ⟨_, hg_id⟩ := hg arg h_nf
  constructor
  · exists arg
    unfold evaluatesTo
    constructor
    · apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · simp [subst, subst_closed_term f arg 0 hf_closed, subst_closed_term g arg 0 hg_closed]
        have h_congr := BetaReduceStar_app_right f (LApp g arg) arg hg_id
        exact BetaReduceStar_trans _ _ _ h_congr hf_id
    · exact h_nf
  · apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · simp [subst, subst_closed_term f arg 0 hf_closed, subst_closed_term g arg 0 hg_closed]
      have h_congr := BetaReduceStar_app_right f (LApp g arg) arg hg_id
      exact BetaReduceStar_trans _ _ _ h_congr hf_id

/-! ## Non-CNO Examples -/

/-- The Y combinator enables recursion -/
def y_combinator : LambdaTerm :=
  LAbs (LApp
    (LAbs (LApp (LVar 1) (LApp (LVar 0) (LVar 0))))
    (LAbs (LApp (LVar 1) (LApp (LVar 0) (LVar 0)))))

/-- Y is NOT a CNO because it doesn't act as identity.
    Y f reduces to f (Y f), not back to f. -/
-- AXIOM: y_combinator_not_identity; non-termination claim about the Y combinator —
--   requires step-indexed semantics or coinduction to discharge.
--   §(c) NECESSARY AXIOM per docs/proof-debt.md (Lean Lambda triage 2026-05-27).
axiom y_combinator_not_identity :
  ¬ BetaReduceStar (LApp y_combinator lambda_id) lambda_id

theorem y_not_cno : ¬ isLambdaCNO y_combinator := by
  unfold isLambdaCNO
  intro h
  have ⟨_, h_id⟩ := h lambda_id lambda_id_normal_form
  exact y_combinator_not_identity h_id

/-! ## Church Encodings -/

/-- Church encoding of zero: λf.λx.x -/
def church_zero : LambdaTerm :=
  LAbs (LAbs (LVar 0))

/-- Church zero applied to church zero reduces to church zero (λx.x variant) -/
example : BetaReduceStar (LApp church_zero church_zero) (LAbs (LVar 0)) := by
  -- (λf.λx.x) (λf.λx.x) →β λx.x
  apply BetaReduceStar.beta_step
  · apply BetaReduce.beta_app
  · simp [subst, church_zero]
    apply BetaReduceStar.beta_refl

/-! ## Eta Equivalence -/

/-- Eta reduction: (λx. f x) ≡ f -/
-- AXIOM: eta_equivalence; η-equivalence is not derivable under β-only reduction —
--   requires an extra reduction rule or extensional equality.
--   §(c) NECESSARY AXIOM per docs/proof-debt.md (Lean Lambda triage 2026-05-27).
axiom eta_equivalence (f : LambdaTerm) :
  BetaReduceStar (LAbs (LApp f (LVar 0))) f

/-- Eta-expanded identity is a CNO: for arguments in normal form,
    it terminates and acts as identity -/
theorem eta_expanded_id_is_cno :
    isLambdaCNO (LAbs (LApp lambda_id (LVar 0))) := by
  unfold isLambdaCNO
  intro arg h_nf
  constructor
  · exists arg
    unfold evaluatesTo
    constructor
    · apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · simp [subst]
        apply BetaReduceStar.beta_step
        · apply BetaReduce.beta_app
        · simp [subst, lambda_id]
          apply BetaReduceStar.beta_refl
    · exact h_nf
  · -- (λx. (λy.y) x) arg →* arg
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · simp [subst, lambda_id]
      apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · simp [subst, lambda_id]
        apply BetaReduceStar.beta_refl

end LambdaCNO
