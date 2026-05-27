(** * Certified Null Operations: Core Framework

    This file formalizes the theory of Certified Null Operations (CNOs) -
    programs that can be mathematically proven to compute nothing.

    Author: Jonathan D. A. Jewell
    Project: Absolute Zero
    License: MPL-2.0
*)

Require Import Coq.Init.Nat.
Require Import Coq.Lists.List.
Require Import Coq.Bool.Bool.
Require Import Coq.Arith.Arith.
Require Import Coq.Logic.FunctionalExtensionality.
Require Import Lia.
Import ListNotations.

(** ** Memory Model *)

(** Memory is modeled as a function from addresses to values *)
Definition Memory : Type := nat -> nat.

(** Empty memory (all zeros) *)
Definition empty_memory : Memory := fun _ => 0.

(** Memory update *)
Definition mem_update (m : Memory) (addr val : nat) : Memory :=
  fun a => if Nat.eqb a addr then val else m a.

(** Memory equality *)
Definition mem_eq (m1 m2 : Memory) : Prop :=
  forall addr, m1 addr = m2 addr.

Notation "m1 '=mem=' m2" := (mem_eq m1 m2) (at level 70).

(** ** Register State *)

(** Registers modeled as a list of natural numbers *)
Definition Registers : Type := list nat.

(** ** I/O State *)

(** I/O state tracks input/output operations *)
Inductive IOOp : Type :=
  | NoIO : IOOp
  | ReadOp : nat -> IOOp
  | WriteOp : nat -> IOOp.

Definition IOState : Type := list IOOp.

(** ** Program State *)

(** Complete program state includes memory, registers, and I/O *)
Record ProgramState : Type := mkState {
  state_memory : Memory;
  state_registers : Registers;
  state_io : IOState;
  state_pc : nat  (* Program counter *)
}.

(** State equality *)
Definition state_eq (s1 s2 : ProgramState) : Prop :=
  (* NOTE (2026-05-18): state_pc is deliberately EXCLUDED. The program
     counter is control-flow bookkeeping, not an observable side effect;
     `step` advances it for every instruction, so requiring PC-equality
     made `nop_is_cno` (and every non-empty CNO claim) FALSE. Observable
     state = memory + registers + I/O. *)
  s1.(state_memory) =mem= s2.(state_memory) /\
  s1.(state_registers) = s2.(state_registers) /\
  s1.(state_io) = s2.(state_io).

Notation "s1 '=st=' s2" := (state_eq s1 s2) (at level 70).

(** ** Instructions *)

(** Abstract instruction set *)
Inductive Instruction : Type :=
  | Nop : Instruction
  | Load : nat -> nat -> Instruction      (* Load mem[addr] to reg *)
  | Store : nat -> nat -> Instruction     (* Store reg to mem[addr] *)
  | Add : nat -> nat -> nat -> Instruction  (* reg3 := reg1 + reg2 *)
  | Jump : nat -> Instruction
  | Halt : Instruction.

(** ** Programs *)

Definition Program : Type := list Instruction.

(** ** Operational Semantics *)

(** Helper: Get register value *)
Fixpoint get_reg (regs : Registers) (n : nat) : option nat :=
  match regs, n with
  | [], _ => None
  | r :: _, 0 => Some r
  | _ :: rs, S n' => get_reg rs n'
  end.

(** Helper: Set register value *)
Fixpoint set_reg (regs : Registers) (n : nat) (val : nat) : Registers :=
  match regs, n with
  | [], _ => []
  | _ :: rs, 0 => val :: rs
  | r :: rs, S n' => r :: set_reg rs n' val
  end.

(** Single step evaluation *)
Inductive step : ProgramState -> Instruction -> ProgramState -> Prop :=
  | step_nop : forall s,
      step s Nop (mkState
        s.(state_memory)
        s.(state_registers)
        s.(state_io)
        (S s.(state_pc)))

  | step_load : forall s addr reg val,
      s.(state_memory) addr = val ->
      step s (Load addr reg) (mkState
        s.(state_memory)
        (set_reg s.(state_registers) reg val)
        s.(state_io)
        (S s.(state_pc)))

  | step_store : forall s addr reg val,
      get_reg s.(state_registers) reg = Some val ->
      step s (Store addr reg) (mkState
        (mem_update s.(state_memory) addr val)
        s.(state_registers)
        s.(state_io)
        (S s.(state_pc)))

  | step_add : forall s r1 r2 r3 v1 v2,
      get_reg s.(state_registers) r1 = Some v1 ->
      get_reg s.(state_registers) r2 = Some v2 ->
      step s (Add r1 r2 r3) (mkState
        s.(state_memory)
        (set_reg s.(state_registers) r3 (v1 + v2))
        s.(state_io)
        (S s.(state_pc)))

  | step_jump : forall s target,
      step s (Jump target) (mkState
        s.(state_memory)
        s.(state_registers)
        s.(state_io)
        target)

  | step_halt : forall s,
      step s Halt s.

