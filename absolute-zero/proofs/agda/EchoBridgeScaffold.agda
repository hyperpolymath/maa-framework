{-# OPTIONS --safe --without-K #-}

-- Echo/CNO Agda bridge scaffold.
--
-- This module is intentionally independent from `CNO.agda` so it can
-- stay typecheckable while that file is still being completed.

module EchoBridgeScaffold where

open import Agda.Primitive using (Level; lsuc; _⊔_)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Sigma using (Σ; _,_)
open import Agda.Builtin.Unit using (⊤; tt)

-- Canonical echo/fiber shape used by echo-types.
Echo : ∀ {a b} {A : Set a} {B : Set b} → (A → B) → B → Set (a ⊔ b)
Echo {A = A} f y = Σ A (λ x → f x ≡ y)

-- Relation-indexed fiber shape, useful when identity is stated up to
-- a semantic relation instead of propositional equality.
EchoRel :
  ∀ {a b r} {A : Set a} {B : Set b} →
  (A → B) → (B → B → Set r) → B → Set (a ⊔ r)
EchoRel {A = A} f _≈_ y = Σ A (λ x → _≈_ (f x) y)

echo-from-rel :
  ∀ {a b r} {A : Set a} {B : Set b}
  (f : A → B) (_≈_ : B → B → Set r)
  (x : A) (y : B) →
  _≈_ (f x) y →
  EchoRel f _≈_ y
echo-from-rel _ _ x _ rel = x , rel

-- Minimal interface needed to connect a CNO model to echoes.
record CNOModel {ℓs ℓo : Level} (State : Set ℓs) : Set (lsuc (ℓs ⊔ ℓo)) where
  field
    Op           : Set ℓo
    run          : Op → State → State
    IsCNO        : Op → Set (ℓs ⊔ ℓo)
    cno-identity : ∀ {op} → IsCNO op → (s : State) → run op s ≡ s

open CNOModel public

-- Any CNO witness yields an echo at any visible state.
echo-from-cno :
  ∀ {ℓs ℓo} {State : Set ℓs}
  (M : CNOModel {ℓs} {ℓo} State)
  (op : Op M) → IsCNO M op →
  (s : State) → Echo (run M op) s
echo-from-cno M op cno s = s , cno-identity M cno s

-- A tiny closed model kept here as a local smoke witness.
TrivialModel : CNOModel ⊤
TrivialModel = record
  { Op = ⊤
  ; run = λ _ s → s
  ; IsCNO = λ _ → ⊤
  ; cno-identity = λ _ _ → refl
  }

trivial-echo : Echo (run TrivialModel tt) tt
trivial-echo = echo-from-cno TrivialModel tt tt tt
