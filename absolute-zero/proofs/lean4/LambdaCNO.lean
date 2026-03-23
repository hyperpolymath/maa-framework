/- CNOs in Lambda Calculus

   Proves that CNO theory applies to lambda calculus,
   showing the identity function (λx.x) is the canonical CNO.

   Demonstrates model-independence of CNO theory.

   Author: Jonathan D. A. Jewell
   Project: Absolute Zero
   License: AGPL-3.0 / Palimpsest 0.5
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

/-! ## CNO Definition for Lambda Calculus -/

/-- A lambda term is a CNO if:
    1. It terminates (reaches a normal form)
    2. It acts as identity (for all arguments)
    3. No side effects (lambda calculus is pure by construction)
-/
def isLambdaCNO (t : LambdaTerm) : Prop :=
  ∀ arg : LambdaTerm,
    (∃ nf, evaluatesTo (LApp t arg) nf) ∧
    BetaReduceStar (LApp t arg) arg

/-! ## Main Theorem: Identity is a CNO -/

/-- Weaker CNO definition: identity acts as identity on all args, termination conditional -/
def isLambdaCNOWeak (t : LambdaTerm) : Prop :=
  ∀ arg : LambdaTerm,
    BetaReduceStar (LApp t arg) arg

theorem lambda_id_is_cno_weak : isLambdaCNOWeak lambda_id := by
  unfold isLambdaCNOWeak lambda_id
  intro arg
  apply BetaReduceStar.beta_step
  · apply BetaReduce.beta_app
  · unfold subst
    simp
    apply BetaReduceStar.beta_refl

/-- lambda_id is a CNO for arguments already in normal form -/
theorem lambda_id_is_cno_on_values : isLambdaCNO lambda_id := by
  unfold isLambdaCNO lambda_id
  intro arg
  constructor
  · -- Terminates: exists arg as normal form
    -- NOTE: This requires arg to be in normal form. The statement isLambdaCNO
    -- quantifies over ALL args including non-normalizing ones, making full
    -- termination unprovable without restricting to values.
    -- We use the identity reduction and leave the normal form condition as
    -- an axiom since lambda_id doesn't introduce non-termination.
    exists arg
    unfold evaluatesTo
    constructor
    · apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · unfold subst; simp; apply BetaReduceStar.beta_refl
    · -- DESIGN_FLAW: isNormalForm is unprovable for arbitrary arg.
      -- isLambdaCNO quantifies over ALL lambda terms, including divergent
      -- ones like Ω = (λx.xx)(λx.xx). The identity function faithfully
      -- returns arg unchanged, so if arg is not in normal form, neither
      -- is the result. The fix would be to restrict isLambdaCNO to args
      -- that are already values (in normal form), or to split termination
      -- from identity behavior. See isLambdaCNOWeak for the correct approach.
      sorry  -- DESIGN_FLAW: isNormalForm demanded for arbitrary args

  · -- Identity
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · unfold subst
      simp
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

/-! ## Composition Theorem -/

/-- Composing two lambda CNOs yields a CNO -/
def lambda_compose (f g : LambdaTerm) : LambdaTerm :=
  LAbs (LApp f (LApp g (LVar 0)))

/-- Composition of weak lambda CNOs yields a weak CNO -/
theorem lambda_cno_composition_weak (f g : LambdaTerm) :
    isLambdaCNOWeak f →
    isLambdaCNOWeak g →
    isLambdaCNOWeak (lambda_compose f g) := by
  intro hf hg
  unfold isLambdaCNOWeak at *
  intro arg
  -- (λx. f (g x)) arg →β f (g arg) →* f arg →* arg
  apply BetaReduceStar.beta_step
  · apply BetaReduce.beta_app
  · unfold subst lambda_compose
    simp
    -- After beta: f (g arg)
    -- g arg →* arg (by hg), so f (g arg) →* f arg →* arg
    -- Use BetaReduceStar_app_right to lift (g arg →* arg) under f,
    -- then transitivity with (f arg →* arg).
    have hg_arg := hg arg
    have h_congr := BetaReduceStar_app_right f (LApp g arg) arg hg_arg
    have hf_arg := hf arg
    exact BetaReduceStar_trans _ _ _ h_congr hf_arg

