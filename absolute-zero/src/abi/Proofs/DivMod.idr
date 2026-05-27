||| Div/mod lemma library for ABI alignment proofs.
|||
||| Consolidates the trusted base of number-theoretic lemmas used by ABI
||| layout modules across the hyperpolymath estate (absolute-zero,
||| civic-connect, and any future Idris2 ABI consumers).
|||
||| Design goals:
|||   * Single shared module — each estate repo imports the same lemmas
|||     rather than re-postulating per file.
|||   * Each lemma is an individually-named declaration so it can be
|||     discharged incrementally (one proof per audit pass) without
|||     touching consumers.
|||   * Definitions of the functions the lemmas talk about live here too,
|||     so the lemma statements don't drift from their referent.
|||
||| Discharge tracker: absolute-zero#27.
|||
||| Notes on Idris2 0.8.0:
|||   * The `postulate` keyword used in older Idris/Agda style code does
|||     not parse in current Idris2. The canonical axiom idiom is
|||     `name = believe_me ()` — semantically equivalent (asserts a term
|||     of the target type with no proof) but explicit at the term level,
|||     so every axiom is grep-able as a `believe_me` occurrence in the
|||     trusted base.
|||
||| @see https://github.com/hyperpolymath/absolute-zero/issues/27

-- SPDX-License-Identifier: MPL-2.0

module AbsoluteZero.ABI.Proofs.DivMod

import Data.So
import Data.Nat

%default total

--------------------------------------------------------------------------------
-- Aligned size
--------------------------------------------------------------------------------

||| Round `size` up to the next multiple of `align`.
||| If `size` is already aligned, returns `size` unchanged.
public export
alignedSize : (size : Nat) -> (align : Nat) -> Nat
alignedSize size align =
  let remainder = size `mod` align
  in if remainder == 0
     then size
     else size + (align `minus` remainder)

--------------------------------------------------------------------------------
-- Trusted lemma surface
--
-- Each `believe_me`-based axiom below is an individually-audit-trackable
-- item. Discharge one at a time by replacing the RHS with a real proof;
-- the lemma name and type signature stay stable so consumers don't break.
--
-- Estate cross-reference (as of 2026-05-20):
--   * civic-connect/src/Abi/Layout.idr defers the same family under
--     `alignUpDivides`, `mkFieldsAligned`, `offsetInBoundsPrf`. Those
--     should migrate to import these names.
--
-- Discharge path to stdlib:
--   * `divModIdentity` is provable from
--     `Data.Nat.Division.DivisionTheorem` (idris2-contrib): converts
--     `So (d /= 0)` to `NonZero d`, then rewrites between
--     `mod`/`div` (Prelude binary ops) and `modNatNZ`/`divNatNZ`.
--   * `multModZero` follows by induction on `k`.
--   * `addModDistrib` is in `Data.Nat.Equational` territory.
--   * `alignedSizeCorrect` then chains them.
--------------------------------------------------------------------------------

||| `alignedSize size align` is always a multiple of `align`.
|||
||| Proof outline (currently asserted; see discharge path above):
|||   Let r = size `mod` align.
|||   Case r == 0: alignedSize = size, divisible by hypothesis.
|||   Case r /= 0: alignedSize = size + (align - r)
|||                            = ((size `div` align) * align + r) + (align - r)
|||                              [by divModIdentity]
|||                            = ((size `div` align) + 1) * align
|||                              [by Nat ring rewriting]
|||                            and (k * align) `mod` align = 0
|||                              [by multModZero].
export
alignedSizeCorrect :
  (size : Nat) -> (align : Nat) ->
  {auto 0 nonZero : So (align /= 0)} ->
  So (alignedSize size align `mod` align == 0)
alignedSizeCorrect _ _ = believe_me ()

||| Euclidean division identity: every Nat decomposes as q*d + r.
||| Provable from `Data.Nat.Division.DivisionTheorem` — the conversion
||| between `So (d /= 0)` and `NonZero d`, plus the rewrite between
||| Prelude `mod`/`div` and `modNatNZ`/`divNatNZ`, is the only real work.
export
divModIdentity :
  (n : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  n = (n `div` d) * d + (n `mod` d)
divModIdentity _ _ = believe_me ()

||| Any multiple of `d` is congruent to zero mod `d`.
||| Provable by induction on `k`.
export
multModZero :
  (k : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  So ((k * d) `mod` d == 0)
multModZero _ _ = believe_me ()

||| Mod distributes over addition (in the sense that `(a + b) mod d` is
||| determined by `a mod d` and `b mod d`).
export
addModDistrib :
  (a : Nat) -> (b : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  (a + b) `mod` d = ((a `mod` d) + (b `mod` d)) `mod` d
addModDistrib _ _ _ = believe_me ()
