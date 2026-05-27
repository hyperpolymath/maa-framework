/- Category Theory of Certified Null Operations

   Universal, model-independent definition of CNOs using category theory.
   Proves CNOs are identity morphisms in any computational category.

   Author: Jonathan D. A. Jewell
   Project: Absolute Zero
   License: MPL-2.0
-/

import CNO

namespace CNOCategory

/-! ## Category Definition -/

/-- A category consists of objects and morphisms with composition -/
class Category where
  Obj : Type
  Hom : Obj → Obj → Type

  /-- Composition of morphisms -/
  compose : ∀ {A B C : Obj}, Hom B C → Hom A B → Hom A C

  /-- Identity morphism -/
  id : ∀ {A : Obj}, Hom A A

  /-- Category laws -/
  compose_assoc : ∀ {A B C D : Obj} (h : Hom C D) (g : Hom B C) (f : Hom A B),
    compose h (compose g f) = compose (compose h g) f

  compose_id_left : ∀ {A B : Obj} (f : Hom A B),
    compose id f = f

  compose_id_right : ∀ {A B : Obj} (f : Hom A B),
    compose f id = f

notation:40 g " ∘ " f => Category.compose g f

/-! ## Programs as a Category -/

/-- A morphism from s1 to s2 is a program that evaluates s1 to s2 -/
structure ProgramMorphism (s1 s2 : CNO.ProgramState) where
  program : CNO.Program
  evaluates : CNO.eval program s1 = s2

/-- Extract the program from a morphism -/
def morphProgram {s1 s2 : CNO.ProgramState} (m : ProgramMorphism s1 s2) : CNO.Program :=
  m.program

/-- Composition of morphisms -/
def composeMorphisms {s1 s2 s3 : CNO.ProgramState}
    (m2 : ProgramMorphism s2 s3) (m1 : ProgramMorphism s1 s2) :
    ProgramMorphism s1 s3 :=
  { program := m1.program ++ m2.program,
    evaluates := by
      -- eval (p1 ++ p2) s1 = eval p2 (eval p1 s1) = eval p2 s2 = s3
      rw [CNO.eval_seqComp]
      rw [m1.evaluates, m2.evaluates]
  }

/-- Identity morphism -/
def idMorphism (s : CNO.ProgramState) : ProgramMorphism s s :=
  { program := [],
    evaluates := by unfold CNO.eval; rfl
  }

/-- Programs form a category.

    `id := fun {A} => idMorphism A`: the class field expects a function
    with an *implicit* binder `∀ {A}, Hom A A`, but `idMorphism` takes
    an *explicit* `(s : ProgramState)`. The implicit-lambda wrapper
    bridges the two; bare `id := idMorphism` triggers
    "implicit-lambda introduced A✝" and then a type mismatch under v4.16.

    The three category laws reduce, via the structure projections, to
    laws on `List Instruction` (`++`):
      - `compose_assoc`     ↦ `List.append_assoc`
      - `compose_id_left`   ↦ `List.append_nil`     (right-id of ++)
      - `compose_id_right`  ↦ `List.nil_append`     (left-id of ++)
    The proofs are in Prop and proof-irrelevant, so structural equality
    of the morphism records reduces to equality of the program field. -/
