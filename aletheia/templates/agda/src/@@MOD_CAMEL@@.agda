-- SPDX-License-Identifier: MPL-2.0
-- Core proofs for @@PROJECT_NAME@@.
--
-- Self-contained by design: only Agda.Builtin is imported, so there is no
-- standard-library dependency to fetch. Replace these with your own
-- definitions and theorems.
module @@MOD_CAMEL@@ where

open import Agda.Builtin.Nat using (Nat; zero; suc; _+_)
open import Agda.Builtin.Equality using (_≡_; refl)

------------------------------------------------------------------------
-- Equality helpers
--
-- Agda.Builtin.Equality ships only _≡_ and refl, and we do not want a
-- stdlib dependency just for these two. Every argument is annotated with
-- its type: `m ≡ n` with untyped `m`/`n` leaves the type index of _≡_
-- ambiguous, and Agda reports unsolved metas rather than guessing.
------------------------------------------------------------------------

sym : ∀ {m n : Nat} → m ≡ n → n ≡ m
sym refl = refl

trans : ∀ {l m n : Nat} → l ≡ m → m ≡ n → l ≡ n
trans refl refl = refl

cong-suc : ∀ {m n : Nat} → m ≡ n → suc m ≡ suc n
cong-suc refl = refl

------------------------------------------------------------------------
-- Theorems about addition
------------------------------------------------------------------------

-- zero is a left identity for _+_. Holds by computation — no induction
-- needed — so `refl` is already a complete proof.
+-identityˡ : ∀ (n : Nat) → zero + n ≡ n
+-identityˡ n = refl

-- suc commutes with _+_ on the right. This needs induction, because _+_
-- recurses on its *first* argument.
suc-+ : ∀ (m n : Nat) → suc (m + n) ≡ m + suc n
suc-+ zero    n = refl
suc-+ (suc m) n = cong-suc (suc-+ m n)

-- zero is a right identity for _+_. Also inductive, for the same reason.
+-identityʳ : ∀ (n : Nat) → n + zero ≡ n
+-identityʳ zero    = refl
+-identityʳ (suc n) = cong-suc (+-identityʳ n)

-- A worked use of all three: zero commutes through addition.
+-zero-comm : ∀ (n : Nat) → n + zero ≡ zero + n
+-zero-comm n = trans (+-identityʳ n) (sym (+-identityˡ n))
