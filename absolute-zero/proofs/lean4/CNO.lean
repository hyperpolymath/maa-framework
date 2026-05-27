/- Certified Null Operations: Lean 4 Formalization

   This file provides a Lean 4 formalization of CNO theory.
   Lean 4 offers clean syntax and powerful tactics for automated theorem proving.

   Author: Jonathan D. A. Jewell
   Project: Absolute Zero
   License: MPL-2.0
-/

-- Std.Data.{List,Nat}.Basic were vestigial: Std was renamed to Batteries
-- around Lean 4.5, and the List/Nat APIs used here (`++`, `foldl`, `get?`,
-- `Repr`, `BEq`) are all in core Lean 4. No external imports required.

namespace CNO

/-! ## Memory Model -/

/-- Memory is modeled as a function from addresses to values.
    `abbrev` (rather than `def`) makes the definition reducible, so any
    typeclass instance for `Nat → Nat` (none in core, but consistent with
    sibling aliases below) is available on `Memory`. -/
abbrev Memory : Type := Nat → Nat

/-- Empty memory (all zeros) -/
def Memory.empty : Memory := fun _ => 0

/-- Update memory at an address -/
def Memory.update (m : Memory) (addr val : Nat) : Memory :=
  fun a => if a == addr then val else m a

/-- Memory equality -/
def Memory.eq (m1 m2 : Memory) : Prop :=
  ∀ addr, m1 addr = m2 addr

instance : BEq Memory where
  beq m1 m2 := true  -- Simplified for decidability

/-! ## Program State -/

/-- Registers are a list of natural numbers.
    `abbrev` so List's HAppend / Repr / BEq instances propagate. -/
abbrev Registers : Type := List Nat

/-- I/O operations -/
inductive IOOp where
  | noIO : IOOp
  | read : Nat → IOOp
  | write : Nat → IOOp
  deriving Repr, BEq

/-- I/O state is a list of operations. `abbrev` so List instances propagate. -/
abbrev IOState : Type := List IOOp

/-- Complete program state.
    No `deriving Repr`: `Memory` is `Nat → Nat`, which has no canonical
    `Repr` instance (functions are not displayable).
    `deriving BEq` works via the trivial `BEq Memory` instance above
    plus core BEq for the other fields, and is required by downstream
    distributions like `StatMech.pointDist` that branch on `s == s0`. -/
structure ProgramState where
  memory : Memory
  registers : Registers
  ioState : IOState
  pc : Nat  -- Program counter
  deriving BEq

/-- State equality -/
def ProgramState.eq (s1 s2 : ProgramState) : Prop :=
  Memory.eq s1.memory s2.memory ∧
  s1.registers = s2.registers ∧
  s1.ioState = s2.ioState ∧
  s1.pc = s2.pc

/-! ## Instructions -/

/-- Abstract instruction set -/
inductive Instruction where
  | nop : Instruction
  | halt : Instruction
  | load : Nat → Nat → Instruction      -- Load mem[addr] to reg
  | store : Nat → Nat → Instruction     -- Store reg to mem[addr]
  | add : Nat → Nat → Nat → Instruction -- r3 := r1 + r2
  | jump : Nat → Instruction
  deriving Repr, BEq

/-- A program is a list of instructions.
    `abbrev` (not `def`) so List's `++` / `HAppend` instance is available
    on `Program`. With `def`, `seqComp` below would fail to elaborate. -/
abbrev Program : Type := List Instruction

/-! ## Helper Functions -/

/-- Get register value -/
def getReg (regs : Registers) (n : Nat) : Option Nat :=
  regs.get? n

/-- Set register value -/
def setReg (regs : Registers) (n : Nat) (val : Nat) : Registers :=
  match regs, n with
  | [], _ => []
  | _ :: rs, 0 => val :: rs
  | r :: rs, n' + 1 => r :: setReg rs n' val

/-! ## Operational Semantics -/