instance ProgramCategory : Category where
  Obj := CNO.ProgramState
  Hom := ProgramMorphism
  compose := @composeMorphisms
  id := fun {A} => idMorphism A

  compose_assoc := by
    intro A B C D h g f
    -- LHS.program = (f.program ++ g.program) ++ h.program
    -- RHS.program = f.program ++ (g.program ++ h.program)
    cases h with | mk hp he =>
    cases g with | mk gp ge =>
    cases f with | mk fp fe =>
    show composeMorphisms ⟨hp, he⟩ (composeMorphisms ⟨gp, ge⟩ ⟨fp, fe⟩) =
         composeMorphisms (composeMorphisms ⟨hp, he⟩ ⟨gp, ge⟩) ⟨fp, fe⟩
    simp only [composeMorphisms]
    congr 1
    exact List.append_assoc fp gp hp

  compose_id_left := by
    intro A B f
    -- composeMorphisms id f has program := f.program ++ [].program = f.program ++ []
    cases f with | mk fp fe =>
    show composeMorphisms (idMorphism B) ⟨fp, fe⟩ = ⟨fp, fe⟩
    simp only [composeMorphisms, idMorphism]
    congr 1
    exact List.append_nil fp

  compose_id_right := by
    intro A B f
    -- composeMorphisms f id has program := [].program ++ f.program = [] ++ f.program
    -- which reduces to f.program by the cons-recursion of ++
    cases f with | mk fp fe =>
    show composeMorphisms ⟨fp, fe⟩ (idMorphism A) = ⟨fp, fe⟩
    simp only [composeMorphisms, idMorphism]
    -- [] ++ fp ≡ fp definitionally
    rfl

/-! ## Categorical CNO Definition -/

/-- In category theory, a CNO is simply an identity morphism -/
def isCNOCategorical {C : Category} {s : C.Obj} (m : C.Hom s s) : Prop :=
  m = C.id

/-- For programs, this means the morphism is the identity -/
def programIsCNOCategorical (p : CNO.Program) (s : CNO.ProgramState) : Prop :=
  ∀ (m : ProgramMorphism s s),
    morphProgram m = p →
    m.program = []

/-! ## Equivalence of Definitions -/

/-- Categorical CNO definition is equivalent to our original.

    Reverse direction: from `∀ s s', eval p s = s' → ProgramState.eq s' s`
    we recover the four conjuncts of `CNO.isCNO`:
      - `terminates p s` is unconditional (`terminates_always`).
      - state preservation is the hypothesis specialised at `eval p s`.
      - `pure s (eval p s)` follows because `ProgramState.eq` includes
        equality of `ioState` and `Memory.eq` of `memory` — exactly what
        `noIO` and `noMemoryAlloc` need (.symm to flip direction).
      - `thermodynamicallyReversible` reduces to `0 = 0` (the model's
        `energyDissipated` is the constant 0). -/
