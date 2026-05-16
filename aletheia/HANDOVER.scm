;;; HANDOVER.scm --- Aletheia Project Handover for Claude Code
;;;
;;; Copyright (C) 2025 Jonathan D. A. Jewell
;;; SPDX-License-Identifier: PMPL-1.0-or-later
;;;
;;; For: AI assistants starting work on the Aletheia project
;;; Updated: 2026-02-05

;;; ============================================================================
;;; CONFORMANCE BADGES
;;; ============================================================================

(define-module (aletheia handover)
  #:export (project-identity
            conformance-badges
            value-chain
            ecosystem
            project-setup
            language-policy
            key-files))

(define conformance-badges
  '((rsr
     (level . bronze)
     (badge . "https://img.shields.io/badge/RSR-Bronze-cd7f32")
     (report . "conformance/RSR-CONFORMANCE.md")
     (status . compliant))

    (palimpsest
     (version . "0.8")
     (badge . "https://img.shields.io/badge/License-Palimpsest%20v0.8-blue")
     (report . "conformance/PALIMPSEST-CONFORMANCE.md")
     (status . compliant))

    (oikos
     (badge . "https://img.shields.io/badge/Oikos-Pending-lightgrey")
     (report . "conformance/OIKOS-CONFORMANCE.md")
     (status . pending))

    (echidna
     (badge . "https://img.shields.io/badge/Echidna-Pending-lightgrey")
     (report . "conformance/ECHIDNA-CONFORMANCE.md")
     (status . pending))

    (value-chain
     (badge . "https://img.shields.io/badge/Value%20Chain-Defined-green")
     (report . "docs/VALUE-CHAIN-POLICY.md")
     (status . defined))))

;;; ============================================================================
;;; PROJECT IDENTITY
;;; ============================================================================

(define project-identity
  '((name . "Aletheia")
    (etymology . "ἀλήθεια - truth, disclosure, unconcealment")
    (purpose . "Precursor research project for building a reversible operating system")
    (future . "r-Minix - reversible Minix")

    ;; What Aletheia IS
    (is . (reversible-computing-research
           cno-theory-application
           r-minix-precursor
           absolute-zero-connected))

    ;; What Aletheia is NOT
    (is-not . (rsr-compliance-tool      ; That's rhodibot
               cicd-pipeline-generator   ; That's rhodium-pipeline
               repository-standards-only))))

;;; ============================================================================
;;; VALUE CHAIN POSITION
;;; ============================================================================

(define value-chain
  '((policy-document . "docs/VALUE-CHAIN-POLICY.md")

    (inbound-logistics
     (source . "Absolute Zero")
     (inputs . (cno-proofs
                landauer-principle
                bennett-reversibility)))

    (operations
     (focus . "Reversible Minix research")
     (activities . (os-primitives
                    undo-redo-mechanisms
                    state-capture)))

    (outbound-logistics
     (targets . (cccp-stack
                 valence-shell
                 future-r-minix
                 svalinn-integration)))))

;;; ============================================================================
;;; THEORETICAL FOUNDATION
;;; ============================================================================

(define theoretical-foundation
  '((source . "Absolute Zero")
    (repository . "https://gitlab.com/hyperpolymath/absolute-zero")

    (concepts
     ((name . "CNO")
      (definition . "Certified Null Operation")
      (formula . "op ;; reverse(op) ≡ identity"))

     ((name . "Landauer's Principle")
      (definition . "Computation dissipates heat")
      (formula . "kT ln 2 per bit erased"))

     ((name . "Bennett's Insight")
      (definition . "Reversible computation = zero heat dissipation"))

     ((name . "Reversibility")
      (definition . "Every operation has an inverse")))))

;;; ============================================================================
;;; ECOSYSTEM CONNECTIONS
;;; ============================================================================

(define ecosystem
  '((upstream
     ((name . "absolute-zero")
      (role . "Formal proofs - provides CNOs")
      (url . "https://gitlab.com/hyperpolymath/absolute-zero")))

    (siblings
     ((name . "valence-shell")
      (role . "Reversible filesystem operations"))

     ((name . "rhodibot")
      (role . "RSR compliance checking")
      (local . "extraction/rhodibot/"))

     ((name . "rhodium-pipeline")
      (role . "CI/CD pipeline generation")
      (local . "extraction/rhodium-pipeline/")))

    (downstream
     ((name . "r-Minix")
      (role . "Future reversible Minix")
      (status . planned)))))