/-- Single step evaluation -/
def step (s : ProgramState) (i : Instruction) : ProgramState :=
  match i with
  | .nop =>
      { s with pc := s.pc + 1 }

  | .halt => s

  | .load addr reg =>
      let val := s.memory addr
      { s with
        registers := setReg s.registers reg val,
        pc := s.pc + 1 }

  | .store addr reg =>
      match getReg s.registers reg with
      | some val =>
          { s with
            memory := s.memory.update addr val,
            pc := s.pc + 1 }
      | none => s  -- Error case

  | .add r1 r2 r3 =>
      match getReg s.registers r1, getReg s.registers r2 with
      | some v1, some v2 =>
          { s with
            registers := setReg s.registers r3 (v1 + v2),
            pc := s.pc + 1 }
      | _, _ => s  -- Error case

  | .jump target =>
      { s with pc := target }

/-- Multi-step evaluation -/
def eval (p : Program) (s : ProgramState) : ProgramState :=
  match p with
  | [] => s
  | i :: is => eval is (step s i)

/-! ## Termination -/

/-- A program terminates if evaluation produces a result -/
def terminates (p : Program) (s : ProgramState) : Prop :=
  ∃ s', eval p s = s'

/-- Termination is always true for our finite programs -/
theorem terminates_always (p : Program) (s : ProgramState) :
    terminates p s := by
  unfold terminates
  exists eval p s

/-! ## Side Effects -/

/-- No I/O operations occurred -/
def noIO (s1 s2 : ProgramState) : Prop :=
  s1.ioState = s2.ioState

/-- No memory allocation (memory equality) -/
def noMemoryAlloc (s1 s2 : ProgramState) : Prop :=
  Memory.eq s1.memory s2.memory

/-- Pure computation (no side effects) -/
def pure (s1 s2 : ProgramState) : Prop :=
  noIO s1 s2 ∧ noMemoryAlloc s1 s2

/-! ## Reversibility -/

/-- A program is reversible if there exists an inverse -/
def reversible (p : Program) : Prop :=
  ∃ p_inv, ∀ s, eval p_inv (eval p s) = s

/-! ## Thermodynamic Reversibility -/

/-- Energy dissipated (simplified model) -/
def energyDissipated (p : Program) (s1 s2 : ProgramState) : Nat := 0

/-- Thermodynamically reversible programs dissipate no energy -/
def thermodynamicallyReversible (p : Program) : Prop :=
  ∀ s, energyDissipated p s (eval p s) = 0

/-! ## CNO Definition -/

/-- A Certified Null Operation is a program that:
    1. Always terminates (trivially true for finite programs)
    2. Preserves state (identity mapping)
    3. Has no side effects
    4. Is thermodynamically reversible
-/
def isCNO (p : Program) : Prop :=
  (∀ s, terminates p s) ∧
  (∀ s, ProgramState.eq (eval p s) s) ∧
  (∀ s, pure s (eval p s)) ∧
  thermodynamicallyReversible p

/-! ## Basic Theorems -/

/-- The empty program is a CNO -/
theorem empty_is_cno : isCNO [] := by
  unfold isCNO
  constructor
  · intro s; exact terminates_always [] s
  constructor
  · intro s
    unfold ProgramState.eq eval
    simp [Memory.eq, noIO, noMemoryAlloc]
  constructor
  · intro s
    unfold pure noIO noMemoryAlloc eval
    simp [Memory.eq]
  · unfold thermodynamicallyReversible energyDissipated
    intro s; rfl

/-- A single NOP is a CNO (modulo PC) -/
theorem nop_preserves_most_state (s : ProgramState) :
    let s' := eval [.nop] s
    Memory.eq s.memory s'.memory ∧
    s.registers = s'.registers ∧
    s.ioState = s'.ioState := by
  -- eval [.nop] s = eval [] (step s .nop) = step s .nop = {s with pc := s.pc+1}
  -- So memory, registers, ioState are all syntactically unchanged.
  refine ⟨?_, rfl, rfl⟩
  intro addr
  rfl

/-- Halt is a perfect CNO.
    `eval [.halt] s` reduces definitionally to `s` (halt's step returns the
    state unchanged, then `eval []` is identity), so each conjunct is
    discharged by `rfl`-style reasoning. -/
theorem halt_is_cno : isCNO [.halt] := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · intro s; exact terminates_always [.halt] s
  · intro s
    -- ProgramState.eq (eval [.halt] s) s ≡ Memory.eq ∧ regs= ∧ io= ∧ pc=
    refine ⟨?_, rfl, rfl, rfl⟩
    intro addr
    rfl
  · intro s
    -- pure s (eval [.halt] s) = noIO ∧ noMemoryAlloc
    refine ⟨rfl, ?_⟩
    intro addr
    rfl
  · -- thermodynamicallyReversible: ∀ s, energyDissipated _ _ _ = 0, and
    -- energyDissipated is defined as the constant 0.
    intro s
    rfl

