(** * Helper Lemmas for StatMech.v

    Auxiliary lemmas needed for thermodynamic CNO proofs.
    These should eventually be integrated into CNO.v.
*)

Require Import CNO.

(** State equality is symmetric *)
Lemma state_eq_sym :
  forall s1 s2 : ProgramState,
    s1 =st= s2 -> s2 =st= s1.
Proof.
  intros s1 s2 [H_mem [H_reg [H_io H_pc]]].
  unfold state_eq.
  repeat split.
  - unfold mem_eq in *. intro addr. symmetry. apply H_mem.
  - symmetry. assumption.
  - symmetry. assumption.
  - symmetry. assumption.
Qed.

(** State equality is transitive *)
Lemma state_eq_trans :
  forall s1 s2 s3 : ProgramState,
    s1 =st= s2 -> s2 =st= s3 -> s1 =st= s3.
Proof.
  intros s1 s2 s3 [H_mem12 [H_reg12 [H_io12 H_pc12]]] [H_mem23 [H_reg23 [H_io23 H_pc23]]].
  unfold state_eq.
  repeat split.
  - unfold mem_eq in *. intro addr.
    transitivity (state_memory s2 addr); [apply H_mem12 | apply H_mem23].
  - transitivity (state_registers s2); assumption.
  - transitivity (state_io s2); assumption.
  - transitivity (state_pc s2); assumption.
Qed.

(** For CNOs, evaluation from any state leads back to that state *)
Lemma cno_eval_identity :
  forall p s,
    is_CNO p ->
    exists s', eval p s s' /\ s =st= s'.
Proof.
  intros p s H_cno.
  destruct H_cno as [H_term [H_id _]].
  destruct (H_term s) as [s' H_eval].
  exists s'.
  split.
  - assumption.
  - apply H_id. assumption.
Qed.