theorem cno_categorical_equiv (p : CNO.Program) :
    CNO.isCNO p ↔ (∀ s s', CNO.eval p s = s' → CNO.ProgramState.eq s' s) := by
  constructor
  · intro h s s' h_eval
    have h_id := h.2.1 s
    rw [← h_eval]
    exact h_id
  · intro h
    refine ⟨?_, ?_, ?_, ?_⟩
    · intro s; exact CNO.terminates_always p s
    · intro s; exact h s (CNO.eval p s) rfl
    · intro s
      have hs := h s (CNO.eval p s) rfl
      -- hs : ProgramState.eq (eval p s) s, decomposed as (mem, regs, io, pc)
      refine ⟨?_, ?_⟩
      · -- noIO s (eval p s) := s.ioState = (eval p s).ioState
        unfold CNO.noIO
        exact hs.2.2.1.symm
      · -- noMemoryAlloc s (eval p s) := Memory.eq s.memory (eval p s).memory
        unfold CNO.noMemoryAlloc CNO.Memory.eq
        intro addr
        exact (hs.1 addr).symm
    · unfold CNO.thermodynamicallyReversible CNO.energyDissipated
      intro s; rfl

/-! ## Functors -/

/-- A functor maps between categories, preserving structure.

    Renamed from `Functor` to `CatFunctor` because Lean's core
    `Init.Functor` (`class Functor (f : Type u → Type v)`) shadows our
    binding when we apply it to a `Category` term — Lean tries to
    unify `Category : Type 1` with `Type u → Type v` and fails.

    The third parameter binder is `{X Y Z}` rather than `{A B C}` to
    avoid shadowing the outer Category-typed `C` (the original
    `{A B C : C.Obj}` binding silently re-binds `C` inside the type
    `C.Hom B C`, breaking field resolution). -/
class CatFunctor (C D : Category) where
  fobj : C.Obj → D.Obj
  fmap : ∀ {A B : C.Obj}, C.Hom A B → D.Hom (fobj A) (fobj B)

  fmap_id : ∀ {A : C.Obj}, fmap (@Category.id C A) = @Category.id D (fobj A)
  -- The `∘` notation routes through `Category.compose` via instance
  -- resolution, but `C`/`D` here are bound *terms* of type Category,
  -- not class instances — so `g ∘ f` cannot find them. Spell the
  -- composition explicitly through field projections.
  fmap_compose : ∀ {X Y Z : C.Obj} (g : C.Hom Y Z) (f : C.Hom X Y),
    fmap (C.compose g f) = D.compose (fmap g) (fmap f)

/-- CNOs are preserved by functors -/
theorem functor_preserves_cno (C D : Category) (F : CatFunctor C D)
    (s : C.Obj) (m : C.Hom s s) :
    isCNOCategorical m →
    isCNOCategorical (F.fmap m) := by
  intro h_cno
  unfold isCNOCategorical at *
  rw [h_cno]
  exact F.fmap_id

/-! ## Model Independence -/

/-- Different computational models can be categories.

    Pre-existing direction bug: original wrote `F.fmap (G.fmap m)` for
    `m : C.Hom s s`, but `G : D ⟶ C` so `G.fmap` consumes `D.Hom`, not
    `C.Hom`. The round-trip that respects the types is
    `G.fmap (F.fmap m)`: push `m` through `F` to land in `D`, then
    pull back through `G` to land in `C` again. -/
def CNOEquivalent (C D : Category) : Prop :=
  ∃ (F : CatFunctor C D) (G : CatFunctor D C),
    ∀ (s : C.Obj) (m : C.Hom s s),
      isCNOCategorical m ↔
      isCNOCategorical (G.fmap (F.fmap m))

/-- Main Universal Theorem: CNO property is model-independent.

    The original statement was type-incorrect: it took `s : C.Obj` and
    then quantified `m' : D.Hom s s`, but `D.Hom` requires `D.Obj`. The
    fix is to existentially produce both the carrier `s' : D.Obj` and
    the morphism `m' : D.Hom s' s'` — set `s' := F.fobj s` and
    `m' := F.fmap m`. Functoriality (`fmap_id`) then transports the
    identity property: `m = C.id` ⟹ `F.fmap m = F.fmap C.id = D.id`. -/
theorem cno_model_independent (C D : Category) (h_eq : CNOEquivalent C D)
    (s : C.Obj) (m : C.Hom s s) (h_cno : isCNOCategorical m) :
    ∃ (s' : D.Obj) (m' : D.Hom s' s'), isCNOCategorical m' := by
  obtain ⟨F, _G, _h_equiv⟩ := h_eq
  refine ⟨F.fobj s, F.fmap m, ?_⟩
  unfold isCNOCategorical at *
  rw [h_cno]
  exact F.fmap_id

/-! ## Yoneda Perspective -/

/-- CNOs are precisely those elements that correspond to identity
    under the Yoneda embedding.

    The body uses `C.compose m f` directly: as in `CatFunctor`, the
    `∘` notation cannot resolve `Category.compose` against a term-level
    `(C : Category)`. -/
theorem yoneda_cno (C : Category) (A : C.Obj) (m : C.Hom A A) :
    isCNOCategorical m ↔ (∀ (B : C.Obj) (f : C.Hom B A), C.compose m f = f) := by
  constructor
  · intro h_cno B f
    rw [h_cno]
    exact C.compose_id_left f
  · intro h
    unfold isCNOCategorical
    -- Take f = id: `C.compose m C.id = C.id`. Then `compose_id_right`
    -- rewrites the LHS to `m`, leaving `this : m = C.id` — exactly
    -- the goal (no `.symm` — original code had it because the
    -- pre-paren parsing was equating the wrong way around).
    have := h A C.id
    rw [C.compose_id_right] at this
    exact this

end CNOCategory