(** Multi-step evaluation *)
Inductive eval : Program -> ProgramState -> ProgramState -> Prop :=
  | eval_empty : forall s,
      eval [] s s

  | eval_step : forall i is s1 s2 s3,
      step s1 i s2 ->
      eval is s2 s3 ->
      eval (i :: is) s1 s3.

(** ** Termination *)

(** A program terminates if evaluation produces a final state *)
Definition terminates (p : Program) (s : ProgramState) : Prop :=
  exists s', eval p s s'.

(** ** Side Effects *)

(** No I/O operations occurred *)
Definition no_io (s1 s2 : ProgramState) : Prop :=
  s1.(state_io) = s2.(state_io).

(** No memory allocation (memory equality) *)
Definition no_memory_alloc (s1 s2 : ProgramState) : Prop :=
  s1.(state_memory) =mem= s2.(state_memory).

(** Pure computation (no side effects) *)
Definition pure (s1 s2 : ProgramState) : Prop :=
  no_io s1 s2 /\ no_memory_alloc s1 s2.

(** ** Reversibility *)

(** A program is reversible if there exists an inverse program *)
Definition reversible (p : Program) : Prop :=
  exists p_inv, forall s s',
    eval p s s' ->
    eval p_inv s' s.

(** ** Thermodynamic Reversibility (Landauer's Principle) *)

(** Energy dissipated by a program (simplified model) *)
(** In reality: E = kT ln(2) per bit erased *)
Definition energy_dissipated (p : Program) (s1 s2 : ProgramState) : nat :=
  0.  (* CNOs dissipate zero energy *)

Definition thermodynamically_reversible (p : Program) : Prop :=
  forall s1 s2, eval p s1 s2 -> energy_dissipated p s1 s2 = 0.

(** ** Certified Null Operation Definition *)

(** A CNO is a program that:
    1. Always terminates
    2. Maps any state to itself (identity)
    3. Has no side effects
    4. Is thermodynamically reversible
*)
Definition is_CNO (p : Program) : Prop :=
  (forall s, terminates p s) /\
  (forall s s', eval p s s' -> s =st= s') /\
  (forall s s', eval p s s' -> pure s s') /\
  thermodynamically_reversible p.

(** ** Basic Theorems *)

(** Theorem: CNOs always terminate *)
Theorem cno_terminates :
  forall p, is_CNO p -> forall s, terminates p s.
Proof.
  intros p [H _] s.
  apply H.
Qed.

(** Theorem: CNOs preserve state *)
Theorem cno_preserves_state :
  forall p s s', is_CNO p -> eval p s s' -> s =st= s'.
Proof.
  intros p s s' [_ [H _]] Heval.
  apply H. assumption.
Qed.

(** Theorem: CNOs are pure *)
Theorem cno_pure :
  forall p s s', is_CNO p -> eval p s s' -> pure s s'.
Proof.
  intros p s s' [_ [_ [H _]]] Heval.
  apply H. assumption.
Qed.

(** Theorem: CNOs are thermodynamically reversible *)
Theorem cno_reversible :
  forall p, is_CNO p -> thermodynamically_reversible p.
Proof.
  intros p [_ [_ [_ H]]].
  assumption.
Qed.

(** ** CNO Composition *)

(** Sequential composition of programs *)
Definition seq_comp (p1 p2 : Program) : Program := p1 ++ p2.

(** Key lemma: evaluation of concatenated programs *)
Lemma eval_app :
  forall p1 p2 s s',
    eval (p1 ++ p2) s s' <->
    exists sm, eval p1 s sm /\ eval p2 sm s'.
Proof.
  intros p1 p2 s s'.
  split.
  - (* -> direction *)
    intros H.
    generalize dependent s'.
    generalize dependent s.
    induction p1 as [| i p1' IH]; intros s s' H.
    + (* p1 = [] *)
      simpl in H.
      exists s.
      split.
      * constructor.  (* eval [] s s by eval_empty *)
      * assumption.   (* H : eval p2 s s' *)
    + (* p1 = i :: p1' *)
      simpl in H.
      inversion H; subst.
      (* Grab the recursive `eval (p1' ++ p2)` premise by shape, not by the
         inversion-autogenerated name (which is Coq-version dependent — the
         old `H3` broke under Coq 8.20). *)
      match goal with
      | [ Hrec : eval (p1' ++ p2) _ _ |- _ ] =>
          apply IH in Hrec; destruct Hrec as [sm [H3a H3b]]
      end.
      exists sm.
      split; try assumption.
      eapply eval_step; eassumption.
  - (* <- direction: induct on the derivation `eval p1 s sm` (robust; no
       reliance on inversion-autogenerated hypothesis names, which broke
       under Coq 8.20). *)
    intros [sm [H1 H2]].
    revert s' H2.
    induction H1 as [ s0 | i is s1 s2 s3 Hstep Hev IH ]; intros s' H2.
    + (* eval_empty: p1 = [], s0 = sm *)
      simpl. exact H2.
    + (* eval_step: eval (i :: is) s1 s3 *)
      simpl. eapply eval_step.
      * exact Hstep.
      * apply IH. exact H2.
Qed.

(** State equality is reflexive *)
Lemma state_eq_refl : forall s, s =st= s.
Proof.
  intros s.
  unfold state_eq.
  repeat split; try reflexivity.
  (* Coq 8.20: `reflexivity` already discharges the mem_eq goal (function
     equality), so the old explicit `unfold mem_eq. reflexivity.` now raises
     "No such goal". Make the finisher no-op-safe across Coq versions. *)
  all: try (unfold mem_eq; reflexivity).
Qed.

(** State equality is transitive *)
Lemma state_eq_trans :
  forall s1 s2 s3, s1 =st= s2 -> s2 =st= s3 -> s1 =st= s3.
Proof.
  intros s1 s2 s3 [Hm1 [Hr1 Hi1]] [Hm2 [Hr2 Hi2]].
  unfold state_eq in *.
  split; [ | split ].
  - (* Memory *)
    unfold mem_eq in *.
    intros addr.
    transitivity (s2.(state_memory) addr); auto.
  - (* Registers *)
    transitivity (s2.(state_registers)); auto.
  - (* I/O *)
    transitivity (s2.(state_io)); auto.
Qed.

(** Purity is transitive *)
Lemma pure_trans :
  forall s1 s2 s3, pure s1 s2 -> pure s2 s3 -> pure s1 s3.
Proof.
  intros s1 s2 s3 [P1a P1b] [P2a P2b].
  unfold pure, no_io, no_memory_alloc in *.
  split.
  - (* I/O *)
    transitivity (s2.(state_io)); auto.
  - (* Memory *)
    unfold mem_eq in *.
    intros addr.
    transitivity (s2.(state_memory) addr); auto.
Qed.

(** Theorem: Composition of CNOs is a CNO *)
Theorem cno_composition :
  forall p1 p2, is_CNO p1 -> is_CNO p2 -> is_CNO (seq_comp p1 p2).
Proof.
  intros p1 p2 H1 H2.
  unfold is_CNO in *.
  destruct H1 as [T1 [I1 [P1 R1]]].
  destruct H2 as [T2 [I2 [P2 R2]]].
  (* Robust: explicit nested split (not `repeat split`, which mis-structured
     the 4-way is_CNO conjunction under Coq 8.20), seq_comp unfolded up front,
     and fresh intro names (`s0`) that cannot clash with earlier hypotheses. *)
  unfold seq_comp in *.
  split; [ | split; [ | split ] ].
  - (* Termination *)
    intros s0.
    unfold terminates in *.
    destruct (T1 s0) as [s1 E1].
    destruct (T2 s1) as [s2 E2].
    exists s2.
    apply eval_app.
    exists s1.
    split; assumption.
  - (* Identity *)
    intros s0 s' Heval.
    apply eval_app in Heval.
    destruct Heval as [sm [E1 E2]].
    apply I1 in E1.
    apply I2 in E2.
    eapply state_eq_trans; eassumption.
  - (* Purity *)
    intros s0 s' Heval.
    apply eval_app in Heval.
    destruct Heval as [sm [E1 E2]].
    apply P1 in E1.
    apply P2 in E2.
    eapply pure_trans; eassumption.
  - (* Thermodynamic reversibility *)
    unfold thermodynamically_reversible, energy_dissipated in *.
    intros s1 s2 Heval.
    reflexivity.
Qed.

(** ** The Simplest CNO: Empty Program *)

(** The empty program is a CNO *)
Theorem empty_is_cno : is_CNO [].
Proof.
  unfold is_CNO.
  split; [ | split; [ | split ] ].
  - (* Termination *)
    intros s0. exists s0. constructor.
  - (* Identity: eval [] s0 s' forces s' = s0, so reuse state_eq_refl *)
    intros s0 s' Heval.
    inversion Heval; subst.
    apply state_eq_refl.
  - (* Purity *)
    intros s0 s' Heval.
    inversion Heval; subst.
    unfold pure, no_io, no_memory_alloc, mem_eq.
    split; [ reflexivity | intros addr; reflexivity ].
  - (* Thermodynamic reversibility *)
    unfold thermodynamically_reversible.
    intros s1 s2 Heval.
    reflexivity.
Qed.

(** ** The Canonical CNO: Single Nop *)

(** A single Nop instruction is a CNO *)
Theorem nop_is_cno : is_CNO [Nop].
Proof.
  unfold is_CNO.
  (* Robust: explicit nested split, fresh names, and inversion hypotheses
     grabbed by shape (match goal) so Coq-version autonaming can't break it.
     With PC excluded from state_eq, [Nop] is genuinely a CNO. *)
  split; [ | split; [ | split ] ].
  - (* Termination *)
    intros s0.
    eexists.
    eapply eval_step.
    + apply step_nop.
    + apply eval_empty.
  - (* Identity *)
    intros s0 s' Heval.
    inversion Heval; subst.
    match goal with H : step _ Nop _ |- _ => inversion H; subst end.
    match goal with H : eval [] _ _ |- _ => inversion H; subst end.
    unfold state_eq, mem_eq.
    split; [ intros addr; reflexivity | split; reflexivity ].
  - (* Purity *)
    intros s0 s' Heval.
    inversion Heval; subst.
    match goal with H : step _ Nop _ |- _ => inversion H; subst end.
    match goal with H : eval [] _ _ |- _ => inversion H; subst end.
    unfold pure, no_io, no_memory_alloc, mem_eq.
    split; [ reflexivity | intros addr; reflexivity ].
  - (* Thermodynamic reversibility *)
    unfold thermodynamically_reversible.
    intros s1 s2 Heval.
    reflexivity.
Qed.

(** ** CNO Equivalence *)

(** Single-step evaluation is fully deterministic: same start state and
    instruction force a syntactically identical result state. The result
    of `step` is always `mkState …` whose components are functions of the
    instruction and the start state (`state_memory`, `get_reg`,
    `set_reg`, `mem_update`); the auxiliary witnesses in [step_load] /
    [step_store] / [step_add] are pinned by their hypotheses. *)
Lemma step_deterministic_strong : forall s i s1 s2,
  step s i s1 -> step s i s2 -> s1 = s2.
Proof.
  intros s i s1 s2 H1 H2.
  destruct i; inversion H1; subst; inversion H2; subst; try reflexivity.
  - (* Store: two get_reg results agree by functional dependence. *)
    match goal with
    | [ Ha : get_reg _ _ = Some ?v1,
        Hb : get_reg _ _ = Some ?v2 |- _ ] =>
        rewrite Ha in Hb; injection Hb as ->
    end.
    reflexivity.
  - (* Add: two pairs of get_reg results. *)
    repeat match goal with
    | [ Ha : get_reg ?regs ?r = Some ?v1,
        Hb : get_reg ?regs ?r = Some ?v2 |- _ ] =>
        rewrite Ha in Hb; injection Hb as ->; clear Ha
    end.
    reflexivity.
Qed.

(** Evaluation is deterministic. Discharged 2026-05-20 from
    [step_deterministic_strong] by induction on the eval derivation —
    [Print Assumptions] reports "Closed under the global context".
    Was previously an [Axiom] (see PROOF-STATUS-2026-05-18.md
    "post-T0 axiom audit"). *)
Theorem eval_deterministic : forall p s s1 s2,
  eval p s s1 -> eval p s s2 -> s1 =st= s2.
Proof.
  intros p s s1 s2 H1.
  generalize dependent s2.
  induction H1 as [ s0 | i is sA sB sC Hstep Hev IH ]; intros s2 H2.
  - (* eval_empty: inversion forces s2 = s0. *)
    inversion H2; subst. apply state_eq_refl.
  - (* eval_step: inversion gives step sA i sB' and eval is sB' s2.
       step_deterministic_strong then forces sB = sB' (syntactic), so the
       induction hypothesis closes the tail. *)
    inversion H2; subst.
    match goal with
    | [ Hs : step sA i ?sBp, He : eval is ?sBp s2 |- _ ] =>
        pose proof (step_deterministic_strong _ _ _ _ Hstep Hs) as Heq;
        subst sBp;
        apply IH; exact He
    end.
Qed.

(** Two programs are CNO-equivalent if they produce the same state transformations *)
Definition cno_equiv (p1 p2 : Program) : Prop :=
  forall s s1 s2,
    eval p1 s s1 -> eval p2 s s2 -> s1 =st= s2.

(** Theorem: CNO equivalence is an equivalence relation *)
Theorem cno_equiv_refl : forall p, cno_equiv p p.
Proof.
  unfold cno_equiv.
  intros p s s1 s2 H1 H2.
  (* Both evaluations of same program produce same result by determinism *)
  apply eval_deterministic with (p := p) (s := s); assumption.
Qed.

Theorem cno_equiv_sym : forall p1 p2, cno_equiv p1 p2 -> cno_equiv p2 p1.
Proof.
  unfold cno_equiv.
  intros p1 p2 H s s1 s2 H1 H2.
  (* `symmetry` needs a registered Symmetric instance for =st= (none), and
     state_eq_sym is defined later in this file. Derive s2 =st= s1 from H
     and flip each observable component by hand. *)
  destruct (H s s2 s1 H2 H1) as [Hm [Hr Hi]].
  unfold state_eq, mem_eq in *.
  split; [ | split ].
  - intros addr; symmetry; apply Hm.
  - symmetry; exact Hr.
  - symmetry; exact Hi.
Qed.

(** State equality is symmetric *)
Lemma state_eq_sym : forall s1 s2, s1 =st= s2 -> s2 =st= s1.
Proof.
  intros s1 s2 [Hm [Hr Hi]].
  unfold state_eq, mem_eq in *.
  split; [ | split ].
  - intros addr. symmetry. apply Hm.
  - symmetry. exact Hr.
  - symmetry. exact Hi.
Qed.

Theorem cno_equiv_trans :
  forall p1 p2 p3,
    (forall s, terminates p2 s) ->  (* Add termination hypothesis *)
    cno_equiv p1 p2 -> cno_equiv p2 p3 -> cno_equiv p1 p3.
Proof.
  unfold cno_equiv, terminates.
  intros p1 p2 p3 T2 H12 H23 s s1 s3 H1 H3.
  (* Since p2 terminates from s, get the intermediate state *)
  destruct (T2 s) as [s2 E2].
  (* Now we can use H12 and H23 *)
  assert (Heq12 : s1 =st= s2) by (apply H12 with s; assumption).
  assert (Heq23 : s2 =st= s3) by (apply H23 with s; assumption).
  eapply state_eq_trans; eassumption.
Qed.

(** Better formulation: equivalence for CNOs specifically *)
Theorem cno_equiv_trans_for_cnos :
  forall p1 p2 p3,
    is_CNO p1 -> is_CNO p2 -> is_CNO p3 ->
    cno_equiv p1 p2 -> cno_equiv p2 p3 -> cno_equiv p1 p3.
Proof.
  unfold cno_equiv.
  intros p1 p2 p3 C1 C2 C3 H12 H23 s s1 s3 H1 H3.
  (* Since p2 is a CNO, it terminates *)
  destruct C2 as [T2 _].
  destruct (T2 s) as [s2 E2].
  unfold terminates in T2.
  (* Now we can use H12 and H23 *)
  assert (Heq12 : s1 =st= s2) by (apply H12 with s; assumption).
  assert (Heq23 : s2 =st= s3) by (apply H23 with s; assumption).
  eapply state_eq_trans; eassumption.
Qed.

(** ** Decidability *)

(** Question: Is CNO verification decidable? *)

(** Universal decidability of [is_CNO] over arbitrary programs was
    previously asserted as an [Axiom] [cno_decidable]. That axiom has
    been dropped (owner ruling, Stage 3 of standards#157): the
    universal form asks for a constructive decision procedure over an
    unbounded state space ([Memory : nat -> nat], unbounded
    registers, unbounded I/O traces), and [is_CNO]'s
    universally-quantified clauses range over all of them. This is
    essentially the Rice's-theorem situation: deciding a non-trivial
    semantic property of an arbitrary program is undecidable, so the
    universal-decidability claim is not constructively achievable in
    Coq and any client that wants decidability must work in a
    restricted syntactic fragment of [Program] (which the current
    development does not formalise). No theorem in this file depended
    on it. *)

(** ** Complexity *)

(** Conjecture: Verifying a CNO is at least as hard as verifying any property *)
(** This relates to the epistemology of proving absence vs. presence *)

(** Verification complexity is measured in terms of:
    - Number of instructions to analyze
    - Number of states to verify (exponential in memory size)
    - Depth of loop/recursion analysis for termination

    For a program p, complexity is at minimum linear in program length,
    but can be exponential due to state space explosion in loop analysis.
*)
Fixpoint instruction_complexity (i : Instruction) : nat :=
  match i with
  | Nop => 1        (* Trivial to verify *)
  | Halt => 1       (* Trivial to verify *)
  | Load _ _ => 2   (* Must track memory access *)
  | Store _ _ => 3  (* Must verify memory unchanged after execution *)
  | Add _ _ _ => 2  (* Must verify register unchanged *)
  | Jump _ => 5     (* Must analyze control flow *)
  end.

(** Verification complexity: sum of instruction complexities *)
Fixpoint verification_complexity (p : Program) : nat :=
  match p with
  | [] => 1  (* Empty program trivially verified *)
  | i :: rest => instruction_complexity i + verification_complexity rest
  end.

(** Theorem: Verification complexity is at least linear in program length *)
Theorem verification_complexity_lower_bound :
  forall p, verification_complexity p >= length p.
Proof.
  induction p as [| i rest IH].
  - (* Empty program *)
    simpl. lia.
  - (* Non-empty program *)
    simpl.
    assert (H: instruction_complexity i >= 1).
    { destruct i; simpl; lia. }
    lia.
Qed.

(** Conjecture: CNO verification has additional overhead due to state preservation check *)
Conjecture cno_verification_overhead :
  forall p, is_CNO p ->
    verification_complexity p >= length p + 1.

(** ** Export for other modules *)

(* Make key definitions and theorems available *)

(** ** State Equality and Evaluation *)

(** UNSOUND-AS-STATED axioms previously declared here
    (`eval_respects_state_eq_right` and `_left`) have been REMOVED
    (2026-05-20). The rescue branch's PC-excluding [state_eq] (memory +
    registers + I/O, not [state_pc]) means two `=st=` states can carry
    different PCs, while the eval relation deterministically propagates
    PC through every step constructor. So [eval p s s'] forces a unique
    [s'] (cf. [eval_deterministic]) — replacing [s'] by an [=st=]-equal
    [s''] is generally unsound (different PC).

    The only consumer that needed the strong form ([cno_logically_reversible]
    in StatMech.v) has been refactored to use [cno_terminates] +
    [cno_preserves_state] directly, with a correspondingly-weakened
    [logically_reversible] definition. See ADR-008. *)

(** For CNOs specifically, termination from one state is equivalent to
    termination from any state-equal state — both can be witnessed by
    [cno_terminates] (the witnesses need not coincide). *)
Lemma cno_eval_on_equal_states :
  forall p s s',
    is_CNO p ->
    s =st= s' ->
    (exists s1, eval p s s1) <-> (exists s2, eval p s' s2).
Proof.
  intros p s s' H_cno _.
  split; intros _.
  - (* termination from s' is direct from is_CNO p *)
    destruct (cno_terminates p H_cno s') as [s2 Heval2].
    exists s2; exact Heval2.
  - destruct (cno_terminates p H_cno s) as [s1 Heval1].
    exists s1; exact Heval1.
Qed.