;;; ============================================================================
;;; PROJECT SETUP
;;; ============================================================================

(define project-setup
  '((bots
     (echidnabot . "Formal proof-based code validation")
     (oikos-bot . "Economic/ecological analysis")
     (rhodibot . "RSR compliance checking"))

    (ai-support-files
     (META.scm . "Project metadata and ADRs")
     (ECOSYSTEM.scm . "Ecosystem connections")
     (STATE.scm . "Current project state")
     (HANDOVER.scm . "Project handover (Scheme)")
     (PLAYBOOK.scm . "Development workflows and runbook")
     (AGENTIC.scm . "AI agent configuration and constraints")
     (NEUROSYM.scm . "Neurosymbolic integration settings"))

    (build-system
     (tool . "mustfile")
     (description . "Hyperpolymath build/task automation"))

    (licensing
     (primary . "PMPL-1.0-or-later")
     (file . "LICENSE"))))

;;; ============================================================================
;;; LANGUAGE POLICY
;;; ============================================================================

(define language-policy
  '((allowed
     ((language . Rust)
      (use-case . "Systems, performance-critical"))
     ((language . ReScript)
      (use-case . "Application code"))
     ((language . Deno)
      (use-case . "Runtime"))
     ((language . Gleam)
      (use-case . "Backend services"))
     ((language . Bash)
      (use-case . "Scripts, automation"))
     ((language . Guile-Scheme)
      (use-case . "State/meta files")))

    (banned
     ((language . TypeScript)
      (replacement . ReScript))
     ((language . Node.js)
      (replacement . Deno))
     ((language . npm)
      (replacement . Deno))
     ((language . Go)
      (replacement . Rust))
     ((language . Python)
      (replacement . "Rust/AffineScript")
      (exception . "SaltStack only")))))

;;; ============================================================================
;;; KEY FILES
;;; ============================================================================

(define key-files
  '((handover
     (HANDOVER.scm . "This file - project handover (Scheme)")
     (ALETHEIA-HANDOVER.md . "Project handover (Markdown)")
     (CROSSREPO-HANDOVER.md . "Full ecosystem documentation"))

    (instructions
     (CLAUDE.md . "AI assistant instructions"))

    (policy
     (docs/VALUE-CHAIN-POLICY.md . "Porter's Value Chain model"))

    (conformance
     (conformance/README.md . "Conformance reports index")
     (conformance/RSR-CONFORMANCE.md . "Repository standards")
     (conformance/PALIMPSEST-CONFORMANCE.md . "License compliance")
     (conformance/OIKOS-CONFORMANCE.md . "Economic/ecological")
     (conformance/ECHIDNA-CONFORMANCE.md . "Formal verification"))

    (extractions
     (extraction/rhodibot/ . "RSR compliance bot package")
     (extraction/rhodium-pipeline/ . "CI/CD generator package"))))

;;; ============================================================================
;;; CONTACT
;;; ============================================================================

(define contact
  '((maintainer . "Jonathan D. A. Jewell")
    (gitlab . "https://gitlab.com/hyperpolymath")
    (github . "https://github.com/Hyperpolymath")))

;;; ============================================================================
;;; QUICK START FOR AI ASSISTANTS
;;; ============================================================================

(define quick-start
  '((step-1 . "Read this file to understand Aletheia's purpose")
    (step-2 . "Check CLAUDE.md for development constraints")
    (step-3 . "Review conformance/ for current compliance status")
    (step-4 . "Focus on reversible OS primitives research")

    (remember
     "Aletheia is NOT an RSR tool - it's reversible OS research"
     "RSR compliance code belongs in rhodibot/rhodium-pipeline"
     "Theoretical foundation comes from Absolute Zero"
     "Goal is to enable future r-Minix development")))

;;; ============================================================================
;;; END OF HANDOVER
;;; ============================================================================

;;; Aletheia: Unconcealing the path to reversible operating systems.
