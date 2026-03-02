;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem position for absolute-zero
;; Media-Type: application/vnd.ecosystem+scm

(ecosystem
  (version "1.0")
  (name "absolute-zero")
  (type "research-project")
  (purpose "Formal verification of Certified Null Operations (CNOs):
    programs proven to compute nothing. Multi-prover theorem proving
    with thermodynamic, categorical, and quantum foundations.")

  (position-in-ecosystem
    (category "formal-methods")
    (subcategory "program-verification")
    (unique-value
      ("First formal treatment of computational nullity"
       "Multi-prover verification across 6 proof systems"
       "Category-theoretic model independence"
       "Thermodynamic grounding via Landauer/Bennett")))

  (related-projects
    (("echidna" "sibling-standard" "Security scanning; potential CNO verifier integration")
     ("echidnabot" "potential-consumer" "GitHub bot; could automate CNO checks in PRs")
     ("valence-shell" "integration-target" "Filesystem ops library proven in FilesystemCNO.v")
     ("rsr-template-repo" "infrastructure" "Repository standards template")))

  (what-this-is
    ("Formal verification research"
     "Multi-prover theorem proving"
     "Theoretical CS contribution"
     "Bridge: computation theory <-> thermodynamics"))

  (what-this-is-not
    ("Runtime library"
     "Programming language"
     "Production software")))
