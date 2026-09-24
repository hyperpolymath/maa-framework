-- SPDX-License-Identifier: MPL-2.0
-- Derived results for @@PROJECT_NAME@@.
--
-- This module is a *consumer* of the core library: it proves new facts
-- from the exported lemmas, so a regression in the core shows up here as
-- a typecheck failure rather than passing unnoticed.
module Properties where

open import Agda.Builtin.Nat using (Nat; zero; suc; _+_)
open import Agda.Builtin.Equality using (_≡_)

-- Pairs. Agda.Builtin does not export _×_, so declare it here rather than
-- pull in the standard library for one constructor.
data _×_ (A B : Set) : Set where
  _,_ : A → B → A × B

open import @@MOD_CAMEL@@ using (sym; trans; +-identityˡ; +-identityʳ; suc-+)

-- zero is an identity for _+_ on both sides at once.
+-zero-identity : ∀ (n : Nat) → ((n + zero) ≡ n) × ((zero + n) ≡ n)
+-zero-identity n = (+-identityʳ n , +-identityˡ n)

-- Adding zero on the right, twice, is still adding zero once.
+-zero-twice : ∀ (n : Nat) → (n + zero) + zero ≡ n
+-zero-twice n = trans (+-identityʳ (n + zero)) (+-identityʳ n)

-- The general suc-+ lemma, instantiated at m = suc zero.
suc-+-concrete : ∀ (n : Nat) → suc (suc zero + n) ≡ suc zero + suc n
suc-+-concrete n = suc-+ (suc zero) n
