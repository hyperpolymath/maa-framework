-- Concrete Echo/CNO instantiation against CNO.Program and CNO.eval.
--
-- Primary bridge: use CNO.state-eq directly as the relation in EchoRel.
-- Secondary bridge: recover propositional equality with extensionality.

module EchoBridgeCNO where

open import Level using (zero)
open import Data.Product using (_,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Axiom.Extensionality.Propositional using (Extensionality)

import CNO
open import EchoBridgeScaffold using
  ( CNOModel
  ; Echo
  ; EchoRel
  ; echo-from-cno
  ; echo-from-rel
  )

echo-from-cno-program-rel :
  (p : CNO.Program) →
  CNO.IsCNO p →
  (s : CNO.ProgramState) →
  EchoRel (CNO.eval p) CNO.state-eq s
echo-from-cno-program-rel p cno s =
  echo-from-rel (CNO.eval p) CNO.state-eq s s
    (CNO.IsCNO.cno-identity cno s)

absolute-zero-echo-rel :
  (s : CNO.ProgramState) →
  EchoRel (CNO.eval CNO.absolute-zero) CNO.state-eq s
absolute-zero-echo-rel s =
  echo-from-cno-program-rel CNO.absolute-zero CNO.absolute-zero-is-cno s

state-eq→≡ :
  Extensionality zero zero →
  ∀ {s₁ s₂ : CNO.ProgramState} →
  CNO.state-eq s₁ s₂ → s₁ ≡ s₂
state-eq→≡ ext {CNO.mk-state m₁ r₁ i₁ pc₁} {CNO.mk-state m₂ r₂ i₂ pc₂}
             (m-eq , r-eq , io-eq , pc-eq)
  rewrite ext m-eq | r-eq | io-eq | pc-eq = refl

program-state-model : Extensionality zero zero → CNOModel CNO.ProgramState
program-state-model ext = record
  { Op = CNO.Program
  ; run = CNO.eval
  ; IsCNO = CNO.IsCNO
  ; cno-identity = λ cno s →
      state-eq→≡ ext (CNO.IsCNO.cno-identity cno s)
  }

echo-from-cno-program-≡ :
  (ext : Extensionality zero zero) →
  (p : CNO.Program) →
  CNO.IsCNO p →
  (s : CNO.ProgramState) →
  Echo (CNO.eval p) s
echo-from-cno-program-≡ ext p cno s =
  echo-from-cno (program-state-model ext) p cno s

absolute-zero-echo-≡ :
  (ext : Extensionality zero zero) →
  (s : CNO.ProgramState) →
  Echo (CNO.eval CNO.absolute-zero) s
absolute-zero-echo-≡ ext s =
  echo-from-cno-program-≡ ext CNO.absolute-zero CNO.absolute-zero-is-cno s