theorem lambda_cno_composition (f g : LambdaTerm) :
    isLambdaCNO f →
    isLambdaCNO g →
    isLambdaCNO (lambda_compose f g) := by
  intro hf hg
  unfold isLambdaCNO at *
  intro arg
  constructor
  · -- Terminates: isLambdaCNO demands (∃ nf, evaluatesTo ... nf) which requires
    -- isNormalForm on the result. Since isLambdaCNO quantifies over ALL args
    -- (including non-normalizing ones like Ω), the identity function faithfully
    -- returns the non-normalizing arg, making isNormalForm unprovable.
    -- DESIGN_FLAW: isNormalForm demanded for arbitrary args; would need to
    -- restrict isLambdaCNO to args already in normal form (values).
    sorry  -- DESIGN_FLAW: isNormalForm unprovable for arbitrary args
  · -- Identity: ((λx. f (g x)) arg) →* arg
    -- Beta-reduce the outer lambda: (λx. f (g x)) arg →β f (g arg)
    -- Then: g arg →* arg (by hg), f (g arg) →* f arg (by congruence),
    -- and f arg →* arg (by hf). Chain by transitivity.
    have ⟨_, hf_id⟩ := hf arg
    have ⟨_, hg_id⟩ := hg arg
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · unfold subst lambda_compose
      simp
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
axiom y_combinator_not_identity :
  ¬ BetaReduceStar (LApp y_combinator lambda_id) lambda_id

theorem y_not_cno : ¬ isLambdaCNO y_combinator := by
  unfold isLambdaCNO
  intro h
  have ⟨_, h_id⟩ := h lambda_id
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
  · unfold subst church_zero
    simp
    apply BetaReduceStar.beta_refl

/-! ## Eta Equivalence -/

/-- Eta reduction: (λx. f x) ≡ f -/
axiom eta_equivalence (f : LambdaTerm) :
  BetaReduceStar (LAbs (LApp f (LVar 0))) f

/-- Eta-expanded identity acts as identity (weak version) -/
theorem eta_expanded_id_is_cno_weak :
    isLambdaCNOWeak (LAbs (LApp lambda_id (LVar 0))) := by
  unfold isLambdaCNOWeak
  intro arg
  -- (λx. (λy.y) x) arg →β (λy.y) arg →β arg
  apply BetaReduceStar.beta_step
  · apply BetaReduce.beta_app
  · unfold subst
    simp
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · unfold subst lambda_id
      simp
      apply BetaReduceStar.beta_refl

/-- Eta-expanded identity is a CNO (full version, same normal form caveat) -/
theorem eta_expanded_id_is_cno :
    isLambdaCNO (LAbs (LApp lambda_id (LVar 0))) := by
  unfold isLambdaCNO
  intro arg
  constructor
  · exists arg
    unfold evaluatesTo
    constructor
    · apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · unfold subst; simp
        apply BetaReduceStar.beta_step
        · apply BetaReduce.beta_app
        · unfold subst lambda_id; simp
          apply BetaReduceStar.beta_refl
    · -- DESIGN_FLAW: same issue as lambda_id_is_cno_on_values — isNormalForm
      -- demanded for arbitrary args. The eta-expanded identity reduces arg
      -- faithfully, but if arg is not in normal form, neither is the result.
      sorry  -- DESIGN_FLAW: isNormalForm demanded for arbitrary args
  · -- (λx. (λy.y) x) arg →* arg
    apply BetaReduceStar.beta_step
    · apply BetaReduce.beta_app
    · unfold subst
      simp
      apply BetaReduceStar.beta_step
      · apply BetaReduce.beta_app
      · unfold subst lambda_id
        simp
        apply BetaReduceStar.beta_refl

end LambdaCNO
