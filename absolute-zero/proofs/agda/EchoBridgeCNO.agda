-- Concrete Echo/CNO instantiation against CNO.Program and CNO.eval.
--
-- Note: CNO identity is phrased as state-eq, so we parameterize by
-- function extensionality to recover propositional equality of states.

module EchoBridgeCNO where

open import Level using (zero)
open import Data.Product using (_,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Axiom.Extensionality.Propositional using (Extensionality)

import CNO
open import EchoBridgeScaffold using (CNOModel; Echo; echo-from-cno)

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

echo-from-cno-program :
  (ext : Extensionality zero zero) →
  (p : CNO.Program) →
  CNO.IsCNO p →
  (s : CNO.ProgramState) →
  Echo (CNO.eval p) s
echo-from-cno-program ext p cno s =
  echo-from-cno (program-state-model ext) p cno s

absolute-zero-echo :
  (ext : Extensionality zero zero) →
  (s : CNO.ProgramState) →
  Echo (CNO.eval CNO.absolute-zero) s
absolute-zero-echo ext s =
  echo-from-cno-program ext CNO.absolute-zero CNO.absolute-zero-is-cno s