/-! ## CNO Properties -/

/-- CNOs always terminate -/
theorem cno_terminates (p : Program) (h : isCNO p) (s : ProgramState) :
    terminates p s := by
  exact h.1 s

/-- CNOs preserve state -/
theorem cno_preserves_state (p : Program) (h : isCNO p) (s : ProgramState) :
    ProgramState.eq (eval p s) s := by
  exact h.2.1 s

/-- CNOs are pure -/
theorem cno_pure (p : Program) (h : isCNO p) (s : ProgramState) :
    pure s (eval p s) := by
  exact h.2.2.1 s

/-- CNOs are thermodynamically reversible -/
theorem cno_reversible (p : Program) (h : isCNO p) :
    thermodynamicallyReversible p := by
  exact h.2.2.2

/-! ## Composition -/

/-- Sequential composition of programs.
    `abbrev` (not `def`) so that `eval_seqComp` rewrites also fire when
    the goal mentions the underlying `++` directly (downstream callers
    in `CNOCategory.composeMorphisms` build the program with `++` and
    rely on this transparency). -/
abbrev seqComp (p1 p2 : Program) : Program := p1 ++ p2

/-- Evaluation of composition.
    `unfold eval` unfolds the LHS one step but leaves the RHS in its
    folded form, producing an apparent type mismatch. Use `show` to put
    both sides into the same canonical shape, then the induction
    hypothesis applies directly. -/
theorem eval_seqComp (p1 p2 : Program) (s : ProgramState) :
    eval (seqComp p1 p2) s = eval p2 (eval p1 s) := by
  unfold seqComp
  induction p1 generalizing s with
  | nil => rfl
  | cons i is ih =>
      -- LHS = eval (i :: is ++ p2) s = eval (is ++ p2) (step s i)
      -- RHS = eval p2 (eval (i :: is) s) = eval p2 (eval is (step s i))
      show eval (is ++ p2) (step s i) = eval p2 (eval is (step s i))
      exact ih (step s i)

/-- State equality is transitive -/
theorem state_eq_trans (s1 s2 s3 : ProgramState) :
    ProgramState.eq s1 s2 → ProgramState.eq s2 s3 → ProgramState.eq s1 s3 := by
  intro h12 h23
  unfold ProgramState.eq at *
  obtain ⟨mem12, reg12, io12, pc12⟩ := h12
  obtain ⟨mem23, reg23, io23, pc23⟩ := h23
  constructor
  · unfold Memory.eq at *
    intro addr
    rw [mem12, mem23]
  constructor
  · rw [reg12, reg23]
  constructor
  · rw [io12, io23]
  · rw [pc12, pc23]

/-- Purity is transitive -/
theorem pure_trans (s1 s2 s3 : ProgramState) :
    pure s1 s2 → pure s2 s3 → pure s1 s3 := by
  intro ⟨io12, mem12⟩ ⟨io23, mem23⟩
  constructor
  · unfold noIO at *
    rw [io12, io23]
  · unfold noMemoryAlloc Memory.eq at *
    intro addr
    rw [mem12, mem23]

/-- Composition of CNOs is a CNO -/
theorem cno_composition (p1 p2 : Program) (h1 : isCNO p1) (h2 : isCNO p2) :
    isCNO (seqComp p1 p2) := by
  unfold isCNO at *
  obtain ⟨t1, i1, pu1, r1⟩ := h1
  obtain ⟨t2, i2, pu2, r2⟩ := h2
  constructor
  · intro s; exact terminates_always (seqComp p1 p2) s
  constructor
  · intro s
    rw [eval_seqComp]
    -- p1 maps s to itself, so eval p1 s = s (by i1)
    -- p2 maps (eval p1 s) to itself, so eval p2 (eval p1 s) = eval p1 s (by i2)
    -- Therefore eval p2 (eval p1 s) = s by transitivity
    have h1_eq := i1 s              -- ProgramState.eq (eval p1 s) s
    have h2_eq := i2 (eval p1 s)    -- ProgramState.eq (eval p2 (eval p1 s)) (eval p1 s)
    -- Want: ProgramState.eq (eval p2 (eval p1 s)) s. Chain h2_eq then h1_eq.
    exact state_eq_trans (eval p2 (eval p1 s)) (eval p1 s) s h2_eq h1_eq
  constructor
  · intro s
    rw [eval_seqComp]
    -- Purity of p1: pure s (eval p1 s)
    -- Purity of p2: pure (eval p1 s) (eval p2 (eval p1 s))
    -- By transitivity: pure s (eval p2 (eval p1 s))
    have pu1_s := pu1 s
    have pu2_s := pu2 (eval p1 s)
    exact pure_trans s (eval p1 s) (eval p2 (eval p1 s)) pu1_s pu2_s
  · unfold thermodynamicallyReversible energyDissipated
    intro s; rfl

