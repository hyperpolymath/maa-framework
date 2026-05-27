;; SPDX-License-Identifier: MPL-2.0
;; META.scm - Meta-level information for absolute-zero
;; Media-Type: application/meta+scheme

(meta
  (architecture-decisions
    (("ADR-001" "accepted" "ProofIrrelevance for morphism equality in category theory")
     ("ADR-002" "accepted" "Dual Landauer formalization: axiom (StatMech.v) + derivation (LandauerDerivation.v)")
     ("ADR-003" "accepted" "Lambda CNO = identity property only, not termination")
     ("ADR-004" "accepted" "post_execution_dist specialized for CNOs (identity on distributions)")
     ("ADR-005" "proposed" "Fix QuantumCNO.v Cexp: real exp -> complex phase factor")
     ("ADR-006" "accepted" "state_eq excludes state_pc — PC is control-flow bookkeeping, not observable side effect (2026-05-18 rescue)")
     ("ADR-007" "accepted" "Discharge eval_deterministic Axiom → Theorem via step_deterministic_strong helper (2026-05-20, PR #24); first post-T0 axiom audit win")
     ("ADR-008" "accepted" "Delete unsound eval_respects_state_eq_{left,right} axioms; weaken logically_reversible to =st= (observational reversibility); re-prove cno_eval_on_equal_states + cno_logically_reversible via cno_terminates + cno_preserves_state (2026-05-20)")
     ("ADR-009" "accepted" "Delete unsound alignmentMatchesPlatformWord Idris2 postulate (HasAlignment carries no evidence; would derive So (1 mod 8 == 0) from CNOResultLayout.alignment); replace single consumer with per-Platform decidable proof. Consolidate remaining alignedSizeCorrect postulate into shared AbsoluteZero.ABI.Proofs.DivMod module as the estate-wide div/mod lemma surface (absolute-zero#27, civic-connect alignUpDivides/mkFieldsAligned/offsetInBoundsPrf migrate here)")
     ("ADR-010" "accepted" "Phase 1 per-axiom triage of 72 Coq Axioms per standards#203 trusted-base policy (2026-05-27, PR #58): 52 §c TRUSTED-BASE + 17 §a DISCHARGE backlog + 3 §b PROPERTY-TEST; canonical disposition in docs/proof-debt-triage.md")))

  (development-practices
    (code-style "Coq proof engineering")
    (security
      (principle "Defense in depth"))
    (testing "Multi-prover cross-validation")
    (versioning "SemVer")
    (documentation "AsciiDoc")
    (branching "main for stable")
    (proof-methodology
      ("Prefer Qed over Admitted"
       "Axiomatize physical laws"
       "Document all Admitted with rationale"
       "Separate helpers into dedicated files")))

  (design-rationale
    ("CNOs are identity morphisms in categories"
     "Multi-prover for maximum confidence"
     "Thermodynamic grounding via Landauer/Bennett"
     "Progressive: axiom -> theorem -> verified")))
