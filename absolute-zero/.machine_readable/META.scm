;; SPDX-License-Identifier: PMPL-1.0-or-later
;; META.scm - Meta-level information for absolute-zero
;; Media-Type: application/meta+scheme

(meta
  (architecture-decisions
    (("ADR-001" "accepted" "ProofIrrelevance for morphism equality in category theory")
     ("ADR-002" "accepted" "Dual Landauer formalization: axiom (StatMech.v) + derivation (LandauerDerivation.v)")
     ("ADR-003" "accepted" "Lambda CNO = identity property only, not termination")
     ("ADR-004" "accepted" "post_execution_dist specialized for CNOs (identity on distributions)")
     ("ADR-005" "proposed" "Fix QuantumCNO.v Cexp: real exp -> complex phase factor")))

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