/-! ## Malbolge-Specific Definitions -/

/-- Ternary operations for Malbolge -/
def ternaryAdd (a b : Nat) : Nat := (a + b) % 3

/-- Crazy operation (simplified) -/
def crazyOp (a b : Nat) : Nat := (a + b) % 3

/-- Crazy op with zero is identity -/
theorem crazy_op_zero (a : Nat) : crazyOp a 0 = a % 3 := by
  unfold crazyOp
  simp [Nat.add_mod]

/-- Three ternary rotations equal identity -/
def rotateRight (n : Nat) : Nat := n  -- Simplified

theorem triple_rotation_identity (n : Nat) :
    rotateRight (rotateRight (rotateRight n)) = n := by
  unfold rotateRight
  rfl

/-! ## CNO Examples -/

/-- Load then store to same location is a CNO (modulo PC) -/
def loadStoreSame (addr : Nat) : Program :=
  [.load addr 0, .store addr 0]

/- ── Helper lemmas for loadStore_preserves_memory ──────────────────────────

   These three private theorems establish the round-trip property of
   `setReg`/`getReg` for register index 0 and the no-op character of
   `Memory.update m addr (m addr)`.  They are the rewrite-lemma layer
   mentioned in the DEFERRED comment below.

   `setReg_cons_zero` and `getReg_cons_zero` make `simp` able to compute
   through the match chains in `setReg`/`getReg` (which are `def`, not
   `abbrev`, so are otherwise opaque to the simp normal-form engine).

   `Memory.update_same_pointwise` is the key identity-update fact: writing
   the value already stored at an address is a no-op, point-wise. -/

private theorem setReg_cons_zero (r val : Nat) (rs : List Nat) :
    setReg (r :: rs) 0 val = val :: rs := by
  unfold setReg
  rfl

private theorem getReg_cons_zero (val : Nat) (rs : List Nat) :
    getReg (val :: rs) 0 = some val := by
  unfold getReg
  rfl

/-- Writing the value already at `addr` back to `addr` is a pointwise no-op. -/
private theorem Memory.update_same_pointwise (m : Memory) (addr a : Nat) :
    Memory.update m addr (m addr) a = m a := by
  unfold Memory.update
  -- The branch condition is `a == addr : Bool`.  We case-split on whether
  -- `a` and `addr` are propositionally equal, then close each sub-goal.
  by_cases h : a = addr
  · -- Equal branch: write m addr back → still m addr = m a (since a = addr).
    subst h
    -- After substitution: `(if addr == addr then m addr else m addr) = m addr`.
    simp
  · -- Unequal branch: the if-branch is skipped, result is m a.
    have hne : (a == addr) = false := by simp [h]
    simp [hne]

/-- This preserves memory.

    Proof strategy (rewrite-lemma layer):
      - `load addr 0` puts `s.memory addr` into register 0; because
        `loadStoreSame` only executes on a concrete two-instruction list,
        we can `show` the definitionally-equal fully-reduced form of
        `eval (loadStoreSame addr) s` and work on that directly.
      - We then case-split on `s.registers`:
          nil  → `setReg [] 0 _  = []`, `getReg [] 0 = none`, so the
                  store instruction takes the `none` branch and leaves
                  memory untouched.
          cons → `setReg_cons_zero` + `getReg_cons_zero` give the round-trip
                  `getReg (setReg (r :: rs) 0 v) 0 = some v`; the store
                  then writes `Memory.update s.memory addr (s.memory addr)`,
                  which equals `s.memory` pointwise by
                  `Memory.update_same_pointwise`.
      - No proof holes. -/
