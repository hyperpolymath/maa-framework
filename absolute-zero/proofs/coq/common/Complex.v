(** * Complex.v — self-contained complex numbers (no external deps)

    Replaces the non-existent `Coq.Complex.Complex` and avoids the
    Coquelicot -> mathcomp2 -> Hierarchy-Builder -> coq-elpi dependency
    chain. The quantum proofs use only shallow field arithmetic on
    C = R*R (Cplus, Cmult, Cconj, RtoC, Cinv, Re/Im), so a small
    self-contained module is the right call.

    Decision recorded in PROOF-STATUS-2026-05-18.md. *)

Require Import Coq.Reals.Reals.
Require Import Coq.micromega.Lra.
Open Scope R_scope.

Definition C : Type := (R * R)%type.

Definition Re (z : C) : R := fst z.
Definition Im (z : C) : R := snd z.

Definition RtoC (r : R) : C := (r, 0).
Coercion RtoC : R >-> C.

Definition C0 : C := (0, 0).
Definition C1 : C := (1, 0).
Definition Ci : C := (0, 1).

Definition Cplus (a b : C) : C := (fst a + fst b, snd a + snd b).
Definition Copp  (a : C)   : C := (- fst a, - snd a).
Definition Cminus (a b : C) : C := Cplus a (Copp b).
Definition Cmult (a b : C) : C :=
  (fst a * fst b - snd a * snd b, fst a * snd b + snd a * fst b).
Definition Cconj (z : C) : C := (fst z, - snd z).

Definition Cnorm2 (z : C) : R := fst z * fst z + snd z * snd z.
Definition Cinv (z : C) : C :=
  (fst z / Cnorm2 z, - snd z / Cnorm2 z).
Definition Cdiv (a b : C) : C := Cmult a (Cinv b).

Declare Scope C_scope.
Delimit Scope C_scope with C.
Bind Scope C_scope with C.
Infix "+" := Cplus : C_scope.
Infix "-" := Cminus : C_scope.
Infix "*" := Cmult : C_scope.
Infix "/" := Cdiv : C_scope.
Notation "- x" := (Copp x) : C_scope.

(** A complex pair is determined by its components. *)
Lemma Cpair_eq : forall a b : C, fst a = fst b -> snd a = snd b -> a = b.
Proof.
  intros [a1 a2] [b1 b2] H1 H2; simpl in *; subst; reflexivity.
Qed.

Ltac Csolve := apply Cpair_eq; simpl; lra.

Lemma Cplus_comm  : forall a b, Cplus a b = Cplus b a.            Proof. intros; Csolve. Qed.
Lemma Cplus_assoc : forall a b c, Cplus a (Cplus b c) = Cplus (Cplus a b) c. Proof. intros; Csolve. Qed.
Lemma Cplus_0_l   : forall a, Cplus C0 a = a.                     Proof. intros [??]; Csolve. Qed.
Lemma Cplus_0_r   : forall a, Cplus a C0 = a.                     Proof. intros [??]; Csolve. Qed.
Lemma Cmult_comm  : forall a b, Cmult a b = Cmult b a.            Proof. intros; Csolve. Qed.
Lemma Cmult_assoc : forall a b c, Cmult a (Cmult b c) = Cmult (Cmult a b) c. Proof. intros; Csolve. Qed.
Lemma Cmult_1_l   : forall a, Cmult C1 a = a.                     Proof. intros [??]; Csolve. Qed.
Lemma Cmult_1_r   : forall a, Cmult a C1 = a.                     Proof. intros [??]; Csolve. Qed.
Lemma Cmult_0_l   : forall a, Cmult C0 a = C0.                    Proof. intros [??]; Csolve. Qed.
Lemma Cmult_plus_distr_l :
  forall a b c, Cmult a (Cplus b c) = Cplus (Cmult a b) (Cmult a c).
Proof. intros; Csolve. Qed.

Lemma Cconj_involutive : forall z, Cconj (Cconj z) = z.
Proof. intros [??]; Csolve. Qed.
Lemma Cconj_plus : forall a b, Cconj (Cplus a b) = Cplus (Cconj a) (Cconj b).
Proof. intros; Csolve. Qed.
Lemma Cconj_mult : forall a b, Cconj (Cmult a b) = Cmult (Cconj a) (Cconj b).
Proof. intros; Csolve. Qed.
Lemma Cconj_RtoC : forall r : R, Cconj (RtoC r) = RtoC r.
Proof. intros; unfold Cconj, RtoC; Csolve. Qed.
Lemma RtoC_plus : forall r s : R, RtoC (r + s) = Cplus (RtoC r) (RtoC s).
Proof. intros; unfold RtoC; Csolve. Qed.
Lemma RtoC_mult : forall r s : R, RtoC (r * s) = Cmult (RtoC r) (RtoC s).
Proof. intros; unfold RtoC; Csolve. Qed.
Lemma RtoC_opp : forall r : R, RtoC (- r) = Copp (RtoC r).
Proof. intros; unfold RtoC; Csolve. Qed.