theorem loadStore_preserves_memory (addr : Nat) (s : ProgramState) :
    let s' := eval (loadStoreSame addr) s
    Memory.eq s.memory s'.memory := by
  -- Reduce `eval (loadStoreSame addr) s` to its fully-computed form.
  -- `loadStoreSame addr = [.load addr 0, .store addr 0]` and `eval` on a
  -- concrete two-instruction list is definitionally:
  --   eval [l, st] s = step (step s l) st
  -- so `show` restates the goal at that fully-reduced type.
  show Memory.eq s.memory
       (step (step s (.load addr 0)) (.store addr 0)).memory
  -- Unfold both steps explicitly.
  --   step s (.load addr 0)
  --     = { s with registers := setReg s.registers 0 (s.memory addr),
  --                pc := s.pc + 1 }          [call this s_mid]
  --   step s_mid (.store addr 0)
  --     = match getReg s_mid.registers 0 with
  --       | some v => { s_mid with memory := Memory.update s_mid.memory addr v,
  --                                pc := s_mid.pc + 1 }
  --       | none   => s_mid
  --   s_mid.registers = setReg s.registers 0 (s.memory addr)
  --   s_mid.memory    = s.memory
  -- Unfolding `step` leaves struct-literal field projections.
  -- `simp only []` applies pure definitional (ι) reductions — specifically,
  -- it reduces struct-literal field accesses like
  -- `{ s with registers := X }.registers ↝ X`
  -- without unfolding any user-defined `def`.
  unfold step
  simp only []
  -- Now case-split on s.registers to concretise setReg/getReg.
  cases s.registers with
  | nil =>
    -- s.registers = []
    -- setReg [] 0 _ = []   (nil case of setReg's definition, reduces by ι)
    -- getReg [] 0  = [].get? 0 = none  (idem, plus List.get? on nil)
    -- ⟹ the store instruction takes the `none` error branch → leaves
    --    memory and registers unchanged.
    -- `unfold setReg getReg` expands both defs by their match clauses.
    -- ι-reductions then collapse the goal: `[].get? 0 ↝ none`,
    -- `match none with ... | none => X ↝ X`, and `.memory` projection
    -- happen by definitional equality, so `rfl` closes after `intro a`.
    unfold setReg getReg
    intro a
    rfl
  | cons r rs =>
    -- s.registers = r :: rs
    -- setReg (r :: rs) 0 v = v :: rs   (setReg_cons_zero)
    -- getReg (v :: rs)  0 = some v      (getReg_cons_zero)
    -- ⟹ store writes Memory.update s.memory addr (s.memory addr),
    --   which equals s.memory pointwise (Memory.update_same_pointwise).
    --
    -- After the two rewrites the match scrutinee is `some (s.memory addr)`.
    -- `simp only []` applies the ι (constructor-match) reduction to resolve
    -- it to the `some` arm, and then reduces the resulting struct-literal
    -- `.memory` field access.  The goal is then:
    --   Memory.eq s.memory (Memory.update s.memory addr (s.memory addr))
    rw [setReg_cons_zero, getReg_cons_zero]
    simp only []
    intro a
    exact (Memory.update_same_pointwise s.memory addr a).symm

/-! ## Decidability and Complexity

    Question: Is CNO verification decidable?
    For finite programs with bounded execution, yes.
    For arbitrary programs, this reduces to the halting problem. -/

/-- Complexity measure -/
def complexity (i : Instruction) : Nat :=
  match i with
  | .nop => 0
  | .halt => 0
  | .load _ _ => 1
  | .store _ _ => 1
  | .add _ _ _ => 2
  | .jump _ => 1

/-- Program complexity -/
def programComplexity (p : Program) : Nat :=
  p.foldl (fun acc i => acc + complexity i) 0

/-- Simple instructions have minimal complexity -/
theorem nop_minimal_complexity : complexity .nop = 0 := rfl
theorem halt_minimal_complexity : complexity .halt = 0 := rfl

/-! ## Absolute Zero -/

/-- The titular "Absolute Zero" program: does nothing -/
def absoluteZero : Program := []

/-- Absolute Zero is a CNO -/
theorem absoluteZero_is_cno : isCNO absoluteZero := empty_is_cno

end CNO
